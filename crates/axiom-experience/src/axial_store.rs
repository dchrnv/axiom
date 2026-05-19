// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxialStore — хранилище осевых оценок Frame.
//
// AxialEvaluator записывает сюда результаты оценки Frame по трём философским осям.
// Один Frame может иметь несколько AxialEvaluation — по разным EvaluationLevel.
//
// Источник: `docs/architecture/AxialEvaluator_V1_0.md §5, §7`

use crate::types::{AxialScore, EvaluationLevel, Octant};
#[cfg(test)]
use crate::types::AxialDominant;
use std::collections::HashMap;

/// Конфликт между аналитической и синтетической оценкой октанта.
///
/// Источник: AxialEvaluator_V1_0.md §6 (Corpus Callosum)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxialConflict {
    /// Октант по раздельным score (анализ)
    pub analytic_octant: Octant,
    /// Октант по целостному распознаванию (синтез)
    pub synthetic_octant: Octant,
    /// Сила противоречия 0..255
    pub conflict_strength: u8,
    pub resolution: ConflictResolution,
}

/// Способ разрешения конфликта анализа и синтеза.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConflictResolution {
    /// Побеждает формальная оценка по осям
    AnalyticDominant,
    /// Побеждает целостное распознавание архетипа
    SyntheticDominant,
    /// Конфликт передан в подсистему Dilemmas (sutra_id Frame-дилеммы)
    DilemmaTriggered(u32),
    /// Конфликт оставлен явным — "искра сознания"
    Unresolved,
}

/// Результат осевой оценки одного Frame на одном уровне.
///
/// Источник: AxialEvaluator_V1_0.md §5
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxialEvaluation {
    pub frame_anchor_sutra_id: u32,
    pub level: EvaluationLevel,
    pub x_axis: AxialScore,
    pub y_axis: AxialScore,
    pub z_axis: AxialScore,
    /// Октант из аналитической оценки (первичный)
    pub octant: Octant,
    /// Конфликт, если analytic ≠ synthetic октант
    pub conflict: Option<AxialConflict>,
    pub computed_at_event: u64,
}

impl AxialEvaluation {
    pub fn new(
        frame_anchor_sutra_id: u32,
        level: EvaluationLevel,
        x_axis: AxialScore,
        y_axis: AxialScore,
        z_axis: AxialScore,
        computed_at_event: u64,
    ) -> Self {
        let octant = Octant::from_scores(&x_axis, &y_axis, &z_axis);
        Self {
            frame_anchor_sutra_id,
            level,
            x_axis,
            y_axis,
            z_axis,
            octant,
            conflict: None,
            computed_at_event,
        }
    }

    pub fn with_conflict(mut self, conflict: AxialConflict) -> Self {
        self.conflict = Some(conflict);
        self
    }

    pub fn has_conflict(&self) -> bool {
        self.conflict.is_some()
    }
}

/// Хранилище всех осевых оценок.
///
/// Ключ — `sutra_id` Frame-анкера. Значение — все оценки (по разным уровням).
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxialStore {
    evaluations: HashMap<u32, Vec<AxialEvaluation>>,
}

impl AxialStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, eval: AxialEvaluation) {
        self.evaluations
            .entry(eval.frame_anchor_sutra_id)
            .or_default()
            .push(eval);
    }

    pub fn get_all(&self, sutra_id: u32) -> &[AxialEvaluation] {
        self.evaluations
            .get(&sutra_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Последняя оценка по времени (наибольший computed_at_event).
    pub fn get_latest(&self, sutra_id: u32) -> Option<&AxialEvaluation> {
        self.evaluations
            .get(&sutra_id)?
            .iter()
            .max_by_key(|e| e.computed_at_event)
    }

    /// Оценка для конкретного уровня (последняя по времени).
    pub fn get_at_level(&self, sutra_id: u32, level: EvaluationLevel) -> Option<&AxialEvaluation> {
        self.evaluations
            .get(&sutra_id)?
            .iter()
            .filter(|e| e.level == level)
            .max_by_key(|e| e.computed_at_event)
    }

    /// Число Frame с хотя бы одной оценкой.
    pub fn frame_count(&self) -> usize {
        self.evaluations.len()
    }

    /// Общее число записей оценок.
    pub fn total_evaluations(&self) -> usize {
        self.evaluations.values().map(|v| v.len()).sum()
    }

    pub fn remove_frame(&mut self, sutra_id: u32) {
        self.evaluations.remove(&sutra_id);
    }

    /// Обрезать историю Frame до `max` последних записей (по computed_at_event).
    pub fn cap_frame(&mut self, sutra_id: u32, max: usize) {
        if let Some(evals) = self.evaluations.get_mut(&sutra_id) {
            if evals.len() > max {
                evals.sort_unstable_by_key(|e| e.computed_at_event);
                let remove = evals.len() - max;
                evals.drain(..remove);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.evaluations.is_empty()
    }

    /// Most common octant across all frames' latest evaluations, as u8 (0–7).
    pub fn most_common_octant(&self) -> Option<u8> {
        let mut counts = [0u32; 8];
        let mut total = 0u32;
        for evals in self.evaluations.values() {
            if let Some(latest) = evals.iter().max_by_key(|e| e.computed_at_event) {
                counts[latest.octant as usize] += 1;
                total += 1;
            }
        }
        if total == 0 {
            return None;
        }
        counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(sutra_id: u32, level: EvaluationLevel, event: u64) -> AxialEvaluation {
        AxialEvaluation::new(
            sutra_id,
            level,
            AxialScore::new(200, 50),
            AxialScore::new(180, 60),
            AxialScore::new(190, 40),
            event,
        )
    }

    #[test]
    fn test_octant_derived_from_scores() {
        let x = AxialScore::new(200, 50); // strongly positive
        let y = AxialScore::new(180, 60); // strongly positive
        let z = AxialScore::new(190, 40); // strongly positive
        assert_eq!(Octant::from_scores(&x, &y, &z), Octant::CreativeAffirmation);
    }

    #[test]
    fn test_add_and_get_latest() {
        let mut store = AxialStore::new();
        store.add(eval(1, EvaluationLevel::Sensory, 10));
        store.add(eval(1, EvaluationLevel::Conceptual, 20));
        let latest = store.get_latest(1).unwrap();
        assert_eq!(latest.computed_at_event, 20);
    }

    #[test]
    fn test_get_at_level() {
        let mut store = AxialStore::new();
        store.add(eval(1, EvaluationLevel::Sensory, 10));
        store.add(eval(1, EvaluationLevel::Conceptual, 20));
        assert!(store.get_at_level(1, EvaluationLevel::Conceptual).is_some());
        assert!(store.get_at_level(1, EvaluationLevel::Social).is_none());
    }

    #[test]
    fn test_multiple_frames() {
        let mut store = AxialStore::new();
        store.add(eval(1, EvaluationLevel::Sensory, 1));
        store.add(eval(2, EvaluationLevel::Sensory, 1));
        assert_eq!(store.frame_count(), 2);
        assert_eq!(store.total_evaluations(), 2);
    }

    #[test]
    fn test_remove_frame() {
        let mut store = AxialStore::new();
        store.add(eval(1, EvaluationLevel::Sensory, 1));
        store.remove_frame(1);
        assert!(store.get_latest(1).is_none());
        assert_eq!(store.frame_count(), 0);
    }

    #[test]
    fn test_axial_dominant_thresholds() {
        assert_eq!(
            AxialDominant::from_diff(200, 50),
            AxialDominant::StronglyPositive
        );
        assert_eq!(
            AxialDominant::from_diff(150, 110),
            AxialDominant::LeaningPositive
        );
        assert_eq!(
            AxialDominant::from_diff(130, 120),
            AxialDominant::Balanced
        );
        assert_eq!(
            AxialDominant::from_diff(110, 150),
            AxialDominant::LeaningNegative
        );
        assert_eq!(
            AxialDominant::from_diff(50, 200),
            AxialDominant::StronglyNegative
        );
    }

    #[test]
    fn test_conflict_stored() {
        let mut store = AxialStore::new();
        let e = eval(1, EvaluationLevel::Sensory, 1).with_conflict(AxialConflict {
            analytic_octant: Octant::CreativeAffirmation,
            synthetic_octant: Octant::HeroicFatal,
            conflict_strength: 128,
            resolution: ConflictResolution::Unresolved,
        });
        assert!(e.has_conflict());
        store.add(e);
        assert!(store.get_latest(1).unwrap().has_conflict());
    }

    #[test]
    fn test_empty_frame_returns_empty_slice() {
        let store = AxialStore::new();
        assert!(store.get_all(999).is_empty());
        assert!(store.get_latest(999).is_none());
    }
}
