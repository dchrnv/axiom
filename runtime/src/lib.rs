// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Axiom Core: COM (Causal Order Model) + UPO v2.1
// Specs: docs/spec/CAUSAL ORDER MODEL (COM).md, docs/spec/UPO v2.1.md

pub mod clock;
pub mod connection;
pub mod event;
pub mod token;
pub mod upo;

pub use clock::CausalClock;
pub use connection::Connection;
pub use event::{Event, EventType, Snapshot};
pub use token::Token;
pub use upo::{DynamicTrace, Screen, UPOConfig, UPO};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::STATE_ACTIVE;

    #[test]
    fn causal_clock_monotonic() {
        CausalClock::reset_for_test();
        let a = CausalClock::next();
        let b = CausalClock::next();
        let c = CausalClock::next();
        assert!(a < b && b < c);
    }

    #[test]
    fn token_default_active() {
        let t = Token::default();
        assert!(t.is_active());
    }

    #[test]
    fn connection_is_active() {
        let mut c = Connection::new(1, 2, 0);
        assert!(!c.is_active());
        c.flags |= connection::FLAG_ACTIVE;
        assert!(c.is_active());
    }

    #[test]
    fn upo_empty_domain_returns_none() {
        let mut upo = UPO::new(UPOConfig::default());
        let r = upo.compute(&[], &[], 1);
        assert!(r.is_none());
    }

    #[test]
    fn upo_single_token_produces_trace() {
        let mut t = Token::new(1, 0);
        t.velocity = [100, 0, 0];
        t.mass = 128;
        t.temperature = 64;
        t.valence = 1;
        t.state = STATE_ACTIVE;

        let mut upo = UPO::new(UPOConfig::default());
        let r = upo.compute(&[t], &[], 42);
        assert!(r.is_some());
        let trace = r.unwrap();
        assert_eq!(trace.event_id, 42);
        assert!(trace.x >= -1.0 && trace.x <= 1.0);
        assert!(trace.y >= -1.0 && trace.y <= 1.0);
        assert!(trace.z >= -1.0 && trace.z <= 1.0);
    }

    #[test]
    fn screen_decay_keeps_min_intensity() {
        let mut screen = Screen::new(0.5, 0.001);
        screen.set_current_event(0);
        let trace = DynamicTrace::new(0.0, 0.0, 0.0, 1.0, 0);
        screen.write(&trace);

        screen.set_current_event(100);
        let trace2 = DynamicTrace::new(0.0, 0.0, 0.0, 0.0, 100);
        screen.write(&trace2);

        let v = screen.grid[128][128][128];
        assert!(v.abs() >= 0.001, "decay should not go below min_intensity");
    }
}
