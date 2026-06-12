// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Сбор тренировочных данных для Neural Integration Этап 1.
//
// Формат: JSONL, каждая строка = TrainingExample.
// Используется для дистилляции: teacher (rule-based) → student (нейронка).
//
// Вход модели (консистентен с ReactivationDepthModel::extract_features_from_onehot):
//   features[9 × 171 = 1539] — FFT-признаки по 9 подсистемам (short9+mid33+long129).
//   Вычисляются из one-hot колец ActivityTrace через ActivityFft + Z-score.
//
// Семантика каналов (ФИКСИРОВАНА, нельзя менять без перетренировки):
//   канал 0 = Writing     (SubsystemId=1)
//   канал 1 = Mathematics (SubsystemId=2)
//   канал 2 = Logic       (SubsystemId=3)
//   канал 3 = Time        (SubsystemId=4)
//   канал 4 = Music       (SubsystemId=5)
//   канал 5 = Values      (SubsystemId=6)
//   канал 6 = Morality    (SubsystemId=7)
//   канал 7 = Abstractions(SubsystemId=8)
//   канал 8 = Dilemmas    (SubsystemId=9)

use serde::{Deserialize, Serialize};
use axiom_runtime::AxiomEngine;
use axiom_neural::{
    ActivityFft,
    reactivation_depth::{N_SUBSYSTEMS, FFT_FEATURES_PER_SUB, INPUT_SIZE},
    zscore_inplace,
};

/// Один тренировочный пример.
///
/// `features`: FFT-признаки [INPUT_SIZE=1539] — консистентны с ReactivationDepthModel.
/// `teacher`: что rule-based советник считает правильным (нормализовано в [0, 1]).
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingExample {
    pub tick: u64,

    /// FFT-признаки: [N_SUBSYSTEMS × FFT_FEATURES_PER_SUB = 1539].
    /// Вычислены из one-hot колец ActivityTrace → ActivityFft → Z-score.
    /// Консистентны с ReactivationDepthModel::extract_features_from_onehot().
    pub features: Vec<f32>,

    /// Teacher output: веса реактивации по 8 октантам (0..1).
    /// Вычислены из avg_depths: weight[i] = max(0, 1 - depth[i] / MAX_DEPTH).
    /// Высокое значение = октант нуждается в реактивации.
    pub reactivation_weights: [f32; 8],

    /// Уверенность teacher (детерминирована для rule-based, отражает силу сигнала).
    pub teacher_confidence: f32,

    /// Контекст для отладки и анализа.
    pub meta: TrainingMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingMeta {
    /// Доминантная подсистема (SubsystemId as u8).
    pub dominant_subsystem: u8,
    /// Доминантный октант (0..7).
    pub dominant_octant: u8,
    /// Число активных дилемм.
    pub active_dilemmas: usize,
    /// Entropy gradient из ActivityDynamics.
    pub entropy_gradient: f32,
    /// Dominant persistence.
    pub dominant_persistence: f32,
    /// Число Experience traces.
    pub experience_traces: usize,
}

/// Максимальная глубина (из REACT_MAX_DEPTH в depth.rs).
const MAX_DEPTH: f32 = 3000.0;

impl TrainingExample {
    /// Снять тренировочный пример из текущего состояния движка.
    /// Вызывается каждые N тиков в OBS runner.
    pub fn capture(engine: &AxiomEngine, tick: u64) -> Self {
        let cr = &engine.context_recognizer;
        let at = cr.activity_trace_snapshot();

        // One-hot кольца → FFT-признаки (консистентно с ReactivationDepthModel)
        let (short_oh, mid_oh, long_oh) = at.extract_onehot_rings();
        let mut afft = ActivityFft::new();
        let mut features = vec![0.0f32; INPUT_SIZE];
        let stride = FFT_FEATURES_PER_SUB;
        for ch in 0..N_SUBSYSTEMS {
            let s = &short_oh[ch * 16..(ch + 1) * 16];
            let m = &mid_oh[ch * 64..(ch + 1) * 64];
            let l = &long_oh[ch * 256..(ch + 1) * 256];
            let out = &mut features[ch * stride..(ch + 1) * stride];
            afft.compute_rings(s, m, l, out);
        }
        zscore_inplace(&mut features);

        // Avg depths по октантам из AxialEvaluator depth store
        let avg_depths = cr.depth_store().avg_depths();

        // Teacher: веса реактивации = насколько каждый октант "мелкий"
        let mut reactivation_weights = [0.0f32; 8];
        let mut max_weight = 0.0f32;
        for (i, &d) in avg_depths.iter().enumerate() {
            let w = (1.0 - (d as f32 / MAX_DEPTH)).max(0.0);
            reactivation_weights[i] = w;
            if w > max_weight { max_weight = w; }
        }

        // Confidence teacher: пропорциональна максимальному сигналу (нормировано)
        let teacher_confidence = (max_weight * 0.85).min(0.85);

        // Метаданные
        let dynamics = cr.activity_dynamics();
        let dominant_subsystem = cr.profile_store().dominant_primary_as_u8().unwrap_or(0);
        let dominant_octant = engine.axial_evaluator.storage().store()
            .most_common_octant().unwrap_or(0);

        let meta = TrainingMeta {
            dominant_subsystem,
            dominant_octant: dominant_octant as u8,
            active_dilemmas: cr.dilemma_store().active_count(),
            entropy_gradient: dynamics.entropy_gradient,
            dominant_persistence: dynamics.dominant_persistence,
            experience_traces: engine.ashti.experience().trace_count(),
        };

        TrainingExample {
            tick,
            features,
            reactivation_weights,
            teacher_confidence,
            meta,
        }
    }
}
