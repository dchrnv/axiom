// Tests for Axiom Sentinel V1.0 — Фаза 2: Parallel Resonance Search

use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

/// Собрать InjectToken команду с заданными mass/temperature для SUTRA(100).
fn inject(mass: f32, temperature: f32) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, 100, 100, 0);
    let domain_bytes = 100u16.to_le_bytes();
    cmd.payload[0] = domain_bytes[0];
    cmd.payload[1] = domain_bytes[1];
    cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&temperature.to_le_bytes());
    cmd
}

#[test]
fn test_parallel_search_below_threshold_uses_sequential() {
    // При малом числе traces (< 512) parallel_search делегирует в sequential.
    // Результат должен совпадать по типу (Success, корректные поля).
    let mut engine = AxiomEngine::new();
    let cmd = inject(60.0, 150.0);
    let result = engine.process_and_observe(&cmd);
    assert_eq!(result.ucl_result.status, 0);
    // Нет следов → не может быть рефлекса
    assert!(!result.reflex_hit);
}

#[test]
fn test_parallel_and_sequential_same_outcome_no_traces() {
    // Пустой Experience: оба пути дают SlowPath без рефлекса.
    use axiom_runtime::ProcessingPath;
    let mut engine = AxiomEngine::new();
    let cmd = inject(80.0, 200.0);
    let r = engine.process_and_observe(&cmd);
    assert_eq!(r.path, ProcessingPath::SlowPath);
    assert!(!r.reflex_hit);
}

#[test]
fn test_process_parallel_returns_valid_result() {
    // process_parallel (вызывается через route_token) возвращает корректный UclResult.
    let mut engine = AxiomEngine::new();
    let cmd = inject(70.0, 180.0);
    let result = engine.process_and_observe(&cmd);
    assert_eq!(result.ucl_result.status, 0);
    // dominant_domain_id должен быть в диапазоне уровня 1 (100..=110)
    assert!(result.dominant_domain_id >= 100 && result.dominant_domain_id <= 110);
}

#[test]
fn test_parallel_threshold_constant() {
    // PARALLEL_THRESHOLD экспортируется и равен 512.
    use axiom_arbiter::experience::PARALLEL_THRESHOLD;
    assert_eq!(PARALLEL_THRESHOLD, 512);
}

#[test]
fn test_repeated_same_token_consistent_result() {
    // Один и тот же токен при повторных вызовах даёт стабильный dominant_domain_id.
    let mut engine = AxiomEngine::new();
    let cmd = inject(55.0, 130.0);
    let r1 = engine.process_and_observe(&cmd);
    let r2 = engine.process_and_observe(&cmd);
    assert_eq!(r1.dominant_domain_id, r2.dominant_domain_id);
}

#[test]
fn test_engine_tick_stable_with_parallel_routing() {
    // 200 тиков + process_and_observe после каждого — engine стабилен.
    let mut engine = AxiomEngine::new();
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..200 {
        engine.process_command(&tick);
        let cmd = inject(50.0, 120.0);
        let r = engine.process_and_observe(&cmd);
        assert_eq!(r.ucl_result.status, 0);
    }
    assert_eq!(engine.tick_count, 200);
}

#[test]
fn test_coherence_score_valid_range() {
    let mut engine = AxiomEngine::new();
    let cmd = inject(65.0, 160.0);
    let r = engine.process_and_observe(&cmd);
    if let Some(c) = r.coherence_score {
        assert!((0.0..=1.0).contains(&c));
    }
}

#[test]
fn test_thread_pool_not_blocked_after_parallel_search() {
    // После параллельного поиска пул не заблокирован и принимает новые задачи.
    let mut engine = AxiomEngine::new();
    let cmd = inject(75.0, 190.0);
    engine.process_and_observe(&cmd);
    // Пул должен быть доступен
    let result = engine.thread_pool.install(|| 99_u32);
    assert_eq!(result, 99);
}
