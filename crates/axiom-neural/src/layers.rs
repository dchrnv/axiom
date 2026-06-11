// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Слои нейронной сети: Conv1D, GlobalAvgPool, Linear, ReLU, Sigmoid.
// Все операции на ndarray, никаких alloc в forward().

use ndarray::{Array1, Array2, Array3, ArrayView1, s};
use serde::{Deserialize, Serialize};

// ── Linear ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Linear {
    /// weights: [out, in]
    pub weights: Array2<f32>,
    /// bias: [out]
    pub bias: Array1<f32>,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        Self {
            weights: Array2::zeros((out_features, in_features)),
            bias: Array1::zeros(out_features),
        }
    }

    pub fn param_count(&self) -> usize {
        self.weights.len() + self.bias.len()
    }

    /// out = weights × input + bias
    pub fn forward(&self, input: ArrayView1<f32>, out: &mut Array1<f32>) {
        out.assign(&self.bias);
        for (o, w_row) in out.iter_mut().zip(self.weights.rows()) {
            *o += w_row.dot(&input);
        }
    }
}

// ── Conv1D ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conv1D {
    /// kernels: [out_channels, in_channels, kernel_size]
    pub kernels: Array3<f32>,
    /// bias: [out_channels]
    pub bias: Array1<f32>,
    pub stride: usize,
}

impl Conv1D {
    pub fn new(in_ch: usize, out_ch: usize, kernel_size: usize, stride: usize) -> Self {
        Self {
            kernels: Array3::zeros((out_ch, in_ch, kernel_size)),
            bias: Array1::zeros(out_ch),
            stride,
        }
    }

    pub fn param_count(&self) -> usize {
        self.kernels.len() + self.bias.len()
    }

    pub fn out_len(&self, in_len: usize) -> usize {
        let k = self.kernels.shape()[2];
        (in_len.saturating_sub(k)) / self.stride + 1
    }

    /// input:  [in_channels, in_len]
    /// output: [out_channels, out_len]  — caller предвыделяет
    pub fn forward(&self, input: &Array2<f32>, output: &mut Array2<f32>) {
        let (out_ch, in_ch, k) = (
            self.kernels.shape()[0],
            self.kernels.shape()[1],
            self.kernels.shape()[2],
        );
        let in_len = input.shape()[1];
        let out_len = self.out_len(in_len);

        for oc in 0..out_ch {
            for pos in 0..out_len {
                let start = pos * self.stride;
                let mut acc = self.bias[oc];
                for ic in 0..in_ch {
                    let kernel = self.kernels.slice(s![oc, ic, ..]);
                    let patch = input.slice(s![ic, start..start + k]);
                    acc += kernel.dot(&patch);
                }
                output[[oc, pos]] = acc;
            }
        }
    }
}

// ── Активации ─────────────────────────────────────────────────────────────────

#[inline]
pub fn relu_inplace(x: &mut Array1<f32>) {
    x.mapv_inplace(|v| v.max(0.0));
}

#[inline]
pub fn relu2_inplace(x: &mut Array2<f32>) {
    x.mapv_inplace(|v| v.max(0.0));
}

#[inline]
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

// ── GlobalAvgPool ─────────────────────────────────────────────────────────────

/// input: [channels, length] → output: [channels]
pub fn global_avg_pool(input: &Array2<f32>, output: &mut Array1<f32>) {
    let len = input.shape()[1] as f32;
    for (ch, row) in input.rows().into_iter().enumerate() {
        output[ch] = row.sum() / len;
    }
}

// ── Тесты ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_linear_zeros() {
        let layer = Linear::new(4, 2);
        let input = Array1::from(vec![1.0f32, 2.0, 3.0, 4.0]);
        let mut out = Array1::zeros(2);
        layer.forward(input.view(), &mut out);
        assert_eq!(out[0], 0.0);
        assert_eq!(out[1], 0.0);
    }

    #[test]
    fn test_conv1d_out_len() {
        let conv = Conv1D::new(1, 1, 3, 1);
        assert_eq!(conv.out_len(10), 8); // (10-3)/1+1 = 8
        let conv2 = Conv1D::new(1, 1, 5, 1);
        assert_eq!(conv2.out_len(16), 12);
    }

    #[test]
    fn test_global_avg_pool() {
        use ndarray::Array2;
        let input = Array2::from_shape_vec((2, 4), vec![
            1.0, 2.0, 3.0, 4.0,
            0.0, 0.0, 4.0, 4.0,
        ]).unwrap();
        let mut out = Array1::zeros(2);
        global_avg_pool(&input, &mut out);
        assert!((out[0] - 2.5).abs() < 1e-6);
        assert!((out[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_relu() {
        let mut x = Array1::from(vec![-1.0f32, 0.0, 2.0, -0.5]);
        relu_inplace(&mut x);
        assert_eq!(x.as_slice().unwrap(), &[0.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    fn test_sigmoid_range() {
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-6);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(-10.0) < 0.01);
    }
}
