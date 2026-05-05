// Этап 10B — CLI Channel: mock stdin → команды → mock stdout
use axiom_agent::channels::cli::{parse_cli_command, CliEffector, CliPerceptor};
use axiom_core::{Event, EventPriority, EventType};
use axiom_runtime::{Effector, Perceptor};
use axiom_ucl::{OpCode, UclCommand};

// ─── parse_cli_command ────────────────────────────────────────────────────────

#[test]
fn test_parse_tick() {
    let cmd = parse_cli_command("tick").unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16);
    assert_eq!(cmd.target_id, 0);
}

#[test]
fn test_parse_inject_with_domain() {
    let cmd = parse_cli_command("inject 106").unwrap();
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    assert_eq!(cmd.target_id, 106);
}

#[test]
fn test_parse_inject_default_domain() {
    let cmd = parse_cli_command("inject").unwrap();
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    assert_eq!(cmd.target_id, 100); // default SUTRA
}

#[test]
fn test_parse_status() {
    let cmd = parse_cli_command("status").unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16); // status → tick (query)
}

#[test]
fn test_parse_quit_returns_none() {
    assert!(parse_cli_command("quit").is_none());
}

#[test]
fn test_parse_exit_returns_none() {
    assert!(parse_cli_command("exit").is_none());
}

#[test]
fn test_parse_unknown_returns_none() {
    assert!(parse_cli_command("foobar").is_none());
    assert!(parse_cli_command("").is_none());
}

// ─── CliPerceptor ─────────────────────────────────────────────────────────────

#[test]
fn test_perceptor_reads_tick() {
    let input = b"tick\n";
    let mut p = CliPerceptor::from_reader(&input[..]);
    let cmd = p.receive().unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16);
}

#[test]
fn test_perceptor_reads_multiple_commands() {
    let input = b"tick\ninject 100\ntick\n";
    let mut p = CliPerceptor::from_reader(&input[..]);
    assert_eq!(p.receive().unwrap().opcode, OpCode::TickForward as u16);
    assert_eq!(p.receive().unwrap().opcode, OpCode::InjectToken as u16);
    assert_eq!(p.receive().unwrap().opcode, OpCode::TickForward as u16);
}

#[test]
fn test_perceptor_quit_returns_none() {
    let input = b"quit\n";
    let mut p = CliPerceptor::from_reader(&input[..]);
    assert!(p.receive().is_none());
}

#[test]
fn test_perceptor_eof_returns_none() {
    let input = b"";
    let mut p = CliPerceptor::from_reader(&input[..]);
    assert!(p.receive().is_none());
    assert!(p.is_done());
}

#[test]
fn test_perceptor_skips_unknown_lines() {
    // unknown → None (skip), then tick → Some
    let input = b"unknown\ntick\n";
    let mut p = CliPerceptor::from_reader(&input[..]);
    assert!(p.receive().is_none()); // unknown skipped
    let cmd = p.receive().unwrap();
    assert_eq!(cmd.opcode, OpCode::TickForward as u16);
}

#[test]
fn test_perceptor_name() {
    let input = b"";
    let p = CliPerceptor::from_reader(&input[..]);
    assert_eq!(p.name(), "cli");
}

// ─── CliEffector ─────────────────────────────────────────────────────────────

fn make_event(et: EventType) -> Event {
    Event::new(1, 100, et, EventPriority::Normal, 0, 0, 0, 0)
}

#[test]
fn test_effector_emit_event() {
    let mut buf = Vec::new();
    let mut e = CliEffector::from_writer(&mut buf);
    e.emit(&make_event(EventType::TokenCreate));
    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("[EVENT]"));
    assert!(out.contains("TokenCreate"));
}

#[test]
fn test_effector_emit_result_ok() {
    let mut buf = Vec::new();
    let mut e = CliEffector::from_writer(&mut buf);
    let result = axiom_runtime::Gateway::with_default_engine().process(&UclCommand::new(
        OpCode::TickForward,
        0,
        0,
        0,
    ));
    e.emit_result(&result);
    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("[RESULT]"));
    assert!(out.contains("OK"));
}

#[test]
fn test_effector_name() {
    let mut buf = Vec::new();
    let e = CliEffector::from_writer(&mut buf);
    assert_eq!(e.name(), "cli");
}

// ─── round-trip: stdin → Gateway → stdout ────────────────────────────────────

#[test]
fn test_cli_roundtrip_no_panic() {
    use axiom_runtime::Gateway;

    let input = b"tick\ntick\ninject 100\n";
    let mut p = CliPerceptor::from_reader(&input[..]);
    let mut buf = Vec::new();
    let mut e = CliEffector::from_writer(&mut buf);
    let mut gw = Gateway::with_default_engine();

    while let Some(cmd) = p.receive() {
        let result = gw.process(&cmd);
        e.emit_result(&result);
    }

    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("[RESULT]"));
}
