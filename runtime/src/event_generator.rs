// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Event-Driven V1: docs/spec/time/Event-Driven V1.md
// Генератор событий из изменений состояния

use crate::event::{Event, EventType, EventPriority};
use crate::token::Token;
use crate::connection::Connection;

/// EventGenerator - механизм обнаружения трансформаций и генерации событий
///
/// Event-Driven V1, раздел 6: "Генерация событий"
/// Симулятор проверяет локальную область изменений и генерирует события
pub struct EventGenerator {
    pub(crate) current_event_id: u64,
    pub(crate) current_pulse_id: u64,
}

impl EventGenerator {
    pub fn new() -> Self {
        Self {
            current_event_id: 0,
            current_pulse_id: 0,
        }
    }

    /// Устанавливает текущий event_id из COM
    pub fn set_event_id(&mut self, event_id: u64) {
        self.current_event_id = event_id;
    }

    /// Устанавливает текущий pulse_id (если активен Heartbeat)
    pub fn set_pulse_id(&mut self, pulse_id: u64) {
        self.current_pulse_id = pulse_id;
    }

    /// Проверяет нужно ли генерировать событие затухания для токена
    ///
    /// Time Model V1.0: использует причинный возраст (current_event_id - last_event_id)
    /// Token V5.2: valence уменьшается при затухании
    pub fn check_decay(&self, token: &Token, decay_rate: f32) -> Option<Event> {
        let causal_age = self.current_event_id.saturating_sub(token.last_event_id);

        // Если причинный возраст превышает порог, генерируем событие
        let decay_threshold = (1.0 / decay_rate) as u64;

        if causal_age > decay_threshold && token.valence.abs() > 0 {
            Some(Event::with_pulse(
                0, // event_id будет присвоен COM
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

    /// Проверяет столкновение между токенами
    pub fn check_collision(&self, token1: &Token, token2: &Token, distance: f32, threshold: f32) -> Option<Event> {
        if distance < threshold {
            Some(Event::with_pulse(
                0,
                token1.domain_id,
                EventType::CollisionDetected,
                EventPriority::High,
                self.compute_collision_hash(token1.sutra_id, token2.sutra_id, distance),
                token1.sutra_id,
                token2.sutra_id,
                self.current_event_id,
                self.current_pulse_id,
            ))
        } else {
            None
        }
    }

    /// Проверяет нужно ли ослабить связь из-за стресса
    pub fn check_connection_stress(&self, connection: &Connection, stress_threshold: f32) -> Option<Event> {
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

    /// Генерирует событие обновления гравитации для токена
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

    /// Генерирует событие столкновения между двумя токенами
    ///
    /// SPACE V6.0: используется после обнаружения столкновения через spatial hash
    pub fn generate_collision(&self, token1: &Token, token2: &Token) -> Event {
        // Вычисляем квадрат расстояния для хеша
        let dist2 = crate::space::distance2(
            token1.position[0], token1.position[1], token1.position[2],
            token2.position[0], token2.position[1], token2.position[2],
        );

        Event::with_pulse(
            0,
            token1.domain_id,
            EventType::TokenCollision,
            EventPriority::High,
            self.compute_collision_hash_from_tokens(token1, token2, dist2),
            token1.sutra_id,
            token2.sutra_id,
            self.current_event_id,
            self.current_pulse_id,
        )
    }

    // --- Hash functions для детерминизма ---

    fn compute_decay_hash(&self, token: &Token, causal_age: u64) -> u64 {
        // Простой хеш для детерминизма
        let mut hash = token.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(causal_age);
        hash = hash.wrapping_mul(31).wrapping_add(token.valence as u64);
        hash
    }

    fn compute_collision_hash(&self, id1: u32, id2: u32, distance: f32) -> u64 {
        let mut hash = id1 as u64;
        hash = hash.wrapping_mul(31).wrapping_add(id2 as u64);
        hash = hash.wrapping_mul(31).wrapping_add(distance.to_bits() as u64);
        hash
    }

    fn compute_stress_hash(&self, connection: &Connection) -> u64 {
        let mut hash = connection.source_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(connection.target_id as u64);
        hash = hash.wrapping_mul(31).wrapping_add(connection.current_stress.to_bits() as u64);
        hash = hash.wrapping_mul(31).wrapping_add(connection.strength.to_bits() as u64);
        hash
    }

    fn compute_gravity_hash(&self, token: &Token) -> u64 {
        let mut hash = token.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(token.position[0] as u64);
        hash = hash.wrapping_mul(31).wrapping_add(token.position[1] as u64);
        hash = hash.wrapping_mul(31).wrapping_add(token.position[2] as u64);
        hash
    }

    fn compute_collision_hash_from_tokens(&self, token1: &Token, token2: &Token, dist2: i64) -> u64 {
        let mut hash = token1.sutra_id as u64;
        hash = hash.wrapping_mul(31).wrapping_add(token2.sutra_id as u64);
        hash = hash.wrapping_mul(31).wrapping_add(dist2 as u64);
        hash
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token(sutra_id: u32, domain_id: u16, last_event_id: u64) -> Token {
        let mut token = Token::new(sutra_id, domain_id);
        token.last_event_id = last_event_id;
        token.valence = 10; // Ненулевой valence
        token
    }

    #[test]
    fn test_decay_detection() {
        let mut generator = EventGenerator::new();
        generator.set_event_id(1000);

        let token = create_test_token(1, 1, 500); // Возраст = 500
        let decay_rate = 0.001; // Порог = 1000 событий

        // Не должно генерировать (возраст < порога)
        assert!(generator.check_decay(&token, decay_rate).is_none());

        let old_token = create_test_token(2, 1, 0); // Возраст = 1000
        // Должно генерировать (возраст >= порога)
        let event = generator.check_decay(&old_token, decay_rate);
        assert!(event.is_some());

        let event = event.unwrap();
        assert_eq!(event.event_type, EventType::TokenDecayed as u16);
        assert_eq!(event.target_id, 2);
        assert_eq!(event.domain_id, 1);
    }

    #[test]
    fn test_collision_detection() {
        let generator = EventGenerator::new();
        let token1 = create_test_token(1, 1, 0);
        let token2 = create_test_token(2, 1, 0);

        // Столкновение обнаружено
        let event = generator.check_collision(&token1, &token2, 5.0, 10.0);
        assert!(event.is_some());

        let event = event.unwrap();
        assert_eq!(event.event_type, EventType::CollisionDetected as u16);
        assert_eq!(event.target_id, 1);
        assert_eq!(event.source_id, 2);

        // Столкновения нет
        let no_event = generator.check_collision(&token1, &token2, 15.0, 10.0);
        assert!(no_event.is_none());
    }

    #[test]
    fn test_connection_stress() {
        let generator = EventGenerator::new();
        let mut connection = Connection::new(10, 20, 1);
        connection.current_stress = 150.0;

        // Слабый стресс - ослабление
        let event = generator.check_connection_stress(&connection, 100.0);
        assert!(event.is_some());
        let unwrapped = event.unwrap();
        assert_eq!(unwrapped.event_type, EventType::ConnectionWeakened as u16);
        assert_eq!(unwrapped.target_id, 10);
        assert_eq!(unwrapped.source_id, 20);

        // Сильный стресс - разрыв
        connection.current_stress = 250.0;
        let event = generator.check_connection_stress(&connection, 100.0);
        assert!(event.is_some());
        assert_eq!(event.unwrap().event_type, EventType::ConnectionBroken as u16);
    }

    #[test]
    fn test_pulse_id_tracking() {
        let mut generator = EventGenerator::new();
        generator.set_event_id(100);
        generator.set_pulse_id(42);

        let token = create_test_token(1, 1, 0);
        let event = generator.generate_gravity_update(&token);

        assert_eq!(event.pulse_id, 42);
        assert_eq!(event.event_type, EventType::GravityUpdate as u16);
    }

    #[test]
    fn test_deterministic_hashes() {
        let generator = EventGenerator::new();
        let token = create_test_token(1, 1, 0);

        // Одинаковые входы должны давать одинаковые хеши
        let hash1 = generator.compute_decay_hash(&token, 100);
        let hash2 = generator.compute_decay_hash(&token, 100);
        assert_eq!(hash1, hash2);

        // Разные входы - разные хеши
        let hash3 = generator.compute_decay_hash(&token, 200);
        assert_ne!(hash1, hash3);
    }
}
