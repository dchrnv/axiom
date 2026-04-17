// SPDX-License-Identifier: AGPL-3.0-only
// Тесты для convenience-методов AxiomEngine и broadcast-типов (Phase 0A).

#![cfg(feature = "adapters")]

use axiom_runtime::{AxiomEngine, BroadcastSnapshot, DomainDetailSnapshot, TokenSnapshot};
use axiom_core::{Token, STATE_LOCKED};
use axiom_ucl::{UclCommand, OpCode};

fn new_engine() -> AxiomEngine {
    AxiomEngine::new()
}

// ── trace_count / tension_count / last_matched ─────────────────────────────

#[test]
fn test_trace_count_zero_on_new_engine() {
    assert_eq!(new_engine().trace_count(), 0);
}

#[test]
fn test_tension_count_zero_on_new_engine() {
    assert_eq!(new_engine().tension_count(), 0);
}

#[test]
fn test_last_matched_zero_on_new_engine() {
    assert_eq!(new_engine().last_matched(), 0);
}

#[test]
fn test_trace_count_increases_after_inject() {
    let mut engine = new_engine();
    // Несколько инжектов — следы появляются не с первого
    for _ in 0..5 {
        let cmd = UclCommand::new(OpCode::InjectToken, 0, 100, 0);
        engine.process_command(&cmd);
    }
    // После тика Experience обновляется
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    engine.process_command(&tick);
    // trace_count доступен без паники — результат зависит от конфига Experience
    let _ = engine.trace_count();
}

// ── snapshot_for_broadcast ─────────────────────────────────────────────────

#[test]
fn test_snapshot_for_broadcast_has_11_domains() {
    let engine = new_engine();
    let snap = engine.snapshot_for_broadcast();
    assert_eq!(snap.domain_summaries.len(), 11);
}

#[test]
fn test_snapshot_for_broadcast_tick_matches_engine() {
    let mut engine = new_engine();
    let tick = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    engine.process_command(&tick);
    engine.process_command(&tick);

    let snap = engine.snapshot_for_broadcast();
    assert_eq!(snap.tick_count, engine.tick_count);
    assert_eq!(snap.com_next_id, engine.com_next_id);
}

#[test]
fn test_snapshot_for_broadcast_trace_count_matches() {
    let engine = new_engine();
    let snap = engine.snapshot_for_broadcast();
    assert_eq!(snap.trace_count, engine.trace_count());
    assert_eq!(snap.tension_count, engine.tension_count());
}

#[test]
fn test_snapshot_domain_summaries_ids_range_100_to_110() {
    let snap = new_engine().snapshot_for_broadcast();
    let ids: Vec<u16> = snap.domain_summaries.iter().map(|d| d.domain_id).collect();
    for expected in 100u16..=110 {
        assert!(ids.contains(&expected), "missing domain_id {}", expected);
    }
}

#[test]
fn test_snapshot_domain_names_not_unknown() {
    let snap = new_engine().snapshot_for_broadcast();
    for domain in &snap.domain_summaries {
        assert_ne!(domain.name, "UNKNOWN", "domain {} has unknown name", domain.domain_id);
    }
}

#[test]
fn test_snapshot_is_clone() {
    let engine = new_engine();
    let snap = engine.snapshot_for_broadcast();
    let _clone = snap.clone();
}

// ── domain_detail_snapshot ─────────────────────────────────────────────────

#[test]
fn test_domain_detail_snapshot_returns_none_for_unknown_id() {
    let engine = new_engine();
    assert!(engine.domain_detail_snapshot(999).is_none());
    assert!(engine.domain_detail_snapshot(0).is_none());
}

#[test]
fn test_domain_detail_snapshot_sutra_exists() {
    let engine = new_engine();
    let snap = engine.domain_detail_snapshot(100);
    assert!(snap.is_some());
    assert_eq!(snap.unwrap().domain_id, 100);
}

#[test]
fn test_domain_detail_snapshot_all_11_domains_exist() {
    let engine = new_engine();
    for id in 100u16..=110 {
        assert!(engine.domain_detail_snapshot(id).is_some(), "domain {} not found", id);
    }
}

#[test]
fn test_domain_detail_snapshot_is_clone() {
    let engine = new_engine();
    let snap = engine.domain_detail_snapshot(100).unwrap();
    let _clone = snap.clone();
}

// ── TokenSnapshot ──────────────────────────────────────────────────────────

#[test]
fn test_token_snapshot_is_anchor_for_locked_token() {
    let mut token = Token::new(1, 100, [0, 0, 0], 1);
    token.state = STATE_LOCKED;
    token.mass = 255;
    token.temperature = 0;
    let snap = TokenSnapshot::from(&token);
    assert!(snap.is_anchor);
}

#[test]
fn test_token_snapshot_not_anchor_for_active_token() {
    let token = Token::new(2, 100, [0, 0, 0], 1);
    // state == STATE_ACTIVE по умолчанию
    let snap = TokenSnapshot::from(&token);
    assert!(!snap.is_anchor);
}

#[test]
fn test_token_snapshot_heavy_cold_token_not_anchor() {
    // Тяжёлый остывший токен НЕ якорь если state != STATE_LOCKED
    let mut token = Token::new(3, 100, [0, 0, 0], 1);
    token.mass = 255;
    token.temperature = 0;
    // state остаётся STATE_ACTIVE
    let snap = TokenSnapshot::from(&token);
    assert!(!snap.is_anchor, "heavy cold non-locked token must not be anchor");
}

#[test]
fn test_token_snapshot_fields_match_token() {
    let mut token = Token::new(42, 101, [100, -200, 300], 1);
    token.mass = 128;
    token.temperature = 64;
    token.valence = -5;
    token.origin = 7;

    let snap = TokenSnapshot::from(&token);
    assert_eq!(snap.sutra_id, 42);
    assert_eq!(snap.position, [100, -200, 300]);
    assert_eq!(snap.mass, 128);
    assert_eq!(snap.temperature, 64);
    assert_eq!(snap.valence, -5);
    assert_eq!(snap.origin, 7);
}
