// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Heartbeat V2.0: docs/spec/time/Heartbeat_V2_0.md
// Периодическая активация фоновых процессов через причинный порядок

use axiom_core::event::{Event, EventPriority, EventType};
use axiom_frontier::CausalFrontier;

/// Конфигурация Heartbeat для домена
///
/// Heartbeat V2.0, раздел 7: Configuration
#[derive(Clone, Copy, Debug)]
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

    /// Активировать Internal Drive — остывание и слив tension traces (Cognitive Depth V1.0)
    /// При каждом пульсе EXPERIENCE остужает следы напряжения и возвращает горячие
    /// в pipeline для повторной обработки.
    pub enable_internal_drive: bool,
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
            enable_internal_drive: false, // Отключено на слабом железе
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
            enable_internal_drive: true, // Включено на среднем+ железе
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
            enable_internal_drive: true, // Включено на мощном железе
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
            enable_internal_drive: false, // Отключено когда heartbeat отключён
        }
    }
}

/// Генератор Heartbeat событий
///
/// Heartbeat V2.0, раздел 3.1: generation by event count
pub struct HeartbeatGenerator {
    /// Количество событий между пульсами
    interval: u32,

    /// Счётчик событий с последнего Heartbeat
    events_since_last_heartbeat: u32,

    /// Монотонный номер пульса
    pulse_number: u64,

    /// ID домена для которого генерируется Heartbeat
    domain_id: u16,
}

impl HeartbeatGenerator {
    /// Создаёт новый генератор для домена
    pub fn new(domain_id: u16, interval: u32) -> Self {
        Self {
            interval,
            events_since_last_heartbeat: 0,
            pulse_number: 0,
            domain_id,
        }
    }

    /// Уведомление о новом событии в домене
    ///
    /// Heartbeat V2.0, раздел 3.1: on_event triggers pulse
    /// Возвращает pulse_number если пора генерировать Heartbeat
    pub fn on_event(&mut self) -> Option<u64> {
        self.events_since_last_heartbeat += 1;

        if self.events_since_last_heartbeat >= self.interval {
            self.events_since_last_heartbeat = 0;
            self.pulse_number += 1;
            Some(self.pulse_number)
        } else {
            None
        }
    }

    /// Создаёт Heartbeat Event для COM
    ///
    /// Heartbeat V2.0, раздел 4: Event structure
    pub fn create_heartbeat_event(&self, event_id: u64, pulse_number: u64) -> Event {
        Event::with_pulse(
            event_id,
            self.domain_id,
            EventType::Heartbeat,
            EventPriority::Low,
            self.compute_pulse_hash(pulse_number),
            0,                          // target_id не используется для Heartbeat
            0,                          // source_id не используется для Heartbeat
            event_id.saturating_sub(1), // parent_event_id
            pulse_number,
        )
    }

    /// Вычисляет детерминированный hash для pulse_number
    fn compute_pulse_hash(&self, pulse_number: u64) -> u64 {
        let mut hash = self.domain_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(pulse_number);
        hash = hash.wrapping_mul(31).wrapping_add(self.interval as u64);
        hash
    }

    /// Получает текущий номер пульса
    pub fn current_pulse(&self) -> u64 {
        self.pulse_number
    }

    /// Сбрасывает генератор (для тестирования)
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.events_since_last_heartbeat = 0;
        self.pulse_number = 0;
    }
}

/// Обработчик Heartbeat события
///
/// Heartbeat V2.0, раздел 6: processing
pub fn handle_heartbeat(
    frontier: &mut CausalFrontier,
    pulse_number: u64,
    config: &HeartbeatConfig,
    total_tokens: usize,
    total_connections: usize,
) {
    // Добавляем токены в frontier для обслуживания
    if config.batch_size > 0 && total_tokens > 0 {
        for i in 0..config.batch_size {
            let token_idx = ((pulse_number as usize) * config.batch_size + i) % total_tokens;
            frontier.push_token(token_idx as u32);
        }
    }

    // Добавляем связи если включено обслуживание
    if config.enable_connection_maintenance
        && config.connection_batch_size > 0
        && total_connections > 0
    {
        for i in 0..config.connection_batch_size {
            let conn_idx =
                ((pulse_number as usize) * config.connection_batch_size + i) % total_connections;
            frontier.push_connection(conn_idx as u32);
        }
    }
}
