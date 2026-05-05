// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// FatigueTracker + IdleTracker — компоненты DreamScheduler.
// Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md, раздел 3.

/// Веса факторов усталости. Сумма не обязана быть 255 — нормировка при делении на total.
#[derive(Debug, Clone, Copy)]
pub struct FatigueWeights {
    /// Сколько кандидатов FrameWeaver висит без кристаллизации.
    pub uncrystallized_candidates: u8,
    /// Насколько EXPERIENCE приближается к token_capacity.
    pub experience_pressure: u8,
    /// Сколько тяжёлых предложений в очереди DreamCycle.
    pub pending_heavy_proposals: u8,
    /// Насколько быстро растёт causal horizon без переработки.
    pub causal_horizon_growth_rate: u8,
}

impl Default for FatigueWeights {
    fn default() -> Self {
        // Экспериментальные дефолты V1.0 — подлежат настройке на live-данных.
        Self {
            uncrystallized_candidates: 80,
            experience_pressure: 100,
            pending_heavy_proposals: 60,
            causal_horizon_growth_rate: 30,
        }
    }
}

impl FatigueWeights {
    pub fn total(&self) -> u32 {
        self.uncrystallized_candidates as u32
            + self.experience_pressure as u32
            + self.pending_heavy_proposals as u32
            + self.causal_horizon_growth_rate as u32
    }
}

/// Снимок метрик для расчёта усталости — собирается в AxiomEngine на каждом тике WAKE.
#[derive(Debug, Default, Clone, Copy)]
pub struct FatigueSnapshot {
    pub uncrystallized_candidates: u32,
    pub experience_token_count: u32,
    pub experience_capacity: u32,
    pub pending_heavy_proposals: u32,
    /// Абсолютный прирост causal horizon с прошлого снимка.
    pub causal_horizon_delta: u64,
    /// Тиков прошло с прошлого снимка (для нормировки скорости роста).
    pub ticks_since_last_check: u32,
}

/// Вычисляет композитную оценку усталости 0..=255.
pub struct FatigueTracker {
    weights: FatigueWeights,
    last_snapshot: FatigueSnapshot,
    last_score: u8,
}

impl FatigueTracker {
    pub fn new(weights: FatigueWeights) -> Self {
        Self {
            weights,
            last_snapshot: FatigueSnapshot::default(),
            last_score: 0,
        }
    }

    pub fn update(&mut self, snapshot: FatigueSnapshot) {
        self.last_snapshot = snapshot;
        self.last_score = self.compute_score();
    }

    pub fn score(&self) -> u8 {
        self.last_score
    }

    fn compute_score(&self) -> u8 {
        let total = self.weights.total();
        if total == 0 {
            return 0;
        }

        let raw = self.candidates_factor() * self.weights.uncrystallized_candidates as u32
            + self.pressure_factor() * self.weights.experience_pressure as u32
            + self.proposals_factor() * self.weights.pending_heavy_proposals as u32
            + self.horizon_factor() * self.weights.causal_horizon_growth_rate as u32;

        ((raw / total).min(255)) as u8
    }

    // Каждый factor: 0..=255 — нормированная доля проблемы

    fn candidates_factor(&self) -> u32 {
        // каждые 10 кандидатов = +25, потолок 255
        (self
            .last_snapshot
            .uncrystallized_candidates
            .saturating_mul(25)
            / 10)
            .min(255)
    }

    fn pressure_factor(&self) -> u32 {
        let cap = self.last_snapshot.experience_capacity;
        if cap == 0 {
            return 0;
        }
        (self
            .last_snapshot
            .experience_token_count
            .saturating_mul(255)
            / cap)
            .min(255)
    }

    fn proposals_factor(&self) -> u32 {
        // каждое предложение = +30, потолок 255
        self.last_snapshot
            .pending_heavy_proposals
            .saturating_mul(30)
            .min(255)
    }

    fn horizon_factor(&self) -> u32 {
        let ticks = self.last_snapshot.ticks_since_last_check.max(1) as u64;
        let rate = self.last_snapshot.causal_horizon_delta / ticks;
        (rate.saturating_mul(5)).min(255) as u32
    }
}

/// Отслеживает количество подряд идущих тиков без внешнего intake.
#[derive(Debug, Default)]
pub struct IdleTracker {
    consecutive_idle_ticks: u32,
}

impl IdleTracker {
    pub fn update(&mut self, intake_present: bool) {
        if intake_present {
            self.consecutive_idle_ticks = 0;
        } else {
            self.consecutive_idle_ticks = self.consecutive_idle_ticks.saturating_add(1);
        }
    }

    pub fn idle_ticks(&self) -> u32 {
        self.consecutive_idle_ticks
    }

    pub fn reset(&mut self) {
        self.consecutive_idle_ticks = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_tracker_increments_without_intake() {
        let mut t = IdleTracker::default();
        t.update(false);
        t.update(false);
        assert_eq!(t.idle_ticks(), 2);
    }

    #[test]
    fn idle_tracker_resets_on_intake() {
        let mut t = IdleTracker::default();
        for _ in 0..30 {
            t.update(false);
        }
        t.update(true);
        assert_eq!(t.idle_ticks(), 0);
    }

    #[test]
    fn fatigue_zero_when_no_pressure() {
        let mut ft = FatigueTracker::new(FatigueWeights::default());
        ft.update(FatigueSnapshot {
            experience_capacity: 1000,
            ..FatigueSnapshot::default()
        });
        assert_eq!(ft.score(), 0);
    }

    #[test]
    fn fatigue_rises_with_experience_pressure() {
        let mut ft = FatigueTracker::new(FatigueWeights {
            experience_pressure: 255,
            uncrystallized_candidates: 0,
            pending_heavy_proposals: 0,
            causal_horizon_growth_rate: 0,
        });
        // EXPERIENCE заполнен наполовину
        ft.update(FatigueSnapshot {
            experience_token_count: 500,
            experience_capacity: 1000,
            ..FatigueSnapshot::default()
        });
        // pressure_factor = 500*255/1000 = 127; score = 127*255/255 = 127
        assert!(ft.score() > 100 && ft.score() < 150, "score={}", ft.score());
    }

    #[test]
    fn fatigue_weights_total() {
        let w = FatigueWeights {
            uncrystallized_candidates: 80,
            experience_pressure: 100,
            pending_heavy_proposals: 60,
            causal_horizon_growth_rate: 30,
        };
        assert_eq!(w.total(), 270);
    }
}
