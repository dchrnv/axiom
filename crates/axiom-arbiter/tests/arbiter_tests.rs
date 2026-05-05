use axiom_arbiter::*;
use axiom_core::Token;
use std::collections::HashMap;

fn create_test_token(id: u32, temp: u8) -> Token {
    let mut token = Token::new(id, 1, [0, 0, 0], 1);
    token.temperature = temp;
    token.mass = 100;
    token
}

#[test]
fn test_arbiter_creation() {
    let domains = HashMap::new();
    let com = COM::new();
    let arbiter = Arbiter::new(domains, com);

    assert!(!arbiter.is_ready());
}

#[test]
fn test_domain_registration() {
    let domains = HashMap::new();
    let com = COM::new();
    let mut arbiter = Arbiter::new(domains, com);

    assert!(arbiter.register_domain(0, 1000).is_ok()); // SUTRA
    assert!(arbiter.register_domain(9, 1009).is_ok()); // EXPERIENCE
    assert!(arbiter.register_domain(10, 1010).is_ok()); // MAYA

    // Регистрируем ASHTI 1-8
    for role in 1..=8 {
        assert!(arbiter.register_domain(role, 1000 + role as u16).is_ok());
    }

    assert!(arbiter.is_ready());
}

#[test]
fn test_invalid_role_registration() {
    let domains = HashMap::new();
    let com = COM::new();
    let mut arbiter = Arbiter::new(domains, com);

    assert!(arbiter.register_domain(11, 1011).is_err());
    assert!(arbiter.register_domain(255, 1255).is_err());
}

#[test]
fn test_routing_without_registration() {
    let domains = HashMap::new();
    let com = COM::new();
    let mut arbiter = Arbiter::new(domains, com);

    let token = create_test_token(1, 100);
    let result = arbiter.route_token(token, 0);

    // Should return error result
    assert_eq!(result.event_id, 0);
}

#[test]
fn test_token_comparison_identical() {
    let domains = HashMap::new();
    let com = COM::new();
    let arbiter = Arbiter::new(domains, com);

    let token = create_test_token(1, 100);
    assert!(arbiter.compare_tokens(&token, &token));
}

#[test]
fn test_token_comparison_similar() {
    let domains = HashMap::new();
    let com = COM::new();
    let arbiter = Arbiter::new(domains, com);

    let token1 = create_test_token(1, 100);
    let mut token2 = create_test_token(2, 105); // Slight temp difference
    token2.mass = 102; // Slight mass difference

    // Should still match (temp and mass within tolerance)
    assert!(arbiter.compare_tokens(&token1, &token2));
}

#[test]
fn test_token_comparison_different() {
    let domains = HashMap::new();
    let com = COM::new();
    let arbiter = Arbiter::new(domains, com);

    let token1 = create_test_token(1, 100);
    let mut token2 = create_test_token(2, 200); // Large temp difference
    token2.mass = 200; // Large mass difference
    token2.valence = -50; // Different valence

    // Should not match (too many differences)
    assert!(!arbiter.compare_tokens(&token1, &token2));
}

#[test]
fn test_euclidean_distance() {
    let domains = HashMap::new();
    let com = COM::new();
    let arbiter = Arbiter::new(domains, com);

    let pos1 = [0, 0, 0];
    let pos2 = [3, 4, 0];

    let dist = arbiter.euclidean_distance(&pos1, &pos2);
    assert!((dist - 5.0).abs() < 0.01); // 3-4-5 triangle
}

#[test]
fn test_cleanup_old_comparisons() {
    let domains = HashMap::new();
    let com = COM::new();
    let mut arbiter = Arbiter::new(domains, com);

    let token = create_test_token(1, 100);

    // Add some comparisons
    arbiter.pending_comparisons.insert(
        100,
        PendingComparison {
            input_pattern: token,
            reflex_prediction: None,
            ashti_results: vec![],
            consolidated_result: None,
            created_at: 100,
            trace_index: None,
        },
    );

    arbiter.pending_comparisons.insert(
        500,
        PendingComparison {
            input_pattern: token,
            reflex_prediction: None,
            ashti_results: vec![],
            consolidated_result: None,
            created_at: 500,
            trace_index: None,
        },
    );

    arbiter.pending_comparisons.insert(
        1000,
        PendingComparison {
            input_pattern: token,
            reflex_prediction: None,
            ashti_results: vec![],
            consolidated_result: None,
            created_at: 1000,
            trace_index: None,
        },
    );

    // Cleanup comparisons older than 600 events
    arbiter.cleanup_old_comparisons(1100, 600);

    assert_eq!(arbiter.pending_comparisons.len(), 2); // 500 and 1000 should remain
    assert!(!arbiter.pending_comparisons.contains_key(&100));
}

// ─── compare_tokens: per-domain tolerances (D-04+D-05) ───────────────────────

#[test]
fn test_compare_tokens_regression_default_tolerance() {
    // Без конфига домена → fallback на модульные константы → поведение не изменилось
    let domains = HashMap::new();
    let arbiter = Arbiter::new(domains, COM::new());

    let t1 = create_test_token(1, 100);
    let mut t2 = create_test_token(1, 108); // diff=8 < temp_tol=10
    t2.mass = 103; // diff=3 < mass_tol=5
    assert!(
        arbiter.compare_tokens(&t1, &t2),
        "должен совпадать при дефолтных порогах"
    );
}

#[test]
fn test_compare_tokens_strict_tolerance_from_config() {
    // Домен с tolerance=0 → только точное совпадение по temp/mass
    let cfg = axiom_config::DomainConfig {
        domain_id: 1,
        token_compare_temp_tolerance: 0,
        token_compare_mass_tolerance: 0,
        token_compare_valence_tolerance: 0,
        ..axiom_config::DomainConfig::default()
    };

    let mut domains = HashMap::new();
    domains.insert(1u16, cfg);
    let arbiter = Arbiter::new(domains, COM::new());

    let t1 = create_test_token(1, 100);
    let mut t2 = create_test_token(2, 101); // diff=1, но tolerance=0
    t2.domain_id = 1;
    // При tolerance=0 temp не совпадает → меньше 3 match → false
    assert!(
        !arbiter.compare_tokens(&t2, &t1),
        "tolerance=0: любое отклонение temperature не засчитывается"
    );
}

#[test]
fn test_compare_tokens_wide_tolerance_from_config() {
    // Домен с очень широким tolerance → большие различия всё равно совпадают
    let cfg = axiom_config::DomainConfig {
        domain_id: 1,
        token_compare_temp_tolerance: 100,
        token_compare_mass_tolerance: 100,
        token_compare_valence_tolerance: 100,
        ..axiom_config::DomainConfig::default()
    };

    let mut domains = HashMap::new();
    domains.insert(1u16, cfg);
    let arbiter = Arbiter::new(domains, COM::new());

    let t1 = create_test_token(1, 50);
    let mut t2 = create_test_token(2, 100); // diff=50 < tol=100
    t2.domain_id = 1;
    t2.mass = 150; // diff=50 < tol=100
    assert!(
        arbiter.compare_tokens(&t2, &t1),
        "широкий tolerance: большие различия должны засчитываться"
    );
}
