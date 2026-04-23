// Integration tests for axiom-runtime AxiomEngine
//
// После введения AshtiCore: AxiomEngine содержит фиксированный уровень из 11 доменов.
// domain_count() всегда == 11, arbiter_ready() всегда == true.
// Домены адресуются по schema: level_id(1) * 100 + role = 100..110.

use std::sync::Arc;
use axiom_runtime::{AxiomEngine, AxiomError};
use axiom_genome::Genome;
use axiom_config::GUARDIAN_CHECK_REQUIRED;
use axiom_ucl::{UclCommand, OpCode};

fn make_cmd(opcode: OpCode, target_id: u32) -> UclCommand {
    UclCommand::new(opcode, target_id, 100, 0)
}

// domain_id доменов AshtiCore уровня 1: 100 (SUTRA) .. 110 (MAYA)
const LOGIC_ID: u16 = 106;

// ============================================================
// Создание Engine
// ============================================================

#[test]
fn test_engine_creation() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.domain_count(), 11);
    assert!(engine.arbiter_ready());
}

#[test]
fn test_try_new_with_valid_genome() {
    let genome = Arc::new(Genome::default_ashti_core());
    let engine = AxiomEngine::try_new(genome).unwrap();
    assert_eq!(engine.domain_count(), 11);
    assert!(engine.arbiter_ready());
}

#[test]
fn test_try_new_with_invalid_genome_returns_err() {
    let mut genome = Genome::default_ashti_core();
    genome.invariants.token_size = 32; // нарушение инварианта
    let result = AxiomEngine::try_new(Arc::new(genome));
    assert!(matches!(result, Err(AxiomError::InvalidGenome(_))));
}

#[test]
fn test_try_new_error_contains_description() {
    let mut genome = Genome::default_ashti_core();
    genome.config.ashti_domain_count = 5;
    let result = AxiomEngine::try_new(Arc::new(genome));
    let err = match result { Err(e) => e, Ok(_) => panic!("expected Err") };
    let msg = format!("{err}");
    assert!(msg.contains("GENOME"), "error message should mention GENOME: {msg}");
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
    let cmd = make_cmd(OpCode::CollapseDomain, LOGIC_ID as u32);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
}

// ============================================================
// process_command: InjectToken
// ============================================================

#[test]
fn test_inject_token_into_valid_domain() {
    let mut engine = AxiomEngine::new();

    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID as u32, 100, 0);
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
fn test_tick_forward_empty_engine_no_events() {
    // Без токенов frontier пуст — физических событий нет
    let mut engine = AxiomEngine::new();
    let cmd = make_cmd(OpCode::TickForward, 0);
    let result = engine.process_command(&cmd);
    assert!(result.is_success());
    assert_eq!(result.events_generated, 0);
    assert!(engine.drain_events().is_empty());
}

#[test]
fn test_drain_events_populated_after_heartbeat() {
    // HeartbeatConfig::medium() имеет interval=1024.
    // После 1025 тиков домен получит первый пульс и запустит process_frontier,
    // который сгенерирует события для инжектированного токена.
    let mut engine = AxiomEngine::new();

    let mut inject = UclCommand::new(OpCode::InjectToken, LOGIC_ID as u32, 1, 0);
    inject.payload[0] = (LOGIC_ID & 0xff) as u8;
    inject.payload[1] = (LOGIC_ID >> 8) as u8;
    engine.process_command(&inject);

    let mut got_events = false;
    for _ in 0..1100 {
        engine.process_command(&make_cmd(OpCode::TickForward, 0));
        let events = engine.drain_events();
        if !events.is_empty() {
            got_events = true;
            break;
        }
    }
    assert!(got_events, "ожидались физические события после heartbeat pulse");
}

#[test]
fn test_drain_events_clears_buffer() {
    let mut engine = AxiomEngine::new();

    let mut inject = UclCommand::new(OpCode::InjectToken, LOGIC_ID as u32, 1, 0);
    inject.payload[0] = (LOGIC_ID & 0xff) as u8;
    inject.payload[1] = (LOGIC_ID >> 8) as u8;
    engine.process_command(&inject);

    for _ in 0..1100 {
        engine.process_command(&make_cmd(OpCode::TickForward, 0));
    }
    engine.drain_events(); // сброс
    assert!(engine.drain_events().is_empty()); // повторный вызов — пусто
}

// ============================================================
// process_command: CoreReset
// ============================================================

#[test]
fn test_core_reset_reinitialises_engine() {
    let mut engine = AxiomEngine::new();
    // inject token
    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID as u32, 100, 0);
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
// ============================================================
// GUARDIAN_CHECK_REQUIRED бит — Шаг 4
// ============================================================

/// Проверяем что константа GUARDIAN_CHECK_REQUIRED определена и равна 0x04
#[test]
fn test_guardian_check_required_constant() {
    assert_eq!(GUARDIAN_CHECK_REQUIRED, 0x04);
}

// ============================================================
// Нереализованные UCL опкоды (D-07) — no-op, не ошибка
// ============================================================

#[test]
fn test_unimplemented_opcodes_return_success() {
    let mut engine = AxiomEngine::new();
    // BondTokens и ReinforceFrame — реализованы (не no-op), поэтому не входят сюда
    let unimplemented = [
        OpCode::LockMembrane,
        OpCode::ReshapeDomain,
        OpCode::ApplyForce,
        OpCode::AnnihilateToken,
        OpCode::SplitToken,
        OpCode::ChangeTemperature,
        OpCode::ApplyGravity,
        OpCode::PhaseTransition,
    ];
    for opcode in unimplemented {
        let result = engine.process_command(&make_cmd(opcode, 0));
        assert!(result.is_success(), "{opcode:?} должен возвращать Success, не UNKNOWN_OPCODE");
        assert_eq!(result.events_generated, 0);
    }
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

// ============================================================
// inject_anchor_tokens
// ============================================================

#[test]
fn test_inject_anchor_tokens_empty_set() {
    let mut engine = AxiomEngine::new();
    let set = axiom_config::AnchorSet::empty();
    let n = engine.inject_anchor_tokens(&set);
    assert_eq!(n, 0);
}

#[test]
fn test_inject_anchor_tokens_axes() {
    use axiom_config::{Anchor, AnchorSet};
    let mut set = AnchorSet::empty();
    set.axes.push(Anchor {
        id: "ax_x".to_string(),
        word: "порядок".to_string(),
        aliases: vec![],
        tags: vec![],
        position: [30000, 0, 0],
        shell: [0; 8],
        description: String::new(),
    });
    let mut engine = AxiomEngine::new();
    let before = engine.token_count(100); // SUTRA
    let n = engine.inject_anchor_tokens(&set);
    assert_eq!(n, 1);
    assert_eq!(engine.token_count(100), before + 1);
}

#[test]
fn test_inject_anchor_tokens_domain() {
    use axiom_config::{Anchor, AnchorSet};
    let mut set = AnchorSet::empty();
    // D1 = index 0 → domain_id=101 (EXECUTION)
    set.domains[0].push(Anchor {
        id: "exec".to_string(),
        word: "действие".to_string(),
        aliases: vec![],
        tags: vec![],
        position: [0, 0, 20000],
        shell: [0; 8],
        description: String::new(),
    });
    let mut engine = AxiomEngine::new();
    let before = engine.token_count(101); // EXECUTION
    let n = engine.inject_anchor_tokens(&set);
    assert_eq!(n, 1);
    assert_eq!(engine.token_count(101), before + 1);
}
