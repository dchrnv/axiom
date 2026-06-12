// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-neural — tiny neural models for NeuralAdvisor.
//
// Pure Rust: rustfft + ndarray/matrixmultiply. No C bindings.
// No alloc in infer() — all buffers pre-allocated at load time.
// Inference only at t%11; caller enforces 1ms timeout + fallback.

pub mod calibration;
pub mod config;
pub mod fft;
pub mod layers;
pub mod model;
pub mod normalize;
pub mod reactivation_depth;
pub mod weights;

pub use calibration::ConfidenceCalibrator;
pub use config::{AdvisorMode, ReactivationDepthArch, ReactivationDepthConfig};
pub use fft::{ActivityFft, FftFrontend};
pub use model::{AdvisorInput, AdvisorOutput, Model, ModelMeta, NeuralError};
pub use normalize::{minmax_inplace, zscore_inplace};
pub use reactivation_depth::ReactivationDepthModel;
