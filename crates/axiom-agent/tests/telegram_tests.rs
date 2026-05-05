// Этап 10C — Telegram Channel: mock JSON → команды
use axiom_agent::channels::telegram::{
    message_to_command, parse_updates, TelegramConfig, TelegramEffector, TelegramPerceptor,
    TelegramUpdate,
};
use axiom_core::{Event, EventPriority, EventType};
use axiom_runtime::{Effector, Perceptor};
use axiom_ucl::OpCode;

fn test_config() -> TelegramConfig {
    TelegramConfig {
        token: "test_token".into(),
        chat_id: 12345,
    }
}

fn make_event(et: EventType) -> Event {
    Event::new(1, 110, et, EventPriority::Normal, 0, 0, 0, 0)
}

// ─── parse_updates ────────────────────────────────────────────────────────────

#[test]
fn test_parse_single_update() {
    let json =
        r#"{"ok":true,"result":[{"update_id":42,"message":{"text":"tick","chat":{"id":123}}}]}"#;
    let updates = parse_updates(json);
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].update_id, 42);
    assert_eq!(updates[0].text, "tick");
    assert_eq!(updates[0].chat_id, 123);
}

#[test]
fn test_parse_multiple_updates() {
    let json = r#"{"ok":true,"result":[
        {"update_id":1,"message":{"text":"tick","chat":{"id":1}}},
        {"update_id":2,"message":{"text":"inject 100","chat":{"id":1}}}
    ]}"#;
    let updates = parse_updates(json);
    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].text, "tick");
    assert_eq!(updates[1].text, "inject 100");
}

#[test]
fn test_parse_empty_result() {
    let json = r#"{"ok":true,"result":[]}"#;
    let updates = parse_updates(json);
    assert!(updates.is_empty());
}

#[test]
fn test_parse_invalid_json_returns_empty() {
    let updates = parse_updates("not json");
    assert!(updates.is_empty());
}

// ─── message_to_command ───────────────────────────────────────────────────────

#[test]
fn test_message_tick_to_command() {
    let cmd = message_to_command("tick").unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16);
}

#[test]
fn test_message_inject_to_command() {
    let cmd = message_to_command("inject 106").unwrap();
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    assert_eq!(cmd.target_id, 106);
}

#[test]
fn test_message_unknown_returns_none() {
    assert!(message_to_command("hello world").is_none());
    assert!(message_to_command("/start").is_none());
}

// ─── TelegramPerceptor ────────────────────────────────────────────────────────

#[test]
fn test_perceptor_feed_update_tick() {
    let mut p = TelegramPerceptor::new(test_config());
    p.feed_update(TelegramUpdate {
        update_id: 1,
        text: "tick".into(),
        chat_id: 1,
    });
    assert_eq!(p.pending_count(), 1);
    let cmd = p.receive().unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16);
}

#[test]
fn test_perceptor_feed_unknown_not_queued() {
    let mut p = TelegramPerceptor::new(test_config());
    p.feed_update(TelegramUpdate {
        update_id: 1,
        text: "/unknown".into(),
        chat_id: 1,
    });
    assert_eq!(p.pending_count(), 0);
}

#[test]
fn test_perceptor_multiple_updates() {
    let mut p = TelegramPerceptor::new(test_config());
    p.feed_update(TelegramUpdate {
        update_id: 1,
        text: "tick".into(),
        chat_id: 1,
    });
    p.feed_update(TelegramUpdate {
        update_id: 2,
        text: "inject 100".into(),
        chat_id: 1,
    });
    assert_eq!(p.pending_count(), 2);
    assert_eq!(p.receive().unwrap().opcode, OpCode::TickForward as u16);
    assert_eq!(p.receive().unwrap().opcode, OpCode::InjectToken as u16);
    assert!(p.receive().is_none());
}

#[test]
fn test_perceptor_empty_returns_none() {
    let mut p = TelegramPerceptor::new(test_config());
    assert!(p.receive().is_none());
}

#[test]
fn test_perceptor_name() {
    let p = TelegramPerceptor::new(test_config());
    assert_eq!(p.name(), "telegram");
}

// ─── TelegramEffector (mock) ──────────────────────────────────────────────────

#[test]
fn test_effector_emit_event_stores_message() {
    let mut e = TelegramEffector::mock(test_config());
    e.emit(&make_event(EventType::TokenCreate));
    assert_eq!(e.sent_messages.len(), 1);
    assert!(e.sent_messages[0].contains("0x0001"));
}

#[test]
fn test_effector_emit_result_ok() {
    use axiom_ucl::UclCommand;
    let mut e = TelegramEffector::mock(test_config());
    let result = axiom_runtime::Gateway::with_default_engine().process(&UclCommand::new(
        OpCode::TickForward,
        0,
        0,
        0,
    ));
    e.emit_result(&result);
    assert_eq!(e.sent_messages.len(), 1);
    assert!(e.sent_messages[0].contains("OK"));
}

#[test]
fn test_effector_name() {
    let e = TelegramEffector::mock(test_config());
    assert_eq!(e.name(), "telegram");
}

// ─── round-trip: feed_update → receive → Gateway ─────────────────────────────

#[test]
fn test_telegram_roundtrip_no_panic() {
    use axiom_runtime::Gateway;

    let mut p = TelegramPerceptor::new(test_config());
    let mut e = TelegramEffector::mock(test_config());
    let mut gw = Gateway::with_default_engine();

    p.feed_update(TelegramUpdate {
        update_id: 1,
        text: "tick".into(),
        chat_id: 1,
    });
    p.feed_update(TelegramUpdate {
        update_id: 2,
        text: "inject 100".into(),
        chat_id: 1,
    });

    while let Some(cmd) = p.receive() {
        let result = gw.process(&cmd);
        e.emit_result(&result);
    }

    assert_eq!(e.sent_messages.len(), 2);
}
