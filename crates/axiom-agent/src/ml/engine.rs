// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ML Engine — обёртка над ONNX-инференсом (tract или mock)
//
// Без feature "onnx": только mock-режим, никаких внешних зависимостей.
// С feature "onnx": реальный инференс через tract-onnx.

use std::path::Path;

/// Ошибки ML Engine.
#[derive(Debug, Clone, PartialEq)]
pub enum MLError {
    /// Файл модели не найден
    ModelNotFound(String),
    /// Ошибка загрузки модели
    LoadFailed(String),
    /// Несоответствие размера входного тензора
    /// Несоответствие размера входного тензора
    ShapeMismatch {
        /// Ожидаемый размер
        expected: usize,
        /// Полученный размер
        got: usize,
    },
    /// Ошибка инференса
    InferenceFailed(String),
    /// ONNX feature не включён (компиляция без --features onnx)
    NotEnabled,
}

impl std::fmt::Display for MLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MLError::ModelNotFound(p) => write!(f, "model not found: {p}"),
            MLError::LoadFailed(e) => write!(f, "load failed: {e}"),
            MLError::ShapeMismatch { expected, got } => {
                write!(f, "shape mismatch: expected {expected}, got {got}")
            }
            MLError::InferenceFailed(e) => write!(f, "inference failed: {e}"),
            MLError::NotEnabled => write!(f, "ONNX not enabled (compile with --features onnx)"),
        }
    }
}

impl std::error::Error for MLError {}

/// Обнаруженный объект (выход VisionPerceptor).
#[derive(Debug, Clone)]
pub struct MLDetection {
    /// Класс объекта (индекс в словаре модели)
    pub class_id: u32,
    /// Уверенность [0.0, 1.0]
    pub confidence: f32,
    /// Bounding box [x_center, y_center, width, height] в пикселях
    pub bbox: [f32; 4],
}

/// ML Engine — единый интерфейс для инференса.
///
/// # Режимы
///
/// - **Mock** (по умолчанию): возвращает заранее заданный выход. Не требует ONNX файлов.
///   Используется в тестах и разработке.
/// - **Real** (feature `onnx`): загружает ONNX модель через tract-onnx.
///
/// # Пример (mock)
///
/// ```rust
/// use axiom_agent::ml::engine::MLEngine;
///
/// let engine = MLEngine::mock(vec![3, 224, 224], vec![0.9, 0.05, 0.05]);
/// let result = engine.infer(&vec![0.0f32; 3 * 224 * 224]).unwrap();
/// assert_eq!(result.len(), 3);
/// ```
#[cfg_attr(not(feature = "onnx"), derive(Debug))]
pub enum MLEngine {
    /// Mock-режим: фиксированный выход для тестов
    Mock {
        /// Ожидаемая форма входного тензора (C×H×W или плоский размер)
        input_shape: Vec<usize>,
        /// Выход инференса (возвращается как есть)
        output: Vec<f32>,
    },
    /// Real ONNX (только при feature = "onnx")
    #[cfg(feature = "onnx")]
    Real {
        model: Box<dyn tract_onnx::prelude::Runnable>,
        input_size: usize,
        output_size: usize,
    },
}

impl MLEngine {
    /// Создать mock-движок с заданным входным размером и фиксированным выходом.
    ///
    /// `input_shape` — произведение элементов = ожидаемый размер входного среза.
    pub fn mock(input_shape: Vec<usize>, output: Vec<f32>) -> Self {
        MLEngine::Mock {
            input_shape,
            output,
        }
    }

    /// Загрузить ONNX модель из файла.
    ///
    /// Требует feature `onnx`. Без него всегда возвращает `Err(MLError::NotEnabled)`.
    pub fn load(path: &Path) -> Result<Self, MLError> {
        if !path.exists() {
            return Err(MLError::ModelNotFound(path.display().to_string()));
        }

        #[cfg(feature = "onnx")]
        {
            use tract_onnx::prelude::*;
            let model = tract_onnx::onnx()
                .model_for_path(path)
                .map_err(|e| MLError::LoadFailed(e.to_string()))?
                .into_optimized()
                .map_err(|e| MLError::LoadFailed(e.to_string()))?
                .into_runnable()
                .map_err(|e| MLError::LoadFailed(e.to_string()))?;
            // Input/output sizes would be derived from model facts here
            Ok(MLEngine::Real {
                model: Box::new(model),
                input_size: 0, // populated from model.input_fact(0)
                output_size: 0,
            })
        }

        #[cfg(not(feature = "onnx"))]
        {
            let _ = path;
            Err(MLError::NotEnabled)
        }
    }

    /// Запустить инференс на входном тензоре.
    ///
    /// `input` должен соответствовать `input_size()`.
    pub fn infer(&self, input: &[f32]) -> Result<Vec<f32>, MLError> {
        match self {
            MLEngine::Mock {
                input_shape,
                output,
            } => {
                let expected: usize = input_shape.iter().product();
                if expected > 0 && input.len() != expected {
                    return Err(MLError::ShapeMismatch {
                        expected,
                        got: input.len(),
                    });
                }
                Ok(output.clone())
            }

            #[cfg(feature = "onnx")]
            MLEngine::Real {
                model, input_size, ..
            } => {
                use tract_onnx::prelude::*;
                if *input_size > 0 && input.len() != *input_size {
                    return Err(MLError::ShapeMismatch {
                        expected: *input_size,
                        got: input.len(),
                    });
                }
                let tensor = tract_ndarray::Array::from_shape_vec(
                    tract_ndarray::IxDyn(&[1, input.len()]),
                    input.to_vec(),
                )
                .map_err(|e| MLError::InferenceFailed(e.to_string()))?
                .into();
                let result = model
                    .run(tvec!(TValue::from(tensor)))
                    .map_err(|e| MLError::InferenceFailed(e.to_string()))?;
                let arr = result[0]
                    .to_array_view::<f32>()
                    .map_err(|e| MLError::InferenceFailed(e.to_string()))?;
                Ok(arr.iter().copied().collect())
            }
        }
    }

    /// Ожидаемый размер входного тензора (произведение всех измерений).
    pub fn input_size(&self) -> usize {
        match self {
            MLEngine::Mock { input_shape, .. } => input_shape.iter().product(),
            #[cfg(feature = "onnx")]
            MLEngine::Real { input_size, .. } => *input_size,
        }
    }

    /// Форма входного тензора.
    pub fn input_shape(&self) -> &[usize] {
        match self {
            MLEngine::Mock { input_shape, .. } => input_shape,
            #[cfg(feature = "onnx")]
            MLEngine::Real { .. } => &[],
        }
    }

    /// Размер выходного тензора.
    pub fn output_size(&self) -> usize {
        match self {
            MLEngine::Mock { output, .. } => output.len(),
            #[cfg(feature = "onnx")]
            MLEngine::Real { output_size, .. } => *output_size,
        }
    }
}
