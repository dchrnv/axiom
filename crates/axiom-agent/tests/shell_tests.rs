// Этап 10D — Shell Effector: whitelist allow/deny
use axiom_agent::channels::shell::ShellEffector;
use axiom_core::{Event, EventType, EventPriority};
use axiom_runtime::Effector;
use std::path::Path;

fn make_effector() -> ShellEffector {
    ShellEffector::new(vec![
        "echo hello".into(),
        "date".into(),
        "uptime".into(),
    ])
}

fn make_shell_event() -> Event {
    Event::new(1, 100, EventType::ShellExec, EventPriority::Normal, 0, 0, 0, 0)
}

// ─── whitelist check ─────────────────────────────────────────────────────────

#[test]
fn test_is_allowed_exact_match() {
    let e = make_effector();
    assert!(e.is_allowed("echo hello"));
    assert!(e.is_allowed("date"));
    assert!(e.is_allowed("uptime"));
}

#[test]
fn test_is_not_allowed_unknown_command() {
    let e = make_effector();
    assert!(!e.is_allowed("rm -rf /"));
    assert!(!e.is_allowed("curl http://evil.com"));
    assert!(!e.is_allowed("echo"));   // partial match not allowed
    assert!(!e.is_allowed("echo hello && rm -rf"));
}

#[test]
fn test_empty_whitelist_denies_all() {
    let e = ShellEffector::new(vec![]);
    assert!(!e.is_allowed("echo hello"));
    assert!(!e.is_allowed("date"));
}

// ─── execute_command ──────────────────────────────────────────────────────────

#[test]
fn test_execute_allowed_command() {
    let mut e = make_effector();
    let result = e.execute_command("echo hello");
    assert!(result);
    assert_eq!(e.executed.len(), 1);
    assert_eq!(e.executed[0], "echo hello");
    assert!(e.denied.is_empty());
}

#[test]
fn test_execute_denied_command() {
    let mut e = make_effector();
    let result = e.execute_command("rm -rf /");
    assert!(!result);
    assert!(e.executed.is_empty());
    assert_eq!(e.denied.len(), 1);
    assert_eq!(e.denied[0], "rm -rf /");
}

#[test]
fn test_execute_multiple_commands() {
    let mut e = make_effector();
    e.execute_command("echo hello");
    e.execute_command("rm -rf /"); // denied
    e.execute_command("date");
    assert_eq!(e.executed.len(), 2);
    assert_eq!(e.denied.len(), 1);
}

// ─── Effector trait ───────────────────────────────────────────────────────────

#[test]
fn test_effector_name() {
    let e = make_effector();
    assert_eq!(e.name(), "shell");
}

#[test]
fn test_effector_emit_non_shell_event_ignored() {
    let mut e = make_effector();
    let event = Event::new(1, 100, EventType::TokenCreate, EventPriority::Normal, 0, 0, 0, 0);
    e.emit(&event); // должен быть проигнорирован без паники
    assert!(e.executed.is_empty());
    assert!(e.denied.is_empty());
}

#[test]
fn test_effector_emit_shell_event_no_panic() {
    let mut e = make_effector();
    e.emit(&make_shell_event()); // ShellExec без payload — no-op в текущем MVP
    // В текущей архитектуре Event не несёт строкового payload
    // команды передаются через execute_command() напрямую
    assert!(e.executed.is_empty()); // no payload = no execution
}

// ─── from_whitelist_file ──────────────────────────────────────────────────────

#[test]
fn test_load_from_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/shell_whitelist.yaml");
    let e = ShellEffector::from_whitelist_file(&path).unwrap();
    assert!(e.is_allowed("echo hello"));
    assert!(e.is_allowed("date"));
    assert!(e.is_allowed("uptime"));
    assert!(e.is_allowed("hostname"));
    assert!(!e.is_allowed("rm -rf /"));
}

#[test]
fn test_load_from_missing_file_error() {
    let result = ShellEffector::from_whitelist_file(Path::new("/nonexistent.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_load_from_temp_yaml() {
    let yaml = "whitelist:\n  - \"echo test\"\n  - \"ls\"\n";
    let tmp = std::env::temp_dir().join("axiom_wl_test.yaml");
    std::fs::write(&tmp, yaml).unwrap();
    let e = ShellEffector::from_whitelist_file(&tmp).unwrap();
    assert!(e.is_allowed("echo test"));
    assert!(e.is_allowed("ls"));
    assert!(!e.is_allowed("rm -rf /"));
    std::fs::remove_file(tmp).ok();
}
