// Integration tests for axiom-runtime Snapshot
//
// AshtiCore содержит 11 фиксированных доменов (domain_id 100–110 для level_id=1).
// snapshot() захватывает все 11 доменов.

use axiom_runtime::{AxiomEngine, EngineSnapshot};
use axiom_ucl::{UclCommand, OpCode};

const LOGIC_ID: u16 = 106; // level_id(1) * 100 + role(6) = 106

fn inject_into(engine: &mut AxiomEngine, domain_id: u16) {
    let mut cmd = UclCommand::new(OpCode::InjectToken, domain_id as u32, 100, 0);
    cmd.payload[0] = (domain_id & 0xff) as u8;
    cmd.payload[1] = (domain_id >> 8) as u8;
    cmd.payload[4..8].copy_from_slice(&50.0f32.to_le_bytes());
    engine.process_command(&cmd);
}

// ============================================================
// EngineSnapshot::empty()
// ============================================================

#[test]
fn test_empty_snapshot() {
    let snap = EngineSnapshot::empty();
    assert_eq!(snap.domain_count(), 0);
    assert_eq!(snap.total_token_count(), 0);
}

// ============================================================
// engine.snapshot()
// ============================================================

#[test]
fn test_snapshot_captures_11_domains() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot();
    assert_eq!(snap.domain_count(), 11);
}

#[test]
fn test_snapshot_captures_tokens() {
    let mut engine = AxiomEngine::new();
    inject_into(&mut engine, LOGIC_ID);

    let snap = engine.snapshot();
    assert_eq!(snap.total_token_count(), 1);
}

#[test]
fn test_snapshot_find_domain() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot();
    assert!(snap.find_domain(100).is_some(), "SUTRA (id=100) должен быть в snapshot");
    assert!(snap.find_domain(110).is_some(), "MAYA (id=110) должен быть в snapshot");
    assert!(snap.find_domain(999).is_none(), "несуществующий домен не должен находиться");
}

// ============================================================
// AxiomEngine::restore_from()
// ============================================================

#[test]
fn test_restore_domain_count() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.domain_count(), 11);
}

#[test]
fn test_restore_token_count() {
    let mut engine = AxiomEngine::new();
    inject_into(&mut engine, LOGIC_ID);

    let snap = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.token_count(LOGIC_ID), 1);
}

#[test]
fn test_snapshot_roundtrip_domain_count() {
    let engine = AxiomEngine::new();
    let snap1 = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap1);
    let snap2 = restored.snapshot();
    assert_eq!(snap1.domain_count(), snap2.domain_count());
}

#[test]
fn test_snapshot_roundtrip_token_count() {
    let mut engine = AxiomEngine::new();
    inject_into(&mut engine, LOGIC_ID);

    let snap1 = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap1);
    let snap2 = restored.snapshot();
    assert_eq!(snap1.total_token_count(), snap2.total_token_count());
}

// ============================================================
// com_next_id — монотонность после restore
// ============================================================

#[test]
fn test_com_next_id_saved_in_snapshot() {
    let mut engine = AxiomEngine::new();
    // Инжектируем токены — com_next_id инкрементируется
    inject_into(&mut engine, LOGIC_ID);
    inject_into(&mut engine, LOGIC_ID);

    let snap = engine.snapshot();
    // com_next_id должен быть больше начального (1)
    assert!(snap.com_next_id > 1, "com_next_id должен сохраняться в snapshot");
}

#[test]
fn test_com_next_id_restored_monotonically() {
    let mut engine = AxiomEngine::new();
    inject_into(&mut engine, LOGIC_ID);
    inject_into(&mut engine, LOGIC_ID);

    let snap = engine.snapshot();
    let saved_com = snap.com_next_id;

    // После restore com_next_id не сбрасывается в 1
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.com_next_id, saved_com,
        "com_next_id после restore должен совпадать со snapshot");
}

// ============================================================
// tick_count — сохранение и восстановление
// ============================================================

#[test]
fn test_tick_count_saved_in_snapshot() {
    let mut engine = AxiomEngine::new();
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..7 {
        engine.process_command(&tick_cmd);
    }

    let snap = engine.snapshot();
    assert_eq!(snap.tick_count, 7, "tick_count должен сохраняться в snapshot");
}

#[test]
fn test_tick_count_restored() {
    let mut engine = AxiomEngine::new();
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    for _ in 0..42 {
        engine.process_command(&tick_cmd);
    }

    let snap = engine.snapshot();
    let restored = AxiomEngine::restore_from(&snap);
    assert_eq!(restored.tick_count, 42,
        "tick_count после restore должен совпадать со snapshot");
}

#[test]
fn test_tick_count_zero_in_empty_snapshot() {
    let snap = EngineSnapshot::empty();
    assert_eq!(snap.tick_count, 0);
}
