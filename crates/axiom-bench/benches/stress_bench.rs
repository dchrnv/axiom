// Стресс-бенчмарки AXIOM: 10K → 100K → 1M → 10M
//
// Три горячих пути:
//   1. apply_gravity_batch  — чистые вычисления, должно быть линейно
//   2. SpatialHashGrid::rebuild — хеш-таблица, ожидаем O(n) с cache pressure
//   3. resonance_search (Experience) — линейный поиск по HashMap

use axiom_arbiter::ExperienceModule;
use axiom_core::Token;
use axiom_space::{apply_gravity_batch, apply_gravity_batch_avx2, GravityModel, SpatialHashGrid};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn make_positions(n: usize) -> Vec<[i16; 3]> {
    (0..n)
        .map(|i| {
            let x = ((i.wrapping_mul(37)) % 60000) as i16 - 30000;
            let y = ((i.wrapping_mul(53)) % 60000) as i16 - 30000;
            let z = ((i.wrapping_mul(71)) % 10000) as i16 - 5000;
            [x, y, z]
        })
        .collect()
}

fn make_masses(n: usize) -> Vec<u16> {
    (0..n).map(|i| (50 + (i % 950)) as u16).collect()
}

fn make_pos_tuples(n: usize) -> Vec<(i16, i16, i16)> {
    (0..n)
        .map(|i| {
            let x = ((i.wrapping_mul(37)) % 60000) as i16 - 30000;
            let y = ((i.wrapping_mul(53)) % 60000) as i16 - 30000;
            let z = ((i.wrapping_mul(71)) % 10000) as i16 - 5000;
            (x, y, z)
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

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    stress,
    bench_gravity_stress,
    bench_gravity_avx2_stress,
    bench_grid_stress,
    bench_resonance_stress,
);

criterion_main!(stress);
