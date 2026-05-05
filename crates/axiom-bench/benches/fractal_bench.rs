// Бенчмарки Этап 12: FractalChain + SIMD batch gravity
//
// Охват:
//   12A: FractalChain::new, tick (N levels), inject+take, exchange_skills
//   12B: apply_gravity_batch vs scalar loop (100..10_000 tokens)

use axiom_core::Token;
use axiom_domain::FractalChain;
use axiom_space::{apply_gravity_batch, compute_gravity, GravityModel};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn make_token(id: u32) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.mass = 100;
    t.temperature = 50;
    t
}

fn make_positions(n: usize) -> Vec<[i16; 3]> {
    (0..n)
        .map(|i| {
            let x = ((i * 37) % 2000) as i16 - 1000;
            let y = ((i * 53) % 2000) as i16 - 1000;
            let z = ((i * 71) % 500) as i16 - 250;
            [x, y, z]
        })
        .collect()
}

fn make_masses(n: usize) -> Vec<u16> {
    (0..n).map(|i| (100 + (i % 900)) as u16).collect()
}

// ─── 12A: FractalChain ───────────────────────────────────────────────────────

fn bench_chain_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("FractalChain::new");
    group.measurement_time(Duration::from_secs(5));

    for depth in [2usize, 3, 5] {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &d| {
            b.iter(|| black_box(FractalChain::new(d)))
        });
    }
    group.finish();
}

fn bench_chain_tick_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("FractalChain::tick/empty");
    group.measurement_time(Duration::from_secs(5));

    for depth in [2usize, 3, 5] {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &d| {
            let mut chain = FractalChain::new(d);
            b.iter(|| {
                let events = chain.tick();
                black_box(events.len())
            })
        });
    }
    group.finish();
}

fn bench_chain_tick_with_tokens(c: &mut Criterion) {
    let mut group = c.benchmark_group("FractalChain::tick/with_tokens");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(20);

    // 2-уровневая цепочка, inject 10 токенов, тикаем
    for n_tokens in [1usize, 10, 50] {
        group.bench_with_input(BenchmarkId::from_parameter(n_tokens), &n_tokens, |b, &n| {
            b.iter_batched(
                || {
                    let mut chain = FractalChain::new(2);
                    for i in 0..n {
                        let _ = chain.inject_input(make_token(i as u32 + 1));
                    }
                    chain
                },
                |mut chain| {
                    let events = chain.tick();
                    black_box(events.len())
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn bench_chain_inject_take(c: &mut Criterion) {
    let mut group = c.benchmark_group("FractalChain::inject_input+take_output");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("inject_input", |b| {
        let mut chain = FractalChain::new(2);
        b.iter(|| {
            let _ = chain.inject_input(black_box(make_token(1)));
            // drain so capacity doesn't fill
            let _ = chain.level_mut(0).unwrap().take_maya_output();
        })
    });

    group.bench_function("take_output_empty", |b| {
        let mut chain = FractalChain::new(2);
        b.iter(|| black_box(chain.take_output()))
    });

    group.finish();
}

fn bench_chain_exchange_skills(c: &mut Criterion) {
    let mut group = c.benchmark_group("FractalChain::exchange_skills");
    group.measurement_time(Duration::from_secs(5));

    for depth in [2usize, 3, 5] {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &d| {
            let mut chain = FractalChain::new(d);
            b.iter(|| black_box(chain.exchange_skills()))
        });
    }
    group.finish();
}

// ─── 12B: apply_gravity_batch vs scalar ──────────────────────────────────────

fn bench_gravity_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_gravity_batch");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(30);

    for n in [100usize, 500, 1_000, 5_000, 10_000] {
        let positions = make_positions(n);
        let masses = make_masses(n);

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
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

fn bench_gravity_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("gravity_scalar_loop");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(30);

    for n in [100usize, 500, 1_000, 5_000, 10_000] {
        let positions = make_positions(n);
        let masses = make_masses(n);

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| {
                let mut out = Vec::with_capacity(positions.len());
                for (pos, &mass) in positions.iter().zip(masses.iter()) {
                    out.push(compute_gravity(
                        pos[0],
                        pos[1],
                        pos[2],
                        mass,
                        24,
                        GravityModel::Linear,
                    ));
                }
                black_box(out)
            })
        });
    }
    group.finish();
}

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    chain_benches,
    bench_chain_new,
    bench_chain_tick_empty,
    bench_chain_tick_with_tokens,
    bench_chain_inject_take,
    bench_chain_exchange_skills,
);

criterion_group!(simd_benches, bench_gravity_batch, bench_gravity_scalar,);

criterion_main!(chain_benches, simd_benches);
