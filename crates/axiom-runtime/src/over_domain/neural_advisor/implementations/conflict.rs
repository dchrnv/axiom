// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Conflict resolvers: V1 rule-based + G2 pattern-learning.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §8.1, docs/ROADMAP.md §G2

use axiom_experience::Octant;

use crate::over_domain::neural_advisor::history::{AdvisoryHistoryEntry, AdvisoryHistoryOutcome};
use crate::over_domain::neural_advisor::traits::{
    ConflictAdvisorInput, ConflictDiagnosis, ConflictResolutionHint, CorpusCallosumResolver,
};

// ─── RuleBasedCorpusCallosumResolver (V1) ────────────────────────────────────

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
            (ConflictDiagnosis::BoundaryFrame, 0.80)
        }
        STRENGTH_TWO_AXES => {
            if input.frame_age_ticks < TRANSITION_AGE_TICKS {
                (ConflictDiagnosis::TransitionState, 0.65)
            } else if input.reactivation_count >= STABLE_REACTIVATION_COUNT {
                (ConflictDiagnosis::DualNature, 0.70)
            } else {
                (ConflictDiagnosis::Unresolved, 0.50)
            }
        }
        _ => {
            (ConflictDiagnosis::Unresolved, 0.50)
        }
    }
}

// ─── PatternLearningResolver (G2) ────────────────────────────────────────────

/// Минимальное число решённых (non-Pending) записей для активации обучения.
pub const MIN_SAMPLES: usize = 5;

/// Минимальная доля принятых советов (accepted / decided) для вывода DominantOctant.
const MIN_ACCEPTANCE_RATE: f32 = 0.5;

/// Заменяет RuleBasedCorpusCallosumResolver когда накоплено достаточно истории.
///
/// Учится на `ConflictAdvisorInput.history` per-Frame: если история содержит
/// ≥ MIN_SAMPLES решённых записей → выводит доминирующий октант с взвешенной
/// уверенностью. Иначе fallback на правила.
pub struct PatternLearningResolver {
    fallback: RuleBasedCorpusCallosumResolver,
}

impl PatternLearningResolver {
    pub fn new() -> Self {
        Self { fallback: RuleBasedCorpusCallosumResolver }
    }
}

impl Default for PatternLearningResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CorpusCallosumResolver for PatternLearningResolver {
    fn resolve(&self, input: &ConflictAdvisorInput) -> ConflictResolutionHint {
        if let Some(history) = &input.history {
            if let Some(hint) = learn_from_history(history) {
                return hint;
            }
        }
        self.fallback.resolve(input)
    }
}

fn learn_from_history(history: &[AdvisoryHistoryEntry]) -> Option<ConflictResolutionHint> {
    let decided: Vec<&AdvisoryHistoryEntry> = history
        .iter()
        .filter(|e| e.outcome != AdvisoryHistoryOutcome::Pending)
        .collect();

    if decided.len() < MIN_SAMPLES {
        return None;
    }

    let accepted: Vec<&AdvisoryHistoryEntry> = decided
        .iter()
        .filter(|e| matches!(
            e.outcome,
            AdvisoryHistoryOutcome::Applied | AdvisoryHistoryOutcome::Confirmed
        ))
        .copied()
        .collect();

    let acceptance_rate = accepted.len() as f32 / decided.len() as f32;
    if acceptance_rate < MIN_ACCEPTANCE_RATE {
        return None;
    }

    // Vote on dominant accepted octant
    let mut counts = [0u32; 8];
    for entry in &accepted {
        if let Some(oct) = entry.octant_suggestion {
            counts[oct.index()] += 1;
        }
    }
    let max = *counts.iter().max().unwrap_or(&0);
    if max == 0 {
        return None;
    }
    let idx = counts.iter().position(|&c| c == max)?;
    let dominant = octant_from_idx(idx);

    // confidence = acceptance_rate × len_factor
    // len_factor: 0.5 at MIN_SAMPLES, 1.0 at 2×MIN_SAMPLES+
    let len_factor = (decided.len() as f32 / (2.0 * MIN_SAMPLES as f32)).min(1.0);
    let confidence = acceptance_rate * len_factor;

    Some(ConflictResolutionHint {
        diagnosis: ConflictDiagnosis::DominantOctant(dominant),
        confidence,
    })
}

fn octant_from_idx(i: usize) -> Octant {
    match i {
        0 => Octant::CreativeAffirmation,
        1 => Octant::EcstaticAffirmation,
        2 => Octant::HeroicFatal,
        3 => Octant::DestructiveActivating,
        4 => Octant::IdealizedConsoling,
        5 => Octant::PassiveSentimental,
        6 => Octant::FormalDenying,
        _ => Octant::SelfDestructiveApathic,
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
            history: None,
        }
    }

    fn decided_entry(oct: Octant, accepted: bool) -> AdvisoryHistoryEntry {
        AdvisoryHistoryEntry {
            computed_at_event: 1,
            octant_suggestion: Some(oct),
            octant_confidence: 0.7,
            subsystem_suggestion: None,
            subsystem_confidence: 0.0,
            outcome: if accepted {
                AdvisoryHistoryOutcome::Confirmed
            } else {
                AdvisoryHistoryOutcome::Rejected
            },
        }
    }

    // ─── RuleBasedCorpusCallosumResolver tests ────────────────────────────────

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

    // ─── PatternLearningResolver tests ───────────────────────────────────────

    #[test]
    fn test_plr_no_history_falls_back_to_rule_based() {
        let resolver = PatternLearningResolver::new();
        // history=None → RuleBased → BoundaryFrame for strength=85
        let hint = resolver.resolve(&input(STRENGTH_ONE_AXIS, 50, 5));
        assert_eq!(hint.diagnosis, ConflictDiagnosis::BoundaryFrame);
    }

    #[test]
    fn test_plr_insufficient_samples_falls_back() {
        let resolver = PatternLearningResolver::new();
        let history = vec![
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            // 4 entries < MIN_SAMPLES=5
        ];
        let mut inp = input(STRENGTH_ONE_AXIS, 50, 5);
        inp.history = Some(history);
        // Falls back to RuleBased
        assert_eq!(resolver.resolve(&inp).diagnosis, ConflictDiagnosis::BoundaryFrame);
    }

    #[test]
    fn test_plr_dominant_octant_learned() {
        let resolver = PatternLearningResolver::new();
        let history = vec![
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
            decided_entry(Octant::HeroicFatal, true),
        ];
        let mut inp = input(STRENGTH_TWO_AXES, 100, 15);
        inp.history = Some(history);
        let hint = resolver.resolve(&inp);
        assert_eq!(hint.diagnosis, ConflictDiagnosis::DominantOctant(Octant::HeroicFatal));
        assert!(hint.confidence > 0.0);
    }

    #[test]
    fn test_plr_low_acceptance_falls_back() {
        let resolver = PatternLearningResolver::new();
        // 5 entries, all rejected → acceptance_rate=0.0 < 0.5 → fallback
        let history: Vec<AdvisoryHistoryEntry> = (0..5)
            .map(|_| decided_entry(Octant::HeroicFatal, false))
            .collect();
        let mut inp = input(STRENGTH_TWO_AXES, TRANSITION_AGE_TICKS + 10, STABLE_REACTIVATION_COUNT);
        inp.history = Some(history);
        // Falls back → DualNature from RuleBased
        assert_eq!(resolver.resolve(&inp).diagnosis, ConflictDiagnosis::DualNature);
    }

    #[test]
    fn test_plr_confidence_formula_at_min_samples() {
        let resolver = PatternLearningResolver::new();
        // 5 accepted, 0 rejected → acceptance_rate=1.0, len_factor=5/10=0.5 → confidence=0.5
        let history = vec![
            decided_entry(Octant::CreativeAffirmation, true),
            decided_entry(Octant::CreativeAffirmation, true),
            decided_entry(Octant::CreativeAffirmation, true),
            decided_entry(Octant::CreativeAffirmation, true),
            decided_entry(Octant::CreativeAffirmation, true),
        ];
        let mut inp = input(STRENGTH_ONE_AXIS, 50, 5);
        inp.history = Some(history);
        let hint = resolver.resolve(&inp);
        assert!((hint.confidence - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_plr_confidence_saturates_at_double_min_samples() {
        let resolver = PatternLearningResolver::new();
        // 10 accepted → acceptance_rate=1.0, len_factor=10/10=1.0 → confidence=1.0
        let history: Vec<AdvisoryHistoryEntry> = (0..10)
            .map(|_| decided_entry(Octant::EcstaticAffirmation, true))
            .collect();
        let mut inp = input(STRENGTH_ONE_AXIS, 50, 5);
        inp.history = Some(history);
        let hint = resolver.resolve(&inp);
        assert!((hint.confidence - 1.0).abs() < 1e-4);
        assert_eq!(hint.diagnosis, ConflictDiagnosis::DominantOctant(Octant::EcstaticAffirmation));
    }

    #[test]
    fn test_plr_pending_entries_not_counted() {
        let resolver = PatternLearningResolver::new();
        // 3 accepted + 3 pending = 3 decided < MIN_SAMPLES → fallback
        let mut history: Vec<AdvisoryHistoryEntry> = (0..3)
            .map(|_| decided_entry(Octant::HeroicFatal, true))
            .collect();
        // Add 3 pending
        for _ in 0..3 {
            history.push(AdvisoryHistoryEntry {
                computed_at_event: 1,
                octant_suggestion: Some(Octant::HeroicFatal),
                octant_confidence: 0.7,
                subsystem_suggestion: None,
                subsystem_confidence: 0.0,
                outcome: AdvisoryHistoryOutcome::Pending,
            });
        }
        let mut inp = input(STRENGTH_ONE_AXIS, 50, 5);
        inp.history = Some(history);
        // Falls back to RuleBased
        assert_eq!(resolver.resolve(&inp).diagnosis, ConflictDiagnosis::BoundaryFrame);
    }
}
