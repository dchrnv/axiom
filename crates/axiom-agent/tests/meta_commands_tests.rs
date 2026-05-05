// SPDX-License-Identifier: AGPL-3.0-only
// Тесты для handle_meta_read / handle_meta_mutate (Phase 0B).

use axiom_agent::channels::cli::{CliConfig, PerfTracker};
use axiom_agent::meta_commands::{handle_meta_mutate, handle_meta_read, MetaAction};
use axiom_persist::{AutoSaver, PersistenceConfig};
use axiom_runtime::AxiomEngine;
use std::collections::HashSet;
use std::collections::VecDeque;

fn make_engine() -> AxiomEngine {
    AxiomEngine::new()
}
fn make_perf() -> PerfTracker {
    PerfTracker::new_for_test()
}
fn make_saver() -> AutoSaver {
    AutoSaver::new(PersistenceConfig::disabled())
}

fn read(line: &str) -> String {
    let engine = make_engine();
    let config = CliConfig::default();
    handle_meta_read(
        line,
        &engine,
        None,
        &config,
        &HashSet::new(),
        &VecDeque::new(),
        &make_perf(),
        0,
        0,
    )
}

// ── handle_meta_read ──────────────────────────────────────────────────────────

#[test]
fn test_handle_meta_read_status_nonempty() {
    let out = read(":status");
    assert!(
        out.contains("tick_count"),
        "expected tick_count in :status output"
    );
}

#[test]
fn test_handle_meta_read_domains_lists_11() {
    let out = read(":domains");
    let lines = out.lines().filter(|l| l.contains("tokens")).count();
    assert_eq!(lines, 11, ":domains should list exactly 11 domains");
}

#[test]
fn test_handle_meta_read_unknown_cmd_hint() {
    let out = read(":nonexistent_command");
    assert!(out.contains(":help"), "unknown command should hint :help");
}

// ── handle_meta_mutate ────────────────────────────────────────────────────────

#[test]
fn test_handle_meta_mutate_quit_returns_quit_action() {
    let mut engine = make_engine();
    let mut saver = make_saver();
    let config = CliConfig::default();
    let result = handle_meta_mutate(":quit", &mut engine, &mut saver, &config);
    assert!(matches!(result.action, MetaAction::Quit));
    assert!(result.output.contains("Завершение"));
}

#[test]
fn test_handle_meta_mutate_tick_advances_engine() {
    let mut engine = make_engine();
    let mut saver = make_saver();
    let config = CliConfig::default();
    let before = engine.tick_count;
    handle_meta_mutate(":tick 5", &mut engine, &mut saver, &config);
    assert_eq!(engine.tick_count, before + 5);
}

#[test]
fn test_handle_meta_mutate_save_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    let dir_str = dir.path().to_str().unwrap().to_string();
    let mut engine = make_engine();
    let mut saver = make_saver();
    let config = CliConfig::default();
    let line = format!(":save {}", dir_str);
    let result = handle_meta_mutate(&line, &mut engine, &mut saver, &config);
    assert!(
        result.output.contains("saved"),
        "expected 'saved' in output"
    );
    assert!(dir.path().exists());
}

#[test]
fn test_handle_meta_mutate_load_replaces_engine() {
    // Сначала сохраняем
    let dir = tempfile::tempdir().unwrap();
    let dir_str = dir.path().to_str().unwrap().to_string();
    let mut engine = make_engine();
    let mut saver = make_saver();
    let config = CliConfig::default();
    handle_meta_mutate(
        &format!(":save {}", dir_str),
        &mut engine,
        &mut saver,
        &config,
    );

    // Теперь загружаем
    let result = handle_meta_mutate(
        &format!(":load {}", dir_str),
        &mut engine,
        &mut saver,
        &config,
    );
    assert!(
        matches!(result.action, MetaAction::EngineReplaced),
        "expected EngineReplaced"
    );
}
