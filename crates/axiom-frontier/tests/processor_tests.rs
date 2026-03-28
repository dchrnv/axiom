use axiom_frontier::{CausalFrontier, EvaluationResult, FrontierConfig, FrontierEntity, FrontierProcessor, LocalRules};

/// Mock LocalRules for testing
struct MockRules {
    token_calls: Vec<u32>,
    connection_calls: Vec<u32>,
    transform_on_token: Option<u32>,
}

impl MockRules {
    fn new() -> Self {
        Self {
            token_calls: Vec::new(),
            connection_calls: Vec::new(),
            transform_on_token: None,
        }
    }

    fn with_transform(mut self, token_id: u32) -> Self {
        self.transform_on_token = Some(token_id);
        self
    }
}

impl LocalRules for MockRules {
    fn evaluate_token(&mut self, token_id: u32) -> EvaluationResult {
        self.token_calls.push(token_id);

        if self.transform_on_token == Some(token_id) {
            EvaluationResult::Transform {
                affected_neighbors: vec![
                    FrontierEntity::Token(token_id + 10),
                    FrontierEntity::Token(token_id + 20),
                ],
            }
        } else {
            EvaluationResult::NoChange
        }
    }

    fn evaluate_connection(&mut self, connection_id: u32) -> EvaluationResult {
        self.connection_calls.push(connection_id);
        EvaluationResult::NoChange
    }
}

#[test]
fn test_processor_step() {
    let mut frontier = CausalFrontier::default();
    frontier.push_token(1);
    frontier.push_token(2);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // begin_cycle not needed for step() — events_this_cycle starts at 0
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1]);

    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1, 2]);

    assert!(!processor.step());
}

#[test]
fn test_processor_with_connections() {
    let mut frontier = CausalFrontier::default();
    frontier.push_token(1);
    frontier.push_connection(10);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Token has priority
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1]);
    assert_eq!(processor.rules().connection_calls, Vec::<u32>::new());

    // Then connection
    assert!(processor.step());
    assert_eq!(processor.rules().connection_calls, vec![10]);
}

#[test]
fn test_processor_with_transform() {
    let mut frontier = CausalFrontier::default();
    frontier.push_token(5);

    let rules = MockRules::new().with_transform(5);
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![5]);

    // Affected neighbors (15 and 25) should be in frontier
    assert_eq!(processor.frontier().size(), 2);
    assert!(processor.frontier().contains_token(15));
    assert!(processor.frontier().contains_token(25));
}

#[test]
fn test_process_until_empty() {
    let mut frontier = CausalFrontier::default();
    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_token(3);
    frontier.push_connection(10);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    let processed = processor.process_until_empty_or_budget();

    assert_eq!(processed, 4); // 3 tokens + 1 connection
    assert!(processor.frontier().is_empty());
    assert_eq!(processor.rules().token_calls, vec![1, 2, 3]);
    assert_eq!(processor.rules().connection_calls, vec![10]);
}

#[test]
fn test_process_respects_budget() {
    let config = FrontierConfig {
        max_frontier_size: 100,
        max_events_per_cycle: 2,
        storm_threshold: 1000,
        enable_batch_events: true,
        batch_size: 0,
        token_capacity: 100,
        connection_capacity: 100,
    };
    let mut frontier = CausalFrontier::new(config);
    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_token(3);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    let processed = processor.process_until_empty_or_budget();

    // Only 2 processed (budget = 2)
    assert_eq!(processed, 2);
    assert!(!processor.frontier().is_empty());
    assert_eq!(processor.frontier().size(), 1); // token 3 remains
}

#[test]
fn test_processor_chain_reaction() {
    let mut frontier = CausalFrontier::default();
    frontier.push_token(1);

    let rules = MockRules::new().with_transform(1);
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Step 1: process token 1, adds 11 and 21
    assert!(processor.step());
    assert_eq!(processor.frontier().size(), 2);

    // Continue without further transforms
    processor.process_until_empty_or_budget();
    assert!(processor.frontier().is_empty());
    assert_eq!(processor.rules().token_calls.len(), 3); // 1, 11, 21
}
