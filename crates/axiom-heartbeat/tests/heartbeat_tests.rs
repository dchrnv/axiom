use axiom_core::{EventPriority, EventType};
use axiom_frontier::{CausalFrontier, FrontierConfig};
use axiom_heartbeat::*;

#[test]
fn test_heartbeat_config_presets() {
    let weak = HeartbeatConfig::weak();
    assert_eq!(weak.interval, 10000);
    assert_eq!(weak.batch_size, 1);
    assert!(!weak.enable_gravity);

    let medium = HeartbeatConfig::medium();
    assert_eq!(medium.interval, 1024);
    assert_eq!(medium.batch_size, 10);
    assert!(medium.enable_gravity);

    let powerful = HeartbeatConfig::powerful();
    assert_eq!(powerful.interval, 256);
    assert_eq!(powerful.batch_size, 50);
    assert!(powerful.enable_thermodynamics);

    let disabled = HeartbeatConfig::disabled();
    assert_eq!(disabled.interval, u32::MAX);
    assert_eq!(disabled.batch_size, 0);
}

#[test]
fn test_heartbeat_generator_creation() {
    let generator = HeartbeatGenerator::new(1, 100);
    assert_eq!(generator.current_pulse(), 0);
    // domain_id is private, so we can't test it directly
}

#[test]
fn test_heartbeat_generation_by_event_count() {
    let mut generator = HeartbeatGenerator::new(1, 5);

    // Первые 4 события - нет пульса
    assert!(generator.on_event().is_none());
    assert!(generator.on_event().is_none());
    assert!(generator.on_event().is_none());
    assert!(generator.on_event().is_none());

    // 5-е событие - первый пульс
    assert_eq!(generator.on_event(), Some(1));
    assert_eq!(generator.current_pulse(), 1);

    // Ещё 5 событий - второй пульс
    for _ in 0..4 {
        assert!(generator.on_event().is_none());
    }
    assert_eq!(generator.on_event(), Some(2));
    assert_eq!(generator.current_pulse(), 2);
}

#[test]
fn test_heartbeat_determinism() {
    let mut gen1 = HeartbeatGenerator::new(1, 10);
    let mut gen2 = HeartbeatGenerator::new(1, 10);

    // Обрабатываем одинаковое количество событий
    for _ in 0..25 {
        let pulse1 = gen1.on_event();
        let pulse2 = gen2.on_event();
        assert_eq!(pulse1, pulse2);
    }

    assert_eq!(gen1.current_pulse(), gen2.current_pulse());
}

#[test]
fn test_heartbeat_event_creation() {
    let generator = HeartbeatGenerator::new(5, 100);
    let event = generator.create_heartbeat_event(1000, 42);

    assert_eq!(event.event_id, 1000);
    assert_eq!(event.domain_id, 5);
    assert_eq!(event.event_type, EventType::Heartbeat as u16);
    assert_eq!(event.pulse_id, 42);
    assert_eq!(event.priority, EventPriority::Low as u8);
}

#[test]
fn test_handle_heartbeat_tokens() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());
    let config = HeartbeatConfig::medium();

    handle_heartbeat(&mut frontier, 1, &config, 100, 50);

    // Должно добавить batch_size токенов
    assert_eq!(frontier.token_count(), config.batch_size);
}

#[test]
fn test_handle_heartbeat_connections() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());
    let mut config = HeartbeatConfig::medium();
    config.enable_connection_maintenance = true;

    handle_heartbeat(&mut frontier, 1, &config, 100, 50);

    assert_eq!(frontier.connection_count(), config.connection_batch_size);
}

#[test]
fn test_handle_heartbeat_deterministic_selection() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());
    let config = HeartbeatConfig {
        batch_size: 3,
        ..HeartbeatConfig::medium()
    };

    // Пульс 0 должен выбрать токены 0, 1, 2
    handle_heartbeat(&mut frontier, 0, &config, 10, 0);

    assert!(frontier.contains_token(0));
    assert!(frontier.contains_token(1));
    assert!(frontier.contains_token(2));

    frontier.clear();

    // Пульс 1 должен выбрать токены 3, 4, 5
    handle_heartbeat(&mut frontier, 1, &config, 10, 0);

    assert!(frontier.contains_token(3));
    assert!(frontier.contains_token(4));
    assert!(frontier.contains_token(5));
}

#[test]
fn test_heartbeat_wraparound() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());
    let config = HeartbeatConfig {
        batch_size: 5,
        ..HeartbeatConfig::medium()
    };

    // total_tokens = 10, batch_size = 5
    // Пульс 2 должен обернуть индексы: 10 % 10 = 0
    handle_heartbeat(&mut frontier, 2, &config, 10, 0);

    // Должны быть добавлены токены 0, 1, 2, 3, 4 (wraparound)
    assert!(frontier.contains_token(0));
    assert!(frontier.contains_token(1));
    assert!(frontier.contains_token(2));
}

#[test]
fn test_heartbeat_full_coverage() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());
    let config = HeartbeatConfig {
        batch_size: 3,
        ..HeartbeatConfig::medium()
    };

    let total_tokens: usize = 10;
    let pulses_needed = total_tokens.div_ceil(config.batch_size);

    // За ceil(10/3) = 4 пульса должны быть покрыты все токены
    for pulse in 1..=pulses_needed {
        handle_heartbeat(&mut frontier, pulse as u64, &config, total_tokens, 0);
    }

    // Проверяем что все токены были добавлены
    for i in 0..total_tokens {
        assert!(frontier.contains_token(i as u32), "Token {} not covered", i);
    }
}

#[test]
fn test_heartbeat_idle_state() {
    let generator = HeartbeatGenerator::new(1, 1000);

    // Если нет событий, нет пульсов
    assert_eq!(generator.current_pulse(), 0);

    // Heartbeat V2.0, раздел 11: no events → no heartbeat → idle
    // Система находится в idle состоянии
}

// ─── Тесты 13B: enable_internal_drive (Cognitive Depth V1.0) ─────────────────

#[test]
fn test_internal_drive_weak_disabled() {
    assert!(
        !HeartbeatConfig::weak().enable_internal_drive,
        "weak: Internal Drive отключён (слабое железо)"
    );
}

#[test]
fn test_internal_drive_medium_enabled() {
    assert!(
        HeartbeatConfig::medium().enable_internal_drive,
        "medium: Internal Drive включён"
    );
}

#[test]
fn test_internal_drive_powerful_enabled() {
    assert!(
        HeartbeatConfig::powerful().enable_internal_drive,
        "powerful: Internal Drive включён"
    );
}

#[test]
fn test_internal_drive_disabled_preset_off() {
    assert!(
        !HeartbeatConfig::disabled().enable_internal_drive,
        "disabled: Internal Drive выключен вместе с heartbeat"
    );
}
