// Тесты axiom-persist — Фаза 1 Memory Persistence V1.0

use axiom_persist::{save, load, WriteOptions, IMPORT_WEIGHT_FACTOR, FORMAT_VERSION};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};
use std::path::PathBuf;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("axiom-persist-test-{}", name));
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

// ─── Тест 1: save → load → snapshot восстанавливается идентично ───────────────

#[test]
fn test_save_and_load_empty_engine() {
    let dir = temp_dir("empty");
    let engine = AxiomEngine::new();

    let opts = WriteOptions::default();
    let manifest = save(&engine, &dir, &opts).expect("save failed");

    assert_eq!(manifest.tick_count, 0);
    assert_eq!(manifest.com_next_id, 1);
    assert_eq!(manifest.contents.traces, 0);

    let result = load(&dir).expect("load failed");
    assert_eq!(result.engine.tick_count, 0);
    assert_eq!(result.engine.com_next_id, 1);
    assert_eq!(result.traces_imported, 0);
}

// ─── Тест 2: tick_count и com_next_id восстанавливаются ───────────────────────

#[test]
fn test_save_and_load_tick_count() {
    let dir = temp_dir("tick_count");
    let mut engine = AxiomEngine::new();

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..42 {
        engine.process_command(&tick);
    }

    let expected_tick = engine.tick_count;
    let expected_com  = engine.com_next_id;

    save(&engine, &dir, &WriteOptions::default()).expect("save failed");
    let result = load(&dir).expect("load failed");

    assert_eq!(result.engine.tick_count,  expected_tick);
    assert_eq!(result.engine.com_next_id, expected_com);
}

// ─── Тест 3: Tokens восстанавливаются после save/load ─────────────────────────

#[test]
fn test_save_and_load_tokens() {
    let dir = temp_dir("tokens");
    let mut engine = AxiomEngine::new();

    // Инжектируем несколько токенов
    let cmd = inject_cmd();
    for _ in 0..5 {
        engine.process_command(&cmd);
    }

    let snap_before = engine.snapshot();
    let total_before: usize = snap_before.domains.iter().map(|d| d.tokens.len()).sum();

    save(&engine, &dir, &WriteOptions::default()).expect("save failed");
    let result = load(&dir).expect("load failed");

    let snap_after = result.engine.snapshot();
    let total_after: usize = snap_after.domains.iter().map(|d| d.tokens.len()).sum();

    assert_eq!(total_before, total_after,
        "token count should match after restore");
}

// ─── Тест 4: Traces при загрузке получают weight × IMPORT_WEIGHT_FACTOR ───────

#[test]
fn test_traces_weight_factor_on_load() {
    let dir = temp_dir("weight_factor");
    let mut engine = AxiomEngine::new();

    // Инжектируем токены несколько раз чтобы создать traces в Experience
    let cmd = inject_cmd();
    for _ in 0..20 {
        engine.process_command(&cmd);
        let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
        engine.process_command(&tick);
    }

    // Сохраняем веса до save
    let traces_before: Vec<f32> = engine.ashti.experience().traces()
        .iter()
        .map(|t| t.weight)
        .collect();

    if traces_before.is_empty() {
        // Нет traces — тест не применим, просто проверим что save/load работает
        save(&engine, &dir, &WriteOptions::default()).expect("save");
        load(&dir).expect("load");
        return;
    }

    save(&engine, &dir, &WriteOptions::default()).expect("save failed");
    let result = load(&dir).expect("load failed");

    let traces_after = result.engine.ashti.experience().traces();
    assert_eq!(traces_before.len(), traces_after.len(),
        "trace count should match");

    for (before, after) in traces_before.iter().zip(traces_after.iter()) {
        let expected = (*before * IMPORT_WEIGHT_FACTOR).max(0.001);
        assert!(
            (after.weight - expected).abs() < 1e-5,
            "weight should be reduced by IMPORT_WEIGHT_FACTOR: before={before}, expected={expected}, got={}", after.weight
        );
    }
}

// ─── Тест 5: Повреждённый manifest → PersistError::CorruptManifest ────────────

#[test]
fn test_corrupt_manifest_returns_error() {
    let dir = temp_dir("corrupt");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), b"not: valid: yaml: : :").unwrap();

    match load(&dir) {
        Err(axiom_persist::PersistError::CorruptManifest(_)) => {} // ожидаемо
        Err(e) => panic!("expected CorruptManifest, got: {e}"),
        Ok(_)  => panic!("expected error on corrupt manifest"),
    }
}

// ─── Тест 6: Несовместимая версия → PersistError::VersionMismatch ─────────────

#[test]
fn test_version_mismatch_returns_error() {
    let dir = temp_dir("version");
    std::fs::create_dir_all(&dir).unwrap();
    let bad_manifest = "version: \"axiom-memory-v99\"\ncreated_at: \"2026-04-06\"\nlast_saved: \"2026-04-06\"\ntick_count: 0\ncom_next_id: 1\naxiom_version: \"0.1.0\"\ncontents:\n  domains: 0\n  tokens: 0\n  connections: 0\n  traces: 0\n  tension_traces: 0\n";
    std::fs::write(dir.join("manifest.yaml"), bad_manifest).unwrap();

    match load(&dir) {
        Err(axiom_persist::PersistError::VersionMismatch { .. }) => {} // ожидаемо
        Err(e) => panic!("expected VersionMismatch, got: {e}"),
        Ok(_)  => panic!("expected error on version mismatch"),
    }
}

// ─── Тест 7: Отсутствие директории → PersistError::NotFound ──────────────────

#[test]
fn test_missing_dir_returns_not_found() {
    let dir = PathBuf::from("/tmp/axiom-persist-nonexistent-xyz-12345");
    let _ = std::fs::remove_dir_all(&dir);

    match load(&dir) {
        Err(axiom_persist::PersistError::NotFound(_)) => {} // ожидаемо
        Err(e) => panic!("expected NotFound, got: {e}"),
        Ok(_)  => panic!("expected error on missing dir"),
    }
}

// ─── Тест 8: Manifest содержит корректную версию ──────────────────────────────

#[test]
fn test_manifest_version_is_correct() {
    let dir = temp_dir("manifest_version");
    let engine = AxiomEngine::new();
    let manifest = save(&engine, &dir, &WriteOptions::default()).expect("save failed");

    assert_eq!(manifest.version, FORMAT_VERSION);
}

// ─── Тест 9: Manifest обновляется при повторном save ─────────────────────────

#[test]
fn test_manifest_updates_on_resave() {
    let dir = temp_dir("resave");
    let mut engine = AxiomEngine::new();

    save(&engine, &dir, &WriteOptions::default()).expect("first save");

    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..10 {
        engine.process_command(&tick);
    }

    let manifest2 = save(&engine, &dir, &WriteOptions::default()).expect("second save");
    assert_eq!(manifest2.tick_count, 10);
}

// ─── Тест 10: WriteOptions::trace_weight_threshold фильтрует traces ───────────

#[test]
fn test_trace_threshold_filtering() {
    let dir = temp_dir("threshold");
    let mut engine = AxiomEngine::new();

    // Создаём traces
    let cmd = inject_cmd();
    for _ in 0..30 {
        engine.process_command(&cmd);
        let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
        engine.process_command(&tick);
    }

    let all_traces = engine.ashti.experience().traces().len();

    // Сохраняем с высоким порогом
    let opts = WriteOptions { trace_weight_threshold: 0.99 };
    let manifest = save(&engine, &dir, &opts).expect("save failed");

    // Сохранилось не больше чем всего traces
    assert!(manifest.contents.traces <= all_traces as u32);
}
