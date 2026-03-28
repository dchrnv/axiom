//! FrontierProcessor — алгоритм обработки причинной границы
//!
//! Основной цикл: begin_cycle → pop → evaluate → push neighbors → end_cycle.
//! Обработка всегда локальна — глобальные проходы запрещены.

use crate::frontier::{CausalFrontier, FrontierEntity};

/// Результат локальной оценки сущности
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationResult {
    /// Трансформация не требуется
    NoChange,
    /// Требуется генерация события. Содержит затронутых соседей (токены и/или связи).
    Transform {
        /// Соседи, которые нужно добавить во frontier
        affected_neighbors: Vec<FrontierEntity>,
    },
}

/// Trait для реализации локальных правил физики/семантики
///
/// Реализуется конкретным доменом (axiom-domain).
pub trait LocalRules {
    /// Оценить токен и определить нужна ли трансформация
    fn evaluate_token(&mut self, token_id: u32) -> EvaluationResult;

    /// Оценить связь и определить нужна ли трансформация
    fn evaluate_connection(&mut self, connection_id: u32) -> EvaluationResult;
}

/// Процессор причинной границы
///
/// Выполняет цикл: begin_cycle → pop → evaluate → push neighbors → end_cycle.
pub struct FrontierProcessor<'a, R: LocalRules> {
    frontier: &'a mut CausalFrontier,
    rules: R,
}

impl<'a, R: LocalRules> FrontierProcessor<'a, R> {
    /// Создать процессор
    pub fn new(frontier: &'a mut CausalFrontier, rules: R) -> Self {
        Self { frontier, rules }
    }

    /// Выполнить один шаг: pop → evaluate → push neighbors.
    /// Возвращает `true` если был обработан элемент.
    pub fn step(&mut self) -> bool {
        let entity = match self.frontier.pop() {
            Some(e) => e,
            None => return false,
        };

        let result = match entity {
            FrontierEntity::Token(id) => self.rules.evaluate_token(id),
            FrontierEntity::Connection(id) => self.rules.evaluate_connection(id),
            // Batch элементы уже слиты — соседи не пересчитываются
            FrontierEntity::BatchToken(_) | FrontierEntity::BatchConnection(_) => {
                EvaluationResult::NoChange
            }
        };

        if let EvaluationResult::Transform { affected_neighbors } = result {
            for neighbor in affected_neighbors {
                match neighbor {
                    FrontierEntity::Token(id) => { self.frontier.push_token(id); }
                    FrontierEntity::Connection(id) => { self.frontier.push_connection(id); }
                    FrontierEntity::BatchToken(_) | FrontierEntity::BatchConnection(_) => {}
                }
            }
        }

        true
    }

    /// Обрабатывать до опустошения frontier или исчерпания budget.
    /// Возвращает количество обработанных элементов.
    pub fn process_until_empty_or_budget(&mut self) -> usize {
        self.frontier.begin_cycle();
        let mut processed = 0;
        while self.step() {
            processed += 1;
        }
        self.frontier.end_cycle();
        processed
    }

    /// Ссылка на frontier
    pub fn frontier(&self) -> &CausalFrontier {
        self.frontier
    }

    /// Мутабельная ссылка на frontier
    pub fn frontier_mut(&mut self) -> &mut CausalFrontier {
        self.frontier
    }

    /// Ссылка на правила (для тестирования)
    pub fn rules(&self) -> &R {
        &self.rules
    }

    /// Мутабельная ссылка на правила (для тестирования)
    pub fn rules_mut(&mut self) -> &mut R {
        &mut self.rules
    }
}
