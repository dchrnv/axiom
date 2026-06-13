// Бенчмарки Этап 12: FractalChain
//
// Охват:
//   12A: FractalChain::new, tick (N levels), inject+take, exchange_skills

use axiom_core::Token;
use axiom_domain::FractalChain;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn make_token(id: u32) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.mass = 100;
    t.temperature = 50;
    t
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

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    chain_benches,
    bench_chain_new,
    bench_chain_tick_empty,
    bench_chain_tick_with_tokens,
    bench_chain_inject_take,
    bench_chain_exchange_skills,
);

criterion_main!(chain_benches);
