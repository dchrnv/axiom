// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ConfidenceCalibrator: отображает raw_confidence → истинную вероятность правоты.
// Учится на парах (raw_confidence, was_correct) из DivergenceLog.
//
// Реализация: isotonic regression через табличное сглаживание (Platt scaling вариант).
// Простая линейная интерполяция по бинам — достаточно для 10–50K параметров модели.

use serde::{Deserialize, Serialize};

const N_BINS: usize = 20;

/// Калибратор уверенности. Один на советника.
///
/// Без данных (insufficient_data=true) возвращает raw_confidence без изменений
/// и не даёт подняться до AutoApply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceCalibrator {
    /// Накопленные пары для каждого бина: (sum_correct, count).
    bins: [(f32, u32); N_BINS],
    /// Минимум примеров в бине для доверия калибровке.
    min_samples_per_bin: u32,
    /// Суммарное число обновлений.
    total_samples: u64,
}

impl ConfidenceCalibrator {
    pub fn new() -> Self {
        Self {
            bins: [(0.0, 0); N_BINS],
            min_samples_per_bin: 10,
            total_samples: 0,
        }
    }

    /// Добавить пару (raw_confidence, was_correct) из DivergenceLog.
    pub fn update(&mut self, raw: f32, was_correct: bool) {
        let bin = self.bin_of(raw);
        self.bins[bin].0 += if was_correct { 1.0 } else { 0.0 };
        self.bins[bin].1 += 1;
        self.total_samples += 1;
    }

    /// Откалибровать уверенность.
    ///
    /// Если недостаточно данных — возвращает raw как есть (советник остаётся
    /// в Ignore/RequireConfirmation до накопления достаточной статистики).
    pub fn calibrate(&self, raw: f32) -> f32 {
        let bin = self.bin_of(raw);
        let (sum, count) = self.bins[bin];
        if count < self.min_samples_per_bin {
            return raw; // недостаточно данных — не калибруем
        }
        sum / count as f32
    }

    /// Достаточно ли данных для калибровки (хотя бы половина бинов заполнена).
    pub fn is_calibrated(&self) -> bool {
        let filled = self.bins.iter().filter(|(_, c)| *c >= self.min_samples_per_bin).count();
        filled >= N_BINS / 2
    }

    pub fn total_samples(&self) -> u64 { self.total_samples }

    fn bin_of(&self, raw: f32) -> usize {
        let clamped = raw.clamp(0.0, 1.0 - 1e-7);
        (clamped * N_BINS as f32) as usize
    }
}

impl Default for ConfidenceCalibrator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrator_no_data_passthrough() {
        let cal = ConfidenceCalibrator::new();
        let out = cal.calibrate(0.8);
        assert!((out - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_calibrator_learns_correctness() {
        let mut cal = ConfidenceCalibrator::new();
        // Модель говорит 0.9, но права только в 50% случаев
        for i in 0..20 {
            cal.update(0.9, i % 2 == 0);
        }
        let calibrated = cal.calibrate(0.9);
        // Должно быть ближе к 0.5 чем к 0.9
        assert!(calibrated < 0.7, "calibrated={calibrated}");
        assert!(calibrated > 0.3, "calibrated={calibrated}");
    }

    #[test]
    fn test_not_calibrated_without_data() {
        let cal = ConfidenceCalibrator::new();
        assert!(!cal.is_calibrated());
    }
}
