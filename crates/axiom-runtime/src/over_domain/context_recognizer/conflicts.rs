// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Разрешение конфликтов подсистем. V1: stub.
// Источник: ContextRecognizer_V5_0.md

use axiom_experience::SubsystemId;

use crate::over_domain::context_recognizer::energy::SubsystemEnergy;

/// Конфликт двух активных подсистем.
#[derive(Debug, Clone)]
pub struct SubsystemConflict {
    pub primary: SubsystemId,
    pub secondary: SubsystemId,
    /// Насколько близки энергии (0.0 = идентичные, 1.0 = полностью разные)
    pub conflict_ratio: f32,
}

/// Обнаружить конфликт двух подсистем с близкими энергиями.
///
/// Конфликт возникает если вторая подсистема имеет энергию >= `threshold` от первой.
pub fn detect_conflict(energies: &[SubsystemEnergy], threshold: f32) -> Option<SubsystemConflict> {
    if energies.len() < 2 {
        return None;
    }
    let primary = &energies[0];
    let secondary = &energies[1];

    if primary.energy <= 0.0 {
        return None;
    }

    let ratio = secondary.energy / primary.energy;
    if ratio >= threshold {
        Some(SubsystemConflict {
            primary: primary.subsystem,
            secondary: secondary.subsystem,
            conflict_ratio: ratio,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn energy(s: SubsystemId, e: f32) -> SubsystemEnergy {
        SubsystemEnergy { subsystem: s, energy: e, contributing_tokens: 1 }
    }

    #[test]
    fn test_no_conflict_single_subsystem() {
        let e = vec![energy(SubsystemId::Writing, 100.0)];
        assert!(detect_conflict(&e, 0.7).is_none());
    }

    #[test]
    fn test_no_conflict_when_secondary_weak() {
        let e = vec![
            energy(SubsystemId::Writing, 100.0),
            energy(SubsystemId::Mathematics, 50.0), // ratio=0.5 < threshold=0.7
        ];
        assert!(detect_conflict(&e, 0.7).is_none());
    }

    #[test]
    fn test_conflict_when_subsystems_equal() {
        let e = vec![
            energy(SubsystemId::Writing, 100.0),
            energy(SubsystemId::Mathematics, 90.0), // ratio=0.9 >= threshold=0.7
        ];
        let c = detect_conflict(&e, 0.7);
        assert!(c.is_some());
        let c = c.unwrap();
        assert_eq!(c.primary, SubsystemId::Writing);
        assert_eq!(c.secondary, SubsystemId::Mathematics);
        assert!((c.conflict_ratio - 0.9).abs() < 1e-5);
    }

    #[test]
    fn test_no_conflict_when_energies_empty() {
        assert!(detect_conflict(&[], 0.7).is_none());
    }
}
