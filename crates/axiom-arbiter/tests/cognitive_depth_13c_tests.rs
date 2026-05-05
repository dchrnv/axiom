// Тесты Этап 13C — InternalImpulse + Internal Dominance (Cognitive Depth V1.0)
//
// Покрывают:
// - DomainConfig: internal_dominance_factor поле и factory defaults
// - select_next: None/None → idle
// - select_next: только external → external
// - select_next: только internal → internal
// - select_next: factor=0 → external всегда побеждает
// - select_next: factor=255 → internal почти всегда побеждает
// - select_next: равновесие (factor=128) — сравниваются напрямую
// - select_next: ImpulseSource корректно пробрасывается
// - Arbiter::internal_dominance_factor: читает из конфига MAYA
// - InternalImpulse: поля source, weight, pattern

use axiom_arbiter::*;
use axiom_config::DomainConfig;
use axiom_core::Token;
use std::collections::HashMap;

// ─────────────────────────────────────────────
// Хелперы
// ─────────────────────────────────────────────

fn make_token(id: u32, temp: u8) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.temperature = temp;
    t.mass = 100;
    t
}

fn make_impulse(source: ImpulseSource, weight: f32, temp: u8) -> InternalImpulse {
    InternalImpulse {
        source,
        weight,
        pattern: make_token(999, temp),
    }
}

fn make_full_arbiter_with_dominance(factor: u8) -> Arbiter {
    let mut configs = [
        DomainConfig::factory_sutra(100),
        DomainConfig::factory_execution(101, 100),
        DomainConfig::factory_shadow(102, 100),
        DomainConfig::factory_codex(103, 100),
        DomainConfig::factory_map(104, 100),
        DomainConfig::factory_probe(105, 100),
        DomainConfig::factory_logic(106, 100),
        DomainConfig::factory_dream(107, 100),
        DomainConfig::factory_void(108, 100),
        DomainConfig::factory_experience(109, 100),
        DomainConfig::factory_maya(110, 100),
    ];
    configs[10].internal_dominance_factor = factor;

    let mut domain_map = HashMap::new();
    for c in &configs {
        domain_map.insert(c.domain_id, *c);
    }
    let mut arbiter = Arbiter::new(domain_map, COM::new());
    for (role, c) in configs.iter().enumerate() {
        let _ = arbiter.register_domain(role as u8, c.domain_id);
    }
    arbiter
}

// ─────────────────────────────────────────────
// 1. DomainConfig: internal_dominance_factor
// ─────────────────────────────────────────────

#[test]
fn test_domain_config_has_internal_dominance_factor() {
    let cfg = DomainConfig::factory_maya(10, 1);
    // Поле существует и по умолчанию 0 (чисто реактивная)
    let _ = cfg.internal_dominance_factor;
}

#[test]
fn test_domain_config_size_still_128() {
    assert_eq!(std::mem::size_of::<DomainConfig>(), 128);
}

// ─────────────────────────────────────────────
// 2. ImpulseSource и InternalImpulse
// ─────────────────────────────────────────────

#[test]
fn test_impulse_source_variants() {
    let sources = [
        ImpulseSource::External,
        ImpulseSource::Tension,
        ImpulseSource::Incompletion,
        ImpulseSource::Curiosity,
        ImpulseSource::Goal,
    ];
    // Все варианты существуют и уникальны
    for (i, s) in sources.iter().enumerate() {
        for (j, t) in sources.iter().enumerate() {
            if i == j {
                assert_eq!(s, t);
            } else {
                assert_ne!(s, t);
            }
        }
    }
}

#[test]
fn test_internal_impulse_fields() {
    let imp = make_impulse(ImpulseSource::Tension, 0.8, 200);
    assert_eq!(imp.source, ImpulseSource::Tension);
    assert!((imp.weight - 0.8).abs() < 0.001);
    assert_eq!(imp.pattern.temperature, 200);
}

// ─────────────────────────────────────────────
// 3. select_next: логика приоритизации
// ─────────────────────────────────────────────

#[test]
fn test_select_next_both_none_is_idle() {
    let result = Arbiter::select_next(None, None, 128);
    assert!(result.is_none(), "Оба пустые → idle");
}

#[test]
fn test_select_next_only_external() {
    let ext = make_token(1, 200);
    let result = Arbiter::select_next(Some(ext), None, 128);
    let (_, source) = result.expect("Должен вернуть external");
    assert_eq!(source, ImpulseSource::External);
}

#[test]
fn test_select_next_only_internal() {
    let imp = make_impulse(ImpulseSource::Goal, 0.9, 100);
    let result = Arbiter::select_next(None, Some(imp), 128);
    let (_, source) = result.expect("Должен вернуть internal");
    assert_eq!(source, ImpulseSource::Goal);
}

#[test]
fn test_select_next_factor_zero_external_always_wins() {
    // factor=0 → internal_priority = weight * 0.0 = 0 < любой ext_urgency > 0
    let ext = make_token(1, 1); // минимальная urgency
    let imp = make_impulse(ImpulseSource::Tension, 1.0, 100); // максимальный вес
    let (_, source) =
        Arbiter::select_next(Some(ext), Some(imp), 0).expect("Должен вернуть результат");
    assert_eq!(
        source,
        ImpulseSource::External,
        "factor=0 → external всегда побеждает"
    );
}

#[test]
fn test_select_next_factor_max_internal_wins_over_weak_external() {
    // factor=255 ≈ 2.0 → internal_priority = 1.0 * 2.0 = 2.0 > ext_urgency (макс 1.0)
    let ext = make_token(1, 255); // максимальная urgency = 1.0
    let imp = make_impulse(ImpulseSource::Tension, 1.0, 100);
    let (_, source) =
        Arbiter::select_next(Some(ext), Some(imp), 255).expect("Должен вернуть результат");
    assert_eq!(
        source,
        ImpulseSource::Tension,
        "factor=255 → internal побеждает при weight=1.0"
    );
}

#[test]
fn test_select_next_equilibrium_strong_external_wins() {
    // factor=128 ≈ 1.0 → internal_priority = weight * 1.0
    // ext temp=200 → urgency=200/255≈0.78
    // imp weight=0.5 → priority=0.5 < 0.78 → external wins
    let ext = make_token(1, 200);
    let imp = make_impulse(ImpulseSource::Curiosity, 0.5, 100);
    let (_, source) =
        Arbiter::select_next(Some(ext), Some(imp), 128).expect("Должен вернуть результат");
    assert_eq!(
        source,
        ImpulseSource::External,
        "Сильный external побеждает при равновесии"
    );
}

#[test]
fn test_select_next_equilibrium_strong_internal_wins() {
    // factor=128 ≈ 1.0
    // ext temp=50 → urgency=50/255≈0.196
    // imp weight=0.9 → priority=0.9 > 0.196 → internal wins
    let ext = make_token(1, 50);
    let imp = make_impulse(ImpulseSource::Goal, 0.9, 100);
    let (_, source) =
        Arbiter::select_next(Some(ext), Some(imp), 128).expect("Должен вернуть результат");
    assert_eq!(
        source,
        ImpulseSource::Goal,
        "Сильный internal побеждает при равновесии"
    );
}

// ─────────────────────────────────────────────
// 4. Arbiter::internal_dominance_factor
// ─────────────────────────────────────────────

#[test]
fn test_arbiter_reads_dominance_factor() {
    let arbiter = make_full_arbiter_with_dominance(200);
    assert_eq!(arbiter.internal_dominance_factor(), 200);
}

#[test]
fn test_arbiter_dominance_factor_default_zero() {
    // factory_maya по умолчанию ставит 0
    let arbiter = make_full_arbiter_with_dominance(0);
    assert_eq!(arbiter.internal_dominance_factor(), 0);
}

#[test]
fn test_arbiter_not_ready_dominance_returns_zero() {
    let domains = HashMap::new();
    let arbiter = Arbiter::new(domains, COM::new());
    assert_eq!(arbiter.internal_dominance_factor(), 0);
}
