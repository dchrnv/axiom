// Integration tests for Phase C coordinator (I1) and from_anchor_set (I2).
//
// I1: AE/CR/NA инстанцированы в Engine, получают on_tick через координатор.
// I2: ContextRecognizer::from_anchor_set строит subsystem_refs из AnchorSet.

use axiom_config::{Anchor, AnchorSet};
use axiom_runtime::AxiomEngine;
use axiom_ucl::{OpCode, UclCommand};

fn tick(engine: &mut AxiomEngine, n: u64) {
    for _ in 0..n {
        let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
        let _ = engine.process_command(&cmd);
    }
}

// AxialEvaluator инстанцирован и его storage доступно с tick=0.
#[test]
fn test_axial_evaluator_present_after_new() {
    let engine = AxiomEngine::new();
    // Storage существует — AxialStore пустой изначально
    assert_eq!(engine.axial_evaluator.storage().store().total_evaluations(), 0);
}

// ContextRecognizer инстанцирован.
#[test]
fn test_context_recognizer_present_after_new() {
    let engine = AxiomEngine::new();
    // profile_store().len() == 0 — нет профилей до первого тика
    assert_eq!(engine.context_recognizer.profile_store().len(), 0);
}

// NeuralAdvisor инстанцирован.
#[test]
fn test_neural_advisor_present_after_new() {
    let engine = AxiomEngine::new();
    // emergent_store() существует — пустой до тиков
    assert_eq!(engine.neural_advisor.emergent_store().len(), 0);
}

// После 5 тиков AE::on_tick вызван хотя бы один раз (tick % 5 == 0).
// В пустом Engine AE не генерирует оценок, но он должен вызваться без паники.
#[test]
fn test_ae_on_tick_runs_without_panic() {
    let mut engine = AxiomEngine::new();
    tick(&mut engine, 5);
    // Нет паники — тест проходит. storage() всё ещё доступно.
    let _ = engine.axial_evaluator.storage().store().total_evaluations();
}

// После 7 тиков CR::on_tick вызван хотя бы один раз.
#[test]
fn test_cr_on_tick_runs_without_panic() {
    let mut engine = AxiomEngine::new();
    tick(&mut engine, 7);
    let _ = engine.context_recognizer.profile_store().len();
}

// После 11 тиков NA::on_tick вызван хотя бы один раз.
#[test]
fn test_na_on_tick_runs_without_panic() {
    let mut engine = AxiomEngine::new();
    tick(&mut engine, 11);
    let _ = engine.neural_advisor.emergent_store().len();
}

// Синхронизация: после тика кратного 5, NA имеет снапшот из AE.
// В пустой системе axial_store клонируется без паники.
#[test]
fn test_axial_store_sync_to_na_runs_without_panic() {
    let mut engine = AxiomEngine::new();
    // tick 5 → AE on_tick + sync AxialStore → CR + NA
    tick(&mut engine, 5);
    // Синхронизация произошла — NA может использовать axial снапшот
    let _ = engine.neural_advisor.emergent_store().len();
}

// ApproveEmergentCandidate (I4): UCL opcode 5201 обрабатывается Engine.
// Для несуществующего sutra_id approve_emergent возвращает false (не паникует).
#[test]
fn test_approve_emergent_candidate_opcode_handled() {
    let mut engine = AxiomEngine::new();
    let mut cmd = UclCommand::new(OpCode::ApproveEmergentCandidate, 0, 0, 0);
    // sutra_id = 999 в payload bytes 0..4 (little-endian)
    let id: u32 = 999;
    cmd.payload[0..4].copy_from_slice(&id.to_le_bytes());
    let result = engine.process_command(&cmd);
    // Команда принята без системной ошибки
    assert_eq!(result.status, axiom_ucl::CommandStatus::Success as u8);
}

// ApproveEmergentCandidatePayload round-trip: поле sutra_id читается корректно.
#[test]
fn test_approve_emergent_payload_layout() {
    let mut cmd = UclCommand::new(OpCode::ApproveEmergentCandidate, 0, 0, 0);
    let id: u32 = 42_000;
    cmd.payload[0..4].copy_from_slice(&id.to_le_bytes());
    let parsed = u32::from_le_bytes([cmd.payload[0], cmd.payload[1], cmd.payload[2], cmd.payload[3]]);
    assert_eq!(parsed, id);
}

// ── I2: ContextRecognizer::from_anchor_set ───────────────────────────────────

fn make_anchor(word: &str, position: [i16; 3]) -> Anchor {
    Anchor {
        id: word.to_string(),
        word: word.to_string(),
        aliases: vec![],
        tags: vec![],
        position,
        shell: [0; 8],
        description: String::new(),
    }
}

fn make_anchor_set_with_writing() -> AnchorSet {
    let mut set = AnchorSet::empty();
    set.subsystems.insert(
        "writing".to_string(),
        vec![
            make_anchor("существительное", [100, 50, 0]),
            make_anchor("глагол", [120, 40, 10]),
            make_anchor("метафора", [80, 60, 20]),
        ],
    );
    set
}

// from_anchor_set с заполненной подсистемой writing — CR готов к расчёту энергий.
#[test]
fn test_from_anchor_set_writing_refs_populated() {
    let anchors = make_anchor_set_with_writing();
    let mut engine = AxiomEngine::new();
    engine.apply_anchor_set(&anchors);
    // CR инициализирован — profile_store пустой (данных нет), но не паникует на тике
    let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
    for _ in 0..7 {
        let _ = engine.process_command(&cmd);
    }
    // После 7 тиков on_tick вызван — profile_store всё ещё пустой (нет Frame в MAYA),
    // но без паники
    assert_eq!(engine.context_recognizer.profile_store().len(), 0);
}

// from_anchor_set с пустым AnchorSet — CR создаётся без паники, работает как new(HashMap::new()).
#[test]
fn test_from_anchor_set_empty_anchors() {
    let anchors = AnchorSet::empty();
    let mut engine = AxiomEngine::new();
    engine.apply_anchor_set(&anchors);
    assert_eq!(engine.context_recognizer.profile_store().len(), 0);
}

// apply_anchor_set дважды не паникует — stores сбрасываются корректно.
#[test]
fn test_apply_anchor_set_twice_is_safe() {
    let anchors = make_anchor_set_with_writing();
    let mut engine = AxiomEngine::new();
    engine.apply_anchor_set(&anchors);
    engine.apply_anchor_set(&AnchorSet::empty());
    assert_eq!(engine.context_recognizer.profile_store().len(), 0);
}
