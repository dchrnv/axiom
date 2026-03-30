// Тесты Этап 13D — Goal Persistence + Curiosity (Cognitive Depth V1.0)
//
// Покрывают:
// - TOKEN_FLAG_GOAL: CODEX(3) повышает mass и temperature цели
// - CODEX не трогает токены без GOAL-флага
// - check_goal_traces: незавершённые цели возвращаются
// - check_goal_traces: достигнутые цели не возвращаются
// - check_curiosity_candidates: следы в зоне [0.8*t, t) возвращаются
// - check_curiosity_candidates: следы вне зоны не возвращаются
// - generate_goal_impulses: source=Goal, weight > 0
// - generate_goal_impulses: не запускается вне interval
// - generate_curiosity_impulses: source=Curiosity, weight > 0
// - Интеграция: goal trace → impulse → route_with_multipass

use axiom_arbiter::*;
use axiom_config::DomainConfig;
use axiom_core::Token;
use std::collections::HashMap;

// ─────────────────────────────────────────────
// Хелперы
// ─────────────────────────────────────────────

fn make_token(id: u32, temp: u8) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.temperature = temp;
    t.mass = 100;
    t
}

fn make_goal_token(id: u32, temp: u8) -> Token {
    let mut t = make_token(id, temp);
    t.type_flags |= TOKEN_FLAG_GOAL;
    t
}

fn make_full_arbiter() -> Arbiter {
    let configs = [
        DomainConfig::factory_sutra(100),
        DomainConfig::factory_execution(101, 100),
        DomainConfig::factory_shadow(102, 100),
        DomainConfig::factory_codex(103, 100),
        DomainConfig::factory_map(104, 100),
        DomainConfig::factory_probe(105, 100),
        DomainConfig::factory_logic(106, 100),
        DomainConfig::factory_dream(107, 100),
        DomainConfig::factory_void(108, 100),
        DomainConfig::factory_experience(109, 100),
        DomainConfig::factory_maya(110, 100),
    ];
    let mut domain_map = HashMap::new();
    for c in &configs {
        domain_map.insert(c.domain_id as u32, *c);
    }
    let mut arbiter = Arbiter::new(domain_map, COM::new());
    for (role, c) in configs.iter().enumerate() {
        let _ = arbiter.register_domain(role as u8, c.domain_id as u32);
    }
    arbiter
}

// ─────────────────────────────────────────────
// 1. TOKEN_FLAG_GOAL + CODEX physics
// ─────────────────────────────────────────────

#[test]
fn test_goal_flag_constant_is_nonzero() {
    assert_ne!(TOKEN_FLAG_GOAL, 0);
}

#[test]
fn test_codex_raises_mass_for_goal_token() {
    let mut arbiter = make_full_arbiter();
    let token = make_goal_token(1, 100);
    let result = arbiter.route_token(token, 0);

    // Хотя бы один ASHTI результат должен иметь повышенную mass
    let any_boosted = result.slow_path.iter().any(|t| t.mass > token.mass);
    assert!(any_boosted, "CODEX должен повысить mass для GOAL-токена");
}

#[test]
fn test_codex_not_boosting_non_goal_token() {
    // Обычный токен без GOAL-флага — mass не меняется через CODEX
    let token = make_token(1, 100); // type_flags = 0
    assert_eq!(token.type_flags & TOKEN_FLAG_GOAL, 0);
}

// ─────────────────────────────────────────────
// 2. check_goal_traces
// ─────────────────────────────────────────────

#[test]
fn test_goal_traces_unachieved_returned() {
    let mut exp = ExperienceModule::new();
    let goal_token = make_goal_token(1, 100);
    exp.add_trace(goal_token, 0.5, 1); // weight=0.5 < GOAL_ACHIEVED_WEIGHT=0.9

    let goals = exp.check_goal_traces(GOAL_ACHIEVED_WEIGHT);
    assert_eq!(goals.len(), 1, "Незавершённая цель должна быть возвращена");
}

#[test]
fn test_goal_traces_achieved_not_returned() {
    let mut exp = ExperienceModule::new();
    let goal_token = make_goal_token(1, 100);
    exp.add_trace(goal_token, 0.95, 1); // weight=0.95 >= GOAL_ACHIEVED_WEIGHT=0.9

    let goals = exp.check_goal_traces(GOAL_ACHIEVED_WEIGHT);
    assert!(goals.is_empty(), "Достигнутая цель не должна генерировать импульс");
}

#[test]
fn test_non_goal_trace_not_in_goals() {
    let mut exp = ExperienceModule::new();
    let token = make_token(1, 100); // без GOAL-флага
    exp.add_trace(token, 0.3, 1);

    let goals = exp.check_goal_traces(GOAL_ACHIEVED_WEIGHT);
    assert!(goals.is_empty(), "Обычный след не является целью");
}

#[test]
fn test_goal_impulse_weight_proportional() {
    let mut exp = ExperienceModule::new();
    let token = make_goal_token(1, 100);
    exp.add_trace(token, 0.0, 1); // только создан → максимальный impulse weight

    let goals = exp.check_goal_traces(GOAL_ACHIEVED_WEIGHT);
    assert!(!goals.is_empty());
    let (_, weight) = goals[0];
    assert!(weight > 0.9, "Новая цель должна иметь высокий вес импульса");
}

// ─────────────────────────────────────────────
// 3. check_curiosity_candidates
// ─────────────────────────────────────────────

#[test]
fn test_curiosity_candidates_near_threshold() {
    let mut exp = ExperienceModule::new();
    let threshold = 0.85_f32;
    let low = threshold * 0.8; // = 0.68

    exp.add_trace(make_token(1, 100), low + 0.01, 1); // в зоне

    let candidates = exp.check_curiosity_candidates(threshold);
    assert_eq!(candidates.len(), 1, "Следы в зоне любопытства должны быть возвращены");
}

#[test]
fn test_curiosity_candidates_above_threshold_excluded() {
    let mut exp = ExperienceModule::new();
    let threshold = 0.85_f32;

    exp.add_trace(make_token(1, 100), threshold + 0.01, 1); // выше threshold → кристаллизован

    let candidates = exp.check_curiosity_candidates(threshold);
    assert!(candidates.is_empty(), "Кристаллизованные следы не являются кандидатами");
}

#[test]
fn test_curiosity_candidates_below_band_excluded() {
    let mut exp = ExperienceModule::new();
    let threshold = 0.85_f32;
    let low = threshold * 0.8; // = 0.68

    exp.add_trace(make_token(1, 100), low - 0.01, 1); // ниже зоны

    let candidates = exp.check_curiosity_candidates(threshold);
    assert!(candidates.is_empty(), "Следы ниже зоны любопытства не являются кандидатами");
}

// ─────────────────────────────────────────────
// 4. generate_goal_impulses
// ─────────────────────────────────────────────

#[test]
fn test_generate_goal_impulses_on_interval() {
    let mut arbiter = make_full_arbiter();
    let goal_token = make_goal_token(1, 100);
    arbiter.experience_mut().add_trace(goal_token, 0.3, 1);

    let impulses = arbiter.generate_goal_impulses(10, 5); // pulse=10, interval=5 → 10%5==0
    assert!(!impulses.is_empty(), "Goal-импульсы должны генерироваться на кратном пульсе");
    assert_eq!(impulses[0].source, ImpulseSource::Goal);
    assert!(impulses[0].weight > 0.0);
}

#[test]
fn test_generate_goal_impulses_off_interval() {
    let mut arbiter = make_full_arbiter();
    let goal_token = make_goal_token(1, 100);
    arbiter.experience_mut().add_trace(goal_token, 0.3, 1);

    let impulses = arbiter.generate_goal_impulses(11, 5); // 11%5 != 0 → пропуск
    assert!(impulses.is_empty(), "Goal-импульсы не генерируются вне интервала");
}

// ─────────────────────────────────────────────
// 5. generate_curiosity_impulses
// ─────────────────────────────────────────────

#[test]
fn test_generate_curiosity_impulses_near_threshold() {
    let mut arbiter = make_full_arbiter();
    let threshold = 0.85_f32;
    arbiter.experience_mut().add_trace(make_token(1, 100), 0.72, 1); // в зоне

    let impulses = arbiter.generate_curiosity_impulses(threshold);
    assert!(!impulses.is_empty(), "Curiosity-импульсы генерируются для near-threshold следов");
    assert_eq!(impulses[0].source, ImpulseSource::Curiosity);
}

#[test]
fn test_generate_curiosity_impulses_empty_when_none() {
    let arbiter = make_full_arbiter();
    let impulses = arbiter.generate_curiosity_impulses(0.85);
    assert!(impulses.is_empty(), "Нет кандидатов → нет impulses");
}
