// Over-Domain Layer benchmarks — V7 (ContextRecognizer, NeuralAdvisor, FatigueStore, TransitionMatrix)
//
// Цель: измерить реальную стоимость тика в трёх режимах:
//   cold  — свежий движок, нет накопленного состояния
//   warm  — после 100 тиков (CR и NA отработали несколько раз)
//   loaded — после 1000 тиков с инъекциями (TransitionMatrix заполнен, FatigueStore активен)
//
// Компоненты Over-Domain Layer, запускающиеся на каждом тике:
//   FrameWeaver V1.3    — каждый тик
//   AxialEvaluator V3.0 — каждые 5 тиков
//   ContextRecognizer V7— каждые 7 тиков (TransitionMatrix, FatigueStore, CompositeSubsystem, SplitMerge)
//   NeuralAdvisor V3.0  — каждые 11 тиков (EmergentPatternDetector, EmergentSubsystemRules)
//   OverDomainArbiter   — обрабатывает pending advisories

use axiom_runtime::AxiomEngine;
use axiom_ucl::{OpCode, UclCommand};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

const SUTRA_ID: u32 = 100;
const LOGIC_ID: u32 = 106;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn inject_n_tokens(engine: &mut AxiomEngine, domain_id: u32, n: usize) {
    for j in 0..n {
        let mut cmd = UclCommand::new(OpCode::InjectToken, domain_id, 100, 0);
        cmd.payload[0] = (domain_id & 0xff) as u8;
        cmd.payload[1] = (domain_id >> 8) as u8;
        let mass = 40.0f32 + (j % 200) as f32;
        cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
        let pos = [(j % 1000) as f32 * 60.0 - 30000.0, 0.0f32, 0.0f32];
        cmd.payload[8..12].copy_from_slice(&pos[0].to_le_bytes());
        let temp = 20.0f32 + (j % 100) as f32;
        cmd.payload[36..40].copy_from_slice(&temp.to_le_bytes());
        engine.process_command(&cmd);
    }
}

fn run_n_ticks(engine: &mut AxiomEngine, n: u64) {
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..n {
        engine.process_command(&tick);
    }
}

/// Движок с N токенами, разогретый M тиками.
fn engine_warmed(tokens: usize, warmup_ticks: u64) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    // Инжектируем токены в несколько доменов для более реалистичного состояния
    inject_n_tokens(&mut engine, SUTRA_ID, tokens / 3);
    inject_n_tokens(&mut engine, LOGIC_ID, tokens / 3);
    inject_n_tokens(&mut engine, 105, tokens - 2 * (tokens / 3)); // MAYA-1 (105)
    run_n_ticks(&mut engine, warmup_ticks);
    engine
}

// ─── 1. Стоимость одного тика в разных состояниях ────────────────────────────

fn bench_tick_cold(c: &mut Criterion) {
    let mut group = c.benchmark_group("over_domain/tick_cold");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(500);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    for tokens in [0usize, 10, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("tokens", tokens),
            &tokens,
            |b, &n| {
                b.iter_batched(
                    || {
                        let mut e = AxiomEngine::new();
                        inject_n_tokens(&mut e, LOGIC_ID, n);
                        e
                    },
                    |mut e| black_box(e.process_command(black_box(&tick))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_tick_warm_100(c: &mut Criterion) {
    let mut group = c.benchmark_group("over_domain/tick_warm_100");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(200);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    for tokens in [0usize, 10, 50, 200] {
        group.bench_with_input(
            BenchmarkId::new("tokens", tokens),
            &tokens,
            |b, &n| {
                b.iter_batched(
                    || engine_warmed(n, 100),
                    |mut e| black_box(e.process_command(black_box(&tick))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_tick_loaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("over_domain/tick_loaded_1000");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // После 1000 тиков: TransitionMatrix ≈ заполнен, FatigueStore активен,
    // CR отработал ~142 раза, NeuralAdvisor ~90 раз
    for tokens in [50usize, 200, 500] {
        group.bench_with_input(
            BenchmarkId::new("tokens", tokens),
            &tokens,
            |b, &n| {
                b.iter_batched(
                    || engine_warmed(n, 1000),
                    |mut e| black_box(e.process_command(black_box(&tick))),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ─── 2. Пропускная способность: тиков/сек ────────────────────────────────────

fn bench_throughput_ticks(c: &mut Criterion) {
    let mut group = c.benchmark_group("over_domain/throughput");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(20);

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Замеряем 1000 последовательных тиков — включает все периодические компоненты
    for tokens in [50usize, 200] {
        group.bench_with_input(
            BenchmarkId::new("1000_ticks_tokens", tokens),
            &tokens,
            |b, &n| {
                b.iter_batched(
                    || engine_warmed(n, 50),
                    |mut e| {
                        for _ in 0..1000 {
                            black_box(e.process_command(black_box(&tick)));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ─── 3. Стоимость инъекции токена в загруженный движок ───────────────────────

fn bench_inject_loaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("over_domain/inject_loaded");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(200);

    let mut cmd = UclCommand::new(OpCode::InjectToken, SUTRA_ID, 100, 0);
    cmd.payload[0] = (SUTRA_ID & 0xff) as u8;
    cmd.payload[1] = (SUTRA_ID >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&100.0f32.to_le_bytes());

    // Сравниваем инъекцию в холодный vs разогретый движок
    group.bench_function("cold", |b| {
        b.iter_batched(
            AxiomEngine::new,
            |mut e| black_box(e.process_command(black_box(&cmd))),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("after_1000_ticks_200_tokens", |b| {
        b.iter_batched(
            || engine_warmed(200, 1000),
            |mut e| black_box(e.process_command(black_box(&cmd))),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

// ─── criterion_main ───────────────────────────────────────────────────────────

criterion_group!(
    over_domain,
    bench_tick_cold,
    bench_tick_warm_100,
    bench_tick_loaded,
    bench_throughput_ticks,
    bench_inject_loaded,
);

criterion_main!(over_domain);
