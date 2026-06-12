// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Конфигурация нейронных моделей советников.
// Загружается из genome.yaml секция neural_advisor.<name>.

use serde::{Deserialize, Serialize};

/// Режим работы советника.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorMode {
    /// Старые rule-based правила (дефолт, безопасный fallback).
    #[default]
    Rule,
    /// Нейронная модель загружается из weights_path.
    Neural,
}

/// Архитектура модели ReactivationDepth.
/// Менять только если готов переобучать модель — старый .bin несовместим.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactivationDepthArch {
    /// Каналы первого Conv1D (in=N_SUBSYSTEMS → out).
    #[serde(default = "default_conv1_channels")]
    pub conv1_channels: usize,
    /// Каналы второго Conv1D.
    #[serde(default = "default_conv2_channels")]
    pub conv2_channels: usize,
    /// Ядро второго Conv1D. Первый всегда kernel=3.
    #[serde(default = "default_conv2_kernel")]
    pub conv2_kernel: usize,
    /// Размер скрытого FC-слоя после GlobalAvgPool.
    #[serde(default = "default_fc1_size")]
    pub fc1_size: usize,
}

fn default_conv1_channels() -> usize { 32 }
fn default_conv2_channels() -> usize { 64 }
fn default_conv2_kernel() -> usize { 5 }
fn default_fc1_size() -> usize { 32 }

impl Default for ReactivationDepthArch {
    fn default() -> Self {
        Self {
            conv1_channels: default_conv1_channels(),
            conv2_channels: default_conv2_channels(),
            conv2_kernel: default_conv2_kernel(),
            fc1_size: default_fc1_size(),
        }
    }
}

/// Полный конфиг советника ReactivationDepth из genome.yaml.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactivationDepthConfig {
    #[serde(default)]
    pub mode: AdvisorMode,
    /// Путь к файлу весов (относительно корня репо).
    #[serde(default = "default_weights_path")]
    pub weights_path: String,
    #[serde(default)]
    pub arch: ReactivationDepthArch,
}

fn default_weights_path() -> String {
    "models/reactivation_depth.bin".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_rule() {
        let cfg = ReactivationDepthConfig::default();
        assert_eq!(cfg.mode, AdvisorMode::Rule);
    }

    #[test]
    fn test_arch_defaults() {
        let arch = ReactivationDepthArch::default();
        assert_eq!(arch.conv1_channels, 32);
        assert_eq!(arch.conv2_channels, 64);
        assert_eq!(arch.conv2_kernel, 5);
        assert_eq!(arch.fc1_size, 32);
    }

    #[test]
    fn test_serde_roundtrip() {
        let cfg = ReactivationDepthConfig::default();
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        let back: ReactivationDepthConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(back.mode, cfg.mode);
        assert_eq!(back.arch.conv1_channels, cfg.arch.conv1_channels);
    }
}
