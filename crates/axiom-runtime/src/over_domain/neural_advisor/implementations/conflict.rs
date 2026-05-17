// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// RuleBasedCorpusCallosumResolver — V1 реализация без ML.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §8.1

use crate::over_domain::neural_advisor::traits::{
    ConflictAdvisorInput, ConflictDiagnosis, ConflictResolutionHint, CorpusCallosumResolver,
};

/// Возраст в тиках ниже которого конфликт считается переходным состоянием.
const TRANSITION_AGE_TICKS: u64 = 20;

/// Минимальное число реактиваций для диагноза DualNature.
const STABLE_REACTIVATION_COUNT: u32 = 10;

/// conflict_strength для ровно одной отличающейся оси.
const STRENGTH_ONE_AXIS: u8 = 85;

/// conflict_strength для двух отличающихся осей.
const STRENGTH_TWO_AXES: u8 = 170;

pub struct RuleBasedCorpusCallosumResolver;

impl CorpusCallosumResolver for RuleBasedCorpusCallosumResolver {
    fn resolve(&self, input: &ConflictAdvisorInput) -> ConflictResolutionHint {
        let (diagnosis, confidence) = diagnose(input);
        ConflictResolutionHint { diagnosis, confidence }
    }
}

fn diagnose(input: &ConflictAdvisorInput) -> (ConflictDiagnosis, f32) {
    match input.conflict_strength {
        STRENGTH_ONE_AXIS => {
            // Один бит отличается — Frame на границе двух соседних октантов.
            // Это структурно нормально: Apollo/Dionysus-граница, например.
            (ConflictDiagnosis::BoundaryFrame, 0.80)
        }
        STRENGTH_TWO_AXES => {
            if input.frame_age_ticks < TRANSITION_AGE_TICKS {
                // Молодой Frame с конфликтом по двум осям — скорее всего ещё не устоялся.
                (ConflictDiagnosis::TransitionState, 0.65)
            } else if input.reactivation_count >= STABLE_REACTIVATION_COUNT {
                // Зрелый Frame, часто реактивируется в разных октантах — двойственная природа.
                (ConflictDiagnosis::DualNature, 0.70)
            } else {
                // Зрелый, но мало реактивирован — недостаточно данных.
                (ConflictDiagnosis::Unresolved, 0.50)
            }
        }
        _ => {
            // Три оси (255) или неожиданное значение — максимальный конфликт,
            // правила не применимы.
            (ConflictDiagnosis::Unresolved, 0.50)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{Octant, SubsystemId};

    fn input(strength: u8, age: u64, reactivations: u32) -> ConflictAdvisorInput {
        ConflictAdvisorInput {
            sutra_id: 1,
            analytic_octant: Octant::CreativeAffirmation,
            synthetic_octant: Octant::EcstaticAffirmation,
            conflict_strength: strength,
            frame_age_ticks: age,
            reactivation_count: reactivations,
            primary_subsystem: SubsystemId::Writing,
            event_id: 100,
        }
    }

    #[test]
    fn test_one_axis_is_boundary() {
        let resolver = RuleBasedCorpusCallosumResolver;
        let hint = resolver.resolve(&input(STRENGTH_ONE_AXIS, 50, 5));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::BoundaryFrame);
        assert!((hint.confidence - 0.80).abs() < f32::EPSILON);
    }

    #[test]
    fn test_two_axes_young_frame_is_transition() {
        let resolver = RuleBasedCorpusCallosumResolver;
        let hint = resolver.resolve(&input(STRENGTH_TWO_AXES, TRANSITION_AGE_TICKS - 1, 20));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::TransitionState);
        assert!((hint.confidence - 0.65).abs() < f32::EPSILON);
    }

    #[test]
    fn test_two_axes_stable_many_reactivations_is_dual() {
        let resolver = RuleBasedCorpusCallosumResolver;
        let hint = resolver.resolve(&input(STRENGTH_TWO_AXES, TRANSITION_AGE_TICKS + 10, STABLE_REACTIVATION_COUNT));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::DualNature);
        assert!((hint.confidence - 0.70).abs() < f32::EPSILON);
    }

    #[test]
    fn test_two_axes_stable_few_reactivations_is_unresolved() {
        let resolver = RuleBasedCorpusCallosumResolver;
        let hint = resolver.resolve(&input(STRENGTH_TWO_AXES, TRANSITION_AGE_TICKS + 10, STABLE_REACTIVATION_COUNT - 1));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::Unresolved);
    }

    #[test]
    fn test_three_axes_is_unresolved() {
        let resolver = RuleBasedCorpusCallosumResolver;
        let hint = resolver.resolve(&input(255, 1000, 100));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::Unresolved);
    }
}
