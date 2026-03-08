// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Axiom Core V0.1.0: Token V5.1, Connection V5.0, COM V1.0, UPO v2.2
// Specs: docs/spec/Token V5.1.md, docs/spec/Connection V5.0.md, 
//        docs/spec/COM V1.0.md, docs/spec/UPO v2.2.md

pub mod token;
pub mod connection;
pub mod event;
pub mod upo;
pub mod domain;
pub mod config;

#[cfg(test)]
mod debug_sizes;

pub use token::{Token, STATE_ACTIVE, STATE_SLEEPING, STATE_LOCKED};
pub use connection::{Connection, FLAG_ACTIVE, FLAG_INHIBITED, FLAG_TEMPORARY, FLAG_CRITICAL};
pub use event::{Event, EventType, EventPriority, Timeline, Snapshot, EVENT_REVERSIBLE, EVENT_CRITICAL};
pub use upo::{
    DynamicTrace, Screen, UPOConfig, UPO,
    TraceSourceType, OctantStats,
    TRACE_ACTIVE, TRACE_FADING, TRACE_LOCKED, TRACE_ETERNAL
};
pub use domain::{
    DomainConfig, StructuralRole, DomainType,
    DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY,
    PROCESSING_IDLE, PROCESSING_ACTIVE, PROCESSING_FROZEN,
    MEMBRANE_OPEN, MEMBRANE_CLOSED, MEMBRANE_SEMI
};
pub use config::{ConfigLoader, AxiomConfig, initialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_v5_1_validation() {
        let mut token = Token::new(1, 1);
        token.last_event_id = 1;
        assert!(token.validate());
        
        // Невалидный токен
        let invalid_token = Token::default();
        assert!(!invalid_token.validate());
    }

    #[test]
    fn token_momentum_update() {
        let mut token = Token::new(1, 1);
        token.update_momentum([10, 20, 30], 42);
        assert_eq!(token.momentum, [10, 20, 30]);
        assert_eq!(token.last_event_id, 42);
    }

    #[test]
    fn connection_v5_0_validation() {
        let mut conn = Connection::new(1, 2, 1);
        conn.created_at = 1;
        conn.last_event_id = 1;
        assert!(conn.validate());
        
        // Невалидная связь
        let invalid_conn = Connection::default();
        assert!(!invalid_conn.validate());
    }

    #[test]
    fn connection_gates() {
        let conn = Connection {
            density_gate: 100,
            thermal_gate: 50,
            ..Default::default()
        };
        
        assert!(conn.can_pass_mass(150));
        assert!(!conn.can_pass_mass(50));
        assert!(conn.can_pass_temperature(30));
        assert!(!conn.can_pass_temperature(70));
    }

    #[test]
    fn com_timeline_monotonic() {
        let mut timeline = Timeline::new();
        let id1 = timeline.next_event_id(1);
        let id2 = timeline.next_event_id(1);
        assert!(id1 < id2);
        assert_eq!(timeline.total_events, 2);
    }

    #[test]
    fn com_event_validation() {
        let mut timeline = Timeline::new();
        let parent_id = timeline.next_event_id(1); // Родительский event
        let event_id = timeline.next_event_id(1); // Новый event
        
        let event = Event::new(
            event_id,
            1,
            EventType::TokenCreate,
            EventPriority::Normal,
            12345,
            1,
            0,
            parent_id,
        );
        
        // Валидация должна проходить, т.к. event_id == current_event_id
        assert!(event.validate(&timeline));
    }

    #[test]
    fn upo_single_token_produces_trace() {
        let mut t = Token::new(1, 0); // domain_id = 0 как в UPOConfig по умолчанию
        t.velocity = [100, 0, 0];
        t.mass = 128;
        t.temperature = 64;
        t.valence = 1;
        t.last_event_id = 42;

        let mut upo = UPO::new(UPOConfig::default());
        let r = upo.compute(&[t], &[], 42);
        assert!(r.is_some());
        let trace = r.unwrap();
        assert_eq!(trace.created_at, 42); // created_at для v2.2
        assert_eq!(trace.last_update, 42);
        assert_eq!(trace.source_type, 1); // TraceSourceType::Token
        assert_eq!(trace.source_id, 1);
    }

    #[test]
    fn upo_v2_2_validation() {
        let screen = Screen::new([256, 256, 256], 0.001, 0.01);
        let trace = DynamicTrace::new(
            0, 0, 0,
            1.0, 440.0,
            1, 1,
            TraceSourceType::Token,
            1,
            0,
        );
        
        assert!(trace.validate(&screen));
        
        // Невалидный след (слишком маленький вес)
        let invalid_trace = DynamicTrace::new(
            0, 0, 0,
            0.0001, 440.0,
            1, 1,
            TraceSourceType::Token,
            1,
            0,
        );
        assert!(!invalid_trace.validate(&screen));
    }

    #[test]
    fn upo_screen_octants() {
        let mut screen = Screen::new([100, 100, 100], 0.001, 0.01);
        
        // След в разных октантах
        let trace1 = DynamicTrace::new(10, 10, 10, 1.0, 440.0, 1, 1, TraceSourceType::Token, 1, 0);
        let trace2 = DynamicTrace::new(-10, -10, -10, 1.0, 440.0, 1, 1, TraceSourceType::Token, 2, 0);
        
        screen.write(&trace1);
        screen.write(&trace2);
        
        assert_eq!(screen.trace_count, 2);
        assert_eq!(screen.octants[7].trace_count, 1); // +++
        assert_eq!(screen.octants[0].trace_count, 1); // ---
    }

    #[test]
    fn domain_v1_3_validation() {
        let domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti1);
        assert!(domain.validate());
        
        // Невалидный домен
        let invalid_domain = DomainConfig::default();
        assert!(!invalid_domain.validate());
    }

    #[test]
    fn domain_membrane_filters() {
        let mut domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti1);
        domain.threshold_mass = 10;
        domain.threshold_temp = 20;
        domain.membrane_state = MEMBRANE_OPEN;
        
        assert!(domain.can_enter(15, 25));
        assert!(!domain.can_enter(5, 25));
        assert!(!domain.can_enter(15, 15));
        
        domain.membrane_state = MEMBRANE_CLOSED;
        assert!(!domain.can_enter(15, 25));
    }

    #[test]
    fn domain_complexity_calculation() {
        let mut domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti1);
        domain.token_capacity = 100;
        domain.connection_capacity = 50;
        domain.gravity_strength = 2.0;
        domain.friction_coeff = 0.2;
        
        let complexity = domain.calculate_complexity();
        assert!(complexity > 0.0);
        
        // Проверяем формулу: 100*0.1 + 50*0.05 + (2.0+0.2)*10.0 = 10 + 2.5 + 22 = 34.5
        assert!((complexity - 34.5).abs() < 0.01);
    }

    #[test]
    fn domain_metadata_update() {
        let mut domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti1);
        domain.created_at = 100;
        domain.error_count = 5;
        
        domain.update_metadata(150);
        assert_eq!(domain.last_update, 150);
        assert_eq!(domain.error_count, 0);
    }

    #[test]
    fn token_states() {
        let mut token = Token::new(1, 1);
        assert!(token.is_active());
        
        token.state = STATE_SLEEPING;
        assert!(token.is_sleeping());
        
        token.state = STATE_LOCKED;
        assert!(token.is_locked());
    }

    #[test]
    fn connection_flags() {
        let mut conn = Connection::new(1, 2, 1);
        assert!(!conn.is_critical());
        
        conn.flags |= FLAG_CRITICAL;
        assert!(conn.is_critical());
        
        conn.flags |= FLAG_INHIBITED;
        assert!(conn.is_inhibited());
    }

    // Интеграционные тесты для Configuration System
    #[test]
    fn config_integration_token_and_connection() {
        // Тест создания токена и соединения из конфигурации
        let token = Token::from_preset("concept", 1, 1).unwrap();
        let connection = Connection::from_preset("strong", 1, 2, 1).unwrap();
        
        // Проверить валидацию с конфигурацией
        assert!(token.validate_with_config().is_ok());
        assert!(connection.validate_with_config().is_ok());
        
        // Проверить, что токен и соединение связаны
        assert_eq!(connection.source_id, token.sutra_id);
        assert_eq!(connection.domain_id, token.domain_id);
    }

    #[test]
    fn config_integration_multiple_presets() {
        // Тест создания нескольких токенов разных типов
        let concept_token = Token::from_preset("concept", 1, 1).unwrap();
        let relation_token = Token::from_preset("relation", 2, 1).unwrap();
        let context_token = Token::from_preset("context", 3, 1).unwrap();
        
        // Тест создания соединений разных типов
        let strong_conn = Connection::from_preset("strong", 1, 2, 1).unwrap();
        let weak_conn = Connection::from_preset("weak", 2, 3, 1).unwrap();
        let temporal_conn = Connection::from_preset("temporal", 3, 1, 1).unwrap();
        
        // Проверить разные значения из пресетов
        assert_eq!(concept_token.resonance, 440);
        assert_eq!(relation_token.resonance, 220);
        assert_eq!(context_token.resonance, 880);
        
        assert_eq!(strong_conn.strength, 1.0);
        assert_eq!(weak_conn.strength, 0.3);
        assert_eq!(temporal_conn.strength, 0.5);
        
        // Все должны проходить валидацию
        assert!(concept_token.validate_with_config().is_ok());
        assert!(relation_token.validate_with_config().is_ok());
        assert!(context_token.validate_with_config().is_ok());
        
        assert!(strong_conn.validate_with_config().is_ok());
        assert!(weak_conn.validate_with_config().is_ok());
        assert!(temporal_conn.validate_with_config().is_ok());
    }

    #[test]
    fn config_integration_domain_with_tokens() {
        // Тест интеграции домена с токенами
        let domain = DomainConfig::from_preset("logic").unwrap();
        let token = Token::from_preset("concept", 1, domain.domain_id).unwrap();
        
        // Проверить, что токен принадлежит домену
        assert_eq!(token.domain_id, domain.domain_id);
        
        // Проверить валидацию
        assert!(domain.validate());
        assert!(token.validate_with_config().is_ok());
    }

    #[test]
    fn config_integration_error_handling() {
        // Тест обработки ошибок конфигурации
        let unknown_token = Token::from_preset("unknown_type", 1, 1);
        assert!(unknown_token.is_err());
        
        let unknown_connection = Connection::from_preset("unknown_type", 1, 2, 1);
        assert!(unknown_connection.is_err());
        
        let unknown_domain = DomainConfig::from_preset("unknown_domain");
        assert!(unknown_domain.is_err());
    }

    #[test]
    fn config_integration_system_validation() {
        // Тест комплексной валидации системы
        let config = initialize().unwrap();
        
        // Проверить, что все схемы загружаются
        assert!(config.schema.domain.contains("domain.yaml"));
        assert!(config.schema.token.contains("token.yaml"));
        assert!(config.schema.connection.contains("connection.yaml"));
        
        // Создать и валидировать компоненты системы
        let domain = DomainConfig::from_preset("logic").unwrap();
        let token = Token::from_preset("concept", 1, domain.domain_id).unwrap();
        let connection = Connection::from_preset("strong", 1, 2, domain.domain_id).unwrap();
        
        // Все компоненты должны быть валидны
        assert!(domain.validate());
        assert!(token.validate_with_config().is_ok());
        assert!(connection.validate_with_config().is_ok());
        
        // Проверить целостность системы
        assert_eq!(token.domain_id, domain.domain_id);
        assert_eq!(connection.domain_id, domain.domain_id);
        assert!(connection.source_id != connection.target_id); // no_self_loops
    }
}
