// SPDX-License-Identifier: AGPL-3.0-only
// Этап 2 — тесты DreamScheduler + FatigueSnapshot на живом AxiomEngine.

use axiom_runtime::{
    AxiomEngine,
    DreamSchedulerConfig, FatigueWeights, FatigueSnapshot,
    SleepDecision, SleepTriggerKind,
};

// 2.5.a — collect_fatigue_snapshot не паникует на свежем Engine
#[test]
fn collect_fatigue_snapshot_does_not_panic() {
    let mut engine = AxiomEngine::new();
    let snap = engine.collect_fatigue_snapshot();
    // experience_capacity > 0 (домен сконфигурирован)
    assert!(snap.experience_capacity > 0, "experience_capacity={}", snap.experience_capacity);
}

// 2.5.b — ticks_since_last_check >= 1 (защита от деления на ноль)
#[test]
fn collect_fatigue_snapshot_ticks_since_at_least_one() {
    let mut engine = AxiomEngine::new();
    let snap = engine.collect_fatigue_snapshot();
    assert!(snap.ticks_since_last_check >= 1);
}

// 2.5.c — causal_horizon_delta == 0 на свежем Engine (ничего не изменилось)
#[test]
fn collect_fatigue_snapshot_horizon_delta_zero_on_fresh_engine() {
    let mut engine = AxiomEngine::new();
    let snap = engine.collect_fatigue_snapshot();
    assert_eq!(snap.causal_horizon_delta, 0);
}

// 2.5.d — DreamScheduler присутствует и возвращает StayAwake на первом тике
#[test]
fn dream_scheduler_stays_awake_initially() {
    let mut engine = AxiomEngine::new();
    let snap = engine.collect_fatigue_snapshot();
    let dec = engine.dream_scheduler.on_wake_tick(0, snap, true);
    assert_eq!(dec, SleepDecision::StayAwake,
        "should stay awake on first tick — min_wake_ticks not elapsed");
}

// 2.5.e — explicit_command переводит систему в GoToSleep после min_wake_ticks
#[test]
fn dream_scheduler_explicit_command_triggers_sleep() {
    let mut engine = AxiomEngine::new();
    // Переконфигурируем scheduler с min_wake=0 чтобы команда сработала сразу
    engine.dream_scheduler = axiom_runtime::DreamScheduler::new(
        DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold: 9999, fatigue_threshold: 255 },
        FatigueWeights::default(),
    );
    engine.dream_scheduler.submit_explicit_command(1);
    let snap = engine.collect_fatigue_snapshot();
    let dec = engine.dream_scheduler.on_wake_tick(0, snap, true);
    assert_eq!(dec, SleepDecision::GoToSleep(SleepTriggerKind::ExplicitCommand));
}

// 2.5.f — stats.sleep_decisions растёт с каждым засыпанием
#[test]
fn dream_scheduler_stats_increment_on_sleep() {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = axiom_runtime::DreamScheduler::new(
        DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold: 2, fatigue_threshold: 255 },
        FatigueWeights::default(),
    );
    let snap = FatigueSnapshot::default();
    // тик 0: idle=1 → не порог
    engine.dream_scheduler.on_wake_tick(0, snap, false);
    // тик 1: idle=2 → порог → GoToSleep
    engine.dream_scheduler.on_wake_tick(1, snap, false);
    assert_eq!(engine.dream_scheduler.stats.sleep_decisions, 1);
    assert_eq!(engine.dream_scheduler.stats.idle_triggers, 1);
}
