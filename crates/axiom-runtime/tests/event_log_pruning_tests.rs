// Этап 7 Шаг 2 — Event Log Pruning тесты
use axiom_runtime::AxiomEngine;
use axiom_core::Token;

fn inject_token_with_event(engine: &mut AxiomEngine, domain_idx: usize, event_id: u64) {
    let domain_id = engine.ashti.domain_id_at(domain_idx).unwrap();
    let mut tok = Token::new(1, domain_id as u16, [0, 0, 0], event_id);
    tok.last_event_id = event_id;
    let _ = engine.ashti.inject_token(domain_id, tok);
}

fn add_experience_trace(engine: &mut AxiomEngine, last_used: u64) {
    let tok = Token::new(1, 109, [0, 0, 0], last_used);
    engine.ashti.experience_mut().add_trace(tok, 0.5, last_used);
}

// ─── snapshot.created_at отражает горизонт ────────────────────────────────────

#[test]
fn test_snapshot_created_at_zero_without_tokens() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot();
    assert_eq!(snap.created_at, 0, "нет токенов → горизонт = 0");
}

#[test]
fn test_snapshot_created_at_equals_causal_horizon() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 500);
    inject_token_with_event(&mut engine, 2, 300);
    inject_token_with_event(&mut engine, 3, 800);

    let horizon = engine.causal_horizon();
    let snap = engine.snapshot();
    assert_eq!(snap.snapshot_event_id(), horizon);
    assert_eq!(horizon, 300, "min = 300");
}

#[test]
fn test_snapshot_event_id_accessor() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 100);

    let snap = engine.snapshot();
    assert_eq!(snap.snapshot_event_id(), snap.created_at);
}

// ─── prune_after_snapshot ─────────────────────────────────────────────────────

#[test]
fn test_prune_noop_without_traces() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 100);

    let snap = engine.snapshot();
    let pruned = engine.prune_after_snapshot(&snap);
    assert_eq!(pruned, 0);
}

#[test]
fn test_prune_removes_stale_traces() {
    let mut engine = AxiomEngine::new();

    // Токены в доменах задают горизонт = 1000
    inject_token_with_event(&mut engine, 1, 1000);
    inject_token_with_event(&mut engine, 2, 2000);

    // Старые следы в Experience
    add_experience_trace(&mut engine, 10);
    add_experience_trace(&mut engine, 50);
    add_experience_trace(&mut engine, 999);
    // Свежий след
    add_experience_trace(&mut engine, 1500);

    assert_eq!(engine.ashti.experience_mut().trace_count(), 4);

    let snap = engine.snapshot(); // created_at = 1000
    assert_eq!(snap.snapshot_event_id(), 1000);

    let pruned = engine.prune_after_snapshot(&snap);
    assert_eq!(pruned, 3, "следы 10, 50, 999 < 1000 удалены");
    assert_eq!(engine.ashti.experience_mut().trace_count(), 1);
}

#[test]
fn test_prune_preserves_traces_at_or_after_horizon() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 100);

    add_experience_trace(&mut engine, 100); // exactly at horizon — сохраняется
    add_experience_trace(&mut engine, 200); // after — сохраняется

    let snap = engine.snapshot(); // created_at = 100
    let pruned = engine.prune_after_snapshot(&snap);
    assert_eq!(pruned, 0, "следы на горизонте и после него сохраняются");
    assert_eq!(engine.ashti.experience_mut().trace_count(), 2);
}

#[test]
fn test_prune_zero_snapshot_event_id_is_noop() {
    let mut engine = AxiomEngine::new();
    // Без токенов горизонт = 0
    add_experience_trace(&mut engine, 5);
    add_experience_trace(&mut engine, 10);

    let snap = engine.snapshot(); // created_at = 0
    assert_eq!(snap.snapshot_event_id(), 0);

    let pruned = engine.prune_after_snapshot(&snap);
    assert_eq!(pruned, 0, "horizon=0 → ничего не удаляем");
    assert_eq!(engine.ashti.experience_mut().trace_count(), 2);
}

// ─── snapshot_and_prune ───────────────────────────────────────────────────────

#[test]
fn test_snapshot_and_prune_combined() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 500);

    add_experience_trace(&mut engine, 100);
    add_experience_trace(&mut engine, 300);
    add_experience_trace(&mut engine, 600);

    let (snap, pruned) = engine.snapshot_and_prune();
    assert_eq!(snap.snapshot_event_id(), 500);
    assert_eq!(pruned, 2, "следы 100 и 300 < 500");
    assert_eq!(engine.ashti.experience_mut().trace_count(), 1);
}

#[test]
fn test_snapshot_and_prune_captures_tokens() {
    let mut engine = AxiomEngine::new();
    let domain_id = engine.ashti.domain_id_at(1).unwrap();
    let mut tok = Token::new(1, domain_id as u16, [10, 20, 30], 42);
    tok.last_event_id = 42;
    let _ = engine.ashti.inject_token(domain_id, tok);

    let (snap, _) = engine.snapshot_and_prune();
    let domain_snap = snap.find_domain(domain_id).unwrap();
    assert_eq!(domain_snap.tokens.len(), 1);
    assert_eq!(domain_snap.tokens[0].position, [10, 20, 30]);
}

// ─── prunable_count ───────────────────────────────────────────────────────────

#[test]
fn test_prunable_count_inspection() {
    let mut engine = AxiomEngine::new();
    inject_token_with_event(&mut engine, 1, 100);

    add_experience_trace(&mut engine, 10);
    add_experience_trace(&mut engine, 50);
    add_experience_trace(&mut engine, 150);

    let horizon = engine.causal_horizon(); // 100
    let count = engine.ashti.experience_mut().prunable_count(horizon);
    assert_eq!(count, 2);
}
