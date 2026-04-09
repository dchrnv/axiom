// Axiom Sentinel V1.0 — Phase 3: Variable Tick Rate
//
// Тесты для AdaptiveTickRate:
//   - unit: алгоритм trigger/on_idle_tick (без Engine)
//   - integration: поле adaptive_tick в TickSchedule AxiomEngine

use axiom_runtime::{AdaptiveTickRate, TickRateReason, AxiomEngine, TickSchedule};

// ─── Unit: trigger ────────────────────────────────────────────────────────────

#[test]
fn test_trigger_increases_hz() {
    let mut a = AdaptiveTickRate::default();
    let before = a.current_hz;
    a.trigger(TickRateReason::ExternalInput);
    assert!(a.current_hz > before, "trigger должен повысить current_hz");
}

#[test]
fn test_trigger_clamps_at_max_hz() {
    let mut a = AdaptiveTickRate::default();
    // Несколько триггеров подряд — не должны выйти за max_hz
    for _ in 0..20 {
        a.trigger(TickRateReason::ExternalInput);
    }
    assert_eq!(a.current_hz, a.max_hz, "current_hz не должен превышать max_hz");
}

#[test]
fn test_trigger_resets_idle_ticks() {
    let mut a = AdaptiveTickRate::default();
    // Имитируем накопление idle_ticks
    for _ in 0..10 {
        a.on_idle_tick();
    }
    assert!(a.idle_ticks > 0, "idle_ticks должны накопиться");
    a.trigger(TickRateReason::ExternalInput);
    assert_eq!(a.idle_ticks, 0, "trigger должен сбросить idle_ticks");
}

#[test]
fn test_trigger_sets_last_reason() {
    let mut a = AdaptiveTickRate::default();
    a.trigger(TickRateReason::TensionHigh);
    assert_eq!(a.last_reason, TickRateReason::TensionHigh);
    a.trigger(TickRateReason::MultiPass);
    assert_eq!(a.last_reason, TickRateReason::MultiPass);
}

// ─── Unit: on_idle_tick ───────────────────────────────────────────────────────

#[test]
fn test_idle_decrements_after_cooldown() {
    let mut a = AdaptiveTickRate::default();
    // Поднимаем hz чтобы было куда снижаться
    a.trigger(TickRateReason::ExternalInput);
    let after_trigger = a.current_hz;

    // Нужно cooldown idle-тиков, чтобы hz снизился
    for _ in 0..a.cooldown {
        a.on_idle_tick();
    }
    assert!(
        a.current_hz < after_trigger,
        "hz должен снизиться после {} idle-тиков", a.cooldown
    );
}

#[test]
fn test_idle_clamps_at_min_hz() {
    let mut a = AdaptiveTickRate::default();
    // Много idle-тиков — hz не должен уйти ниже min_hz
    for _ in 0..10_000 {
        a.on_idle_tick();
    }
    assert_eq!(a.current_hz, a.min_hz, "current_hz не должен быть ниже min_hz");
}

#[test]
fn test_idle_at_min_hz_sets_idle_reason() {
    let mut a = AdaptiveTickRate::default();
    // Опускаем hz до min
    for _ in 0..10_000 {
        a.on_idle_tick();
    }
    assert_eq!(a.last_reason, TickRateReason::Idle, "при min_hz reason = Idle");
}

// ─── Unit: вспомогательные методы ────────────────────────────────────────────

#[test]
fn test_interval_ms_math() {
    let mut a = AdaptiveTickRate::default();
    // При default min_hz = 60 → 1000/60 = 16 ms
    assert_eq!(a.interval_ms(), 1000 / a.current_hz.max(1) as u64);

    a.trigger(TickRateReason::ExternalInput);
    // После триггера hz выросло, ms уменьшилось
    assert_eq!(a.interval_ms(), 1000 / a.current_hz.max(1) as u64);
}

#[test]
fn test_is_idle_at_min_hz() {
    let a = AdaptiveTickRate::default();
    // По умолчанию current_hz == min_hz → is_idle() = true
    assert!(a.is_idle(), "default state должен быть idle (current_hz == min_hz)");
}

#[test]
fn test_is_not_idle_after_trigger() {
    let mut a = AdaptiveTickRate::default();
    a.trigger(TickRateReason::ExternalInput);
    assert!(!a.is_idle(), "после trigger не должен быть idle");
}

// ─── Integration: TickSchedule + AxiomEngine ─────────────────────────────────

#[test]
fn test_tick_schedule_has_adaptive_tick() {
    let s = TickSchedule::default();
    // Поле должно существовать и иметь корректные дефолты
    assert_eq!(s.adaptive_tick.min_hz, 60);
    assert_eq!(s.adaptive_tick.max_hz, 1000);
    assert_eq!(s.adaptive_tick.current_hz, 60);
    assert_eq!(s.adaptive_tick.step_up, 200);
    assert_eq!(s.adaptive_tick.step_down, 20);
    assert_eq!(s.adaptive_tick.cooldown, 50);
}

#[test]
fn test_engine_tick_schedule_has_adaptive() {
    let engine = AxiomEngine::new();
    let a = &engine.tick_schedule.adaptive_tick;
    assert_eq!(a.min_hz, 60);
    assert_eq!(a.max_hz, 1000);
    assert!(a.current_hz >= a.min_hz && a.current_hz <= a.max_hz);
}

#[test]
fn test_adaptive_tick_mutable_via_engine() {
    let mut engine = AxiomEngine::new();
    engine.tick_schedule.adaptive_tick.trigger(TickRateReason::TensionHigh);
    let a = &engine.tick_schedule.adaptive_tick;
    assert!(a.current_hz > a.min_hz, "trigger через Engine должен повысить hz");
    assert_eq!(a.last_reason, TickRateReason::TensionHigh);
}

#[test]
fn test_hz_increases_on_tension_trigger() {
    let mut a = AdaptiveTickRate::default();
    let before = a.current_hz;
    a.trigger(TickRateReason::TensionHigh);
    assert!(a.current_hz > before, "TensionHigh должен повысить hz");
}

#[test]
fn test_hz_increases_on_multipass_trigger() {
    let mut a = AdaptiveTickRate::default();
    let before = a.current_hz;
    a.trigger(TickRateReason::MultiPass);
    assert!(a.current_hz > before, "MultiPass должен повысить hz");
}

#[test]
fn test_hz_decreases_after_idle_n_ticks() {
    let mut a = AdaptiveTickRate::default();
    a.trigger(TickRateReason::ExternalInput); // поднимаем hz
    let high_hz = a.current_hz;

    // cooldown idle-тиков → hz снижается
    for _ in 0..a.cooldown {
        a.on_idle_tick();
    }
    assert!(a.current_hz < high_hz, "hz должен снизиться после {} idle тиков", a.cooldown);
}
