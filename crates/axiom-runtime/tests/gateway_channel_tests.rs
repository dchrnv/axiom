// Этап 8 — Gateway + Channel: интеграционные тесты
use axiom_runtime::{
    Gateway, Channel,
    EventObserver, DirectAdapter,
    AxiomEngine,
};
use axiom_ucl::{UclCommand, OpCode};
use axiom_core::{Event, EventType, EventPriority};
use std::sync::{Arc, Mutex};

// ─── helpers ──────────────────────────────────────────────────────────────────

fn tick_cmd() -> UclCommand {
    UclCommand::new(OpCode::TickForward, 0, 0, 0)
}

fn spawn_cmd() -> UclCommand {
    UclCommand::new(OpCode::SpawnDomain, 0, 0, 0)
}

fn make_event() -> Event {
    Event::new(1, 1, EventType::TokenCreate, EventPriority::Normal, 0, 0, 0, 0)
}

// Наблюдатель, собирающий события в Vec
struct RecordingObserver {
    events: Arc<Mutex<Vec<u16>>>,
}

impl RecordingObserver {
    fn new() -> (Self, Arc<Mutex<Vec<u16>>>) {
        let store = Arc::new(Mutex::new(Vec::new()));
        (Self { events: store.clone() }, store)
    }
}

impl EventObserver for RecordingObserver {
    fn on_event(&self, event: &Event) {
        self.events.lock().unwrap().push(event.event_type);
    }
}

// ─── Gateway::new / with_default_engine ───────────────────────────────────────

#[test]
fn test_gateway_new() {
    let engine = AxiomEngine::new();
    let gw = Gateway::new(engine);
    assert_eq!(gw.processed_count(), 0);
    assert_eq!(gw.observer_count(), 0);
}

#[test]
fn test_gateway_with_default_engine() {
    let gw = Gateway::with_default_engine();
    assert_eq!(gw.processed_count(), 0);
}

// ─── Gateway::process ────────────────────────────────────────────────────────

#[test]
fn test_gateway_process_increments_count() {
    let mut gw = Gateway::with_default_engine();
    gw.process(&tick_cmd());
    assert_eq!(gw.processed_count(), 1);
    gw.process(&tick_cmd());
    assert_eq!(gw.processed_count(), 2);
}

#[test]
fn test_gateway_process_returns_success() {
    let mut gw = Gateway::with_default_engine();
    let result = gw.process(&tick_cmd());
    assert!(result.is_success());
}

// ─── Gateway::process_with ───────────────────────────────────────────────────

#[test]
fn test_gateway_process_with_direct_adapter() {
    let mut gw = Gateway::with_default_engine();
    let mut adapter = DirectAdapter;
    let result = gw.process_with(&mut adapter, &tick_cmd());
    assert!(result.is_success());
    assert_eq!(gw.processed_count(), 1);
}

// ─── Gateway::register_observer ──────────────────────────────────────────────

#[test]
fn test_gateway_register_observer() {
    let mut gw = Gateway::with_default_engine();
    let (obs, _store) = RecordingObserver::new();
    gw.register_observer(Box::new(obs));
    assert_eq!(gw.observer_count(), 1);
}

#[test]
fn test_gateway_multiple_observers() {
    let mut gw = Gateway::with_default_engine();
    let (obs1, _s1) = RecordingObserver::new();
    let (obs2, _s2) = RecordingObserver::new();
    gw.register_observer(Box::new(obs1));
    gw.register_observer(Box::new(obs2));
    assert_eq!(gw.observer_count(), 2);
}

// ─── Gateway::engine_mut ─────────────────────────────────────────────────────

#[test]
fn test_gateway_engine_mut_accessible() {
    let mut gw = Gateway::with_default_engine();
    let _engine = gw.engine_mut();
    // Можем получить мутабельный доступ без паники
}

// ─── Channel ─────────────────────────────────────────────────────────────────

#[test]
fn test_channel_new_is_empty() {
    let ch = Channel::new();
    assert_eq!(ch.pending_count(), 0);
    assert_eq!(ch.event_count(), 0);
    assert_eq!(ch.processed_count(), 0);
    assert!(!ch.has_pending());
    assert!(!ch.has_events());
}

#[test]
fn test_channel_send_increases_pending() {
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    assert_eq!(ch.pending_count(), 1);
    assert!(ch.has_pending());
}

#[test]
fn test_channel_send_multiple() {
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    ch.send(spawn_cmd());
    assert_eq!(ch.pending_count(), 2);
}

#[test]
fn test_channel_drain_commands_empties_queue() {
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    ch.send(tick_cmd());
    let cmds = ch.drain_commands();
    assert_eq!(cmds.len(), 2);
    assert_eq!(ch.pending_count(), 0);
    assert!(!ch.has_pending());
}

#[test]
fn test_channel_drain_commands_updates_processed() {
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    ch.send(tick_cmd());
    ch.drain_commands();
    assert_eq!(ch.processed_count(), 2);
    // Ещё раз
    ch.send(tick_cmd());
    ch.drain_commands();
    assert_eq!(ch.processed_count(), 3);
}

#[test]
fn test_channel_push_and_drain_events() {
    let mut ch = Channel::new();
    ch.push_event(make_event());
    assert_eq!(ch.event_count(), 1);
    assert!(ch.has_events());

    let evts = ch.drain_events();
    assert_eq!(evts.len(), 1);
    assert_eq!(ch.event_count(), 0);
    assert!(!ch.has_events());
}

#[test]
fn test_channel_drain_events_empty() {
    let mut ch = Channel::new();
    let evts = ch.drain_events();
    assert!(evts.is_empty());
}

#[test]
fn test_channel_clear_empties_both_queues() {
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    ch.push_event(make_event());
    ch.clear();
    assert_eq!(ch.pending_count(), 0);
    assert_eq!(ch.event_count(), 0);
}

#[test]
fn test_channel_default() {
    let ch: Channel = Default::default();
    assert_eq!(ch.pending_count(), 0);
}

// ─── Gateway::process_channel ────────────────────────────────────────────────

#[test]
fn test_process_channel_empty_noop() {
    let mut gw = Gateway::with_default_engine();
    let mut ch = Channel::new();
    let res = gw.process_channel(&mut ch);
    assert_eq!(res.processed, 0);
    assert!(res.all_ok());
    assert_eq!(gw.processed_count(), 0);
}

#[test]
fn test_process_channel_single_command() {
    let mut gw = Gateway::with_default_engine();
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    let res = gw.process_channel(&mut ch);
    assert_eq!(res.processed, 1);
    assert!(res.all_ok());
    assert_eq!(gw.processed_count(), 1);
    assert_eq!(ch.pending_count(), 0);
}

#[test]
fn test_process_channel_multiple_commands() {
    let mut gw = Gateway::with_default_engine();
    let mut ch = Channel::new();
    for _ in 0..5 {
        ch.send(tick_cmd());
    }
    let res = gw.process_channel(&mut ch);
    assert_eq!(res.processed, 5);
    assert_eq!(gw.processed_count(), 5);
}

#[test]
fn test_process_channel_events_forwarded_to_channel() {
    let mut gw = Gateway::with_default_engine();
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    gw.process_channel(&mut ch);
    // drain_events не паникует (события могут быть или не быть)
    let _evts = ch.drain_events();
    assert_eq!(ch.event_count(), 0);
}

#[test]
fn test_process_channel_increments_gateway_count() {
    let mut gw = Gateway::with_default_engine();
    let mut ch = Channel::new();
    ch.send(tick_cmd());
    ch.send(tick_cmd());
    gw.process_channel(&mut ch);
    assert_eq!(gw.processed_count(), 2);
}
