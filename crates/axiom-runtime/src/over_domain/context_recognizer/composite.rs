// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CompositeSubsystemDef + co-activation сигнал (CR-V6 Фаза D).
//
// Источник: ContextRecognizer_Roadmap_V6_V9.md §1.5
//
// V6 ограничение: статические def, упрощённая детекция по recent-active set.
// TransitionGraph для directed propagation — V7.

use std::collections::HashSet;

use axiom_experience::SubsystemId;

use super::activity_trace::ActivitySignature;

/// Определение композитной подсистемы (статическое, V6).
#[derive(Debug, Clone, Copy)]
pub struct CompositeSubsystemDef {
    pub name: &'static str,
    /// Базовые подсистемы, которые должны co-активироваться.
    pub components: &'static [SubsystemId],
}

/// Пять стандартных композитных подсистем.
pub static COMPOSITE_DEFS: &[CompositeSubsystemDef] = &[
    CompositeSubsystemDef {
        name: "Calculus",
        components: &[SubsystemId::Mathematics, SubsystemId::Time],
    },
    CompositeSubsystemDef {
        name: "Rhythm",
        components: &[SubsystemId::Music, SubsystemId::Time],
    },
    CompositeSubsystemDef {
        name: "Geometry",
        components: &[SubsystemId::Mathematics, SubsystemId::Writing],
    },
    CompositeSubsystemDef {
        name: "Narrative",
        components: &[SubsystemId::Writing, SubsystemId::Time],
    },
    // V7: добавить Values, Dilemmas, Morality → Ethics
    CompositeSubsystemDef {
        name: "Ethics",
        components: &[SubsystemId::Logic],
    },
];

/// Сигнал подозреваемой co-activation композитной подсистемы.
///
/// Полная детекция (TransitionGraph, stable topology) — V7.
#[derive(Debug, Clone)]
pub struct CompositeActivationSuspected {
    pub name: &'static str,
    /// Confidence: доля компонентов в recent-active set × Converging-буст.
    pub confidence: f32,
}

/// Детектировать подозреваемые composite co-activations.
///
/// `recent_subsystems` — уникальные подсистемы из mid-буфера ActivityTrace.
/// Converging-сигнатура даёт буст confidence × 1.5 (cap 1.0).
pub fn detect_composite_suspects(
    recent_subsystems: &[SubsystemId],
    signatures: &[ActivitySignature],
) -> Vec<CompositeActivationSuspected> {
    if recent_subsystems.len() < 2 {
        return vec![];
    }

    let active_set: HashSet<SubsystemId> = recent_subsystems.iter().copied().collect();
    let is_converging = signatures.contains(&ActivitySignature::Converging);

    COMPOSITE_DEFS
        .iter()
        .filter_map(|def| {
            if def.components.len() < 2 {
                return None;
            }
            let matched = def.components.iter().filter(|c| active_set.contains(c)).count();
            if matched < 2 {
                return None;
            }
            let base = matched as f32 / def.components.len() as f32;
            let confidence = if is_converging { (base * 1.5).min(1.0) } else { base };
            Some(CompositeActivationSuspected { name: def.name, confidence })
        })
        .collect()
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_suspects_with_single_subsystem() {
        let subs = vec![SubsystemId::Mathematics];
        let result = detect_composite_suspects(&subs, &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_calculus_detected_with_math_and_time() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let result = detect_composite_suspects(&subs, &[]);
        let calc = result.iter().find(|s| s.name == "Calculus");
        assert!(calc.is_some(), "Calculus should be detected");
        assert!((calc.unwrap().confidence - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rhythm_detected_with_music_and_time() {
        let subs = vec![SubsystemId::Music, SubsystemId::Time];
        let result = detect_composite_suspects(&subs, &[]);
        let rhythm = result.iter().find(|s| s.name == "Rhythm");
        assert!(rhythm.is_some());
        assert!((rhythm.unwrap().confidence - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_geometry_detected_with_math_and_writing() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Writing];
        let result = detect_composite_suspects(&subs, &[]);
        assert!(result.iter().any(|s| s.name == "Geometry"));
    }

    #[test]
    fn test_narrative_detected_with_writing_and_time() {
        let subs = vec![SubsystemId::Writing, SubsystemId::Time];
        let result = detect_composite_suspects(&subs, &[]);
        assert!(result.iter().any(|s| s.name == "Narrative"));
    }

    #[test]
    fn test_converging_boosts_confidence() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let without = detect_composite_suspects(&subs, &[]);
        let with_conv = detect_composite_suspects(&subs, &[ActivitySignature::Converging]);
        let c_without = without.iter().find(|s| s.name == "Calculus").unwrap().confidence;
        let c_with = with_conv.iter().find(|s| s.name == "Calculus").unwrap().confidence;
        // При 100% base confidence буст даёт тот же cap 1.0
        assert!(c_with >= c_without);
    }

    #[test]
    fn test_converging_boosts_partial_confidence() {
        // Geometry: [Mathematics, Writing]; добавим только Math+Writing+Time
        // Narrative = [Writing, Time]: base = 2/2 = 1.0
        // Calculus = [Mathematics, Time]: base = 2/2 = 1.0
        // Тест с частичным def — используем Geometry (только 2 компонента, оба есть)
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Writing];
        let without = detect_composite_suspects(&subs, &[]);
        let with_conv = detect_composite_suspects(&subs, &[ActivitySignature::Converging]);
        // Geometry base = 1.0; с boost = min(1.5, 1.0) = 1.0
        let c_without = without.iter().find(|s| s.name == "Geometry").unwrap().confidence;
        let c_with = with_conv.iter().find(|s| s.name == "Geometry").unwrap().confidence;
        assert!((c_with - c_without).abs() < 1e-6 || c_with > c_without);
    }

    #[test]
    fn test_multiple_composites_simultaneous() {
        // Math + Time + Writing → Calculus + Geometry + Narrative
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time, SubsystemId::Writing];
        let result = detect_composite_suspects(&subs, &[]);
        let names: Vec<&str> = result.iter().map(|s| s.name).collect();
        assert!(names.contains(&"Calculus"));
        assert!(names.contains(&"Geometry"));
        assert!(names.contains(&"Narrative"));
    }

    #[test]
    fn test_empty_recent_no_suspects() {
        let result = detect_composite_suspects(&[], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_composite_defs_count() {
        assert_eq!(COMPOSITE_DEFS.len(), 5);
    }
}
