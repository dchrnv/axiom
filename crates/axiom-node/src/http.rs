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
//   GET  /api/status               — JSON {tick, dream_phase} для tray/healthcheck
//   POST /api/lab/run              — запустить lab job (obs/bench/test/showcase)
//   POST /api/lab/stop             — остановить текущий job
//   POST /api/lab/pause            — SIGSTOP текущего job
//   POST /api/lab/resume           — SIGCONT текущего job
//   POST /api/lab/import-obs       — импортировать OBS traces в живой движок
//   GET  /api/lab/status           — статус текущего job (Running/Paused/Done/Failed)
//   GET  /api/lab/ws/log           — WebSocket stream лога текущего job
//   GET  /*                        — статика React SPA из web_dist/

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

use axiom_broadcasting::BroadcastHandle;
use axiom_corpus::{GenerateMode, generate};


/// Команды из HTTP → tick loop.
pub enum NodeCmd {
    AdvisoryConfirm(u64),
    AdvisoryReject(u64),
    SubmitText(String),
    ImportObs(std::path::PathBuf),
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
        .route("/api/status", get(api_status))
        .route("/api/lab/import-obs", post(api_import_obs))
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

// ── OBS import ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ImportObsBody {
    #[serde(default)]
    path: Option<String>,
}

#[derive(serde::Serialize)]
struct ImportObsResponse {
    imported: u32,
    guardian_rejected: u32,
    path: String,
}

async fn api_import_obs(
    State(s): State<Arc<AppState>>,
    Json(body): Json<ImportObsBody>,
) -> Result<Json<ImportObsResponse>, StatusCode> {
    let path = body.path.unwrap_or_else(|| "obs_out/traces.bin".to_string());
    let pb = std::path::PathBuf::from(&path);
    if !pb.exists() {
        warn!("import-obs: file not found: {path}");
        return Err(StatusCode::NOT_FOUND);
    }
    let _ = s.cmd_tx.send(NodeCmd::ImportObs(pb.clone()));
    info!("import-obs: queued import from {path}");
    Ok(Json(ImportObsResponse { imported: 0, guardian_rejected: 0, path }))
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

// ── Status (healthcheck / tray) ──────────────────────────────────────────────

#[derive(serde::Serialize)]
struct StatusResponse {
    tick: u64,
    dream_phase: &'static str,
}

async fn api_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (tick, dream_phase) = match state.handle.latest_snapshot() {
        Some(snap) => {
            let phase = match snap.engine_state {
                axiom_protocol::events::EngineState::Wake          => "wake",
                axiom_protocol::events::EngineState::FallingAsleep => "falling_asleep",
                axiom_protocol::events::EngineState::Dreaming      => "dreaming",
                axiom_protocol::events::EngineState::Waking        => "waking",
            };
            (snap.current_tick, phase)
        }
        None => (0, "wake"),
    };
    Json(StatusResponse { tick, dream_phase })
}
