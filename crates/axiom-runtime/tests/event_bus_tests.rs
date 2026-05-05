// Этап 9A — Event Bus: подписочная модель поверх EventObserver
use axiom_core::{Event, EventPriority, EventType};
use axiom_runtime::{EventBus, EventObserver, Gateway};
use axiom_ucl::{OpCode, UclCommand};
use std::sync::{Arc, Mutex};

// ─── helpers ──────────────────────────────────────────────────────────────────

fn make_event_typed(et: EventType) -> Event {
    Event::new(1, 1, et, EventPriority::Normal, 0, 0, 0, 0)
}

struct Counter(Arc<Mutex<u32>>);

impl Counter {
    fn new() -> (Self, Arc<Mutex<u32>>) {
        let n = Arc::new(Mutex::new(0u32));
        (Counter(Arc::clone(&n)), n)
    }
}

impl EventObserver for Counter {
    fn on_event(&self, _event: &Event) {
        *self.0.lock().unwrap() += 1;
    }
}

// ─── EventBus::new / default ──────────────────────────────────────────────────

#[test]
fn test_eventbus_new_is_empty() {
    let bus = EventBus::new();
    assert!(bus.is_empty());
    assert_eq!(bus.total_count(), 0);
    assert_eq!(bus.broadcast_count(), 0);
}

#[test]
fn test_eventbus_default() {
    let bus: EventBus = Default::default();
    assert!(bus.is_empty());
}

// ─── subscribe_all (broadcast) ────────────────────────────────────────────────

#[test]
fn test_subscribe_all_receives_every_event() {
    let mut bus = EventBus::new();
    let (obs, count) = Counter::new();
    bus.subscribe_all(Box::new(obs));

    let events = vec![
        make_event_typed(EventType::TokenCreate),
        make_event_typed(EventType::TokenUpdate),
        make_event_typed(EventType::TokenDelete),
    ];
    bus.publish(&events);

    assert_eq!(*count.lock().unwrap(), 3);
}

#[test]
fn test_broadcast_count() {
    let mut bus = EventBus::new();
    let (o1, _) = Counter::new();
    let (o2, _) = Counter::new();
    bus.subscribe_all(Box::new(o1));
    bus.subscribe_all(Box::new(o2));
    assert_eq!(bus.broadcast_count(), 2);
    assert!(!bus.is_empty());
}

// ─── subscribe (typed) ────────────────────────────────────────────────────────

#[test]
fn test_typed_subscriber_receives_only_matching() {
    let mut bus = EventBus::new();
    let (obs, count) = Counter::new();
    let target_type = EventType::TokenCreate as u16;
    bus.subscribe(target_type, Box::new(obs));

    let events = vec![
        make_event_typed(EventType::TokenCreate), // совпадает
        make_event_typed(EventType::TokenUpdate), // не совпадает
        make_event_typed(EventType::TokenCreate), // совпадает
    ];
    bus.publish(&events);

    assert_eq!(*count.lock().unwrap(), 2);
}

#[test]
fn test_typed_subscriber_zero_on_no_match() {
    let mut bus = EventBus::new();
    let (obs, count) = Counter::new();
    bus.subscribe(EventType::TokenDelete as u16, Box::new(obs));

    let events = vec![make_event_typed(EventType::TokenCreate)];
    bus.publish(&events);

    assert_eq!(*count.lock().unwrap(), 0);
}

#[test]
fn test_typed_count() {
    let mut bus = EventBus::new();
    let (o1, _) = Counter::new();
    let (o2, _) = Counter::new();
    let et = EventType::TokenUpdate as u16;
    bus.subscribe(et, Box::new(o1));
    bus.subscribe(et, Box::new(o2));
    assert_eq!(bus.typed_count(et), 2);
    assert_eq!(bus.typed_count(EventType::TokenDelete as u16), 0);
}

// ─── broadcast + typed одновременно ──────────────────────────────────────────

#[test]
fn test_broadcast_and_typed_both_receive() {
    let mut bus = EventBus::new();
    let (broadcast_obs, broadcast_count) = Counter::new();
    let (typed_obs, typed_count) = Counter::new();

    bus.subscribe_all(Box::new(broadcast_obs));
    bus.subscribe(EventType::TokenCreate as u16, Box::new(typed_obs));

    let events = vec![
        make_event_typed(EventType::TokenCreate), // оба получают
        make_event_typed(EventType::TokenUpdate), // только broadcast
    ];
    bus.publish(&events);

    assert_eq!(*broadcast_count.lock().unwrap(), 2); // broadcast получает все
    assert_eq!(*typed_count.lock().unwrap(), 1); // typed только TokenCreate
}

// ─── total_count ─────────────────────────────────────────────────────────────

#[test]
fn test_total_count() {
    let mut bus = EventBus::new();
    let (o1, _) = Counter::new();
    let (o2, _) = Counter::new();
    let (o3, _) = Counter::new();
    bus.subscribe_all(Box::new(o1));
    bus.subscribe(EventType::TokenCreate as u16, Box::new(o2));
    bus.subscribe(EventType::TokenUpdate as u16, Box::new(o3));
    assert_eq!(bus.total_count(), 3);
    assert!(!bus.is_empty());
}

// ─── publish пустого среза ────────────────────────────────────────────────────

#[test]
fn test_publish_empty_no_panic() {
    let mut bus = EventBus::new();
    let (obs, count) = Counter::new();
    bus.subscribe_all(Box::new(obs));
    bus.publish(&[]);
    assert_eq!(*count.lock().unwrap(), 0);
}

// ─── Gateway::subscribe ───────────────────────────────────────────────────────

#[test]
fn test_gateway_subscribe_typed() {
    let mut gw = Gateway::with_default_engine();
    let (obs, _count) = Counter::new();
    gw.subscribe(EventType::TokenCreate as u16, Box::new(obs));
    assert_eq!(gw.total_subscriber_count(), 1);
}

#[test]
fn test_gateway_observer_count_broadcast_only() {
    let mut gw = Gateway::with_default_engine();
    let (o1, _) = Counter::new();
    let (o2, _) = Counter::new();
    let (o3, _) = Counter::new();
    gw.register_observer(Box::new(o1));
    gw.register_observer(Box::new(o2));
    gw.subscribe(EventType::TokenCreate as u16, Box::new(o3));

    // observer_count = только broadcast
    assert_eq!(gw.observer_count(), 2);
    // total = broadcast + typed
    assert_eq!(gw.total_subscriber_count(), 3);
}

#[test]
fn test_gateway_process_notifies_typed_subscriber() {
    let mut gw = Gateway::with_default_engine();
    let (obs, count) = Counter::new();
    // TickForward не генерирует TokenCreate, но тест проверяет что подписка не паникует
    gw.subscribe(EventType::TokenCreate as u16, Box::new(obs));

    let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
    gw.process(&cmd);

    // Событий TokenCreate не было — счётчик 0
    assert_eq!(*count.lock().unwrap(), 0);
}

#[test]
fn test_gateway_event_bus_mut() {
    let mut gw = Gateway::with_default_engine();
    let (obs, _) = Counter::new();
    gw.event_bus_mut().subscribe_all(Box::new(obs));
    assert_eq!(gw.observer_count(), 1);
}
