// Integration tests for axiom-runtime AxiomEngine
use axiom_runtime::AxiomEngine;
use axiom_config::DomainConfig;
use axiom_ucl::{UclCommand, OpCode};

fn make_cmd(opcode: OpCode, target_id: u32) -> UclCommand {
    UclCommand::new(opcode, target_id, 100, 0)
}

fn logic_domain(id: u16) -> DomainConfig {
    DomainConfig::factory_logic(id, 0)
}

// ============================================================
// Создание Engine
// ============================================================

#[test]
fn test_engine_creation() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.domain_count(), 0);
    assert!(!engine.arbiter_ready());
}

// ============================================================
// Добавление доменов
// ============================================================

#[test]
fn test_add_domain_increases_count() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(10)).unwrap();
    assert_eq!(engine.domain_count(), 1);
}

#[test]
fn test_add_duplicate_domain_is_error() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(10)).unwrap();
    assert!(engine.add_domain(logic_domain(10)).is_err());
    assert_eq!(engine.domain_count(), 1);
}

#[test]
fn test_add_multiple_domains() {
    let mut engine = AxiomEngine::new();
    for id in 1u16..=5 {
        engine.add_domain(logic_domain(id)).unwrap();
    }
    assert_eq!(engine.domain_count(), 5);
}

// ============================================================
// process_command: SpawnDomain
// ============================================================

#[test]
fn test_spawn_domain_via_command() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::SpawnDomain, 42);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(engine.domain_count(), 1);
}

#[test]
fn test_spawn_duplicate_domain_returns_error() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::SpawnDomain, 42);
    engine.process_command(&cmd);
    let result2 = engine.process_command(&cmd);
    assert!(!result2.is_success());
}

// ============================================================
// process_command: CollapseDomain
// ============================================================

#[test]
fn test_collapse_domain_removes_it() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(10)).unwrap();
    let cmd = make_cmd(OpCode::CollapseDomain, 10);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(engine.domain_count(), 0);
}

#[test]
fn test_collapse_nonexistent_domain_returns_error() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::CollapseDomain, 999);
    let result = engine.process_command(&cmd);
    assert!(!result.is_success());
}

// ============================================================
// process_command: InjectToken
// ============================================================

#[test]
fn test_inject_token_into_domain() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(10)).unwrap();

    let mut cmd = UclCommand::new(OpCode::InjectToken, 10, 100, 0);
    // target_domain_id = 10 (little-endian u16 в payload[0..2])
    cmd.payload[0] = 10;
    cmd.payload[1] = 0;
    // mass = 100.0 f32 в payload[4..8]
    let mass_bytes = 100.0f32.to_le_bytes();
    cmd.payload[4..8].copy_from_slice(&mass_bytes);

    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(engine.token_count(10), 1);
}

#[test]
fn test_inject_token_to_unknown_domain_returns_error() {
    let mut engine = AxiomEngine::new();
    let mut cmd = UclCommand::new(OpCode::InjectToken, 99, 100, 0);
    cmd.payload[0] = 99;
    cmd.payload[1] = 0;
    let result = engine.process_command(&cmd);
    assert!(!result.is_success());
}

// ============================================================
// process_command: TickForward
// ============================================================

#[test]
fn test_tick_forward_does_not_panic() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(10)).unwrap();
    let cmd = make_cmd(OpCode::TickForward, 0);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
}

// ============================================================
// process_command: CoreReset
// ============================================================

#[test]
fn test_core_reset_clears_domains() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(1)).unwrap();
    engine.add_domain(logic_domain(2)).unwrap();
    let cmd = make_cmd(OpCode::CoreReset, 0);
    engine.process_command(&cmd);
    assert_eq!(engine.domain_count(), 0);
}

// ============================================================
// process_command: BackupState
// ============================================================

#[test]
fn test_backup_returns_domain_count() {
    let mut engine = AxiomEngine::new();
    engine.add_domain(logic_domain(1)).unwrap();
    engine.add_domain(logic_domain(2)).unwrap();
    let cmd = make_cmd(OpCode::BackupState, 0);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(result.events_generated, 2);
}

// ============================================================
// Неизвестный opcode
// ============================================================

#[test]
fn test_unknown_opcode_returns_error() {
    let mut engine = AxiomEngine::new();
    let cmd = UclCommand {
        payload: [0; 48],
        command_id: 1,
        target_id: 0,
        opcode: 9999, // несуществующий
        priority: 0,
        flags: 0,
    };
    let result = engine.process_command(&cmd);
    assert!(!result.is_success());
}
