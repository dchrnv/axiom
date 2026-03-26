// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// EXPERIENCE module - ассоциативная память Arbiter V1.0

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

/// След опыта (паттерн + вес + метаданные)
#[derive(Debug, Clone)]
pub struct ExperienceTrace {
    /// Паттерн токена
    pub pattern: Token,
    /// Вес следа (0.0 - 1.0)
    pub weight: f32,
    /// Когда создан (event_id)
    pub created_at: u64,
    /// Последнее использование (event_id)
    pub last_used: u64,
    /// Число успешных рефлексов
    pub success_count: u32,
    /// Хэш паттерна для быстрого отбора
    pub pattern_hash: u64,
}

/// Результат резонансного поиска
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// Уровень резонанса
    pub level: ResonanceLevel,
    /// След, если найден
    pub trace: Option<ExperienceTrace>,
}

/// Experience - ассоциативная память (реализация)
pub struct Experience {
    traces: Vec<ExperienceTrace>,
    /// Порог рефлекса (0..255 → 0.0..1.0)
    reflex_threshold: u8,
    /// Порог ассоциации (0..255 → 0.0..1.0)
    association_threshold: u8,
    /// Максимальное число следов
    max_traces: usize,
}

impl Experience {
    /// Создать новый Experience модуль с порогами по умолчанию
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            reflex_threshold: 128,
            association_threshold: 64,
            max_traces: 1000,
        }
    }

    /// Установить пороги из конфигурации домена
    pub fn set_thresholds(&mut self, reflex_threshold: u8, association_threshold: u8) {
        self.reflex_threshold = reflex_threshold;
        self.association_threshold = association_threshold;
    }

    /// Резонансный поиск по паттерну.
    ///
    /// Возвращает Reflex если лучший след имеет score >= reflex_threshold/255.0,
    /// Association если score >= association_threshold/255.0, иначе None.
    pub fn resonance_search(&self, token: &Token) -> ResonanceResult {
        if self.traces.is_empty() {
            return ResonanceResult { level: ResonanceLevel::None, trace: None };
        }

        let input_hash = pattern_hash(token);
        let reflex_t = self.reflex_threshold as f32 / 255.0;
        let assoc_t = self.association_threshold as f32 / 255.0;

        let mut best_score = 0.0f32;
        let mut best_idx: Option<usize> = None;

        for (i, trace) in self.traces.iter().enumerate() {
            // Quick pre-filter: if hashes differ by too much, skip
            let hash_dist = (input_hash ^ trace.pattern_hash).count_ones();
            if hash_dist > 40 {
                continue;
            }

            let similarity = pattern_similarity(token, &trace.pattern);
            let score = similarity * trace.weight;

            if score > best_score {
                best_score = score;
                best_idx = Some(i);
            }
        }

        let level = if best_score >= reflex_t {
            ResonanceLevel::Reflex
        } else if best_score >= assoc_t {
            ResonanceLevel::Association
        } else {
            ResonanceLevel::None
        };

        if level == ResonanceLevel::None {
            return ResonanceResult { level, trace: None };
        }

        ResonanceResult {
            level,
            trace: best_idx.map(|i| self.traces[i].clone()),
        }
    }

    /// Добавить след опыта. Если лимит достигнут, вытесняет след с наименьшим весом.
    pub fn add_trace(&mut self, pattern: Token, weight: f32, created_at: u64) {
        if self.traces.len() >= self.max_traces {
            // Evict lowest weight trace
            if let Some(min_idx) = self.traces.iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.weight.partial_cmp(&b.weight).unwrap())
                .map(|(i, _)| i)
            {
                self.traces.remove(min_idx);
            }
        }

        let ph = pattern_hash(&pattern);
        self.traces.push(ExperienceTrace {
            pattern,
            weight: weight.clamp(0.0, 1.0),
            created_at,
            last_used: created_at,
            success_count: 0,
            pattern_hash: ph,
        });
    }

    /// Усилить след по индексу
    pub fn strengthen_trace(&mut self, idx: usize, delta: f32) {
        if let Some(trace) = self.traces.get_mut(idx) {
            trace.weight = (trace.weight + delta).min(1.0);
            trace.success_count = trace.success_count.saturating_add(1);
        }
    }

    /// Ослабить след по индексу
    pub fn weaken_trace(&mut self, idx: usize, delta: f32) {
        if let Some(trace) = self.traces.get_mut(idx) {
            trace.weight = (trace.weight - delta).max(0.0);
        }
    }

    /// Получить количество следов
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }
}

/// Сходство паттернов токенов — нормализованное значение 0.0 (полное отличие) до 1.0 (идентичны).
///
/// Учитывает температуру, массу, валентность и позицию.
fn pattern_similarity(a: &Token, b: &Token) -> f32 {
    let temp_diff = (a.temperature as i16 - b.temperature as i16).unsigned_abs() as f32 / 255.0;
    let mass_diff = (a.mass as i16 - b.mass as i16).unsigned_abs() as f32 / 255.0;
    let val_diff = (a.valence as i16 - b.valence as i16).unsigned_abs() as f32 / 254.0;

    let dx = (a.position[0] as i32 - b.position[0] as i32) as f32;
    let dy = (a.position[1] as i32 - b.position[1] as i32) as f32;
    let dz = (a.position[2] as i32 - b.position[2] as i32) as f32;
    // Normalize by max possible distance in i16 space (~56755)
    let pos_diff = (dx * dx + dy * dy + dz * dz).sqrt() / 56755.0;

    let avg_diff = (temp_diff + mass_diff + val_diff + pos_diff) * 0.25;
    1.0 - avg_diff.min(1.0)
}

/// Быстрый хэш паттерна токена для предварительной фильтрации
fn pattern_hash(token: &Token) -> u64 {
    // FNV-1a style mixing of key fields
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= token.temperature as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.mass as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= (token.valence as u8) as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[0] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[1] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[2] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h
}

impl Default for Experience {
    fn default() -> Self {
        Self::new()
    }
}
