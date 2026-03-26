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

    /// Получает ссылку на правила (для тестирования)
    pub fn rules(&self) -> &R {
        &self.rules
    }

    /// Получает мутабельную ссылку на правила (для тестирования)
    pub fn rules_mut(&mut self) -> &mut R {
        &mut self.rules
    }
}
