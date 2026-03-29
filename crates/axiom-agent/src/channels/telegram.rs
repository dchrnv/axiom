// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Telegram Channel — Telegram Bot API транспорт для AXIOM Agent
//
// TelegramPerceptor: polling Telegram Bot API, сообщение → UclCommand.
// TelegramEffector: отправка ответа в чат при событии MAYA.
// Логика парсинга отделена от HTTP — тестируется без сети.

use axiom_core::Event;
use axiom_ucl::{UclCommand, UclResult};
use axiom_runtime::{Perceptor, Effector};

/// Конфигурация Telegram-канала.
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    /// Bot API токен (из BotFather)
    pub token: String,
    /// Chat ID для отправки ответов
    pub chat_id: i64,
}

/// Входящее сообщение Telegram (минимальная структура).
#[derive(Debug, Clone)]
pub struct TelegramUpdate {
    /// ID апдейта (для пагинации polling)
    pub update_id: u64,
    /// Текст сообщения
    pub text: String,
    /// ID чата-отправителя
    pub chat_id: i64,
}

/// Разобрать JSON ответа Telegram Bot API в список апдейтов.
///
/// Тестируется отдельно от HTTP — принимает `&str`.
///
/// # Формат входного JSON
/// ```json
/// {"ok": true, "result": [{"update_id": 1, "message": {"text": "tick", "chat": {"id": 123}}}]}
/// ```
pub fn parse_updates(json: &str) -> Vec<TelegramUpdate> {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(json) else {
        return Vec::new();
    };
    let Some(results) = val.get("result").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    results
        .iter()
        .filter_map(|u| {
            let update_id = u.get("update_id")?.as_u64()?;
            let message = u.get("message")?;
            let text = message.get("text")?.as_str()?.to_string();
            let chat_id = message.get("chat")?.get("id")?.as_i64()?;
            Some(TelegramUpdate { update_id, text, chat_id })
        })
        .collect()
}

/// Преобразовать текст Telegram-сообщения в UclCommand.
///
/// Синтаксис команд идентичен CLI:
/// - `tick` → `TickForward`
/// - `inject <domain_id>` → `InjectToken`
/// Неизвестные тексты → `None`.
pub fn message_to_command(text: &str) -> Option<UclCommand> {
    crate::channels::cli::parse_cli_command(text)
}

/// Входящий Telegram-адаптер: polling Bot API → UclCommand.
///
/// В тестах используйте `parse_updates` + `message_to_command` напрямую.
pub struct TelegramPerceptor {
    config: TelegramConfig,
    last_update_id: u64,
    pending: std::collections::VecDeque<UclCommand>,
}

impl TelegramPerceptor {
    /// Создать перцептор с указанной конфигурацией.
    pub fn new(config: TelegramConfig) -> Self {
        Self {
            config,
            last_update_id: 0,
            pending: std::collections::VecDeque::new(),
        }
    }

    /// Опросить Telegram Bot API и добавить команды в очередь.
    ///
    /// Использует блокирующий reqwest. В тестах не вызывайте этот метод —
    /// используйте `feed_update` для инжекции тестовых данных.
    pub fn poll_blocking(&mut self) -> Result<(), String> {
        let url = format!(
            "https://api.telegram.org/bot{}/getUpdates?offset={}&timeout=0",
            self.config.token,
            self.last_update_id + 1
        );
        let body = reqwest::blocking::get(&url)
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;

        let updates = parse_updates(&body);
        for update in updates {
            if update.update_id > self.last_update_id {
                self.last_update_id = update.update_id;
            }
            if let Some(cmd) = message_to_command(&update.text) {
                self.pending.push_back(cmd);
            }
        }
        Ok(())
    }

    /// Инжектировать апдейт напрямую (для тестов без HTTP).
    pub fn feed_update(&mut self, update: TelegramUpdate) {
        if update.update_id > self.last_update_id {
            self.last_update_id = update.update_id;
        }
        if let Some(cmd) = message_to_command(&update.text) {
            self.pending.push_back(cmd);
        }
    }

    /// Число команд в очереди.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

impl Perceptor for TelegramPerceptor {
    fn receive(&mut self) -> Option<UclCommand> {
        self.pending.pop_front()
    }

    fn name(&self) -> &str {
        "telegram"
    }
}

/// Исходящий Telegram-адаптер: события MAYA → ответ в чат.
pub struct TelegramEffector {
    config: TelegramConfig,
    /// Собранные сообщения (для тестов без HTTP)
    pub sent_messages: Vec<String>,
    /// Отправлять через реальный API?
    send_enabled: bool,
}

impl TelegramEffector {
    /// Создать эффектор в режиме mock (без реальных HTTP запросов).
    pub fn mock(config: TelegramConfig) -> Self {
        Self {
            config,
            sent_messages: Vec::new(),
            send_enabled: false,
        }
    }

    /// Создать эффектор с реальными HTTP запросами.
    pub fn new(config: TelegramConfig) -> Self {
        Self {
            config,
            sent_messages: Vec::new(),
            send_enabled: true,
        }
    }

    fn format_event(event: &Event) -> String {
        format!(
            "AXIOM event: type={:#06x} domain={} token={}",
            event.event_type, event.domain_id, event.target_id
        )
    }

    fn send(&mut self, text: String) {
        self.sent_messages.push(text.clone());
        if self.send_enabled {
            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                self.config.token
            );
            let _ = reqwest::blocking::Client::new()
                .post(&url)
                .json(&serde_json::json!({
                    "chat_id": self.config.chat_id,
                    "text": text
                }))
                .send();
        }
    }
}

impl Effector for TelegramEffector {
    fn emit(&mut self, event: &Event) {
        let msg = Self::format_event(event);
        self.send(msg);
    }

    fn emit_result(&mut self, result: &UclResult) {
        let status = if result.is_success() { "OK" } else { "ERR" };
        let msg = format!("AXIOM result: {} code={}", status, result.error_code);
        self.send(msg);
    }

    fn name(&self) -> &str {
        "telegram"
    }
}
