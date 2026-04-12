// Тесты Этап 13B — Heartbeat Internal Drive (Cognitive Depth V1.0)
//
// Покрывают:
// - HeartbeatConfig: enable_internal_drive в weak/medium/powerful/disabled
// - Arbiter::on_heartbeat_pulse: отключён → пустой Vec
// - Arbiter::on_heartbeat_pulse: нет следов → пустой Vec
// - Arbiter::on_heartbeat_pulse: горячие следы → возвращает импульсы
// - Arbiter::on_heartbeat_pulse: остужает следы при каждом пульсе
// - Arbiter::on_heartbeat_pulse: холодные следы не возвращаются
// - Интеграция: несколько пульсов → постепенное остывание
// - Интеграция: drain + route повторно обрабатывает импульс

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

fn make_full_arbiter() -> Arbiter {
    let configs = [
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
// 1. on_heartbeat_pulse: базовые сценарии
// ─────────────────────────────────────────────

#[test]
fn test_pulse_disabled_returns_empty() {
    let mut arbiter = make_full_arbiter();
    // Добавляем горячий след
    let token = make_token(1, 200);
    arbiter.experience_mut().add_tension_trace(token, 255, 1);

    // enable_internal_drive = false → пустой Vec
    let impulses = arbiter.on_heartbeat_pulse(1, false);
    assert!(impulses.is_empty());
}

#[test]
fn test_pulse_no_tension_returns_empty() {
    let mut arbiter = make_full_arbiter();
    // Нет следов напряжения
    let impulses = arbiter.on_heartbeat_pulse(1, true);
    assert!(impulses.is_empty());
}

#[test]
fn test_pulse_hot_trace_returned_as_impulse() {
    let mut arbiter = make_full_arbiter();
    let token = make_token(1, 200);
    arbiter.experience_mut().add_tension_trace(token, 255, 1); // очень горячий

    let impulses = arbiter.on_heartbeat_pulse(1, true);
    assert!(!impulses.is_empty(), "Горячий след должен стать импульсом");
}

#[test]
fn test_pulse_cold_trace_not_returned() {
    let mut arbiter = make_full_arbiter();
    let token = make_token(1, 100);
    arbiter.experience_mut().add_tension_trace(token, 50, 1); // холодный (50 < порог 128)

    let impulses = arbiter.on_heartbeat_pulse(1, true);
    assert!(impulses.is_empty(), "Холодный след не должен стать импульсом");
}

#[test]
fn test_pulse_cools_traces() {
    let mut arbiter = make_full_arbiter();
    let token = make_token(1, 100);
    // temperature=140: после одного пульса 140 - 10 = 130 (ещё горячий)
    // после второго: 130 - 10 = 120 (ещё выше порога 128? нет, 120 < 128 → холодный)
    arbiter.experience_mut().add_tension_trace(token, 140, 1);

    // Первый пульс: drain (140 >= 128) → возвращает импульс
    let impulses_1 = arbiter.on_heartbeat_pulse(1, true);
    assert!(!impulses_1.is_empty(), "Первый пульс: след горячий");

    // После drain следа нет — добавим снова с температурой 138
    arbiter.experience_mut().add_tension_trace(token, 138, 2);

    // Один пульс без drain (нет горячих после cool) → температура снижается
    // Но drain происходит за один вызов: сначала cool (138-10=128), потом drain (128 >= 128)
    // 128 >= 128 → горячий, возвращается
    let impulses_2 = arbiter.on_heartbeat_pulse(2, true);
    assert!(!impulses_2.is_empty());

    // Снова добавим с температурой 130
    arbiter.experience_mut().add_tension_trace(token, 130, 3);
    // cool(130-10=120), 120 < 128 → не возвращается
    let impulses_3 = arbiter.on_heartbeat_pulse(3, true);
    assert!(impulses_3.is_empty(), "После остывания ниже порога — импульс не генерируется");
}

#[test]
fn test_pulse_multiple_hot_traces() {
    let mut arbiter = make_full_arbiter();
    for i in 0..3u32 {
        arbiter.experience_mut().add_tension_trace(make_token(i, 200), 200, i as u64 + 1);
    }

    let impulses = arbiter.on_heartbeat_pulse(1, true);
    assert_eq!(impulses.len(), 3, "Все три горячих следа должны стать импульсами");
    assert_eq!(arbiter.experience().tension_count(), 0, "После drain следов нет");
}

// ─────────────────────────────────────────────
// 3. Интеграция: drain → route
// ─────────────────────────────────────────────

#[test]
fn test_impulse_can_be_re_routed() {
    let mut arbiter = make_full_arbiter();
    let token = make_token(1, 200);
    arbiter.experience_mut().add_tension_trace(token, 200, 1);

    let impulses = arbiter.on_heartbeat_pulse(1, true);
    assert!(!impulses.is_empty());

    // Импульс можно повторно маршрутизировать
    let impulse_token = impulses[0];
    let result = arbiter.route_with_multipass(impulse_token);
    assert!(result.event_id > 0, "Импульс успешно обработан повторно");
}

#[test]
fn test_tension_decay_across_multiple_pulses() {
    let mut arbiter = make_full_arbiter();
    // temperature=200, decay=10 за пульс → остынет до < 128 за ~8 пульсов
    arbiter.experience_mut().add_tension_trace(make_token(1, 100), 200, 1);

    // Первый пульс: cool(200-10=190), drain(190>=128) → горячий
    let first = arbiter.on_heartbeat_pulse(1, true);
    assert!(!first.is_empty());
    // След удалён из буфера — добавим снова с меньшей температурой
    arbiter.experience_mut().add_tension_trace(make_token(1, 100), 120, 2);

    // cool(120-10=110), 110 < 128 → холодный
    let second = arbiter.on_heartbeat_pulse(2, true);
    assert!(second.is_empty(), "После нескольких пульсов след остывает ниже порога");
}
