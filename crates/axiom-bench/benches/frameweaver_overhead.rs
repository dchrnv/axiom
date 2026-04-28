// FrameWeaver Overhead — A/B/C/D бенчмарк
//
// Цель: измерить стоимость FrameWeaver в pipeline TickForward и локализовать регрессию.
// Результаты записать в docs/spec/Weaver/erratas/FrameWeaver_V1_1_errata.md (секция E3).
//
// Группы:
//   A — FrameWeaver disabled (scan_interval=u32::MAX): только drain_commands + interval check
//   B — FrameWeaver enabled, scan_interval=1, MAYA пуста
//   C — FrameWeaver enabled, scan_interval=1, MAYA с 5 синтаксическими узорами
//   D — FrameWeaver enabled, scan_interval=1, MAYA с 20 синтаксическими узорами
//
// Что показывают переходы:
//   A → B: стоимость холостого scan (ранний выход из пустой MAYA)
//   B → C: стоимость реального сканирования (5 узоров)
//   C → D: масштабирование по числу узоров

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;
use axiom_runtime::{AxiomEngine, FrameWeaver, FrameWeaverConfig};
use axiom_core::{Connection, FLAG_ACTIVE};
use axiom_ucl::{UclCommand, OpCode};

const MAYA_ID:  u16 = 110;
const LOGIC_ID: u32 = 106;

fn tick_cmd() -> UclCommand {
    UclCommand::new(OpCode::TickForward, 0, 100, 0)
}

/// Базовый engine с 50 токенами в LOGIC.
fn base_engine(fw_config: FrameWeaverConfig) -> AxiomEngine {
    let mut engine = AxiomEngine::new();
    engine.frame_weaver = FrameWeaver::new(fw_config);

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

/// Создать синтаксическую связь в MAYA (link_type категории 0x08).
fn syn_conn(source: u32, target: u32, layer: u8) -> Connection {
    let mut c = Connection::new(source, target, MAYA_ID, 1);
    c.link_type = 0x0800 | ((layer as u16) << 4);
    c.flags = FLAG_ACTIVE;
    c
}

/// Добавить N синтаксических узоров (каждый = 1 head + 2 participants, 2 слоя).
fn add_maya_patterns(engine: &mut AxiomEngine, patterns: usize) {
    for i in 0..patterns {
        let head   = (i * 3 + 1) as u32;
        let tgt1   = (i * 3 + 2) as u32;
        let tgt2   = (i * 3 + 3) as u32;
        engine.ashti.inject_connection(MAYA_ID, syn_conn(head, tgt1, 1)).ok();
        engine.ashti.inject_connection(MAYA_ID, syn_conn(head, tgt2, 2)).ok();
    }
}

fn config_disabled() -> FrameWeaverConfig {
    FrameWeaverConfig { scan_interval_ticks: u32::MAX, ..Default::default() }
}

fn config_active() -> FrameWeaverConfig {
    FrameWeaverConfig { scan_interval_ticks: 1, ..Default::default() }
}

// ── Группа A: FW disabled ────────────────────────────────────────────────────

fn bench_fw_disabled(c: &mut Criterion) {
    let mut group = c.benchmark_group("FrameWeaver overhead / A: disabled (drain only)");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(500);

    for tokens in [0, 10, 50] {
        let cmd = tick_cmd();
        group.bench_with_input(
            BenchmarkId::new("tokens_in_logic", tokens),
            &tokens,
            |b, &n| {
                let mut engine = base_engine(config_disabled());
                // Добавить дополнительные токены
                for j in 51u32..=50 + n as u32 {
                    let mut c = UclCommand::new(OpCode::InjectToken, LOGIC_ID, 100, 0);
                    c.payload[0] = (LOGIC_ID & 0xff) as u8;
                    c.payload[1] = (LOGIC_ID >> 8)   as u8;
                    let mass = j as f32;
                    c.payload[4..8].copy_from_slice(&mass.to_le_bytes());
                    engine.process_command(&c);
                }
                b.iter(|| black_box(engine.process_command(black_box(&cmd))))
            },
        );
    }
    group.finish();
}

// ── Группа B: FW enabled, scan=1, MAYA пуста ─────────────────────────────────

fn bench_fw_empty_maya(c: &mut Criterion) {
    let mut group = c.benchmark_group("FrameWeaver overhead / B: active, MAYA empty");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(500);

    let cmd = tick_cmd();
    group.bench_function("tokens_50", |b| {
        let mut engine = base_engine(config_active());
        b.iter(|| black_box(engine.process_command(black_box(&cmd))))
    });

    group.finish();
}

// ── Группа C: FW enabled, MAYA с 5 узорами ───────────────────────────────────

fn bench_fw_patterns_5(c: &mut Criterion) {
    let mut group = c.benchmark_group("FrameWeaver overhead / C: active, 5 MAYA patterns");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(500);

    let cmd = tick_cmd();
    group.bench_function("tokens_50", |b| {
        let mut engine = base_engine(config_active());
        add_maya_patterns(&mut engine, 5);
        b.iter(|| black_box(engine.process_command(black_box(&cmd))))
    });

    group.finish();
}

// ── Группа D: FW enabled, MAYA с 20 узорами ──────────────────────────────────

fn bench_fw_patterns_20(c: &mut Criterion) {
    let mut group = c.benchmark_group("FrameWeaver overhead / D: active, 20 MAYA patterns");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(500);

    let cmd = tick_cmd();
    group.bench_function("tokens_50", |b| {
        let mut engine = base_engine(config_active());
        add_maya_patterns(&mut engine, 20);
        b.iter(|| black_box(engine.process_command(black_box(&cmd))))
    });

    group.finish();
}

// ── Изолированный замер scan_state ───────────────────────────────────────────

fn bench_scan_state_isolated(c: &mut Criterion) {
    use axiom_domain::DomainState;
    use axiom_config::DomainConfig;

    let mut group = c.benchmark_group("FrameWeaver: scan_state isolated");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(500);

    for patterns in [0, 5, 20, 50] {
        group.bench_with_input(
            BenchmarkId::new("maya_patterns", patterns),
            &patterns,
            |b, &n| {
                let fw = FrameWeaver::new(config_active());
                let mut state = DomainState::new(&DomainConfig::default());
                for i in 0..n {
                    let head = (i * 3 + 1) as u32;
                    let t1   = (i * 3 + 2) as u32;
                    let t2   = (i * 3 + 3) as u32;
                    state.connections.push(syn_conn(head, t1, 1));
                    state.connections.push(syn_conn(head, t2, 2));
                }
                b.iter(|| black_box(fw.scan_state_pub(black_box(&state), 110)))
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches_fw_overhead,
    bench_fw_disabled,
    bench_fw_empty_maya,
    bench_fw_patterns_5,
    bench_fw_patterns_20,
    bench_scan_state_isolated,
);
criterion_main!(benches_fw_overhead);
