// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Сбор тренировочных данных для Neural Integration Этап 1.
//
// Формат: JSONL, каждая строка = TrainingExample.
// Используется для дистилляции: teacher (rule-based) → student (нейронка).
//
// Семантика входных каналов (ФИКСИРОВАНА, нельзя менять без перетренировки):
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

/// Один тренировочный пример.
///
/// `rings`: one-hot матрицы [9 подсистем × T] — dominant subsystem в каждый тик.
/// `teacher`: что rule-based советник считает правильным (нормализовано в [0, 1]).
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingExample {
    pub tick: u64,

    /// One-hot кольца. Порядок: [9, 16] short, [9, 64] mid, [9, 256] long.
    /// Плоские векторы: `short[ch * 16 + t]` = 1.0 если в момент t доминировал канал ch.
    pub short: Vec<f32>,   // len = 9 * 16  = 144
    pub mid: Vec<f32>,     // len = 9 * 64  = 576
    pub long: Vec<f32>,    // len = 9 * 256 = 2304

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

        // One-hot кольца из ActivityTrace
        let (short, mid, long) = at.extract_onehot_rings();

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
            short,
            mid,
            long,
            reactivation_weights,
            teacher_confidence,
            meta,
        }
    }
}
