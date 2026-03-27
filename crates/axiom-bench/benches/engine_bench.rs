// Бенчмарки axiom-runtime: AxiomEngine полный цикл
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use axiom_runtime::AxiomEngine;
use axiom_config::DomainConfig;
use axiom_ucl::{UclCommand, OpCode};
use std::time::Duration;

fn engine_with_tokens(domain_count: usize, tokens_per_domain: usize) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for i in 0..domain_count {
        let id = (i + 1) as u16;
        engine.add_domain(DomainConfig::factory_logic(id, 0)).unwrap();

        // Inject tokens
        for j in 0..tokens_per_domain {
            let mut cmd = UclCommand::new(OpCode::InjectToken, id as u32, 100, 0);
            cmd.payload[0] = id as u8;
            cmd.payload[1] = (id >> 8) as u8;
            let mass = 50.0f32 + j as f32;
            cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
            let temp = (j % 200) as f32 + 20.0;
            cmd.payload[36..40].copy_from_slice(&temp.to_le_bytes());
            engine.process_command(&cmd);
        }
    }
    engine
}

fn bench_engine_creation(c: &mut Criterion) {
    c.bench_function("AxiomEngine::new", |b| {
        b.iter(|| black_box(AxiomEngine::new()))
    });
}

fn bench_add_domain(c: &mut Criterion) {
    let mut group = c.benchmark_group("AxiomEngine::add_domain");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    group.bench_function("10_domains", |b| {
        b.iter(|| {
            let mut engine = AxiomEngine::new();
            for i in 1u16..=10 {
                engine.add_domain(black_box(DomainConfig::factory_logic(i, 0))).unwrap();
            }
            black_box(engine)
        })
    });
    group.finish();
}

fn bench_inject_token(c: &mut Criterion) {
    let mut cmd = UclCommand::new(OpCode::InjectToken, 1, 100, 0);
    cmd.payload[0] = 1;
    cmd.payload[1] = 0;
    cmd.payload[4..8].copy_from_slice(&100.0f32.to_le_bytes());

    // Пересоздаём engine в каждой итерации — иначе токены накапливаются
    // и каждая следующая итерация тяжелее предыдущей
    c.bench_function("AxiomEngine: InjectToken", |b| {
        b.iter_batched(
            || {
                let mut e = AxiomEngine::new();
                e.add_domain(DomainConfig::factory_logic(1, 0)).unwrap();
                e
            },
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
        let mut engine = engine_with_tokens(3, tokens);
        group.bench_with_input(
            BenchmarkId::new("tokens_per_domain", tokens),
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
        let engine = engine_with_tokens(3, tokens);
        group.bench_with_input(
            BenchmarkId::new("tokens_per_domain", tokens),
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
        let engine = engine_with_tokens(3, tokens);
        let snap = engine.snapshot();
        group.bench_with_input(
            BenchmarkId::new("tokens_per_domain", tokens),
            &tokens,
            |b, _| {
                b.iter(|| black_box(AxiomEngine::restore_from(black_box(&snap))))
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_engine_creation,
    bench_add_domain,
    bench_inject_token,
    bench_tick_forward,
    bench_snapshot_capture,
    bench_snapshot_restore,
);
criterion_main!(benches);
