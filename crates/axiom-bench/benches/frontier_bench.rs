// Бенчмарки axiom-frontier: Storm Control, state machine, batch events
//
// Целевые показатели (Этап 2):
//   - Overhead state machine transitions: минимальный
//   - TickForward с 1000+ токенами под Storm Control: без зависания

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use axiom_frontier::{CausalFrontier, FrontierConfig};
use std::time::Duration;

// ─────────────────────────────────────────────────────────────────────────────
// push + pop без шторма — baseline
// ─────────────────────────────────────────────────────────────────────────────

fn bench_push_pop_normal(c: &mut Criterion) {
    let config = FrontierConfig::medium();

    c.bench_function("frontier/push_pop_normal_100", |b| {
        b.iter(|| {
            let mut f = CausalFrontier::new(config);
            f.begin_cycle();
            for i in 0..100u32 {
                f.push_token(black_box(i));
            }
            while f.pop().is_some() {}
            f.end_cycle();
        });
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// State machine: begin_cycle / end_cycle overhead
// ─────────────────────────────────────────────────────────────────────────────

fn bench_cycle_overhead(c: &mut Criterion) {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());

    c.bench_function("frontier/begin_end_cycle_overhead", |b| {
        b.iter(|| {
            frontier.begin_cycle();
            black_box(frontier.state());
            frontier.end_cycle();
        });
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// Storm Control: 1000+ токенов с budget enforcement
// ─────────────────────────────────────────────────────────────────────────────

fn bench_storm_control(c: &mut Criterion) {
    let mut group = c.benchmark_group("frontier/storm_control");
    group.measurement_time(Duration::from_secs(5));

    for n_tokens in [500u32, 1000, 5000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(n_tokens),
            &n_tokens,
            |b, &n| {
                let config = FrontierConfig {
                    max_frontier_size: n + 100,
                    max_events_per_cycle: 200, // budget — не даём обработать всё за цикл
                    storm_threshold: n / 2,
                    enable_batch_events: true,
                    batch_size: 20,
                    token_capacity: n + 100,
                    connection_capacity: 100,
                };

                b.iter(|| {
                    let mut f = CausalFrontier::new(config);
                    // Загружаем frontier выше storm_threshold
                    for i in 0..n {
                        f.push_token(black_box(i));
                    }
                    // Один цикл с budget
                    f.begin_cycle();
                    while f.pop().is_some() {}
                    f.end_cycle();
                    black_box(f.state());
                });
            },
        );
    }
    group.finish();
}

// ─────────────────────────────────────────────────────────────────────────────
// Batch Events: сравнение обычного pop vs batch pop при Storm
// ─────────────────────────────────────────────────────────────────────────────

fn bench_batch_vs_normal(c: &mut Criterion) {
    let mut group = c.benchmark_group("frontier/batch_vs_normal");
    group.measurement_time(Duration::from_secs(5));

    // Normal: каждый токен отдельно
    group.bench_function("normal_pop_1000", |b| {
        b.iter(|| {
            let config = FrontierConfig {
                max_frontier_size: 2000,
                max_events_per_cycle: 2000,
                storm_threshold: 5000, // никогда не Storm → нет batching
                enable_batch_events: false,
                batch_size: 0,
                token_capacity: 2000,
                connection_capacity: 100,
            };
            let mut f = CausalFrontier::new(config);
            for i in 0..1000u32 {
                f.push_token(i);
            }
            f.begin_cycle();
            while f.pop().is_some() {}
            f.end_cycle();
        });
    });

    // Batch: Storm активен, batch_size=20
    group.bench_function("batch_pop_1000_storm", |b| {
        b.iter(|| {
            let config = FrontierConfig {
                max_frontier_size: 2000,
                max_events_per_cycle: 2000,
                storm_threshold: 10, // сразу Storm
                enable_batch_events: true,
                batch_size: 20,
                token_capacity: 2000,
                connection_capacity: 100,
            };
            let mut f = CausalFrontier::new(config);
            for i in 0..1000u32 {
                f.push_token(i);
            }
            // Сначала end_cycle для обновления state до Storm
            f.begin_cycle();
            f.end_cycle();
            // Теперь pop() использует batching
            f.begin_cycle();
            while f.pop().is_some() {}
            f.end_cycle();
        });
    });

    group.finish();
}

criterion_group!(
    frontier_benches,
    bench_push_pop_normal,
    bench_cycle_overhead,
    bench_storm_control,
    bench_batch_vs_normal,
);
criterion_main!(frontier_benches);
