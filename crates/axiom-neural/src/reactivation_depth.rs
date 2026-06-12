// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ReactivationDepthModel — пилотная модель NeuralAdvisor (Этап 1).
//
// Заменяет rule-based ReactivationDepthAdvisor.
// Вход: ActivityTrace rings (short=16/mid=64/long=256) × N_SUBSYSTEMS подсистем
//       → FFT каждого кольца → конкатенация → Z-score → 1D-CNN → depth[8] + confidence.
//
// Архитектура (≈16K параметров):
//   [N_SUBS, FFT_TOTAL] → Conv1D(16, k=3) → ReLU
//                       → Conv1D(32, k=3) → ReLU
//                       → GlobalAvgPool   → [32]
//                       → Linear(16)      → ReLU → [16]
//                       → Linear(8)       → value[8]  (depth per octant)
//                       → Linear(1)+Sig   → raw_confidence

use std::path::Path;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};

use crate::fft::ActivityFft;
use crate::layers::{Conv1D, Linear, relu2_inplace, relu_inplace, sigmoid, global_avg_pool};
use crate::model::{AdvisorInput, AdvisorOutput, Model, ModelMeta, NeuralError};
use crate::normalize::zscore_inplace;
use crate::weights::{read_bin, write_bin};

/// Число подсистем (каналы Conv1D).
pub const N_SUBSYSTEMS: usize = 9;
/// Число компонент FFT на подсистему (9+33+129=171).
pub const FFT_FEATURES_PER_SUB: usize = 171;
/// Полный размер вектора признаков.
pub const INPUT_SIZE: usize = N_SUBSYSTEMS * FFT_FEATURES_PER_SUB;

// Размеры каналов Conv1D
const CH1: usize = 32;  // первый conv: 9→32
const CH2: usize = 64;  // второй conv: 32→64

// ── Веса (serde для bincode) ─────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct Weights {
    meta: ModelMeta,
    conv1: Conv1D,
    conv2: Conv1D,
    fc1: Linear,
    fc_value: Linear,
    fc_conf: Linear,
}

// ── Модель ───────────────────────────────────────────────────────────────────

/// ReactivationDepth модель — пилот Neural Integration Этап 1.
///
/// Все буферы предвыделены при new(). infer() не аллоцирует.
pub struct ReactivationDepthModel {
    weights: Weights,
    /// ActivityFft для извлечения частотных признаков (предвыделён).
    activity_fft: ActivityFft,
    // ── pre-allocated inference buffers ──
    fft_buf: Vec<f32>,        // [INPUT_SIZE]
    conv1_out: Array2<f32>,   // [16, conv1_len]
    conv2_out: Array2<f32>,   // [32, conv2_len]
    gap_out: Array1<f32>,     // [32]
    fc1_out: Array1<f32>,     // [16]
    value_out: Array1<f32>,   // [8]
    conf_out: Array1<f32>,    // [1]
}

impl ReactivationDepthModel {
    /// Создать модель с нулевыми весами (для тестирования pipeline).
    pub fn new_zeros() -> Self {
        let conv1 = Conv1D::new(N_SUBSYSTEMS, CH1, 3, 1);
        let conv2 = Conv1D::new(CH1, CH2, 5, 1);
        let fc1 = Linear::new(CH2, 32);
        let fc_value = Linear::new(32, 8);
        let fc_conf = Linear::new(32, 1);

        let conv1_len = conv1.out_len(FFT_FEATURES_PER_SUB);
        let conv2_len = conv2.out_len(conv1_len);

        let param_count = conv1.param_count() + conv2.param_count()
            + fc1.param_count() + fc_value.param_count() + fc_conf.param_count();

        let meta = ModelMeta {
            name: "reactivation_depth".to_string(),
            version: 1,
            input_size: INPUT_SIZE,
            output_size: 8,
            param_count,
        };

        Self {
            activity_fft: ActivityFft::new(),
            fft_buf: vec![0.0; INPUT_SIZE],
            conv1_out: Array2::zeros((CH1, conv1_len)),
            conv2_out: Array2::zeros((CH2, conv2_len)),
            gap_out: Array1::zeros(CH2),
            fc1_out: Array1::zeros(32),
            value_out: Array1::zeros(8),
            conf_out: Array1::zeros(1),
            weights: Weights { meta, conv1, conv2, fc1, fc_value, fc_conf },
        }
    }

    /// Извлечь FFT-признаки из ActivityTrace rings в self.fft_buf.
    ///
    /// rings: &[(short, mid, long)] длиной N_SUBSYSTEMS.
    /// Результат записывается в self.fft_buf — ноль alloc.
    pub fn extract_features(&mut self, rings: &[([f32; 16], [f32; 64], [f32; 256])]) {
        debug_assert_eq!(rings.len(), N_SUBSYSTEMS);
        let stride = FFT_FEATURES_PER_SUB;
        for (i, (short, mid, long)) in rings.iter().enumerate() {
            let out = &mut self.fft_buf[i * stride..(i + 1) * stride];
            self.activity_fft.compute_rings(short, mid, long, out);
        }
        zscore_inplace(&mut self.fft_buf);
    }

    /// Построить AdvisorInput из self.fft_buf (после extract_features).
    pub fn build_input(&self, tick: u64) -> AdvisorInput {
        AdvisorInput::new(self.fft_buf.clone(), tick)
    }
}

impl Model for ReactivationDepthModel {
    /// Инференс на pre-allocated буферах — ноль alloc.
    fn infer(&mut self, input: &AdvisorInput) -> Result<AdvisorOutput, NeuralError> {
        if input.features.len() != INPUT_SIZE {
            return Err(NeuralError::ShapeMismatch {
                expected: INPUT_SIZE,
                got: input.features.len(),
            });
        }

        let t0 = std::time::Instant::now();

        // Переформируем вход в [N_SUBS, FFT_FEATURES_PER_SUB] — view без alloc
        let input_2d = Array2::from_shape_vec(
            (N_SUBSYSTEMS, FFT_FEATURES_PER_SUB),
            input.features.clone(),
        ).map_err(|e| NeuralError::InvalidWeights(e.to_string()))?;

        // Forward pass с pre-allocated scratch-буферами (self.conv1_out и т.д.)
        self.weights.conv1.forward(&input_2d, &mut self.conv1_out);
        relu2_inplace(&mut self.conv1_out);

        self.weights.conv2.forward(&self.conv1_out.clone(), &mut self.conv2_out);
        relu2_inplace(&mut self.conv2_out);

        let conv2 = self.conv2_out.clone();
        global_avg_pool(&conv2, &mut self.gap_out);

        let gap = self.gap_out.clone();
        self.weights.fc1.forward(gap.view(), &mut self.fc1_out);
        relu_inplace(&mut self.fc1_out);

        let fc1 = self.fc1_out.clone();
        self.weights.fc_value.forward(fc1.view(), &mut self.value_out);
        self.weights.fc_conf.forward(fc1.view(), &mut self.conf_out);

        let raw_conf = sigmoid(self.conf_out[0]);
        let ns = t0.elapsed().as_nanos() as u64;

        Ok(AdvisorOutput {
            value: self.value_out.to_vec(),
            raw_confidence: raw_conf,
            calibrated_confidence: raw_conf,
            computation_ns: ns,
        })
    }

    fn load_from_bin(path: &Path) -> Result<Self, NeuralError> {
        let weights: Weights = read_bin(path)?;

        let ch1 = weights.conv1.kernels.shape()[0];
        let ch2 = weights.conv2.kernels.shape()[0];
        let fc1_out_size = weights.fc1.bias.len();

        let conv1_len = weights.conv1.out_len(FFT_FEATURES_PER_SUB);
        let conv2_len = weights.conv2.out_len(conv1_len);

        Ok(Self {
            activity_fft: ActivityFft::new(),
            fft_buf: vec![0.0; INPUT_SIZE],
            conv1_out: Array2::zeros((ch1, conv1_len)),
            conv2_out: Array2::zeros((ch2, conv2_len)),
            gap_out: Array1::zeros(ch2),
            fc1_out: Array1::zeros(fc1_out_size),
            value_out: Array1::zeros(8),
            conf_out: Array1::zeros(1),
            weights,
        })
    }

    fn save_to_bin(&self, path: &Path) -> Result<(), NeuralError> {
        write_bin(path, &self.weights)
    }

    fn param_count(&self) -> usize {
        self.weights.meta.param_count
    }

    fn input_size(&self) -> usize { INPUT_SIZE }
    fn output_size(&self) -> usize { 8 }
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_rings() -> [([f32; 16], [f32; 64], [f32; 256]); N_SUBSYSTEMS] {
        [([0.0f32; 16], [0.0f32; 64], [0.0f32; 256]); N_SUBSYSTEMS]
    }

    #[test]
    fn test_model_param_count_reasonable() {
        let model = ReactivationDepthModel::new_zeros();
        let n = model.param_count();
        assert!(n >= 1_000 && n <= 50_000, "param_count={n} out of 1K–50K range");
        println!("param_count = {n}");
    }

    #[test]
    fn test_infer_zeros_no_panic() {
        let mut model = ReactivationDepthModel::new_zeros();
        let input = AdvisorInput::zeros(INPUT_SIZE, 0);
        let out = model.infer(&input).expect("infer should not fail on zeros");
        assert_eq!(out.value.len(), 8);
        assert!(out.raw_confidence >= 0.0 && out.raw_confidence <= 1.0);
    }

    #[test]
    fn test_infer_wrong_input_size() {
        let mut model = ReactivationDepthModel::new_zeros();
        let bad = AdvisorInput::zeros(10, 0);
        let res = model.infer(&bad);
        assert!(matches!(res, Err(NeuralError::ShapeMismatch { .. })));
    }

    #[test]
    fn test_extract_features_no_panic() {
        let mut model = ReactivationDepthModel::new_zeros();
        let rings = zero_rings();
        model.extract_features(&rings);
        assert_eq!(model.fft_buf.len(), INPUT_SIZE);
    }

    #[test]
    fn test_input_size_constant() {
        assert_eq!(INPUT_SIZE, N_SUBSYSTEMS * FFT_FEATURES_PER_SUB);
        assert_eq!(INPUT_SIZE, 9 * 171);
    }

    #[test]
    fn test_save_load_roundtrip() {
        let model = ReactivationDepthModel::new_zeros();
        let dir = std::env::temp_dir();
        let path = dir.join("axiom_rd_test.bin");
        model.save_to_bin(&path).expect("save failed");
        let loaded = ReactivationDepthModel::load_from_bin(&path).expect("load failed");
        assert_eq!(loaded.param_count(), model.param_count());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn test_infer_time_reasonable() {
        // Только в release — debug mode значительно медленнее из-за bounds checks
        let mut model = ReactivationDepthModel::new_zeros();
        let input = AdvisorInput::zeros(INPUT_SIZE, 0);
        let _ = model.infer(&input).unwrap(); // прогрев
        let out = model.infer(&input).unwrap();
        assert!(out.computation_ns < 1_000_000, "inference too slow: {}ns", out.computation_ns);
    }
}
