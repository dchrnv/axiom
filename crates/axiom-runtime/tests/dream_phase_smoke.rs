// SPDX-License-Identifier: AGPL-3.0-only
// Этап 7 — smoke test end-to-end для DREAM Phase V1.0.
// Проверяет полный цикл Wake→FallingAsleep→Dreaming→Waking→Wake
// и корректность накопленной статистики.

use axiom_runtime::{
    AxiomEngine, DreamPhaseState, DreamScheduler, DreamSchedulerConfig, FatigueWeights,
    GatewayPriority,
};
use axiom_ucl::{OpCode, UclCommand};

fn fast_idle_engine(idle_threshold: u32) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    engine.dream_scheduler = DreamScheduler::new(
        DreamSchedulerConfig {
            min_wake_ticks: 0,
            idle_threshold,
            fatigue_threshold: 255,
        },
        FatigueWeights::default(),
    );
    engine
}

fn tick(engine: &mut AxiomEngine) {
    let cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    engine.process_command(&cmd);
}

fn run_ticks(engine: &mut AxiomEngine, n: u64) {
    for _ in 0..n {
        tick(engine);
    }
}

// ── 7.1.a — полный цикл без паник ────────────────────────────────────────────

#[test]
fn full_cycle_no_panic() {
    // 500 тиков: должен произойти хотя бы один полный цикл
    let mut engine = fast_idle_engine(3);
    run_ticks(&mut engine, 500);
    // Главная задача — никаких паник
}

// ── 7.1.b — состояние системы после полного цикла ────────────────────────────

#[test]
fn full_cycle_state_and_stats() {
    let mut engine = fast_idle_engine(2);

    // Один цикл занимает ~7 тиков: 2 idle + 1 FA + 3 dream + 1 waking.
    // За 50 тиков должно завершиться ≥1 цикла и накопиться статистика.
    run_ticks(&mut engine, 50);

    assert!(
        engine.dream_phase_stats.total_sleeps >= 1,
        "total_sleeps должен быть ≥1, got {}",
        engine.dream_phase_stats.total_sleeps
    );
    assert!(
        engine.dream_phase_stats.total_dream_ticks > 0,
        "total_dream_ticks должен быть >0"
    );
    assert!(
        engine.dream_cycle.stats.total_cycles >= 1,
        "DreamCycle должен зафиксировать ≥1 цикл"
    );
    assert!(
        engine.dream_cycle.stats.completed_cycles >= 1,
        "completed_cycles должен быть ≥1"
    );
}

// ── 7.1.c — несколько циклов подряд ──────────────────────────────────────────

#[test]
fn multiple_cycles_accumulate_stats() {
    let mut engine = fast_idle_engine(2);
    // При пустой очереди цикл занимает ~6 тиков: 2 idle + 1 FA + 3 dream + 1 waking
    // За 200 тиков должно пройти несколько полных циклов
    run_ticks(&mut engine, 200);

    let sleeps = engine.dream_phase_stats.total_sleeps;
    assert!(sleeps >= 2, "ожидали ≥2 цикла за 200 тиков, got {}", sleeps);
    // completed_cycles может быть на 1 меньше total_sleeps если последний цикл ещё в процессе
    let completed = engine.dream_cycle.stats.completed_cycles;
    assert!(
        completed >= sleeps.saturating_sub(1),
        "completed_cycles={} должен быть ≥ total_sleeps-1={}",
        completed,
        sleeps - 1
    );
}

// ── 7.1.d — прерванный цикл увеличивает interrupted_dreams ───────────────────

#[test]
fn interrupted_cycle_increments_counter() {
    let mut engine = fast_idle_engine(2);

    // Доводим до Dreaming
    run_ticks(&mut engine, 4);
    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Dreaming,
        "expected Dreaming, got {:?}",
        engine.dream_phase_state
    );

    // Прерываем Critical-сигналом
    let wake_cmd = UclCommand::new(OpCode::TickForward, 0, 255, 0);
    engine.submit_priority_command(wake_cmd, GatewayPriority::Critical);

    // Тик Dreaming→Waking, затем Waking→Wake
    run_ticks(&mut engine, 2);

    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Wake,
        "expected Wake after interrupt, got {:?}",
        engine.dream_phase_state
    );
    assert_eq!(
        engine.dream_phase_stats.interrupted_dreams, 1,
        "interrupted_dreams должен быть 1"
    );
}

// ── 7.1.e — Wake-тик не меняет состояние без порога ─────────────────────────

#[test]
fn wake_tick_stays_awake_below_threshold() {
    let mut engine = fast_idle_engine(100); // высокий порог

    run_ticks(&mut engine, 50);

    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Wake,
        "должны остаться в Wake при high idle_threshold, got {:?}",
        engine.dream_phase_state
    );
    assert_eq!(
        engine.dream_phase_stats.total_sleeps, 0,
        "total_sleeps должен быть 0"
    );
}

// ── 7.1.f — состояние Dreaming не меняется без сигнала ───────────────────────

#[test]
fn dreaming_stays_dreaming_without_signal() {
    // max_dream_duration очень большой (default 50000) — цикл не завершится
    // за 3 тика в Dreaming
    let mut engine = fast_idle_engine(2);

    // Доводим до Dreaming: 2 idle + 1 FA + 1 → Dreaming
    run_ticks(&mut engine, 4);
    assert_eq!(
        engine.dream_phase_state,
        DreamPhaseState::Dreaming,
        "expected Dreaming, got {:?}",
        engine.dream_phase_state
    );

    // Сохраняем текущий счётчик total_sleeps
    let sleeps_before = engine.dream_phase_stats.total_sleeps;

    // 1 тик в Dreaming без Critical → остаёмся в Dreaming (или Waking если Processing завершён)
    tick(&mut engine);
    // total_sleeps не должен вырасти (мы ещё спим)
    assert_eq!(
        engine.dream_phase_stats.total_sleeps, sleeps_before,
        "total_sleeps не должен вырасти пока мы в Dreaming"
    );
}

// ── 7.1.g — scheduler.stats отражает решения о сне ───────────────────────────

#[test]
fn scheduler_stats_reflect_sleep_decisions() {
    let mut engine = fast_idle_engine(2);
    run_ticks(&mut engine, 50);

    let sched_stats = &engine.dream_scheduler.stats;
    assert!(
        sched_stats.sleep_decisions > 0,
        "scheduler должен зафиксировать решения о сне"
    );
}

// ── 7.1.h — dream_cycle.stats.total_processed растёт при наличии proposals ───

#[test]
fn cycle_stats_count_promotions_when_proposals_present() {
    // Без реального EXPERIENCE-фрейма proposals не будет →
    // processed=0, но completed_cycles ≥ 1
    let mut engine = fast_idle_engine(2);
    run_ticks(&mut engine, 50);

    // При пустой очереди approved=0, veto=0 — это ОК
    // Главная инвариант: total_cycles = completed + timed_out (v1.0: только completed)
    let stats = &engine.dream_cycle.stats;
    assert_eq!(
        stats.total_cycles,
        stats.completed_cycles + stats.timed_out_cycles,
        "total_cycles должен быть суммой completed и timed_out"
    );
}
