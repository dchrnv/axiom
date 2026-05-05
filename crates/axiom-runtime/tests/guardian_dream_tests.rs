// SPDX-License-Identifier: AGPL-3.0-only
// Этап 5 — тесты GUARDIAN state-aware: SUTRA frame-anchor guard.

use axiom_core::{FRAME_CATEGORY_SYNTAX, STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_runtime::{
    AxiomEngine, DreamPhaseState, DreamScheduler, DreamSchedulerConfig, FatigueWeights,
};
use axiom_ucl::{flags as ucl_flags, InjectFrameAnchorPayload, OpCode, UclCommand};

/// Построить InjectFrameAnchor команду для целевого домена.
fn frame_anchor_cmd(target_domain: u16) -> UclCommand {
    let payload = InjectFrameAnchorPayload {
        lineage_hash: 0xDEAD_BEEF_1234_5678,
        proposed_sutra_id: 42,
        target_domain_id: target_domain,
        type_flags: TOKEN_FLAG_FRAME_ANCHOR | FRAME_CATEGORY_SYNTAX,
        position: [0; 3],
        state: STATE_ACTIVE,
        mass: 100,
        temperature: 200,
        valence: 0,
        reserved: [0; 22],
    };
    UclCommand::new(
        OpCode::InjectToken,
        target_domain as u32,
        200,
        ucl_flags::FRAME_ANCHOR,
    )
    .with_payload(&payload)
}

fn tick_cmd() -> UclCommand {
    UclCommand::new(OpCode::TickForward, 0, 100, 0)
}

fn run_ticks(engine: &mut AxiomEngine, n: u64) {
    let cmd = tick_cmd();
    for _ in 0..n {
        engine.process_command(&cmd);
    }
}

// ── 5.3.a — guardian_blocks_frame_anchor_sutra_write_in_wake ─────────────────

#[test]
fn guardian_blocks_frame_anchor_sutra_write_in_wake() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Wake);

    // InjectToken с FRAME_ANCHOR в SUTRA(100) в WAKE → GUARDIAN_VIOLATION
    let cmd = frame_anchor_cmd(100);
    let result =
        engine
            .guardian
            .check_frame_anchor_sutra_write(cmd.flags, 100, DreamPhaseState::Wake);
    assert!(result.is_some(), "должно быть вето в Wake");
}

// ── 5.3.b — guardian_allows_frame_anchor_sutra_write_in_dreaming ─────────────

#[test]
fn guardian_allows_frame_anchor_sutra_write_in_dreaming() {
    let engine = AxiomEngine::new();

    // В DREAMING — разрешено
    let result = engine.guardian.check_frame_anchor_sutra_write(
        ucl_flags::FRAME_ANCHOR,
        100,
        DreamPhaseState::Dreaming,
    );
    assert!(result.is_none(), "не должно быть вето в Dreaming");
}

// ── 5.3.c — guardian_allows_non_frame_anchor_writes_anywhere ─────────────────

#[test]
fn guardian_allows_regular_token_write_to_sutra_in_wake() {
    let engine = AxiomEngine::new();

    // Обычный InjectToken без FRAME_ANCHOR → не блокируется
    let result = engine.guardian.check_frame_anchor_sutra_write(
        0, // нет флага FRAME_ANCHOR
        100,
        DreamPhaseState::Wake,
    );
    assert!(
        result.is_none(),
        "обычные записи в SUTRA не должны блокироваться"
    );
}

// ── 5.3.d — guardian_allows_frame_anchor_to_experience_in_wake ───────────────

#[test]
fn guardian_allows_frame_anchor_to_experience_in_wake() {
    let engine = AxiomEngine::new();

    // FRAME_ANCHOR в EXPERIENCE(109) в WAKE → не SUTRA, разрешено
    let result = engine.guardian.check_frame_anchor_sutra_write(
        ucl_flags::FRAME_ANCHOR,
        109,
        DreamPhaseState::Wake,
    );
    assert!(
        result.is_none(),
        "EXPERIENCE writes должны быть разрешены в Wake"
    );
}

// ── 5.3.e — engine_rejects_frame_anchor_sutra_cmd_in_wake ────────────────────

#[test]
fn engine_rejects_frame_anchor_sutra_cmd_in_wake() {
    let mut engine = AxiomEngine::new();
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Wake);

    let cmd = frame_anchor_cmd(100);
    let result = engine.process_command(&cmd);

    // Ожидаем GUARDIAN_VIOLATION (SystemError, error_code 3001)
    use axiom_ucl::CommandStatus;
    assert_eq!(
        result.status,
        CommandStatus::SystemError as u8,
        "ожидается SystemError для SUTRA write в Wake"
    );
}

// ── 5.3.f — engine_accepts_frame_anchor_sutra_cmd_in_dreaming ────────────────

#[test]
fn engine_accepts_frame_anchor_experience_cmd_in_wake() {
    let mut engine = AxiomEngine::new();
    // FRAME_ANCHOR в EXPERIENCE — должен проходить в WAKE
    let cmd = frame_anchor_cmd(109);
    let result = engine.process_command(&cmd);

    use axiom_ucl::CommandStatus;
    assert_eq!(
        result.status,
        CommandStatus::Success as u8,
        "EXPERIENCE writes в Wake должны быть OK, got: {:?}",
        result.status
    );
}

// ── 5.3.g — full cycle: sleep, sutra write succeeds ──────────────────────────

#[test]
fn frame_anchor_sutra_write_succeeds_during_dreaming() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 2,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    // Входим в Dreaming: 2 idle → FallingAsleep, +1 → Dreaming
    run_ticks(&mut engine, 4);
    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Dreaming,
        "expected Dreaming, got {:?}",
        engine.dream_phase_state
    );

    // Теперь FRAME_ANCHOR в SUTRA должен проходить
    let cmd = frame_anchor_cmd(100);
    let result = engine.process_command(&cmd);

    use axiom_ucl::CommandStatus;
    assert_eq!(
        result.status,
        CommandStatus::Success as u8,
        "SUTRA frame anchor write в Dreaming должен быть OK"
    );
}
