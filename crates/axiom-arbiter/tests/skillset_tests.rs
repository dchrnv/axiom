use axiom_arbiter::{Skill, SkillSet};
use axiom_arbiter::ExperienceModule;
use axiom_core::Token;

fn token_at(x: i16, y: i16, z: i16) -> Token {
    let mut t = Token::new(1, 0, [x, y, z], 0);
    t.temperature = 100;
    t.mass = 50;
    t
}

fn make_strong_trace(token: Token) -> axiom_arbiter::ExperienceModule {
    // Используем Experience публично через ExperienceModule
    let mut exp = ExperienceModule::new();
    // Добавить trace с высоким весом — имитируем уже обученный опыт
    exp.add_trace(token, 0.85, 1);
    // Но success_count = 0, нужно strengthen_trace несколько раз
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    exp
}

// ─── SkillSet базовые ────────────────────────────────────────────────────────

#[test]
fn test_skillset_new() {
    let ss = SkillSet::new();
    assert_eq!(ss.skill_count(), 0);
    assert!((ss.crystallization_threshold - 0.8).abs() < 0.001);
    assert_eq!(ss.min_success_count, 50);
}

#[test]
fn test_skillset_find_skill_empty() {
    let ss = SkillSet::new();
    let t = token_at(0, 0, 0);
    assert!(ss.find_skill(&t).is_none());
}

// ─── Кристаллизация ──────────────────────────────────────────────────────────

#[test]
fn test_try_crystallize_below_weight_threshold() {
    let mut ss = SkillSet::new();
    let exp = ExperienceModule::new();
    // Нет следов, нечего кристаллизовать
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    assert!(candidates.is_empty());
    assert_eq!(ss.skill_count(), 0);
}

#[test]
fn test_try_crystallize_insufficient_success_count() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    // Высокий вес, но success_count = 0
    exp.add_trace(token_at(10, 20, 30), 0.9, 1);
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    assert!(candidates.is_empty()); // success_count = 0 < 50
    assert_eq!(ss.skill_count(), 0);
}

#[test]
fn test_try_crystallize_meets_criteria() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    exp.add_trace(token_at(10, 20, 30), 0.85, 1);
    // Strengthen до success_count = 50
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    assert!(!candidates.is_empty());
    let crystallized = ss.try_crystallize(&candidates[0]);
    assert!(crystallized);
    assert_eq!(ss.skill_count(), 1);
}

#[test]
fn test_try_crystallize_no_duplicate() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    exp.add_trace(token_at(10, 20, 30), 0.9, 1);
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    // Первый раз кристаллизует
    assert!(ss.try_crystallize(&candidates[0]));
    // Второй раз — дубликат, не создаёт
    assert!(!ss.try_crystallize(&candidates[0]));
    assert_eq!(ss.skill_count(), 1);
}

// ─── find_skill (активация) ───────────────────────────────────────────────────

#[test]
fn test_find_skill_exact_match() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    let t = token_at(100, 200, 50);
    exp.add_trace(t, 0.9, 1);
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    ss.try_crystallize(&candidates[0]);

    // Тот же токен должен активировать навык
    let found = ss.find_skill(&t);
    assert!(found.is_some());
    assert_eq!(found.unwrap().pattern.position, t.position);
}

#[test]
fn test_find_skill_no_match_different_position() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    exp.add_trace(token_at(100, 200, 50), 0.9, 1);
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    ss.try_crystallize(&candidates[0]);

    // Очень далёкий токен — не должен активировать навык
    let far_token = token_at(30000, 30000, 30000);
    assert!(ss.find_skill(&far_token).is_none());
}

// ─── record_activation ───────────────────────────────────────────────────────

#[test]
fn test_record_activation_increments_count() {
    let mut ss = SkillSet::new();
    let mut exp = ExperienceModule::new();
    exp.add_trace(token_at(0, 0, 0), 0.9, 1);
    for _ in 0..50 {
        exp.strengthen_trace(0, 0.001);
    }
    let candidates = exp.find_crystallizable(ss.crystallization_threshold, ss.min_success_count);
    ss.try_crystallize(&candidates[0]);

    ss.record_activation(0);
    ss.record_activation(0);
    assert_eq!(ss.skills()[0].success_count, 2);
}

// ─── import_skill ─────────────────────────────────────────────────────────────

#[test]
fn test_import_skill_lowers_weight() {
    let mut ss = SkillSet::new();
    let skill = Skill {
        pattern: token_at(0, 0, 0),
        activation_weight: 0.9,
        created_at: 1,
        success_count: 100,
        pattern_hash: 42,
    };
    ss.import_skill(skill);
    assert_eq!(ss.skill_count(), 1);
    // Вес должен снизиться до 0.9 * 0.3 ≈ 0.27
    assert!(ss.skills()[0].activation_weight < 0.35);
    assert_eq!(ss.skills()[0].success_count, 0);
}

// ─── find_crystallizable (Experience) ────────────────────────────────────────

#[test]
fn test_find_crystallizable_empty() {
    let exp = ExperienceModule::new();
    let result = exp.find_crystallizable(0.8, 50);
    assert!(result.is_empty());
}

#[test]
fn test_find_crystallizable_filters_weight() {
    let mut exp = ExperienceModule::new();
    exp.add_trace(token_at(0, 0, 0), 0.5, 1); // below threshold
    exp.add_trace(token_at(10, 0, 0), 0.9, 2); // above threshold but no success
    let result = exp.find_crystallizable(0.8, 0); // min_success = 0
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].pattern.position, [10, 0, 0]);
}
