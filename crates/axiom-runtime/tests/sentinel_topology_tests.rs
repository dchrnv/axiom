// Tests for Axiom Sentinel V1.0 — Фаза 1: Hardware-Aware Topology

use axiom_runtime::AxiomEngine;

#[test]
fn test_worker_count_at_least_one() {
    let engine = AxiomEngine::new();
    assert!(engine.worker_count >= 1,
        "worker_count должен быть >= 1, получено: {}", engine.worker_count);
}

#[test]
fn test_worker_count_matches_available_parallelism() {
    let expected = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let engine = AxiomEngine::new();
    assert_eq!(engine.worker_count, expected,
        "worker_count должен совпадать с available_parallelism()");
}

#[test]
fn test_thread_pool_size_is_worker_count_minus_one() {
    let engine = AxiomEngine::new();
    let expected_threads = engine.worker_count.saturating_sub(1).max(1);
    let actual_threads = engine.thread_pool.current_num_threads();
    assert_eq!(actual_threads, expected_threads,
        "pool_threads={} expected={}", actual_threads, expected_threads);
}

#[test]
fn test_thread_pool_min_one_thread_even_on_single_core() {
    // Независимо от worker_count пул никогда не пустой
    let engine = AxiomEngine::new();
    assert!(engine.thread_pool.current_num_threads() >= 1);
}

#[test]
fn test_engine_functional_after_topology_init() {
    // Engine обрабатывает команды корректно после инициализации с ThreadPool
    use axiom_ucl::{UclCommand, OpCode};
    let mut engine = AxiomEngine::new();
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let result = engine.process_command(&tick);
    assert_eq!(result.status, 0); // Success
    assert_eq!(engine.tick_count, 1);
}

#[test]
fn test_restore_from_preserves_worker_count() {
    // restore_from вызывает new() → worker_count корректен в восстановленном engine
    let original = AxiomEngine::new();
    let snap = original.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.worker_count, original.worker_count);
}

#[test]
fn test_thread_pool_executes_work() {
    let engine = AxiomEngine::new();
    let result = engine.thread_pool.install(|| {
        42_u32
    });
    assert_eq!(result, 42);
}

#[test]
fn test_multiple_engines_independent_pools() {
    // Каждый AxiomEngine имеет собственный пул, не разделяет глобальный
    let e1 = AxiomEngine::new();
    let e2 = AxiomEngine::new();
    assert_eq!(e1.thread_pool.current_num_threads(), e2.thread_pool.current_num_threads());
    // Запускаем работу в обоих — не конфликтуют
    let r1 = e1.thread_pool.install(|| 1_u32);
    let r2 = e2.thread_pool.install(|| 2_u32);
    assert_eq!(r1, 1);
    assert_eq!(r2, 2);
}
