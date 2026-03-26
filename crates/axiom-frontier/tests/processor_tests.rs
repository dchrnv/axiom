use axiom_frontier::{CausalFrontier, EvaluationResult, FrontierProcessor, LocalRules};

/// Mock реализация LocalRules для тестирования
struct MockRules {
    token_calls: Vec<usize>,
    connection_calls: Vec<usize>,
    transform_on_token: Option<usize>,
}

impl MockRules {
    fn new() -> Self {
        Self {
            token_calls: Vec::new(),
            connection_calls: Vec::new(),
            transform_on_token: None,
        }
    }

    fn with_transform(mut self, token_id: usize) -> Self {
        self.transform_on_token = Some(token_id);
        self
    }
}

impl LocalRules for MockRules {
    fn evaluate_token(&mut self, token_id: usize) -> EvaluationResult {
        self.token_calls.push(token_id);

        if self.transform_on_token == Some(token_id) {
            // Симулируем трансформацию с affected neighbors
            EvaluationResult::Transform {
                affected_neighbors: vec![token_id + 10, token_id + 20],
            }
        } else {
            EvaluationResult::NoChange
        }
    }

    fn evaluate_connection(&mut self, connection_id: usize) -> EvaluationResult {
        self.connection_calls.push(connection_id);
        EvaluationResult::NoChange
    }
}

#[test]
fn test_processor_step() {
    let mut frontier = CausalFrontier::new();
    frontier.push_token(1);
    frontier.push_token(2);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Первый step
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1]);

    // Второй step
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1, 2]);

    // Frontier пуст
    assert!(!processor.step());
}

#[test]
fn test_processor_with_connections() {
    let mut frontier = CausalFrontier::new();
    frontier.push_token(1);
    frontier.push_connection(10);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Сначала обрабатывается token
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![1]);
    assert_eq!(processor.rules().connection_calls, Vec::<usize>::new());

    // Потом connection
    assert!(processor.step());
    assert_eq!(processor.rules().connection_calls, vec![10]);
}

#[test]
fn test_processor_with_transform() {
    let mut frontier = CausalFrontier::new();
    frontier.push_token(5);

    let rules = MockRules::new().with_transform(5);
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Обрабатываем token 5, который генерирует transform
    assert!(processor.step());
    assert_eq!(processor.rules().token_calls, vec![5]);

    // Проверяем что affected neighbors добавлены во frontier
    assert_eq!(processor.frontier().size(), 2); // 15 и 25
    assert!(processor.frontier().contains_token(15));
    assert!(processor.frontier().contains_token(25));
}

#[test]
fn test_process_until_empty() {
    let mut frontier = CausalFrontier::new();
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
    let mut frontier = CausalFrontier::with_config(100, 1000, 2); // budget = 2
    frontier.push_token(1);
    frontier.push_token(2);
    frontier.push_token(3);

    let rules = MockRules::new();
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    let processed = processor.process_until_empty_or_budget();

    // Должно обработаться только 2 элемента (budget exhausted)
    assert_eq!(processed, 2);
    assert!(!processor.frontier().is_empty());
    assert_eq!(processor.frontier().size(), 1); // Остался token 3
}

#[test]
fn test_processor_chain_reaction() {
    let mut frontier = CausalFrontier::new();
    frontier.push_token(1);

    let rules = MockRules::new().with_transform(1);
    let mut processor = FrontierProcessor::new(&mut frontier, rules);

    // Первый step: обрабатываем token 1, добавляются 11 и 21
    assert!(processor.step());
    assert_eq!(processor.frontier().size(), 2);

    // Продолжаем обработку (без дополнительных трансформаций)
    processor.process_until_empty_or_budget();
    assert!(processor.frontier().is_empty());
    assert_eq!(processor.rules().token_calls.len(), 3); // 1, 11, 21
}
