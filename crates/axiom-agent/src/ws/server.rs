// SPDX-License-Identifier: AGPL-3.0-only

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use axum::{Router, extract::{State, WebSocketUpgrade}, response::IntoResponse, routing::get};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axiom_runtime::BroadcastSnapshot;

use crate::adapter_command::AdapterCommand;
use crate::protocol::ServerMessage;
use super::handler::handle_socket;

/// Общее состояние — клонируется per-request.
#[derive(Clone)]
pub struct AppState {
    pub command_tx:   mpsc::Sender<AdapterCommand>,
    pub broadcast_tx: broadcast::Sender<ServerMessage>,
    pub snapshot:     Arc<RwLock<BroadcastSnapshot>>,
    pub next_conn_id: Arc<AtomicU64>,
}

/// Привязать TCP-listener к порту. Порт 0 → ОС выбирает свободный.
pub async fn bind(port: u16) -> TcpListener {
    TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind WebSocket port")
}

/// Запустить сервер (WebSocket + REST) на уже привязанном listener.
/// Блокирует задачу — вызывать через tokio::spawn.
pub async fn serve_ws(listener: TcpListener, state: AppState) {
    let app = Router::new()
        .route("/ws", get(ws_upgrade))
        .merge(crate::rest::rest_routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("WebSocket server error");
}

async fn ws_upgrade(
    ws:             WebSocketUpgrade,
    State(state):   State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}
