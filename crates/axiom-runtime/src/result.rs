// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ProcessingResult — диагностический результат process_and_observe().
//
// Используется CLI Channel и другими адаптерами для наблюдения за
// внутренним состоянием ядра после обработки команды.

use axiom_ucl::UclResult;

/// Путь обработки токена через когнитивный конвейер.
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingPath {
    /// Быстрый рефлекс из EXPERIENCE (score ≥ reflex_threshold)
    Reflex,
    /// Медленный путь: ASHTI(1-8) → MAYA
    SlowPath,
    /// Многопроходная обработка (Cognitive Depth — низкая когерентность)
    MultiPass(u8),
}

/// Диагностический результат `process_and_observe()`.
///
/// Содержит как результат UCL-команды, так и наблюдаемое состояние
/// когнитивного конвейера после её выполнения.
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    /// Результат UCL-команды (статус, error_code, data)
    pub ucl_result: UclResult,
    /// Путь обработки токена
    pub path: ProcessingPath,
    /// ID домена, выдавшего консолидированный результат
    pub dominant_domain_id: u16,
    /// Оценка когерентности ASHTI-результатов (None если не маршрутизировалось)
    pub coherence_score: Option<f32>,
    /// Число активных tension traces в EXPERIENCE
    pub tension_count: u32,
    /// Shell-профиль выходного токена (приближение из полей Token)
    pub output_shell: [u8; 8],
    /// Позиция выходного токена в семантическом пространстве
    pub output_position: [i16; 3],
    /// Был ли использован рефлекс
    pub reflex_hit: bool,
    /// Число следов, прошедших хэш-фильтр при последнем поиске
    pub traces_matched: u32,

    // --- Расширенная диагностика (Фаза 1) ---

    /// Число выполненных проходов (1 = обычный, >1 = multi-pass)
    pub passes: u8,
    /// Максимальное число проходов из конфига MAYA
    pub max_passes: u8,
    /// Минимальный порог coherence из конфига MAYA
    pub min_coherence: f32,
    /// Общее число experience traces (до маршрутизации)
    pub total_traces: u32,
    /// COM event_id созданного события
    pub event_id: u64,
    /// Позиция входного токена (до обработки)
    pub input_position: [i16; 3],
    /// Shell входного токена [0,0,0,valence,temp,mass,0,0]
    pub input_shell: [u8; 8],
    /// FNV-хэш входного токена (для диагностики)
    pub input_hash: u64,
    /// Был ли создан tension trace в ходе этой маршрутизации
    pub tension_created: bool,
}

impl ProcessingResult {
    /// Минимальный результат — команда обработана без маршрутизации.
    pub fn from_ucl(ucl_result: UclResult) -> Self {
        Self {
            ucl_result,
            path: ProcessingPath::SlowPath,
            dominant_domain_id: 0,
            coherence_score: None,
            tension_count: 0,
            output_shell: [0u8; 8],
            output_position: [0i16; 3],
            reflex_hit: false,
            traces_matched: 0,
            passes: 0,
            max_passes: 0,
            min_coherence: 0.6,
            total_traces: 0,
            event_id: 0,
            input_position: [0i16; 3],
            input_shell: [0u8; 8],
            input_hash: 0,
            tension_created: false,
        }
    }
}
