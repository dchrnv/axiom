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
fn test_process_and_observe_slow_path_initially() {
    // При пустом Experience — всегда SlowPath (нет следов для рефлекса)
    let mut engine = AxiomEngine::new();
    let cmd = inject_cmd(80.0, 200.0);
    let result = engine.process_and_observe(&cmd);
    assert_eq!(result.path, ProcessingPath::SlowPath);
    assert!(!result.reflex_hit);
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
