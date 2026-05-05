// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Протокол обмена между tick_loop и адаптерами (CLI, WebSocket, REST, Telegram).
// Полный WebSocket protocol (ClientMessage, serde tag и пр.) — Phase 1.

use axiom_runtime::{BroadcastSnapshot, DomainDetailSnapshot};
use serde::Serialize;

/// Сообщение от tick_loop к любому адаптеру.
///
/// CLI-подписчик печатает CommandResult/Result в stdout.
/// WebSocket-подписчик сериализует в JSON и отправляет клиенту.
#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    /// Результат inject-команды (текстовый ввод → Engine).
    #[serde(rename = "result")]
    Result {
        /// ID команды для корреляции ответа
        command_id: String,
        /// Путь обработки (Direct / MultiPass / ...)
        path: String,
        /// Доминирующий домен
        domain_id: u16,
        /// Имя доминирующего домена
        domain_name: String,
        /// Когерентность обработки
        coherence: f32,
        /// Был ли рефлекторный ответ
        reflex_hit: bool,
        /// Число совпавших опытных следов
        traces_matched: u32,
        /// Выходная позиция в семантическом пространстве
        position: [i16; 3],
        /// Выходной семантический профиль
        shell: [u8; 8],
        /// COM event_id
        event_id: u64,
    },

    /// Периодический тик-пульс (каждые tick_broadcast_interval тиков).
    #[serde(rename = "tick")]
    Tick {
        tick_count: u64,
        traces: u32,
        tension: u32,
        last_matched: u32,
    },

    /// Полный broadcast-снапшот состояния Engine.
    #[serde(rename = "state")]
    State {
        tick_count: u64,
        snapshot: BroadcastSnapshot,
    },

    /// Ответ на MetaRead/MetaMutate команду — уже отформатированный текст.
    #[serde(rename = "command_result")]
    CommandResult { command_id: String, output: String },

    /// Детальный снапшот одного домена (по запросу DomainSnapshot).
    #[serde(rename = "domain_detail")]
    DomainDetail(DomainDetailSnapshot),

    /// Ошибка обработки команды.
    #[serde(rename = "error")]
    Error {
        command_id: Option<String>,
        message: String,
    },
}
