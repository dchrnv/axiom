// Интеграционный тест: инжектировать стихи 200 раз, наблюдать накопление опыта.
//
// Запуск: cargo test -p axiom-agent --test ingest_poetry_test -- --nocapture --ignored

use axiom_agent::ingester::FileIngester;
use axiom_config::AnchorSet;
use axiom_runtime::AxiomEngine;
use axiom_ucl::{OpCode, UclCommand};
use std::path::Path;
use std::sync::Arc;

const POETRY_PATH: &str = "../../docs/archive/Стихи 2025.md";
const ROUNDS: usize = 200;
const TICKS_PER_ROUND: usize = 20;

#[test]
#[ignore]
fn test_ingest_poetry_repeated() {
    let poetry_path = Path::new(POETRY_PATH);
    if !poetry_path.exists() {
        eprintln!("skip: {} not found", POETRY_PATH);
        return;
    }

    // Загружаем движок с якорями
    let config_path = Path::new("../../config");
    let anchor_set = AnchorSet::load_or_empty(config_path);
    let crystal_count = anchor_set.crystal.len();
    let anchor_count = anchor_set.total_count();

    let ingester = FileIngester::with_anchors(Arc::new(anchor_set));

    let mut engine = AxiomEngine::new();
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Предварительный dry-run
    let report = ingester.dry_run_md(poetry_path).unwrap();
    println!("\n=== ПОЭЗИЯ: dry-run ===");
    print!("{report}");
    println!("  якоря: {anchor_count}, crystal: {crystal_count}");
    println!("  файл: {}\n", poetry_path.display());

    // Первый прогон — собираем команды один раз (они детерминированы)
    let (commands, info) = ingester.load_md(poetry_path).unwrap();
    println!("=== КОМАНДЫ ===");
    println!("  чанков: {}", info.chunks_total);
    println!("  UCL команд: {}", commands.len());
    println!("  C1 биграмм: {}", info.c1_seeds_total);
    if !info.hints_mismatch.is_empty() {
        println!("  hint расхождений: {}", info.hints_mismatch.len());
        for m in &info.hints_mismatch[..info.hints_mismatch.len().min(3)] {
            println!("    {m}");
        }
    }

    println!("\n=== ПРОГОН {ROUNDS} РАУНДОВ (по {TICKS_PER_ROUND} тиков после каждого) ===");
    println!("{:>6} {:>8} {:>8} {:>8} {:>8}",
        "round", "traces", "tension", "frames", "matched");

    let snapshots = [1, 5, 10, 25, 50, 100, 150, ROUNDS];
    let mut prev_traces = 0usize;

    for round in 1..=ROUNDS {
        // Инжектируем через process_and_observe → Arbiter полный путь → Experience
        // stable_id детерминирован → повтор = подкрепление, не дубль
        for cmd in &commands {
            if cmd.opcode == axiom_ucl::OpCode::InjectToken as u16 {
                engine.process_and_observe(cmd);
            } else {
                engine.process_command(cmd);
            }
        }

        // Даём движку тики для обработки
        for _ in 0..TICKS_PER_ROUND {
            engine.process_command(&tick_cmd);
        }

        if snapshots.contains(&round) {
            let traces = engine.trace_count();
            let tension = engine.tension_count();
            let frames = engine.frame_weaver.composition_store.len();
            let matched = engine.last_matched();

            let delta = if traces > prev_traces {
                format!("+{}", traces - prev_traces)
            } else {
                "=".to_string()
            };

            println!("{:>6} {:>6}({:>4}) {:>8} {:>8} {:>8}",
                round, traces, delta, tension, frames, matched);
            prev_traces = traces;
        }
    }

    println!("\n=== ФИНАЛЬНОЕ СОСТОЯНИЕ (после {} раундов) ===", ROUNDS);
    let final_traces = engine.trace_count();
    let final_tension = engine.tension_count();

    println!("  Experience traces:  {final_traces}");
    println!("  Tension traces:     {final_tension}");
    println!("  Crystallized frames:{}", engine.frame_weaver.composition_store.len());
    println!("  Last matched:       {}", engine.last_matched());
    println!("  Tick count:         {}", engine.tick_count);

    println!("\n=== ДОМИНИРУЮЩИЕ ПОДСИСТЕМЫ (dry-run) ===");
    // Ещё раз dry-run чтобы увидеть детекцию
    let report2 = ingester.dry_run_md(poetry_path).unwrap();
    let mut subs: Vec<_> = report2.subsystems.iter().collect();
    subs.sort_by(|a, b| b.1.cmp(a.1));
    for (sub, count) in &subs {
        let bar = "#".repeat((**count).min(40usize));
        println!("  {:20} {:>4} {}", sub, count, bar);
    }

    // Проверяем что experience накопился
    assert!(final_traces > 0, "должны быть Experience traces после {ROUNDS} раундов");
}
