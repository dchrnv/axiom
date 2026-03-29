// Этап 7 Шаг 4 — Обмен скиллами: export/import_batch/clear
use axiom_arbiter::SkillSet;
use axiom_core::Token;

fn make_token(x: i16, temp: u8, mass: u8) -> Token {
    let mut t = Token::new(1, 1, [x, 0, 0], 1);
    t.temperature = temp;
    t.mass = mass;
    t
}

fn crystallize_skill(ss: &mut SkillSet, x: i16, temp: u8) -> bool {
    use axiom_arbiter::ExperienceModule;
    // Создаём ExperienceTrace вручную через add_trace + find_crystallizable
    let mut exp = ExperienceModule::new();
    let t = make_token(x, temp, 100);
    exp.add_trace(t, 0.9, 1);
    // Имитируем success_count через strengthen_by_hash
    let ph = {
        let mut h: u64 = 0xcbf29ce484222325;
        h ^= t.temperature as u64; h = h.wrapping_mul(0x100000001b3);
        h ^= t.mass as u64;        h = h.wrapping_mul(0x100000001b3);
        h ^= (t.valence as u8) as u64; h = h.wrapping_mul(0x100000001b3);
        h ^= t.position[0] as u64; h = h.wrapping_mul(0x100000001b3);
        h ^= t.position[1] as u64; h = h.wrapping_mul(0x100000001b3);
        h ^= t.position[2] as u64; h = h.wrapping_mul(0x100000001b3);
        h
    };
    for _ in 0..50 {
        exp.strengthen_by_hash(ph, 0.001);
    }
    let candidates = exp.find_crystallizable(0.8, 50);
    if candidates.is_empty() { return false; }
    ss.try_crystallize(&candidates[0])
}

// ─── export ──────────────────────────────────────────────────────────────────

#[test]
fn test_export_empty_skillset() {
    let ss = SkillSet::new();
    assert!(ss.export().is_empty());
}

#[test]
fn test_export_returns_clones() {
    let mut ss = SkillSet::new();
    crystallize_skill(&mut ss, 0, 100);
    let exported = ss.export();
    assert_eq!(exported.len(), ss.skill_count());
}

// ─── import_batch ────────────────────────────────────────────────────────────

#[test]
fn test_import_batch_basic() {
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 100);
    crystallize_skill(&mut src, 500, 200);
    let exported = src.export();
    assert_eq!(exported.len(), 2);

    let mut dst = SkillSet::new();
    let imported = dst.import_batch(&exported);
    assert_eq!(imported, 2);
    assert_eq!(dst.skill_count(), 2);
}

#[test]
fn test_import_batch_reduces_weight() {
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 100);
    let exported = src.export();
    let original_weight = exported[0].activation_weight;

    let mut dst = SkillSet::new();
    dst.import_batch(&exported);
    let imported_weight = dst.skills()[0].activation_weight;

    assert!(
        (imported_weight - original_weight * 0.3).abs() < 0.001,
        "imported weight = {} (expected {} × 0.3 = {})",
        imported_weight, original_weight, original_weight * 0.3
    );
}

#[test]
fn test_import_batch_resets_success_count() {
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 100);
    let exported = src.export();

    let mut dst = SkillSet::new();
    dst.import_batch(&exported);
    assert_eq!(dst.skills()[0].success_count, 0);
}

#[test]
fn test_import_batch_dedup_skips_similar() {
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 100);
    let exported = src.export();

    let mut dst = SkillSet::new();
    // Первый импорт
    dst.import_batch(&exported);
    assert_eq!(dst.skill_count(), 1);

    // Повторный импорт тех же — дубли пропускаются
    let second = dst.import_batch(&exported);
    assert_eq!(second, 0, "дубли пропущены");
    assert_eq!(dst.skill_count(), 1);
}

#[test]
fn test_import_batch_distinct_skills_all_imported() {
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 50);
    crystallize_skill(&mut src, 1000, 200);
    let exported = src.export();

    let mut dst = SkillSet::new();
    let imported = dst.import_batch(&exported);
    assert_eq!(imported, 2);
}

#[test]
fn test_import_batch_empty_slice() {
    let mut dst = SkillSet::new();
    let imported = dst.import_batch(&[]);
    assert_eq!(imported, 0);
    assert_eq!(dst.skill_count(), 0);
}

// ─── clear ───────────────────────────────────────────────────────────────────

#[test]
fn test_clear_empties_skillset() {
    let mut ss = SkillSet::new();
    crystallize_skill(&mut ss, 0, 100);
    assert_eq!(ss.skill_count(), 1);
    ss.clear();
    assert_eq!(ss.skill_count(), 0);
}

// ─── full exchange workflow ───────────────────────────────────────────────────

#[test]
fn test_full_export_import_workflow() {
    // Источник: кристаллизуем навык
    let mut src = SkillSet::new();
    crystallize_skill(&mut src, 0, 128);
    assert_eq!(src.skill_count(), 1);

    // Экспортируем
    let snapshot = src.export();

    // Получатель: чистый SkillSet, импортируем
    let mut dst = SkillSet::new();
    let n = dst.import_batch(&snapshot);
    assert_eq!(n, 1);
    assert_eq!(dst.skill_count(), 1);

    // Импортированный навык должен работать (найтись)
    let query = make_token(0, 128, 100);
    assert!(dst.find_skill(&query).is_some(), "импортированный навык находится");
}
