// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use std::path::Path;
use serde::{Deserialize, Serialize};

/// Вход советника: предвычисленные признаки из Sensorium (ActivityTrace + FFT).
/// Размер фиксирован для конкретной модели и определяется при load_from_bin().
#[derive(Debug, Clone)]
pub struct AdvisorInput {
    /// Нормализованные признаки (Z-score после FFT над ActivityTrace rings).
    pub features: Vec<f32>,
    pub tick: u64,
}

impl AdvisorInput {
    pub fn new(features: Vec<f32>, tick: u64) -> Self {
        Self { features, tick }
    }

    pub fn zeros(feature_len: usize, tick: u64) -> Self {
        Self { features: vec![0.0; feature_len], tick }
    }
}

/// Выход любой нейронной модели советника.
#[derive(Debug, Clone)]
pub struct AdvisorOutput {
    /// Значения специфичные для советника (напр. predicted_depth[8]).
    pub value: Vec<f32>,
    /// Уверенность до калибровки (0..1, из последнего слоя Sigmoid).
    pub raw_confidence: f32,
    /// Уверенность после ConfidenceCalibrator — именно её использует TrustConfig.
    pub calibrated_confidence: f32,
    /// Время инференса в наносекундах — для мониторинга таймаута.
    pub computation_ns: u64,
}

impl AdvisorOutput {
    pub fn zeros(value_len: usize) -> Self {
        Self {
            value: vec![0.0; value_len],
            raw_confidence: 0.0,
            calibrated_confidence: 0.0,
            computation_ns: 0,
        }
    }
}

/// Ошибки модели.
#[derive(Debug, Clone, PartialEq)]
pub enum NeuralError {
    InvalidWeights(String),
    ShapeMismatch { expected: usize, got: usize },
    IoError(String),
    InferTimeout,
}

impl std::fmt::Display for NeuralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeuralError::InvalidWeights(s) => write!(f, "invalid weights: {s}"),
            NeuralError::ShapeMismatch { expected, got } =>
                write!(f, "shape mismatch: expected {expected}, got {got}"),
            NeuralError::IoError(s) => write!(f, "io error: {s}"),
            NeuralError::InferTimeout => write!(f, "inference timeout"),
        }
    }
}

/// Трейт для любой нейронной модели советника.
pub trait Model: Send + Sync {
    /// Инференс. НЕТ alloc — все буферы предвыделены при load_from_bin().
    fn infer(&self, input: &AdvisorInput) -> Result<AdvisorOutput, NeuralError>;

    /// Загрузить модель из бинарного файла весов.
    fn load_from_bin(path: &Path) -> Result<Self, NeuralError> where Self: Sized;

    /// Сохранить веса в файл.
    fn save_to_bin(&self, path: &Path) -> Result<(), NeuralError>;

    /// Число параметров модели.
    fn param_count(&self) -> usize;

    /// Ожидаемый размер входного вектора.
    fn input_size(&self) -> usize;

    /// Размер выходного вектора value.
    fn output_size(&self) -> usize;
}

/// Метаданные модели — хранятся в начале .bin файла.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMeta {
    pub name: String,
    pub version: u32,
    pub input_size: usize,
    pub output_size: usize,
    pub param_count: usize,
}
