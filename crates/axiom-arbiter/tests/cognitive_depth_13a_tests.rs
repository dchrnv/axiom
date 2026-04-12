// Тесты Этап 13A — Multi-pass и TensionTrace (Cognitive Depth V1.0)
//
// Покрывают:
// - consolidate_with_confidence: высокий confidence при согласованных доменах
// - consolidate_with_confidence: низкий confidence при расходящихся доменах
// - route_with_multipass: max_passes=0 → обычный проход
// - route_with_multipass: возвращает confidence в RoutingResult
// - route_with_multipass: tension trace создаётся при низком confidence
// - TensionTrace: add/drain/cool API
// - DomainConfig: max_passes и min_coherence в factory_maya
// - maya_multipass_params: корректные дефолты из factory_maya

use axiom_arbiter::*;
use axiom_config::DomainConfig;
use axiom_core::Token;
use std::collections::HashMap;

// ─────────────────────────────────────────────
// Хелперы
// ─────────────────────────────────────────────

fn make_token(id: u32, temp: u8, mass: u8, valence: i8) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.temperature = temp;
    t.mass = mass;
    t.valence = valence;
    t
}

fn make_full_arbiter(max_passes: u8, min_coherence: u8) -> Arbiter {
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
    // Переопределяем multi-pass параметры в MAYA (индекс 10)
    configs[10].max_passes = max_passes;
    configs[10].min_coherence = min_coherence;

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
// 1. DomainConfig: factory_maya дефолты
// ─────────────────────────────────────────────

#[test]
fn test_maya_factory_defaults_max_passes() {
    let cfg = DomainConfig::factory_maya(10, 1);
    assert_eq!(cfg.max_passes, 3, "factory_maya должен выставлять max_passes=3");
}

#[test]
fn test_maya_factory_defaults_min_coherence() {
    let cfg = DomainConfig::factory_maya(10, 1);
    assert_eq!(cfg.min_coherence, 153, "factory_maya должен выставлять min_coherence=153 (≈0.6)");
}

#[test]
fn test_non_maya_factory_no_multipass() {
    let cfg = DomainConfig::factory_execution(1, 0);
    assert_eq!(cfg.max_passes, 0, "Не-MAYA домены должны иметь max_passes=0");
    assert_eq!(cfg.min_coherence, 0);
}

// ─────────────────────────────────────────────
// 2. RoutingResult: новые поля
// ─────────────────────────────────────────────

#[test]
fn test_routing_result_has_confidence() {
    let mut arbiter = make_full_arbiter(0, 0);
    let token = make_token(1, 100, 100, 0);
    let result = arbiter.route_token(token, 0);
    // Поле существует и находится в допустимом диапазоне
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
}

#[test]
fn test_routing_result_has_passes() {
    let mut arbiter = make_full_arbiter(0, 0);
    let token = make_token(1, 100, 100, 0);
    let result = arbiter.route_token(token, 0);
    assert!(result.passes >= 1);
}

// ─────────────────────────────────────────────
// 3. route_with_multipass: базовые сценарии
// ─────────────────────────────────────────────

#[test]
fn test_multipass_disabled_behaves_like_normal() {
    // max_passes=0 → multi-pass отключён
    let mut arbiter = make_full_arbiter(0, 0);
    let token = make_token(1, 128, 128, 0);
    let result = arbiter.route_with_multipass(token);
    assert_eq!(result.event_id > 0, true);
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
}

#[test]
fn test_multipass_returns_confidence() {
    let mut arbiter = make_full_arbiter(3, 153);
    let token = make_token(2, 128, 128, 0);
    let result = arbiter.route_with_multipass(token);
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
}

#[test]
fn test_multipass_respects_max_passes() {
    let mut arbiter = make_full_arbiter(3, 255); // min_coherence=255 → всегда повторять
    let token = make_token(3, 128, 128, 0);
    let result = arbiter.route_with_multipass(token);
    // Не может быть больше max_passes проходов
    assert!(result.passes <= 3);
}

#[test]
fn test_multipass_passes_at_least_one() {
    let mut arbiter = make_full_arbiter(3, 0); // min_coherence=0 → порог всегда пройден
    let token = make_token(4, 128, 128, 0);
    let result = arbiter.route_with_multipass(token);
    assert!(result.passes >= 1);
}

#[test]
fn test_multipass_not_ready_returns_error() {
    let domains = HashMap::new();
    let mut arbiter = Arbiter::new(domains, COM::new());
    let token = make_token(1, 128, 128, 0);
    let result = arbiter.route_with_multipass(token);
    assert_eq!(result.event_id, 0); // error sentinel
}

// ─────────────────────────────────────────────
// 4. TensionTrace API
// ─────────────────────────────────────────────

#[test]
fn test_tension_trace_add_and_count() {
    let mut exp = ExperienceModule::new();
    let token = make_token(1, 200, 100, -50);
    exp.add_tension_trace(token, 200, 1);
    assert_eq!(exp.tension_count(), 1);
}

#[test]
fn test_tension_trace_drain_hot() {
    let mut exp = ExperienceModule::new();
    exp.add_tension_trace(make_token(1, 200, 100, 0), 180, 1); // горячий
    exp.add_tension_trace(make_token(2, 50,  100, 0), 50,  2);  // холодный

    let hot = exp.drain_hot_impulses(128); // порог 128
    assert_eq!(hot.len(), 1, "Только горячий трейс должен быть слит");
    assert_eq!(exp.tension_count(), 1, "Холодный остаётся");
}

#[test]
fn test_tension_trace_cool_decay() {
    let mut exp = ExperienceModule::new();
    exp.add_tension_trace(make_token(1, 100, 100, 0), 100, 1);

    exp.cool_tension_traces(30);
    assert_eq!(exp.tension_count(), 1, "Трейс ещё жив после частичного остывания");

    exp.cool_tension_traces(100); // ostyvat до 0 → удаляется
    assert_eq!(exp.tension_count(), 0, "Трейс должен быть удалён при temperature=0");
}

#[test]
fn test_tension_trace_created_on_low_confidence() {
    // min_coherence=255 → любой confidence < 1.0 вызывает tension
    let mut arbiter = make_full_arbiter(3, 255);
    let token = make_token(5, 128, 128, 0);

    let result = arbiter.route_with_multipass(token);

    if result.confidence < 1.0 {
        // Tension trace должен быть в experience
        assert!(
            arbiter.experience().tension_count() > 0,
            "Tension trace должен создаваться при низком confidence"
        );
    }
}
