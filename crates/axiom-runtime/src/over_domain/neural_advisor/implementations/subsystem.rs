// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AnchorVotingAdvisor — V2 реализация SubsystemAttributionAdvisor.
//
// Отличие от AnchorSet::dominant_subsystem_of():
//   - Работает с energy_weights из InterpretationProfile (накопленные за жизнь Frame),
//     а не с сырым текстом.
//   - Усиливает сигнал по глубине Frame в аффинном октанте (depth_bonus).
//   - Поддерживает dual-subsystem вывод при близких score.
//
// Источник: docs/guides/NeuralAdvisor_V2_Plan.md Фаза 3

use axiom_experience::SubsystemId;

use crate::over_domain::neural_advisor::implementations::depth::SUBSYSTEM_AFFINITY;
use crate::over_domain::neural_advisor::traits::{
    SubsystemAdvisorInput, SubsystemAttributionAdvisor, SubsystemSuggestion,
};

/// Минимальный энергетический вес для участия в голосовании.
pub const AV_MIN_ENERGY_WEIGHT: u8 = 20;

/// Если доля победителя < этого — confidence штрафуется (нет явного доминирования).
pub const AV_DOMINANCE_THRESHOLD: f32 = 0.50;

/// Если разрыв между 1-м и 2-м < этого от суммы — вернуть secondary.
pub const AV_DUAL_THRESHOLD: f32 = 0.15;

/// Нормировщик depth bonus: depth / этого = вклад в мультипликатор (cap 2.0).
const AV_DEPTH_NORM: f32 = 2000.0;

/// Порог реактиваций для полного доверия.
const AV_MIN_FULL_TRUST_REACTIVATIONS: u32 = 5;

/// Советник на основе голосования энергетических весов подсистем.
pub struct AnchorVotingAdvisor {
    min_energy_weight: u8,
    dominance_threshold: f32,
    dual_threshold: f32,
}

impl Default for AnchorVotingAdvisor {
    fn default() -> Self {
        Self {
            min_energy_weight: AV_MIN_ENERGY_WEIGHT,
            dominance_threshold: AV_DOMINANCE_THRESHOLD,
            dual_threshold: AV_DUAL_THRESHOLD,
        }
    }
}

impl SubsystemAttributionAdvisor for AnchorVotingAdvisor {
    fn suggest_subsystem(&self, input: &SubsystemAdvisorInput) -> Option<SubsystemSuggestion> {
        // Фильтрация по min_energy_weight
        let candidates: Vec<(SubsystemId, f32)> = input
            .energy_weights
            .iter()
            .filter(|(_, w)| *w >= self.min_energy_weight)
            .map(|(s, w)| {
                let depth_bonus = depth_bonus_for(*s, &input.depth_per_octant);
                (*s, *w as f32 * depth_bonus)
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let total: f32 = candidates.iter().map(|(_, s)| s).sum();
        if total <= 0.0 {
            return None;
        }

        // Сортировать по score (убывание)
        let mut sorted = candidates.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (primary_sub, primary_score) = sorted[0];

        // Penalty если нет явного доминирования
        let dominance_ratio = primary_score / total;
        let mut confidence = dominance_ratio.min(1.0);
        if dominance_ratio < self.dominance_threshold {
            confidence *= 0.7;
        }

        // Штраф за малый reactivation_count
        if input.reactivation_count < AV_MIN_FULL_TRUST_REACTIVATIONS {
            confidence *= input.reactivation_count as f32 / AV_MIN_FULL_TRUST_REACTIVATIONS as f32;
        }

        if confidence <= 0.0 {
            return None;
        }

        // Определить secondary если разрыв мал
        let secondary = if sorted.len() >= 2 {
            let (second_sub, second_score) = sorted[1];
            let gap = (primary_score - second_score) / total;
            if gap < self.dual_threshold {
                Some(second_sub)
            } else {
                None
            }
        } else {
            None
        };

        Some(SubsystemSuggestion { primary: primary_sub, secondary, confidence })
    }
}

/// depth_bonus: мультипликатор для подсистемы на основе глубины в аффинном октанте.
/// Формула: (1.0 + depth_in_affine_octant / AV_DEPTH_NORM).min(2.0)
fn depth_bonus_for(subsystem: SubsystemId, depth_per_octant: &[u16; 8]) -> f32 {
    let idx = subsystem as usize;
    let affine_oct = if idx < SUBSYSTEM_AFFINITY.len() {
        SUBSYSTEM_AFFINITY[idx] as usize
    } else {
        0
    };
    let depth = depth_per_octant[affine_oct.min(7)];
    (1.0 + depth as f32 / AV_DEPTH_NORM).min(2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::Octant;

    fn input_with_weights(
        weights: Vec<(SubsystemId, u8)>,
        depth: [u16; 8],
        reactivations: u32,
    ) -> SubsystemAdvisorInput {
        SubsystemAdvisorInput {
            sutra_id: 1,
            energy_weights: weights,
            primary_octant: Octant::CreativeAffirmation,
            depth_per_octant: depth,
            reactivation_count: reactivations,
            event_id: 1000,
        }
    }

    #[test]
    fn test_empty_weights_returns_none() {
        let adv = AnchorVotingAdvisor::default();
        let res = adv.suggest_subsystem(&input_with_weights(vec![], [0; 8], 10));
        assert!(res.is_none());
    }

    #[test]
    fn test_low_weight_filtered_out() {
        let adv = AnchorVotingAdvisor::default();
        // Все веса ниже min_energy_weight (20)
        let weights = vec![(SubsystemId::Writing, 10u8), (SubsystemId::Logic, 5u8)];
        let res = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 10));
        assert!(res.is_none());
    }

    #[test]
    fn test_single_subsystem_wins() {
        let adv = AnchorVotingAdvisor::default();
        let weights = vec![(SubsystemId::Mathematics, 200u8)];
        let res = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 10));
        assert!(res.is_some());
        let s = res.unwrap();
        assert_eq!(s.primary, SubsystemId::Mathematics);
        assert!(s.secondary.is_none());
        assert!(s.confidence > 0.0);
    }

    #[test]
    fn test_dual_subsystem_detected_when_close() {
        let adv = AnchorVotingAdvisor::default();
        // Writing=100, Logic=95 → разрыв (100-95)/195 ≈ 0.026 < 0.15 → secondary
        let weights = vec![(SubsystemId::Writing, 100u8), (SubsystemId::Logic, 95u8)];
        let res = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 10));
        assert!(res.is_some());
        let s = res.unwrap();
        assert_eq!(s.primary, SubsystemId::Writing);
        assert_eq!(s.secondary, Some(SubsystemId::Logic));
    }

    #[test]
    fn test_no_dual_subsystem_when_far_apart() {
        let adv = AnchorVotingAdvisor::default();
        // Writing=200, Logic=30 → разрыв большой → no secondary
        let weights = vec![(SubsystemId::Writing, 200u8), (SubsystemId::Logic, 30u8)];
        let res = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 10));
        assert!(res.is_some());
        assert!(res.unwrap().secondary.is_none());
    }

    #[test]
    fn test_depth_bonus_boosts_affine_subsystem() {
        let adv = AnchorVotingAdvisor::default();
        // Writing (idx=0) аффинен к SUBSYSTEM_AFFINITY[0]=0 (CreativeAffirmation)
        // Высокая глубина в oct0 должна усилить Writing
        let weights = vec![(SubsystemId::Writing, 100u8), (SubsystemId::Logic, 90u8)];

        let depth_zero = [0u16; 8];
        let mut depth_high = [0u16; 8];
        depth_high[SUBSYSTEM_AFFINITY[0] as usize] = 2000;

        let r_zero = adv.suggest_subsystem(&input_with_weights(weights.clone(), depth_zero, 10));
        let r_high = adv.suggest_subsystem(&input_with_weights(weights, depth_high, 10));

        // При depth_high Writing должен быть ещё более доминирующим
        let conf_zero = r_zero.map(|r| r.confidence).unwrap_or(0.0);
        let conf_high = r_high.map(|r| r.confidence).unwrap_or(0.0);
        assert!(conf_high >= conf_zero, "depth bonus should not reduce confidence");
    }

    #[test]
    fn test_confidence_reduced_on_low_reactivation() {
        let adv = AnchorVotingAdvisor::default();
        let weights = vec![(SubsystemId::Mathematics, 200u8)];

        let r_low = adv.suggest_subsystem(&input_with_weights(weights.clone(), [0; 8], 1));
        let r_high = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 10));

        let c_low = r_low.map(|r| r.confidence).unwrap_or(0.0);
        let c_high = r_high.map(|r| r.confidence).unwrap_or(0.0);
        assert!(c_high > c_low);
    }

    #[test]
    fn test_zero_reactivations_returns_none() {
        let adv = AnchorVotingAdvisor::default();
        let weights = vec![(SubsystemId::Values, 200u8)];
        let res = adv.suggest_subsystem(&input_with_weights(weights, [0; 8], 0));
        assert!(res.is_none());
    }
}
