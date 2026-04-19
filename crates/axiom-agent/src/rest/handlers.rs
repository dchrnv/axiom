// SPDX-License-Identifier: AGPL-3.0-only

use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::adapter_command::{AdapterCommand, AdapterPayload, AdapterSource};
use crate::protocol::ServerMessage;
use crate::ws::AppState;

// ── router ────────────────────────────────────────────────────────────────────

/// Вернуть Router со всеми REST-маршрутами для монтирования в общий axum router.
pub fn rest_routes() -> Router<AppState> {
    Router::new()
        .route("/api/status",      get(get_status))
        .route("/api/domains",     get(get_domains))
        .route("/api/domain/{id}", get(get_domain))
        .route("/api/inject",      post(post_inject))
        .route("/api/command",     post(post_command))
}

// ── GET /api/status ───────────────────────────────────────────────────────────

async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    let snap = state.snapshot.read().await;
    Json(snap.clone())
}

// ── GET /api/domains ──────────────────────────────────────────────────────────

async fn get_domains(State(state): State<AppState>) -> impl IntoResponse {
    let snap = state.snapshot.read().await;
    Json(snap.domain_summaries.clone())
}

// ── GET /api/domain/:id ───────────────────────────────────────────────────────

async fn get_domain(
    Path(id):     Path<u16>,
    State(state): State<AppState>,
) -> Response {
    let req_id = format!("rest{}", state.next_conn_id.fetch_add(1, Ordering::Relaxed));
    let mut rx = state.broadcast_tx.subscribe();

    if state.command_tx.send(AdapterCommand {
        id:      req_id.clone(),
        source:  AdapterSource::Rest,
        payload: AdapterPayload::DomainSnapshot { domain_id: id },
    }).await.is_err() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    match wait_for(Duration::from_secs(5), async move {
        loop {
            match rx.recv().await {
                Ok(msg @ ServerMessage::DomainDetail(_)) => {
                    // DomainDetail не несёт command_id — берём первый ответ после нашего запроса.
                    // В однопользовательском сценарии это корректно; в multi-client
                    // следует добавить command_id в DomainDetail (EA-TD будущего).
                    return Some(msg);
                }
                Ok(ServerMessage::Error { command_id: Some(ref cid), .. }) if *cid == req_id => {
                    return None; // domain not found
                }
                Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }).await {
        Some(msg) => (StatusCode::OK, Json(msg)).into_response(),
        None      => StatusCode::NOT_FOUND.into_response(),
    }
}

// ── POST /api/inject ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct InjectBody { text: String }

async fn post_inject(
    State(state): State<AppState>,
    body: Result<Json<InjectBody>, axum::extract::rejection::JsonRejection>,
) -> Response {
    let Json(body) = match body {
        Ok(b)  => b,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let req_id = format!("rest{}", state.next_conn_id.fetch_add(1, Ordering::Relaxed));
    let mut rx = state.broadcast_tx.subscribe();

    if state.command_tx.send(AdapterCommand {
        id:      req_id.clone(),
        source:  AdapterSource::Rest,
        payload: AdapterPayload::Inject { text: body.text },
    }).await.is_err() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    match wait_for(Duration::from_secs(5), async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if let ServerMessage::Result { ref command_id, .. } = msg {
                        if *command_id == req_id { return Some(msg); }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }).await {
        Some(msg) => (StatusCode::OK, Json(msg)).into_response(),
        None      => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ── POST /api/command ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct CommandBody {
    cmd:              String,
    #[serde(rename = "type", default)]
    cmd_type: String, // "read" | "mutate" (default: "read")
}

async fn post_command(
    State(state): State<AppState>,
    body: Result<Json<CommandBody>, axum::extract::rejection::JsonRejection>,
) -> Response {
    let Json(body) = match body {
        Ok(b)  => b,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let req_id = format!("rest{}", state.next_conn_id.fetch_add(1, Ordering::Relaxed));
    let mut rx = state.broadcast_tx.subscribe();

    let payload = if body.cmd_type == "mutate" {
        AdapterPayload::MetaMutate { cmd: body.cmd }
    } else {
        AdapterPayload::MetaRead { cmd: body.cmd }
    };

    if state.command_tx.send(AdapterCommand {
        id:      req_id.clone(),
        source:  AdapterSource::Rest,
        payload,
    }).await.is_err() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    match wait_for(Duration::from_secs(5), async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if let ServerMessage::CommandResult { ref command_id, .. } = msg {
                        if *command_id == req_id { return Some(msg); }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }).await {
        Some(msg) => (StatusCode::OK, Json(msg)).into_response(),
        None      => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ── helper ────────────────────────────────────────────────────────────────────

async fn wait_for<T, F>(timeout: Duration, fut: F) -> T
where
    F: std::future::Future<Output = T>,
    T: Default,
{
    tokio::time::timeout(timeout, fut).await.unwrap_or_default()
}
