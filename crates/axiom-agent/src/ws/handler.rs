// SPDX-License-Identifier: AGPL-3.0-only

use std::collections::HashSet;
use std::sync::atomic::Ordering;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;

use super::server::AppState;
use crate::adapter_command::{AdapterCommand, AdapterPayload, AdapterSource};
use crate::protocol::ServerMessage;

/// Входящее сообщение от WebSocket-клиента.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Inject { text: String },
    ReadCommand { cmd: String },
    MutateCommand { cmd: String },
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    DomainSnapshot { domain_id: u16 },
}

/// Обработать одно WebSocket-соединение.
///
/// Читает ClientMessage → AdapterCommand → command_tx.
/// Читает broadcast_rx → фильтрует по подпискам → отправляет клиенту.
pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let conn_id = state.next_conn_id.fetch_add(1, Ordering::Relaxed);
    let (mut ws_tx, mut ws_rx) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();
    // None  = никогда не подписывался → получать всё (default)
    // Some  = явно управляет подписками; пустой set = ничего, кроме прямых ответов
    let mut subscriptions: Option<HashSet<String>> = None;
    let mut seq: u64 = 0;

    loop {
        tokio::select! {
            incoming = ws_rx.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(msg) => {
                                seq += 1;
                                if !dispatch(msg, conn_id, seq, &mut subscriptions,
                                             &state, &mut ws_tx).await
                                {
                                    break;
                                }
                            }
                            Err(e) => {
                                let err = error_msg(None, format!("parse error: {e}"));
                                if ws_tx.send(err).await.is_err() { break; }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}   // Binary, Ping, Pong — ignore
                    Some(Err(_)) => break,
                }
            }

            outgoing = broadcast_rx.recv() => {
                match outgoing {
                    Ok(msg) => {
                        if !should_send(&msg, &subscriptions) { continue; }
                        let json = serde_json::to_string(&msg).unwrap_or_default();
                        if ws_tx.send(Message::Text(json.into())).await.is_err() { break; }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        let warn = error_msg(None, format!("lagged by {n} messages"));
                        if ws_tx.send(warn).await.is_err() { break; }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Отправить AdapterCommand или обновить подписки.
/// Возвращает false если нужно закрыть соединение.
async fn dispatch(
    msg: ClientMessage,
    conn_id: u64,
    seq: u64,
    subscriptions: &mut Option<HashSet<String>>,
    state: &AppState,
    ws_tx: &mut (impl SinkExt<Message, Error = axum::Error> + Unpin),
) -> bool {
    match msg {
        ClientMessage::Subscribe { channels } => {
            subscriptions
                .get_or_insert_with(HashSet::new)
                .extend(channels);
        }
        ClientMessage::Unsubscribe { channels } => {
            if let Some(set) = subscriptions.as_mut() {
                for ch in channels {
                    set.remove(&ch);
                }
            }
        }
        other => {
            let id = format!("ws{conn_id}_{seq}");
            let payload = to_payload(other);
            let cmd = AdapterCommand {
                id,
                source: AdapterSource::WebSocket(conn_id),
                payload,
                priority: axiom_runtime::GatewayPriority::Normal,
            };
            if state.command_tx.send(cmd).await.is_err() {
                let err = error_msg(None, "engine unavailable".to_string());
                let _ = ws_tx.send(err).await;
                return false;
            }
        }
    }
    true
}

fn to_payload(msg: ClientMessage) -> AdapterPayload {
    match msg {
        ClientMessage::Inject { text } => AdapterPayload::Inject { text },
        ClientMessage::ReadCommand { cmd } => AdapterPayload::MetaRead { cmd },
        ClientMessage::MutateCommand { cmd } => AdapterPayload::MetaMutate { cmd },
        ClientMessage::DomainSnapshot { domain_id } => AdapterPayload::DomainSnapshot { domain_id },
        ClientMessage::Subscribe { channels } => AdapterPayload::Subscribe { channels },
        ClientMessage::Unsubscribe { channels } => AdapterPayload::Unsubscribe { channels },
    }
}

/// Нужно ли отправить сообщение этому клиенту с учётом его подписок?
///
/// `None`  = никогда не подписывался → получать всё (default).
/// `Some`  = явное управление: включён только то, на что подписан.
///           Пустой Some → только прямые ответы.
fn should_send(msg: &ServerMessage, subs: &Option<HashSet<String>>) -> bool {
    match subs {
        None => true,
        Some(set) => match msg {
            ServerMessage::Tick { .. } => set.contains("ticks"),
            ServerMessage::State { .. } => set.contains("state"),
            // Прямые ответы доставляются независимо от подписок
            ServerMessage::Result { .. }
            | ServerMessage::CommandResult { .. }
            | ServerMessage::DomainDetail(_)
            | ServerMessage::Error { .. } => true,
        },
    }
}

fn error_msg(command_id: Option<String>, message: String) -> Message {
    let json = serde_json::to_string(&ServerMessage::Error {
        command_id,
        message,
    })
    .unwrap_or_default();
    Message::Text(json.into())
}
