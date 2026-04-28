// Integration tests for axiom-runtime AxiomEngine
//
// После введения AshtiCore: AxiomEngine содержит фиксированный уровень из 11 доменов.
// domain_count() всегда == 11, arbiter_ready() всегда == true.
// Домены адресуются по schema: level_id(1) * 100 + role = 100..110.

use std::sync::Arc;
use axiom_runtime::{AxiomEngine, AxiomError, DreamPhaseState, GatewayPriority};
use axiom_genome::Genome;
use axiom_config::GUARDIAN_CHECK_REQUIRED;
use axiom_ucl::{UclCommand, OpCode, UnfoldFramePayload};
use axiom_core::{Token, Connection, FLAG_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR, FRAME_CATEGORY_SYNTAX};

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

// ============================================================
// UnfoldFrame (Этап 2 стабилизации FrameWeaver)
// ============================================================

const EXPERIENCE_ID: u16 = 109;
const SUTRA_ID:      u16 = 100;

/// Подготовить EXPERIENCE-Frame в engine.ashti: анкер + participants.
fn inject_test_frame(engine: &mut AxiomEngine, anchor_id: u32, participant_ids: &[u32]) {
    let mut anchor = Token::new(anchor_id, EXPERIENCE_ID, [0; 3], 0);
    anchor.type_flags = TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX;
    anchor.lineage_hash = anchor_id as u64 ^ 0xDEAD;
    engine.ashti.inject_token(EXPERIENCE_ID, anchor).unwrap();

    for (i, &pid) in participant_ids.iter().enumerate() {
        engine.ashti.inject_token(EXPERIENCE_ID, Token::new(pid, EXPERIENCE_ID, [0; 3], 0)).unwrap();
        let layer = (i as u8 % 8) + 1;
        let link_type = 0x0800u16 | ((layer as u16) << 4);
        let mut conn = Connection::new(anchor_id, pid, EXPERIENCE_ID, 1);
        conn.link_type = link_type;
        conn.flags = FLAG_ACTIVE;
        conn.reserved_gate[1] = 110; // origin_domain=MAYA
        engine.ashti.inject_connection(EXPERIENCE_ID, conn).unwrap();
    }
}

fn make_unfold_cmd(anchor_id: u32, target_domain: u16) -> UclCommand {
    let payload = UnfoldFramePayload {
        frame_anchor_id:  anchor_id,
        target_domain_id: target_domain,
        unfold_depth:     1,
        reserved:         [0; 41],
    };
    UclCommand::new(OpCode::UnfoldFrame, 0, 10, 0).with_payload(&payload)
}

#[test]
fn unfold_frame_to_target_domain() {
    let mut engine = AxiomEngine::new();
    inject_test_frame(&mut engine, 8000, &[8001, 8002, 8003]);

    let before_tokens = engine.token_count(LOGIC_ID);
    let before_conns = engine.ashti.state(engine.ashti.index_of(LOGIC_ID).unwrap()).unwrap().connections.len();

    let cmd = make_unfold_cmd(8000, LOGIC_ID);
    let result = engine.process_command(&cmd);
    assert_eq!(result.status, axiom_ucl::CommandStatus::Success as u8);

    // В LOGIC должен появиться новый токен-анкер
    assert_eq!(engine.token_count(LOGIC_ID), before_tokens + 1);
    // В LOGIC должны появиться 3 новые связи
    let after_conns = engine.ashti.state(engine.ashti.index_of(LOGIC_ID).unwrap()).unwrap().connections.len();
    assert_eq!(after_conns, before_conns + 3);

    // Оригинальный Frame в EXPERIENCE остался нетронутым
    assert!(engine.ashti.find_token_by_sutra_id(EXPERIENCE_ID, 8000).is_some());
}

#[test]
fn unfold_frame_source_auto_detect_experience() {
    let mut engine = AxiomEngine::new();
    inject_test_frame(&mut engine, 8100, &[8101, 8102]);

    let cmd = make_unfold_cmd(8100, LOGIC_ID);
    let result = engine.process_command(&cmd);
    assert_eq!(result.status, axiom_ucl::CommandStatus::Success as u8,
        "должен найти Frame в EXPERIENCE и успешно развернуть");
}

#[test]
fn unfold_frame_returns_error_for_missing_anchor() {
    let mut engine = AxiomEngine::new();
    // Анкера 9999 нет ни в EXPERIENCE, ни в SUTRA
    let cmd = make_unfold_cmd(9999, LOGIC_ID);
    let result = engine.process_command(&cmd);
    assert_ne!(result.status, axiom_ucl::CommandStatus::Success as u8,
        "должен вернуть ошибку при отсутствии анкера");
}

// ── DREAM Phase Stage 1: состояния и GatewayPriority ─────────────────────────

#[test]
fn dream_phase_state_starts_as_wake() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Wake,
        "AxiomEngine должен стартовать в состоянии Wake");
}

#[test]
fn dream_phase_state_default_is_wake() {
    assert_eq!(DreamPhaseState::default(), DreamPhaseState::Wake);
}

#[test]
fn gateway_priority_default_is_normal() {
    let p = GatewayPriority::default();
    assert_eq!(p, GatewayPriority::Normal,
        "GatewayPriority по умолчанию должен быть Normal");
}

#[test]
fn dream_phase_stats_total_sleeps_starts_zero() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.dream_phase_stats.total_sleeps, 0);
    assert_eq!(engine.dream_phase_stats.total_dream_ticks, 0);
    assert_eq!(engine.dream_phase_stats.interrupted_dreams, 0);
}
