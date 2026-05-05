// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Конфигурация tick_loop и всех адаптеров.
// Собирается из CliConfig при старте; в будущем — из axiom-cli.yaml.

use crate::channels::cli::CliConfig;
use crate::effectors::message::DetailLevel;
use axiom_runtime::TickSchedule;

/// Конфигурация WebSocket-адаптера.
pub struct WebSocketConfig {
    /// Включён ли WebSocket (Phase 1)
    pub enabled: bool,
    /// Порт WebSocket-сервера
    pub port: u16,
    /// Отправлять Tick-broadcast каждые N тиков
    pub tick_broadcast_interval: u32,
    /// Обновлять State-snapshot каждые N тиков
    pub state_broadcast_interval: u32,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 8765,
            tick_broadcast_interval: 1,
            state_broadcast_interval: 100,
        }
    }
}

/// Конфигурация tick_loop и адаптеров.
pub struct AdaptersConfig {
    /// Целевая частота тиков
    pub tick_hz: u32,
    /// Директория хранилища для autosave
    pub data_dir: String,
    /// Расписание периодических задач
    pub tick_schedule: TickSchedule,
    /// Конфигурация WebSocket-адаптера
    pub websocket: WebSocketConfig,
    /// Telegram Bot API токен (Some → включён, None → выключен)
    pub telegram_token: Option<String>,
    /// Разрешённые Telegram user_id (пустой = все)
    pub telegram_allowed: Vec<i64>,
    /// OpenSearch URL (Some → включён, None → выключен)
    pub opensearch_url: Option<String>,
    /// Имя индекса OpenSearch (default: "axiom-events")
    pub opensearch_index: String,
    /// Индексировать Tick каждые N тиков (0 = не индексировать)
    pub opensearch_tick_interval: u64,

    pub verbose: bool,
    pub detail_level: DetailLevel,
    pub adaptive_tick_rate: bool,
}

impl AdaptersConfig {
    /// Построить конфигурацию адаптеров из CLI-конфига.
    pub fn from_cli_config(c: &CliConfig) -> Self {
        Self {
            tick_hz: c.tick_hz,
            data_dir: c.data_dir.clone(),
            tick_schedule: c.tick_schedule.clone(),
            websocket: WebSocketConfig {
                enabled: c.ws_enabled,
                port: c.ws_port,
                ..WebSocketConfig::default()
            },
            telegram_token: c.telegram_token.clone(),
            telegram_allowed: c.telegram_allowed.clone(),
            opensearch_url: c.opensearch_url.clone(),
            opensearch_index: c.opensearch_index.clone(),
            opensearch_tick_interval: c.opensearch_tick_interval,
            verbose: c.verbose,
            detail_level: c.detail_level,
            adaptive_tick_rate: c.adaptive_tick_rate,
        }
    }
}
