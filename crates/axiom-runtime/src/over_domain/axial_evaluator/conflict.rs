// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Детектор конфликта Corpus Callosum: аналитический vs синтетический октант.
// Источник: AxialEvaluator_V1_0.md §6

use axiom_experience::{AxialConflict, ConflictResolution, Octant};

/// Обнаружить конфликт между аналитическим (по метрикам) и синтетическим (позиционным) октантом.
///
/// Если октанты совпадают — None.
/// Если различаются — AxialConflict с силой пропорциональной числу различающихся бит.
///
/// Сила: 1 ось = 85, 2 оси = 170, 3 оси = 255.
pub fn detect_conflict(analytic: Octant, synthetic: Octant) -> Option<AxialConflict> {
    if analytic == synthetic {
        return None;
    }

    let a = analytic.index() as u8;
    let s = synthetic.index() as u8;
    let differing_bits = (a ^ s).count_ones() as u8;
    let strength = (differing_bits * 85).min(255);

    Some(AxialConflict {
        analytic_octant: analytic,
        synthetic_octant: synthetic,
        conflict_strength: strength,
        resolution: ConflictResolution::Unresolved,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_octant_no_conflict() {
        assert!(detect_conflict(Octant::CreativeAffirmation, Octant::CreativeAffirmation).is_none());
    }

    #[test]
    fn test_one_bit_diff_strength_85() {
        // CreativeAffirmation (0b000) vs EcstaticAffirmation (0b001) → 1 bit
        let conflict = detect_conflict(Octant::CreativeAffirmation, Octant::EcstaticAffirmation);
        assert!(conflict.is_some());
        let c = conflict.unwrap();
        assert_eq!(c.conflict_strength, 85);
        assert_eq!(c.resolution, ConflictResolution::Unresolved);
    }

    #[test]
    fn test_three_bit_diff_strength_255() {
        // CreativeAffirmation (0b000) vs SelfDestructiveApathic (0b111) → 3 bits
        let conflict =
            detect_conflict(Octant::CreativeAffirmation, Octant::SelfDestructiveApathic);
        assert!(conflict.is_some());
        let c = conflict.unwrap();
        assert_eq!(c.conflict_strength, 255);
    }

    #[test]
    fn test_conflict_stores_correct_octants() {
        let c = detect_conflict(Octant::HeroicFatal, Octant::PassiveSentimental).unwrap();
        assert_eq!(c.analytic_octant, Octant::HeroicFatal);
        assert_eq!(c.synthetic_octant, Octant::PassiveSentimental);
    }
}
