// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// CognitiveProfile — когнитивная ориентация агента.
//
// Задаёт веса доверия по октантам: OctantCorrection advisory с confidence=0.5
// для «важного» октанта (weight=2.0) будет воспринят как 1.0 ≥ min_confidence.
//
// Ортогонален TrustConfig:
//   TrustConfig  = кому доверять (по источнику и типу advisory)
//   CognitiveProfile = куда смотреть (по октанту)
//
// Источник: docs/guides/NeuralAdvisor_V2_Plan.md Фаза 4; DEFERRED.md → PROFILE-01
//           docs/architecture/OverDomainArbiter_V2_0.md §4 (PROFILE-01)

use std::path::Path;

use serde::Deserialize;

/// YAML-схема для config/profiles/*.yaml
#[derive(Deserialize)]
struct ProfileYaml {
    #[allow(dead_code)]
    name: Option<String>,
    octant_weights: [f32; 8],
}

/// Когнитивный профиль: мультипликаторы confidence per-octant.
///
/// Инициализируются 1.0 (нейтральный профиль).
/// Обновляются online через `update()` по исходам advisory от Arbiter.
#[derive(Debug, Clone)]
pub struct CognitiveProfile {
    /// `octant_weights[i]` — мультипликатор для октанта i (0..7).
    /// Диапазон: [WEIGHT_MIN, WEIGHT_MAX].
    pub octant_weights: [f32; 8],
}

impl CognitiveProfile {
    pub const WEIGHT_MIN: f32 = 0.5;
    pub const WEIGHT_MAX: f32 = 2.0;
    pub const LEARNING_RATE: f32 = 0.05;

    /// Создать с явными весами.
    pub fn with_weights(weights: [f32; 8]) -> Self {
        let clamped = weights.map(|w| w.clamp(Self::WEIGHT_MIN, Self::WEIGHT_MAX));
        Self { octant_weights: clamped }
    }

    /// V2: загрузить профиль из YAML-файла (config/profiles/*.yaml).
    /// Клампирует веса в [WEIGHT_MIN, WEIGHT_MAX].
    pub fn from_yaml(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: ProfileYaml = serde_yaml::from_str(&content)?;
        Ok(Self::with_weights(parsed.octant_weights))
    }

    /// V2: загрузить из YAML или вернуть default при ошибке.
    pub fn from_yaml_or_default(path: &Path) -> Self {
        Self::from_yaml(path).unwrap_or_default()
    }

    /// Применить мультипликатор октанта к raw confidence.
    /// Результат зажат в [0.0, 1.0].
    pub fn scale_confidence(&self, octant_idx: usize, raw_confidence: f32) -> f32 {
        let idx = octant_idx.min(7);
        (raw_confidence * self.octant_weights[idx]).min(1.0)
    }

    /// Обновить вес октанта по исходу advisory.
    /// Accepted (Applied/Confirmed) → увеличить. Rejected → уменьшить.
    pub fn update(&mut self, octant_idx: usize, accepted: bool) {
        let idx = octant_idx.min(7);
        let delta = if accepted { Self::LEARNING_RATE } else { -Self::LEARNING_RATE };
        self.octant_weights[idx] =
            (self.octant_weights[idx] + delta).clamp(Self::WEIGHT_MIN, Self::WEIGHT_MAX);
    }
}

impl Default for CognitiveProfile {
    fn default() -> Self {
        Self { octant_weights: [1.0; 8] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile_unity_weights() {
        let p = CognitiveProfile::default();
        assert!(p.octant_weights.iter().all(|&w| (w - 1.0).abs() < f32::EPSILON));
    }

    #[test]
    fn test_scale_confidence_multiplied() {
        let mut p = CognitiveProfile::default();
        p.octant_weights[3] = 1.5;
        let scaled = p.scale_confidence(3, 0.5);
        assert!((scaled - 0.75).abs() < 1e-5);
    }

    #[test]
    fn test_scale_clamped_to_one() {
        let mut p = CognitiveProfile::default();
        p.octant_weights[0] = 2.0;
        let scaled = p.scale_confidence(0, 0.8);
        // 0.8 * 2.0 = 1.6 → clamped to 1.0
        assert!((scaled - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale_out_of_bounds_idx_uses_last() {
        let p = CognitiveProfile::default();
        // idx=99 → clamped to 7
        let scaled = p.scale_confidence(99, 0.6);
        assert!((scaled - 0.6).abs() < 1e-5);
    }

    #[test]
    fn test_update_accepted_increases_weight() {
        let mut p = CognitiveProfile::default();
        p.update(2, true);
        assert!(p.octant_weights[2] > 1.0);
        assert!((p.octant_weights[2] - (1.0 + CognitiveProfile::LEARNING_RATE)).abs() < 1e-5);
    }

    #[test]
    fn test_update_rejected_decreases_weight() {
        let mut p = CognitiveProfile::default();
        p.update(2, false);
        assert!(p.octant_weights[2] < 1.0);
        assert!((p.octant_weights[2] - (1.0 - CognitiveProfile::LEARNING_RATE)).abs() < 1e-5);
    }

    #[test]
    fn test_weight_clamped_to_max() {
        let mut p = CognitiveProfile::default();
        p.octant_weights[0] = CognitiveProfile::WEIGHT_MAX;
        p.update(0, true);
        assert!((p.octant_weights[0] - CognitiveProfile::WEIGHT_MAX).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weight_clamped_to_min() {
        let mut p = CognitiveProfile::default();
        p.octant_weights[0] = CognitiveProfile::WEIGHT_MIN;
        p.update(0, false);
        assert!((p.octant_weights[0] - CognitiveProfile::WEIGHT_MIN).abs() < f32::EPSILON);
    }

    #[test]
    fn test_with_weights_clamps_values() {
        let p = CognitiveProfile::with_weights([5.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        assert!((p.octant_weights[0] - CognitiveProfile::WEIGHT_MAX).abs() < f32::EPSILON);
        assert!((p.octant_weights[1] - CognitiveProfile::WEIGHT_MIN).abs() < f32::EPSILON);
    }

    #[test]
    fn test_octant_correction_scaled_by_profile_passes_threshold() {
        let mut p = CognitiveProfile::default();
        p.octant_weights[3] = 2.0;
        let effective = p.scale_confidence(3, 0.5);
        assert!(effective >= 0.60, "scaled confidence should pass min_confidence threshold");
    }

    #[test]
    fn test_from_yaml_parses_weights() {
        let yaml = "name: test\noctant_weights: [1.0, 1.2, 0.8, 1.4, 0.7, 0.9, 1.5, 0.5]";
        let parsed: ProfileYaml = serde_yaml::from_str(yaml).unwrap();
        let p = CognitiveProfile::with_weights(parsed.octant_weights);
        assert!((p.octant_weights[2] - 0.8).abs() < 1e-5);
        assert!((p.octant_weights[6] - 1.5).abs() < 1e-5);
    }

    #[test]
    fn test_from_yaml_clamps_out_of_range() {
        let yaml = "octant_weights: [5.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]";
        let parsed: ProfileYaml = serde_yaml::from_str(yaml).unwrap();
        let p = CognitiveProfile::with_weights(parsed.octant_weights);
        assert!((p.octant_weights[0] - CognitiveProfile::WEIGHT_MAX).abs() < f32::EPSILON);
        assert!((p.octant_weights[1] - CognitiveProfile::WEIGHT_MIN).abs() < f32::EPSILON);
    }

    #[test]
    fn test_from_yaml_or_default_missing_file() {
        let p = CognitiveProfile::from_yaml_or_default(std::path::Path::new("/nonexistent.yaml"));
        assert!(p.octant_weights.iter().all(|&w| (w - 1.0).abs() < f32::EPSILON));
    }
}
