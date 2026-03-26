// Integration tests for axiom-arbiter COM module
use axiom_arbiter::COM;

#[test]
fn test_com_starts_at_one() {
    let mut com = COM::new();
    let id = com.next_event_id(0);
    assert_eq!(id, 1);
}

#[test]
fn test_com_ids_are_monotone() {
    let mut com = COM::new();
    let a = com.next_event_id(0);
    let b = com.next_event_id(1);
    let c = com.next_event_id(0);
    assert!(a < b && b < c, "IDs must be strictly increasing");
}

#[test]
fn test_com_tracks_domain_event_count() {
    let mut com = COM::new();
    com.next_event_id(5);
    com.next_event_id(5);
    com.next_event_id(7);

    assert_eq!(com.domain_event_count(5), 2);
    assert_eq!(com.domain_event_count(7), 1);
    assert_eq!(com.domain_event_count(99), 0); // Never used
}

#[test]
fn test_com_current_id_advances() {
    let mut com = COM::new();
    let before = com.current_id();
    com.next_event_id(0);
    let after = com.current_id();
    assert_eq!(after, before + 1);
}

#[test]
fn test_com_different_domains_share_global_counter() {
    let mut com = COM::new();
    let id_a = com.next_event_id(1);
    let id_b = com.next_event_id(2);
    // Both domains get different IDs from the same global counter
    assert_ne!(id_a, id_b);
    assert_eq!(com.domain_event_count(1), 1);
    assert_eq!(com.domain_event_count(2), 1);
}
