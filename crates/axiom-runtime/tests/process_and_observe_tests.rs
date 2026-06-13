// Tests for AxiomEngine::process_and_observe() — Фаза 1 CLI Channel V1.1

use axiom_runtime::{AxiomEngine, ProcessingPath};
use axiom_ucl::{OpCode, UclCommand};

const SUTRA: u32 = 100;
const _MAYA: u16 = 110;

/// Собрать InjectToken UclCommand для SUTRA с заданными mass и temperature.
fn inject_cmd(mass: f32, temperature: f32) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, SUTRA, 100, 0);
    // payload layout (зеркало parse_inject_token_payload):
    // [0..2]   = target_domain_id (u16 LE)
    // [2]      = token_type
    // [4..8]   = mass (f32 LE)
    // [8..20]  = position[0..3] (3×f32 LE)
    // [20..32] = velocity[0..3] (3×f32 LE)
    // [32..36] = semantic_weight (f32 LE)
    // [36..40] = temperature (f32 LE)
    let domain_bytes = (SUTRA as u16).to_le_bytes();
    cmd.payload[0] = domain_bytes[0];
    cmd.payload[1] = domain_bytes[1];
    cmd.payload[4..8].copy_from_slice(&mass.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&temperature.to_le_bytes());
    cmd
}

#[test]
fn test_process_and_observe_returns_result() {
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(50.0, 100.0);
    let result = engine.process_and_observe(&cmd);
    // Просто не паникует и возвращает корректный статус
    assert_eq!(result.ucl_result.status, 0); // CommandStatus::Success = 0
}

#[test]
fn test_process_and_observe_no_reflex_initially() {
    // При пустом Experience рефлекса нет — нечему матчить.
    // Путь может быть SlowPath или MultiPass (зависит от coherence конфига MAYA
    // и мембранных трансформов по 8 доменам) — не пинаем конкретное значение.
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(80.0, 200.0);
    let result = engine.process_and_observe(&cmd);
    assert!(!result.reflex_hit, "без Experience не должно быть рефлекса");
    assert!(
        !matches!(result.path, ProcessingPath::Reflex),
        "путь не должен быть Reflex без Experience, получено {:?}", result.path
    );
}

#[test]
fn test_process_and_observe_coherence_score_present() {
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(60.0, 150.0);
    let result = engine.process_and_observe(&cmd);
    // coherence_score заполняется при маршрутизации
    assert!(result.coherence_score.is_some());
    let c = result.coherence_score.unwrap();
    assert!((0.0..=1.0).contains(&c));
}

#[test]
fn test_process_and_observe_dominant_domain_valid() {
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(70.0, 180.0);
    let result = engine.process_and_observe(&cmd);
    // dominant_domain_id должен быть в диапазоне 100..=110
    assert!(
        result.dominant_domain_id >= 100 && result.dominant_domain_id <= 110,
        "dominant_domain_id {} out of range",
        result.dominant_domain_id
    );
}

#[test]
fn test_process_and_observe_tension_count_non_negative() {
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(40.0, 90.0);
    let result = engine.process_and_observe(&cmd);
    // tension_count — всегда корректное u32
    let _ = result.tension_count; // компилируется
}

#[test]
fn test_process_and_observe_non_inject_returns_ucl_result() {
    let mut engine = AxiomEngine::new();
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let result = engine.process_and_observe(&tick);
    // TickForward обрабатывается через process_command — статус Success (= 0)
    assert_eq!(result.ucl_result.status, 0);
    // Без маршрутизации — нет coherence_score
    assert!(result.coherence_score.is_none());
}

#[test]
fn test_process_and_observe_overhead_reasonable() {
    use std::time::Instant;
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(55.0, 130.0);

    // Прогрев
    for _ in 0..10 {
        engine.process_and_observe(&cmd);
    }

    let n = 100;
    let start = Instant::now();
    for _ in 0..n {
        std::hint::black_box(engine.process_and_observe(&cmd));
    }
    let elapsed = start.elapsed();
    let per_call = elapsed / n;

    // Не жёсткий порог — просто не должно быть катастрофической деградации
    // При debug-билде накладные расходы выше, поэтому 5ms — мягкий потолок
    assert!(
        per_call.as_millis() < 5,
        "process_and_observe overhead too high: {:?}",
        per_call
    );
}

#[test]
fn test_resolution_tension_created_in_experience() {
    // После разрешения дилеммы engine создаёт TensionTrace в Experience.
    // Engine выполняет CR on_tick каждые 7 тиков — прогоняем 14+ тиков.
    use axiom_runtime::over_domain::context_recognizer::dilemma_store::{
        DilemmaResolution, DilemmaType,
    };
    use axiom_ucl::OpCode;

    let mut engine = AxiomEngine::new();
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);

    // Создать дилемму напрямую в dilemma_store
    engine.context_recognizer.dilemma_store.push_active(
        DilemmaType::ValueConflict,
        vec![1, 2],
        0,
        0.9,
    );
    let id = engine.context_recognizer.dilemma_store.active[0].id;
    engine.context_recognizer.dilemma_store.resolve(id, DilemmaResolution::ContextualPriority { winner: 1 });

    let tension_before = engine.tension_count();

    // Прогнать 14 тиков — on_tick вызывается в t%7, drain_resolution_tensions в нём же.
    for _ in 0..14 {
        engine.process_command(&tick_cmd);
    }

    let tension_after = engine.tension_count();
    assert!(
        tension_after > tension_before,
        "TensionTrace должен быть создан после разрешения дилеммы: before={tension_before} after={tension_after}"
    );
}

#[test]
fn test_tension_decay_lower_traces_persist_longer() {
    // TensionTrace с temperature=255 и TENSION_DECAY=1 должен жить ~255 тиков.
    // Проверяем что trace создан и ещё жив через 200 тиков.
    use axiom_core::{Token, TOKEN_FLAG_DILEMMA};

    let mut engine = AxiomEngine::new();
    let exp_id = engine.ashti.level_id() * 100 + 9;
    let tok = {
        let mut t = Token::new(0xD000_0042, exp_id, [0i16; 3], 1);
        t.type_flags = TOKEN_FLAG_DILEMMA;
        t.temperature = 127;
        t
    };
    engine.ashti.experience_mut().add_tension_trace(tok, 127, 1);

    assert_eq!(engine.tension_count(), 1, "trace создан");

    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    // Decay вызывается в arbiter on_tick (cool_tension_traces). Прогоняем 200 тиков.
    // Нельзя напрямую вызвать cool — используем TickForward.
    // Heartbeat вызывает on_tick arbiter, что декрементирует temperature на TENSION_DECAY=1 за тик.
    for _ in 0..200 {
        engine.process_command(&tick_cmd);
    }

    // temperature=127, TENSION_DECAY=1, tension_check_interval=10.
    // После 200 тиков: 200/10=20 cool-циклов → temp=127-20=107 → trace ещё жив.
    assert!(
        engine.tension_count() > 0,
        "trace должен быть жив через 200 тиков при TENSION_DECAY=1, temp=127"
    );
}
