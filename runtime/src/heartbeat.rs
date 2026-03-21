// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Heartbeat V2.0: docs/spec/time/Heartbeat_V2_0.md
// Периодическая активация фоновых процессов через причинный порядок

use crate::event::{Event, EventType, EventPriority};
use crate::causal_frontier::CausalFrontier;

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
            enable_shell_reconciliation: false, // Disabled for weak hardware
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
            enable_shell_reconciliation: true, // Enabled for medium+ hardware
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
            enable_shell_reconciliation: true, // Enabled for medium+ hardware
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
            enable_shell_reconciliation: false, // Disabled when heartbeat disabled
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
            0, // target_id не используется для Heartbeat
            0, // source_id не используется для Heartbeat
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
            frontier.push_token(token_idx);
        }
    }

    // Добавляем связи если включено обслуживание
    if config.enable_connection_maintenance && config.connection_batch_size > 0 && total_connections > 0 {
        for i in 0..config.connection_batch_size {
            let conn_idx = ((pulse_number as usize) * config.connection_batch_size + i) % total_connections;
            frontier.push_connection(conn_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_config_presets() {
        let weak = HeartbeatConfig::weak();
        assert_eq!(weak.interval, 10000);
        assert_eq!(weak.batch_size, 1);
        assert!(!weak.enable_gravity);

        let medium = HeartbeatConfig::medium();
        assert_eq!(medium.interval, 1024);
        assert_eq!(medium.batch_size, 10);
        assert!(medium.enable_gravity);

        let powerful = HeartbeatConfig::powerful();
        assert_eq!(powerful.interval, 256);
        assert_eq!(powerful.batch_size, 50);
        assert!(powerful.enable_thermodynamics);

        let disabled = HeartbeatConfig::disabled();
        assert_eq!(disabled.interval, u32::MAX);
        assert_eq!(disabled.batch_size, 0);
    }

    #[test]
    fn test_heartbeat_generator_creation() {
        let generator = HeartbeatGenerator::new(1, 100);
        assert_eq!(generator.current_pulse(), 0);
        assert_eq!(generator.domain_id, 1);
    }

    #[test]
    fn test_heartbeat_generation_by_event_count() {
        let mut generator = HeartbeatGenerator::new(1, 5);

        // Первые 4 события - нет пульса
        assert!(generator.on_event().is_none());
        assert!(generator.on_event().is_none());
        assert!(generator.on_event().is_none());
        assert!(generator.on_event().is_none());

        // 5-е событие - первый пульс
        assert_eq!(generator.on_event(), Some(1));
        assert_eq!(generator.current_pulse(), 1);

        // Ещё 5 событий - второй пульс
        for _ in 0..4 {
            assert!(generator.on_event().is_none());
        }
        assert_eq!(generator.on_event(), Some(2));
        assert_eq!(generator.current_pulse(), 2);
    }

    #[test]
    fn test_heartbeat_determinism() {
        let mut gen1 = HeartbeatGenerator::new(1, 10);
        let mut gen2 = HeartbeatGenerator::new(1, 10);

        // Обрабатываем одинаковое количество событий
        for _ in 0..25 {
            let pulse1 = gen1.on_event();
            let pulse2 = gen2.on_event();
            assert_eq!(pulse1, pulse2);
        }

        assert_eq!(gen1.current_pulse(), gen2.current_pulse());
    }

    #[test]
    fn test_heartbeat_event_creation() {
        let generator = HeartbeatGenerator::new(5, 100);
        let event = generator.create_heartbeat_event(1000, 42);

        assert_eq!(event.event_id, 1000);
        assert_eq!(event.domain_id, 5);
        assert_eq!(event.event_type, EventType::Heartbeat as u16);
        assert_eq!(event.pulse_id, 42);
        assert_eq!(event.priority, EventPriority::Low as u8);
    }

    #[test]
    fn test_handle_heartbeat_tokens() {
        let mut frontier = CausalFrontier::with_config(100, 1000, 100);
        let config = HeartbeatConfig::medium();

        handle_heartbeat(&mut frontier, 1, &config, 100, 50);

        // Должно добавить batch_size токенов
        assert_eq!(frontier.token_count(), config.batch_size);
    }

    #[test]
    fn test_handle_heartbeat_connections() {
        let mut frontier = CausalFrontier::with_config(100, 1000, 100);
        let mut config = HeartbeatConfig::medium();
        config.enable_connection_maintenance = true;

        handle_heartbeat(&mut frontier, 1, &config, 100, 50);

        assert_eq!(frontier.connection_count(), config.connection_batch_size);
    }

    #[test]
    fn test_handle_heartbeat_deterministic_selection() {
        let mut frontier = CausalFrontier::with_config(100, 1000, 100);
        let config = HeartbeatConfig {
            batch_size: 3,
            ..HeartbeatConfig::medium()
        };

        // Пульс 0 должен выбрать токены 0, 1, 2
        handle_heartbeat(&mut frontier, 0, &config, 10, 0);

        assert!(frontier.contains_token(0));
        assert!(frontier.contains_token(1));
        assert!(frontier.contains_token(2));

        frontier.clear();

        // Пульс 1 должен выбрать токены 3, 4, 5
        handle_heartbeat(&mut frontier, 1, &config, 10, 0);

        assert!(frontier.contains_token(3));
        assert!(frontier.contains_token(4));
        assert!(frontier.contains_token(5));
    }

    #[test]
    fn test_handle_heartbeat_wraparound() {
        let mut frontier = CausalFrontier::with_config(100, 1000, 100);
        let config = HeartbeatConfig {
            batch_size: 5,
            ..HeartbeatConfig::medium()
        };

        // total_tokens = 10, batch_size = 5
        // Пульс 2 должен обернуть индексы: 10 % 10 = 0
        handle_heartbeat(&mut frontier, 2, &config, 10, 0);

        // Должны быть добавлены токены 0, 1, 2, 3, 4 (wraparound)
        assert!(frontier.contains_token(0));
        assert!(frontier.contains_token(1));
        assert!(frontier.contains_token(2));
    }

    #[test]
    fn test_heartbeat_full_coverage() {
        let mut frontier = CausalFrontier::with_config(100, 1000, 100);
        let config = HeartbeatConfig {
            batch_size: 3,
            ..HeartbeatConfig::medium()
        };

        let total_tokens = 10;
        let pulses_needed = (total_tokens + config.batch_size - 1) / config.batch_size;

        // За ceil(10/3) = 4 пульса должны быть покрыты все токены
        for pulse in 1..=pulses_needed {
            handle_heartbeat(&mut frontier, pulse as u64, &config, total_tokens, 0);
        }

        // Проверяем что все токены были добавлены
        for i in 0..total_tokens {
            assert!(frontier.contains_token(i), "Token {} not covered", i);
        }
    }

    #[test]
    fn test_heartbeat_idle_state() {
        let mut generator = HeartbeatGenerator::new(1, 1000);

        // Если нет событий, нет пульсов
        assert_eq!(generator.current_pulse(), 0);

        // Heartbeat V2.0, раздел 11: no events → no heartbeat → idle
        // Система находится в idle состоянии
    }
}
