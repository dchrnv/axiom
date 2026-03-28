use axiom_arbiter::{Reflector, ReflexStats, DomainProfile};

// ─── ReflexStats ─────────────────────────────────────────────────────────────

#[test]
fn test_reflex_stats_default() {
    let stats = ReflexStats::default();
    assert_eq!(stats.total(), 0);
    assert_eq!(stats.success_rate(), 0.0);
}

#[test]
fn test_reflex_stats_record_success() {
    let mut stats = ReflexStats::default();
    stats.record(true);
    stats.record(true);
    stats.record(false);
    assert_eq!(stats.success_count, 2);
    assert_eq!(stats.fail_count, 1);
    assert_eq!(stats.total(), 3);
}

#[test]
fn test_reflex_stats_success_rate() {
    let mut stats = ReflexStats::default();
    for _ in 0..7 { stats.record(true); }
    for _ in 0..3 { stats.record(false); }
    let rate = stats.success_rate();
    assert!((rate - 0.7).abs() < 0.01);
}

#[test]
fn test_reflex_stats_all_success() {
    let mut stats = ReflexStats::default();
    for _ in 0..10 { stats.record(true); }
    assert!((stats.success_rate() - 1.0).abs() < 0.001);
}

#[test]
fn test_reflex_stats_all_fail() {
    let mut stats = ReflexStats::default();
    for _ in 0..5 { stats.record(false); }
    assert_eq!(stats.success_rate(), 0.0);
}

// ─── DomainProfile ────────────────────────────────────────────────────────────

#[test]
fn test_domain_profile_default() {
    let profile = DomainProfile::default();
    for i in 0..8 {
        assert_eq!(profile.layer_success_rate(i), 0.0);
    }
    assert_eq!(profile.active_layers(), 0);
}

#[test]
fn test_domain_profile_record_active_layers() {
    let mut profile = DomainProfile::default();
    // Shell с активными слоями L1, L5
    let shell: [u8; 8] = [200, 0, 0, 0, 100, 0, 0, 0];
    profile.record(&shell, true);
    assert_eq!(profile.active_layers(), 2);
    assert!((profile.layer_success_rate(0) - 1.0).abs() < 0.001);
    assert!((profile.layer_success_rate(4) - 1.0).abs() < 0.001);
    assert_eq!(profile.layer_success_rate(1), 0.0);
}

#[test]
fn test_domain_profile_record_mixed() {
    let mut profile = DomainProfile::default();
    let shell: [u8; 8] = [100, 0, 0, 0, 0, 0, 0, 0];
    profile.record(&shell, true);
    profile.record(&shell, true);
    profile.record(&shell, false);
    let rate = profile.layer_success_rate(0);
    assert!((rate - 2.0 / 3.0).abs() < 0.01);
}

#[test]
fn test_domain_profile_zero_shell() {
    let mut profile = DomainProfile::default();
    let shell: [u8; 8] = [0; 8];
    profile.record(&shell, true);
    assert_eq!(profile.active_layers(), 0);
}

// ─── Reflector ────────────────────────────────────────────────────────────────

#[test]
fn test_reflector_new() {
    let r = Reflector::new();
    assert_eq!(r.tracked_patterns(), 0);
    assert!(r.get_stats(42).is_none());
}

#[test]
fn test_reflector_record_reflex() {
    let mut r = Reflector::new();
    r.record_reflex(0xABCD, true);
    r.record_reflex(0xABCD, false);
    let stats = r.get_stats(0xABCD).unwrap();
    assert_eq!(stats.success_count, 1);
    assert_eq!(stats.fail_count, 1);
}

#[test]
fn test_reflector_multiple_patterns() {
    let mut r = Reflector::new();
    r.record_reflex(1, true);
    r.record_reflex(2, false);
    r.record_reflex(3, true);
    assert_eq!(r.tracked_patterns(), 3);
}

#[test]
fn test_reflector_record_domain_valid() {
    let mut r = Reflector::new();
    let shell: [u8; 8] = [100, 0, 50, 0, 0, 0, 0, 0];
    r.record_domain(1, &shell, true);
    let profile = r.domain_profile(1).unwrap();
    assert!(profile.layer_success_rate(0) > 0.0);
}

#[test]
fn test_reflector_record_domain_invalid_role() {
    let mut r = Reflector::new();
    let shell: [u8; 8] = [100; 8];
    // Role 0 и 9 — невалидные для profile
    r.record_domain(0, &shell, true);
    r.record_domain(9, &shell, true);
    assert!(r.domain_profile(0).is_none());
    assert!(r.domain_profile(9).is_none());
}

#[test]
fn test_reflector_domain_profiles_all_roles() {
    let mut r = Reflector::new();
    let shell: [u8; 8] = [50; 8];
    for role in 1..=8 {
        r.record_domain(role, &shell, true);
        assert!(r.domain_profile(role).is_some());
    }
}
