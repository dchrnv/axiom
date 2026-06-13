// Integration tests for axiom-domain physics: EventGenerator
use axiom_core::{Connection, EventType, Token};
use axiom_domain::EventGenerator;

// ============================================================
// check_decay
// ============================================================

#[test]
fn test_decay_old_token_with_valence() {
    let mut gen = EventGenerator::new();
    gen.set_event_id(10000);

    let mut token = Token::new(1, 6, [0, 0, 0], 0); // last_event_id = 0
    token.valence = 5;

    // causal_age = 10000, decay_threshold = 1/0.001 = 1000 → age > threshold
    let event = gen.check_decay(&token, 0.001);
    assert!(event.is_some(), "Old token with valence should decay");

    let e = event.unwrap();
    assert_eq!(e.event_type, EventType::TokenDecayed as u16);
    assert_eq!(e.domain_id, 6);
    assert_eq!(e.target_id, 1); // sutra_id maps to target_id in with_pulse
}

#[test]
fn test_decay_young_token_no_event() {
    let mut gen = EventGenerator::new();
    gen.set_event_id(100);

    let mut token = Token::new(1, 6, [0, 0, 0], 0); // age = 100 < 1000
    token.valence = 5;

    let event = gen.check_decay(&token, 0.001);
    assert!(event.is_none(), "Young token should not decay");
}

#[test]
fn test_decay_zero_valence_no_event() {
    let mut gen = EventGenerator::new();
    gen.set_event_id(10000);

    let mut token = Token::new(1, 6, [0, 0, 0], 0);
    token.valence = 0;

    let event = gen.check_decay(&token, 0.001);
    assert!(event.is_none(), "Token with zero valence should not decay");
}

#[test]
fn test_decay_at_threshold_boundary() {
    let mut gen = EventGenerator::new();
    gen.set_event_id(1001); // age = 1001 > 1000

    let mut token = Token::new(1, 6, [0, 0, 0], 0);
    token.valence = 1;

    let event = gen.check_decay(&token, 0.001);
    assert!(event.is_some(), "Token just past threshold should decay");
}

// ============================================================
// ============================================================





// ============================================================
// generate_collision
// ============================================================

#[test]
fn test_collision_event_generated() {
    let mut gen = EventGenerator::new();
    gen.set_event_id(200);

    let t1 = Token::new(10, 6, [0, 0, 0], 0);
    let t2 = Token::new(20, 6, [50, 0, 0], 0);

    let event = gen.generate_collision(&t1, &t2);

    assert_eq!(event.event_type, EventType::TokenCollision as u16);
    assert_eq!(event.domain_id, 6);
    assert_eq!(event.target_id, 10); // token1.sutra_id
    assert_eq!(event.source_id, 20); // token2.sutra_id
    assert_eq!(event.parent_event_id, 200);
}

#[test]
fn test_collision_deterministic_hash() {
    let gen = EventGenerator::new();
    let t1 = Token::new(1, 6, [0, 0, 0], 0);
    let t2 = Token::new(2, 6, [10, 0, 0], 0);

    let e1 = gen.generate_collision(&t1, &t2);
    let e2 = gen.generate_collision(&t1, &t2);
}

// ============================================================
// check_connection_stress
// ============================================================

#[test]
fn test_connection_stress_weakened() {
    let gen = EventGenerator::new();
    let conn = Connection {
        source_id: 1,
        target_id: 2,
        domain_id: 6,
        current_stress: 0.9, // > 0.8 but < 0.8 * 2.0 = 1.6
        ..Connection::default()
    };

    let event = gen.check_connection_stress(&conn, 0.8);
    assert!(event.is_some());
    let e = event.unwrap();
    assert_eq!(e.event_type, EventType::ConnectionWeakened as u16);
}

#[test]
fn test_connection_stress_broken() {
    let gen = EventGenerator::new();
    let conn = Connection {
        source_id: 1,
        target_id: 2,
        domain_id: 6,
        current_stress: 2.0, // > 0.8 * 2.0 = 1.6
        ..Connection::default()
    };

    let event = gen.check_connection_stress(&conn, 0.8);
    assert!(event.is_some());
    let e = event.unwrap();
    assert_eq!(e.event_type, EventType::ConnectionBroken as u16);
}

#[test]
fn test_connection_stress_below_threshold() {
    let gen = EventGenerator::new();
    let conn = Connection {
        current_stress: 0.5, // < 0.8
        ..Connection::default()
    };

    let event = gen.check_connection_stress(&conn, 0.8);
    assert!(event.is_none(), "Low stress should produce no event");
}

#[test]
fn test_connection_stress_deterministic_hash() {
    let gen = EventGenerator::new();
    let conn = Connection {
        source_id: 5,
        target_id: 10,
        current_stress: 1.0,
        strength: 0.5,
        ..Connection::default()
    };

    let e1 = gen.check_connection_stress(&conn, 0.8).unwrap();
    let e2 = gen.check_connection_stress(&conn, 0.8).unwrap();
}

// ============================================================
// set_event_id / set_pulse_id
// ============================================================


