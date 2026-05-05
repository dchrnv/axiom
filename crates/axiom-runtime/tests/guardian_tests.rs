// Integration tests for axiom-runtime Guardian
use axiom_core::{Token, STATE_LOCKED};
use axiom_domain::{DomainConfig, DomainState};
use axiom_genome::{ModuleId, Permission, ResourceId};
use axiom_runtime::{CodexAction, Guardian, InhibitReason, ReflexDecision, VetoReason};

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
    let mut guardian = Guardian::with_default_genome();
    let token = make_token(1, 100, 5);
    assert_eq!(guardian.validate_reflex(&token), ReflexDecision::Allow);
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_locked_token_blocked() {
    let mut guardian = Guardian::with_default_genome();
    let mut token = make_token(1, 100, 5);
    token.state = STATE_LOCKED;
    assert_eq!(
        guardian.validate_reflex(&token),
        ReflexDecision::Veto(VetoReason::TokenLocked)
    );
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_zero_sutra_id_blocked() {
    let mut guardian = Guardian::with_default_genome();
    let token = make_token(0, 100, 5);
    assert_eq!(
        guardian.validate_reflex(&token),
        ReflexDecision::Veto(VetoReason::ZeroSutraId)
    );
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_valence_without_mass_blocked() {
    let mut guardian = Guardian::with_default_genome();
    let token = make_token(1, 0, 5);
    assert_eq!(
        guardian.validate_reflex(&token),
        ReflexDecision::Veto(VetoReason::ValenceWithoutMass)
    );
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_zero_valence_zero_mass_allowed() {
    let mut guardian = Guardian::with_default_genome();
    let token = make_token(1, 0, 0);
    assert_eq!(guardian.validate_reflex(&token), ReflexDecision::Allow);
}

#[test]
fn test_violations_accumulate() {
    let mut guardian = Guardian::with_default_genome();
    let bad1 = make_token(0, 100, 5); // sutra_id=0
    let bad2 = make_token(1, 0, 3); // valence without mass
    guardian.validate_reflex(&bad1);
    guardian.validate_reflex(&bad2);
    assert_eq!(guardian.violation_count(), 2);
}

#[test]
fn test_reset_violations() {
    let mut guardian = Guardian::with_default_genome();
    let bad = make_token(0, 100, 5);
    guardian.validate_reflex(&bad);
    assert_eq!(guardian.violation_count(), 1);
    guardian.reset_violations();
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_reflex_decision_is_allowed() {
    assert!(ReflexDecision::Allow.is_allowed());
    assert!(!ReflexDecision::Veto(VetoReason::TokenLocked).is_allowed());
}

// ============================================================
// scan_domain
// ============================================================

#[test]
fn test_scan_empty_domain_no_violations() {
    let mut guardian = Guardian::with_default_genome();
    let config = DomainConfig::factory_logic(1, 0);
    let state = DomainState::new(&config);
    let actions = guardian.scan_domain(&state);
    assert!(actions.is_empty());
}

#[test]
fn test_scan_domain_detects_bad_tokens() {
    let mut guardian = Guardian::with_default_genome();
    let config = DomainConfig::factory_logic(1, 0);
    let mut state = DomainState::new(&config);

    let bad = make_token(1, 0, 5);
    state.add_token(bad).unwrap();

    let actions = guardian.scan_domain(&state);
    assert_eq!(actions.len(), 1);
    assert_eq!(
        actions[0].reason,
        InhibitReason::ValenceWithoutMass { token_index: 0 }
    );
    assert_eq!(guardian.violation_count(), 1);
}

#[test]
fn test_scan_domain_multiple_bad_tokens() {
    let mut guardian = Guardian::with_default_genome();
    let config = DomainConfig::factory_logic(1, 0);
    let mut state = DomainState::new(&config);

    state.add_token(make_token(1, 0, 1)).unwrap(); // bad
    state.add_token(make_token(2, 10, 0)).unwrap(); // ok
    state.add_token(make_token(3, 0, 2)).unwrap(); // bad

    let actions = guardian.scan_domain(&state);
    assert_eq!(actions.len(), 2);
    assert_eq!(guardian.violation_count(), 2);
}

// ============================================================
// enforce_access / enforce_protocol (GENOME)
// ============================================================

#[test]
fn test_enforce_access_guardian_codex_allowed() {
    let mut guardian = Guardian::with_default_genome();
    assert!(guardian.enforce_access(
        ModuleId::Guardian,
        ResourceId::CodexRules,
        Permission::ReadWrite,
    ));
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_enforce_access_adapters_codex_denied() {
    let mut guardian = Guardian::with_default_genome();
    assert!(!guardian.enforce_access(
        ModuleId::Adapters,
        ResourceId::CodexRules,
        Permission::ReadWrite,
    ));
    assert_eq!(guardian.violation_count(), 1);
    assert_eq!(guardian.stats().access_denied, 1);
}

#[test]
fn test_enforce_protocol_sutra_to_experience_allowed() {
    let mut guardian = Guardian::with_default_genome();
    assert!(guardian.enforce_protocol(ModuleId::Sutra, ModuleId::Experience));
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_enforce_protocol_adapters_to_sutra_denied() {
    let mut guardian = Guardian::with_default_genome();
    assert!(!guardian.enforce_protocol(ModuleId::Adapters, ModuleId::Sutra));
    assert_eq!(guardian.violation_count(), 1);
    assert_eq!(guardian.stats().protocol_denied, 1);
}

// ============================================================
// update_codex
// ============================================================

#[test]
fn test_update_codex_reset_violations() {
    let mut guardian = Guardian::with_default_genome();
    let mut codex = DomainState::new(&DomainConfig::factory_logic(99, 0));

    // Создаём нарушение
    let bad = make_token(0, 100, 5);
    guardian.validate_reflex(&bad);
    assert_eq!(guardian.violation_count(), 1);

    // ResetViolations через update_codex
    guardian
        .update_codex(&mut codex, CodexAction::ResetViolations)
        .unwrap();
    assert_eq!(guardian.violation_count(), 0);
}

#[test]
fn test_update_codex_add_rule() {
    let mut guardian = Guardian::with_default_genome();
    let mut codex = DomainState::new(&DomainConfig::factory_logic(99, 0));
    let rule_token = make_token(42, 1, 0);
    guardian
        .update_codex(&mut codex, CodexAction::AddRule(rule_token))
        .unwrap();
    assert_eq!(codex.token_count(), 1);
}

// ============================================================
// genome accessor
// ============================================================

#[test]
fn test_genome_accessor() {
    let guardian = Guardian::with_default_genome();
    assert_eq!(guardian.genome().version, 1);
}
