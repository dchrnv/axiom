//! Causal Frontier V2.0 — управление активной причинной границей
//!
//! Frontier содержит только те элементы состояния, которые могут породить новое событие.
//! Система никогда не выполняет глобальный проход по состоянию.
//!
//! # Архитектура
//!
//! - `FrontierConfig` — конфигурация с пресетами (weak/medium/powerful)
//! - `FrontierEntity` — типизированный элемент frontier (Token | Connection)
//! - `EntityQueue` — очередь с дедупликацией через BitVec
//! - `CausalFrontier` — главная структура с begin_cycle/end_cycle API
//! - `StormMetrics` — метрики для наблюдения за состоянием frontier

use std::collections::VecDeque;
use bitvec::prelude::*;

// ============================================================================
// FrontierConfig
// ============================================================================

/// Конфигурация Causal Frontier
///
/// Определяет лимиты и поведение frontier.
/// Разные домены могут использовать разные конфиги
/// (например, EXPERIENCE — больший max_frontier_size).
#[derive(Debug, Clone, Copy)]
pub struct FrontierConfig {
    /// Жёсткий лимит размера frontier. При превышении push отбрасывает сущности.
    pub max_frontier_size: u32,
    /// Causal budget: максимум событий за один цикл.
    pub max_events_per_cycle: u32,
    /// Порог для перехода в состояние Storm.
    pub storm_threshold: u32,
    /// Включить объединение однотипных событий при шторме.
    pub enable_batch_events: bool,
    /// Ёмкость для предвыделения BitVec токенов.
    pub token_capacity: u32,
    /// Ёмкость для предвыделения BitVec связей.
    pub connection_capacity: u32,
}

impl FrontierConfig {
    /// Слабое оборудование: жёсткие лимиты
    pub fn weak() -> Self {
        Self {
            max_frontier_size: 1000,
            max_events_per_cycle: 100,
            storm_threshold: 500,
            enable_batch_events: true,
            token_capacity: 1024,
            connection_capacity: 512,
        }
    }

    /// Среднее оборудование (по умолчанию)
    pub fn medium() -> Self {
        Self {
            max_frontier_size: 10000,
            max_events_per_cycle: 1000,
            storm_threshold: 5000,
            enable_batch_events: true,
            token_capacity: 4096,
            connection_capacity: 2048,
        }
    }

    /// Мощный сервер
    pub fn powerful() -> Self {
        Self {
            max_frontier_size: 100000,
            max_events_per_cycle: 10000,
            storm_threshold: 50000,
            enable_batch_events: false,
            token_capacity: 65536,
            connection_capacity: 32768,
        }
    }
}

impl Default for FrontierConfig {
    fn default() -> Self {
        Self::medium()
    }
}

// ============================================================================
// FrontierEntity
// ============================================================================

/// Типизированный элемент frontier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierEntity {
    /// Индекс токена в массиве домена
    Token(u32),
    /// Индекс связи в массиве домена
    Connection(u32),
}

// ============================================================================
// StormMetrics
// ============================================================================

/// Метрики состояния frontier для наблюдения
#[derive(Debug, Clone, Copy, Default)]
pub struct StormMetrics {
    /// Сколько событий сгенерировано в текущем цикле
    pub events_this_cycle: u32,
    /// Текущий размер frontier
    pub frontier_size: u32,
    /// Изменение размера за последний цикл (может быть отрицательным)
    pub frontier_growth_rate: i32,
}

// ============================================================================
// EntityQueue
// ============================================================================

/// Типизированная очередь с дедупликацией через BitVec
pub struct EntityQueue {
    queue: VecDeque<u32>,
    visited: BitVec,
    capacity: u32,
}

impl EntityQueue {
    /// Создаёт очередь с предвыделённой ёмкостью
    pub fn new(capacity: u32) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity as usize),
            visited: bitvec![0; capacity as usize],
            capacity,
        }
    }

    /// Добавляет элемент если его ещё нет в visited.
    /// Возвращает `true` если добавлен.
    pub fn push(&mut self, id: u32) -> bool {
        // Расширяем visited при необходимости
        if id as usize >= self.visited.len() {
            self.visited.resize(id as usize + 1, false);
            self.capacity = self.capacity.max(id + 1);
        }
        if !self.visited[id as usize] {
            self.visited.set(id as usize, true);
            self.queue.push_back(id);
            true
        } else {
            false
        }
    }

    /// Извлекает следующий элемент (FIFO). Сбрасывает visited для повторного использования.
    pub fn pop(&mut self) -> Option<u32> {
        let id = self.queue.pop_front()?;
        self.visited.set(id as usize, false);
        Some(id)
    }

    /// Проверяет присутствие элемента в очереди
    pub fn contains(&self, id: u32) -> bool {
        (id as usize) < self.visited.len() && self.visited[id as usize]
    }

    /// Очищает очередь и сбрасывает visited
    pub fn clear(&mut self) {
        self.queue.clear();
        self.visited.fill(false);
    }

    /// Размер очереди
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Пуста ли очередь
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

// ============================================================================
// FrontierState
// ============================================================================

/// Состояния жизненного цикла Causal Frontier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierState {
    /// Frontier пуст. CPU не используется.
    Empty,
    /// Нормальная обработка. frontier_size <= storm_threshold.
    Active,
    /// frontier_size превысил storm_threshold. Mitigation активен.
    Storm,
    /// Выход из шторма. frontier_size упал ниже порога.
    /// Batch events остаются включёнными ещё один цикл.
    Stabilizing,
    /// Frontier обработан до конца в этом цикле.
    Idle,
}

// ============================================================================
// CausalFrontier
// ============================================================================

/// Causal Frontier V2.0 — активная причинная граница системы
///
/// Содержит только те элементы состояния, которые могут породить новое событие.
/// Все вычисления выполняются только внутри frontier — глобальные проходы запрещены.
pub struct CausalFrontier {
    token_queue: EntityQueue,
    connection_queue: EntityQueue,
    state: FrontierState,
    events_this_cycle: u32,
    prev_size: usize,
    frontier_growth_rate: i32,
    config: FrontierConfig,
}

impl CausalFrontier {
    /// Создать frontier из конфигурации
    pub fn new(config: FrontierConfig) -> Self {
        Self {
            token_queue: EntityQueue::new(config.token_capacity),
            connection_queue: EntityQueue::new(config.connection_capacity),
            state: FrontierState::Empty,
            events_this_cycle: 0,
            prev_size: 0,
            frontier_growth_rate: 0,
            config,
        }
    }

    // --- Цикл обработки ---

    /// Начать новый цикл. Сбрасывает events_this_cycle.
    pub fn begin_cycle(&mut self) {
        self.events_this_cycle = 0;
    }

    /// Завершить цикл. Обновляет frontier_growth_rate и пересчитывает state.
    pub fn end_cycle(&mut self) {
        let current_size = self.size();
        self.frontier_growth_rate = current_size as i32 - self.prev_size as i32;
        self.prev_size = current_size;

        self.state = match self.state {
            _ if current_size == 0 => FrontierState::Empty,
            FrontierState::Storm if current_size <= self.config.storm_threshold as usize => {
                FrontierState::Stabilizing
            }
            FrontierState::Stabilizing => {
                if current_size == 0 {
                    FrontierState::Empty
                } else {
                    FrontierState::Active
                }
            }
            _ if current_size > self.config.storm_threshold as usize => FrontierState::Storm,
            _ if current_size > 0 => FrontierState::Active,
            _ => FrontierState::Idle,
        };
    }

    // --- Push ---

    /// Добавить токен во frontier. Дедупликация через BitVec.
    /// Возвращает `false` если лимит размера исчерпан или элемент уже есть.
    pub fn push_token(&mut self, token_index: u32) -> bool {
        if self.size() >= self.config.max_frontier_size as usize {
            return false;
        }
        self.token_queue.push(token_index)
    }

    /// Добавить связь во frontier. Дедупликация через BitVec.
    /// Возвращает `false` если лимит размера исчерпан или элемент уже есть.
    pub fn push_connection(&mut self, connection_index: u32) -> bool {
        if self.size() >= self.config.max_frontier_size as usize {
            return false;
        }
        self.connection_queue.push(connection_index)
    }

    // --- Pop ---

    /// Извлечь следующую сущность.
    /// Токены имеют приоритет над связями.
    /// Возвращает `None` если frontier пуст или causal budget исчерпан.
    pub fn pop(&mut self) -> Option<FrontierEntity> {
        if self.events_this_cycle >= self.config.max_events_per_cycle {
            return None;
        }
        if let Some(id) = self.token_queue.pop() {
            self.events_this_cycle += 1;
            return Some(FrontierEntity::Token(id));
        }
        if let Some(id) = self.connection_queue.pop() {
            self.events_this_cycle += 1;
            return Some(FrontierEntity::Connection(id));
        }
        None
    }

    // --- Содержимое ---

    /// Проверить наличие токена во frontier
    pub fn contains_token(&self, token_index: u32) -> bool {
        self.token_queue.contains(token_index)
    }

    /// Проверить наличие связи во frontier
    pub fn contains_connection(&self, connection_index: u32) -> bool {
        self.connection_queue.contains(connection_index)
    }

    /// Очистить frontier и сбросить visited
    pub fn clear(&mut self) {
        self.token_queue.clear();
        self.connection_queue.clear();
        self.state = FrontierState::Empty;
        self.events_this_cycle = 0;
    }

    // --- Метрики ---

    /// Текущий размер frontier (токены + связи)
    pub fn size(&self) -> usize {
        self.token_queue.len() + self.connection_queue.len()
    }

    /// Пуст ли frontier
    pub fn is_empty(&self) -> bool {
        self.token_queue.is_empty() && self.connection_queue.is_empty()
    }

    /// Текущее состояние жизненного цикла
    pub fn state(&self) -> FrontierState {
        self.state
    }

    /// Снимок метрик для наблюдения
    pub fn metrics(&self) -> StormMetrics {
        StormMetrics {
            events_this_cycle: self.events_this_cycle,
            frontier_size: self.size() as u32,
            frontier_growth_rate: self.frontier_growth_rate,
        }
    }

    /// Causal budget исчерпан в этом цикле
    pub fn is_budget_exhausted(&self) -> bool {
        self.events_this_cycle >= self.config.max_events_per_cycle
    }

    /// Frontier находится в Storm состоянии
    pub fn is_storm(&self) -> bool {
        matches!(self.state, FrontierState::Storm)
    }

    /// Количество токенов в frontier
    pub fn token_count(&self) -> usize {
        self.token_queue.len()
    }

    /// Количество связей в frontier
    pub fn connection_count(&self) -> usize {
        self.connection_queue.len()
    }

    /// Процент заполнения относительно max_frontier_size
    pub fn memory_usage(&self) -> f32 {
        self.size() as f32 / self.config.max_frontier_size as f32 * 100.0
    }

    /// Порог storm из конфигурации
    pub fn storm_threshold(&self) -> u32 {
        self.config.storm_threshold
    }

    /// Максимальный размер frontier из конфигурации
    pub fn max_frontier_size(&self) -> u32 {
        self.config.max_frontier_size
    }

    /// Обновить state на основе текущего размера (совместимость с тестами)
    pub fn update_state(&mut self) {
        let size = self.size();
        self.state = if size == 0 {
            FrontierState::Idle
        } else if size > self.config.storm_threshold as usize {
            FrontierState::Storm
        } else if self.state == FrontierState::Storm {
            FrontierState::Stabilizing
        } else if size > 0 {
            FrontierState::Active
        } else {
            FrontierState::Empty
        };
    }
}

impl Default for CausalFrontier {
    fn default() -> Self {
        Self::new(FrontierConfig::medium())
    }
}
