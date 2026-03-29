// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AgentConfig — конфигурация axiom-agent из channels.yaml

use std::path::Path;
use serde::{Deserialize, Serialize};

/// Конфигурация всех каналов агента
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentConfig {
    /// Настройки каналов
    #[serde(default)]
    pub channels: ChannelsConfig,
}

/// Конфигурация отдельных каналов
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelsConfig {
    /// CLI-канал всегда включён по умолчанию
    #[serde(default = "default_true")]
    pub cli: bool,
    /// Telegram-канал (опционально)
    #[serde(default)]
    pub telegram: TelegramChannelConfig,
    /// Shell Effector (опционально)
    #[serde(default)]
    pub shell: ShellChannelConfig,
}

/// Конфигурация Telegram-канала
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelegramChannelConfig {
    /// Включить Telegram-канал
    #[serde(default)]
    pub enabled: bool,
    /// Bot API токен
    #[serde(default)]
    pub token: String,
    /// Chat ID для ответов
    #[serde(default)]
    pub chat_id: i64,
}

/// Конфигурация Shell Effector
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellChannelConfig {
    /// Включить Shell Effector
    #[serde(default)]
    pub enabled: bool,
    /// Путь к файлу белого списка
    #[serde(default)]
    pub whitelist: String,
}

fn default_true() -> bool { true }

impl AgentConfig {
    /// Загрузить конфигурацию из YAML файла.
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("read channels.yaml: {e}"))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("parse channels.yaml: {e}"))
    }
}
