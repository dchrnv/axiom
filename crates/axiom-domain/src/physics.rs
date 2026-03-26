// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Event-Driven V1: docs/spec/time/Event-Driven_V1.md
// Time Model V1.0: причинный возраст вместо wall-clock времени

use axiom_core::{Token, Connection, Event, EventType, EventPriority};

/// Порог затухания по умолчанию: 1/decay_rate событий причинного возраста.
/// Decay_rate = 0.001 → порог = 1000 событий.
/// Вынесено в DEFERRED.md §3.1 для будущей настройки через DomainConfig.
pub const DEFAULT_DECAY_RATE: f32 = 0.001;

/// Порог стресса связи по умолчанию (80% от максимума).
pub const DEFAULT_STRESS_THRESHOLD: f32 = 0.8;

/// Радиус обнаружения столкновений по умолчанию (Space V6.0).
/// DEFERRED.md §3.1: будет вынесен в DomainConfig/SpatialConfig.
pub const DEFAULT_COLLISION_RADIUS: i16 = 100;

/// EventGenerator — механизм обнаружения физических изменений и генерации событий.
///
/// Event-Driven V1, раздел 6: "Генерация событий"
/// Time Model V1.0: использует причинный возраст (event_id delta), не wall-clock.
pub struct EventGenerator {
    current_event_id: u64,
    current_pulse_id: u64,
}

impl EventGenerator {
    pub fn new() -> Self {
        Self {
            current_event_id: 0,
            current_pulse_id: 0,
        }
    }

    /// Установить текущий event_id из COM
    pub fn set_event_id(&mut self, event_id: u64) {
        self.current_event_id = event_id;
    }

    /// Установить текущий pulse_id (если активен Heartbeat)
    pub fn set_pulse_id(&mut self, pulse_id: u64) {
        self.current_pulse_id = pulse_id;
    }

    /// Проверить нужно ли генерировать событие затухания для токена.
    ///
    /// Time Model V1.0: причинный возраст = current_event_id - token.last_event_id.
    /// Затухание происходит когда возраст превышает порог (1.0 / decay_rate).
    pub fn check_decay(&self, token: &Token, decay_rate: f32) -> Option<Event> {
        let causal_age = self.current_event_id.saturating_sub(token.last_event_id);
        let decay_threshold = (1.0 / decay_rate) as u64;

        if causal_age > decay_threshold && token.valence.abs() > 0 {
            Some(Event::with_pulse(
                0, // event_id присваивается COM
                token.domain_id,
                EventType::TokenDecayed,
                EventPriority::Low,
                self.compute_decay_hash(token, causal_age),
                token.sutra_id,
                0,
                self.current_event_id,
                self.current_pulse_id,
            ))
        } else {
            None
        }
    }

    /// Генерировать событие гравитационного обновления для токена.
    ///
    /// Вызывается когда gravity_strength > 0 и токен нужно притянуть к Anchor (0,0,0).
    pub fn generate_gravity_update(&self, token: &Token) -> Event {
        Event::with_pulse(
            0,
            token.domain_id,
            EventType::GravityUpdate,
            EventPriority::Low,
            self.compute_gravity_hash(token),
            token.sutra_id,
            0,
            self.current_event_id,
            self.current_pulse_id,
        )
    }

    /// Генерировать событие столкновения между двумя токенами.
    ///
    /// SPACE V6.0: используется после обнаружения столкновения через spatial hash.
    pub fn generate_collision(&self, token1: &Token, token2: &Token) -> Event {
        let dist2 = axiom_space::distance2(
            token1.position[0], token1.position[1], token1.position[2],
            token2.position[0], token2.position[1], token2.position[2],
        );

        Event::with_pulse(
            0,
            token1.domain_id,
            EventType::TokenCollision,
            EventPriority::High,
            self.compute_collision_hash(token1, token2, dist2),
            token1.sutra_id,
            token2.sutra_id,
            self.current_event_id,
            self.current_pulse_id,
        )
    }

    /// Проверить нужно ли ослабить или разорвать связь из-за стресса.
    ///
    /// - stress > threshold → ConnectionWeakened
    /// - stress > threshold * 2.0 → ConnectionBroken
    pub fn check_connection_stress(
        &self,
        connection: &Connection,
        stress_threshold: f32,
    ) -> Option<Event> {
        if connection.current_stress > stress_threshold {
            let event_type = if connection.current_stress > stress_threshold * 2.0 {
                EventType::ConnectionBroken
            } else {
                EventType::ConnectionWeakened
            };

            Some(Event::with_pulse(
                0,
                connection.domain_id,
                event_type,
                EventPriority::Normal,
                self.compute_stress_hash(connection),
                connection.source_id,
                connection.target_id,
                self.current_event_id,
                self.current_pulse_id,
            ))
        } else {
            None
        }
    }

    // --- Детерминированные хеши для payload_hash ---

    fn compute_decay_hash(&self, token: &Token, causal_age: u64) -> u64 {
        let mut hash = token.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(causal_age);
        hash = hash.wrapping_mul(31).wrapping_add(token.valence as u64);
        hash
    }

    fn compute_gravity_hash(&self, token: &Token) -> u64 {
        let mut hash = token.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(token.position[0] as u64);
        hash = hash.wrapping_mul(31).wrapping_add(token.position[1] as u64);
        hash = hash.wrapping_mul(31).wrapping_add(token.position[2] as u64);
        hash
    }

    fn compute_collision_hash(&self, token1: &Token, token2: &Token, dist2: i64) -> u64 {
        let mut hash = token1.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(token2.sutra_id as u64);
        hash = hash.wrapping_mul(31).wrapping_add(dist2 as u64);
        hash
    }

    fn compute_stress_hash(&self, connection: &Connection) -> u64 {
        let mut hash = connection.source_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(connection.target_id as u64);
        hash = hash.wrapping_mul(31).wrapping_add(connection.current_stress.to_bits() as u64);
        hash = hash.wrapping_mul(31).wrapping_add(connection.strength.to_bits() as u64);
        hash
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self::new()
    }
}
