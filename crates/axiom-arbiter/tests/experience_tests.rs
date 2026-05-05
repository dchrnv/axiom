// Integration tests for axiom-arbiter Experience module
use axiom_arbiter::ExperienceModule as Experience;
use axiom_arbiter::ResonanceLevelEnum as ResonanceLevel;
use axiom_core::Token;

fn make_token(temp: u8, mass: u8) -> Token {
    let mut t = Token::new(1, 1, [0, 0, 0], 1);
    t.temperature = temp;
    t.mass = mass;
    t
}

// ============================================================
// resonance_search — no traces
// ============================================================

#[test]
fn test_empty_experience_returns_none() {
    let exp = Experience::new();
    let token = make_token(100, 100);
    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::None);
    assert!(result.trace.is_none());
}

// ============================================================
// resonance_search — with traces
// ============================================================

#[test]
fn test_high_weight_identical_token_returns_reflex() {
    let mut exp = Experience::new();
    // Default reflex_threshold = 128/255 ≈ 0.502
    // Identical token → similarity = 1.0, score = 1.0 * 0.9 = 0.9 >= 0.502
    let token = make_token(100, 100);
    exp.add_trace(token, 0.9, 1);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::Reflex);
    assert!(result.trace.is_some());
}

#[test]
fn test_low_weight_identical_token_returns_association() {
    let mut exp = Experience::new();
    // association_threshold = 64/255 ≈ 0.251
    // score = 1.0 * 0.4 = 0.4 >= 0.251 but < 0.502
    let token = make_token(100, 100);
    exp.add_trace(token, 0.4, 1);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::Association);
}

#[test]
fn test_very_low_weight_returns_none() {
    let mut exp = Experience::new();
    // score = 1.0 * 0.1 = 0.1 < 0.251
    let token = make_token(100, 100);
    exp.add_trace(token, 0.1, 1);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::None);
}

#[test]
fn test_different_token_returns_none() {
    let mut exp = Experience::new();
    // Trace: all fields at one extreme
    let mut trace_token = Token::new(1, 1, [i16::MIN, i16::MIN, i16::MIN], 1);
    trace_token.temperature = 0;
    trace_token.mass = 0;
    trace_token.valence = i8::MIN;
    exp.add_trace(trace_token, 0.9, 1);

    // Query: all fields at the opposite extreme
    let mut query = Token::new(1, 1, [i16::MAX, i16::MAX, i16::MAX], 1);
    query.temperature = 255;
    query.mass = 255;
    query.valence = i8::MAX;
    let result = exp.resonance_search(&query);
    assert_eq!(result.level, ResonanceLevel::None);
}

// ============================================================
// set_thresholds
// ============================================================

#[test]
fn test_custom_thresholds_affect_classification() {
    let mut exp = Experience::new();
    // Set very low thresholds — even weight=0.15 should trigger Association
    exp.set_thresholds(50, 30); // 50/255≈0.196 reflex, 30/255≈0.118 assoc
    let token = make_token(100, 100);
    exp.add_trace(token, 0.15, 1);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::Association);
}

#[test]
fn test_high_thresholds_block_reflex() {
    let mut exp = Experience::new();
    // Very high reflex threshold
    exp.set_thresholds(255, 200);
    let token = make_token(100, 100);
    exp.add_trace(token, 0.9, 1);

    // Even high weight can't exceed 1.0 score, but 255/255=1.0 means score must be >= 1.0
    // score = 1.0 * 0.9 = 0.9 < 1.0 → no reflex
    let result = exp.resonance_search(&token);
    assert_ne!(result.level, ResonanceLevel::Reflex);
}

// ============================================================
// add_trace / trace_count
// ============================================================

#[test]
fn test_add_trace_increases_count() {
    let mut exp = Experience::new();
    assert_eq!(exp.trace_count(), 0);
    exp.add_trace(make_token(1, 1), 0.5, 1);
    assert_eq!(exp.trace_count(), 1);
    exp.add_trace(make_token(2, 2), 0.5, 2);
    assert_eq!(exp.trace_count(), 2);
}

// ============================================================
// strengthen_trace / weaken_trace
// ============================================================

#[test]
fn test_strengthen_trace_raises_weight() {
    let mut exp = Experience::new();
    let token = make_token(100, 100);
    exp.add_trace(token, 0.4, 1);

    // After strengthening, weight should be above reflex threshold (0.502)
    exp.strengthen_trace(0, 0.2);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::Reflex);
}

#[test]
fn test_weaken_trace_lowers_weight() {
    let mut exp = Experience::new();
    let token = make_token(100, 100);
    exp.add_trace(token, 0.9, 1);

    // After weakening, score should drop below association threshold (0.251)
    exp.weaken_trace(0, 0.85);

    let result = exp.resonance_search(&token);
    assert_eq!(result.level, ResonanceLevel::None);
}

#[test]
fn test_strengthen_clamps_to_one() {
    let mut exp = Experience::new();
    exp.add_trace(make_token(100, 100), 0.9, 1);
    exp.strengthen_trace(0, 100.0); // Large delta
                                    // Should not panic, weight clamped to 1.0
    assert_eq!(exp.trace_count(), 1);
}

#[test]
fn test_weaken_clamps_to_zero() {
    let mut exp = Experience::new();
    exp.add_trace(make_token(100, 100), 0.5, 1);
    exp.weaken_trace(0, 100.0); // Large delta
                                // Should not panic, weight clamped to 0.0
    assert_eq!(exp.trace_count(), 1);
}
