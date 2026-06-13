// Стресс-бенчмарки AXIOM: 10K → 100K → 1M → 10M
//
// Горячие пути:
//   1. SpatialHashGrid::rebuild — хеш-таблица, ожидаем O(n) с cache pressure
//   2. resonance_search (Experience) — линейный поиск по HashMap
//   3. apply_subsystem_gravity (PRIM-TD-03) — семантическая гравитация

use axiom_arbiter::ExperienceModule;
use axiom_config::DomainConfig;
use axiom_core::{Token, STATE_ACTIVE};
use axiom_domain::DomainState;
use axiom_runtime::subsystem_gravity::{apply_subsystem_gravity, SubsystemGravityRule};
use axiom_space::SpatialHashGrid;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn make_pos_tuples(n: usize) -> Vec<(i16, i16, i16)> {
    (0..n)
        .map(|i| {
            let x = ((i.wrapping_mul(37)) % 60000) as i32 - 30000;
            let y = ((i.wrapping_mul(53)) % 60000) as i32 - 30000;
            let z = ((i.wrapping_mul(71)) % 10000) as i32 - 5000;
            (x as i16, y as i16, z as i16)
        })
        .collect()
}

// ─── 1. apply_gravity_batch: 10K → 10M ───────────────────────────────────────

fn bench_gravity_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/apply_gravity_batch");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &n in &[10_000usize, 100_000, 1_000_000, 10_000_000] {
        let positions = make_positions(n);
        let masses = make_masses(n);

        group.bench_with_input(BenchmarkId::new("tokens", n), &n, |b, _| {
            b.iter(|| {
                black_box(apply_gravity_batch(
                    black_box(&positions),
                    black_box(&masses),
                    24,
                    GravityModel::Linear,
                ))
            })
        });
    }
    group.finish();
}

// ─── 1b. apply_gravity_batch_avx2: shift=8, 10K → 1M ────────────────────────
// Реальная нагрузка: shift=8 даёт ненулевые силы, AVX2 path активен.

fn bench_gravity_avx2_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/apply_gravity_batch_avx2");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &n in &[10_000usize, 100_000, 1_000_000] {
        let positions = make_positions(n);
        let masses = make_masses(n);

        group.bench_with_input(BenchmarkId::new("tokens", n), &n, |b, _| {
            b.iter(|| {
                black_box(apply_gravity_batch_avx2(
                    black_box(&positions),
                    black_box(&masses),
                    8,
                    GravityModel::Linear,
                ))
            })
        });
    }
    group.finish();
}

// ─── 2. SpatialHashGrid::rebuild: 10K → 1M ───────────────────────────────────

fn bench_grid_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/SpatialHashGrid::rebuild");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &n in &[10_000usize, 50_000, 100_000, 500_000, 1_000_000] {
        let positions = make_pos_tuples(n);
        let mut grid = SpatialHashGrid::new();

        group.bench_with_input(BenchmarkId::new("tokens", n), &n, |b, _| {
            b.iter(|| {
                grid.rebuild(positions.len(), |i| positions[i]);
                black_box(&grid);
            })
        });
    }
    group.finish();
}

// ─── 3. Experience::resonance_search: 1K → 50K ───────────────────────────────

fn bench_resonance_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/resonance_search");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &n in &[1_000usize, 5_000, 10_000, 50_000] {
        let mut exp = ExperienceModule::new();
        for i in 0..n {
            let mut t = Token::new(i as u32 + 1, 100, [0, 0, 0], 1);
            t.temperature = (i % 256) as u8;
            t.mass = ((i * 3) % 256) as u8;
            exp.add_trace(t, 0.9, i as u64 + 1);
        }
        let query = Token::new(42, 100, [0, 0, 0], 1);

        group.bench_with_input(BenchmarkId::new("traces", n), &n, |b, _| {
            b.iter(|| black_box(exp.resonance_search(black_box(&query))))
        });
    }
    group.finish();
}

// ─── 4. apply_subsystem_gravity: 100 → 10K токенов (PRIM-TD-03) ─────────────

fn make_subsystem_rules() -> Vec<SubsystemGravityRule> {
    vec![
        SubsystemGravityRule {
            anchor_sutra_id: 1,
            anchor_position: [8000, 12000, 13000], // val_beneficial
            direction: 1.0,
            strength_factor: 0.20,
            radius: None,
            target_domain: 110,
        },
        SubsystemGravityRule {
            anchor_sutra_id: 2,
            anchor_position: [3000, 1000, 11000], // val_harmful
            direction: -1.0,
            strength_factor: 0.20,
            radius: None,
            target_domain: 110,
        },
        SubsystemGravityRule {
            anchor_sutra_id: 3,
            anchor_position: [13000, 10000, 14000], // abstraction_theory
            direction: 1.0,
            strength_factor: 0.08,
            radius: Some(8000),
            target_domain: 110,
        },
        SubsystemGravityRule {
            anchor_sutra_id: 4,
            anchor_position: [14000, 12000, 15000], // abstraction_constructor
            direction: 1.0,
            strength_factor: 0.08,
            radius: Some(8000),
            target_domain: 110,
        },
    ]
}

fn bench_subsystem_gravity(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/apply_subsystem_gravity");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let rules = make_subsystem_rules();
    let config = DomainConfig::default();

    for &n in &[100usize, 500, 1_000, 5_000, 10_000] {
        let mut state = DomainState::new(&config);
        for i in 0..n {
            let x = ((i.wrapping_mul(37)) % 60000) as i32 - 30000;
            let y = ((i.wrapping_mul(53)) % 60000) as i32 - 30000;
            let z = ((i.wrapping_mul(71)) % 10000) as i32 - 5000;
            let mut t = Token::new(i as u32 + 1, 110, [x as i16, y as i16, z as i16], 1);
            t.state = STATE_ACTIVE;
            t.mass = 100;
            let _ = state.add_token(t);
        }

        group.bench_with_input(BenchmarkId::new("tokens", n), &n, |b, _| {
            b.iter(|| apply_subsystem_gravity(black_box(&mut state), black_box(&rules)))
        });
    }
    group.finish();
}

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    stress,
    bench_grid_stress,
    bench_resonance_stress,
    bench_subsystem_gravity,
);

criterion_main!(stress);
