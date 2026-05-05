// Этап 6: Адаптивные пороги — тесты Guardian::adapt_thresholds, adapt_domain_physics, dream_propose
use axiom_arbiter::Reflector;
use axiom_config::DomainConfig;
use axiom_core::Token;
use axiom_runtime::{AxiomEngine, CodexAction, Guardian, GuardianConfig, RoleStats};
use std::collections::HashMap;

fn make_token(x: i16, y: i16) -> Token {
    Token::new(1, 0, [x, y, 0], 1)
}

fn make_configs_for_roles(roles: &[u8]) -> HashMap<u16, DomainConfig> {
    let mut map = HashMap::new();
    for &role in roles {
        let mut cfg = DomainConfig::factory_execution(role as u16 + 100, 100);
        cfg.structural_role = role;
        map.insert(role as u16, cfg);
    }
    map
}

// ─── adapt_thresholds ────────────────────────────────────────────────────────

#[test]
fn test_adapt_thresholds_high_success_rate_decreases_threshold() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[1]);
    let initial = configs[&1].reflex_threshold;

    let stats = vec![RoleStats {
        role: 1,
        success_rate: 0.9,
        total_calls: 20,
    }];
    let updated = guardian.adapt_thresholds(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.contains(&1));
    assert!(
        configs[&1].reflex_threshold < initial,
        "high rate → lower threshold"
    );
}

#[test]
fn test_adapt_thresholds_low_success_rate_increases_threshold() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[2]);
    let initial = configs[&2].reflex_threshold;

    let stats = vec![RoleStats {
        role: 2,
        success_rate: 0.1,
        total_calls: 20,
    }];
    let updated = guardian.adapt_thresholds(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.contains(&2));
    assert!(
        configs[&2].reflex_threshold > initial,
        "low rate → higher threshold"
    );
}

#[test]
fn test_adapt_thresholds_mid_rate_no_change() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[3]);
    let initial = configs[&3].reflex_threshold;

    let stats = vec![RoleStats {
        role: 3,
        success_rate: 0.5,
        total_calls: 20,
    }];
    let updated = guardian.adapt_thresholds(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.is_empty(), "mid rate → no change");
    assert_eq!(configs[&3].reflex_threshold, initial);
}

#[test]
fn test_adapt_thresholds_skips_insufficient_data() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[4]);
    let initial = configs[&4].reflex_threshold;

    // total_calls < 10 → пропускаем
    let stats = vec![RoleStats {
        role: 4,
        success_rate: 0.95,
        total_calls: 5,
    }];
    let updated = guardian.adapt_thresholds(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.is_empty());
    assert_eq!(configs[&4].reflex_threshold, initial);
}

#[test]
fn test_adapt_thresholds_updates_guardian_stats() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[1, 2]);

    let stats = vec![
        RoleStats {
            role: 1,
            success_rate: 0.9,
            total_calls: 20,
        },
        RoleStats {
            role: 2,
            success_rate: 0.1,
            total_calls: 20,
        },
    ];
    guardian.adapt_thresholds(&stats, &mut configs, &GuardianConfig::default());

    assert!(guardian.stats().thresholds_adapted >= 2);
}

// ─── adapt_domain_physics ─────────────────────────────────────────────────────

#[test]
fn test_adapt_physics_high_success_cools_and_increases_freq() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[5]);
    let initial_temp = configs[&5].temperature;
    let initial_freq = configs[&5].resonance_freq;

    let stats = vec![RoleStats {
        role: 5,
        success_rate: 0.8,
        total_calls: 20,
    }];
    let updated = guardian.adapt_domain_physics(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.contains(&5));
    assert!(configs[&5].temperature < initial_temp, "high rate → cooler");
    assert!(
        configs[&5].resonance_freq > initial_freq,
        "high rate → faster resonance"
    );
}

#[test]
fn test_adapt_physics_low_success_heats_and_decreases_freq() {
    let mut guardian = Guardian::with_default_genome();
    let mut configs = make_configs_for_roles(&[6]);
    // Явно задаём resonance_freq > 10 чтобы насыщение не мешало
    configs.get_mut(&6).unwrap().resonance_freq = 100;
    let initial_temp = configs[&6].temperature;
    let initial_freq = configs[&6].resonance_freq;

    let stats = vec![RoleStats {
        role: 6,
        success_rate: 0.2,
        total_calls: 20,
    }];
    let updated = guardian.adapt_domain_physics(&stats, &mut configs, &GuardianConfig::default());

    assert!(updated.contains(&6));
    assert!(configs[&6].temperature > initial_temp, "low rate → hotter");
    assert!(
        configs[&6].resonance_freq < initial_freq,
        "low rate → slower resonance"
    );
}

// ─── dream_propose ────────────────────────────────────────────────────────────

#[test]
fn test_dream_propose_returns_add_rule_actions() {
    let mut guardian = Guardian::with_default_genome();
    let candidates = vec![make_token(1, 2), make_token(3, 4), make_token(5, 6)];
    let proposals = guardian.dream_propose(&candidates);

    assert_eq!(proposals.len(), 3);
    for p in &proposals {
        assert!(matches!(p, CodexAction::AddRule(_)));
    }
    assert_eq!(guardian.stats().dream_proposals, 3);
}

#[test]
fn test_dream_propose_caps_at_5() {
    let mut guardian = Guardian::with_default_genome();
    let candidates: Vec<Token> = (0..10).map(|i| make_token(i, 0)).collect();
    let proposals = guardian.dream_propose(&candidates);

    assert_eq!(proposals.len(), 5, "capped at 5 proposals per call");
}

#[test]
fn test_dream_propose_empty_candidates() {
    let mut guardian = Guardian::with_default_genome();
    let proposals = guardian.dream_propose(&[]);

    assert!(proposals.is_empty());
    assert_eq!(guardian.stats().dream_proposals, 0);
}

// ─── run_adaptation via AxiomEngine ──────────────────────────────────────────

#[test]
fn test_run_adaptation_no_stats_returns_empty() {
    let mut engine = AxiomEngine::new();
    // Нет статистики в Reflector → пустой результат
    let updated = engine.run_adaptation();
    assert!(updated.is_empty());
}

#[test]
fn test_dream_propose_engine_empty_experience() {
    let mut engine = AxiomEngine::new();
    // Нет кандидатов → пустые предложения
    let proposals = engine.dream_propose();
    assert!(proposals.is_empty());
}

// ─── DomainProfile новые методы ──────────────────────────────────────────────

#[test]
fn test_domain_profile_total_calls_and_success_rate() {
    use axiom_arbiter::Reflector;

    let mut reflector = Reflector::new();
    let shell: [u8; 8] = [100, 50, 0, 0, 0, 0, 0, 0]; // слои 1-2 активны

    // 8 успехов, 2 провала в роль 1
    for _ in 0..8 {
        reflector.record_domain(1, &shell, true);
    }
    for _ in 0..2 {
        reflector.record_domain(1, &shell, false);
    }

    let profile = reflector.domain_profile(1).unwrap();
    assert_eq!(profile.total_calls(), 20); // 10 попыток × 2 активных слоя
    let rate = profile.overall_success_rate();
    assert!((rate - 0.8).abs() < 0.01, "rate={}", rate);
}

#[test]
fn test_reflector_global_success_rate() {
    let mut reflector = Reflector::new();

    reflector.record_reflex(1, true);
    reflector.record_reflex(1, true);
    reflector.record_reflex(2, false);

    let rate = reflector.global_success_rate();
    assert!((rate - 2.0 / 3.0).abs() < 0.01, "rate={}", rate);
}
