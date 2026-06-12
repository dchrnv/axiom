// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NeuralReactivationDepthAdvisor — нейронная реализация DepthPredictionAdvisor.
//
// Wraps ReactivationDepthModel из axiom-neural.
// Inference: update_from_trace() вызывается из NeuralAdvisor::on_tick раз в 11 тиков.
// predict_depth(): читает кешированный результат — ноль alloc, мгновенно.

use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

use axiom_neural::{
    AdvisorMode, Model, ReactivationDepthConfig, ReactivationDepthModel,
};

use crate::over_domain::context_recognizer::ActivityTrace;
use super::traits::{DepthAdvisorInput, DepthHint, DepthPredictionAdvisor};

const REACT_MAX_DEPTH: f32 = 3000.0;
const NEURAL_MIN_WEIGHT: f32 = 0.15;
const NEURAL_MAX_CONFIDENCE: f32 = 0.80;
const INFER_TIMEOUT_NS: u64 = 1_000_000; // 1ms

/// Внутреннее мутируемое состояние (за Mutex — trait требует Send+Sync).
struct Inner {
    model: ReactivationDepthModel,
    cached_weights: [f32; 8],
    last_infer_tick: u64,
    last_infer_ns: u64,
}

/// Советник глубины с двумя режимами: rule-based или нейронный.
///
/// В mode=rule — кеш обновляется из avg_depths (как teacher).
/// В mode=neural — кеш обновляется из inference модели.
pub struct NeuralReactivationDepthAdvisor {
    mode: AdvisorMode,
    inner: Mutex<Inner>,
}

impl NeuralReactivationDepthAdvisor {
    pub fn from_config(cfg: &ReactivationDepthConfig, repo_root: &Path) -> Self {
        let model = if cfg.mode == AdvisorMode::Neural {
            let weights_path = repo_root.join(&cfg.weights_path);
            if weights_path.exists() {
                ReactivationDepthModel::load_from_bin(&weights_path)
                    .unwrap_or_else(|_| ReactivationDepthModel::from_arch(&cfg.arch))
            } else {
                ReactivationDepthModel::from_arch(&cfg.arch)
            }
        } else {
            ReactivationDepthModel::from_arch(&cfg.arch)
        };

        Self {
            mode: cfg.mode,
            inner: Mutex::new(Inner {
                model,
                cached_weights: [0.0f32; 8],
                last_infer_tick: 0,
                last_infer_ns: 0,
            }),
        }
    }

    /// Обновить кеш. Вызывается из NeuralAdvisor::on_tick раз в 11 тиков.
    pub fn update_from_trace(
        &self,
        activity_trace: &ActivityTrace,
        avg_depths: &[u32; 8],
        tick: u64,
    ) {
        let Ok(mut inner) = self.inner.try_lock() else { return };

        if self.mode == AdvisorMode::Rule {
            for (i, &d) in avg_depths.iter().enumerate() {
                inner.cached_weights[i] = (1.0 - d as f32 / REACT_MAX_DEPTH).max(0.0);
            }
            inner.last_infer_tick = tick;
            return;
        }

        // mode=neural: FFT над one-hot кольцами → INPUT_SIZE=1539 (консистентно с training_data.jsonl)
        let (short_oh, mid_oh, long_oh) = activity_trace.extract_onehot_rings();
        inner.model.extract_features_from_onehot(&short_oh, &mid_oh, &long_oh);
        let input = inner.model.build_input(tick);

        let t0 = Instant::now();
        let result = inner.model.infer(&input);
        let elapsed_ns = t0.elapsed().as_nanos() as u64;
        inner.last_infer_ns = elapsed_ns;

        if elapsed_ns > INFER_TIMEOUT_NS {
            return; // не обновляем кеш при таймауте
        }

        if let Ok(out) = result {
            for (i, &v) in out.value.iter().take(8).enumerate() {
                inner.cached_weights[i] = v.clamp(0.0, 1.0);
            }
            inner.last_infer_tick = tick;
        }
    }

    pub fn last_infer_ns(&self) -> u64 {
        self.inner.try_lock().map(|g| g.last_infer_ns).unwrap_or(0)
    }

    pub fn last_infer_tick(&self) -> u64 {
        self.inner.try_lock().map(|g| g.last_infer_tick).unwrap_or(0)
    }

    pub fn cached_weights(&self) -> [f32; 8] {
        self.inner.try_lock().map(|g| g.cached_weights).unwrap_or([0.0; 8])
    }

    pub fn mode(&self) -> AdvisorMode { self.mode }
}

impl DepthPredictionAdvisor for NeuralReactivationDepthAdvisor {
    fn predict_depth(&self, input: &DepthAdvisorInput) -> Option<DepthHint> {
        let Ok(inner) = self.inner.try_lock() else { return None };
        let oct = input.primary_octant as usize;
        if oct >= 8 { return None; }

        let weight = inner.cached_weights[oct];
        if weight < NEURAL_MIN_WEIGHT { return None; }

        let suggested_depth = (weight * REACT_MAX_DEPTH) as u16;
        let confidence = (weight * NEURAL_MAX_CONFIDENCE).min(NEURAL_MAX_CONFIDENCE);

        Some(DepthHint {
            target_octant: input.primary_octant,
            suggested_depth,
            confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{Octant, SubsystemId};
    use axiom_neural::ReactivationDepthConfig;

    fn make_advisor(mode: AdvisorMode) -> NeuralReactivationDepthAdvisor {
        let mut cfg = ReactivationDepthConfig::default();
        cfg.mode = mode;
        NeuralReactivationDepthAdvisor::from_config(&cfg, Path::new("."))
    }

    fn make_input(oct: Octant) -> DepthAdvisorInput {
        DepthAdvisorInput {
            sutra_id: 1,
            subsystem: SubsystemId::Writing,
            current_depth_per_octant: [100; 8],
            reactivation_count: 30,
            frame_age_ticks: 100,
            primary_octant: oct,
            event_id: 1,
        }
    }

    #[test]
    fn test_no_hint_before_update() {
        let advisor = make_advisor(AdvisorMode::Rule);
        let hint = advisor.predict_depth(&make_input(Octant::CreativeAffirmation));
        assert!(hint.is_none(), "кеш нулевой до update_from_trace");
    }

    #[test]
    fn test_rule_mode_hint_after_update() {
        let advisor = make_advisor(AdvisorMode::Rule);
        // Октант 0 мелкий (depth=100 из 3000) → высокий вес реактивации
        let avg_depths = [100u32, 2900, 2900, 2900, 2900, 2900, 2900, 2900];
        let trace = ActivityTrace::new();
        advisor.update_from_trace(&trace, &avg_depths, 11);

        let hint = advisor.predict_depth(&make_input(Octant::CreativeAffirmation));
        assert!(hint.is_some());
        assert!(hint.unwrap().suggested_depth > 0);
    }

    #[test]
    fn test_deep_octant_no_hint() {
        let advisor = make_advisor(AdvisorMode::Rule);
        // Октант 0 глубокий (depth=2900 из 3000) → вес < threshold → нет совета
        let avg_depths = [2900u32, 100, 100, 100, 100, 100, 100, 100];
        let trace = ActivityTrace::new();
        advisor.update_from_trace(&trace, &avg_depths, 11);

        let hint = advisor.predict_depth(&make_input(Octant::CreativeAffirmation));
        assert!(hint.is_none(), "глубокий октант не нуждается в реактивации");
    }

    #[test]
    fn test_neural_mode_no_panic() {
        let advisor = make_advisor(AdvisorMode::Neural);
        let avg_depths = [0u32; 8];
        let trace = ActivityTrace::new();
        // Не должен паниковать даже с пустым trace
        advisor.update_from_trace(&trace, &avg_depths, 11);
    }
}
