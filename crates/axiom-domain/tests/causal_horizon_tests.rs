// Этап 7 Шаг 1 — CausalHorizon тесты
use axiom_core::Token;
use axiom_domain::{CausalHorizon, DomainConfig, DomainState};

fn make_state_with_tokens(event_ids: &[u64]) -> DomainState {
    let cfg = DomainConfig::factory_execution(1, 0);
    let mut state = DomainState::new(&cfg);
    for &ev in event_ids {
        let mut t = Token::new(1, 1, [0, 0, 0], ev);
        t.last_event_id = ev;
        let _ = state.add_token(t);
    }
    state
}

// ─── CausalHorizon::compute ───────────────────────────────────────────────────

#[test]
fn test_horizon_empty_domains_returns_zero() {
    let s = make_state_with_tokens(&[]);
    assert_eq!(CausalHorizon::compute(&[&s]), 0);
}

#[test]
fn test_horizon_single_token() {
    let s = make_state_with_tokens(&[42]);
    assert_eq!(CausalHorizon::compute(&[&s]), 42);
}

#[test]
fn test_horizon_min_across_tokens() {
    let s = make_state_with_tokens(&[100, 50, 200]);
    assert_eq!(CausalHorizon::compute(&[&s]), 50);
}

#[test]
fn test_horizon_min_across_domains() {
    let s1 = make_state_with_tokens(&[100, 200]);
    let s2 = make_state_with_tokens(&[30, 400]);
    assert_eq!(CausalHorizon::compute(&[&s1, &s2]), 30);
}

#[test]
fn test_horizon_ignores_zero_event_id() {
    let s = make_state_with_tokens(&[0, 50, 100]);
    // event_id=0 игнорируется — horizon = 50
    assert_eq!(CausalHorizon::compute(&[&s]), 50);
}

// ─── CausalHorizon::advance ───────────────────────────────────────────────────

#[test]
fn test_horizon_monotonically_increases() {
    let mut ch = CausalHorizon::new();

    // Первый advance — горизонт = 100
    let s1 = make_state_with_tokens(&[100]);
    ch.advance(&[&s1]);
    assert_eq!(ch.horizon, 100);

    // Добавляем домен с меньшим event_id — горизонт НЕ откатывается (монотонный)
    let s2 = make_state_with_tokens(&[10]);
    ch.advance(&[&s1, &s2]);
    assert_eq!(
        ch.horizon, 100,
        "монотонный — не убывает при появлении меньшего min"
    );

    // Все токены обновились — горизонт растёт
    let s3 = make_state_with_tokens(&[150]);
    ch.advance(&[&s3]);
    assert_eq!(ch.horizon, 150);
}

#[test]
fn test_horizon_does_not_decrease() {
    let mut ch = CausalHorizon::new();
    let s1 = make_state_with_tokens(&[200]);
    ch.advance(&[&s1]);
    assert_eq!(ch.horizon, 200);

    // Состояние без токенов → compute возвращает 0 → horizon не меняется
    let empty = make_state_with_tokens(&[]);
    ch.advance(&[&empty]);
    assert_eq!(ch.horizon, 200, "empty state must not reset horizon");
}

// ─── CausalHorizon::is_behind ────────────────────────────────────────────────

#[test]
fn test_is_behind_horizon_zero() {
    let ch = CausalHorizon::new(); // horizon = 0
    assert!(!ch.is_behind(5), "horizon=0 → nothing is behind");
}

#[test]
fn test_is_behind_true() {
    let ch = CausalHorizon {
        horizon: 100,
        archived_count: 0,
    };
    assert!(ch.is_behind(50));
    assert!(ch.is_behind(99));
    assert!(!ch.is_behind(100)); // равно — не за горизонтом
    assert!(!ch.is_behind(101));
}

// ─── Experience::archive_behind_horizon ──────────────────────────────────────

#[test]
fn test_archive_behind_horizon_removes_stale_traces() {
    use axiom_arbiter::ExperienceModule;

    let mut exp = ExperienceModule::new();

    // Добавляем следы с разными created_at (= last_used изначально)
    let t1 = Token::new(1, 1, [0, 0, 0], 10);
    let t2 = Token::new(1, 1, [0, 0, 0], 50);
    let t3 = Token::new(1, 1, [0, 0, 0], 200);
    exp.add_trace(t1, 0.5, 10);
    exp.add_trace(t2, 0.5, 50);
    exp.add_trace(t3, 0.5, 200);

    assert_eq!(exp.trace_count(), 3);
    assert_eq!(exp.index.trace_count(), 3);

    // Архивируем всё до горизонта 100 → t1(10) и t2(50) удаляются
    let removed = exp.archive_behind_horizon(100);
    assert_eq!(removed, 2);
    assert_eq!(exp.trace_count(), 1);
    assert_eq!(exp.index.trace_count(), 1);
}

#[test]
fn test_archive_behind_horizon_zero_is_noop() {
    use axiom_arbiter::ExperienceModule;

    let mut exp = ExperienceModule::new();
    let t = Token::new(1, 1, [0, 0, 0], 5);
    exp.add_trace(t, 0.5, 5);

    let removed = exp.archive_behind_horizon(0);
    assert_eq!(removed, 0);
    assert_eq!(exp.trace_count(), 1);
}

#[test]
fn test_archive_cleans_associative_index() {
    use axiom_arbiter::ExperienceModule;

    let mut exp = ExperienceModule::new();
    for i in 1u64..=5 {
        let t = Token::new(1, 1, [(i * 10) as i16, 0, 0], i);
        exp.add_trace(t, 0.5, i);
    }
    assert_eq!(exp.index.trace_count(), 5);

    exp.archive_behind_horizon(4); // удаляем 1,2,3
    assert_eq!(exp.index.trace_count(), 2); // остались 4,5
}

// ─── AshtiCore: compute_horizon + run_horizon_gc ─────────────────────────────

#[test]
fn test_ashti_compute_horizon_empty() {
    use axiom_domain::AshtiCore;
    let core = AshtiCore::new(1);
    // Нет токенов → горизонт = 0
    assert_eq!(core.compute_horizon(), 0);
}

#[test]
fn test_ashti_run_horizon_gc_noop_without_tokens() {
    use axiom_domain::AshtiCore;
    let mut core = AshtiCore::new(1);
    let removed = core.run_horizon_gc();
    assert_eq!(removed, 0);
}

#[test]
fn test_ashti_run_horizon_gc_removes_stale_experience() {
    use axiom_core::Token;
    use axiom_domain::AshtiCore;

    let mut core = AshtiCore::new(1);

    // Добавляем стары следы в Experience
    let exp = core.experience_mut();
    for i in 1u64..=3 {
        let t = Token::new(1, 109, [(i * 10) as i16, 0, 0], i);
        exp.add_trace(t, 0.5, i); // created_at = i, last_used = i
    }

    // Впрыскиваем токен в EXECUTION домен с большим event_id
    let domain_id = core.domain_id_at(1).unwrap(); // EXECUTION
    let mut tok = Token::new(1, domain_id, [0, 0, 0], 1000);
    tok.last_event_id = 1000;
    let _ = core.inject_token(domain_id, tok);

    // horizon = min(token.last_event_id) = 1000
    // Следы 1,2,3 < 1000 → архивируются
    let removed = core.run_horizon_gc();
    assert_eq!(removed, 3, "все 3 следа за горизонтом");
    assert_eq!(core.experience_mut().trace_count(), 0);
}
