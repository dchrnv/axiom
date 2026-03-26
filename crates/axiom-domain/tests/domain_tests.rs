// Integration tests for axiom-domain: Domain, DomainState
use axiom_domain::{Domain, DomainState, CapacityExceeded, EventGenerator};
use axiom_config::DomainConfig;
use axiom_core::{Token, Connection, EventType};
use axiom_heartbeat::HeartbeatConfig;
use axiom_frontier::FrontierState;

// --- Helper ---

fn make_token(sutra_id: u32, domain_id: u16) -> Token {
    Token::new(sutra_id, domain_id, [0, 0, 0], 0)
}

// ============================================================
// DomainConfig factory tests (smoke — полные в axiom-config)
// ============================================================

#[test]
fn test_factory_experience_basic() {
    let config = DomainConfig::factory_experience(9, 0);
    assert_eq!(config.domain_id, 9);
    assert_eq!(config.parent_domain_id, 0);
    assert_eq!(config.structural_role, 9);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_experience_physics() {
    let config = DomainConfig::factory_experience(9, 0);
    assert_eq!(config.field_size, [5000.0, 5000.0, 5000.0]);
    assert_eq!(config.gravity_strength, 0.5);
    assert_eq!(config.temperature, 300.0);
    assert_eq!(config.resonance_freq, 1000);
    assert_eq!(config.friction_coeff, 20);
    assert_eq!(config.viscosity, 200);
}

#[test]
fn test_factory_experience_capacities() {
    let config = DomainConfig::factory_experience(9, 0);
    assert_eq!(config.token_capacity, 100000);
    assert_eq!(config.connection_capacity, 50000);
    assert_eq!(config.permeability, 200);
    assert_eq!(config.membrane_state, 1); // SEMI
}

#[test]
fn test_all_factory_methods_valid() {
    let configs = vec![
        DomainConfig::factory_sutra(1),
        DomainConfig::factory_execution(1, 0),
        DomainConfig::factory_shadow(2, 0),
        DomainConfig::factory_codex(3, 1),
        DomainConfig::factory_map(4, 0),
        DomainConfig::factory_probe(5, 0),
        DomainConfig::factory_logic(6, 1),
        DomainConfig::factory_dream(7, 1),
        DomainConfig::factory_void(8, 0),
        DomainConfig::factory_experience(9, 1),
        DomainConfig::factory_maya(10, 1),
    ];
    for config in &configs {
        assert!(
            config.validate().is_ok(),
            "factory for role {} produced invalid config: {:?}",
            config.structural_role,
            config.validate()
        );
    }
}

#[test]
fn test_codex_domain_stability() {
    let config = DomainConfig::factory_codex(3, 1);
    assert_eq!(config.structural_role, 3);
    assert_eq!(config.temperature, 10.0);
    assert_eq!(config.viscosity, 250);
    assert_eq!(config.membrane_state, 2); // CLOSED
    assert_eq!(config.reflex_threshold, 0);
    assert_eq!(config.arbiter_flags, 0b00000000);
}

#[test]
fn test_probe_domain_exploration() {
    let config = DomainConfig::factory_probe(5, 0);
    assert_eq!(config.structural_role, 5);
    assert_eq!(config.temperature, 350.0);
    assert_eq!(config.resonance_freq, 800);
    assert_eq!(config.membrane_state, 0); // OPEN
    assert_eq!(config.reflex_threshold, 160);
    assert_eq!(config.reflex_cooldown, 1);
    assert_eq!(config.max_concurrent_hints, 5);
}

#[test]
fn test_void_domain_transformation() {
    let config = DomainConfig::factory_void(8, 0);
    assert_eq!(config.structural_role, 8);
    assert_eq!(config.temperature, 1000.0);
    assert_eq!(config.gravity_strength, 100.0);
    assert_eq!(config.friction_coeff, 200);
    assert_eq!(config.permeability, 255);
    assert_eq!(config.membrane_state, 0); // OPEN
    assert_eq!(config.reflex_threshold, 0);
    assert_eq!(config.arbiter_flags, 0b00000000);
    assert_eq!(config.token_capacity, 2000);
}

#[test]
fn test_shadow_domain_simulation() {
    let config = DomainConfig::factory_shadow(2, 0);
    assert_eq!(config.structural_role, 2);
    assert_eq!(config.temperature, 250.0);
    assert_eq!(config.gravity_strength, 5.0);
    assert_eq!(config.viscosity, 180);
    assert_eq!(config.membrane_state, 2); // CLOSED
    assert_eq!(config.reflex_threshold, 180);
    assert_eq!(config.arbiter_flags, 0b00010111);
}

#[test]
fn test_map_domain_facts() {
    let config = DomainConfig::factory_map(4, 0);
    assert_eq!(config.structural_role, 4);
    assert_eq!(config.temperature, 280.0);
    assert_eq!(config.gravity_strength, 15.0);
    assert_eq!(config.viscosity, 200);
    assert_eq!(config.membrane_state, 2); // CLOSED
    assert_eq!(config.reflex_threshold, 200);
    assert_eq!(config.arbiter_flags, 0b00011111);
}

#[test]
fn test_logic_domain_computation() {
    let config = DomainConfig::factory_logic(6, 1);
    assert_eq!(config.structural_role, 6);
    assert_eq!(config.temperature, 273.0);
    assert_eq!(config.gravity_strength, 9.81);
    assert_eq!(config.elasticity, 200);
    assert_eq!(config.friction_coeff, 25);
    assert_eq!(config.membrane_state, 3); // ADAPTIVE
    assert_eq!(config.reflex_threshold, 230);
    assert_eq!(config.association_threshold, 100);
}

#[test]
fn test_dream_domain_optimization() {
    let config = DomainConfig::factory_dream(7, 1);
    assert_eq!(config.structural_role, 7);
    assert_eq!(config.temperature, 500.0);
    assert_eq!(config.gravity_strength, 0.0);
    assert_eq!(config.quantum_noise, 200);
    assert_eq!(config.time_dilation, 50);
    assert_eq!(config.membrane_state, 0); // OPEN
    assert_eq!(config.reflex_threshold, 0);
    assert_eq!(config.association_threshold, 25);
}

#[test]
fn test_experience_domain_memory() {
    let config = DomainConfig::factory_experience(9, 1);
    assert_eq!(config.structural_role, 9);
    assert_eq!(config.temperature, 300.0);
    assert_eq!(config.gravity_strength, 0.5);
    assert_eq!(config.resonance_freq, 1000);
    assert_eq!(config.viscosity, 200);
    assert_eq!(config.token_capacity, 100000);
    assert_eq!(config.connection_capacity, 50000);
    assert_eq!(config.permeability, 200);
    assert_eq!(config.membrane_state, 1); // SEMI
    assert_eq!(config.reflex_threshold, 0);
    assert_eq!(config.arbiter_flags, 0b00000100);
}

// ============================================================
// DomainState tests
// ============================================================

#[test]
fn test_domain_state_new() {
    let config = DomainConfig::factory_logic(6, 1);
    let state = DomainState::new(&config);
    assert_eq!(state.token_count(), 0);
    assert_eq!(state.connection_count(), 0);
}

#[test]
fn test_domain_state_add_token() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut state = DomainState::new(&config);
    let token = make_token(1, 6);
    let idx = state.add_token(token).unwrap();
    assert_eq!(idx, 0);
    assert_eq!(state.token_count(), 1);
}

#[test]
fn test_domain_state_add_token_overflow() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.token_capacity = 2;
    let mut state = DomainState::new(&config);
    state.add_token(make_token(1, 6)).unwrap();
    state.add_token(make_token(2, 6)).unwrap();
    let result = state.add_token(make_token(3, 6));
    assert_eq!(result, Err(CapacityExceeded));
}

#[test]
fn test_domain_state_add_connection_overflow() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.connection_capacity = 1;
    let mut state = DomainState::new(&config);
    state.add_connection(Connection::default()).unwrap();
    let result = state.add_connection(Connection::default());
    assert_eq!(result, Err(CapacityExceeded));
}

// ============================================================
// Domain Runtime Tests
// ============================================================

#[test]
fn test_domain_creation() {
    let config = DomainConfig::factory_logic(6, 1);
    let domain = Domain::new(config);
    assert_eq!(domain.config.domain_id, 6);
    assert_eq!(domain.active_tokens, 0);
    assert_eq!(domain.active_connections, 0);
    assert!(domain.frontier.is_empty());
}

#[test]
fn test_domain_frontier_integration() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    assert!(domain.frontier.push_token(1));
    assert!(domain.frontier.push_token(2));
    assert_eq!(domain.frontier.size(), 2);

    domain.update_frontier_state();
    assert_eq!(domain.frontier.state(), FrontierState::Active);
}

#[test]
fn test_domain_capacity_limits() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.token_capacity = 10;
    config.connection_capacity = 5;

    let mut domain = Domain::new(config);
    domain.active_tokens = 10;
    domain.active_connections = 5;

    assert!(domain.is_at_capacity());
}

#[test]
fn test_domain_storm_threshold() {
    let mut config = DomainConfig::factory_experience(9, 0);
    config.token_capacity = 1000; // storm_threshold = 100

    let mut domain = Domain::new(config);

    for i in 0..150 {
        domain.frontier.push_token(i);
    }

    domain.update_frontier_state();
    assert_eq!(domain.frontier.state(), FrontierState::Storm);
}

#[test]
fn test_domain_frontier_memory_usage() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    domain.frontier.push_token(1);
    domain.frontier.push_token(2);
    domain.frontier.push_connection(10);

    let usage = domain.frontier_memory_usage();
    assert!(usage > 0.0);
    assert!(usage < 100.0);
}

#[test]
fn test_domain_isolation() {
    let config1 = DomainConfig::factory_logic(6, 1);
    let config2 = DomainConfig::factory_dream(7, 1);

    let mut domain1 = Domain::new(config1);
    let mut domain2 = Domain::new(config2);

    domain1.frontier.push_token(1);
    domain2.frontier.push_token(2);

    assert_eq!(domain1.frontier.size(), 1);
    assert_eq!(domain2.frontier.size(), 1);
    assert!(domain1.frontier.contains_token(1));
    assert!(!domain1.frontier.contains_token(2));
}

// ============================================================
// Heartbeat Integration Tests
// ============================================================

#[test]
fn test_domain_with_heartbeat() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig::medium();
    let domain = Domain::with_heartbeat(config, heartbeat_config);

    assert_eq!(domain.current_pulse(), 0);
    assert_eq!(domain.heartbeat_config.interval, 1024);
}

#[test]
fn test_domain_heartbeat_generation() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig {
        interval: 5,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    assert!(domain.on_event().is_none());
    assert!(domain.on_event().is_none());
    assert!(domain.on_event().is_none());
    assert!(domain.on_event().is_none());

    let pulse = domain.on_event();
    assert_eq!(pulse, Some(1));
    assert_eq!(domain.current_pulse(), 1);
}

#[test]
fn test_domain_handle_heartbeat() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.token_capacity = 100;

    let heartbeat_config = HeartbeatConfig {
        batch_size: 5,
        connection_batch_size: 2,
        enable_connection_maintenance: true,
        ..HeartbeatConfig::medium()
    };

    let mut domain = Domain::with_heartbeat(config, heartbeat_config);
    domain.active_tokens = 100;
    domain.active_connections = 50;

    domain.handle_heartbeat(1);

    assert_eq!(domain.frontier.token_count(), 5);
    assert_eq!(domain.frontier.connection_count(), 2);
}

#[test]
fn test_domain_heartbeat_isolation() {
    let config1 = DomainConfig::factory_logic(6, 1);
    let config2 = DomainConfig::factory_dream(7, 1);

    let heartbeat_config = HeartbeatConfig {
        interval: 5,
        ..HeartbeatConfig::medium()
    };

    let mut domain1 = Domain::with_heartbeat(config1, heartbeat_config);
    let mut domain2 = Domain::with_heartbeat(config2, heartbeat_config);

    for _ in 0..5 {
        domain1.on_event();
    }
    for _ in 0..2 {
        domain2.on_event();
    }

    assert_eq!(domain1.current_pulse(), 1);
    assert_eq!(domain2.current_pulse(), 0);
}

#[test]
fn test_domain_heartbeat_frontier_update() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.token_capacity = 50;

    let heartbeat_config = HeartbeatConfig {
        interval: 10,
        batch_size: 3,
        ..HeartbeatConfig::medium()
    };

    let mut domain = Domain::with_heartbeat(config, heartbeat_config);
    domain.active_tokens = 50;

    for _ in 0..10 {
        if let Some(pulse) = domain.on_event() {
            domain.handle_heartbeat(pulse);
        }
    }

    assert_eq!(domain.frontier.token_count(), 3);
    assert_eq!(domain.current_pulse(), 1);

    domain.update_frontier_state();
    assert_eq!(domain.frontier.state(), FrontierState::Active);
}

// ============================================================
// Frontier Processing Tests
// ============================================================

#[test]
fn test_process_frontier_basic() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig::medium();
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let tokens: Vec<Token> = (0..10).map(|i| make_token(i, 6)).collect();
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    domain.frontier.push_token(0);
    domain.frontier.push_token(1);
    domain.frontier.push_token(2);

    let _events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    // Frontier должен быть обработан (не проверяем пустоту событий без enable_*)
    assert_eq!(domain.frontier.token_count(), 0);
}

#[test]
fn test_process_frontier_decay() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig {
        enable_decay: true,
        enable_gravity: false,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let mut token = make_token(1, 6);
    token.valence = 10; // ненулевой valence → должен затухать
    // last_event_id = 0, event_generator.current_event_id = 10000 → age = 10000 > 1000

    let tokens = vec![token];
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(10000);

    domain.frontier.push_token(0);
    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    assert!(!events.is_empty());
    let has_decay = events
        .iter()
        .any(|e| e.event_type == EventType::TokenDecayed as u16);
    assert!(has_decay, "Expected TokenDecayed event");
}

#[test]
fn test_process_frontier_gravity() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.gravity_strength = 10.0;

    let heartbeat_config = HeartbeatConfig {
        enable_decay: false,
        enable_gravity: true,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let tokens = vec![make_token(1, 6)];
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    domain.frontier.push_token(0);
    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    assert!(!events.is_empty());
    let has_gravity = events
        .iter()
        .any(|e| e.event_type == EventType::GravityUpdate as u16);
    assert!(has_gravity, "Expected GravityUpdate event");
}

#[test]
fn test_process_frontier_connection_stress() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig {
        enable_decay: false,
        enable_gravity: false,
        enable_connection_maintenance: true,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let mut connection = Connection::default();
    connection.source_id = 1;
    connection.target_id = 2;
    connection.domain_id = 6;
    connection.current_stress = 1.0; // > 0.8 порога

    let tokens = vec![];
    let connections = vec![connection];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    domain.frontier.push_connection(0);
    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    assert!(!events.is_empty());
    let has_stress = events.iter().any(|e| {
        e.event_type == EventType::ConnectionWeakened as u16
            || e.event_type == EventType::ConnectionBroken as u16
    });
    assert!(has_stress, "Expected connection stress event");
}

#[test]
fn test_process_frontier_budget_limit() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig::medium();
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let tokens: Vec<Token> = (0u32..2000).map(|i| make_token(i, 6)).collect();
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    for i in 0..1500 {
        domain.frontier.push_token(i);
    }
    let initial_count = domain.frontier.token_count();

    let _events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    let processed = initial_count - domain.frontier.token_count();
    assert!(processed <= 1000, "Should respect budget limit");
}

#[test]
fn test_process_frontier_empty() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig::medium();
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let tokens = vec![];
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    assert!(events.is_empty());
    assert!(domain.frontier.is_empty());
}

#[test]
fn test_process_frontier_state_update() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig::medium();
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let tokens: Vec<Token> = (0u32..10).map(|i| make_token(i, 6)).collect();
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    for i in 0..5 {
        domain.frontier.push_token(i);
    }

    let _events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    let state = domain.frontier.state();
    assert!(
        state == FrontierState::Empty || state == FrontierState::Idle,
        "Expected Empty or Idle after processing, got {:?}",
        state
    );
}

// ============================================================
// Full Flow Integration Tests
// ============================================================

#[test]
fn test_full_heartbeat_to_event_flow() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.gravity_strength = 5.0;

    let heartbeat_config = HeartbeatConfig {
        interval: 5,
        batch_size: 2,
        enable_decay: true,
        enable_gravity: true,
        enable_connection_maintenance: false,
        ..HeartbeatConfig::medium()
    };

    let mut domain = Domain::with_heartbeat(config, heartbeat_config);
    domain.active_tokens = 10;

    let mut tokens: Vec<Token> = (0u32..10)
        .map(|i| {
            let mut t = make_token(i, 6);
            t.valence = 5;
            t
        })
        .collect();
    // last_event_id уже 0 из Token::new

    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(10000);

    for _ in 0..5 {
        if let Some(pulse) = domain.on_event() {
            domain.handle_heartbeat(pulse);
            event_generator.set_pulse_id(pulse);
            let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

            assert!(!events.is_empty(), "Expected events from frontier processing");
            for event in &events {
                assert_eq!(event.pulse_id, pulse);
            }
        }
    }

    assert_eq!(domain.current_pulse(), 1);
}

#[test]
fn test_full_flow_multiple_cycles() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.gravity_strength = 1.0;

    let heartbeat_config = HeartbeatConfig {
        interval: 3,
        batch_size: 2,
        enable_decay: false,
        enable_gravity: true,
        enable_connection_maintenance: false,
        ..HeartbeatConfig::medium()
    };

    let mut domain = Domain::with_heartbeat(config, heartbeat_config);
    domain.active_tokens = 10;

    let tokens: Vec<Token> = (0u32..10).map(|i| make_token(i, 6)).collect();
    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    let mut total_events = 0;

    for cycle in 0u64..10 {
        if let Some(pulse) = domain.on_event() {
            event_generator.set_pulse_id(pulse);
            domain.handle_heartbeat(pulse);

            let events = domain.process_frontier(&tokens, &connections, &mut event_generator);
            total_events += events.len();

            domain.frontier.reset_cycle();
        }

        event_generator.set_event_id(1000 + cycle * 10);
    }

    assert!(total_events > 0, "Expected events from multiple cycles");
    assert!(domain.current_pulse() >= 3);
}

// ============================================================
// SPACE V6.0 Integration Tests
// ============================================================

#[test]
fn test_domain_spatial_grid_initialization() {
    let config = DomainConfig::factory_logic(6, 1);
    let domain = Domain::new(config);
    assert_eq!(domain.spatial_grid.entry_count, 0);
    assert_eq!(domain.events_since_rebuild, 0);
}

#[test]
fn test_domain_rebuild_spatial_grid() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    let tokens: Vec<Token> = (0u32..10)
        .map(|i| {
            let mut t = make_token(i, 6);
            t.position = [i as i16 * 100, i as i16 * 50, 0];
            t
        })
        .collect();
    // Note: Token::new doesn't set position — we mutate it here
    // (Token is Copy so this works via make_token + mutation)

    domain.active_tokens = 10;
    domain.rebuild_spatial_grid(&tokens);

    assert_eq!(domain.spatial_grid.entry_count, 10);
    assert_eq!(domain.events_since_rebuild, 0);
}

#[test]
fn test_domain_should_rebuild_spatial_grid() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.rebuild_frequency = 10;

    let mut domain = Domain::new(config);

    assert!(!domain.should_rebuild_spatial_grid());

    for _ in 0..9 {
        domain.increment_events_since_rebuild();
    }
    assert!(!domain.should_rebuild_spatial_grid());

    domain.increment_events_since_rebuild();
    assert!(domain.should_rebuild_spatial_grid());
}

#[test]
fn test_domain_rebuild_frequency_disabled() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.rebuild_frequency = 0;

    let mut domain = Domain::new(config);
    for _ in 0..100 {
        domain.increment_events_since_rebuild();
    }
    assert!(!domain.should_rebuild_spatial_grid());
}

#[test]
fn test_domain_find_neighbors() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    let mut tokens: Vec<Token> = Vec::new();

    let mut t0 = make_token(0, 6);
    t0.position = [0, 0, 0];
    tokens.push(t0);

    let mut t1 = make_token(1, 6);
    t1.position = [100, 0, 0];
    tokens.push(t1);

    let mut t2 = make_token(2, 6);
    t2.position = [1000, 1000, 1000];
    tokens.push(t2);

    domain.active_tokens = 3;
    domain.rebuild_spatial_grid(&tokens);

    let neighbors = domain.find_neighbors(&tokens[0], 200, &tokens);

    assert!(neighbors.contains(&1), "Should find token 1");
    assert!(!neighbors.contains(&2), "Should not find token 2");
}

#[test]
fn test_domain_spatial_grid_rebuild_resets_counter() {
    let mut config = DomainConfig::factory_logic(6, 1);
    config.rebuild_frequency = 5;

    let mut domain = Domain::new(config);

    let tokens: Vec<Token> = (0u32..5)
        .map(|i| {
            let mut t = make_token(i, 6);
            t.position = [i as i16 * 100, 0, 0];
            t
        })
        .collect();

    domain.active_tokens = 5;

    for _ in 0..10 {
        domain.increment_events_since_rebuild();
    }
    assert_eq!(domain.events_since_rebuild, 10);

    domain.rebuild_spatial_grid(&tokens);
    assert_eq!(domain.events_since_rebuild, 0);
}

#[test]
fn test_domain_spatial_grid_with_empty_tokens() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    let tokens: Vec<Token> = Vec::new();
    domain.active_tokens = 0;
    domain.rebuild_spatial_grid(&tokens);

    assert_eq!(domain.spatial_grid.entry_count, 0);
}

#[test]
fn test_domain_find_neighbors_empty_grid() {
    let config = DomainConfig::factory_logic(6, 1);
    let domain = Domain::new(config);

    let token = make_token(0, 6);
    let tokens = vec![token];

    let neighbors = domain.find_neighbors(&tokens[0], 100, &tokens);
    assert!(neighbors.is_empty());
}

#[test]
fn test_domain_spatial_grid_multiple_rebuilds() {
    let config = DomainConfig::factory_logic(6, 1);
    let mut domain = Domain::new(config);

    let mut tokens: Vec<Token> = (0u32..5)
        .map(|i| {
            let mut t = make_token(i, 6);
            t.position = [i as i16 * 100, 0, 0];
            t
        })
        .collect();

    domain.active_tokens = 5;
    domain.rebuild_spatial_grid(&tokens);
    assert_eq!(domain.spatial_grid.entry_count, 5);

    for t in &mut tokens {
        t.position[0] += 50;
    }
    domain.rebuild_spatial_grid(&tokens);
    assert_eq!(domain.spatial_grid.entry_count, 5);

    let neighbors = domain.find_neighbors(&tokens[0], 200, &tokens);
    assert!(neighbors.contains(&1), "Should find neighbor after rebuild");
}

// ============================================================
// SPACE V6.0 + Causal Frontier Integration Tests
// ============================================================

#[test]
fn test_process_frontier_with_spatial_collision_detection() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig {
        enable_decay: false,
        enable_gravity: false,
        enable_spatial_collision: true,
        enable_connection_maintenance: false,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let mut t0 = make_token(100, 6);
    t0.position = [0, 0, 0];

    let mut t1 = make_token(101, 6);
    t1.position = [50, 0, 0]; // расстояние 50 < DEFAULT_COLLISION_RADIUS=100

    let tokens = vec![t0, t1];
    domain.active_tokens = 2;
    domain.rebuild_spatial_grid(&tokens);
    domain.frontier.push_token(0);

    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    let collision_events: Vec<_> = events
        .iter()
        .filter(|e| e.event_type == EventType::TokenCollision as u16)
        .collect();

    assert!(!collision_events.is_empty(), "Expected collision event");

    let c = collision_events[0];
    assert!(
        (c.source_id == 100 && c.target_id == 101)
            || (c.source_id == 101 && c.target_id == 100),
        "Collision should be between token 100 and 101"
    );
}

#[test]
fn test_process_frontier_no_collision_when_far_apart() {
    let config = DomainConfig::factory_logic(6, 1);
    let heartbeat_config = HeartbeatConfig {
        enable_decay: false,
        enable_gravity: false,
        enable_spatial_collision: true,
        enable_connection_maintenance: false,
        ..HeartbeatConfig::medium()
    };
    let mut domain = Domain::with_heartbeat(config, heartbeat_config);

    let mut t0 = make_token(100, 6);
    t0.position = [0, 0, 0];

    let mut t1 = make_token(101, 6);
    t1.position = [500, 0, 0]; // расстояние 500 > 100

    let tokens = vec![t0, t1];
    domain.active_tokens = 2;
    domain.rebuild_spatial_grid(&tokens);
    domain.frontier.push_token(0);

    let connections = vec![];
    let mut event_generator = EventGenerator::new();
    event_generator.set_event_id(1000);

    let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

    let has_collision = events
        .iter()
        .any(|e| e.event_type == EventType::TokenCollision as u16);
    assert!(!has_collision, "Should not have collision when far apart");
}
