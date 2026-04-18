// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Конфигурация tick_loop и всех адаптеров.
// Собирается из CliConfig при старте; в будущем — из axiom-cli.yaml.

use axiom_runtime::TickSchedule;
use crate::channels::cli::CliConfig;

/// Конфигурация WebSocket-адаптера.
pub struct WebSocketConfig {
    /// Включён ли WebSocket (Phase 1)
    pub enabled:                   bool,
    /// Порт WebSocket-сервера
    pub port:                      u16,
    /// Отправлять Tick-broadcast каждые N тиков
    pub tick_broadcast_interval:   u32,
    /// Обновлять State-snapshot каждые N тиков
    pub state_broadcast_interval:  u32,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            enabled:                  false,
            port:                     8765,
            tick_broadcast_interval:  1,
            state_broadcast_interval: 100,
        }
    }
}

/// Конфигурация tick_loop и адаптеров.
///
/// В Phase 0C создаётся через `from_cli_config`. В Phase 1+ дополнится
/// WebSocket/REST/Telegram секциями из axiom-cli.yaml.
pub struct AdaptersConfig {
    /// Целевая частота тиков
    pub tick_hz:          u32,
    /// Директория хранилища для autosave
    pub data_dir:         String,
    /// Расписание периодических задач
    pub tick_schedule:    TickSchedule,
    /// Конфигурация WebSocket-адаптера
    pub websocket:        WebSocketConfig,
}

impl AdaptersConfig {
    /// Построить конфигурацию адаптеров из CLI-конфига.
    pub fn from_cli_config(c: &CliConfig) -> Self {
        Self {
            tick_hz:       c.tick_hz,
            data_dir:      c.data_dir.clone(),
            tick_schedule: c.tick_schedule,
            websocket:     WebSocketConfig::default(),
        }
    }
}
