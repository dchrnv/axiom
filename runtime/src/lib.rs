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
}
