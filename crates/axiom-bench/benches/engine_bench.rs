// Бенчмарки axiom-runtime: AxiomEngine полный цикл + AshtiCore pipeline
//
// AshtiCore (v2.0): 11 фиксированных доменов, domain_id = level_id*100 + role.
// Level 1 → SUTRA=100, EXECUTION=101 .. VOID=108, EXPERIENCE=109, MAYA=110.
// LOGIC = 106 (role=6) — используется как целевой домен для InjectToken.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use axiom_runtime::AxiomEngine;
use axiom_core::Token;
use axiom_ucl::{UclCommand, OpCode};
use std::time::Duration;

const LOGIC_ID: u32 = 106; // level_id(1)*100 + role(6)

// Создаёт engine с N токенами в LOGIC-домене (106)
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

// Создаёт engine с N следами в Experience — для разогрева памяти AshtiCore
fn engine_with_traces(trace_count: usize) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for i in 0..trace_count {
        let mut t = Token::new(i as u32 + 1, 100, [0, 0, 0], 1);
        t.temperature = (i % 256) as u8;
        t.mass = ((i * 3) % 256) as u8;
        engine.ashti.experience_mut().add_trace(t, 0.7, i as u64 + 1);
    }
    engine
}

fn bench_engine_creation(c: &mut Criterion) {
    c.bench_function("AxiomEngine::new", |b| {
        b.iter(|| black_box(AxiomEngine::new()))
    });
}

fn bench_inject_token(c: &mut Criterion) {
    let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
    cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
    cmd.payload[1] = (LOGIC_ID >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&100.0f32.to_le_bytes());

    // Пересоздаём engine в каждой итерации — иначе токены накапливаются
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
            |b, _| {
                b.iter(|| black_box(engine.process_command(black_box(&cmd))))
            },
        );
    }
    group.finish();
}

fn bench_snapshot_capture(c: &mut Criterion) {
    let mut group = c.benchmark_group("EngineSnapshot::capture");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for tokens in [0, 10, 100] {
        let engine = engine_with_tokens(tokens);
        group.bench_with_input(
            BenchmarkId::new("tokens_in_logic", tokens),
            &tokens,
            |b, _| {
                b.iter(|| black_box(engine.snapshot()))
            },
        );
    }
    group.finish();
}

fn bench_snapshot_restore(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine::restore_from");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for tokens in [0, 10, 100] {
        let engine = engine_with_tokens(tokens);
        let snap = engine.snapshot();
        group.bench_with_input(
            BenchmarkId::new("tokens_in_logic", tokens),
            &tokens,
            |b, _| {
                b.iter(|| black_box(AxiomEngine::restore_from(black_box(&snap))))
            },
        );
    }
    group.finish();
}

// ============================================================
// AshtiCore: полный dual-path pipeline
//
// Измеряет latency одного "мыслительного акта":
// token → SUTRA(100) → EXPERIENCE(109) → classify →
//   fast path: reflex (если есть) || slow path: ASHTI(101-108) → MAYA(110)
// ============================================================

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
                    || engine_with_traces(tc),
                    |mut e| black_box(e.ashti.process(black_box(token))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_engine_creation,
    bench_inject_token,
    bench_tick_forward,
    bench_snapshot_capture,
    bench_snapshot_restore,
    bench_ashti_pipeline,
);
criterion_main!(benches);
