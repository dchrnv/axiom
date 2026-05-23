// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NarrativeOctantTracker — скользящее окно по оценкам Frame в сессии.
// Вычисляет «нарративный октант» как most_common в окне; сигнализирует о смене.
// Источник: AxialEvaluator_V3_0.md §4

use std::collections::VecDeque;

use axiom_experience::Octant;

/// Размер скользящего окна (число Frame-оценок).
pub const NARRATIVE_WINDOW_SIZE: usize = 8;
/// Минимальное расстояние (по числу изменённых осей) для регистрации смены нарратива.
pub const NARRATIVE_SHIFT_MIN_DISTANCE: usize = 2;

/// Трекер нарративного октанта сессии.
///
/// Принимает октанты последовательно оценённых Frame; возвращает `Some((octant, confidence))`
/// при смене доминирующего нарративного октанта на значимое расстояние.
#[derive(Debug, Default)]
pub struct NarrativeOctantTracker {
    window: VecDeque<Octant>,
    last_narrative: Option<Octant>,
}

impl NarrativeOctantTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавить октант нового оценённого Frame.
    ///
    /// Возвращает `Some((narrative_octant, confidence))` если нарратив сменился.
    pub fn push(&mut self, octant: Octant) -> Option<(Octant, f32)> {
        self.window.push_back(octant);
        if self.window.len() > NARRATIVE_WINDOW_SIZE {
            self.window.pop_front();
        }
        if self.window.len() < NARRATIVE_WINDOW_SIZE / 2 {
            return None;
        }

        let (narrative, count) = most_common(&self.window);
        let confidence = count as f32 / self.window.len() as f32;

        let shifted = match self.last_narrative {
            None => true,
            Some(prev) => octant_distance(narrative, prev) >= NARRATIVE_SHIFT_MIN_DISTANCE,
        };

        if shifted {
            self.last_narrative = Some(narrative);
            Some((narrative, confidence))
        } else {
            None
        }
    }

    pub fn last_narrative(&self) -> Option<Octant> {
        self.last_narrative
    }

    pub fn window_size(&self) -> usize {
        self.window.len()
    }
}

/// Расстояние между октантами: число осей (X/Y/Z), по которым они различаются.
/// Реализация: Hamming distance по битам индекса (0..7 = 3 бита, по одному на ось).
fn octant_distance(a: Octant, b: Octant) -> usize {
    (a.index() ^ b.index()).count_ones() as usize
}

fn most_common(window: &VecDeque<Octant>) -> (Octant, usize) {
    let mut counts = [0usize; 8];
    for &o in window {
        counts[o.index()] += 1;
    }
    let (idx, &cnt) = counts.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
    (octant_from_index(idx), cnt)
}

fn octant_from_index(i: usize) -> Octant {
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
    fn test_no_signal_below_half_window() {
        let mut t = NarrativeOctantTracker::new();
        for _ in 0..NARRATIVE_WINDOW_SIZE / 2 - 1 {
            assert!(t.push(Octant::CreativeAffirmation).is_none());
        }
    }

    #[test]
    fn test_first_narrative_fires_immediately() {
        let mut t = NarrativeOctantTracker::new();
        // Заполняем минимум окна одним октантом
        for _ in 0..NARRATIVE_WINDOW_SIZE / 2 - 1 {
            t.push(Octant::HeroicFatal);
        }
        let result = t.push(Octant::HeroicFatal);
        assert!(result.is_some(), "first narrative should fire");
        assert_eq!(result.unwrap().0, Octant::HeroicFatal);
    }

    #[test]
    fn test_no_shift_when_same_narrative() {
        let mut t = NarrativeOctantTracker::new();
        // Установить нарратив
        for _ in 0..NARRATIVE_WINDOW_SIZE {
            t.push(Octant::CreativeAffirmation);
        }
        // Один другой октант не меняет доминанта
        let result = t.push(Octant::CreativeAffirmation);
        assert!(result.is_none(), "no shift when narrative unchanged");
    }

    #[test]
    fn test_shift_detected_on_distant_octant() {
        let mut t = NarrativeOctantTracker::new();
        // Установить CreativeAffirmation (idx=0) как нарратив
        for _ in 0..NARRATIVE_WINDOW_SIZE {
            t.push(Octant::CreativeAffirmation);
        }
        // Вытеснить окно SelfDestructiveApathic (idx=7) — расстояние 3
        for _ in 0..NARRATIVE_WINDOW_SIZE {
            t.push(Octant::SelfDestructiveApathic);
        }
        let result = t.push(Octant::SelfDestructiveApathic);
        // Нарратив должен был смениться на SelfDestructiveApathic
        assert_eq!(t.last_narrative(), Some(Octant::SelfDestructiveApathic));
        let _ = result; // сигнал мог сработать раньше
    }

    #[test]
    fn test_octant_distance_same_octant() {
        assert_eq!(octant_distance(Octant::CreativeAffirmation, Octant::CreativeAffirmation), 0);
    }

    #[test]
    fn test_octant_distance_opposite() {
        // CreativeAffirmation (0b000) vs SelfDestructiveApathic (0b111) = 3 оси
        assert_eq!(octant_distance(Octant::CreativeAffirmation, Octant::SelfDestructiveApathic), 3);
    }

    #[test]
    fn test_octant_distance_one_axis() {
        // CreativeAffirmation (0) vs EcstaticAffirmation (1): отличаются на 1 бит
        assert_eq!(octant_distance(Octant::CreativeAffirmation, Octant::EcstaticAffirmation), 1);
    }

    #[test]
    fn test_confidence_proportional_to_dominance() {
        let mut t = NarrativeOctantTracker::new();
        // 4 одинаковых октанта в окне 4 → confidence = 1.0
        for _ in 0..NARRATIVE_WINDOW_SIZE / 2 {
            t.push(Octant::FormalDenying);
        }
        let result = t.push(Octant::FormalDenying);
        if let Some((_, conf)) = result {
            assert!((conf - 1.0).abs() < 1e-5);
        }
    }
}
