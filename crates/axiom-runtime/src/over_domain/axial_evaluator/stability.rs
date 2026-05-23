// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// OctantStabilityTracker — следит за стабильностью октанта Frame.
// V2: глобальный фиксированный порог.
// V3: per-Frame адаптивный порог; on_feedback корректирует его по confirmed/rejected.
// Источник: AxialEvaluator_V3_0.md §2

use std::collections::{HashMap, VecDeque};

use axiom_experience::Octant;

pub const STABILITY_HISTORY_DEPTH: usize = 10;
/// Начальный порог стабильности — корректируется per-Frame через on_feedback.
pub const STABILITY_THRESHOLD: f32 = 0.7;
pub const STABILITY_MIN_HISTORY: usize = 5;

pub const CALIBRATION_STEP: f32 = 0.02;
pub const THRESHOLD_MIN: f32 = 0.50;
pub const THRESHOLD_MAX: f32 = 0.90;

/// Состояние стабильности одного Frame с адаптивным порогом.
#[derive(Debug)]
pub struct FrameStabilityState {
    history: VecDeque<Octant>,
    /// Текущий порог — смещается on_feedback.
    pub threshold: f32,
}

impl FrameStabilityState {
    fn new() -> Self {
        Self {
            history: VecDeque::new(),
            threshold: STABILITY_THRESHOLD,
        }
    }

    fn push(&mut self, octant: Octant) -> Option<(Octant, f32)> {
        self.history.push_back(octant);
        if self.history.len() > STABILITY_HISTORY_DEPTH {
            self.history.pop_front();
        }
        if self.history.len() < STABILITY_MIN_HISTORY {
            return None;
        }
        let (dominant, count) = most_common(&self.history);
        let confidence = count as f32 / self.history.len() as f32;
        if confidence >= self.threshold {
            self.history.clear();
            Some((dominant, confidence))
        } else {
            None
        }
    }

    fn on_feedback(&mut self, accepted: bool) {
        if accepted {
            self.threshold = (self.threshold - CALIBRATION_STEP).max(THRESHOLD_MIN);
        } else {
            self.threshold = (self.threshold + CALIBRATION_STEP).min(THRESHOLD_MAX);
        }
    }
}

/// Трекер стабильности октанта для набора Frame с per-Frame адаптивным порогом.
///
/// Вызывать `push` после каждой оценки, `on_feedback` при confirmed/rejected.
#[derive(Debug, Default)]
pub struct OctantStabilityTracker {
    states: HashMap<u32, FrameStabilityState>,
}

impl OctantStabilityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Записать новую оценку. Возвращает `Some((octant, confidence))` при достижении стабильности.
    pub fn push(&mut self, sutra_id: u32, octant: Octant) -> Option<(Octant, f32)> {
        self.states.entry(sutra_id).or_insert_with(FrameStabilityState::new).push(octant)
    }

    /// Обратная связь от Arbiter — корректирует порог Frame.
    pub fn on_feedback(&mut self, sutra_id: u32, accepted: bool) {
        if let Some(state) = self.states.get_mut(&sutra_id) {
            state.on_feedback(accepted);
        }
    }

    pub fn remove(&mut self, sutra_id: u32) {
        self.states.remove(&sutra_id);
    }

    #[cfg(test)]
    pub fn threshold_for(&self, sutra_id: u32) -> Option<f32> {
        self.states.get(&sutra_id).map(|s| s.threshold)
    }
}

fn most_common(hist: &VecDeque<Octant>) -> (Octant, usize) {
    let mut counts = [0usize; 8];
    for &o in hist {
        counts[o as usize] += 1;
    }
    let (idx, &cnt) = counts.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
    (octant_from_index(idx as u8), cnt)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_signal_below_min_history() {
        let mut t = OctantStabilityTracker::new();
        for _ in 0..STABILITY_MIN_HISTORY - 1 {
            assert!(t.push(1, Octant::CreativeAffirmation).is_none());
        }
    }

    #[test]
    fn test_stable_octant_fires() {
        let mut t = OctantStabilityTracker::new();
        for _ in 0..STABILITY_MIN_HISTORY - 1 {
            t.push(1, Octant::CreativeAffirmation);
        }
        let result = t.push(1, Octant::CreativeAffirmation);
        assert!(result.is_some());
        let (oct, conf) = result.unwrap();
        assert_eq!(oct, Octant::CreativeAffirmation);
        assert!(conf >= STABILITY_THRESHOLD);
    }

    #[test]
    fn test_history_resets_after_fire() {
        let mut t = OctantStabilityTracker::new();
        for _ in 0..STABILITY_HISTORY_DEPTH {
            t.push(1, Octant::FormalDenying);
        }
        for _ in 0..STABILITY_MIN_HISTORY - 1 {
            assert!(t.push(1, Octant::FormalDenying).is_none());
        }
    }

    #[test]
    fn test_noisy_octants_no_fire() {
        let mut t = OctantStabilityTracker::new();
        let octants = [
            Octant::CreativeAffirmation,
            Octant::HeroicFatal,
            Octant::FormalDenying,
            Octant::PassiveSentimental,
            Octant::EcstaticAffirmation,
            Octant::IdealizedConsoling,
        ];
        for i in 0..STABILITY_HISTORY_DEPTH {
            let _ = t.push(1, octants[i % octants.len()]);
        }
        assert!(t.push(1, Octant::DestructiveActivating).is_none());
    }

    #[test]
    fn test_confirmed_feedback_lowers_threshold() {
        let mut t = OctantStabilityTracker::new();
        t.push(42, Octant::HeroicFatal);
        let initial = t.threshold_for(42).unwrap();
        t.on_feedback(42, true);
        let after = t.threshold_for(42).unwrap();
        assert!(after < initial, "confirmed should lower threshold");
        assert!(after >= THRESHOLD_MIN);
    }

    #[test]
    fn test_rejected_feedback_raises_threshold() {
        let mut t = OctantStabilityTracker::new();
        t.push(42, Octant::HeroicFatal);
        let initial = t.threshold_for(42).unwrap();
        t.on_feedback(42, false);
        let after = t.threshold_for(42).unwrap();
        assert!(after > initial, "rejected should raise threshold");
        assert!(after <= THRESHOLD_MAX);
    }

    #[test]
    fn test_threshold_clamped_at_min() {
        let mut t = OctantStabilityTracker::new();
        t.push(1, Octant::CreativeAffirmation);
        for _ in 0..100 {
            t.on_feedback(1, true);
        }
        assert!((t.threshold_for(1).unwrap() - THRESHOLD_MIN).abs() < 1e-5);
    }

    #[test]
    fn test_threshold_clamped_at_max() {
        let mut t = OctantStabilityTracker::new();
        t.push(1, Octant::CreativeAffirmation);
        for _ in 0..100 {
            t.on_feedback(1, false);
        }
        assert!((t.threshold_for(1).unwrap() - THRESHOLD_MAX).abs() < 1e-5);
    }

    #[test]
    fn test_lower_threshold_fires_earlier() {
        // После confirmed-feedback порог снижается → Frame может стабилизироваться
        // при меньшей доле доминирующего октанта.
        let mut t = OctantStabilityTracker::new();
        // Заполнить: 3 из 5 — CreativeAffirmation (confidence=0.6 < default 0.7)
        t.push(1, Octant::CreativeAffirmation);
        t.push(1, Octant::HeroicFatal);
        t.push(1, Octant::CreativeAffirmation);
        t.push(1, Octant::HeroicFatal);
        // Снизить порог до 0.50
        for _ in 0..10 {
            t.on_feedback(1, true);
        }
        // 5-й push → confidence=3/5=0.6 ≥ 0.50 → должен сработать
        let result = t.push(1, Octant::CreativeAffirmation);
        assert!(result.is_some());
    }
}
