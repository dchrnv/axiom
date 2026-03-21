use axiom_frontier::{CausalFrontier, FrontierState};

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
    assert!(!queue.push(1)); // Дубликат
    assert!(queue.push(2));
    assert!(!queue.push(2)); // Дубликат

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
    assert!(!queue.contains(1)); // После pop visited сбрасывается
}

#[test]
fn test_frontier_creation() {
    let frontier = CausalFrontier::new();

    assert_eq!(frontier.size(), 0);
    assert!(frontier.is_empty());
    assert_eq!(frontier.state(), FrontierState::Empty);
}

#[test]
fn test_frontier_push_pop_tokens() {
    let mut frontier = CausalFrontier::new();

    assert!(frontier.push_token(1));
    assert!(frontier.push_token(2));
    assert!(frontier.push_token(3));

    assert_eq!(frontier.size(), 3);
    assert_eq!(frontier.pop_token(), Some(1));
    assert_eq!(frontier.pop_token(), Some(2));
    assert_eq!(frontier.pop_token(), Some(3));
    assert_eq!(frontier.pop_token(), None);
}

#[test]
fn test_frontier_push_pop_connections() {
    let mut frontier = CausalFrontier::new();

    assert!(frontier.push_connection(10));
    assert!(frontier.push_connection(20));

    assert_eq!(frontier.size(), 2);
    assert_eq!(frontier.pop_connection(), Some(10));
    assert_eq!(frontier.pop_connection(), Some(20));
}

#[test]
fn test_frontier_mixed_entities() {
    let mut frontier = CausalFrontier::new();

    frontier.push_token(1);
    frontier.push_connection(10);
    frontier.push_token(2);

    assert_eq!(frontier.size(), 3);
    assert!(frontier.contains_token(1));
    assert!(frontier.contains_connection(10));
}

#[test]
fn test_frontier_state_transitions() {
    let mut frontier = CausalFrontier::with_config(5, 100, 10);

    // Empty → Active
    frontier.push_token(1);
    frontier.update_state();
    assert_eq!(frontier.state(), FrontierState::Active);

    // Active → Storm
    for i in 2..=10 {
        frontier.push_token(i);
    }
    frontier.update_state();
    assert_eq!(frontier.state(), FrontierState::Storm);

    // Storm → Stabilized
    while frontier.size() > 2 {
        frontier.pop_token();
    }
    frontier.update_state();
    assert_eq!(frontier.state(), FrontierState::Stabilized);

    // Stabilized → Idle
    frontier.clear();
    frontier.update_state();
    assert_eq!(frontier.state(), FrontierState::Idle);
}

#[test]
fn test_causal_budget() {
    let mut frontier = CausalFrontier::with_config(100, 1000, 5);

    assert!(!frontier.is_budget_exhausted());

    for _ in 0..5 {
        frontier.increment_processed();
    }

    assert!(frontier.is_budget_exhausted());

    frontier.reset_cycle();
    assert!(!frontier.is_budget_exhausted());
}

#[test]
fn test_memory_limit() {
    let mut frontier = CausalFrontier::with_config(10, 5, 100);

    // Добавляем до лимита
    for i in 0..5 {
        assert!(frontier.push_token(i));
    }

    // Превышение лимита
    assert!(!frontier.push_token(10));
    assert_eq!(frontier.size(), 5);
}

#[test]
fn test_memory_usage() {
    let mut frontier = CausalFrontier::with_config(10, 100, 10);

    frontier.push_token(1);
    frontier.push_token(2);

    assert_eq!(frontier.memory_usage(), 2.0); // 2/100 * 100 = 2%

    for i in 3..=50 {
        frontier.push_token(i);
    }

    assert_eq!(frontier.memory_usage(), 50.0); // 50/100 * 100 = 50%
}

#[test]
fn test_frontier_clear() {
    let mut frontier = CausalFrontier::new();

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    assert_eq!(frontier.size(), 3);

    frontier.clear();

    assert_eq!(frontier.size(), 0);
    assert!(frontier.is_empty());
    assert_eq!(frontier.state(), FrontierState::Empty);
}

#[test]
fn test_deterministic_order() {
    let mut frontier = CausalFrontier::new();

    // Добавляем в определённом порядке
    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_token(3);

    // Извлекаем в том же порядке (FIFO)
    assert_eq!(frontier.pop_token(), Some(1));
    assert_eq!(frontier.pop_token(), Some(2));
    assert_eq!(frontier.pop_token(), Some(3));
}

#[test]
fn test_storm_detection_helpers() {
    let mut frontier = CausalFrontier::with_config(10, 100, 10);

    // Добавляем до порога storm
    for i in 0..15 {
        frontier.push_token(i);
    }

    frontier.update_state();

    assert!(frontier.is_storm());
    assert_eq!(frontier.storm_threshold(), 10);
    assert_eq!(frontier.token_count(), 15);
}

#[test]
fn test_causal_budget_integration() {
    let mut frontier = CausalFrontier::with_config(100, 1000, 5);

    // Обработка нескольких циклов
    for _ in 0..5 {
        frontier.increment_processed();
    }

    assert!(frontier.is_budget_exhausted());

    // Новый цикл
    frontier.reset_cycle();
    assert!(!frontier.is_budget_exhausted());

    // Можем обрабатывать дальше
    frontier.increment_processed();
    assert!(!frontier.is_budget_exhausted());
}

#[test]
fn test_frontier_getters() {
    let mut frontier = CausalFrontier::with_config(100, 1000, 10);

    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_connection(10);

    assert_eq!(frontier.token_count(), 2);
    assert_eq!(frontier.connection_count(), 1);
    assert_eq!(frontier.size(), 3);
    assert_eq!(frontier.max_frontier_size(), 1000);
}
