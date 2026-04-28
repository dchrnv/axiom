// Hot Path Regression — постоянный бенчмарк для отслеживания просадок.
//
// Цель: держать TickForward (50 токенов, default FW config) ≤ 150 ns.
// Запускается при каждом релизе: cargo bench --bench hot_path_regression
//
// Если медиана выйдет за 150 ns — ищи регрессию в FrameWeaver интеграции
// или в основном pipeline AshtiCore (hot path).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

const LOGIC_ID: u32 = 106;

fn engine_with_50_tokens() -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    for j in 1u32..=50 {
        let mut cmd = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
        cmd.payload[0] = (LOGIC_ID & 0xff) as u8;
        cmd.payload[1] = (LOGIC_ID >> 8)   as u8;
        let mass = 50.0f32 + j as f32;
        cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
        engine.process_command(&cmd);
    }
    engine
}

fn bench_hot_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("HotPath regression");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);

    // Эталонный замер: TickForward с 50 токенами в LOGIC, default FrameWeaver config
    // История: ~96.5 ns до FrameWeaver V1.1; ~310 ns после интеграции (до оптимизации).
    // Целевая планка после оптимизации: ≤ 150 ns.
    let cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let mut engine = engine_with_50_tokens();
    group.bench_function("TickForward / tokens_in_logic / 50", |b| {
        b.iter(|| black_box(engine.process_command(black_box(&cmd))))
    });

    group.finish();
}

criterion_group!(benches, bench_hot_path);
criterion_main!(benches);
