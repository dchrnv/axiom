// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Три детерминированных советника глубины (V2, без ML).

use axiom_experience::Octant;

use crate::over_domain::neural_advisor::traits::{
    DepthAdvisorInput, DepthHint, DepthPredictionAdvisor,
};

#[inline]
fn octant_from_index(i: u8) -> Octant {
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

// ── Константы ────────────────────────────────────────────────────────────────

/// Минимальная глубина для «мёртвых» фреймов — чуть выше нуля, но не сброс в ноль.
pub const DEPTH_FLOOR: u16 = 50;

// ReactivationDepthAdvisor
const REACT_MIN_COUNT: u32 = 20;
const REACT_MIN_AGE: u64 = 50;
const REACT_LOW_DEPTH_THRESHOLD: u16 = 500;
const REACT_DEPTH_PER_REACTIVATION: u32 = 15;
const REACT_MAX_DEPTH: u16 = 3000;
const REACT_CONFIDENCE_DIVISOR: f32 = 50.0;
const REACT_MAX_CONFIDENCE: f32 = 0.85;

// SubsystemAffinityDepthAdvisor
const AFFINITY_LOW_DEPTH_THRESHOLD: u16 = 800;
const AFFINITY_SUGGESTED_DEPTH: u16 = 1500;
const AFFINITY_CONFIDENCE: f32 = 0.70;

// AgeDecayAdvisor
const DECAY_MIN_AGE: u64 = 200;

// ── Таблица аффинитета подсистем ─────────────────────────────────────────────
//
// SubsystemId → предпочтительный октант (индекс 0..7).
// Октанты кодируются как u8: биты XYZ (X=4, Y=2, Z=1).
// Таблица: 16 подсистем × 1 предпочтительный октант.
const SUBSYSTEM_AFFINITY: [u8; 16] = [
    0, // 0 — Neutral          → oct0 (---) низкая энергия
    4, // 1 — Cognitive        → oct4 (+--)  Apollo
    2, // 2 — Affective        → oct2 (-+-)  Eros
    6, // 3 — Conative         → oct6 (++-) Apollo+Eros
    1, // 4 — Somatic          → oct1 (--+) Will
    5, // 5 — Social           → oct5 (+-+) Apollo+Will
    3, // 6 — Aesthetic        → oct3 (-++) Eros+Will
    7, // 7 — Transcendent     → oct7 (+++) все полюса
    4, // 8 — Analytical       → oct4 (+--)
    2, // 9 — Emotional        → oct2 (-+-)
    6, // 10 — Motivational    → oct6 (++-)
    1, // 11 — Physical        → oct1 (--+)
    5, // 12 — Interpersonal   → oct5 (+-+)
    3, // 13 — Creative        → oct3 (-++)
    7, // 14 — Spiritual       → oct7 (+++)
    0, // 15 — Undefined       → oct0
];

// ── ReactivationDepthAdvisor ──────────────────────────────────────────────────

/// Углубляет фреймы, которые часто реактивируются, но всё ещё мелкие.
/// Сигнал: высокая частота реактиваций при низкой текущей глубине в основном октанте.
pub struct ReactivationDepthAdvisor;

impl DepthPredictionAdvisor for ReactivationDepthAdvisor {
    fn predict_depth(&self, input: &DepthAdvisorInput) -> Option<DepthHint> {
        if input.reactivation_count < REACT_MIN_COUNT {
            return None;
        }
        if input.frame_age_ticks < REACT_MIN_AGE {
            return None;
        }
        let oct = input.primary_octant as usize;
        if input.current_depth_per_octant[oct] >= REACT_LOW_DEPTH_THRESHOLD {
            return None;
        }
        let suggested = (input.reactivation_count * REACT_DEPTH_PER_REACTIVATION)
            .min(REACT_MAX_DEPTH as u32) as u16;
        let confidence = (input.reactivation_count as f32 / REACT_CONFIDENCE_DIVISOR)
            .min(REACT_MAX_CONFIDENCE);
        Some(DepthHint {
            target_octant: input.primary_octant,
            suggested_depth: suggested,
            confidence,
        })
    }
}

// ── SubsystemAffinityDepthAdvisor ─────────────────────────────────────────────

/// Углубляет фреймы в октант, соответствующий их подсистеме, когда глубина там мала.
pub struct SubsystemAffinityDepthAdvisor;

impl DepthPredictionAdvisor for SubsystemAffinityDepthAdvisor {
    fn predict_depth(&self, input: &DepthAdvisorInput) -> Option<DepthHint> {
        let subsystem_idx = input.subsystem as usize;
        let affinity_oct = if subsystem_idx < SUBSYSTEM_AFFINITY.len() {
            SUBSYSTEM_AFFINITY[subsystem_idx] as usize
        } else {
            0
        };
        if input.current_depth_per_octant[affinity_oct] >= AFFINITY_LOW_DEPTH_THRESHOLD {
            return None;
        }
        // Не советуем то же самое, что уже делает ReactivationDepthAdvisor
        if affinity_oct == input.primary_octant as usize
            && input.reactivation_count >= REACT_MIN_COUNT
        {
            return None;
        }
        let target_octant = octant_from_index(affinity_oct as u8);
        Some(DepthHint {
            target_octant,
            suggested_depth: AFFINITY_SUGGESTED_DEPTH,
            confidence: AFFINITY_CONFIDENCE,
        })
    }
}

// ── AgeDecayAdvisor ───────────────────────────────────────────────────────────

/// Устанавливает минимальный «пол» глубины для давно неактивных фреймов.
/// НЕ обнуляет: минимальное значение = DEPTH_FLOOR, чтобы фрейм не «пропал».
pub struct AgeDecayAdvisor;

impl DepthPredictionAdvisor for AgeDecayAdvisor {
    fn predict_depth(&self, input: &DepthAdvisorInput) -> Option<DepthHint> {
        if input.frame_age_ticks < DECAY_MIN_AGE {
            return None;
        }
        if input.reactivation_count > 0 {
            return None;
        }
        let oct = input.primary_octant as usize;
        let current = input.current_depth_per_octant[oct];
        if current <= DEPTH_FLOOR {
            return None;
        }
        // Уже достаточно «тихий» фрейм — тянем к полу, а не к нулю.
        let confidence = ((input.frame_age_ticks - DECAY_MIN_AGE) as f32 / 500.0).min(0.65);
        Some(DepthHint {
            target_octant: input.primary_octant,
            suggested_depth: DEPTH_FLOOR,
            confidence,
        })
    }
}
