use axiom_core::{Event, EventType, EventPriority, Snapshot, EVENT_REVERSIBLE, EVENT_CRITICAL, EVENT_BATCHED};

#[test]
fn test_event_size() {
    assert_eq!(std::mem::size_of::<Event>(), 64);
    assert_eq!(std::mem::align_of::<Event>(), 64);
}

#[test]
fn test_event_creation_without_pulse() {
    let event = Event::new(
        1,
        0,
        EventType::TokenDecayed,
        EventPriority::Normal,
        0x1234567890ABCDEF,
        100,
        200,
        0,
    );

    assert_eq!(event.event_id, 1);
    assert_eq!(event.domain_id, 0);
    assert_eq!(event.event_type, EventType::TokenDecayed as u16);
    assert_eq!(event.pulse_id, 0);
    assert!(event.validate().is_ok());
}

#[test]
fn test_event_creation_with_pulse() {
    let event = Event::with_pulse(
        5,
        1,
        EventType::GravityUpdate,
        EventPriority::Low,
        0xABCDEF1234567890,
        150,
        250,
        4,
        42,
    );

    assert_eq!(event.event_id, 5);
    assert_eq!(event.pulse_id, 42);
    assert_eq!(event.event_type, EventType::GravityUpdate as u16);
    assert!(event.validate().is_ok());
}

#[test]
fn test_event_validation() {
    let mut event = Event::new(
        1,
        0,
        EventType::TokenCreate,
        EventPriority::Normal,
        0x1234,
        1,
        0,
        0,
    );
    assert!(event.validate().is_ok());

    event.event_id = 0;
    assert!(event.validate().is_err());
    event.event_id = 1;

    event.parent_event_id = 1;
    assert!(event.validate().is_err());
    event.parent_event_id = 0;

    event.payload_hash = 0;
    assert!(event.validate().is_err());
    event.payload_hash = 0x1234;

    event.event_type = 0xFFFF;
    assert!(event.validate().is_err());
}

#[test]
fn test_event_flags() {
    let mut event = Event::new(
        1,
        0,
        EventType::TokenCreate,
        EventPriority::Normal,
        0x1234,
        1,
        0,
        0,
    );

    event.flags = EVENT_CRITICAL;
    assert!(event.is_critical());
    assert!(!event.is_reversible());
    assert!(!event.is_batched());

    event.flags = EVENT_REVERSIBLE;
    assert!(!event.is_critical());
    assert!(event.is_reversible());

    event.flags = EVENT_BATCHED;
    assert!(event.is_batched());

    event.flags = EVENT_CRITICAL | EVENT_REVERSIBLE;
    assert!(event.is_critical());
    assert!(event.is_reversible());
}

#[test]
fn test_event_type_conversion() {
    assert_eq!(EventType::from(0x0001), EventType::TokenCreate);
    assert_eq!(EventType::from(0x0006), EventType::TokenDecayed);
    assert_eq!(EventType::from(0x1005), EventType::ConnectionWeakened);
    assert_eq!(EventType::from(0x3002), EventType::GravityUpdate);
}

#[test]
#[should_panic(expected = "Unknown event type")]
fn test_invalid_event_type_conversion() {
    EventType::from(0xAAAA);
}

#[test]
fn test_snapshot() {
    let snapshot = Snapshot::new(1000, 0x123456789ABCDEF0, 1000);
    assert_eq!(snapshot.snapshot_id, 1000);
    assert_eq!(snapshot.state_hash, 0x123456789ABCDEF0);
    assert_eq!(snapshot.event_count, 1000);
}

#[test]
fn test_event_priority() {
    let event = Event::new(
        1,
        0,
        EventType::TokenCreate,
        EventPriority::Critical,
        0x1234,
        1,
        0,
        0,
    );
    assert_eq!(event.get_priority(), EventPriority::Critical);
}

#[test]
fn test_semantic_event_types() {
    assert_eq!(EventType::TokenDecayed as u16, 0x0006);
    assert_eq!(EventType::TokenMerged as u16, 0x0007);
    assert_eq!(EventType::ConnectionWeakened as u16, 0x1005);
    assert_eq!(EventType::ConnectionBroken as u16, 0x1007);
    assert_eq!(EventType::GravityUpdate as u16, 0x3002);
    assert_eq!(EventType::CollisionDetected as u16, 0x3003);
}
