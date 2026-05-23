// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DepthThresholdEmergentDetector — V1 реализация без ML.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §8.2
//
// Кандидат обнаруживается по трём порогам: глубина, реактивации, возраст.
// Все пороги — именованные константы, будут откалиброваны по OBS-01.

use crate::over_domain::neural_advisor::traits::{
    EmergentAdvisorInput, EmergentDetectionResult, EmergentPatternAdvisor,
};

/// Минимальная глубина в октанте для статуса кандидата.
/// ~1.5% от PRIMITIVE_DEPTH (65535). Откалибровано по OBS-02: O7 avg=1198.
pub const EMERGENT_CANDIDATE_MIN_DEPTH: u16 = 1000;

/// Минимальное число реактиваций (DREAM-циклов с активностью).
/// Откалибровано по OBS-02: ~10-15 циклов за 30k тиков.
pub const EMERGENT_CANDIDATE_MIN_REACTIVATIONS: u32 = 5;

/// Минимальный возраст Frame в тиках.
pub const EMERGENT_CANDIDATE_MIN_AGE_TICKS: u64 = 100;

/// Confidence детектора на V1 правилах — намеренно консервативная.
const CANDIDATE_CONFIDENCE: f32 = 0.60;

pub struct DepthThresholdEmergentDetector;

impl EmergentPatternAdvisor for DepthThresholdEmergentDetector {
    fn detect(&self, input: &EmergentAdvisorInput) -> EmergentDetectionResult {
        // Уже является примитивом — не кандидат
        if input.known_primitive_ids.contains(&input.sutra_id) {
            return EmergentDetectionResult { is_candidate: false, confidence: 0.0 };
        }

        let octant_depth = input.depth_per_octant[input.octant.index()];

        let passes_depth = octant_depth >= EMERGENT_CANDIDATE_MIN_DEPTH;
        let passes_reactivations = input.reactivation_count >= EMERGENT_CANDIDATE_MIN_REACTIVATIONS;
        let passes_age = input.frame_age_ticks >= EMERGENT_CANDIDATE_MIN_AGE_TICKS;

        if passes_depth && passes_reactivations && passes_age {
            EmergentDetectionResult {
                is_candidate: true,
                confidence: CANDIDATE_CONFIDENCE,
            }
        } else {
            EmergentDetectionResult {
                is_candidate: false,
                confidence: 0.0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::Octant;

    fn input(depth: u16, reactivations: u32, age: u64, is_primitive: bool) -> EmergentAdvisorInput {
        let mut depth_arr = [0u16; 8];
        depth_arr[0] = depth; // CreativeAffirmation = index 0
        EmergentAdvisorInput {
            sutra_id: 42,
            octant: Octant::CreativeAffirmation,
            depth_per_octant: depth_arr,
            reactivation_count: reactivations,
            frame_age_ticks: age,
            known_primitive_ids: if is_primitive { vec![42] } else { vec![] },
            event_id: 1000,
        }
    }

    #[test]
    fn test_all_thresholds_met_is_candidate() {
        let detector = DepthThresholdEmergentDetector;
        let result = detector.detect(&input(
            EMERGENT_CANDIDATE_MIN_DEPTH,
            EMERGENT_CANDIDATE_MIN_REACTIVATIONS,
            EMERGENT_CANDIDATE_MIN_AGE_TICKS,
            false,
        ));
        assert!(result.is_candidate);
        assert!((result.confidence - CANDIDATE_CONFIDENCE).abs() < f32::EPSILON);
    }

    #[test]
    fn test_depth_below_threshold_not_candidate() {
        let detector = DepthThresholdEmergentDetector;
        let result = detector.detect(&input(
            EMERGENT_CANDIDATE_MIN_DEPTH - 1,
            EMERGENT_CANDIDATE_MIN_REACTIVATIONS,
            EMERGENT_CANDIDATE_MIN_AGE_TICKS,
            false,
        ));
        assert!(!result.is_candidate);
    }

    #[test]
    fn test_insufficient_reactivations_not_candidate() {
        let detector = DepthThresholdEmergentDetector;
        let result = detector.detect(&input(
            EMERGENT_CANDIDATE_MIN_DEPTH,
            EMERGENT_CANDIDATE_MIN_REACTIVATIONS - 1,
            EMERGENT_CANDIDATE_MIN_AGE_TICKS,
            false,
        ));
        assert!(!result.is_candidate);
    }

    #[test]
    fn test_too_young_not_candidate() {
        let detector = DepthThresholdEmergentDetector;
        let result = detector.detect(&input(
            EMERGENT_CANDIDATE_MIN_DEPTH,
            EMERGENT_CANDIDATE_MIN_REACTIVATIONS,
            EMERGENT_CANDIDATE_MIN_AGE_TICKS - 1,
            false,
        ));
        assert!(!result.is_candidate);
    }

    #[test]
    fn test_known_primitive_never_candidate() {
        let detector = DepthThresholdEmergentDetector;
        // All thresholds met, but already a primitive
        let result = detector.detect(&input(
            u16::MAX,
            u32::MAX,
            u64::MAX,
            true,
        ));
        assert!(!result.is_candidate);
        assert_eq!(result.confidence, 0.0);
    }
}
