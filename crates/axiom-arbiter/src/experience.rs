// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// EXPERIENCE module stub
// TODO: Replace with full implementation when experience module is migrated

use axiom_core::Token;

/// Уровень резонанса
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResonanceLevel {
    /// Нет резонанса
    None,
    /// Ассоциация (подсказка)
    Association,
    /// Рефлекс (прямой ответ)
    Reflex,
}

/// След опыта (паттерн + вес)
#[derive(Debug, Clone)]
pub struct ExperienceTrace {
    /// Паттерн токена
    pub pattern: Token,
    /// Вес следа (0.0 - 1.0)
    pub weight: f32,
    /// Когда создан (event_id)
    pub created_at: u64,
}

/// Результат резонансного поиска
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// Уровень резонанса
    pub level: ResonanceLevel,
    /// След, если найден
    pub trace: Option<ExperienceTrace>,
}

/// Experience - ассоциативная память (stub)
pub struct Experience {
    traces: Vec<ExperienceTrace>,
}

impl Experience {
    /// Создать новый Experience модуль
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
        }
    }

    /// Резонансный поиск по паттерну
    pub fn resonance_search(&self, _token: &Token) -> ResonanceResult {
        // Stub: всегда возвращаем None
        ResonanceResult {
            level: ResonanceLevel::None,
            trace: None,
        }
    }

    /// Добавить след опыта
    pub fn add_trace(&mut self, pattern: Token, weight: f32, created_at: u64) {
        self.traces.push(ExperienceTrace {
            pattern,
            weight,
            created_at,
        });
    }

    /// Получить количество следов
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }
}

impl Default for Experience {
    fn default() -> Self {
        Self::new()
    }
}
