// Integration benchmark: все модули вместе
//
// Три режима:
//   normal/   — точные замеры при 100k и 1M операций (iter_custom + Throughput)
//   periodic/ — TickSchedule: hot/warm/cold пути, reconcile_all
//   stress/   — 10 минут sustained throughput
//
// Топология: level_id(1)*100 → SUTRA=100..MAYA=110, LOGIC=106

use criterion::{
    black_box, criterion_group, criterion_main,
    Criterion, BenchmarkId, Throughput,
};
use axiom_runtime::{AxiomEngine, TickSchedule};
use axiom_ucl::{UclCommand, OpCode};
use axiom_core::Token;
use axiom_config::DomainConfig;
use axiom_arbiter::{Arbiter, COM};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const LOGIC_ID: u16 = 106;
const SUTRA_ID: u16 = 100;

// ─── builders ────────────────────────────────────────────────────────────────

fn inject_cmd(domain_id: u16, mass: f32, temp: f32) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, domain_id as u32, 100, 0);
    cmd.payload[0] = (domain_id & 0xff) as u8;
    cmd.payload[1] = (domain_id >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&temp.to_le_bytes());
    cmd
}

/// Engine с токенами в LOGIC и Experience-следами
fn engine_loaded(tokens: usize, traces: usize) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for i in 0..tokens {
        let mass = 50.0 + (i % 200) as f32;
        let temp = 20.0 + (i % 230) as f32;
        engine.process_command(&inject_cmd(LOGIC_ID, mass, temp));
    }
    for i in 0..traces {
        let mut t = Token::new(i as u32 + 1, SUTRA_ID as u16, [0, 0, 0], 1);
        t.temperature = (i % 256) as u8;
        t.mass = ((i * 3 + 50) % 256) as u8;
        engine.ashti.experience_mut().add_trace(t, 0.6 + (i % 40) as f32 / 100.0, i as u64 + 1);
    }
    engine
}

/// Engine с кастомным TickSchedule
fn engine_with_schedule(tokens: usize, schedule: TickSchedule) -> AxiomEngine {
    let mut engine = engine_loaded(tokens, 50);
    engine.tick_schedule = schedule;
    engine
}

/// TickSchedule: все периодические задачи отключены (только hot path)
fn schedule_hot_only() -> TickSchedule {
    TickSchedule {
        adaptation_interval:    0,
        horizon_gc_interval:    0,
        snapshot_interval:      0,
        dream_interval:         0,
        tension_check_interval: 0,
        goal_check_interval:    0,
        reconcile_interval:     0,
        persist_check_interval: 0,
        adaptive_tick:          axiom_runtime::AdaptiveTickRate::default(),
    }
}

/// TickSchedule: все задачи каждый тик (максимальная нагрузка)
fn schedule_max_load() -> TickSchedule {
    TickSchedule {
        adaptation_interval:    1,
        horizon_gc_interval:    1,
        snapshot_interval:      0, // snapshot дорог, оставим выключенным
        dream_interval:         1,
        tension_check_interval: 1,
        goal_check_interval:    1,
        reconcile_interval:     1,
        persist_check_interval: 0,
        adaptive_tick:          axiom_runtime::AdaptiveTickRate::default(),
    }
}

// ─── normal/: точные замеры 100k и 1M тиков ──────────────────────────────────

fn bench_normal_100k(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal/100k_ticks");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);
    group.throughput(Throughput::Elements(100_000));

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Пустой engine — только hot path
    group.bench_function("engine_empty", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = AxiomEngine::new();
                let start = Instant::now();
                for _ in 0..100_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    // 50 токенов в LOGIC
    group.bench_function("engine_50_tokens", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_loaded(50, 0);
                let start = Instant::now();
                for _ in 0..100_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    // 50 токенов + 100 следов + дефолтный TickSchedule (warm/cold пути)
    group.bench_function("engine_50tok_100tr_default_schedule", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_loaded(50, 100);
                let start = Instant::now();
                for _ in 0..100_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    // 50 токенов + максимальная нагрузка TickSchedule
    group.bench_function("engine_50tok_max_schedule", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_with_schedule(50, schedule_max_load());
                let start = Instant::now();
                for _ in 0..100_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    group.finish();
}

fn bench_normal_1m(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal/1M_ticks");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    group.throughput(Throughput::Elements(1_000_000));

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    group.bench_function("engine_empty", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = AxiomEngine::new();
                let start = Instant::now();
                for _ in 0..1_000_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    group.bench_function("engine_50tok_hot_only", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_with_schedule(50, schedule_hot_only());
                let start = Instant::now();
                for _ in 0..1_000_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    group.bench_function("engine_50tok_default_schedule", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_loaded(50, 100);
                let start = Instant::now();
                for _ in 0..1_000_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    group.finish();
}

// ─── periodic/: TickSchedule и reconcile_all ─────────────────────────────────

fn bench_tick_schedule_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("periodic/tick_schedule_overhead");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Сравниваем hot-only vs default vs max по одному тику
    group.bench_function("hot_only", |b| {
        b.iter_batched(
            || engine_with_schedule(50, schedule_hot_only()),
            |mut e| black_box(e.process_command(&tick)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("default_schedule", |b| {
        b.iter_batched(
            || engine_loaded(50, 50),
            |mut e| black_box(e.process_command(&tick)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("max_schedule", |b| {
        b.iter_batched(
            || engine_with_schedule(50, schedule_max_load()),
            |mut e| black_box(e.process_command(&tick)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_reconcile_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("periodic/reconcile_all");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for (tokens, conns) in [(0, 0), (50, 0), (50, 100), (200, 500)] {
        let label = format!("t{tokens}_c{conns}");
        group.bench_with_input(
            BenchmarkId::new("tokens_connections", &label),
            &(tokens, conns),
            |b, &(t, c_count)| {
                b.iter_batched(
                    || {
                        let mut engine = engine_loaded(t, 0);
                        // Добавим связи вручную через state
                        let idx = engine.ashti.index_of(LOGIC_ID).unwrap();
                        for i in 0..c_count.min(t.saturating_sub(1)) {
                            let conn = axiom_core::Connection::new(
                                i as u32 + 1,
                                i as u32 + 2,
                                LOGIC_ID as u16,
                                i as u64 + 1,
                            );
                            let _ = engine.ashti.state_mut(idx).unwrap().add_connection(conn);
                        }
                        engine
                    },
                    |mut e| black_box(e.ashti.reconcile_all()),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_snapshot_restore_with_tick_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("periodic/snapshot_restore_tick_count");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // snapshot после N тиков (tick_count сохраняется)
    for n_ticks in [0u64, 1000, 50_000] {
        group.bench_with_input(
            BenchmarkId::new("after_ticks", n_ticks),
            &n_ticks,
            |b, &n| {
                b.iter_batched(
                    || {
                        let mut engine = engine_loaded(20, 20);
                        for _ in 0..n {
                            engine.process_command(&tick);
                        }
                        engine
                    },
                    |e| black_box(e.snapshot()),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    // restore (tick_count восстанавливается, нет сброса)
    group.bench_function("restore_preserves_tick_count", |b| {
        b.iter_batched(
            || {
                let mut engine = engine_loaded(20, 20);
                let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
                for _ in 0..100 { engine.process_command(&tick); }
                engine.snapshot()
            },
            |snap| {
                let engine = AxiomEngine::restore_from(&snap);
                black_box(engine.tick_count)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

// ─── compare_tokens: дефолтный конфиг vs per-domain ─────────────────────────

fn bench_compare_tokens(c: &mut Criterion) {
    let mut group = c.benchmark_group("periodic/compare_tokens");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    // Fallback — нет конфига домена
    let arbiter_no_cfg = {
        let domains = HashMap::new();
        Arbiter::new(domains, COM::new())
    };

    // Per-domain конфиг с tolerances
    let arbiter_with_cfg = {
        let mut cfg = DomainConfig::default();
        cfg.domain_id = LOGIC_ID as u16;
        cfg.token_compare_temp_tolerance    = 15;
        cfg.token_compare_mass_tolerance    = 8;
        cfg.token_compare_valence_tolerance = 3;
        let mut domains = HashMap::new();
        domains.insert(LOGIC_ID as u16, cfg);
        Arbiter::new(domains, COM::new())
    };

    let mut t1 = Token::new(1, LOGIC_ID as u16, [0, 0, 0], 1);
    t1.temperature = 100; t1.mass = 100;
    let mut t2 = Token::new(2, LOGIC_ID as u16, [5, 5, 0], 2);
    t2.temperature = 108; t2.mass = 104;

    group.bench_function("fallback_constants", |b| {
        b.iter(|| black_box(arbiter_no_cfg.compare_tokens(&t1, &t2)))
    });

    group.bench_function("per_domain_config", |b| {
        b.iter(|| black_box(arbiter_with_cfg.compare_tokens(&t1, &t2)))
    });

    group.finish();
}

// ─── normal/: полный интегрированный цикл (inject → tick → reconcile) ────────

fn bench_integrated_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal/integrated_cycle");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);
    group.throughput(Throughput::Elements(1));

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Один полный цикл: inject + tick + reconcile
    group.bench_function("inject_tick_reconcile", |b| {
        b.iter_batched(
            || engine_loaded(50, 50),
            |mut e| {
                let _ = e.process_command(&inject_cmd(LOGIC_ID, 75.0, 120.0));
                let _ = e.process_command(&tick);
                let _ = e.ashti.reconcile_all();
                black_box(e.tick_count)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Цикл с snapshot каждые 1000 тиков
    group.bench_function("1000ticks_then_snapshot", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_loaded(50, 50);
                let start = Instant::now();
                for i in 0..1000u64 {
                    engine.process_command(&tick);
                    if i % 100 == 0 {
                        engine.ashti.reconcile_all();
                    }
                }
                let snap = engine.snapshot();
                black_box(snap.tick_count);
                total += start.elapsed();
            }
            total
        })
    });

    group.finish();
}

// ─── stress/: 10 минут sustained throughput ──────────────────────────────────

fn bench_stress_sustained(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress/sustained_10min");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(5));
    group.throughput(Throughput::Elements(1_000));

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Реалистичный сценарий: 50 токенов, дефолтный schedule
    // reconcile каждые 200, snapshot каждые 5000
    group.bench_function("realistic_engine_50tok", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_loaded(50, 100);
                let start = Instant::now();
                for _ in 0..1_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    // Нагруженный сценарий: 200 токенов, max schedule
    group.bench_function("heavy_engine_200tok_max_schedule", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_with_schedule(200, schedule_max_load());
                let start = Instant::now();
                for _ in 0..1_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    // Только hot path: baseline для сравнения деградации
    group.bench_function("baseline_hot_only_50tok", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                let mut engine = engine_with_schedule(50, schedule_hot_only());
                let start = Instant::now();
                for _ in 0..1_000 {
                    black_box(engine.process_command(&tick));
                }
                total += start.elapsed();
            }
            total
        })
    });

    group.finish();
}

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    benches_normal,
    bench_normal_100k,
    bench_normal_1m,
    bench_integrated_cycle,
);
criterion_group!(
    benches_periodic,
    bench_tick_schedule_overhead,
    bench_reconcile_all,
    bench_snapshot_restore_with_tick_count,
    bench_compare_tokens,
);
criterion_group!(
    benches_stress,
    bench_stress_sustained,
);

criterion_main!(benches_normal, benches_periodic, benches_stress);
