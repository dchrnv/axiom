// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// http — axum HTTP server для axiom-node.
// Раздаёт React SPA (dist/) и предоставляет JSON-WebSocket мост + REST API.
//
// Маршруты:
//   GET  /api/ws                   — WebSocket, EngineMessage → JSON для браузера
//   POST /api/advisory/confirm/:id — подтвердить advisory
//   POST /api/advisory/reject/:id  — отклонить advisory
//   POST /api/text/submit          — отправить текст в движок
//   GET  /api/corpus/generate      — сгенерировать текстовый корпус (mode/count/seed)
//   GET  /metrics                  — Prometheus text format
//   POST /api/lab/run              — запустить lab job (obs/bench/test/showcase)
//   POST /api/lab/stop             — остановить текущий job
//   POST /api/lab/pause            — SIGSTOP текущего job
//   POST /api/lab/resume           — SIGCONT текущего job
//   GET  /api/lab/status           — статус текущего job (Running/Paused/Done/Failed)
//   GET  /api/lab/ws/log           — WebSocket stream лога текущего job
//   GET  /*                        — статика React SPA из web_dist/

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

use axiom_broadcasting::BroadcastHandle;
use axiom_corpus::{GenerateMode, generate};
use axiom_protocol::snapshot::SystemSnapshot;


/// Команды из HTTP → tick loop.
pub enum NodeCmd {
    AdvisoryConfirm(u64),
    AdvisoryReject(u64),
    SubmitText(String),
}

struct AppState {
    handle: Arc<BroadcastHandle>,
    cmd_tx: mpsc::UnboundedSender<NodeCmd>,
}

pub fn create_cmd_channel() -> (
    mpsc::UnboundedSender<NodeCmd>,
    mpsc::UnboundedReceiver<NodeCmd>,
) {
    mpsc::unbounded_channel()
}

pub async fn run(
    addr: SocketAddr,
    handle: Arc<BroadcastHandle>,
    web_dist: PathBuf,
    cmd_tx: mpsc::UnboundedSender<NodeCmd>,
    lab: Arc<crate::lab::LabHandle>,
) {
    let state = Arc::new(AppState { handle, cmd_tx });

    let lab_router = Router::new()
        .route("/run", post(crate::lab::route_run))
        .route("/stop", post(crate::lab::route_stop))
        .route("/pause", post(crate::lab::route_pause))
        .route("/resume", post(crate::lab::route_resume))
        .route("/status", get(crate::lab::route_status))
        .route("/ws/log", get(crate::lab::route_log_ws))
        .with_state(lab);

    let app = Router::new()
        .route("/api/ws", get(ws_handler))
        .route("/api/advisory/confirm/{id}", post(api_confirm))
        .route("/api/advisory/reject/{id}", post(api_reject))
        .route("/api/text/submit", post(api_text_submit))
        .route("/api/corpus/generate", get(api_corpus_generate))
        .route("/metrics", get(metrics_handler))
        .nest("/api/lab", lab_router)
        .fallback_service(ServeDir::new(&web_dist).append_index_html_on_directories(true))
        .with_state(state);

    info!("HTTP server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind HTTP addr");
    axum::serve(listener, app).await.expect("HTTP server error");
}

// ── WebSocket JSON bridge ────────────────────────────────────────────────────

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let handle = state.handle.clone();
    ws.on_upgrade(|socket| handle_ws(socket, handle))
}

async fn handle_ws(mut socket: WebSocket, handle: Arc<BroadcastHandle>) {
    let mut rx = handle.subscribe_events();

    // Отправляем SystemSnapshot при коннекте
    if let Some(snap) = handle.latest_snapshot() {
        use axiom_protocol::messages::EngineMessage;
        if let Ok(json) = serde_json::to_string(&EngineMessage::Snapshot(snap)) {
            let _ = socket.send(Message::Text(json.into())).await;
        }
    }

    // SEN-TD-01 Фаза B: отправляем последний SensoriumState при коннекте
    // Формат: {"Sensorium":{...}} — консистентно с serde enum encoding
    if let Some(json) = handle.latest_sensorium_json() {
        let envelope = format!("{{\"Sensorium\":{json}}}");
        let _ = socket.send(Message::Text(envelope.into())).await;
    }

    loop {
        match rx.recv().await {
            Ok(msg) => {
                let json = match serde_json::to_string(&msg) {
                    Ok(j) => j,
                    Err(e) => { warn!("JSON serialize error: {e}"); continue; }
                };
                if socket.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                warn!("JSON WS client lagged, dropped {n} messages");
            }
            Err(_) => break,
        }
    }
}

// ── Advisory API ─────────────────────────────────────────────────────────────

async fn api_confirm(State(s): State<Arc<AppState>>, Path(id): Path<u64>) -> StatusCode {
    let _ = s.cmd_tx.send(NodeCmd::AdvisoryConfirm(id));
    StatusCode::OK
}

async fn api_reject(State(s): State<Arc<AppState>>, Path(id): Path<u64>) -> StatusCode {
    let _ = s.cmd_tx.send(NodeCmd::AdvisoryReject(id));
    StatusCode::OK
}

// ── Text submit API ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SubmitTextBody {
    text: String,
}

async fn api_text_submit(
    State(s): State<Arc<AppState>>,
    Json(body): Json<SubmitTextBody>,
) -> StatusCode {
    if body.text.trim().is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    let _ = s.cmd_tx.send(NodeCmd::SubmitText(body.text));
    StatusCode::OK
}

// ── Corpus generator ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct CorpusQuery {
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    count: Option<usize>,
    #[serde(default)]
    seed: Option<u64>,
}

#[derive(serde::Serialize)]
struct CorpusResponse {
    lines: Vec<String>,
    mode: String,
}

async fn api_corpus_generate(
    Query(q): Query<CorpusQuery>,
) -> Result<Json<CorpusResponse>, StatusCode> {
    let mode: GenerateMode = q.mode
        .as_deref()
        .unwrap_or("prose")
        .parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let count = q.count.unwrap_or(20).min(500).max(1);
    let seed = q.seed.unwrap_or(0);
    let lines = generate(mode, count, seed);
    let mode_str = format!("{mode:?}").to_lowercase();
    Ok(Json(CorpusResponse { lines, mode: mode_str }))
}

// ── Prometheus metrics ───────────────────────────────────────────────────────

async fn metrics_handler(State(state): State<Arc<AppState>>) -> Response<String> {
    let body = match state.handle.latest_snapshot() {
        Some(snap) => format_metrics(&snap),
        None => "# axiom metrics: no snapshot yet\n".to_string(),
    };
    axum::response::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
        .body(body)
        .unwrap()
}

fn format_metrics(s: &SystemSnapshot) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(2048);

    let state_str = match s.engine_state {
        axiom_protocol::events::EngineState::Wake          => "wake",
        axiom_protocol::events::EngineState::FallingAsleep => "falling_asleep",
        axiom_protocol::events::EngineState::Dreaming      => "dreaming",
        axiom_protocol::events::EngineState::Waking        => "waking",
    };

    let _ = writeln!(out, "# Engine");
    let _ = writeln!(out, "engine_tick_total {}", s.current_tick);
    let _ = writeln!(out, "engine_event_total {}", s.current_event);
    let _ = writeln!(out, "engine_hot_path_ns {}", s.hot_path_ns);
    let _ = writeln!(out, "engine_tokens_total {}", s.over_domain.total_tokens);
    let _ = writeln!(out, "engine_connections_total {}", s.over_domain.total_connections);
    let _ = writeln!(out, "engine_state{{state=\"{state_str}\"}} 1");

    let _ = writeln!(out, "\n# Dream / Fatigue");
    let _ = writeln!(out, "dream_fatigue_current {:.4}", s.fatigue.current);
    let _ = writeln!(out, "dream_fatigue_threshold {:.4}", s.fatigue.threshold);
    let _ = writeln!(out, "dream_cycles_total {}", s.dream_phase_stats.cycles_completed);
    let _ = writeln!(out, "dream_ticks_since_last {}", s.fatigue.ticks_since_dream);
    let _ = writeln!(out, "dream_token_rate {:.4}", s.fatigue.token_rate);

    let _ = writeln!(out, "\n# Guardian");
    let _ = writeln!(out, "guardian_vetoes_total {}", s.guardian_stats.total_vetoes);
    let _ = writeln!(out, "guardian_vetoes_since_wake {}", s.guardian_stats.vetoes_since_wake);

    if let Some(ref fw) = s.frame_weaver_stats {
        let _ = writeln!(out, "\n# FrameWeaver");
        let _ = writeln!(out, "fw_frames_total {}", fw.total_frames);
        let _ = writeln!(out, "fw_frames_in_sutra {}", fw.frames_in_sutra);
        let _ = writeln!(out, "fw_promotions_since_wake {}", fw.promotions_since_wake);
        for (i, v) in fw.syntactic_layer_activations.iter().enumerate() {
            let _ = writeln!(out, "fw_syntactic_layer_activations{{layer=\"S{}\"}} {}", i + 1, v);
        }
    }

    if let Some(ref pc) = s.phase_c {
        let _ = writeln!(out, "\n# Phase C");
        if let Some(oct) = pc.dominant_octant {
            let _ = writeln!(out, "ae_dominant_octant {}", oct);
        }
        if let Some(sub) = pc.dominant_subsystem {
            let _ = writeln!(out, "cr_dominant_subsystem {}", sub);
        }
        let _ = writeln!(out, "arbiter_pending_count {}", pc.pending_advisories.len());
        let _ = writeln!(out, "cr_emergent_pending_count {}", pc.pending_emergent_count);
        for (i, depth) in pc.octant_depth_avg.iter().enumerate() {
            let _ = writeln!(out, "ae_octant_depth_avg{{octant=\"{}\"}} {}", i, depth);
        }
    }

    let _ = writeln!(out, "\n# Domains");
    for d in &s.domains {
        let name = &d.name;
        let _ = writeln!(out, "domain_tokens{{name=\"{name}\"}} {}", d.token_count);
        let _ = writeln!(out, "domain_temperature_avg{{name=\"{name}\"}} {}", d.temperature_avg);
        let _ = writeln!(out, "domain_activity{{name=\"{name}\"}} {}", d.recent_activity);
    }

    let _ = writeln!(out, "\n# Layer activations (over-domain aggregate)");
    for (i, v) in s.over_domain.layer_activations.iter().enumerate() {
        let _ = writeln!(out, "layer_activation{{layer=\"L{}\"}} {}", i + 1, v);
    }

    out
}
