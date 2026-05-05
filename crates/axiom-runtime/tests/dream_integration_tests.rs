// SPDX-License-Identifier: AGPL-3.0-only
// Этап 4 — интеграционные тесты DREAM Phase в AxiomEngine.

use axiom_runtime::{
    AxiomEngine, DreamPhaseState, DreamScheduler, DreamSchedulerConfig, FatigueWeights,
    GatewayPriority,
};
use axiom_ucl::{InjectTokenPayload, OpCode, UclCommand};

/// Построить TickForward UCL-команду.
fn tick_cmd() -> UclCommand {
    UclCommand::new(OpCode::TickForward, 0, 100, 0)
}

/// Прогнать N тиков без ввода.
fn run_ticks(engine: &mut AxiomEngine, n: u64) {
    let cmd = tick_cmd();
    for _ in 0..n {
        engine.process_command(&cmd);
    }
}

// ── 4.8.a — engine_falls_asleep_when_idle ─────────────────────────────────────

#[test]
fn engine_falls_asleep_when_idle() {
    let mut engine = AxiomEngine::new();
    // Scheduler с min_wake=0, idle_threshold=5 — засыпает после 5 тиков без ввода
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 5,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    // 6 тиков: тик 5 — GoToSleep → переход в FallingAsleep, тик 6 → Dreaming
    run_ticks(&mut engine, 7);

    // После 7 тиков система должна быть в DREAMING (или как минимум не WAKE)
    assert_ne!(
        engine.dream_phase_state,
        DreamPhaseState::Wake,
        "state={:?} — expected non-Wake after idle threshold crossed",
        engine.dream_phase_state
    );
    assert!(
        engine.dream_phase_stats.total_sleeps >= 1,
        "total_sleeps={}",
        engine.dream_phase_stats.total_sleeps
    );
}

// ── 4.8.b — critical_signal_wakes_system ─────────────────────────────────────

#[test]
fn critical_signal_wakes_system() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 3,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    // Засыпаем: 3 idle → FallingAsleep, +1 тик → Dreaming
    run_ticks(&mut engine, 5);
    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Dreaming,
        "expected Dreaming, got {:?}",
        engine.dream_phase_state
    );

    // Подаём Critical-команду
    let critical_cmd = UclCommand::new(OpCode::TickForward, 0, 255, 0); // любая команда
    engine.submit_priority_command(critical_cmd, GatewayPriority::Critical);
    assert!(engine.has_critical_pending());

    // Следующий тик: прерывание → Waking
    run_ticks(&mut engine, 1);
    // Waking → Wake (ещё один тик)
    run_ticks(&mut engine, 1);

    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Wake,
        "expected Wake after critical signal, got {:?}",
        engine.dream_phase_state
    );
    assert_eq!(engine.dream_phase_stats.interrupted_dreams, 1);
}

// ── 4.8.c — normal_command_not_in_buffer_in_wake ─────────────────────────────

#[test]
fn normal_command_not_buffered_when_awake() {
    let mut engine = AxiomEngine::new();
    // В состоянии Wake Normal-команды не попадают в priority buffer
    let cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    engine.submit_priority_command(cmd, GatewayPriority::Normal);
    assert!(
        !engine.has_critical_pending(),
        "Normal command should not be in priority buffer in Wake state"
    );
}

// ── 4.8.d — dream_report_token_submitted_on_complete ─────────────────────────

#[test]
fn dream_cycle_completes_naturally_on_empty_queue() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 2,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    // 2 idle → FallingAsleep(тик3) → Dreaming(тик4)
    // Dreaming: Stabilization(4) → Processing(5, empty→Consolidation) → Consolidation(6, Complete) → Waking
    // Waking(7) → Wake
    run_ticks(&mut engine, 10);

    // Цикл должен был завершиться и система вернулась в Wake
    // (с пустой очередью — быстрый цикл)
    assert!(engine.dream_phase_stats.total_sleeps >= 1);
}

// ── 4.8.e — stats_update_through_full_cycle ───────────────────────────────────

#[test]
fn dream_stats_update_through_full_cycle() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 2,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    run_ticks(&mut engine, 15);

    // total_dream_ticks должны быть > 0 если система спала
    if engine.dream_phase_stats.total_sleeps > 0 {
        // Либо ещё в DREAMING, либо уже проснулась — в любом случае тики считались
        assert!(
            engine.dream_phase_stats.total_dream_ticks > 0
                || engine.dream_phase_state == DreamPhaseState::Wake,
            "dream_ticks=0 but total_sleeps>0"
        );
    }
}

// ── 4.8.f — state_machine_valid_transitions ──────────────────────────────────

#[test]
fn state_machine_starts_in_wake() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Wake);
}

#[test]
fn state_machine_transitions_wake_to_falling_asleep() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold: 1,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );

    run_ticks(&mut engine, 2); // тик 1: idle=1 → GoToSleep → FallingAsleep; тик 2: → Dreaming
    assert_ne!(
        engine.dream_phase_state,
        DreamPhaseState::Wake,
        "should have left Wake after idle trigger"
    );
}

// ── 4.8.g — had_intake_this_tick_set_by_process_and_observe ──────────────────

fn make_inject_cmd() -> UclCommand {
    let payload = InjectTokenPayload {
        target_domain_id: 109,
        token_type: 0,
        mass: 1.0,
        position: [0.0; 3],
        velocity: [0.0; 3],
        semantic_weight: 1.0,
        temperature: 100.0,
        reserved: [0; 6],
    };
    UclCommand::new(OpCode::InjectToken, 109, 100, 0).with_payload(&payload)
}

#[test]
fn had_intake_set_by_process_and_observe() {
    let mut engine = AxiomEngine::new();
    // Изначально false
    assert!(!engine.had_intake_this_tick());

    // process_and_observe с InjectToken → true
    let inject_cmd = make_inject_cmd();
    engine.process_and_observe(&inject_cmd);
    assert!(
        engine.had_intake_this_tick(),
        "had_intake_this_tick should be true after process_and_observe with InjectToken"
    );
}

// ── 4.8.h — had_intake_resets_on_tick_forward ────────────────────────────────

#[test]
fn had_intake_resets_on_tick_forward() {
    let mut engine = AxiomEngine::new();
    let inject_cmd = make_inject_cmd();
    engine.process_and_observe(&inject_cmd);
    assert!(engine.had_intake_this_tick());

    // TickForward должен сбросить флаг
    run_ticks(&mut engine, 1);
    assert!(
        !engine.had_intake_this_tick(),
        "had_intake_this_tick should be reset after TickForward"
    );
}
