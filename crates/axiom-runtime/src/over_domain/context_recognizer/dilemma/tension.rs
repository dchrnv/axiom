// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Расчёт интенсивности конфликта подсистем.
// Источник: DilemmaDetector_V2_0.md §4.1

use crate::over_domain::context_recognizer::conflicts::SubsystemConflict;

/// Порог интенсивности: ниже этого значения конфликт не становится дилеммой.
/// Соответствует 128/255 из спеки.
pub const DILEMMA_THRESHOLD: f32 = 0.5;

/// Вычислить интенсивность конфликта подсистем (0.0..1.0).
///
/// Основа: conflict_ratio — насколько равны по силе обе подсистемы.
/// При вызове из детектора conflict_ratio >= SUBSYSTEM_CONFLICT_THRESHOLD (0.75).
pub fn compute_tension_score(conflict: &SubsystemConflict) -> f32 {
    conflict.conflict_ratio.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::SubsystemId;

    fn conflict(ratio: f32) -> SubsystemConflict {
        SubsystemConflict {
            primary: SubsystemId::Mathematics,
            secondary: SubsystemId::Morality,
            conflict_ratio: ratio,
        }
    }

    #[test]
    fn test_tension_score_is_conflict_ratio() {
        let c = conflict(0.85);
        assert!((compute_tension_score(&c) - 0.85).abs() < 1e-6);
    }

    #[test]
    fn test_tension_score_clamps_above_one() {
        let c = conflict(1.2);
        assert_eq!(compute_tension_score(&c), 1.0);
    }

    #[test]
    fn test_tension_score_clamps_below_zero() {
        let c = conflict(-0.1);
        assert_eq!(compute_tension_score(&c), 0.0);
    }

    #[test]
    fn test_threshold_constant() {
        assert!((DILEMMA_THRESHOLD - 0.5).abs() < 1e-6);
    }
}
