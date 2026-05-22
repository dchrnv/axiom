// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NodeConfig — конфигурация axiom-node.
// Загружается из CLI-аргументов; значения по умолчанию обеспечивают
// работу без явной конфигурации ("zero-config start").

use clap::Parser;
use std::path::PathBuf;

/// Живой когнитивный движок Axiom с WebSocket-интерфейсом для Workstation.
#[derive(Parser, Debug)]
#[command(name = "axiom-node", version, about)]
pub struct NodeConfig {
    /// Адрес WebSocket-сервера (Workstation подключается сюда)
    #[arg(long, default_value = "127.0.0.1:9876")]
    pub addr: String,

    /// Директория хранилища: persist, логи
    #[arg(long, default_value = "data")]
    pub data_dir: PathBuf,

    /// Путь к axiom.yaml (Genome, DomainConfig, DreamConfig)
    #[arg(long, default_value = "config/axiom.yaml")]
    pub axiom_yaml: PathBuf,

    /// Директория якорных YAML-файлов
    #[arg(long, default_value = "config/anchors")]
    pub anchors_dir: PathBuf,

    /// Путь к meta_primitives.yaml для MetaDetector
    #[arg(long, default_value = "config/meta_primitives.yaml")]
    pub meta_primitives_yaml: PathBuf,

    /// Целевая частота тиков (Hz)
    #[arg(long, default_value_t = 60)]
    pub tick_hz: u32,

    /// Публиковать Tick-событие каждые N тиков (0 = выключено)
    #[arg(long, default_value_t = 1)]
    pub tick_interval: u32,

    /// Обновлять SystemSnapshot каждые N тиков
    #[arg(long, default_value_t = 20)]
    pub snapshot_interval: u32,

    /// Уровень логирования (trace/debug/info/warn/error)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Отключить адаптивный tick rate (фиксированный tick_hz)
    #[arg(long, default_value_t = false)]
    pub fixed_tick: bool,
}

impl NodeConfig {
    pub fn adaptive_tick(&self) -> bool {
        !self.fixed_tick
    }
}
