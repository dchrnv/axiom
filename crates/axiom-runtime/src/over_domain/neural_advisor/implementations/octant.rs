// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DepthHistoryBiasAdvisor — V2 реализация OctantCorrectionAdvisor.
// Предлагает октант на основе исторических глубин Frame в SutraDepthStore.
// Источник: docs/guides/NeuralAdvisor_V2_Plan.md Фаза 2

use axiom_experience::Octant;

use crate::over_domain::neural_advisor::traits::{
    OctantAdvisorInput, OctantCorrectionAdvisor, OctantSuggestion, OctantSuggestionReason,
};

/// Минимальная глубина в октанте чтобы советник его рассматривал.
pub const DHB_MIN_DEPTH_THRESHOLD: u16 = 800;

/// Минимальное преимущество лидирующего октанта над analytic_octant.
pub const DHB_MIN_DEPTH_ADVANTAGE: u16 = 300;

/// Базовый нормировщик confidence: depth / этой константы (cap 0.85).
const DHB_CONFIDENCE_DEPTH_NORM: f32 = 3000.0;
const DHB_MAX_CONFIDENCE: f32 = 0.85;

/// Порог реактиваций для полного доверия; ниже — confidence штрафуется.
const DHB_MIN_FULL_TRUST_REACTIVATIONS: u32 = 10;

/// Советник по исторической глубине октантов.
///
/// Логика: если в SutraDepthStore у Frame есть октант с явно большей глубиной,
/// чем у текущего analytic_octant, это значит что Frame исторически «живёт» в
/// другом октанте — стоит предложить его как коррекцию.
pub struct DepthHistoryBiasAdvisor {
    min_depth_threshold: u16,
    min_depth_advantage: u16,
}

impl Default for DepthHistoryBiasAdvisor {
    fn default() -> Self {
        Self {
            min_depth_threshold: DHB_MIN_DEPTH_THRESHOLD,
            min_depth_advantage: DHB_MIN_DEPTH_ADVANTAGE,
        }
    }
}

impl OctantCorrectionAdvisor for DepthHistoryBiasAdvisor {
    fn suggest_octant(&self, input: &OctantAdvisorInput) -> Option<OctantSuggestion> {
        // Найти октант с максимальной глубиной
        let best_idx = input
            .depth_per_octant
            .iter()
            .enumerate()
            .max_by_key(|(_, &d)| d)
            .map(|(i, _)| i)?;

        let best_depth = input.depth_per_octant[best_idx];

        // Недостаточно истории
        if best_depth < self.min_depth_threshold {
            return None;
        }

        let analytic_idx = input.analytic_octant.index();
        let analytic_depth = input.depth_per_octant[analytic_idx];

        // Аналитика уже соответствует истории и нет конфликта — молчать
        if best_idx == analytic_idx && input.analytic_octant == input.synthetic_octant {
            return None;
        }

        // Преимущество лидера над аналитическим октантом
        if best_idx != analytic_idx
            && best_depth.saturating_sub(analytic_depth) < self.min_depth_advantage
        {
            return None;
        }

        // Confidence: пропорционально глубине, со штрафом за малый reactivation_count
        let base = (best_depth as f32 / DHB_CONFIDENCE_DEPTH_NORM).min(DHB_MAX_CONFIDENCE);
        let confidence = if input.reactivation_count < DHB_MIN_FULL_TRUST_REACTIVATIONS {
            base * (input.reactivation_count as f32 / DHB_MIN_FULL_TRUST_REACTIVATIONS as f32)
        } else {
            base
        };

        if confidence <= 0.0 {
            return None;
        }

        Some(OctantSuggestion {
            octant: octant_from_idx(best_idx),
            confidence,
            reason: OctantSuggestionReason::DepthHistoryBias,
        })
    }
}

#[inline]
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
    use axiom_experience::{EvaluationLevel, SubsystemId};

    fn input(
        depth: [u16; 8],
        reactivations: u32,
        analytic: Octant,
        synthetic: Octant,
    ) -> OctantAdvisorInput {
        OctantAdvisorInput {
            sutra_id: 1,
            analytic_octant: analytic,
            synthetic_octant: synthetic,
            evaluation_level: EvaluationLevel::Transcendent,
            x_positive_pole: 128,
            x_negative_pole: 100,
            y_positive_pole: 100,
            y_negative_pole: 100,
            z_positive_pole: 100,
            z_negative_pole: 100,
            primary_subsystem: SubsystemId::Unknown,
            event_id: 1000,
            depth_per_octant: depth,
            reactivation_count: reactivations,
        }
    }

    #[test]
    fn test_no_suggestion_when_all_depths_zero() {
        let adv = DepthHistoryBiasAdvisor::default();
        let res = adv.suggest_octant(&input([0; 8], 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        assert!(res.is_none());
    }

    #[test]
    fn test_no_suggestion_when_analytic_already_dominant_no_conflict() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth = [0u16; 8];
        depth[Octant::CreativeAffirmation.index()] = 2000;
        // analytic == best && analytic == synthetic → молчать
        let res = adv.suggest_octant(&input(depth, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        assert!(res.is_none());
    }

    #[test]
    fn test_suggests_historical_octant_when_dominant() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth = [0u16; 8];
        depth[Octant::HeroicFatal.index()] = 2000;
        depth[Octant::CreativeAffirmation.index()] = 500;
        let res = adv.suggest_octant(&input(depth, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        assert!(res.is_some());
        let s = res.unwrap();
        assert_eq!(s.octant, Octant::HeroicFatal);
        assert_eq!(s.reason, OctantSuggestionReason::DepthHistoryBias);
    }

    #[test]
    fn test_no_suggestion_when_advantage_insufficient() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth = [0u16; 8];
        // HeroicFatal выше но advantage < DHB_MIN_DEPTH_ADVANTAGE
        depth[Octant::HeroicFatal.index()] = 900;
        depth[Octant::CreativeAffirmation.index()] = 800;
        let res = adv.suggest_octant(&input(depth, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        assert!(res.is_none());
    }

    #[test]
    fn test_confidence_scales_with_depth() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth_low = [0u16; 8];
        depth_low[Octant::HeroicFatal.index()] = 900;
        let mut depth_high = [0u16; 8];
        depth_high[Octant::HeroicFatal.index()] = 2700;

        let r_low = adv.suggest_octant(&input(depth_low, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        let r_high = adv.suggest_octant(&input(depth_high, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));

        let c_low = r_low.map(|r| r.confidence).unwrap_or(0.0);
        let c_high = r_high.map(|r| r.confidence).unwrap_or(0.0);
        assert!(c_high > c_low, "deeper frame should have higher confidence");
    }

    #[test]
    fn test_low_reactivation_reduces_confidence() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth = [0u16; 8];
        depth[Octant::HeroicFatal.index()] = 2000;
        depth[Octant::CreativeAffirmation.index()] = 100;

        let r_low = adv.suggest_octant(&input(depth, 2, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        let r_high = adv.suggest_octant(&input(depth, 20, Octant::CreativeAffirmation, Octant::CreativeAffirmation));

        let c_low = r_low.map(|r| r.confidence).unwrap_or(0.0);
        let c_high = r_high.map(|r| r.confidence).unwrap_or(0.0);
        assert!(c_high > c_low);
    }

    #[test]
    fn test_zero_reactivations_returns_none() {
        let adv = DepthHistoryBiasAdvisor::default();
        let mut depth = [0u16; 8];
        depth[Octant::HeroicFatal.index()] = 2000;
        depth[Octant::CreativeAffirmation.index()] = 100;
        // reactivation_count == 0 → confidence = 0.0 → None
        let res = adv.suggest_octant(&input(depth, 0, Octant::CreativeAffirmation, Octant::CreativeAffirmation));
        assert!(res.is_none());
    }
}
