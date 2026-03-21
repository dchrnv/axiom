//! Processor — алгоритм обработки причинной границы
//!
//! Causal Frontier V1, раздел 5: основной цикл симуляции через frontier.
//!
//! # Алгоритм
//!
//! ```text
//! while frontier not empty:
//!     entity = frontier.pop()
//!     evaluate_local_rules(entity)
//!     if transformation detected:
//!         event = generate_event()
//!         apply_event(event)
//!         affected = collect_neighbors(event)
//!         frontier.add(affected)
//! ```
//!
//! # Принцип локальности
//!
//! Обработка всегда локальна. Никогда не выполняется глобальный проход по состоянию.
//! Все вычисления выполняются только внутри frontier.

use crate::frontier::CausalFrontier;

/// Результат локальной оценки entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationResult {
    /// Трансформация не требуется
    NoChange,

    /// Требуется генерация события с указанными affected neighbors
    Transform {
        /// ID соседей, которые нужно добавить во frontier
        affected_neighbors: Vec<usize>,
    },
}

/// Trait для реализации локальных правил физики/семантики
///
/// Реализуется конкретным доменом (axiom-domain) для определения
/// как оценивать entity и какие соседи затронуты.
pub trait LocalRules {
    /// Оценить token и определить нужна ли трансформация
    ///
    /// # Arguments
    /// * `token_id` - ID токена для оценки
    ///
    /// # Returns
    /// `EvaluationResult` с информацией о трансформации и затронутых соседях
    fn evaluate_token(&mut self, token_id: usize) -> EvaluationResult;

    /// Оценить connection и определить нужна ли трансформация
    ///
    /// # Arguments
    /// * `connection_id` - ID связи для оценки
    ///
    /// # Returns
    /// `EvaluationResult` с информацией о трансформации и затронутых соседях
    fn evaluate_connection(&mut self, connection_id: usize) -> EvaluationResult;
}

/// Процессор причинной границы
///
/// Выполняет основной цикл обработки: pop → evaluate → transform → push neighbors.
pub struct FrontierProcessor<'a, R: LocalRules> {
    frontier: &'a mut CausalFrontier,
    rules: R,
}

impl<'a, R: LocalRules> FrontierProcessor<'a, R> {
    /// Создаёт новый процессор
    ///
    /// # Arguments
    /// * `frontier` - Мутабельная ссылка на frontier
    /// * `rules` - Реализация локальных правил
    pub fn new(frontier: &'a mut CausalFrontier, rules: R) -> Self {
        Self { frontier, rules }
    }

    /// Выполняет один шаг обработки (pop + evaluate + push)
    ///
    /// # Returns
    /// `true` если был обработан хотя бы один элемент, `false` если frontier пуст
    pub fn step(&mut self) -> bool {
        // Сначала пробуем tokens
        if let Some(token_id) = self.frontier.pop_token() {
            self.frontier.increment_processed();
            let result = self.rules.evaluate_token(token_id);
            self.handle_evaluation_result(result);
            return true;
        }

        // Потом connections
        if let Some(connection_id) = self.frontier.pop_connection() {
            self.frontier.increment_processed();
            let result = self.rules.evaluate_connection(connection_id);
            self.handle_evaluation_result(result);
            return true;
        }

        false
    }

    /// Выполняет обработку до опустошения frontier или исчерпания budget
    ///
    /// # Returns
    /// Количество обработанных элементов
    pub fn process_until_empty_or_budget(&mut self) -> usize {
        let mut processed = 0;

        while !self.frontier.is_empty() && !self.frontier.is_budget_exhausted() {
            if self.step() {
                processed += 1;
            } else {
                break;
            }
        }

        processed
    }

    /// Обрабатывает результат оценки и добавляет affected neighbors во frontier
    fn handle_evaluation_result(&mut self, result: EvaluationResult) {
        match result {
            EvaluationResult::NoChange => {
                // Ничего не делаем
            }
            EvaluationResult::Transform { affected_neighbors } => {
                // Добавляем затронутых соседей во frontier
                // Предполагаем что это token ID (можно расширить в будущем)
                for neighbor_id in affected_neighbors {
                    self.frontier.push_token(neighbor_id);
                }
            }
        }
    }

    /// Получает ссылку на frontier
    pub fn frontier(&self) -> &CausalFrontier {
        self.frontier
    }

    /// Получает мутабельную ссылку на frontier
    pub fn frontier_mut(&mut self) -> &mut CausalFrontier {
        self.frontier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(processor.rules.token_calls, vec![1]);

        // Второй step
        assert!(processor.step());
        assert_eq!(processor.rules.token_calls, vec![1, 2]);

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
        assert_eq!(processor.rules.token_calls, vec![1]);
        assert_eq!(processor.rules.connection_calls, Vec::<usize>::new());

        // Потом connection
        assert!(processor.step());
        assert_eq!(processor.rules.connection_calls, vec![10]);
    }

    #[test]
    fn test_processor_with_transform() {
        let mut frontier = CausalFrontier::new();
        frontier.push_token(5);

        let rules = MockRules::new().with_transform(5);
        let mut processor = FrontierProcessor::new(&mut frontier, rules);

        // Обрабатываем token 5, который генерирует transform
        assert!(processor.step());
        assert_eq!(processor.rules.token_calls, vec![5]);

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
        assert_eq!(processor.rules.token_calls, vec![1, 2, 3]);
        assert_eq!(processor.rules.connection_calls, vec![10]);
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
        assert_eq!(processor.rules.token_calls.len(), 3); // 1, 11, 21
    }
}
