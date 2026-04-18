// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Команды от адаптеров в tick_loop и ответы tick_loop адаптерам.
// Единый канал — mpsc::Sender<AdapterCommand>.

use crate::protocol::ServerMessage;

/// Команда от любого адаптера в tick loop.
pub struct AdapterCommand {
    /// UUID-подобный ID для корреляции ответа (пустая строка допустима для CLI)
    pub id:      String,
    /// Источник команды
    pub source:  AdapterSource,
    /// Содержимое команды
    pub payload: AdapterPayload,
}

/// Источник команды.
#[derive(Debug, Clone)]
pub enum AdapterSource {
    /// stdin/stdout CLI
    Cli,
    /// WebSocket-соединение (connection_id)
    WebSocket(u64),
    /// REST API
    Rest,
    /// Telegram (chat_id)
    Telegram(i64),
}

/// Тип команды от адаптера.
pub enum AdapterPayload {
    /// Текстовый ввод → InjectToken → Engine
    Inject         { text: String },
    /// Мета-команда только для чтения (:status, :domains, ...)
    MetaRead       { cmd: String },
    /// Мутирующая мета-команда (:save, :load, :quit, ...)
    MetaMutate     { cmd: String },
    /// Подписаться на каналы broadcast (ticks / state / ...)
    Subscribe      { channels: Vec<String> },
    /// Отписаться от каналов broadcast
    Unsubscribe    { channels: Vec<String> },
    /// Запросить детальный снапшот домена
    DomainSnapshot { domain_id: u16 },
}

impl AdapterCommand {
    /// Команда graceful shutdown — для SIGTERM из любого источника.
    pub fn shutdown() -> Self {
        Self {
            id:      "shutdown".to_string(),
            source:  AdapterSource::Cli,
            payload: AdapterPayload::MetaMutate { cmd: ":quit".to_string() },
        }
    }
}

/// Результат обработки одной AdapterCommand в tick loop.
pub enum CommandResponse {
    /// Готово к отправке через broadcast_tx
    Message(ServerMessage),
    /// :quit → автосохранение → выход из tick loop
    Quit,
    /// Subscribe/Unsubscribe — обработано на уровне адаптера, нет ответа
    None,
}
