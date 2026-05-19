// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Интерфейс к AxialStore для AxialEvaluator.
// Источник: AxialEvaluator_V1_0.md §7.2

use axiom_experience::{AxialEvaluation, AxialStore};

/// Внутреннее хранилище AxialEvaluator.
///
/// Держит AxialStore + счётчики для диагностики.
#[derive(Debug, Default)]
pub struct EvaluatorStorage {
    store: AxialStore,
    pub total_evaluated: u64,
    pub total_conflicts: u64,
}

impl EvaluatorStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Максимальное число оценок на один Frame (V2).
pub const MAX_EVALUATIONS_PER_FRAME: usize = 20;

    /// Записать оценку. Обрезает историю Frame до MAX_EVALUATIONS_PER_FRAME.
    pub fn record(&mut self, eval: AxialEvaluation) {
        if eval.has_conflict() {
            self.total_conflicts += 1;
        }
        self.total_evaluated += 1;
        let sutra_id = eval.frame_anchor_sutra_id;
        self.store.add(eval);
        self.store.cap_frame(sutra_id, Self::MAX_EVALUATIONS_PER_FRAME);
    }

    pub fn store(&self) -> &AxialStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut AxialStore {
        &mut self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{AxialConflict, AxialScore, ConflictResolution, EvaluationLevel, Octant};

    #[test]
    fn test_record_increments_count() {
        let mut s = EvaluatorStorage::new();
        let eval = AxialEvaluation::new(
            1,
            EvaluationLevel::Conceptual,
            AxialScore::new(200, 50),
            AxialScore::new(180, 60),
            AxialScore::new(190, 40),
            1,
        );
        s.record(eval);
        assert_eq!(s.total_evaluated, 1);
        assert_eq!(s.total_conflicts, 0);
    }

    #[test]
    fn test_conflict_increments_conflict_count() {
        let mut s = EvaluatorStorage::new();
        let eval = AxialEvaluation::new(
            1,
            EvaluationLevel::Conceptual,
            AxialScore::new(200, 50),
            AxialScore::new(180, 60),
            AxialScore::new(190, 40),
            1,
        )
        .with_conflict(AxialConflict {
            analytic_octant: Octant::CreativeAffirmation,
            synthetic_octant: Octant::HeroicFatal,
            conflict_strength: 85,
            resolution: ConflictResolution::Unresolved,
        });
        s.record(eval);
        assert_eq!(s.total_evaluated, 1);
        assert_eq!(s.total_conflicts, 1);
    }

    #[test]
    fn test_store_accessible() {
        let mut s = EvaluatorStorage::new();
        assert!(s.store().is_empty());
        let eval = AxialEvaluation::new(
            42,
            EvaluationLevel::Sensory,
            AxialScore::new(128, 128),
            AxialScore::new(128, 128),
            AxialScore::new(128, 128),
            10,
        );
        s.record(eval);
        assert!(!s.store().is_empty());
        assert_eq!(s.store().frame_count(), 1);
    }
}
