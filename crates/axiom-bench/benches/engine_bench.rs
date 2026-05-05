// Бенчмарки axiom-runtime: AxiomEngine полный цикл, Gateway, Horizon, Adaptation
//
// Охват по этапам:
//   Этап 1-3: Engine::new, InjectToken, TickForward, Snapshot
//   Этап 4-5: AshtiCore dual-path pipeline
//   Этап 6:   run_adaptation (adapt_thresholds + adapt_domain_physics)
//   Этап 7:   snapshot_and_prune, run_horizon_gc, export_skills
//   Этап 8:   Gateway::process, Gateway::process_channel
//
// Топология: level_id(1)*100 + role → SUTRA=100 .. MAYA=110, LOGIC=106

use axiom_core::Token;
use axiom_runtime::{AxiomEngine, Channel, Gateway};
use axiom_ucl::{OpCode, UclCommand};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

const LOGIC_ID: u32 = 106;

// ─── builders ────────────────────────────────────────────────────────────────

fn engine_with_tokens(tokens: usize) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for j in 0..tokens {
        let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
        cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
        cmd.payload[1] = (LOGIC_ID >> 8) as u8;
        let mass = 50.0f32 + j as f32;
        cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
        let temp = (j % 200) as f32 + 20.0;
        cmd.payload[36..40].copy_from_slice(&temp.to_le_bytes());
        engine.process_command(&cmd);
    }
    engine
}

fn engine_with_experience(trace_count: usize) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for i in 0..trace_count {
        let mut t = Token::new(i as u32 + 1, 100, [0, 0, 0], 1);
        t.temperature = (i % 256) as u8;
        t.mass = ((i * 3) % 256) as u8;
        engine
            .ashti
            .experience_mut()
            .add_trace(t, 0.9, i as u64 + 1);
    }
    engine
}

// ─── Этап 1-3: базовые операции Engine ───────────────────────────────────────

fn bench_engine_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine::new breakdown");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(50);

    // Полная стоимость
    group.bench_function("full", |b| b.iter(|| black_box(AxiomEngine::new())));

    // AshtiCore без rayon — изолируем стоимость thread pool
    group.bench_function("AshtiCore::new only", |b| {
        b.iter(|| black_box(axiom_domain::AshtiCore::new(1)))
    });

    group.finish();
}

fn bench_inject_token(c: &mut Criterion) {
    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
    cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
    cmd.payload[1] = (LOGIC_ID >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&100.0f32.to_le_bytes());

    c.bench_function("AxiomEngine: InjectToken", |b| {
        b.iter_batched(
            AxiomEngine::new,
            |mut e| black_box(e.process_command(black_box(&cmd))),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_tick_forward(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: TickForward");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    let cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    for tokens in [0, 10, 50, 100] {
        let mut engine = engine_with_tokens(tokens);
        group.bench_with_input(
            BenchmarkId::new("tokens_in_logic", tokens),
            &tokens,
            |b, _| b.iter(|| black_box(engine.process_command(black_box(&cmd)))),
        );
    }
    group.finish();
}

fn bench_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: snapshot");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for tokens in [0, 10, 100] {
        let engine = engine_with_tokens(tokens);
        group.bench_with_input(BenchmarkId::new("tokens", tokens), &tokens, |b, _| {
            b.iter(|| black_box(engine.snapshot()))
        });
    }
    group.finish();
}

fn bench_restore(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: restore_from");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for tokens in [0, 10, 100] {
        let snap = engine_with_tokens(tokens).snapshot();
        group.bench_with_input(BenchmarkId::new("tokens", tokens), &tokens, |b, _| {
            b.iter(|| black_box(AxiomEngine::restore_from(black_box(&snap))))
        });
    }
    group.finish();
}

// ─── Этап 4-5: AshtiCore pipeline (dual-path + GridHash) ─────────────────────

fn bench_ashti_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("AshtiCore: full pipeline");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let token = Token::new(1, 100, [10, 20, 30], 1);

    for trace_count in [0, 10, 100] {
        group.bench_with_input(
            BenchmarkId::new("traces", trace_count),
            &trace_count,
            |b, &tc| {
                b.iter_batched(
                    || engine_with_experience(tc),
                    |mut e| black_box(e.ashti.process(black_box(token))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ─── Этап 6: Адаптивные пороги ───────────────────────────────────────────────

fn bench_run_adaptation(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: run_adaptation");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for trace_count in [0, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("traces", trace_count),
            &trace_count,
            |b, &tc| {
                b.iter_batched(
                    || engine_with_experience(tc),
                    |mut e| black_box(e.run_adaptation()),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ─── Этап 7: Causal Horizon + pruning + skills ────────────────────────────────

fn bench_snapshot_and_prune(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: snapshot_and_prune");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for trace_count in [0, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("traces", trace_count),
            &trace_count,
            |b, &tc| {
                b.iter_batched(
                    || engine_with_experience(tc),
                    |mut e| black_box(e.snapshot_and_prune()),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_run_horizon_gc(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: run_horizon_gc");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for trace_count in [0, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("traces", trace_count),
            &trace_count,
            |b, &tc| {
                b.iter_batched(
                    || engine_with_experience(tc),
                    |mut e| black_box(e.run_horizon_gc()),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_run_horizon_gc_isolated(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: run_horizon_gc (isolated)");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(50);

    // Engine создаётся один раз снаружи — исключает iter_batched overhead
    for trace_count in [0, 50, 200] {
        let mut engine = engine_with_experience(trace_count);
        group.bench_with_input(
            BenchmarkId::new("traces", trace_count),
            &trace_count,
            |b, _| b.iter(|| black_box(engine.run_horizon_gc())),
        );
    }
    group.finish();
}

fn bench_causal_horizon(c: &mut Criterion) {
    c.bench_function("AxiomEngine: causal_horizon", |b| {
        let engine = AxiomEngine::new();
        b.iter(|| black_box(engine.causal_horizon()))
    });
}

fn bench_export_skills(c: &mut Criterion) {
    c.bench_function("AxiomEngine: export_skills", |b| {
        let engine = AxiomEngine::new();
        b.iter(|| black_box(engine.export_skills()))
    });
}

// ─── Этап 8: Gateway + Channel ───────────────────────────────────────────────

fn bench_gateway_process(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gateway: process");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    group.bench_function("TickForward_no_observers", |b| {
        b.iter_batched(
            Gateway::with_default_engine,
            |mut gw| black_box(gw.process(black_box(&tick))),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gateway_process_channel(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gateway: process_channel");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    for batch_size in [1, 10, 50] {
        group.bench_with_input(
            BenchmarkId::new("cmds", batch_size),
            &batch_size,
            |b, &n| {
                b.iter_batched(
                    || {
                        let mut ch = Channel::new();
                        for _ in 0..n {
                            ch.send(tick);
                        }
                        (Gateway::with_default_engine(), ch)
                    },
                    |(mut gw, mut ch)| black_box(gw.process_channel(black_box(&mut ch))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ─── domain_detail_snapshot (EA-TD-02: точный compute_shell) ─────────────────

fn engine_with_tokens_and_connections(tokens: usize, conns_per_token: usize) -> AxiomEngine {
    use axiom_core::Connection;
    let mut engine = engine_with_tokens(tokens);
    let idx = engine.ashti.index_of(106).unwrap();
    let state = engine.ashti.state_mut(idx).unwrap();
    for i in 0..tokens {
        for j in 0..conns_per_token {
            let src = (i + 1) as u32;
            let tgt = ((i + j + 1) % tokens + 1) as u32;
            let conn = Connection {
                source_id: src,
                target_id: tgt,
                strength: 0.5 + (j as f32) * 0.1,
                ..Connection::default()
            };
            let _ = state.add_connection(conn);
        }
    }
    engine
}

fn bench_domain_detail_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine: domain_detail_snapshot");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(50);

    // (tokens, connections_per_token) → общее число связей
    for (tokens, conns) in [(10, 0), (10, 5), (50, 5), (50, 20), (200, 10)] {
        let engine = engine_with_tokens_and_connections(tokens, conns);
        let label = format!("t{}_c{}", tokens, tokens * conns);
        group.bench_function(&label, |b| {
            b.iter(|| black_box(engine.domain_detail_snapshot(black_box(106))))
        });
    }
    group.finish();
}

// ─── groups ──────────────────────────────────────────────────────────────────

criterion_group!(
    benches_core,
    bench_engine_creation,
    bench_inject_token,
    bench_tick_forward,
    bench_snapshot,
    bench_restore,
);
criterion_group!(benches_ashti, bench_ashti_pipeline,);
criterion_group!(benches_etap6, bench_run_adaptation,);
criterion_group!(
    benches_etap7,
    bench_snapshot_and_prune,
    bench_run_horizon_gc,
    bench_run_horizon_gc_isolated,
    bench_causal_horizon,
    bench_export_skills,
);
criterion_group!(
    benches_etap8,
    bench_gateway_process,
    bench_gateway_process_channel,
);
criterion_group!(benches_adapters, bench_domain_detail_snapshot,);

criterion_main!(
    benches_core,
    benches_ashti,
    benches_etap6,
    benches_etap7,
    benches_etap8,
    benches_adapters,
);
