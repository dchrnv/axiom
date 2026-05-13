// S6: Speculative Layer — тесты prepare_speculative_grids + reconcile_all

use axiom_core::Token;
use axiom_domain::AshtiCore;

fn token_at(sutra_id: u32, x: i16, y: i16, z: i16) -> Token {
    Token::new(sutra_id, 1, [x, y, z], 1)
}

fn pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap()
}

// Включить rebuild и форсировать достижение порога для домена по индексу.
fn force_rebuild_needed(core: &mut AshtiCore, domain_idx: usize) {
    let domain = core.domain_mut(domain_idx).unwrap();
    domain.config.rebuild_frequency = 5;
    domain.events_since_rebuild = 5;
}

// ── базовые ─────────────────────────────────────────────────────────────────

#[test]
fn test_speculative_no_work_when_no_rebuild_needed() {
    let mut core = AshtiCore::new(0);
    let pool = pool();
    // Без токенов и без достижения порога — ни один домен не нуждается в rebuild.
    core.prepare_speculative_grids(&pool);
    let (hits, misses) = core.speculative_stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
}

#[test]
fn test_speculative_hit_on_reconcile() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    // Добавить токены в домен 1 (EXECUTION) и форсировать порог rebuild.
    let domain_id = core.domain_id_at(1).unwrap();
    core.inject_token(domain_id, token_at(10, 100, 200, 300)).unwrap();
    core.inject_token(domain_id, token_at(11, -100, 0, 50)).unwrap();
    force_rebuild_needed(&mut core, 1);

    // Подготовить спекулятивный грид.
    core.prepare_speculative_grids(&pool);

    // reconcile_all должен использовать pre-built грид.
    let _ = core.reconcile_all();

    let (hits, misses) = core.speculative_stats();
    assert_eq!(hits, 1, "один домен должен использовать speculative grid");
    assert_eq!(misses, 0);
}

#[test]
fn test_speculative_miss_without_prepare() {
    let mut core = AshtiCore::new(0);

    let domain_id = core.domain_id_at(2).unwrap();
    core.inject_token(domain_id, token_at(20, 50, 50, 50)).unwrap();
    force_rebuild_needed(&mut core, 2);

    // Без prepare_speculative_grids — reconcile_all делает обычный rebuild.
    let _ = core.reconcile_all();

    let (hits, misses) = core.speculative_stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 1, "должен быть один miss");
}

#[test]
fn test_speculative_grid_used_only_once() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    let domain_id = core.domain_id_at(3).unwrap();
    core.inject_token(domain_id, token_at(30, 10, 10, 10)).unwrap();
    force_rebuild_needed(&mut core, 3);

    core.prepare_speculative_grids(&pool);

    // Первый reconcile: использует speculative grid.
    let _ = core.reconcile_all();
    let (hits1, misses1) = core.speculative_stats();
    assert_eq!(hits1, 1);
    assert_eq!(misses1, 0);

    // Второй reconcile без prepare: порог ещё не достигнут — ни hits ни misses не прибавляется.
    let _ = core.reconcile_all();
    let (hits2, misses2) = core.speculative_stats();
    assert_eq!(hits2, 1, "грид уже был сброшен после первого reconcile");
    assert_eq!(misses2, 0);
}

#[test]
fn test_speculative_resets_events_since_rebuild() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    let domain_id = core.domain_id_at(4).unwrap();
    core.inject_token(domain_id, token_at(40, 0, 0, 0)).unwrap();
    force_rebuild_needed(&mut core, 4);

    assert!(core.domain(4).unwrap().should_rebuild_spatial_grid());

    core.prepare_speculative_grids(&pool);
    let _ = core.reconcile_all();

    // После reconcile events_since_rebuild должен быть сброшен.
    assert_eq!(core.domain(4).unwrap().events_since_rebuild, 0);
    assert!(!core.domain(4).unwrap().should_rebuild_spatial_grid());
}

// ── корректность грида ───────────────────────────────────────────────────────

#[test]
fn test_speculative_grid_has_correct_entry_count() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    let domain_id = core.domain_id_at(5).unwrap();
    let n = 8usize;
    for i in 0..n {
        core.inject_token(domain_id, token_at(50 + i as u32, i as i16 * 100, 0, 0))
            .unwrap();
    }
    force_rebuild_needed(&mut core, 5);
    core.prepare_speculative_grids(&pool);
    let _ = core.reconcile_all();

    assert_eq!(core.domain(5).unwrap().spatial_grid.entry_count, n);
}

#[test]
fn test_speculative_matches_normal_rebuild() {
    // Сравниваем entry_count грида через speculative path и через normal path.
    let mut core_spec = AshtiCore::new(0);
    let mut core_norm = AshtiCore::new(0);
    let pool = pool();

    let domain_id_spec = core_spec.domain_id_at(6).unwrap();
    let domain_id_norm = core_norm.domain_id_at(6).unwrap();

    for i in 0..5u32 {
        let t = token_at(60 + i, (i as i16) * 50, (i as i16) * 30, 0);
        core_spec.inject_token(domain_id_spec, t).unwrap();
        core_norm.inject_token(domain_id_norm, t).unwrap();
    }

    force_rebuild_needed(&mut core_spec, 6);
    force_rebuild_needed(&mut core_norm, 6);

    core_spec.prepare_speculative_grids(&pool);
    let _ = core_spec.reconcile_all();
    let _ = core_norm.reconcile_all();

    let count_spec = core_spec.domain(6).unwrap().spatial_grid.entry_count;
    let count_norm = core_norm.domain(6).unwrap().spatial_grid.entry_count;
    assert_eq!(count_spec, count_norm, "speculative и normal grid должны дать одинаковый entry_count");
}

// ── несколько доменов ────────────────────────────────────────────────────────

#[test]
fn test_speculative_multiple_domains_in_parallel() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    // Форсировать несколько доменов одновременно.
    for idx in [1usize, 3, 7] {
        let domain_id = core.domain_id_at(idx).unwrap();
        core.inject_token(domain_id, token_at(idx as u32 * 100, 10, 20, 30))
            .unwrap();
        force_rebuild_needed(&mut core, idx);
    }

    core.prepare_speculative_grids(&pool);
    let _ = core.reconcile_all();

    let (hits, misses) = core.speculative_stats();
    assert_eq!(hits, 3, "три домена должны использовать speculative grid");
    assert_eq!(misses, 0);
}

#[test]
fn test_speculative_empty_domain_no_panic() {
    let mut core = AshtiCore::new(0);
    let pool = pool();

    // Домен без токенов, но с форсированным порогом.
    force_rebuild_needed(&mut core, 0);

    // Не паникуем при нулевом количестве токенов.
    core.prepare_speculative_grids(&pool);
    let _ = core.reconcile_all();

    let (hits, _) = core.speculative_stats();
    assert_eq!(hits, 1);
    assert_eq!(core.domain(0).unwrap().spatial_grid.entry_count, 0);
}
