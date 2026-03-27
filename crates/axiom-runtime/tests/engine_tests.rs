// Integration tests for axiom-runtime AxiomEngine
//
// После введения AshtiCore: AxiomEngine содержит фиксированный уровень из 11 доменов.
// domain_count() всегда == 11, arbiter_ready() всегда == true.
// Домены адресуются по schema: level_id(1) * 100 + role = 100..110.

use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

fn make_cmd(opcode: OpCode, target_id: u32) -> UclCommand {
    UclCommand::new(opcode, target_id, 100, 0)
}

// domain_id доменов AshtiCore уровня 1: 100 (SUTRA) .. 110 (MAYA)
const LOGIC_ID: u32 = 106;

// ============================================================
// Создание Engine
// ============================================================

#[test]
fn test_engine_creation() {
    let engine = AxiomEngine::new();
    // AshtiCore всегда содержит 11 доменов
    assert_eq!(engine.domain_count(), 11);
    assert!(engine.arbiter_ready());
}

// ============================================================
// domain_count и arbiter_ready
// ============================================================

#[test]
fn test_domain_count_is_always_11() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.domain_count(), 11);
}

#[test]
fn test_arbiter_ready_after_creation() {
    let engine = AxiomEngine::new();
    assert!(engine.arbiter_ready(), "AshtiCore регистрирует все 11 доменов при создании");
}

// ============================================================
// process_command: SpawnDomain (no-op, обратная совместимость UCL)
// ============================================================

#[test]
fn test_spawn_domain_returns_success() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::SpawnDomain, 42);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
}

// ============================================================
// process_command: CollapseDomain (no-op)
// ============================================================

#[test]
fn test_collapse_domain_returns_success() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::CollapseDomain, LOGIC_ID);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
}

// ============================================================
// process_command: InjectToken
// ============================================================

#[test]
fn test_inject_token_into_valid_domain() {
    let mut engine = AxiomEngine::new();

    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
    // target_domain_id = 106 (little-endian u16)
    cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
    cmd.payload[1] = (LOGIC_ID >> 8) as u8;
    let mass_bytes = 100.0f32.to_le_bytes();
    cmd.payload[4..8].copy_from_slice(&mass_bytes);

    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(engine.token_count(LOGIC_ID), 1);
}

#[test]
fn test_inject_token_into_unknown_domain_returns_error() {
    let mut engine = AxiomEngine::new();
    let mut cmd = UclCommand::new(OpCode::InjectToken, 9999, 100, 0);
    cmd.payload[0] = 0x0f;
    cmd.payload[1] = 0x27; // 9999 в le
    let result = engine.process_command(&cmd);
    assert!(!result.is_success());
}

// ============================================================
// process_command: TickForward
// ============================================================

#[test]
fn test_tick_forward_does_not_panic() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::TickForward, 0);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
}

#[test]
fn test_tick_forward_reports_11_domains() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::TickForward, 0);
    let result = engine.process_command(&cmd);
    assert_eq!(result.events_generated, 11);
}

// ============================================================
// process_command: CoreReset
// ============================================================

#[test]
fn test_core_reset_reinitialises_engine() {
    let mut engine = AxiomEngine::new();
    // inject token
    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
    cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
    cmd.payload[1] = (LOGIC_ID >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&50.0f32.to_le_bytes());
    engine.process_command(&cmd);
    assert_eq!(engine.token_count(LOGIC_ID), 1);

    // reset
    let reset_cmd = make_cmd(OpCode::CoreReset, 0);
    engine.process_command(&reset_cmd);
    assert_eq!(engine.token_count(LOGIC_ID), 0, "после reset токены должны быть удалены");
    assert_eq!(engine.domain_count(), 11, "после reset AshtiCore пересоздаётся с 11 доменами");
}

// ============================================================
// process_command: BackupState
// ============================================================

#[test]
fn test_backup_returns_11_domains() {
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::BackupState, 0);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(result.events_generated, 11);
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
        opcode: 9999,
        priority: 0,
        flags: 0,
    };
    let result = engine.process_command(&cmd);
    assert!(!result.is_success());
}
