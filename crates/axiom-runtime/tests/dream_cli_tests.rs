// SPDX-License-Identifier: AGPL-3.0-only
// Этап 6 — тесты CLI команд и BroadcastSnapshot.

use axiom_runtime::{AxiomEngine, DreamPhaseState, DreamSchedulerConfig, DreamScheduler, FatigueWeights};

fn make_idle_engine(idle_threshold: u32) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold, fatigue_threshold: 255 },
        FatigueWeights::default(),
    );
    engine
}

fn run_ticks(engine: &mut AxiomEngine, n: u64) {
    let cmd = axiom_ucl::UclCommand::new(axiom_ucl::OpCode::TickForward, 0, 100, 0);
    for _ in 0..n { engine.process_command(&cmd); }
}

// ── 6.3.a — dream_stats accessor через scheduler ──────────────────────────────

#[test]
fn dream_stats_accessible_from_engine() {
    let engine = AxiomEngine::new();
    // Все поля доступны без паники
    let _fatigue = engine.dream_scheduler.current_fatigue();
    let _idle    = engine.dream_scheduler.current_idle_ticks();
    let _sleeps  = engine.dream_phase_stats.total_sleeps;
    let _dticks  = engine.dream_phase_stats.total_dream_ticks;
    let _sched   = &engine.dream_scheduler.stats;
    let _cycle   = &engine.dream_cycle.stats;
}

// ── 6.3.b — force-sleep triggers sleep ────────────────────────────────────────

#[test]
fn force_sleep_triggers_sleep_after_tick() {
    let mut engine = make_idle_engine(9999); // idle_threshold высокий — сам не засыпает
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Wake);

    // Эмулируем :force-sleep
    engine.dream_scheduler.submit_explicit_command(0);

    // Один тик: GoToSleep(Explicit) → FallingAsleep
    run_ticks(&mut engine, 1);
    assert_ne!(engine.dream_phase_state, DreamPhaseState::Wake,
        "after force-sleep + 1 tick should not be Wake, got {:?}", engine.dream_phase_state);
}

// ── 6.3.c — wake-up sends critical signal ─────────────────────────────────────

#[test]
fn wake_up_sends_critical_signal_to_sleeping_engine() {
    let mut engine = make_idle_engine(2);

    // Засыпаем
    run_ticks(&mut engine, 4);
    assert_eq!(engine.dream_phase_state, DreamPhaseState::Dreaming,
        "expected Dreaming, got {:?}", engine.dream_phase_state);

    // Эмулируем :wake-up
    let wake_cmd = axiom_ucl::UclCommand::new(axiom_ucl::OpCode::TickForward, 0, 255, 0);
    engine.submit_priority_command(wake_cmd, axiom_runtime::GatewayPriority::Critical);
    assert!(engine.has_critical_pending(), "critical signal должен быть в буфере");

    // Следующий тик: Dreaming → Waking (critical interrupt)
    run_ticks(&mut engine, 1);
    assert_ne!(engine.dream_phase_state, DreamPhaseState::Dreaming,
        "after wake-up should leave Dreaming, got {:?}", engine.dream_phase_state);
}

// ── 6.3.d — BroadcastSnapshot включает dream_phase ───────────────────────────

#[test]
#[cfg(feature = "adapters")]
fn broadcast_snapshot_includes_dream_phase() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot_for_broadcast();
    let dp = snap.dream_phase.expect("dream_phase should be present in snapshot");
    assert_eq!(dp.state, DreamPhaseState::Wake);
    assert_eq!(dp.stats.total_sleeps, 0);
}

// ── 6.3.e — dream_phase snapshot state matches engine state ──────────────────

#[test]
#[cfg(feature = "adapters")]
fn broadcast_snapshot_state_matches_engine() {
    let mut engine = make_idle_engine(2);
    run_ticks(&mut engine, 4); // → Dreaming
    let snap = engine.snapshot_for_broadcast();
    let dp = snap.dream_phase.unwrap();
    assert_eq!(dp.state, engine.dream_phase_state,
        "snapshot state {:?} != engine state {:?}", dp.state, engine.dream_phase_state);
}
