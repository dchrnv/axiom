// Lab — управление dev-задачами (OBS, бенчи, тесты) прямо из браузера.
//
// Один активный job одновременно. Stdout+stderr пишутся в broadcast канал
// и читаются через WebSocket /api/lab/ws/log.

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn};

const LOG_CAPACITY: usize = 512;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Idle,
    Running,
    Paused,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobState {
    pub status: JobStatus,
    pub job: Option<String>,
    pub exit_code: Option<i32>,
}

pub struct LabHandle {
    state: Arc<Mutex<JobState>>,
    log_tx: broadcast::Sender<String>,
    kill_tx: Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
    child_pid: Arc<Mutex<Option<u32>>>,
    repo_root: PathBuf,
}

impl LabHandle {
    pub fn new(repo_root: PathBuf) -> Arc<Self> {
        let (log_tx, _) = broadcast::channel(LOG_CAPACITY);
        Arc::new(Self {
            state: Arc::new(Mutex::new(JobState {
                status: JobStatus::Idle,
                job: None,
                exit_code: None,
            })),
            log_tx,
            kill_tx: Arc::new(Mutex::new(None)),
            child_pid: Arc::new(Mutex::new(None)),
            repo_root,
        })
    }

    pub async fn status(&self) -> JobState {
        self.state.lock().await.clone()
    }

    pub fn subscribe_log(&self) -> broadcast::Receiver<String> {
        self.log_tx.subscribe()
    }

    pub async fn stop(&self) {
        let mut kill = self.kill_tx.lock().await;
        if let Some(tx) = kill.take() {
            let _ = tx.send(());
        }
        *self.child_pid.lock().await = None;
    }

    pub async fn pause(&self) -> bool {
        let pid = *self.child_pid.lock().await;
        let Some(pid) = pid else { return false };
        let st = self.state.lock().await.status.clone();
        if st != JobStatus::Running { return false; }
        let ok = std::process::Command::new("kill")
            .args(["-STOP", &pid.to_string()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ok {
            self.state.lock().await.status = JobStatus::Paused;
            let _ = self.log_tx.send("[lab] paused".to_string());
        }
        ok
    }

    pub async fn resume(&self) -> bool {
        let pid = *self.child_pid.lock().await;
        let Some(pid) = pid else { return false };
        let st = self.state.lock().await.status.clone();
        if st != JobStatus::Paused { return false; }
        let ok = std::process::Command::new("kill")
            .args(["-CONT", &pid.to_string()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if ok {
            self.state.lock().await.status = JobStatus::Running;
            let _ = self.log_tx.send("[lab] resumed".to_string());
        }
        ok
    }

    pub async fn run(&self, job: &str, corpus: Option<String>) -> Result<(), &'static str> {
        {
            let s = self.state.lock().await;
            if s.status == JobStatus::Running {
                return Err("job already running");
            }
        }

        let cmd_parts = self.resolve_command(job, corpus).ok_or("unknown job")?;
        let job_name = job.to_string();
        let root = self.repo_root.clone();

        let (kill_tx, mut kill_rx) = tokio::sync::oneshot::channel::<()>();
        *self.kill_tx.lock().await = Some(kill_tx);

        {
            let mut s = self.state.lock().await;
            *s = JobState { status: JobStatus::Running, job: Some(job_name.clone()), exit_code: None };
        }

        let state_clone = self.state.clone();
        let log_tx_clone = self.log_tx.clone();
        let child_pid_clone = self.child_pid.clone();

        tokio::spawn(async move {
            let prog = &cmd_parts[0];
            let args = &cmd_parts[1..];

            let mut child = match Command::new(prog)
                .args(args)
                .current_dir(&root)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    warn!("lab: failed to spawn {prog}: {e}");
                    let _ = log_tx_clone.send(format!("[lab] error: {e}"));
                    let mut s = state_clone.lock().await;
                    *s = JobState { status: JobStatus::Failed, job: Some(job_name), exit_code: None };
                    return;
                }
            };

            *child_pid_clone.lock().await = child.id();
            info!("lab: started job={job_name} pid={:?}", child.id());

            // Stream stdout
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();
            let tx_out = log_tx_clone.clone();
            let tx_err = log_tx_clone.clone();

            tokio::spawn(async move {
                let mut lines = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = tx_out.send(line);
                }
            });
            tokio::spawn(async move {
                let mut lines = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = tx_err.send(line);
                }
            });

            // Wait for exit or kill
            let exit_status = tokio::select! {
                status = child.wait() => status.ok(),
                _ = &mut kill_rx => {
                    let _ = child.kill().await;
                    let _ = log_tx_clone.send("[lab] stopped by user".to_string());
                    None
                }
            };

            let code = exit_status.and_then(|s| s.code());
            let success = code == Some(0);
            info!("lab: job={job_name} finished, exit_code={code:?}");
            let _ = log_tx_clone.send(format!(
                "[lab] done — {}",
                if success { "success" } else { "failed" }
            ));

            *child_pid_clone.lock().await = None;
            let mut s = state_clone.lock().await;
            *s = JobState {
                status: if success { JobStatus::Done } else { JobStatus::Failed },
                job: Some(job_name),
                exit_code: code,
            };
        });

        Ok(())
    }

    fn resolve_command(&self, job: &str, corpus: Option<String>) -> Option<Vec<String>> {
        let root = self.repo_root.to_string_lossy().to_string();
        match job {
            "obs" => {
                let c = corpus.unwrap_or_else(|| "config/obs/corpus_large.yaml".to_string());
                Some(vec![
                    format!("{root}/target/release/axiom-observe"),
                    c,
                    "showcase/obs_out".to_string(),
                    "config/anchors".to_string(),
                ])
            }
            "obs_quick" => Some(vec![
                format!("{root}/target/release/axiom-observe"),
                "config/obs/corpus_mixed.yaml".to_string(),
                "showcase/obs_out".to_string(),
                "config/anchors".to_string(),
            ]),
            "bench_hot" => Some(vec![
                "cargo".to_string(),
                "bench".to_string(),
                "--bench".to_string(),
                "hot_path_regression".to_string(),
                "--".to_string(),
                "--noplot".to_string(),
            ]),
            "bench_od" => Some(vec![
                "cargo".to_string(),
                "bench".to_string(),
                "--bench".to_string(),
                "over_domain_bench".to_string(),
                "--".to_string(),
                "--noplot".to_string(),
            ]),
            "bench_stress" => Some(vec![
                "cargo".to_string(),
                "bench".to_string(),
                "--bench".to_string(),
                "stress_bench".to_string(),
                "--".to_string(),
                "--noplot".to_string(),
            ]),
            "test" => Some(vec![
                "cargo".to_string(),
                "test".to_string(),
                "--workspace".to_string(),
            ]),
            "showcase" => Some(vec![
                format!("{root}/scripts/showcase.sh"),
            ]),
            _ => None,
        }
    }
}

// ── HTTP handlers ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RunRequest {
    pub job: String,
    #[serde(default)]
    pub corpus: Option<String>,
}

pub async fn route_run(
    State(lab): State<Arc<LabHandle>>,
    Json(body): Json<RunRequest>,
) -> StatusCode {
    match lab.run(&body.job, body.corpus).await {
        Ok(()) => StatusCode::ACCEPTED,
        Err("job already running") => StatusCode::CONFLICT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn route_stop(State(lab): State<Arc<LabHandle>>) -> StatusCode {
    lab.stop().await;
    StatusCode::OK
}

pub async fn route_pause(State(lab): State<Arc<LabHandle>>) -> StatusCode {
    if lab.pause().await { StatusCode::OK } else { StatusCode::CONFLICT }
}

pub async fn route_resume(State(lab): State<Arc<LabHandle>>) -> StatusCode {
    if lab.resume().await { StatusCode::OK } else { StatusCode::CONFLICT }
}

pub async fn route_status(State(lab): State<Arc<LabHandle>>) -> Json<JobState> {
    Json(lab.status().await)
}

pub async fn route_log_ws(
    ws: WebSocketUpgrade,
    State(lab): State<Arc<LabHandle>>,
) -> impl IntoResponse {
    let rx = lab.subscribe_log();
    ws.on_upgrade(|socket| handle_log_ws(socket, rx))
}

async fn handle_log_ws(mut socket: WebSocket, mut rx: broadcast::Receiver<String>) {
    loop {
        match rx.recv().await {
            Ok(line) => {
                if socket.send(Message::Text(line.into())).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                warn!("lab log WS lagged, dropped {n} messages");
            }
            Err(_) => break,
        }
    }
}
