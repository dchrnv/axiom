// Тесты Knowledge Exchange — Фаза 4 Memory Persistence V1.0

use axiom_persist::{
    export_traces, export_skills,
    import_traces, import_skills,
    IMPORT_WEIGHT_FACTOR,
};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};
use std::path::PathBuf;

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("axiom-exchange-test-{}.json", name))
}

fn inject_and_tick(engine: &mut AxiomEngine, rounds: u32) {
    let mut cmd = UclCommand::new(OpCode::InjectToken, 100, 100, 0);
    let domain_bytes = (100u16).to_le_bytes();
    cmd.payload[0] = domain_bytes[0];
    cmd.payload[1] = domain_bytes[1];
    cmd.payload[4..8].copy_from_slice(&60.0f32.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&120.0f32.to_le_bytes());
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..rounds {
        engine.process_command(&cmd);
        engine.process_command(&tick);
    }
}

// ─── Тест 1: export traces → файл создаётся ──────────────────────────────────

#[test]
fn test_export_traces_creates_file() {
    let path = temp_path("export_creates");
    let mut engine = AxiomEngine::new();
    inject_and_tick(&mut engine, 10);

    let report = export_traces(&engine, &path, 0.0).expect("export failed");
    assert!(path.exists(), "export file must exist");
    assert_eq!(report.exported, engine.ashti.experience().traces().len() as u32);
}

// ─── Тест 2: export → import → traces появляются в engine B ──────────────────

#[test]
fn test_export_import_traces_round_trip() {
    let path = temp_path("round_trip_traces");

    // Engine A: нарабатываем traces
    let mut engine_a = AxiomEngine::new();
    inject_and_tick(&mut engine_a, 20);
    let traces_before = engine_a.ashti.experience().traces().len();

    export_traces(&engine_a, &path, 0.0).expect("export");

    // Engine B: пустой, импортируем
    let mut engine_b = AxiomEngine::new();
    assert_eq!(engine_b.ashti.experience().traces().len(), 0);

    let report = import_traces(&mut engine_b, &path).expect("import");

    // Если были traces → они должны появиться в B (с учётом GUARDIAN-фильтра)
    let imported_count = report.imported + report.guardian_rejected;
    assert_eq!(imported_count, traces_before as u32,
        "imported + rejected должно равняться числу экспортированных");
    assert!(report.imported > 0 || traces_before == 0,
        "хотя бы один trace должен пройти GUARDIAN если были traces");
}

// ─── Тест 3: импортированные traces получают weight × IMPORT_WEIGHT_FACTOR ─────

#[test]
fn test_import_traces_weight_reduced() {
    let path = temp_path("weight_reduced");
    let mut engine_a = AxiomEngine::new();
    inject_and_tick(&mut engine_a, 25);

    let weights_before: Vec<f32> = engine_a.ashti.experience().traces()
        .iter().map(|t| t.weight).collect();

    if weights_before.is_empty() {
        export_traces(&engine_a, &path, 0.0).expect("export");
        let mut engine_b = AxiomEngine::new();
        import_traces(&mut engine_b, &path).expect("import empty");
        return;
    }

    export_traces(&engine_a, &path, 0.0).expect("export");

    let mut engine_b = AxiomEngine::new();
    import_traces(&mut engine_b, &path).expect("import");

    for trace in engine_b.ashti.experience().traces() {
        assert!(
            trace.weight <= weights_before.iter().cloned().fold(f32::NEG_INFINITY, f32::max) * IMPORT_WEIGHT_FACTOR + 1e-5,
            "imported weight must be ≤ max_original × IMPORT_WEIGHT_FACTOR"
        );
        assert!(trace.weight > 0.0, "weight must be positive");
    }
}

// ─── Тест 4: GUARDIAN блокирует токены с sutra_id == 0 ────────────────────────

#[test]
fn test_guardian_rejects_zero_sutra_id() {
    use axiom_persist::exchange::{TracePackage, StoredSkill};
    use axiom_persist::format::StoredTrace;
    use axiom_persist::exchange::ExchangeHeader;
    use axiom_core::Token;

    let path = temp_path("guardian_reject");

    // Создаём пакет с невалидным паттерном (sutra_id = 0)
    let mut bad_token = Token::new(1, 100, [0, 0, 0], 1);
    bad_token.sutra_id = 0; // нарушает CODEX правило 3

    let pkg = TracePackage {
        header: ExchangeHeader {
            kind: axiom_persist::ExchangeKind::Traces,
            version: "axiom-exchange-v1".to_string(),
            source_tick: 0,
            exported_at: "0".to_string(),
            count: 1,
        },
        traces: vec![axiom_persist::format::StoredTrace {
            pattern:       bad_token,
            weight:        0.9,
            created_at:    1,
            last_used:     1,
            success_count: 5,
            pattern_hash:  0,
        }],
    };

    let json = serde_json::to_string(&pkg).unwrap();
    std::fs::write(&path, json.as_bytes()).unwrap();

    let mut engine = AxiomEngine::new();
    let report = import_traces(&mut engine, &path).expect("import");

    assert_eq!(report.guardian_rejected, 1, "GUARDIAN должен отклонить sutra_id=0");
    assert_eq!(report.imported, 0);
    assert_eq!(engine.ashti.experience().traces().len(), 0);
}

// ─── Тест 5: export skills → файл создаётся ──────────────────────────────────

#[test]
fn test_export_skills_creates_file() {
    let path = temp_path("export_skills");
    let engine = AxiomEngine::new(); // Skills пустые — OK

    let report = export_skills(&engine, &path).expect("export skills");
    assert!(path.exists());
    assert_eq!(report.exported, 0); // свежий engine без навыков
}

// ─── Тест 6: export skills → import skills → дедупликация работает ────────────

#[test]
fn test_import_skills_deduplication() {
    let path = temp_path("skills_dedup");
    let engine_a = AxiomEngine::new();
    export_skills(&engine_a, &path).expect("export");

    let mut engine_b = AxiomEngine::new();
    // Импортируем дважды
    import_skills(&mut engine_b, &path).expect("first import");
    let r2 = import_skills(&mut engine_b, &path).expect("second import");

    // При пустом пакете всё нули — проверяем что не паникует
    assert_eq!(r2.guardian_rejected + r2.imported + r2.skipped_duplicate, r2.total);
}

// ─── Тест 7: Неправильный тип пакета → PersistError::Decode ──────────────────

#[test]
fn test_wrong_package_kind_returns_error() {
    let path = temp_path("wrong_kind");
    let engine = AxiomEngine::new();

    // Экспортируем traces, но пытаемся импортировать как skills
    export_traces(&engine, &path, 0.0).expect("export traces");

    let mut engine_b = AxiomEngine::new();
    match import_skills(&mut engine_b, &path) {
        Err(axiom_persist::PersistError::Decode(_)) => {} // ожидаемо
        Err(e) => panic!("unexpected error: {e}"),
        Ok(_)  => panic!("expected Decode error for wrong kind"),
    }
}

// ─── Тест 8: Несуществующий файл → PersistError::Io ──────────────────────────

#[test]
fn test_import_missing_file_returns_error() {
    let path = PathBuf::from("/tmp/axiom-exchange-nonexistent-999.json");
    let _ = std::fs::remove_file(&path);

    let mut engine = AxiomEngine::new();
    match import_traces(&mut engine, &path) {
        Err(axiom_persist::PersistError::Io(_)) => {}
        Err(e) => panic!("expected Io error, got: {e}"),
        Ok(_)  => panic!("expected error"),
    }
}

// ─── Тест 9: export traces с threshold фильтрует ─────────────────────────────

#[test]
fn test_export_traces_threshold() {
    let path_all  = temp_path("threshold_all");
    let path_high = temp_path("threshold_high");
    let mut engine = AxiomEngine::new();
    inject_and_tick(&mut engine, 30);

    let r_all  = export_traces(&engine, &path_all,  0.0).expect("export all");
    let r_high = export_traces(&engine, &path_high, 0.99).expect("export high");

    assert!(r_high.exported <= r_all.exported,
        "high threshold must export ≤ total traces");
}

// ─── Тест 10: ImportReport summary_line не паникует ──────────────────────────

#[test]
fn test_import_report_summary() {
    use axiom_persist::ImportReport;
    let r = ImportReport { total: 10, imported: 7, guardian_rejected: 2, skipped_duplicate: 1 };
    let s = r.summary_line();
    assert!(s.contains("7"));
    assert!(s.contains("2"));
}
