// Тесты boot sequence — Фаза 2 Memory Persistence V1.0

use axiom_persist::{save, load, WriteOptions};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};
use std::path::PathBuf;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("axiom-boot-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn inject_cmd() -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, 100, 100, 0);
    let domain_bytes = (100u16).to_le_bytes();
    cmd.payload[0] = domain_bytes[0];
    cmd.payload[1] = domain_bytes[1];
    cmd.payload[4..8].copy_from_slice(&50.0f32.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&100.0f32.to_le_bytes());
    cmd
}

// ─── Тест 1: save → новый engine → load → состояние восстановлено ─────────────

#[test]
fn test_boot_restore_full_cycle() {
    let dir = temp_dir("boot_full");
    let mut engine_a = AxiomEngine::new();

    // Нарабатываем состояние
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let cmd = inject_cmd();
    for _ in 0..20 {
        engine_a.process_command(&cmd);
        engine_a.process_command(&tick);
    }

    let tick_before    = engine_a.tick_count;
    let com_before     = engine_a.com_next_id;
    let tokens_before: usize = engine_a.snapshot().domains.iter().map(|d| d.tokens.len()).sum();

    save(&engine_a, &dir, &WriteOptions::default()).expect("save failed");

    // "Перезапуск" — новый engine, загружаем
    let result = load(&dir).expect("load failed");

    assert_eq!(result.engine.tick_count,  tick_before,  "tick_count мismatch");
    assert_eq!(result.engine.com_next_id, com_before,   "com_next_id mismatch");

    let tokens_after: usize = result.engine.snapshot().domains.iter().map(|d| d.tokens.len()).sum();
    assert_eq!(tokens_before, tokens_after, "token count mismatch after boot restore");
}

// ─── Тест 2: Отсутствие axiom-data/ → load возвращает NotFound ────────────────

#[test]
fn test_boot_missing_data_dir() {
    let dir = PathBuf::from("/tmp/axiom-boot-nonexistent-9999");
    let _ = std::fs::remove_dir_all(&dir);

    // Эмулируем логику boot_engine: нет manifest → чистый старт
    let manifest_path = dir.join("manifest.yaml");
    assert!(!manifest_path.exists());

    // Убеждаемся что load возвращает ошибку (а не паникует)
    match load(&dir) {
        Err(axiom_persist::PersistError::NotFound(_)) => {} // ожидаемо
        Err(e) => panic!("unexpected error: {e}"),
        Ok(_)  => panic!("expected error when dir missing"),
    }
}

// ─── Тест 3: После load traces работают (рефлексы восстанавливаются) ───────────

#[test]
fn test_boot_traces_are_active_after_load() {
    let dir = temp_dir("boot_traces");
    let mut engine_a = AxiomEngine::new();

    // Создаём traces через инъекции
    let cmd = inject_cmd();
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..30 {
        engine_a.process_command(&cmd);
        engine_a.process_command(&tick);
    }

    let traces_before = engine_a.ashti.experience().traces().len();
    save(&engine_a, &dir, &WriteOptions::default()).expect("save failed");
    let result = load(&dir).expect("load failed");

    let traces_after = result.engine.ashti.experience().traces().len();
    assert_eq!(traces_before, traces_after,
        "trace count should survive boot restore: before={traces_before}, after={traces_after}");

    // Traces должны быть ненулевого веса (пусть и сниженного)
    for trace in result.engine.ashti.experience().traces() {
        assert!(trace.weight > 0.0, "all traces must have positive weight after load");
    }
}

// ─── Тест 4: Повторный save после load не теряет данные ───────────────────────

#[test]
fn test_boot_resave_after_load() {
    let dir = temp_dir("boot_resave");
    let mut engine_a = AxiomEngine::new();

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..5 {
        engine_a.process_command(&tick);
    }

    save(&engine_a, &dir, &WriteOptions::default()).expect("initial save");
    let r1 = load(&dir).expect("first load");

    // Дополнительные тики после загрузки
    let mut engine_b = r1.engine;
    for _ in 0..5 {
        engine_b.process_command(&tick);
    }
    let tick2 = engine_b.tick_count;

    save(&engine_b, &dir, &WriteOptions::default()).expect("resave");
    let r2 = load(&dir).expect("second load");

    assert_eq!(r2.engine.tick_count, tick2, "tick_count after resave/reload");
}

// ─── Тест 5: data_dir по умолчанию "axiom-data" ───────────────────────────────

#[test]
fn test_default_data_dir_name() {
    // Проверяем что дефолтное значение правильное — это контракт между
    // CliConfig::default() и bin/axiom-cli.rs
    use axiom_agent::channels::cli::CliConfig;
    let cfg = CliConfig::default();
    assert_eq!(cfg.data_dir, "axiom-data");
}
