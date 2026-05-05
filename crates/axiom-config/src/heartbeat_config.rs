//! Heartbeat Configuration
//!
//! Heartbeat V2.0: Периодическая активация фоновых процессов через причинный порядок

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Конфигурация Heartbeat для домена
///
/// Heartbeat V2.0, раздел 7: Configuration
#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeartbeatConfig {
    /// Количество событий между пульсами
    pub interval: u32,

    /// Токенов добавляется в frontier за один пульс
    pub batch_size: usize,

    /// Связей добавляется в frontier за один пульс
    pub connection_batch_size: usize,

    /// Активировать затухание токенов
    pub enable_decay: bool,

    /// Активировать гравитационные обновления
    pub enable_gravity: bool,

    /// Активировать пространственные проверки столкновений (SPACE V6.0)
    pub enable_spatial_collision: bool,

    /// Активировать обслуживание связей
    pub enable_connection_maintenance: bool,

    /// Активировать термодинамические процессы
    pub enable_thermodynamics: bool,

    /// Добавлять pulse_id к генерируемым событиям
    pub attach_pulse_id: bool,

    /// Активировать Shell reconciliation (Shell V3.0 Phase 2.7)
    /// Пересчёт и проверка семантических профилей токенов
    pub enable_shell_reconciliation: bool,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self::medium()
    }
}

impl HeartbeatConfig {
    /// Конфигурация для слабого оборудования (минимальная нагрузка)
    ///
    /// Heartbeat V2.0, раздел 7.1: weak hardware preset
    pub fn weak() -> Self {
        Self {
            interval: 10000,
            batch_size: 1,
            connection_batch_size: 1,
            enable_decay: true,
            enable_gravity: false,
            enable_spatial_collision: false,
            enable_connection_maintenance: false,
            enable_thermodynamics: false,
            attach_pulse_id: false,
            enable_shell_reconciliation: false,
        }
    }

    /// Конфигурация для среднего оборудования
    ///
    /// Heartbeat V2.0, раздел 7.1: medium hardware preset
    pub fn medium() -> Self {
        Self {
            interval: 1024,
            batch_size: 10,
            connection_batch_size: 5,
            enable_decay: true,
            enable_gravity: true,
            enable_spatial_collision: true,
            enable_connection_maintenance: true,
            enable_thermodynamics: true,
            attach_pulse_id: true,
            enable_shell_reconciliation: true,
        }
    }

    /// Конфигурация для мощного сервера
    ///
    /// Heartbeat V2.0, раздел 7.1: powerful hardware preset
    pub fn powerful() -> Self {
        Self {
            interval: 256,
            batch_size: 50,
            connection_batch_size: 25,
            enable_decay: true,
            enable_gravity: true,
            enable_spatial_collision: true,
            enable_connection_maintenance: true,
            enable_thermodynamics: true,
            attach_pulse_id: true,
            enable_shell_reconciliation: true,
        }
    }

    /// Отключенный Heartbeat (для тестирования или статичных доменов)
    pub fn disabled() -> Self {
        Self {
            interval: u32::MAX,
            batch_size: 0,
            connection_batch_size: 0,
            enable_spatial_collision: false,
            enable_decay: false,
            enable_gravity: false,
            enable_connection_maintenance: false,
            enable_thermodynamics: false,
            attach_pulse_id: false,
            enable_shell_reconciliation: false,
        }
    }

    /// Валидация конфигурации
    pub fn validate(&self) -> Result<(), String> {
        if self.interval == 0 {
            return Err("interval must be > 0".to_string());
        }

        Ok(())
    }
}
