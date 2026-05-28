// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ActivityAnalyzer (бывший TransitionDetector) — лёгкий детектор смены подсистемы.
// TransitionMatrix (V7-B1) — 16×16 матрица вероятностей переходов между подсистемами.
// CR-V6: переименован; остаётся как lightweight компонент рядом с ActivityTrace.

use axiom_experience::SubsystemId;

/// Событие переключения между подсистемами.
#[derive(Debug, Clone)]
pub struct SubsystemTransition {
    pub from: SubsystemId,
    pub to: SubsystemId,
    pub at_event: u64,
}

/// Лёгкий анализатор переключений между подсистемами.
///
/// Фиксирует факт смены доминирующей подсистемы.
/// Для анализа паттернов активности использовать `ActivityTrace`.
#[derive(Debug)]
pub struct ActivityAnalyzer {
    last_primary: SubsystemId,
    last_event: u64,
}

/// Совместимый псевдоним для кода, использующего старое имя.
pub type TransitionDetector = ActivityAnalyzer;

impl Default for ActivityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityAnalyzer {
    pub fn new() -> Self {
        Self {
            last_primary: SubsystemId::Unknown,
            last_event: 0,
        }
    }

    /// Обновить состояние. Возвращает событие переключения если подсистема сменилась.
    pub fn update(&mut self, new_primary: SubsystemId, event_id: u64) -> Option<SubsystemTransition> {
        if self.last_primary == new_primary {
            return None;
        }
        let from = self.last_primary;
        self.last_primary = new_primary;
        self.last_event = event_id;
        if from == SubsystemId::Unknown {
            return None; // первое обнаружение — не переключение
        }
        Some(SubsystemTransition { from, to: new_primary, at_event: event_id })
    }

    pub fn current(&self) -> SubsystemId {
        self.last_primary
    }

    pub fn last_event(&self) -> u64 {
        self.last_event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_update_no_transition() {
        let mut d = ActivityAnalyzer::new();
        assert!(d.update(SubsystemId::Writing, 1).is_none());
    }

    #[test]
    fn test_same_subsystem_no_transition() {
        let mut d = ActivityAnalyzer::new();
        d.update(SubsystemId::Writing, 1);
        assert!(d.update(SubsystemId::Writing, 2).is_none());
    }

    #[test]
    fn test_different_subsystem_returns_transition() {
        let mut d = ActivityAnalyzer::new();
        d.update(SubsystemId::Writing, 1);
        let t = d.update(SubsystemId::Mathematics, 2);
        assert!(t.is_some());
        let t = t.unwrap();
        assert_eq!(t.from, SubsystemId::Writing);
        assert_eq!(t.to, SubsystemId::Mathematics);
        assert_eq!(t.at_event, 2);
    }

    #[test]
    fn test_compat_alias_transition_detector() {
        let mut d = TransitionDetector::new();
        d.update(SubsystemId::Writing, 1);
        assert!(d.update(SubsystemId::Mathematics, 2).is_some());
    }
}

// ── TransitionMatrix (V7-B1) ─────────────────────────────────────────────────

/// 16×16 матрица счётчиков переходов между подсистемами.
///
/// Строка = from, столбец = to. Decay применяется на каждом тике ContextRecognizer.
/// Размер фиксирован: 16 слотов (10 подсистем + 6 зарезервировано).
#[derive(Debug, Clone)]
pub struct TransitionMatrix {
    counts: [[f32; 16]; 16],
    /// Decay-коэффициент на тик (например 0.995 → ~50% за 139 тиков).
    pub decay_factor: f32,
}

impl Default for TransitionMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionMatrix {
    pub fn new() -> Self {
        Self { counts: [[0.0; 16]; 16], decay_factor: 0.995 }
    }

    /// Зафиксировать переход from → to.
    pub fn record(&mut self, from: SubsystemId, to: SubsystemId) {
        if from == SubsystemId::Unknown || to == SubsystemId::Unknown {
            return;
        }
        self.counts[subsystem_index(from)][subsystem_index(to)] += 1.0;
    }

    /// Применить decay ко всей матрице (вызывать на каждом тике CR).
    pub fn decay(&mut self) {
        let f = self.decay_factor;
        for row in &mut self.counts {
            for cell in row.iter_mut() {
                *cell *= f;
            }
        }
    }

    /// Нормированная вероятность перехода from → to (0.0 если нет данных).
    pub fn probability_of(&self, from: SubsystemId, to: SubsystemId) -> f32 {
        let fi = subsystem_index(from);
        let row_sum: f32 = self.counts[fi].iter().sum();
        if row_sum <= 0.0 {
            return 0.0;
        }
        self.counts[fi][subsystem_index(to)] / row_sum
    }

    /// Наиболее вероятная следующая подсистема после from (None если нет данных).
    pub fn most_likely_next(&self, from: SubsystemId) -> Option<SubsystemId> {
        let fi = subsystem_index(from);
        let row_sum: f32 = self.counts[fi].iter().sum();
        if row_sum <= 0.0 {
            return None;
        }
        let (best_idx, &best_val) = self.counts[fi]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())?;
        if best_val <= 0.0 {
            return None;
        }
        subsystem_from_index(best_idx)
    }

    /// Прямой доступ к внутренней матрице (для диагностики / сериализации).
    pub fn raw(&self) -> &[[f32; 16]; 16] {
        &self.counts
    }

    /// Проверить, есть ли хоть какие-то данные.
    pub fn is_empty(&self) -> bool {
        self.counts.iter().all(|row| row.iter().all(|&v| v == 0.0))
    }
}

/// Индекс подсистемы в матрице (0..=15).
fn subsystem_index(s: SubsystemId) -> usize {
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

/// Обратное преобразование: индекс → SubsystemId.
fn subsystem_from_index(idx: usize) -> Option<SubsystemId> {
    match idx {
        0  => Some(SubsystemId::Writing),
        1  => Some(SubsystemId::Mathematics),
        2  => Some(SubsystemId::Music),
        3  => Some(SubsystemId::Time),
        4  => Some(SubsystemId::Logic),
        5  => Some(SubsystemId::Values),
        6  => Some(SubsystemId::Morality),
        7  => Some(SubsystemId::Abstractions),
        8  => Some(SubsystemId::Dilemmas),
        15 => Some(SubsystemId::Unknown),
        _  => None,
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_new_matrix_is_empty() {
        let m = TransitionMatrix::new();
        assert!(m.is_empty());
    }

    #[test]
    fn test_record_increments_count() {
        let mut m = TransitionMatrix::new();
        m.record(SubsystemId::Writing, SubsystemId::Mathematics);
        assert!(!m.is_empty());
        let p = m.probability_of(SubsystemId::Writing, SubsystemId::Mathematics);
        assert!((p - 1.0).abs() < 1e-5, "p={p}");
    }

    #[test]
    fn test_probability_sums_to_one() {
        let mut m = TransitionMatrix::new();
        m.record(SubsystemId::Writing, SubsystemId::Mathematics);
        m.record(SubsystemId::Writing, SubsystemId::Music);
        m.record(SubsystemId::Writing, SubsystemId::Music);
        let p_math = m.probability_of(SubsystemId::Writing, SubsystemId::Mathematics);
        let p_music = m.probability_of(SubsystemId::Writing, SubsystemId::Music);
        assert!((p_math - 1.0 / 3.0).abs() < 1e-5, "p_math={p_math}");
        assert!((p_music - 2.0 / 3.0).abs() < 1e-5, "p_music={p_music}");
    }

    #[test]
    fn test_most_likely_next_correct() {
        let mut m = TransitionMatrix::new();
        m.record(SubsystemId::Logic, SubsystemId::Mathematics);
        m.record(SubsystemId::Logic, SubsystemId::Mathematics);
        m.record(SubsystemId::Logic, SubsystemId::Writing);
        let next = m.most_likely_next(SubsystemId::Logic);
        assert_eq!(next, Some(SubsystemId::Mathematics));
    }

    #[test]
    fn test_most_likely_next_none_when_empty() {
        let m = TransitionMatrix::new();
        assert!(m.most_likely_next(SubsystemId::Writing).is_none());
    }

    #[test]
    fn test_decay_reduces_counts() {
        let mut m = TransitionMatrix::new();
        m.record(SubsystemId::Writing, SubsystemId::Mathematics);
        m.decay();
        let p_before = 1.0_f32;
        let p_after = m.probability_of(SubsystemId::Writing, SubsystemId::Mathematics);
        // Probability unchanged by decay (both numerator and row_sum scaled equally)
        assert!((p_after - p_before).abs() < 1e-4, "p_after={p_after}");
        // But raw counts should be smaller
        let raw_count = m.raw()[subsystem_index(SubsystemId::Writing)][subsystem_index(SubsystemId::Mathematics)];
        assert!(raw_count < 1.0, "expected decay, got {raw_count}");
    }

    #[test]
    fn test_unknown_subsystem_ignored() {
        let mut m = TransitionMatrix::new();
        m.record(SubsystemId::Unknown, SubsystemId::Writing);
        m.record(SubsystemId::Writing, SubsystemId::Unknown);
        assert!(m.is_empty());
    }

    #[test]
    fn test_index_roundtrip_all_variants() {
        use SubsystemId::*;
        let variants = [Writing, Mathematics, Music, Time, Logic, Values, Morality, Abstractions, Dilemmas];
        for v in variants {
            let idx = subsystem_index(v);
            let back = subsystem_from_index(idx);
            assert_eq!(back, Some(v), "roundtrip failed for {:?}", v);
        }
    }
}
