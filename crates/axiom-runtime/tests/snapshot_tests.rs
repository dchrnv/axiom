// Integration tests for axiom-runtime Snapshot
use axiom_runtime::{AxiomEngine, EngineSnapshot};
use axiom_config::DomainConfig;
use axiom_ucl::{UclCommand, OpCode};

fn engine_with_domains() -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    engine.add_domain(DomainConfig::factory_logic(1, 0)).unwrap();
    engine.add_domain(DomainConfig::factory_logic(2, 0)).unwrap();
    engine
}

// ============================================================
// EngineSnapshot::empty()
// ============================================================

#[test]
fn test_empty_snapshot() {
    let snap = EngineSnapshot::empty();
    assert_eq!(snap.domain_count(), 0);
    assert_eq!(snap.total_token_count(), 0);
}

// ============================================================
// engine.snapshot()
// ============================================================

#[test]
fn test_snapshot_captures_domain_count() {
    let engine = engine_with_domains();
    let snap = engine.snapshot();
    assert_eq!(snap.domain_count(), 2);
}

#[test]
fn test_snapshot_captures_tokens() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(DomainConfig::factory_logic(10, 0)).unwrap();

    // Inject a token
    let mut cmd = UclCommand::new(OpCode::InjectToken, 10, 100, 0);
    cmd.payload[0] = 10;
    cmd.payload[1] = 0;
    let mass_bytes = 50.0f32.to_le_bytes();
    cmd.payload[4..8].copy_from_slice(&mass_bytes);
    engine.process_command(&cmd);

    let snap = engine.snapshot();
    assert_eq!(snap.total_token_count(), 1);
}

#[test]
fn test_snapshot_find_domain() {
    let engine = engine_with_domains();
    let snap = engine.snapshot();
    assert!(snap.find_domain(1).is_some());
    assert!(snap.find_domain(99).is_none());
}

// ============================================================
// AxiomEngine::restore_from()
// ============================================================

#[test]
fn test_restore_domain_count() {
    let engine = engine_with_domains();
    let snap = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.domain_count(), 2);
}

#[test]
fn test_restore_token_count() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(DomainConfig::factory_logic(10, 0)).unwrap();

    let mut cmd = UclCommand::new(OpCode::InjectToken, 10, 100, 0);
    cmd.payload[0] = 10;
    cmd.payload[1] = 0;
    let mass_bytes = 50.0f32.to_le_bytes();
    cmd.payload[4..8].copy_from_slice(&mass_bytes);
    engine.process_command(&cmd);

    let snap = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.token_count(10), 1);
}

#[test]
fn test_snapshot_roundtrip_domain_count() {
    let engine = engine_with_domains();
    let snap1 = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap1);
    let snap2 = restored.snapshot();
    assert_eq!(snap1.domain_count(), snap2.domain_count());
}
