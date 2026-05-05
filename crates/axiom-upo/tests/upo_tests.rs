use axiom_core::Token;
use axiom_upo::{
    DynamicTrace, Screen, TraceSourceType, UPOConfig, TRACE_ACTIVE, TRACE_ETERNAL, TRACE_FADING,
    UPO,
};

// ============================================================================
// TraceSourceType enum values
// ============================================================================

#[test]
fn test_trace_source_type_values() {
    assert_eq!(TraceSourceType::Token as u8, 1);
    assert_eq!(TraceSourceType::Connection as u8, 2);
    assert_eq!(TraceSourceType::Field as u8, 3);
}

// ============================================================================
// DynamicTrace size and alignment
// ============================================================================

#[test]
fn test_dynamic_trace_size() {
    assert_eq!(std::mem::size_of::<DynamicTrace>(), 32);
}

#[test]
fn test_dynamic_trace_alignment() {
    assert_eq!(std::mem::align_of::<DynamicTrace>(), 32);
}

// ============================================================================
// DynamicTrace::new and flag checks
// ============================================================================

#[test]
fn test_dynamic_trace_new_initial_values() {
    let trace = DynamicTrace::new(10, -5, 3, 1.5, 440.0, 1000, TraceSourceType::Token, 42, 7);

    assert_eq!(trace.x, 10);
    assert_eq!(trace.y, -5);
    assert_eq!(trace.z, 3);
    assert_eq!(trace.weight, 1.5);
    assert_eq!(trace.frequency, 440.0);
    assert_eq!(trace.last_update, 1000);
    assert_eq!(trace.source_type, TraceSourceType::Token as u8);
    assert_eq!(trace.source_id, 42);
    assert_eq!(trace.resonance_class, 7);
    assert_eq!(trace._pad, [0; 3]);
}

#[test]
fn test_dynamic_trace_new_starts_active() {
    let trace = DynamicTrace::new(0, 0, 0, 1.0, 1.0, 1, TraceSourceType::Token, 1, 0);
    assert!(trace.is_active());
    assert!(!trace.is_fading());
    assert!(!trace.is_eternal());
    assert_eq!(trace.flags, TRACE_ACTIVE);
}

#[test]
fn test_dynamic_trace_flag_constants() {
    assert_eq!(TRACE_ACTIVE, 0b0001);
    assert_eq!(TRACE_FADING, 0b0010);
    assert_eq!(TRACE_ETERNAL, 0b1000);
}

// ============================================================================
// Screen
// ============================================================================

#[test]
fn test_screen_new() {
    let screen = Screen::new([256, 256, 256], 0.001, 0.01);
    assert_eq!(screen.size, [256, 256, 256]);
    assert_eq!(screen.trace_count, 0);
    assert_eq!(screen.total_energy, 0.0);
    assert_eq!(screen.octant_mask, 0xFF);
}

#[test]
fn test_screen_write_valid_trace() {
    let mut screen = Screen::new([256, 256, 256], 0.001, 0.01);
    let trace = DynamicTrace::new(10, 10, 10, 1.0, 440.0, 1, TraceSourceType::Token, 1, 0);
    screen.set_current_event(1);
    screen.write(&trace);

    assert_eq!(screen.trace_count, 1);
    assert_eq!(screen.traces.len(), 1);
    assert!(screen.total_energy > 0.0);
}

#[test]
fn test_screen_write_invalid_trace_rejected() {
    let mut screen = Screen::new([100, 100, 100], 0.001, 0.01);
    // last_update=0 → validate() fails (требует > 0)
    let mut trace = DynamicTrace::new(0, 0, 0, 1.0, 1.0, 0, TraceSourceType::Token, 1, 0);
    trace.last_update = 0;
    screen.write(&trace);

    assert_eq!(screen.trace_count, 0);
}

#[test]
fn test_screen_decay_reduces_weight() {
    let mut screen = Screen::new([256, 256, 256], 0.001, 0.1);
    let trace = DynamicTrace::new(0, 0, 0, 10.0, 1.0, 1, TraceSourceType::Token, 1, 0);
    screen.set_current_event(1);
    screen.write(&trace);

    let weight_before = screen.traces[0].weight;
    screen.set_current_event(100); // большой шаг → сильное затухание
    let weight_after = screen.traces[0].weight;

    assert!(weight_after < weight_before);
}

#[test]
fn test_screen_cleanup_removes_old_traces() {
    // decay_rate=0.0 — вес не меняется, TRACE_ETERNAL не ставится,
    // трейс удаляется чисто по age > max_age
    let mut screen = Screen::new([256, 256, 256], 0.001, 0.0);
    let trace = DynamicTrace::new(0, 0, 0, 1.0, 1.0, 1, TraceSourceType::Token, 1, 0);
    screen.set_current_event(1);
    screen.write(&trace);

    screen.set_current_event(1000);
    screen.cleanup(10); // max_age=10, age=999 → удалён

    assert_eq!(screen.trace_count, 0);
    assert!(screen.traces.is_empty());
}

// ============================================================================
// UPO::compute
// ============================================================================

#[test]
fn test_upo_compute_returns_none_without_tokens() {
    let config = UPOConfig {
        domain_id: 1,
        min_tokens: 1,
        ..UPOConfig::default()
    };
    let mut upo = UPO::new(config);
    let result = upo.compute(&[], &[], 100);
    assert!(result.is_none());
}

#[test]
fn test_upo_compute_returns_some_with_active_token() {
    let config = UPOConfig {
        domain_id: 1,
        min_tokens: 1,
        ..UPOConfig::default()
    };
    let mut upo = UPO::new(config);

    let mut token = Token::new(1, 1, [0, 0, 0], 1);
    token.mass = 100;
    token.temperature = 200;

    let result = upo.compute(&[token], &[], 100);
    assert!(result.is_some());
    let trace = result.unwrap();
    assert_eq!(trace.source_type, TraceSourceType::Token as u8);
    assert_eq!(trace.last_update, 100);
}
