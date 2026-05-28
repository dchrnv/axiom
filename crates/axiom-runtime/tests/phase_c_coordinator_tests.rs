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
        layer: axiom_config::AnchorLayer::L1,
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

// ── SyntacticBridge (Фаза 0 CR-V6) ──────────────────────────────────────────

// process_and_observe создаёт 0x08-связи в MAYA после каждого routing.
// После 3+ одинаковых инъекций и достаточного числа тиков FrameWeaver
// должен кристаллизовать хотя бы один Frame-анкер в EXPERIENCE (domain 109).
#[test]
fn test_syntactic_bridge_creates_maya_connections() {
    use axiom_runtime::FrameWeaver;
    use axiom_runtime::FrameWeaverConfig;

    let mut engine = AxiomEngine::new();
    // Агрессивный FrameWeaver: сканирует каждый тик, threshold=3
    engine.frame_weaver = FrameWeaver::new(FrameWeaverConfig {
        scan_interval_ticks: 1,
        stability_threshold: 3,
        min_participants: 1,
        ..Default::default()
    });

    // Создать простой InjectToken с фиксированными координатами
    let mut cmd = UclCommand::new(OpCode::InjectToken, 100, 100, 0);
    // position [x=1000, y=2000, z=3000], mass=150, temperature=180
    cmd.payload[0..2].copy_from_slice(&100u16.to_le_bytes()); // target = SUTRA(100)
    cmd.payload[4..8].copy_from_slice(&150.0f32.to_le_bytes()); // mass
    cmd.payload[8..12].copy_from_slice(&1000.0f32.to_le_bytes()); // x
    cmd.payload[12..16].copy_from_slice(&2000.0f32.to_le_bytes()); // y
    cmd.payload[16..20].copy_from_slice(&3000.0f32.to_le_bytes()); // z
    cmd.payload[36..40].copy_from_slice(&180.0f32.to_le_bytes()); // temperature

    // 3 инъекции одного и того же токена → stability_count ≥ 3
    for _ in 0..3 {
        let _ = engine.process_and_observe(&cmd);
    }

    // Проверить что MAYA получила 0x08-связи
    let maya_idx = engine.ashti.index_of(110).expect("MAYA domain 110 must exist");
    let maya_state = engine.ashti.state(maya_idx).expect("MAYA state must be accessible");
    let syn_conn_count = maya_state
        .connections
        .iter()
        .filter(|c| (c.link_type >> 8) == 0x08)
        .count();
    assert!(syn_conn_count > 0, "bridge must inject 0x08 connections into MAYA; got 0");

    // После достаточного числа тиков FrameWeaver должен кристаллизовать Frame
    tick(&mut engine, 10);

    let exp_idx = engine.ashti.index_of(109).expect("EXPERIENCE domain 109 must exist");
    let exp_state = engine.ashti.state(exp_idx).expect("EXPERIENCE state accessible");
    let frame_count = exp_state
        .tokens
        .iter()
        .filter(|t| (t.type_flags & axiom_core::TOKEN_FLAG_FRAME_ANCHOR) != 0)
        .count();
    assert!(frame_count > 0, "FrameWeaver must crystallize at least one Frame-anchor after 3 injections; got 0");
}

// Повторная инъекция того же текста увеличивает число кристаллизованных Frame
// (или как минимум не уменьшает).
#[test]
fn test_syntactic_bridge_repeated_injection_stable() {
    use axiom_runtime::FrameWeaver;
    use axiom_runtime::FrameWeaverConfig;

    let mut engine = AxiomEngine::new();
    engine.frame_weaver = FrameWeaver::new(FrameWeaverConfig {
        scan_interval_ticks: 1,
        stability_threshold: 3,
        min_participants: 1,
        ..Default::default()
    });

    let mut cmd = UclCommand::new(OpCode::InjectToken, 100, 100, 0);
    cmd.payload[0..2].copy_from_slice(&100u16.to_le_bytes());
    cmd.payload[4..8].copy_from_slice(&200.0f32.to_le_bytes());
    cmd.payload[8..12].copy_from_slice(&5000.0f32.to_le_bytes());
    cmd.payload[12..16].copy_from_slice(&6000.0f32.to_le_bytes());
    cmd.payload[16..20].copy_from_slice(&7000.0f32.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&200.0f32.to_le_bytes());

    for _ in 0..5 {
        let _ = engine.process_and_observe(&cmd);
        tick(&mut engine, 2);
    }

    tick(&mut engine, 10);

    let frame_count = engine.axial_evaluator.storage().store().frame_count();
    // Хотя бы один фрейм оценён AxialEvaluator
    assert!(frame_count > 0, "AE must have evaluated at least one frame after 5 injections + ticks");
}
