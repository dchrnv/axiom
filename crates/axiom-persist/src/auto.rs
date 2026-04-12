// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AutoSaver — автосохранение по интервалу тиков.
//
// Принцип: axiom-runtime не знает про persist. AutoSaver живёт на стороне
// axiom-agent и вызывается из tick loop после каждого TickForward.
//
// data_dir не хранится в AutoSaver — передаётся при каждом вызове tick/force_save.
// Единственный источник правды для пути — CliConfig.data_dir в вызывающем коде.

use std::path::Path;
use schemars::JsonSchema;
use axiom_runtime::AxiomEngine;
use crate::error::PersistError;
use crate::writer::{save, WriteOptions};

/// Конфигурация автосохранения.
#[derive(Debug, Clone, JsonSchema)]
pub struct PersistenceConfig {
    /// Автосохранение включено
    pub enabled: bool,
    /// Каждые N тиков проверять необходимость сохранения (0 = отключено)
    pub interval_ticks: u32,
    /// Минимальный weight trace для сохранения
    pub trace_weight_threshold: f32,
}

impl PersistenceConfig {
    pub fn new(interval_ticks: u32) -> Self {
        Self {
            enabled: interval_ticks > 0,
            interval_ticks,
            trace_weight_threshold: 0.0,
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            interval_ticks: 0,
            trace_weight_threshold: 0.0,
        }
    }
}

/// Автосохранение состояния Engine по интервалу тиков.
///
/// Вызывается из tick loop после каждого `TickForward`.
/// Не вызывает I/O если интервал не наступил.
///
/// `data_dir` не хранится здесь — передаётся при вызове [`tick`] и [`force_save`].
pub struct AutoSaver {
    pub config: PersistenceConfig,
    /// tick_count на момент последнего успешного сохранения
    last_save_tick: u64,
    /// Счётчик автосохранений (диагностика)
    pub save_count: u64,
    /// Последняя ошибка автосохранения (для :autosave status)
    pub last_error: Option<String>,
}

impl AutoSaver {
    pub fn new(config: PersistenceConfig) -> Self {
        Self {
            config,
            last_save_tick: 0,
            save_count: 0,
            last_error: None,
        }
    }

    /// Включить/выключить автосохранение.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }

    /// Установить интервал (0 = отключить).
    pub fn set_interval(&mut self, ticks: u32) {
        self.config.interval_ticks = ticks;
        self.config.enabled = ticks > 0;
    }

    /// Проверить нужно ли сохранять прямо сейчас.
    pub fn should_save(&self, engine: &AxiomEngine) -> bool {
        if !self.config.enabled || self.config.interval_ticks == 0 {
            return false;
        }
        engine.tick_count > 0
            && engine.tick_count >= self.last_save_tick + self.config.interval_ticks as u64
    }

    /// Попытаться сохранить если нужно. Возвращает true если сохранение выполнено.
    ///
    /// Ошибки записываются в `self.last_error`, не паникуют.
    pub fn tick(&mut self, engine: &AxiomEngine, data_dir: &Path) -> bool {
        if !self.should_save(engine) {
            return false;
        }
        let opts = WriteOptions {
            trace_weight_threshold: self.config.trace_weight_threshold,
        };
        match save(engine, data_dir, &opts) {
            Ok(_) => {
                self.last_save_tick = engine.tick_count;
                self.save_count += 1;
                self.last_error = None;
                true
            }
            Err(e) => {
                self.last_error = Some(e.to_string());
                false
            }
        }
    }

    /// Принудительное сохранение (например при :quit).
    pub fn force_save(&mut self, engine: &AxiomEngine, data_dir: &Path) -> Result<(), PersistError> {
        let opts = WriteOptions {
            trace_weight_threshold: self.config.trace_weight_threshold,
        };
        save(engine, data_dir, &opts).map(|m| {
            self.last_save_tick = m.tick_count;
            self.save_count += 1;
            self.last_error = None;
        })
    }

    /// Сбросить last_save_tick (вызывается после :load).
    pub fn reset_save_tick(&mut self, tick: u64) {
        self.last_save_tick = tick;
    }

    /// Статус для :autosave status.
    pub fn status_line(&self, data_dir: &Path) -> String {
        if !self.config.enabled || self.config.interval_ticks == 0 {
            return "  autosave: off".to_string();
        }
        let mut s = format!(
            "  autosave: on  interval={} ticks  dir={}  saves={}  last_save_tick={}",
            self.config.interval_ticks,
            data_dir.display(),
            self.save_count,
            self.last_save_tick,
        );
        if let Some(e) = &self.last_error {
            s.push_str(&format!("\n  last error: {e}"));
        }
        s
    }
}
