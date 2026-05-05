// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Audio Perceptor — аудио → токены речи в SUTRA(100)
//
// VAD: простой энергетический детектор (RMS по фреймам).
// Выход: InjectToken в SUTRA(100), temperature = energy * 255.

use axiom_runtime::Perceptor;
use axiom_ucl::{OpCode, UclCommand};
use std::path::Path;

/// Домен для аудио-токенов (SUTRA=100)
pub const AUDIO_DEFAULT_DOMAIN: u32 = 100;

/// Размер фрейма VAD (сэмплы)
pub const VAD_FRAME_SIZE: usize = 512;

/// Вычислить RMS энергию фрейма [0.0, 1.0].
///
/// Принимает i16 сэмплы (стандарт WAV).
pub fn frame_rms(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f64 = samples
        .iter()
        .map(|&s| {
            let f = s as f64 / i16::MAX as f64;
            f * f
        })
        .sum();
    (sum_sq / samples.len() as f64).sqrt() as f32
}

/// Результат VAD-анализа одного фрейма.
#[derive(Debug, Clone)]
pub struct VadFrame {
    /// Индекс фрейма
    pub frame_idx: usize,
    /// RMS энергия [0.0, 1.0]
    pub energy: f32,
    /// Превышает ли порог речевой активности
    pub is_speech: bool,
}

/// Разбить аудио на фреймы и применить VAD.
pub fn analyze_frames(samples: &[i16], threshold: f32) -> Vec<VadFrame> {
    samples
        .chunks(VAD_FRAME_SIZE)
        .enumerate()
        .map(|(idx, chunk)| {
            let energy = frame_rms(chunk);
            VadFrame {
                frame_idx: idx,
                energy,
                is_speech: energy >= threshold,
            }
        })
        .collect()
}

/// Конвертировать VAD-фрейм в UclCommand (InjectToken в SUTRA).
///
/// `temperature` = (energy * 255) as u8 — кодирует интенсивность речи.
pub fn vad_frame_to_command(frame: &VadFrame, domain_id: u32) -> UclCommand {
    let priority = (frame.energy * 255.0).clamp(0.0, 255.0) as u8;
    UclCommand::new(OpCode::InjectToken, domain_id, priority, 0)
}

/// AudioPerceptor — детектирует речь в аудио и генерирует InjectToken.
///
/// В тестах: `AudioPerceptor::from_samples()` для инжекции синтетических данных.
/// В prod: `process_wav()` для чтения WAV файла.
pub struct AudioPerceptor {
    domain_id: u32,
    energy_threshold: f32,
    pending: std::collections::VecDeque<UclCommand>,
}

#[allow(clippy::new_without_default)]
impl AudioPerceptor {
    /// Создать перцептор с настройками по умолчанию.
    pub fn new() -> Self {
        Self {
            domain_id: AUDIO_DEFAULT_DOMAIN,
            energy_threshold: 0.01, // ~1% от максимальной громкости
            pending: std::collections::VecDeque::new(),
        }
    }

    /// Установить домен назначения.
    pub fn with_domain(mut self, domain_id: u32) -> Self {
        self.domain_id = domain_id;
        self
    }

    /// Установить порог энергии для VAD.
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.energy_threshold = threshold;
        self
    }

    /// Обработать сэмплы напрямую (для тестов без WAV файла).
    pub fn feed_samples(&mut self, samples: &[i16]) -> usize {
        let frames = analyze_frames(samples, self.energy_threshold);
        let speech_frames: Vec<_> = frames.iter().filter(|f| f.is_speech).collect();
        let count = speech_frames.len();
        for frame in speech_frames {
            let cmd = vad_frame_to_command(frame, self.domain_id);
            self.pending.push_back(cmd);
        }
        count
    }

    /// Читать WAV файл и обработать речевые сегменты.
    pub fn process_wav(&mut self, path: &Path) -> Result<usize, String> {
        let mut reader = hound::WavReader::open(path).map_err(|e| format!("hound: {e}"))?;

        let samples: Vec<i16> = match reader.spec().sample_format {
            hound::SampleFormat::Int => reader.samples::<i16>().filter_map(|s| s.ok()).collect(),
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .filter_map(|s| s.ok())
                .map(|f| (f * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16)
                .collect(),
        };

        Ok(self.feed_samples(&samples))
    }

    /// Число команд в очереди.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

impl Perceptor for AudioPerceptor {
    fn receive(&mut self) -> Option<UclCommand> {
        self.pending.pop_front()
    }

    fn name(&self) -> &str {
        "audio"
    }
}
