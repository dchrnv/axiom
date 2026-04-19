// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Telegram External Adapter (Phase 4).
//
// Два async-таска:
//   1. poll_task  — long-polling getUpdates → AdapterCommand в tick_loop
//   2. notify_task — broadcast_rx → sendMessage при наличии chat_id в pending
//
// Корреляция ответов: перед отправкой AdapterCommand сохраняем
// command_id → chat_id в pending HashMap.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::{broadcast, mpsc};

use crate::adapter_command::{AdapterCommand, AdapterPayload, AdapterSource};
use crate::protocol::ServerMessage;

/// Конфигурация Telegram-адаптера.
pub struct TelegramConfig {
    /// Bot API токен (из BotFather)
    pub token: String,
    /// Список разрешённых user_id (пустой = разрешены все)
    pub allowed_users: Vec<i64>,
}

/// Входящее сообщение из getUpdates (минимальный набор полей).
#[derive(Debug, Clone)]
pub struct TelegramUpdate {
    pub update_id: u64,
    pub chat_id:   i64,
    pub user_id:   i64,
    pub text:      String,
}

/// Разобрать JSON-ответ getUpdates в список апдейтов.
///
/// Тестируется отдельно от HTTP.
pub fn parse_updates(json: &str) -> Vec<TelegramUpdate> {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(json) else {
        return Vec::new();
    };
    let Some(arr) = val.get("result").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|u| {
            let update_id = u.get("update_id")?.as_u64()?;
            let msg       = u.get("message")?;
            let text      = msg.get("text")?.as_str()?.to_string();
            let chat_id   = msg.get("chat")?.get("id")?.as_i64()?;
            let user_id   = msg.get("from")?.get("id")?.as_i64()?;
            Some(TelegramUpdate { update_id, chat_id, user_id, text })
        })
        .collect()
}

/// Преобразовать текст Telegram-сообщения в AdapterPayload.
///
/// Возвращает `None` для `/start` (обрабатывается отдельно — welcome-сообщение без команды).
pub fn route_message(text: &str) -> Option<AdapterPayload> {
    let t = text.trim();
    if t == "/start" {
        return None; // handled specially — send welcome, no engine command
    }
    if t == "/status" || t == "/state" {
        return Some(AdapterPayload::MetaRead { cmd: ":status".to_string() });
    }
    if t == "/domains" {
        return Some(AdapterPayload::MetaRead { cmd: ":domains".to_string() });
    }
    if t == "/traces" {
        return Some(AdapterPayload::MetaRead { cmd: ":traces".to_string() });
    }
    if t.starts_with('/') {
        // Unknown bot command — ignore
        return None;
    }
    if t.starts_with(':') {
        let cmd = t.splitn(2, ' ').next().unwrap_or(t);
        let is_mutate = matches!(cmd,
            ":save"|":load"|":autosave"|":tick"|":export"|":import"|":quit"|":q"
        );
        return Some(if is_mutate {
            AdapterPayload::MetaMutate { cmd: t.to_string() }
        } else {
            AdapterPayload::MetaRead { cmd: t.to_string() }
        });
    }
    Some(AdapterPayload::Inject { text: t.to_string() })
}

/// Telegram External Adapter.
pub struct TelegramAdapter {
    config:  TelegramConfig,
    /// command_id → chat_id (ожидание ответа)
    pending: Arc<Mutex<HashMap<String, i64>>>,
    /// Монотонный счётчик для уникальных command_id
    seq:     u64,
}

impl TelegramAdapter {
    pub fn new(config: TelegramConfig) -> Self {
        Self {
            config,
            pending: Arc::new(Mutex::new(HashMap::new())),
            seq:     0,
        }
    }

    /// Запустить адаптер: создаёт два async-таска, возвращает немедленно.
    pub fn run(
        self,
        command_tx:   mpsc::Sender<AdapterCommand>,
        broadcast_tx: broadcast::Sender<ServerMessage>,
    ) {
        let TelegramAdapter { config, pending, seq: _ } = self;
        let config = Arc::new(config);
        let broadcast_rx = broadcast_tx.subscribe();

        // Task 1: polling Telegram → AdapterCommand
        tokio::spawn(poll_task(
            Arc::clone(&config),
            command_tx,
            Arc::clone(&pending),
        ));

        // Task 2: broadcast → Telegram sendMessage
        tokio::spawn(notify_task(
            Arc::clone(&config),
            broadcast_rx,
            Arc::clone(&pending),
        ));
    }
}

// ─── Task 1: polling ──────────────────────────────────────────────────────────

async fn poll_task(
    config:     Arc<TelegramConfig>,
    command_tx: mpsc::Sender<AdapterCommand>,
    pending:    Arc<Mutex<HashMap<String, i64>>>,
) {
    let client = reqwest::Client::new();
    let mut offset: u64 = 0;
    let mut seq:    u64 = 0;

    loop {
        let url = format!(
            "https://api.telegram.org/bot{}/getUpdates?offset={}&timeout=30",
            config.token,
            offset,
        );
        let body = match client.get(&url)
            .timeout(Duration::from_secs(40))
            .send().await
        {
            Ok(r)  => match r.text().await { Ok(t) => t, Err(_) => continue },
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        let updates = parse_updates(&body);
        for upd in updates {
            if offset <= upd.update_id {
                offset = upd.update_id + 1;
            }

            // Access control
            if !config.allowed_users.is_empty()
                && !config.allowed_users.contains(&upd.user_id)
            {
                let _ = send_message(
                    &client, &config.token, upd.chat_id,
                    "Access denied."
                ).await;
                continue;
            }

            // /start — welcome message, no engine command
            if upd.text.trim() == "/start" {
                let _ = send_message(
                    &client, &config.token, upd.chat_id,
                    "AXIOM Agent ready. Send text to inject, or /status /domains /traces."
                ).await;
                continue;
            }

            let Some(payload) = route_message(&upd.text) else { continue };

            seq += 1;
            let id = format!("tg{seq}");
            pending.lock().unwrap().insert(id.clone(), upd.chat_id);

            let cmd = AdapterCommand {
                id,
                source:  AdapterSource::Telegram(upd.chat_id),
                payload,
            };
            if command_tx.send(cmd).await.is_err() {
                return; // tick_loop shut down
            }
        }
    }
}

// ─── Task 2: notify ───────────────────────────────────────────────────────────

async fn notify_task(
    config:      Arc<TelegramConfig>,
    mut rx:      broadcast::Receiver<ServerMessage>,
    pending:     Arc<Mutex<HashMap<String, i64>>>,
) {
    let client = reqwest::Client::new();
    loop {
        match rx.recv().await {
            Ok(msg) => {
                if let Some((id, text)) = extract_response(&msg) {
                    let chat_id = pending.lock().unwrap().remove(&id);
                    if let Some(cid) = chat_id {
                        let _ = send_message(&client, &config.token, cid, &text).await;
                    }
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => {}
            Err(broadcast::error::RecvError::Closed)    => return,
        }
    }
}

/// Извлечь (command_id, отображаемый текст) из ServerMessage для отправки в Telegram.
fn extract_response(msg: &ServerMessage) -> Option<(String, String)> {
    match msg {
        ServerMessage::CommandResult { command_id, output } => {
            Some((command_id.clone(), output.trim().to_string()))
        }
        ServerMessage::Result {
            command_id, domain_name, coherence, traces_matched, position, path, reflex_hit, ..
        } => {
            let [x, y, z] = position;
            let reflex = if *reflex_hit { " ⚡" } else { "" };
            let text = format!(
                "[{}{}] → {} | coh={:.2} matched={} pos=({},{},{})",
                path, reflex, domain_name, coherence, traces_matched, x, y, z
            );
            Some((command_id.clone(), text))
        }
        ServerMessage::Error { command_id: Some(id), message } => {
            Some((id.clone(), format!("error: {message}")))
        }
        _ => None,
    }
}

async fn send_message(
    client:  &reqwest::Client,
    token:   &str,
    chat_id: i64,
    text:    &str,
) -> Result<(), reqwest::Error> {
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");
    client.post(&url)
        .json(&serde_json::json!({ "chat_id": chat_id, "text": text }))
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_text_update() {
        let json = r#"{"ok":true,"result":[{
            "update_id":101,
            "message":{"text":"hello","chat":{"id":42},"from":{"id":7}}
        }]}"#;
        let updates = parse_updates(json);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].update_id, 101);
        assert_eq!(updates[0].chat_id, 42);
        assert_eq!(updates[0].user_id, 7);
        assert_eq!(updates[0].text, "hello");
    }

    #[test]
    fn parse_empty_result() {
        let updates = parse_updates(r#"{"ok":true,"result":[]}"#);
        assert!(updates.is_empty());
    }

    #[test]
    fn parse_invalid_json() {
        let updates = parse_updates("not json");
        assert!(updates.is_empty());
    }

    #[test]
    fn route_plain_text_to_inject() {
        let p = route_message("hello world").unwrap();
        assert!(matches!(p, AdapterPayload::Inject { text } if text == "hello world"));
    }

    #[test]
    fn route_status_command() {
        let p = route_message("/status").unwrap();
        assert!(matches!(p, AdapterPayload::MetaRead { cmd } if cmd == ":status"));
    }

    #[test]
    fn route_domains_command() {
        let p = route_message("/domains").unwrap();
        assert!(matches!(p, AdapterPayload::MetaRead { cmd } if cmd == ":domains"));
    }

    #[test]
    fn route_meta_read() {
        let p = route_message(":status").unwrap();
        assert!(matches!(p, AdapterPayload::MetaRead { cmd } if cmd == ":status"));
    }

    #[test]
    fn route_meta_mutate() {
        let p = route_message(":save").unwrap();
        assert!(matches!(p, AdapterPayload::MetaMutate { cmd } if cmd == ":save"));
    }

    #[test]
    fn route_start_returns_none() {
        assert!(route_message("/start").is_none());
    }

    #[test]
    fn route_unknown_bot_command_returns_none() {
        assert!(route_message("/unknown_cmd").is_none());
    }

    #[test]
    fn extract_command_result() {
        let msg = ServerMessage::CommandResult {
            command_id: "tg1".to_string(),
            output:     "ok\n".to_string(),
        };
        let (id, text) = extract_response(&msg).unwrap();
        assert_eq!(id, "tg1");
        assert_eq!(text, "ok");
    }

    #[test]
    fn extract_result_message() {
        let msg = ServerMessage::Result {
            command_id:     "tg2".to_string(),
            path:           "Direct".to_string(),
            domain_id:      100,
            domain_name:    "SUTRA".to_string(),
            coherence:      0.85,
            reflex_hit:     false,
            traces_matched: 3,
            position:       [1, 2, 3],
            shell:          [0; 8],
            event_id:       0,
        };
        let (id, text) = extract_response(&msg).unwrap();
        assert_eq!(id, "tg2");
        assert!(text.contains("SUTRA"));
        assert!(text.contains("0.85"));
    }

    #[test]
    fn extract_ignores_tick_state() {
        let tick = ServerMessage::Tick {
            tick_count: 1, traces: 0, tension: 0, last_matched: 0,
        };
        assert!(extract_response(&tick).is_none());
    }

    #[test]
    fn allowed_users_filter_logic() {
        let allowed = vec![10i64, 20i64];
        assert!(allowed.contains(&10));
        assert!(!allowed.contains(&99));
        // empty list = allow all
        let empty: Vec<i64> = vec![];
        assert!(empty.is_empty());
    }
}
