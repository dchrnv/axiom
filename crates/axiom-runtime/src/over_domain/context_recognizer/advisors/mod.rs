// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NeuralAdvisor интерфейс. V1: stub trait без реализаций.
// Источник: ContextRecognizer_V5_0.md §8

use axiom_experience::{Octant, SubsystemId};

/// Подсказка советника: в какую сторону Frame хочет укорениться.
#[derive(Debug, Clone)]
pub struct DepthHint {
    pub target_octant: Octant,
    pub suggested_depth: u16,
    pub confidence: f32, // 0.0..1.0
}

/// Советник по предсказанию глубины SUTRA.
///
/// V1: нет реализаций. Интерфейс задан для V2.
pub trait DepthPredictionAdvisor: Send + Sync {
    fn predict_depth(&self, sutra_id: u32, subsystem: SubsystemId) -> Option<DepthHint>;
}
