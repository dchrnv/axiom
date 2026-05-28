// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SplitMergeDetector (V7-D2) — обнаружение Split/Merge сигналов в DREAM-фазе.
//
// Не применяется автоматически — только предлагается chrnv для решения.
//
// Split-сигнал:  высокая нагрузка + высокая энтропия исходящих переходов
//                (подсистема → много разных следующих → внутренняя неоднородность)
// Merge-сигнал:  обе P(A→B) и P(B→A) ≥ MERGE_THRESHOLD (сильная двусторонняя связь)

use axiom_experience::{FatigueStore, SubsystemId, MAX_ACTIVATION_LOAD};

use super::transitions::TransitionMatrix;

/// Порог для merge-сигнала (обе стороны).
pub const MERGE_THRESHOLD: f32 = 0.25;
/// Порог нагрузки для split-сигнала (доля MAX_ACTIVATION_LOAD).
pub const SPLIT_LOAD_THRESHOLD: f32 = 0.6;
/// Порог энтропии исходящих переходов для split-сигнала.
pub const SPLIT_ENTROPY_THRESHOLD: f32 = 1.5;

/// Причина split-сигнала.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitReason {
    /// Подсистема перегружена (load > 0.6 MAX) и имеет разнообразные исходящие переходы.
    HighLoadHighEntropy,
}

/// Причина merge-сигнала.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeReason {
    /// Обе P(A→B) и P(B→A) ≥ MERGE_THRESHOLD.
    StrongBidirectionalCoupling,
}

/// Кандидат на разделение подсистемы (V7-D2).
#[derive(Debug, Clone)]
pub struct SplitCandidate {
    pub subsystem: SubsystemId,
    /// Сила сигнала: 0..1.
    pub signal_strength: f32,
    pub reason: SplitReason,
    /// Энтропия исходящих переходов из TransitionMatrix.
    pub outgoing_entropy: f32,
    /// activation_load / MAX_ACTIVATION_LOAD.
    pub load_ratio: f32,
}

/// Кандидат на слияние двух подсистем (V7-D2).
#[derive(Debug, Clone)]
pub struct MergeCandidate {
    pub a: SubsystemId,
    pub b: SubsystemId,
    /// Сила сигнала: 0..1.
    pub signal_strength: f32,
    pub reason: MergeReason,
    pub prob_ab: f32,
    pub prob_ba: f32,
}

/// Хранилище split/merge кандидатов.
#[derive(Debug, Default)]
pub struct SplitMergeCandidateStore {
    pub split_candidates: Vec<SplitCandidate>,
    pub merge_candidates: Vec<MergeCandidate>,
}

impl SplitMergeCandidateStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.split_candidates.is_empty() && self.merge_candidates.is_empty()
    }
}

/// Детектор split/merge сигналов. Вызывается в DREAM-фазе (on_tick CR).
pub struct SplitMergeDetector;

impl SplitMergeDetector {
    /// Вычислить split/merge кандидатов из текущего состояния.
    ///
    /// `fatigue` — текущие нагрузки подсистем.
    /// `matrix` — матрица переходов (нужна для обоих сигналов).
    pub fn detect(fatigue: &FatigueStore, matrix: &TransitionMatrix) -> SplitMergeCandidateStore {
        let mut store = SplitMergeCandidateStore::new();
        if matrix.is_empty() {
            return store;
        }

        let known: &[SubsystemId] = &[
            SubsystemId::Writing,
            SubsystemId::Mathematics,
            SubsystemId::Music,
            SubsystemId::Time,
            SubsystemId::Logic,
            SubsystemId::Values,
            SubsystemId::Morality,
            SubsystemId::Abstractions,
            SubsystemId::Dilemmas,
        ];

        // ── Split detection ──────────────────────────────────────────────────
        for &sub in known {
            let load_ratio = fatigue
                .get(sub)
                .map(|f| f.activation_load / MAX_ACTIVATION_LOAD)
                .unwrap_or(0.0);

            if load_ratio < SPLIT_LOAD_THRESHOLD {
                continue;
            }

            let entropy = outgoing_entropy(sub, matrix);
            if entropy < SPLIT_ENTROPY_THRESHOLD {
                continue;
            }

            // signal_strength: комбинация load и нормированной энтропии
            let entropy_norm = (entropy / 3.0).min(1.0); // log2(8) ≈ 3.0
            let signal_strength = (load_ratio + entropy_norm) / 2.0;

            store.split_candidates.push(SplitCandidate {
                subsystem: sub,
                signal_strength,
                reason: SplitReason::HighLoadHighEntropy,
                outgoing_entropy: entropy,
                load_ratio,
            });
        }

        // ── Merge detection ──────────────────────────────────────────────────
        for i in 0..known.len() {
            for j in (i + 1)..known.len() {
                let a = known[i];
                let b = known[j];
                let prob_ab = matrix.probability_of(a, b);
                let prob_ba = matrix.probability_of(b, a);
                if prob_ab >= MERGE_THRESHOLD && prob_ba >= MERGE_THRESHOLD {
                    let signal_strength = (prob_ab + prob_ba) / 2.0;
                    store.merge_candidates.push(MergeCandidate {
                        a,
                        b,
                        signal_strength,
                        reason: MergeReason::StrongBidirectionalCoupling,
                        prob_ab,
                        prob_ba,
                    });
                }
            }
        }

        store
    }
}

/// Shannon-энтропия исходящих переходов для подсистемы.
///
/// H = -Σ p_i * log2(p_i) по ненулевым вероятностям.
/// Высокая энтропия → переходы в много разных подсистем → потенциальная внутренняя неоднородность.
fn outgoing_entropy(sub: SubsystemId, matrix: &TransitionMatrix) -> f32 {
    let row_sum: f32 = matrix.raw()[subsystem_idx(sub)].iter().sum();
    if row_sum <= 0.0 {
        return 0.0;
    }
    -matrix.raw()[subsystem_idx(sub)]
        .iter()
        .filter(|&&v| v > 0.0)
        .map(|&v| {
            let p = v / row_sum;
            p * p.log2()
        })
        .sum::<f32>()
}

fn subsystem_idx(s: SubsystemId) -> usize {
    match s {
        SubsystemId::Writing      => 0,
        SubsystemId::Mathematics  => 1,
        SubsystemId::Music        => 2,
        SubsystemId::Time         => 3,
        SubsystemId::Logic        => 4,
        SubsystemId::Values       => 5,
        SubsystemId::Morality     => 6,
        SubsystemId::Abstractions => 7,
        SubsystemId::Dilemmas     => 8,
        SubsystemId::Unknown      => 15,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fatigue_at_max(sub: SubsystemId) -> FatigueStore {
        let mut f = FatigueStore::new();
        for _ in 0..100 {
            f.update(sub);
        }
        f
    }

    fn make_matrix_uniform_outgoing(sub: SubsystemId) -> TransitionMatrix {
        let mut m = TransitionMatrix::new();
        // Равномерно много разных переходов
        let targets = [
            SubsystemId::Writing, SubsystemId::Mathematics, SubsystemId::Music,
            SubsystemId::Time, SubsystemId::Logic, SubsystemId::Values,
        ];
        for &t in &targets {
            if t != sub {
                for _ in 0..5 {
                    m.record(sub, t);
                }
            }
        }
        m
    }

    fn make_matrix_bidirectional(a: SubsystemId, b: SubsystemId) -> TransitionMatrix {
        let mut m = TransitionMatrix::new();
        for _ in 0..5 {
            m.record(a, b);
            m.record(b, a);
        }
        m
    }

    #[test]
    fn test_no_candidates_empty_matrix() {
        let f = FatigueStore::new();
        let m = TransitionMatrix::new();
        let store = SplitMergeDetector::detect(&f, &m);
        assert!(store.is_empty());
    }

    #[test]
    fn test_no_split_low_load() {
        let f = FatigueStore::new(); // нулевая нагрузка
        let m = make_matrix_uniform_outgoing(SubsystemId::Writing);
        let store = SplitMergeDetector::detect(&f, &m);
        assert!(store.split_candidates.is_empty(), "no split without sufficient load");
    }

    #[test]
    fn test_split_detected_high_load_high_entropy() {
        let f = make_fatigue_at_max(SubsystemId::Writing);
        let m = make_matrix_uniform_outgoing(SubsystemId::Writing);
        let store = SplitMergeDetector::detect(&f, &m);
        let split = store.split_candidates.iter().find(|c| c.subsystem == SubsystemId::Writing);
        assert!(split.is_some(), "split should be detected with high load + high outgoing entropy");
        let s = split.unwrap();
        assert!(s.load_ratio >= SPLIT_LOAD_THRESHOLD);
        assert!(s.outgoing_entropy >= SPLIT_ENTROPY_THRESHOLD);
    }

    #[test]
    fn test_no_split_focused_outgoing() {
        let f = make_fatigue_at_max(SubsystemId::Writing);
        let mut m = TransitionMatrix::new();
        // Только один фиксированный переход → низкая энтропия
        for _ in 0..20 {
            m.record(SubsystemId::Writing, SubsystemId::Mathematics);
        }
        let store = SplitMergeDetector::detect(&f, &m);
        let split = store.split_candidates.iter().find(|c| c.subsystem == SubsystemId::Writing);
        assert!(split.is_none(), "low entropy → no split");
    }

    #[test]
    fn test_merge_detected_bidirectional() {
        let f = FatigueStore::new();
        let m = make_matrix_bidirectional(SubsystemId::Mathematics, SubsystemId::Writing);
        let store = SplitMergeDetector::detect(&f, &m);
        let merge = store.merge_candidates.iter().find(|c| {
            (c.a == SubsystemId::Mathematics && c.b == SubsystemId::Writing)
            || (c.a == SubsystemId::Writing && c.b == SubsystemId::Mathematics)
        });
        assert!(merge.is_some(), "strong bidirectional → merge candidate");
        let mc = merge.unwrap();
        assert!(mc.prob_ab >= MERGE_THRESHOLD);
        assert!(mc.prob_ba >= MERGE_THRESHOLD);
    }

    #[test]
    fn test_no_merge_unidirectional() {
        let f = FatigueStore::new();
        let mut m = TransitionMatrix::new();
        for _ in 0..10 {
            m.record(SubsystemId::Mathematics, SubsystemId::Writing);
            // Writing → Mathematics не записываем
        }
        let store = SplitMergeDetector::detect(&f, &m);
        let merge = store.merge_candidates.iter().find(|c| {
            (c.a == SubsystemId::Mathematics && c.b == SubsystemId::Writing)
            || (c.a == SubsystemId::Writing && c.b == SubsystemId::Mathematics)
        });
        assert!(merge.is_none(), "unidirectional → no merge");
    }

    #[test]
    fn test_split_signal_strength_in_range() {
        let f = make_fatigue_at_max(SubsystemId::Logic);
        let m = make_matrix_uniform_outgoing(SubsystemId::Logic);
        let store = SplitMergeDetector::detect(&f, &m);
        if let Some(s) = store.split_candidates.iter().find(|c| c.subsystem == SubsystemId::Logic) {
            assert!(s.signal_strength > 0.0 && s.signal_strength <= 1.0);
        }
    }

    #[test]
    fn test_outgoing_entropy_zero_no_data() {
        let m = TransitionMatrix::new();
        let e = outgoing_entropy(SubsystemId::Writing, &m);
        assert_eq!(e, 0.0);
    }

    #[test]
    fn test_outgoing_entropy_higher_for_diverse_transitions() {
        let mut m1 = TransitionMatrix::new();
        m1.record(SubsystemId::Writing, SubsystemId::Mathematics);
        m1.record(SubsystemId::Writing, SubsystemId::Mathematics);
        m1.record(SubsystemId::Writing, SubsystemId::Mathematics);

        let mut m2 = TransitionMatrix::new();
        m2.record(SubsystemId::Writing, SubsystemId::Mathematics);
        m2.record(SubsystemId::Writing, SubsystemId::Music);
        m2.record(SubsystemId::Writing, SubsystemId::Time);

        let e1 = outgoing_entropy(SubsystemId::Writing, &m1);
        let e2 = outgoing_entropy(SubsystemId::Writing, &m2);
        assert!(e2 > e1, "diverse transitions → higher entropy: e1={e1}, e2={e2}");
    }
}
