// Integration tests for axiom-runtime Guardian
use axiom_runtime::Guardian;
use axiom_domain::{DomainState, DomainConfig};
use axiom_core::{Token, STATE_LOCKED};

fn make_token(sutra_id: u32, mass: u8, valence: i8) -> Token {
    let mut t = Token::new(sutra_id, 1, [0, 0, 0], 1);
    t.mass = mass;
    t.valence = valence;
    t
}

// ============================================================
// validate_reflex
// ============================================================

#[test]
fn test_valid_token_passes() {
    let mut guardian = Guardian::new();
    let token = make_token(1, 100, 5);
    assert!(guardian.validate_reflex(&token));
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_locked_token_blocked() {
    let mut guardian = Guardian::new();
    let mut token = make_token(1, 100, 5);
    token.state = STATE_LOCKED;
    assert!(!guardian.validate_reflex(&token));
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_zero_sutra_id_blocked() {
    let mut guardian = Guardian::new();
    let token = make_token(0, 100, 5); // sutra_id = 0 → invalid
    assert!(!guardian.validate_reflex(&token));
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_valence_without_mass_blocked() {
    let mut guardian = Guardian::new();
    let token = make_token(1, 0, 5); // mass=0 with valence!=0
    assert!(!guardian.validate_reflex(&token));
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_zero_valence_zero_mass_allowed() {
    let mut guardian = Guardian::new();
    let token = make_token(1, 0, 0); // mass=0 but valence=0 → ok
    assert!(guardian.validate_reflex(&token));
}

#[test]
fn test_violations_accumulate() {
    let mut guardian = Guardian::new();
    let bad1 = make_token(0, 100, 5);   // sutra_id=0
    let bad2 = make_token(1, 0, 3);     // valence without mass
    guardian.validate_reflex(&bad1);
    guardian.validate_reflex(&bad2);
    assert_eq!(guardian.violation_count(), 2);
}

#[test]
fn test_reset_violations() {
    let mut guardian = Guardian::new();
    let bad = make_token(0, 100, 5);
    guardian.validate_reflex(&bad);
    assert_eq!(guardian.violation_count(), 1);
    guardian.reset_violations();
    assert_eq!(guardian.violation_count(), 0);
}

// ============================================================
// scan_domain
// ============================================================

#[test]
fn test_scan_empty_domain_no_violations() {
    let mut guardian = Guardian::new();
    let config = DomainConfig::factory_logic(1, 0);
    let state = DomainState::new(&config);
    let violations = guardian.scan_domain(&state);
    assert_eq!(violations, 0);
}

#[test]
fn test_scan_domain_detects_bad_tokens() {
    let mut guardian = Guardian::new();
    let config = DomainConfig::factory_logic(1, 0);
    let mut state = DomainState::new(&config);

    // Добавляем токен с валентностью но без массы
    let bad = make_token(1, 0, 5);
    state.add_token(bad).unwrap();

    let violations = guardian.scan_domain(&state);
    assert_eq!(violations, 1);
    assert_eq!(guardian.violation_count(), 1);
}
