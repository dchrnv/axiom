// Integration tests for axiom-domain membrane: can_enter_domain, can_exit_domain
use axiom_domain::{can_enter_domain, can_exit_domain};
use axiom_config::{DomainConfig, MEMBRANE_OPEN, MEMBRANE_CLOSED, MEMBRANE_SEMI, MEMBRANE_ADAPTIVE};
use axiom_core::{Token, STATE_LOCKED};

fn make_token(sutra_id: u32) -> Token {
    Token::new(sutra_id, 1, [0, 0, 0], 0)
}

// ============================================================
// can_enter_domain
// ============================================================

#[test]
fn test_closed_membrane_blocks_entry() {
    let config = DomainConfig::factory_sutra(1); // CLOSED
    assert_eq!(config.membrane_state, MEMBRANE_CLOSED);
    let token = make_token(1);
    assert!(!can_enter_domain(&token, &config));
}

#[test]
fn test_open_membrane_allows_entry() {
    let mut config = DomainConfig::factory_probe(5, 0); // OPEN
    assert_eq!(config.membrane_state, MEMBRANE_OPEN);
    config.threshold_mass = 0; // ensure mass check passes
    config.input_filter = u64::MAX; // no bloom filter
    let token = make_token(1);
    assert!(can_enter_domain(&token, &config));
}

#[test]
fn test_mass_below_threshold_blocks_entry() {
    let mut config = DomainConfig::factory_probe(5, 0);
    config.membrane_state = MEMBRANE_OPEN;
    config.threshold_mass = 200; // Token::mass is u8, max 255
    config.input_filter = u64::MAX;
    let token = make_token(1); // token.mass = 0 by default
    assert!(!can_enter_domain(&token, &config));
}

#[test]
fn test_mass_at_threshold_allows_entry() {
    let mut config = DomainConfig::factory_probe(5, 0);
    config.membrane_state = MEMBRANE_OPEN;
    config.threshold_mass = 0;
    config.input_filter = u64::MAX;
    let token = make_token(1);
    assert!(can_enter_domain(&token, &config));
}

#[test]
fn test_semi_membrane_allows_entry_with_mass() {
    let mut config = DomainConfig::factory_execution(1, 0); // SEMI
    assert_eq!(config.membrane_state, MEMBRANE_SEMI);
    config.threshold_mass = 0;
    config.input_filter = u64::MAX;
    let token = make_token(1);
    assert!(can_enter_domain(&token, &config));
}

#[test]
fn test_adaptive_membrane_allows_entry() {
    let mut config = DomainConfig::factory_logic(6, 1); // ADAPTIVE
    assert_eq!(config.membrane_state, MEMBRANE_ADAPTIVE);
    config.threshold_mass = 0;
    config.input_filter = u64::MAX;
    let token = make_token(1);
    assert!(can_enter_domain(&token, &config));
}

// ============================================================
// can_exit_domain
// ============================================================

#[test]
fn test_closed_membrane_blocks_exit() {
    let config = DomainConfig::factory_sutra(1); // CLOSED
    let token = make_token(1);
    assert!(!can_exit_domain(&token, &config));
}

#[test]
fn test_open_membrane_allows_exit() {
    let config = DomainConfig::factory_probe(5, 0); // OPEN
    let token = make_token(1); // not locked
    assert!(can_exit_domain(&token, &config));
}

#[test]
fn test_locked_token_cannot_exit() {
    let config = DomainConfig::factory_probe(5, 0); // OPEN
    let mut token = make_token(1);
    token.state = STATE_LOCKED;
    assert!(!can_exit_domain(&token, &config));
}

#[test]
fn test_unlocked_token_can_exit_open_membrane() {
    let config = DomainConfig::factory_void(8, 0); // OPEN
    let mut token = make_token(1);
    token.state = 0; // not locked
    assert!(can_exit_domain(&token, &config));
}

#[test]
fn test_locked_token_cannot_exit_semi_membrane() {
    let mut config = DomainConfig::factory_execution(1, 0); // SEMI
    let mut token = make_token(1);
    token.state = STATE_LOCKED;
    assert!(!can_exit_domain(&token, &config));
}
