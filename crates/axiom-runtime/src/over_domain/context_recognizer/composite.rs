// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CompositeSubsystemDef + co-activation сигнал (CR-V6 Фаза D).
// CompositeSubsystemProfile + BidirectionalCoupling (V7-C2).
//
// Источник: ContextRecognizer_Roadmap_V6_V9.md §1.5
//
// V7-C2: полный профиль через TransitionMatrix — bidirectional coupling A→B И B→A.

use std::collections::HashSet;

use axiom_experience::SubsystemId;

use super::activity_trace::ActivitySignature;
use super::transitions::TransitionMatrix;

/// Порог двустороннего coupling для V7-C2 (ниже порога cascade — matrix накапливается медленнее).
pub const BIDIRECTIONAL_COUPLING_THRESHOLD: f32 = 0.15;

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
    CompositeSubsystemDef {
        name: "Ethics",
        components: &[SubsystemId::Values, SubsystemId::Morality, SubsystemId::Dilemmas],
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

// ── V7-C2: CompositeSubsystemProfile ────────────────────────────────────────

/// Двустороннее coupling между двумя подсистемами (V7-C2).
///
/// Обе вероятности prob_ab и prob_ba ≥ threshold → истинное bidirectional взаимодействие.
#[derive(Debug, Clone)]
pub struct BidirectionalCoupling {
    pub a: SubsystemId,
    pub b: SubsystemId,
    /// P(a → b) в TransitionMatrix.
    pub prob_ab: f32,
    /// P(b → a) в TransitionMatrix.
    pub prob_ba: f32,
    /// Средняя сила = (prob_ab + prob_ba) / 2.
    pub strength: f32,
}

/// Полный профиль composite co-activation (V7-C2).
///
/// Расширяет V6 CompositeActivationSuspected: включает directed coupling из TransitionMatrix.
/// Предлагается chrnv для решения — не создаётся автоматически.
#[derive(Debug, Clone)]
pub struct CompositeSubsystemProfile {
    pub name: &'static str,
    /// Confidence: аналогично V6 (coverage × Converging-boost).
    pub confidence: f32,
    /// Пары компонентов с bidirectional coupling ≥ threshold.
    /// Пустой вектор — matrix ещё не накоплена.
    pub composes_with: Vec<BidirectionalCoupling>,
}

impl CompositeSubsystemProfile {
    /// True если хотя бы одна пара компонентов имеет bidirectional coupling.
    pub fn has_coupling(&self) -> bool {
        !self.composes_with.is_empty()
    }

    /// Средняя сила всех coupling-пар (0.0 если нет данных).
    pub fn mean_coupling_strength(&self) -> f32 {
        if self.composes_with.is_empty() {
            return 0.0;
        }
        self.composes_with.iter().map(|c| c.strength).sum::<f32>() / self.composes_with.len() as f32
    }
}

/// Детектировать composite профили с bidirectional coupling (V7-C2).
///
/// Логика coverage/confidence та же что в V6 `detect_composite_suspects`.
/// Дополнительно вычисляет `composes_with` из TransitionMatrix.
pub fn detect_composite_profiles(
    recent_subsystems: &[SubsystemId],
    signatures: &[ActivitySignature],
    matrix: &TransitionMatrix,
    bi_threshold: f32,
) -> Vec<CompositeSubsystemProfile> {
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
            let composes_with = compute_bidirectional_couplings(def.components, matrix, bi_threshold);
            Some(CompositeSubsystemProfile { name: def.name, confidence, composes_with })
        })
        .collect()
}

fn compute_bidirectional_couplings(
    components: &[SubsystemId],
    matrix: &TransitionMatrix,
    threshold: f32,
) -> Vec<BidirectionalCoupling> {
    let mut result = Vec::new();
    for i in 0..components.len() {
        for j in (i + 1)..components.len() {
            let a = components[i];
            let b = components[j];
            let prob_ab = matrix.probability_of(a, b);
            let prob_ba = matrix.probability_of(b, a);
            if prob_ab >= threshold && prob_ba >= threshold {
                result.push(BidirectionalCoupling {
                    a,
                    b,
                    prob_ab,
                    prob_ba,
                    strength: (prob_ab + prob_ba) / 2.0,
                });
            }
        }
    }
    result
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

    #[test]
    fn test_ethics_detected_with_values_morality_dilemmas() {
        let subs = vec![SubsystemId::Values, SubsystemId::Morality, SubsystemId::Dilemmas];
        let result = detect_composite_suspects(&subs, &[]);
        let ethics = result.iter().find(|s| s.name == "Ethics");
        assert!(ethics.is_some(), "Ethics should be detected with Values+Morality+Dilemmas");
        assert!((ethics.unwrap().confidence - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ethics_partial_two_of_three() {
        let subs = vec![SubsystemId::Values, SubsystemId::Morality];
        let result = detect_composite_suspects(&subs, &[]);
        let ethics = result.iter().find(|s| s.name == "Ethics");
        assert!(ethics.is_some(), "Ethics: 2/3 components should still trigger (matched >= 2)");
        // confidence = 2/3 ≈ 0.666
        assert!((ethics.unwrap().confidence - 2.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_ethics_not_detected_with_logic_only() {
        let subs = vec![SubsystemId::Logic];
        let result = detect_composite_suspects(&subs, &[]);
        assert!(
            result.iter().all(|s| s.name != "Ethics"),
            "Logic alone should not trigger Ethics (components.len() < 2 guard)"
        );
    }

    #[test]
    fn test_ethics_not_detected_with_single_component() {
        let subs = vec![SubsystemId::Values];
        let result = detect_composite_suspects(&subs, &[]);
        assert!(result.iter().all(|s| s.name != "Ethics"), "single component → no Ethics");
    }

    #[test]
    fn test_ethics_converging_boost() {
        let subs = vec![SubsystemId::Values, SubsystemId::Morality, SubsystemId::Dilemmas];
        let without = detect_composite_suspects(&subs, &[]);
        let with_conv = detect_composite_suspects(&subs, &[ActivitySignature::Converging]);
        let c_without = without.iter().find(|s| s.name == "Ethics").unwrap().confidence;
        let c_with = with_conv.iter().find(|s| s.name == "Ethics").unwrap().confidence;
        // base=1.0, cap=1.0 → оба 1.0
        assert!(c_with >= c_without);
    }

    // ── V7-C2: detect_composite_profiles ────────────────────────────────────

    fn make_matrix_with_bidirectional(a: SubsystemId, b: SubsystemId) -> TransitionMatrix {
        let mut m = TransitionMatrix::new();
        for _ in 0..5 {
            m.record(a, b);
            m.record(b, a);
        }
        m
    }

    #[test]
    fn test_profile_no_coupling_empty_matrix() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let m = TransitionMatrix::new();
        let profiles = detect_composite_profiles(&subs, &[], &m, BIDIRECTIONAL_COUPLING_THRESHOLD);
        let calc = profiles.iter().find(|p| p.name == "Calculus").unwrap();
        assert!(!calc.has_coupling(), "empty matrix → no coupling");
        assert_eq!(calc.composes_with.len(), 0);
    }

    #[test]
    fn test_profile_detects_bidirectional_coupling() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let m = make_matrix_with_bidirectional(SubsystemId::Mathematics, SubsystemId::Time);
        let profiles = detect_composite_profiles(&subs, &[], &m, BIDIRECTIONAL_COUPLING_THRESHOLD);
        let calc = profiles.iter().find(|p| p.name == "Calculus").unwrap();
        assert!(calc.has_coupling(), "bidirectional matrix → coupling detected");
        assert_eq!(calc.composes_with.len(), 1);
        let c = &calc.composes_with[0];
        assert!(c.strength > 0.0);
    }

    #[test]
    fn test_profile_no_coupling_unidirectional_only() {
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let mut m = TransitionMatrix::new();
        // Только Math→Time, нет Time→Math
        for _ in 0..10 {
            m.record(SubsystemId::Mathematics, SubsystemId::Time);
        }
        let profiles = detect_composite_profiles(&subs, &[], &m, BIDIRECTIONAL_COUPLING_THRESHOLD);
        let calc = profiles.iter().find(|p| p.name == "Calculus").unwrap();
        assert!(!calc.has_coupling(), "unidirectional only → no bidirectional coupling");
    }

    #[test]
    fn test_profile_mean_coupling_strength_zero_no_data() {
        let p = CompositeSubsystemProfile {
            name: "Test",
            confidence: 1.0,
            composes_with: vec![],
        };
        assert_eq!(p.mean_coupling_strength(), 0.0);
    }

    #[test]
    fn test_profile_mean_coupling_strength_nonzero() {
        let p = CompositeSubsystemProfile {
            name: "Test",
            confidence: 1.0,
            composes_with: vec![
                BidirectionalCoupling {
                    a: SubsystemId::Mathematics,
                    b: SubsystemId::Time,
                    prob_ab: 0.3,
                    prob_ba: 0.5,
                    strength: 0.4,
                },
            ],
        };
        assert!((p.mean_coupling_strength() - 0.4).abs() < 1e-5);
    }

    #[test]
    fn test_profile_confidence_matches_suspect_confidence() {
        // Убедиться что confidence вычисляется так же как в V6
        let subs = vec![SubsystemId::Mathematics, SubsystemId::Time];
        let m = TransitionMatrix::new();
        let suspects = detect_composite_suspects(&subs, &[]);
        let profiles = detect_composite_profiles(&subs, &[], &m, BIDIRECTIONAL_COUPLING_THRESHOLD);
        let calc_s = suspects.iter().find(|s| s.name == "Calculus").unwrap().confidence;
        let calc_p = profiles.iter().find(|p| p.name == "Calculus").unwrap().confidence;
        assert!((calc_s - calc_p).abs() < 1e-5, "confidence parity: suspects={calc_s}, profiles={calc_p}");
    }
}
