use axiom_frontier::{CausalFrontier, FrontierConfig, FrontierEntity, FrontierState};

// ============================================================================
// EntityQueue tests
// ============================================================================

#[test]
fn test_entity_queue_push_pop() {
    use axiom_frontier::EntityQueue;

    let mut queue = EntityQueue::new(10);

    assert!(queue.push(1));
    assert!(queue.push(2));
    assert!(queue.push(3));

    assert_eq!(queue.len(), 3);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);
}

#[test]
fn test_entity_queue_deduplication() {
    use axiom_frontier::EntityQueue;

    let mut queue = EntityQueue::new(10);

    assert!(queue.push(1));
    assert!(!queue.push(1)); // duplicate
    assert!(queue.push(2));
    assert!(!queue.push(2)); // duplicate

    assert_eq!(queue.len(), 2);
}

#[test]
fn test_entity_queue_contains() {
    use axiom_frontier::EntityQueue;

    let mut queue = EntityQueue::new(10);

    queue.push(1);
    queue.push(2);

    assert!(queue.contains(1));
    assert!(queue.contains(2));
    assert!(!queue.contains(3));

    queue.pop();
    assert!(!queue.contains(1)); // visited cleared after pop
}

#[test]
fn test_entity_queue_clear() {
    use axiom_frontier::EntityQueue;

    let mut queue = EntityQueue::new(10);
    queue.push(1);
    queue.push(2);
    queue.clear();

    assert!(queue.is_empty());
    // Can re-add after clear
    assert!(queue.push(1));
}

// ============================================================================
// FrontierConfig presets
// ============================================================================

#[test]
fn test_frontier_config_presets() {
    let weak = FrontierConfig::weak();
    let medium = FrontierConfig::medium();
    let powerful = FrontierConfig::powerful();

    assert!(weak.max_frontier_size < medium.max_frontier_size);
    assert!(medium.max_frontier_size < powerful.max_frontier_size);

    assert!(weak.storm_threshold < medium.storm_threshold);
    assert!(medium.storm_threshold < powerful.storm_threshold);

    assert!(weak.max_events_per_cycle < medium.max_events_per_cycle);
    assert!(medium.max_events_per_cycle < powerful.max_events_per_cycle);
}

#[test]
fn test_frontier_config_default_is_medium() {
    let default = FrontierConfig::default();
    let medium = FrontierConfig::medium();

    assert_eq!(default.max_frontier_size, medium.max_frontier_size);
    assert_eq!(default.storm_threshold, medium.storm_threshold);
    assert_eq!(default.max_events_per_cycle, medium.max_events_per_cycle);
}

// ============================================================================
// CausalFrontier creation
// ============================================================================

#[test]
fn test_frontier_creation_default() {
    let frontier = CausalFrontier::default();

    assert_eq!(frontier.size(), 0);
    assert!(frontier.is_empty());
    assert_eq!(frontier.state(), FrontierState::Empty);
}

#[test]
fn test_frontier_creation_with_config() {
    let config = FrontierConfig::weak();
    let frontier = CausalFrontier::new(config);

    assert_eq!(frontier.size(), 0);
    assert!(frontier.is_empty());
    assert_eq!(frontier.max_frontier_size(), FrontierConfig::weak().max_frontier_size);
    assert_eq!(frontier.storm_threshold(), FrontierConfig::weak().storm_threshold);
}

// ============================================================================
// Push / pop — FrontierEntity API
// ============================================================================

#[test]
fn test_frontier_pop_returns_frontier_entity() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    frontier.begin_cycle();
    // Tokens have priority over connections
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(1)));
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(2)));
    assert_eq!(frontier.pop(), Some(FrontierEntity::Connection(10)));
    assert_eq!(frontier.pop(), None);
    frontier.end_cycle();
}

#[test]
fn test_frontier_token_priority_over_connections() {
    let mut frontier = CausalFrontier::default();

    // Add connection first, then token
    frontier.push_connection(99);
    frontier.push_token(1);

    frontier.begin_cycle();
    // Token should come first
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(1)));
    assert_eq!(frontier.pop(), Some(FrontierEntity::Connection(99)));
    frontier.end_cycle();
}

#[test]
fn test_frontier_push_pop_deduplication() {
    let mut frontier = CausalFrontier::default();

    assert!(frontier.push_token(5));
    assert!(!frontier.push_token(5)); // duplicate

    assert!(frontier.push_connection(10));
    assert!(!frontier.push_connection(10)); // duplicate

    assert_eq!(frontier.size(), 2);
}

#[test]
fn test_frontier_contains() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(1);
    frontier.push_connection(10);

    assert!(frontier.contains_token(1));
    assert!(frontier.contains_connection(10));
    assert!(!frontier.contains_token(2));
    assert!(!frontier.contains_connection(20));
}

// ============================================================================
// begin_cycle / end_cycle
// ============================================================================

#[test]
fn test_begin_end_cycle_resets_budget() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 3,
        storm_threshold: 500,
        enable_batch_events: true,
        batch_size: 0,
        token_capacity: 100,
        connection_capacity: 100,
    };
    let mut frontier = CausalFrontier::new(config);

    for i in 0..5 {
        frontier.push_token(i);
    }

    frontier.begin_cycle();
    // Budget = 3, only 3 pops succeed
    assert!(frontier.pop().is_some());
    assert!(frontier.pop().is_some());
    assert!(frontier.pop().is_some());
    assert!(frontier.is_budget_exhausted());
    assert!(frontier.pop().is_none()); // budget exhausted
    frontier.end_cycle();

    // New cycle — budget resets
    frontier.begin_cycle();
    assert!(!frontier.is_budget_exhausted());
    assert!(frontier.pop().is_some());
    frontier.end_cycle();
}

#[test]
fn test_end_cycle_updates_state() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(1);
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Active);

    frontier.begin_cycle();
    frontier.pop(); // drain
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Empty);
}

#[test]
fn test_frontier_growth_rate() {
    let mut frontier = CausalFrontier::default();

    // Cycle 1: add 3 tokens
    frontier.begin_cycle();
    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_token(3);
    frontier.end_cycle();
    let metrics = frontier.metrics();
    assert_eq!(metrics.frontier_size, 3);

    // Cycle 2: add 2 more (size goes from 3 to 5)
    frontier.begin_cycle();
    frontier.push_token(4);
    frontier.push_token(5);
    frontier.end_cycle();
    let metrics = frontier.metrics();
    assert_eq!(metrics.frontier_size, 5);
    assert_eq!(metrics.frontier_growth_rate, 2);
}

// ============================================================================
// StormMetrics
// ============================================================================

#[test]
fn test_storm_metrics_snapshot() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    frontier.begin_cycle();
    frontier.pop();
    let metrics = frontier.metrics();
    frontier.end_cycle();

    assert_eq!(metrics.frontier_size, 2); // 3 - 1 popped = 2 remaining
    assert_eq!(metrics.events_this_cycle, 1);
}

// ============================================================================
// Storm state transitions (V2.0 API)
// ============================================================================

#[test]
fn test_frontier_state_transitions_v2() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 1000,
        storm_threshold: 5,
        enable_batch_events: true,
        batch_size: 0,
        token_capacity: 100,
        connection_capacity: 100,
    };
    let mut frontier = CausalFrontier::new(config);

    // Empty → Active
    frontier.push_token(1);
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Active);

    // Active → Storm
    for i in 2..=10 {
        frontier.push_token(i);
    }
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Storm);
    assert!(frontier.is_storm());

    // Drain until below storm_threshold
    frontier.begin_cycle();
    while frontier.size() > 3 {
        frontier.pop();
    }
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Stabilizing);

    // Stabilizing → Empty
    frontier.begin_cycle();
    while frontier.pop().is_some() {}
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Empty);
}

// ============================================================================
// Memory limit
// ============================================================================

#[test]
fn test_memory_limit() {
    let config = FrontierConfig {
        max_frontier_size: 5,
        max_events_per_cycle: 1000,
        storm_threshold: 100,
        enable_batch_events: true,
        batch_size: 0,
        token_capacity: 100,
        connection_capacity: 100,
    };
    let mut frontier = CausalFrontier::new(config);

    for i in 0..5 {
        assert!(frontier.push_token(i));
    }

    // At capacity — next push rejected
    assert!(!frontier.push_token(10));
    assert_eq!(frontier.size(), 5);
}

#[test]
fn test_memory_usage() {
    let config = FrontierConfig {
        max_frontier_size: 100,
        max_events_per_cycle: 1000,
        storm_threshold: 50,
        enable_batch_events: true,
        batch_size: 0,
        token_capacity: 200,
        connection_capacity: 200,
    };
    let mut frontier = CausalFrontier::new(config);

    frontier.push_token(1);
    frontier.push_token(2);
    assert_eq!(frontier.memory_usage(), 2.0); // 2/100 * 100 = 2%

    for i in 3..=50 {
        frontier.push_token(i);
    }
    assert_eq!(frontier.memory_usage(), 50.0); // 50/100 * 100 = 50%
}

// ============================================================================
// Clear and getters
// ============================================================================

#[test]
fn test_frontier_clear() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    frontier.clear();

    assert_eq!(frontier.size(), 0);
    assert!(frontier.is_empty());
    assert_eq!(frontier.state(), FrontierState::Empty);

    // Can add again after clear
    assert!(frontier.push_token(1));
}

#[test]
fn test_frontier_getters() {
    let mut frontier = CausalFrontier::new(FrontierConfig::medium());

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    assert_eq!(frontier.token_count(), 2);
    assert_eq!(frontier.connection_count(), 1);
    assert_eq!(frontier.size(), 3);
    assert_eq!(frontier.max_frontier_size(), FrontierConfig::medium().max_frontier_size);
    assert_eq!(frontier.storm_threshold(), FrontierConfig::medium().storm_threshold);
}

#[test]
fn test_deterministic_fifo_order() {
    let mut frontier = CausalFrontier::default();

    frontier.push_token(10);
    frontier.push_token(20);
    frontier.push_token(30);

    frontier.begin_cycle();
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(10)));
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(20)));
    assert_eq!(frontier.pop(), Some(FrontierEntity::Token(30)));
    frontier.end_cycle();
}

// ============================================================================
// Batch Events — Storm mitigation
// ============================================================================

#[test]
fn test_batch_tokens_during_storm() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 1000,
        storm_threshold: 5,
        enable_batch_events: true,
        batch_size: 4,
        token_capacity: 200,
        connection_capacity: 200,
    };
    let mut frontier = CausalFrontier::new(config);

    // Переходим в Storm: добавляем > storm_threshold токенов
    for i in 0..10 {
        frontier.push_token(i);
    }
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Storm);

    // При Storm pop() должен вернуть BatchToken
    frontier.begin_cycle();
    let entity = frontier.pop().unwrap();
    assert!(matches!(entity, FrontierEntity::BatchToken(n) if n == 4));
    frontier.end_cycle();
}

#[test]
fn test_batch_size_reduces_pop_count() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 1000,
        storm_threshold: 3,
        enable_batch_events: true,
        batch_size: 5,
        token_capacity: 200,
        connection_capacity: 200,
    };
    let mut frontier = CausalFrontier::new(config);

    // Добавляем 10 токенов → Storm
    for i in 0..10 {
        frontier.push_token(i);
    }
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Storm);

    // При batch_size=5: первый pop → BatchToken(5), потребляет 1 budget
    frontier.begin_cycle();
    let first = frontier.pop().unwrap();
    assert!(matches!(first, FrontierEntity::BatchToken(5)));
    // frontier.events_this_cycle == 1, не 5
    assert_eq!(frontier.metrics().events_this_cycle, 1);
    frontier.end_cycle();
}

#[test]
fn test_no_batching_without_storm() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 1000,
        storm_threshold: 100,
        enable_batch_events: true,
        batch_size: 10,
        token_capacity: 200,
        connection_capacity: 200,
    };
    let mut frontier = CausalFrontier::new(config);

    // Добавляем 3 токена — не Storm
    for i in 0..3 {
        frontier.push_token(i);
    }
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Active);

    // Без шторма — обычные Token, не Batch
    frontier.begin_cycle();
    assert!(matches!(frontier.pop().unwrap(), FrontierEntity::Token(_)));
    frontier.end_cycle();
}

#[test]
fn test_batch_connections_during_storm() {
    let config = FrontierConfig {
        max_frontier_size: 1000,
        max_events_per_cycle: 1000,
        storm_threshold: 3,
        enable_batch_events: true,
        batch_size: 3,
        token_capacity: 200,
        connection_capacity: 200,
    };
    let mut frontier = CausalFrontier::new(config);

    // Только связи, без токенов
    for i in 0..8 {
        frontier.push_connection(i);
    }
    frontier.begin_cycle();
    frontier.end_cycle();
    assert_eq!(frontier.state(), FrontierState::Storm);

    frontier.begin_cycle();
    let entity = frontier.pop().unwrap();
    assert!(matches!(entity, FrontierEntity::BatchConnection(3)));
    frontier.end_cycle();
}
