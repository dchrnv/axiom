// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CrossModalCandidate — кандидат на создание cross-modal bond.
//
// Источник: Cross_Modal_Binding_V1_0.md §3

use axiom_experience::Modality;

/// Минимальное число ко-активаций перед предложением bond.
pub const MIN_CROSS_MODAL_COACTIVATION: u32 = 50;

/// Пара Frame из разных модальностей, обнаруженных вместе в близком временном окне.
#[derive(Debug, Clone)]
pub struct CrossModalCandidate {
    /// sutra_id Frame из первой модальности (меньший ID для канонического порядка).
    pub frame_a: u32,
    /// sutra_id Frame из второй модальности.
    pub frame_b: u32,
    pub modality_a: Modality,
    pub modality_b: Modality,
    /// Число тиков CR, когда оба Frame были одновременно активны.
    pub co_activation_count: u32,
    pub first_seen_tick: u64,
    pub last_seen_tick: u64,
    /// Достиг порога и готов к предложению в DREAM Phase.
    pub ready_for_dream: bool,
}

impl CrossModalCandidate {
    pub fn new(
        frame_a: u32,
        frame_b: u32,
        modality_a: Modality,
        modality_b: Modality,
        tick: u64,
    ) -> Self {
        // Канонический порядок: меньший sutra_id первым
        let (fa, fb, ma, mb) = if frame_a <= frame_b {
            (frame_a, frame_b, modality_a, modality_b)
        } else {
            (frame_b, frame_a, modality_b, modality_a)
        };
        Self {
            frame_a: fa,
            frame_b: fb,
            modality_a: ma,
            modality_b: mb,
            co_activation_count: 1,
            first_seen_tick: tick,
            last_seen_tick: tick,
            ready_for_dream: false,
        }
    }

    pub fn increment(&mut self, tick: u64) {
        self.co_activation_count += 1;
        self.last_seen_tick = tick;
        if self.co_activation_count >= MIN_CROSS_MODAL_COACTIVATION {
            self.ready_for_dream = true;
        }
    }

    /// Нормализованная сила (0.0..1.0) для bond strength.
    pub fn normalized_strength(&self) -> f32 {
        (self.co_activation_count as f32 / MIN_CROSS_MODAL_COACTIVATION as f32).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_order() {
        let c = CrossModalCandidate::new(200, 100, Modality::Vision, Modality::Text, 0);
        assert!(c.frame_a <= c.frame_b, "canonical order: smaller sutra_id first");
        assert_eq!(c.frame_a, 100);
        assert_eq!(c.modality_a, Modality::Text);
    }

    #[test]
    fn test_not_ready_before_threshold() {
        let mut c = CrossModalCandidate::new(1, 2, Modality::Text, Modality::Vision, 0);
        // new() → count=1. Need MIN-2 more to stay below threshold.
        for i in 1..(MIN_CROSS_MODAL_COACTIVATION - 1) {
            c.increment(i as u64);
            assert!(!c.ready_for_dream, "should not be ready at count={}", i + 1);
        }
    }

    #[test]
    fn test_ready_at_threshold() {
        let mut c = CrossModalCandidate::new(1, 2, Modality::Text, Modality::Vision, 0);
        // new() → count=1. MIN-1 more increments → count=MIN → ready.
        for i in 1..MIN_CROSS_MODAL_COACTIVATION {
            c.increment(i as u64);
        }
        assert!(c.ready_for_dream);
        assert!((c.normalized_strength() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_strength_clamps_at_one() {
        let mut c = CrossModalCandidate::new(1, 2, Modality::Text, Modality::Vision, 0);
        for i in 0..(MIN_CROSS_MODAL_COACTIVATION * 2) {
            c.increment(i as u64);
        }
        assert!(c.normalized_strength() <= 1.0);
    }
}
