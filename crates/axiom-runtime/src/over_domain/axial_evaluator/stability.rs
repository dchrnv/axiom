// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// OctantStabilityTracker — следит за стабильностью октанта Frame.
// Источник: AxialEvaluator_V2_0.md §2

use std::collections::{HashMap, VecDeque};

use axiom_experience::Octant;

pub const STABILITY_HISTORY_DEPTH: usize = 10;
pub const STABILITY_THRESHOLD: f32 = 0.7;
pub const STABILITY_MIN_HISTORY: usize = 5;

/// Трекер стабильности октанта для набора Frame.
///
/// После каждой оценки вызвать `push` — если Frame стабилен, возвращает доминирующий октант
/// и confidence, после чего история сбрасывается.
#[derive(Debug, Default)]
pub struct OctantStabilityTracker {
    history: HashMap<u32, VecDeque<Octant>>,
}

impl OctantStabilityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Записать новую оценку. Возвращает `Some((octant, confidence))` при достижении стабильности.
    pub fn push(&mut self, sutra_id: u32, octant: Octant) -> Option<(Octant, f32)> {
        let hist = self.history.entry(sutra_id).or_default();
        hist.push_back(octant);
        if hist.len() > STABILITY_HISTORY_DEPTH {
            hist.pop_front();
        }
        if hist.len() < STABILITY_MIN_HISTORY {
            return None;
        }
        let (dominant, count) = most_common(hist);
        let confidence = count as f32 / hist.len() as f32;
        if confidence >= STABILITY_THRESHOLD {
            hist.clear();
            Some((dominant, confidence))
        } else {
            None
        }
    }

    pub fn remove(&mut self, sutra_id: u32) {
        self.history.remove(&sutra_id);
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
        // После fire история пуста — следующие STABILITY_MIN_HISTORY-1 не дают сигнала
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
        for (i, &o) in octants.iter().cycle().take(STABILITY_HISTORY_DEPTH).enumerate() {
            let _ = t.push(1, octants[i % octants.len()]);
            let _ = o;
        }
        // Последний push не должен давать стабильность
        assert!(t.push(1, Octant::DestructiveActivating).is_none());
    }
}
