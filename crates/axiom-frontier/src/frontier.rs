//! Causal Frontier System — управление активной причинной границей
//!
//! Causal Frontier V1: Механизм управления вычислениями через активную причинную границу.
//! Frontier содержит только те элементы состояния, которые могут породить новое событие.
//! Система никогда не выполняет глобальный проход по состоянию.
//!
//! # Архитектура
//!
//! - `EntityQueue` — типизированная очередь с дедупликацией через visited tracking
//! - `CausalFrontier` — главная структура с token_frontier и connection_frontier
//! - `FrontierState` — состояния жизненного цикла (Empty, Active, Storm, Stabilized, Idle)
//!
//! # Storm Detection
//!
//! Когда размер frontier превышает `storm_threshold`, активируется Storm режим.
//! Storm mitigation происходит через causal budget (`max_events_per_cycle`).

use std::collections::VecDeque;

/// Типизированная очередь с дедупликацией
///
/// Causal Frontier V1, раздел 4: базовая структура с visited tracking
pub struct EntityQueue {
    queue: VecDeque<usize>,
    visited: Vec<bool>, // BitSet для дедупликации
    max_id: usize,
}

impl EntityQueue {
    /// Создаёт новую очередь с начальной ёмкостью
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            visited: vec![false; capacity],
            max_id: capacity,
        }
    }

    /// Добавляет элемент если его ещё нет в visited
    pub fn push(&mut self, id: usize) -> bool {
        if id >= self.max_id {
            // Расширяем visited если нужно
            self.visited.resize(id + 1, false);
            self.max_id = id + 1;
        }

        if !self.visited[id] {
            self.visited[id] = true;
            self.queue.push_back(id);
            true
        } else {
            false
        }
    }

    /// Извлекает элемент из начала очереди
    pub fn pop(&mut self) -> Option<usize> {
        if let Some(id) = self.queue.pop_front() {
            self.visited[id] = false; // Сбрасываем visited для повторного использования
            Some(id)
        } else {
            None
        }
    }

    /// Проверяет содержится ли элемент в очереди
    pub fn contains(&self, id: usize) -> bool {
        id < self.max_id && self.visited[id]
    }

    /// Очищает очередь
    pub fn clear(&mut self) {
        self.queue.clear();
        self.visited.fill(false);
    }

    /// Размер очереди
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Состояние Causal Frontier
///
/// Causal Frontier V1, раздел 13: lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierState {
    /// Frontier пуст
    Empty,
    /// Нормальная обработка
    Active,
    /// Causal storm detected
    Storm,
    /// Возврат к нормальному состоянию после storm
    Stabilized,
    /// Нет активных событий, система в покое
    Idle,
}

/// CausalFrontier - управление активной причинной границей системы
///
/// Causal Frontier V1: содержит только те элементы состояния,
/// которые могут породить новое событие. Система никогда не выполняет
/// глобальный проход по состоянию.
pub struct CausalFrontier {
    /// Очередь токенов для обработки
    token_frontier: EntityQueue,

    /// Очередь связей для обработки
    connection_frontier: EntityQueue,

    /// Текущее состояние frontier
    state: FrontierState,

    /// Порог для определения causal storm
    storm_threshold: usize,

    /// Максимальный размер frontier (memory limit)
    max_frontier_size: usize,

    /// Счётчик обработанных элементов в текущем цикле
    processed_this_cycle: usize,

    /// Лимит обработки событий за цикл (causal budget)
    max_events_per_cycle: usize,
}

impl CausalFrontier {
    /// Создаёт новый Causal Frontier с параметрами по умолчанию
    pub fn new() -> Self {
        Self::with_config(1000, 10000, 1000)
    }

    /// Создаёт Causal Frontier с кастомной конфигурацией
    ///
    /// # Arguments
    /// * `storm_threshold` - размер frontier для активации storm mode
    /// * `max_frontier_size` - жёсткий лимит памяти
    /// * `max_events_per_cycle` - causal budget
    pub fn with_config(
        storm_threshold: usize,
        max_frontier_size: usize,
        max_events_per_cycle: usize,
    ) -> Self {
        Self {
            token_frontier: EntityQueue::new(1024),
            connection_frontier: EntityQueue::new(512),
            state: FrontierState::Empty,
            storm_threshold,
            max_frontier_size,
            processed_this_cycle: 0,
            max_events_per_cycle,
        }
    }

    /// Добавляет токен в frontier
    ///
    /// Causal Frontier V1, раздел 6: добавление элементов
    ///
    /// # Returns
    /// `true` если элемент добавлен, `false` если превышен лимит памяти или элемент уже есть
    pub fn push_token(&mut self, token_id: usize) -> bool {
        if self.check_memory_limit() {
            self.token_frontier.push(token_id)
        } else {
            false
        }
    }

    /// Добавляет связь в frontier
    ///
    /// # Returns
    /// `true` если элемент добавлен, `false` если превышен лимит памяти или элемент уже есть
    pub fn push_connection(&mut self, connection_id: usize) -> bool {
        if self.check_memory_limit() {
            self.connection_frontier.push(connection_id)
        } else {
            false
        }
    }

    /// Извлекает следующий токен для обработки
    pub fn pop_token(&mut self) -> Option<usize> {
        self.token_frontier.pop()
    }

    /// Извлекает следующую связь для обработки
    pub fn pop_connection(&mut self) -> Option<usize> {
        self.connection_frontier.pop()
    }

    /// Проверяет содержится ли токен в frontier
    pub fn contains_token(&self, token_id: usize) -> bool {
        self.token_frontier.contains(token_id)
    }

    /// Проверяет содержится ли связь в frontier
    pub fn contains_connection(&self, connection_id: usize) -> bool {
        self.connection_frontier.contains(connection_id)
    }

    /// Очищает frontier полностью
    pub fn clear(&mut self) {
        self.token_frontier.clear();
        self.connection_frontier.clear();
        self.state = FrontierState::Empty;
        self.processed_this_cycle = 0;
    }

    /// Возвращает общий размер frontier
    pub fn size(&self) -> usize {
        self.token_frontier.len() + self.connection_frontier.len()
    }

    /// Проверяет пуст ли frontier
    pub fn is_empty(&self) -> bool {
        self.token_frontier.is_empty() && self.connection_frontier.is_empty()
    }

    /// Получает текущее состояние frontier
    pub fn state(&self) -> FrontierState {
        self.state
    }

    /// Обновляет состояние frontier на основе размера
    ///
    /// Causal Frontier V1, раздел 9: Storm detection
    pub fn update_state(&mut self) {
        let size = self.size();

        self.state = if size == 0 {
            FrontierState::Idle
        } else if size > self.storm_threshold {
            FrontierState::Storm
        } else if self.state == FrontierState::Storm && size < self.storm_threshold / 2 {
            // Storm → Stabilized: размер упал ниже половины порога
            FrontierState::Stabilized
        } else if self.state == FrontierState::Stabilized || (self.state == FrontierState::Storm && size <= self.storm_threshold) {
            // Stabilized сохраняется пока не станет Active или не уйдет в Idle
            // Storm с размером на границе также переходит в Stabilized
            FrontierState::Stabilized
        } else if size > 0 {
            FrontierState::Active
        } else {
            FrontierState::Empty
        };
    }

    /// Проверяет достигнут ли causal budget для текущего цикла
    ///
    /// Causal Frontier V1, раздел 9.3: Causal budget
    pub fn is_budget_exhausted(&self) -> bool {
        self.processed_this_cycle >= self.max_events_per_cycle
    }

    /// Увеличивает счётчик обработанных событий
    pub fn increment_processed(&mut self) {
        self.processed_this_cycle += 1;
    }

    /// Сбрасывает счётчик обработанных событий (новый цикл)
    pub fn reset_cycle(&mut self) {
        self.processed_this_cycle = 0;
    }

    /// Проверяет не превышен ли лимит памяти
    ///
    /// Causal Frontier V1, раздел 10: Frontier memory limits
    fn check_memory_limit(&self) -> bool {
        self.size() < self.max_frontier_size
    }

    /// Получает процент заполнения frontier относительно лимита памяти
    pub fn memory_usage(&self) -> f32 {
        (self.size() as f32 / self.max_frontier_size as f32) * 100.0
    }

    /// Возвращает количество токенов в frontier
    pub fn token_count(&self) -> usize {
        self.token_frontier.len()
    }

    /// Возвращает количество связей в frontier
    pub fn connection_count(&self) -> usize {
        self.connection_frontier.len()
    }

    /// Проверяет находится ли frontier в storm состоянии
    pub fn is_storm(&self) -> bool {
        matches!(self.state, FrontierState::Storm)
    }

    /// Получает текущий порог storm
    pub fn storm_threshold(&self) -> usize {
        self.storm_threshold
    }

    /// Получает максимальный размер frontier
    pub fn max_frontier_size(&self) -> usize {
        self.max_frontier_size
    }
}

impl Default for CausalFrontier {
    fn default() -> Self {
        Self::new()
    }
}
