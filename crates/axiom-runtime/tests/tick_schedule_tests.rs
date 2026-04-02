// Tests for TickSchedule and tick_count (Phase 6)

use axiom_runtime::{AxiomEngine, TickSchedule};
use axiom_ucl::{UclCommand, OpCode};

fn tick_cmd() -> UclCommand {
    UclCommand::new(OpCode::TickForward, 0, 100, 0)
}

// ============================================================
// tick_count: монотонность
// ============================================================

#[test]
fn test_tick_count_starts_at_zero() {
    let engine = AxiomEngine::new();
    assert_eq!(engine.tick_count, 0);
}

#[test]
fn test_tick_count_increments_per_tick() {
    let mut engine = AxiomEngine::new();
    engine.process_command(&tick_cmd());
    assert_eq!(engine.tick_count, 1);
    engine.process_command(&tick_cmd());
    assert_eq!(engine.tick_count, 2);
    engine.process_command(&tick_cmd());
    assert_eq!(engine.tick_count, 3);
}

#[test]
fn test_tick_count_after_many_ticks() {
    let mut engine = AxiomEngine::new();
    for _ in 0..100 {
        engine.process_command(&tick_cmd());
    }
    assert_eq!(engine.tick_count, 100);
}

// ============================================================
// TickSchedule: значения по умолчанию
// ============================================================

#[test]
fn test_tick_schedule_defaults() {
    let s = TickSchedule::default();
    assert_eq!(s.adaptation_interval,    50);
    assert_eq!(s.horizon_gc_interval,    500);
    assert_eq!(s.snapshot_interval,      5000);
    assert_eq!(s.dream_interval,         100);
    assert_eq!(s.tension_check_interval, 10);
    assert_eq!(s.goal_check_interval,    10);
    assert_eq!(s.reconcile_interval,     200);
}

#[test]
fn test_engine_uses_default_schedule() {
    let engine = AxiomEngine::new();
    let s = engine.tick_schedule;
    assert_eq!(s.adaptation_interval, 50);
}

// ============================================================
// TickSchedule: интервальный огонь
// ============================================================

#[test]
fn test_adaptation_fires_at_interval() {
    let mut engine = AxiomEngine::new();
    // adaptation_interval = 50, запустим ровно 50 тиков — не должно паниковать
    for _ in 0..50 {
        engine.process_command(&tick_cmd());
    }
    assert_eq!(engine.tick_count, 50);
    assert!(engine.process_command(&tick_cmd()).is_success());
}

#[test]
fn test_disabled_interval_does_not_panic() {
    let mut engine = AxiomEngine::new();
    engine.tick_schedule.tension_check_interval = 0;
    engine.tick_schedule.goal_check_interval    = 0;
    for _ in 0..20 {
        let r = engine.process_command(&tick_cmd());
        assert!(r.is_success());
    }
}

#[test]
fn test_custom_schedule_is_respected() {
    let mut engine = AxiomEngine::new();
    // Ставим очень маленький интервал — всё должно работать без паники
    engine.tick_schedule.adaptation_interval    = 1;
    engine.tick_schedule.dream_interval         = 1;
    engine.tick_schedule.tension_check_interval = 1;
    engine.tick_schedule.goal_check_interval    = 1;

    for _ in 0..10 {
        let r = engine.process_command(&tick_cmd());
        assert!(r.is_success());
    }
    assert_eq!(engine.tick_count, 10);
}
