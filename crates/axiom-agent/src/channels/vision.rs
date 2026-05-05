// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Vision Perceptor — изображение → токены объектов через ML-инференс
//
// Источник: файл изображения (image crate) или инжектированные детекции (тесты).
// Выход: InjectToken в LOGIC(106) или MAP(104).
//   temperature = (confidence * 255) as u8
//   position[0] = bbox x_center, position[1] = bbox y_center

use crate::ml::engine::{MLDetection, MLEngine, MLError};
use axiom_runtime::Perceptor;
use axiom_ucl::{OpCode, UclCommand};
use std::path::Path;

/// Домен по умолчанию для визуальных токенов (LOGIC=106)
pub const VISION_DEFAULT_DOMAIN: u32 = 106;

/// Конвертировать ML-детекцию в UclCommand (InjectToken).
///
/// - `target_id` = domain_id (по умолчанию LOGIC=106)
/// - `priority` = (confidence * 255) as u8 — кодирует уверенность
pub fn detection_to_command(detection: &MLDetection, domain_id: u32) -> UclCommand {
    let priority = (detection.confidence * 255.0).clamp(0.0, 255.0) as u8;
    UclCommand::new(OpCode::InjectToken, domain_id, priority, 0)
}

/// Нормализовать пиксели изображения в f32 [0.0, 1.0].
pub fn pixels_to_tensor(rgba: &[u8]) -> Vec<f32> {
    rgba.iter().map(|&p| p as f32 / 255.0).collect()
}

/// Интерпретировать плоский выход модели как список детекций.
///
/// Формат: каждые 6 float = [class_id, confidence, x, y, w, h].
/// Детекции с `confidence < threshold` отфильтровываются.
pub fn parse_detections(output: &[f32], threshold: f32) -> Vec<MLDetection> {
    output
        .chunks_exact(6)
        .filter_map(|chunk| {
            let confidence = chunk[1];
            if confidence < threshold {
                return None;
            }
            Some(MLDetection {
                class_id: chunk[0] as u32,
                confidence,
                bbox: [chunk[2], chunk[3], chunk[4], chunk[5]],
            })
        })
        .collect()
}

/// VisionPerceptor — обнаруживает объекты на изображении и генерирует InjectToken.
///
/// В тестах используйте `VisionPerceptor::from_detections()` для инжекции.
pub struct VisionPerceptor {
    engine: MLEngine,
    domain_id: u32,
    confidence_threshold: f32,
    pending: std::collections::VecDeque<UclCommand>,
}

impl VisionPerceptor {
    /// Создать перцептор с указанным ML-движком.
    pub fn new(engine: MLEngine) -> Self {
        Self {
            engine,
            domain_id: VISION_DEFAULT_DOMAIN,
            confidence_threshold: 0.5,
            pending: std::collections::VecDeque::new(),
        }
    }

    /// Установить домен назначения (по умолчанию LOGIC=106).
    pub fn with_domain(mut self, domain_id: u32) -> Self {
        self.domain_id = domain_id;
        self
    }

    /// Установить порог уверенности (по умолчанию 0.5).
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Инжектировать готовые детекции (для тестов без изображения).
    pub fn feed_detections(&mut self, detections: Vec<MLDetection>) {
        for det in detections {
            let cmd = detection_to_command(&det, self.domain_id);
            self.pending.push_back(cmd);
        }
    }

    /// Обработать файл изображения: декодировать → инференс → детекции → команды.
    pub fn process_image(&mut self, path: &Path) -> Result<usize, MLError> {
        let img = image::open(path).map_err(|e| MLError::LoadFailed(format!("image: {e}")))?;

        // Изменяем размер до входного тензора движка
        let input_size = self.engine.input_size();
        let side = if input_size > 0 {
            ((input_size / 3) as f64).sqrt() as u32
        } else {
            224
        };

        let resized = img.resize_exact(side, side, image::imageops::FilterType::Nearest);
        let rgba = resized.to_rgba8();
        let tensor = pixels_to_tensor(rgba.as_raw());

        let output = self.engine.infer(&tensor)?;
        let detections = parse_detections(&output, self.confidence_threshold);
        let count = detections.len();
        self.feed_detections(detections);
        Ok(count)
    }

    /// Число команд в очереди.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

impl Perceptor for VisionPerceptor {
    fn receive(&mut self) -> Option<UclCommand> {
        self.pending.pop_front()
    }

    fn name(&self) -> &str {
        "vision"
    }
}
