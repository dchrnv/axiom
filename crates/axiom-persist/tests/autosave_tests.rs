// Тесты автосохранения — Фаза 3 Memory Persistence V1.0

use axiom_persist::{AutoSaver, PersistenceConfig, load};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};
use std::path::PathBuf;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("axiom-autosave-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn tick_engine(engine: &mut AxiomEngine, n: u64) {
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..n {
        engine.process_command(&tick);
    }
}

// ─── Тест 1: should_save = false при отключённом autosave ────────────────────

#[test]
fn test_autosave_disabled_never_saves() {
    let dir = temp_dir("disabled");
    let cfg = PersistenceConfig::disabled();
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 10000);

    for _ in 0..5 {
        assert!(!saver.tick(&engine, &dir), "disabled autosaver must never trigger");
    }
    assert!(!dir.join("manifest.yaml").exists());
}

// ─── Тест 2: should_save = false пока интервал не наступил ───────────────────

#[test]
fn test_autosave_before_interval() {
    let dir = temp_dir("before_interval");
    let cfg = PersistenceConfig::new(100);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 50); // меньше интервала

    assert!(!saver.should_save(&engine));
    assert!(!saver.tick(&engine, &dir));
}

// ─── Тест 3: should_save = true при достижении интервала ─────────────────────

#[test]
fn test_autosave_triggers_at_interval() {
    let dir = temp_dir("triggers");
    let cfg = PersistenceConfig::new(100);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 100);

    assert!(saver.should_save(&engine));
    let saved = saver.tick(&engine, &dir);
    assert!(saved, "autosave должен сработать при tick=100");
    assert!(dir.join("manifest.yaml").exists());
}

// ─── Тест 4: После сохранения — следующее только через интервал ───────────────

#[test]
fn test_autosave_respects_interval_after_save() {
    let dir = temp_dir("respects");
    let cfg = PersistenceConfig::new(100);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 100);
    saver.tick(&engine, &dir); // первое сохранение

    // Добавляем 50 тиков — ещё не достигли следующего интервала
    tick_engine(&mut engine, 50);
    assert!(!saver.should_save(&engine), "должен ждать ещё 50 тиков");

    // Ещё 50 — достигли
    tick_engine(&mut engine, 50);
    assert!(saver.should_save(&engine), "должен сработать на tick=200");
}

// ─── Тест 5: save_count корректно считает ─────────────────────────────────────

#[test]
fn test_autosave_count() {
    let dir = temp_dir("count");
    let cfg = PersistenceConfig::new(10);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    for i in 1..=3u64 {
        tick_engine(&mut engine, 10);
        saver.tick(&engine, &dir);
        assert_eq!(saver.save_count, i, "save_count after {i} intervals");
    }
}

// ─── Тест 6: Файл появляется без явного :save ─────────────────────────────────

#[test]
fn test_autosave_file_created_automatically() {
    let dir = temp_dir("auto_file");
    assert!(!dir.join("manifest.yaml").exists());

    let cfg = PersistenceConfig::new(50);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 50);
    saver.tick(&engine, &dir);

    assert!(dir.join("manifest.yaml").exists(),   "manifest должен появиться");
    assert!(dir.join("engine_state.json").exists(), "engine_state должен появиться");
}

// ─── Тест 7: После autosave — load восстанавливает состояние ─────────────────

#[test]
fn test_autosave_then_load_restores() {
    let dir = temp_dir("autosave_load");
    let cfg = PersistenceConfig::new(30);
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 30);
    saver.tick(&engine, &dir);

    let tick_saved = engine.tick_count;
    let result = load(&dir).expect("load after autosave");
    assert_eq!(result.engine.tick_count, tick_saved);
}

// ─── Тест 8: Прерванная запись — manifest не обновлён → load читает старый ────

#[test]
fn test_interrupted_write_old_state_survives() {
    let dir = temp_dir("interrupted");

    let mut engine = AxiomEngine::new();
    tick_engine(&mut engine, 5);
    let cfg = PersistenceConfig::new(5);
    let mut saver = AutoSaver::new(cfg);
    saver.tick(&engine, &dir);

    let tick_before = engine.tick_count;

    let tmp_path = dir.join("engine_state.json.tmp");
    std::fs::write(&tmp_path, b"garbage corrupted data").unwrap();

    let result = load(&dir).expect("load should succeed with orphaned .tmp file");
    assert_eq!(result.engine.tick_count, tick_before,
        "должен загрузить старое состояние, .tmp проигнорирован");
}

// ─── Тест 9: force_save работает ─────────────────────────────────────────────

#[test]
fn test_force_save() {
    let dir = temp_dir("force_save");
    let cfg = PersistenceConfig::new(10000); // большой интервал
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 5);

    assert!(!saver.should_save(&engine));
    saver.force_save(&engine, &dir).expect("force_save должен работать");
    assert!(dir.join("manifest.yaml").exists());
    assert_eq!(saver.save_count, 1);
}

// ─── Тест 10: set_interval включает/выключает autosave ───────────────────────

#[test]
fn test_set_interval_toggles() {
    let dir = temp_dir("toggle");
    let cfg = PersistenceConfig::disabled();
    let mut saver = AutoSaver::new(cfg);
    let mut engine = AxiomEngine::new();

    tick_engine(&mut engine, 100);
    assert!(!saver.should_save(&engine), "disabled → no save");

    saver.set_interval(50);
    assert!(saver.should_save(&engine), "enabled → should save at tick=100");

    saver.tick(&engine, &dir);
    saver.set_interval(0);
    tick_engine(&mut engine, 1000);
    assert!(!saver.should_save(&engine), "disabled again → no save");
}
