// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ActivityTrace + ActivityDynamics + ActivitySignature (CR-V6 Фаза A).
//
// Три кольцевых буфера активности подсистем → непрерывные метрики → классификатор.

use std::collections::{HashMap, HashSet, VecDeque};

use axiom_experience::SubsystemId;

/// Ёмкость кольцевого буфера короткого окна (детекция осцилляции).
pub const SHORT_CAP: usize = 16;
/// Ёмкость кольцевого буфера среднего окна (конвергенция, дивергенция, каскад).
pub const MID_CAP: usize = 64;
/// Ёмкость кольцевого буфера длинного окна (fatigue, Фаза B).
pub const LONG_CAP: usize = 256;
/// Минимум событий в short-окне перед первой классификацией.
pub const MIN_WINDOW_FILL: usize = SHORT_CAP;

/// Лейбл типа активности подсистемной динамики.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ActivitySignature {
    /// Холодный старт: short-буфер ещё не заполнен.
    Uncertain,
    /// Одна подсистема стабильно доминирует.
    Steady,
    /// Переключения туда-обратно между двумя подсистемами.
    Oscillating,
    /// Последовательная активация ≥3 различных подсистем.
    Cascading,
    /// Несколько подсистем сходятся к одной (энтропия падает).
    Converging,
    /// Одна подсистема порождает активность нескольких (энтропия растёт).
    Diverging,
}

impl ActivitySignature {
    pub fn name(self) -> &'static str {
        match self {
            ActivitySignature::Uncertain => "Uncertain",
            ActivitySignature::Steady => "Steady",
            ActivitySignature::Oscillating => "Oscillating",
            ActivitySignature::Cascading => "Cascading",
            ActivitySignature::Converging => "Converging",
            ActivitySignature::Diverging => "Diverging",
        }
    }
}

/// Порог вероятности перехода для directed cascade (V7-C1).
pub const DIRECTED_CASCADE_THRESHOLD: f32 = 0.20;

/// Непрерывные метрики активности, вычисляемые из кольцевых буферов.
#[derive(Debug, Clone)]
pub struct ActivityDynamics {
    /// Нормированный градиент Шеннон-энтропии по трём третям mid-окна.
    /// > 0 → энтропия растёт (Diverging); < 0 → падает (Converging).
    pub entropy_gradient: f32,
    /// Доля пар в short-окне где подсистема меняется на одну из двух альтернирующих.
    pub oscillation_score: f32,
    /// Нормированная длина самого длинного каскада (≥3 строго новых подсистем) в mid-окне.
    pub cascade_score: f32,
    /// Directed cascade score через TransitionMatrix (V7-C1). 0.0 пока matrix не накоплена.
    /// Если > 0 — classify() предпочитает его вместо cascade_score.
    pub directed_cascade_score: f32,
    /// Доля mid-окна, занятая наиболее частой подсистемой.
    pub dominant_persistence: f32,
    /// Текущее число записей в short-буфере (для проверки MIN_WINDOW_FILL).
    pub fill_count: usize,
}

/// Три кольцевых буфера активности подсистем.
///
/// `push(subsystem, event_id)` одновременно пишет во все три буфера.
/// `compute_dynamics()` возвращает метрики по текущему состоянию.
#[derive(Debug)]
pub struct ActivityTrace {
    short: RingBuf,
    mid: RingBuf,
    long: RingBuf,
}

impl Default for ActivityTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityTrace {
    pub fn new() -> Self {
        Self {
            short: RingBuf::new(SHORT_CAP),
            mid: RingBuf::new(MID_CAP),
            long: RingBuf::new(LONG_CAP),
        }
    }

    /// Добавить запись активности подсистемы. `SubsystemId::Unknown` игнорируется.
    pub fn push(&mut self, subsystem: SubsystemId, event_id: u64) {
        if subsystem == SubsystemId::Unknown {
            return;
        }
        self.short.push((subsystem, event_id));
        self.mid.push((subsystem, event_id));
        self.long.push((subsystem, event_id));
    }

    /// Число записей в short-буфере (для cold-start проверки).
    pub fn fill_count(&self) -> usize {
        self.short.len()
    }

    /// Вычислить метрики активности из текущего состояния буферов.
    pub fn compute_dynamics(&self) -> ActivityDynamics {
        let fill_count = self.short.len();

        if fill_count < MIN_WINDOW_FILL {
            return ActivityDynamics {
                entropy_gradient: 0.0,
                oscillation_score: 0.0,
                cascade_score: 0.0,
                directed_cascade_score: 0.0,
                dominant_persistence: 0.0,
                fill_count,
            };
        }

        ActivityDynamics {
            entropy_gradient: compute_entropy_gradient(&self.mid),
            oscillation_score: compute_oscillation_score(&self.short),
            cascade_score: compute_cascade_score(&self.mid),
            directed_cascade_score: 0.0, // заполняется в CR::on_tick через directed_cascade_score()
            dominant_persistence: compute_dominant_persistence(&self.mid),
            fill_count,
        }
    }

    /// Итератор по long-буферу (для SubsystemFatigue в Фазе B).
    pub fn long_iter(&self) -> impl Iterator<Item = &(SubsystemId, u64)> {
        self.long.iter()
    }

    /// Уникальные подсистемы из mid-буфера (для детекции composite co-activation).
    pub fn unique_subsystems_in_mid(&self) -> Vec<SubsystemId> {
        let mut seen = HashSet::new();
        self.mid
            .iter()
            .map(|(s, _)| *s)
            .filter(|s| seen.insert(*s))
            .collect()
    }

    /// Directed cascade score (V7-C1): каскад A→B→C где prob(A→B) и prob(B→C) ≥ threshold.
    ///
    /// Возвращает 0.0 если matrix пустая или данных недостаточно.
    pub fn directed_cascade_score(
        &self,
        matrix: &crate::over_domain::context_recognizer::transitions::TransitionMatrix,
        threshold: f32,
    ) -> f32 {
        if matrix.is_empty() {
            return 0.0;
        }
        let items: Vec<SubsystemId> = self.mid.iter().map(|&(s, _)| s).collect();
        compute_directed_cascade_impl(&items, matrix, threshold)
    }
}

/// Классифицировать динамику активности в набор лейблов.
///
/// Возвращает `[Uncertain]` если short-буфер ещё не заполнен.
/// Приоритет проверок: Steady → Oscillating → Cascading → Converging → Diverging.
/// Несколько лейблов возможны (например Steady + Converging).
pub fn classify(dynamics: &ActivityDynamics) -> Vec<ActivitySignature> {
    if dynamics.fill_count < MIN_WINDOW_FILL {
        return vec![ActivitySignature::Uncertain];
    }

    let mut sigs = Vec::new();

    if dynamics.dominant_persistence > 0.7 {
        sigs.push(ActivitySignature::Steady);
    }
    // Oscillating: не применяется когда один явный доминант
    if dynamics.oscillation_score > 0.5 && dynamics.dominant_persistence <= 0.7 {
        sigs.push(ActivitySignature::Oscillating);
    }
    // Cascading: directed_cascade_score (V7-C1) предпочтительнее cascade_score
    let effective_cascade = if dynamics.directed_cascade_score > 0.0 {
        dynamics.directed_cascade_score
    } else {
        dynamics.cascade_score
    };
    if effective_cascade > 0.4 && dynamics.dominant_persistence <= 0.7 {
        sigs.push(ActivitySignature::Cascading);
    }
    if dynamics.entropy_gradient < -0.15 {
        sigs.push(ActivitySignature::Converging);
    }
    if dynamics.entropy_gradient > 0.15 {
        sigs.push(ActivitySignature::Diverging);
    }

    // Fallback: если ни один лейбл не сработал — Steady (система делает что-то стабильное)
    if sigs.is_empty() {
        sigs.push(ActivitySignature::Steady);
    }

    sigs
}

// ── Кольцевой буфер ─────────────────────────────────────────────────────────

#[derive(Debug)]
struct RingBuf {
    data: VecDeque<(SubsystemId, u64)>,
    cap: usize,
}

impl RingBuf {
    fn new(cap: usize) -> Self {
        Self { data: VecDeque::with_capacity(cap), cap }
    }

    fn push(&mut self, item: (SubsystemId, u64)) {
        if self.data.len() >= self.cap {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> impl Iterator<Item = &(SubsystemId, u64)> {
        self.data.iter()
    }
}

// ── Вспомогательные функции вычисления метрик ────────────────────────────────

/// Доля mid-буфера, занятая наиболее частой подсистемой.
fn compute_dominant_persistence(mid: &RingBuf) -> f32 {
    let n = mid.len();
    if n == 0 {
        return 0.0;
    }
    let mut counts: HashMap<SubsystemId, usize> = HashMap::new();
    for &(sub, _) in mid.iter() {
        *counts.entry(sub).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap_or(0);
    max_count as f32 / n as f32
}

/// Оценка осцилляции: насколько short-буфер чередуется между двумя подсистемами.
///
/// Возвращает 0 если меньше 4 записей или только одна подсистема.
/// Возвращает долю пар (A→B или B→A) среди всех пар sub-последовательности.
fn compute_oscillation_score(short: &RingBuf) -> f32 {
    let items: Vec<SubsystemId> = short.iter().map(|&(s, _)| s).collect();
    let n = items.len();
    if n < 4 {
        return 0.0;
    }

    // Топ-2 самые частые подсистемы
    let mut counts: HashMap<SubsystemId, usize> = HashMap::new();
    for &s in &items {
        *counts.entry(s).or_insert(0) += 1;
    }
    let mut top: Vec<(SubsystemId, usize)> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));

    if top.len() < 2 {
        return 0.0; // только одна подсистема
    }

    let a = top[0].0;
    let b = top[1].0;

    // Оставить только записи a или b
    let ab: Vec<SubsystemId> = items.iter().copied().filter(|&s| s == a || s == b).collect();
    let ab_n = ab.len();
    if ab_n < 2 {
        return 0.0;
    }

    // ab-последовательность должна составлять ≥75% буфера
    if ab_n < n * 3 / 4 {
        return 0.0;
    }

    // Доля смен направления в ab-последовательности
    let changes = ab.windows(2).filter(|w| w[0] != w[1]).count();
    changes as f32 / (ab_n - 1) as f32
}

/// Нормированный градиент Шеннон-энтропии по трём третям mid-буфера.
///
/// Положительный → энтропия растёт (дивергенция).
/// Отрицательный → энтропия падает (конвергенция).
fn compute_entropy_gradient(mid: &RingBuf) -> f32 {
    let items: Vec<SubsystemId> = mid.iter().map(|&(s, _)| s).collect();
    let n = items.len();
    if n < 6 {
        return 0.0;
    }

    let third = n / 3;
    let e1 = shannon_entropy(&items[..third]);
    let e3 = shannon_entropy(&items[n - third..]);

    // log2(8 подсистем) ≈ 3.0 — нормировочный делитель
    (e3 - e1) / 3.0
}

fn shannon_entropy(items: &[SubsystemId]) -> f32 {
    if items.is_empty() {
        return 0.0;
    }
    let n = items.len() as f32;
    let mut counts: HashMap<SubsystemId, usize> = HashMap::new();
    for &s in items {
        *counts.entry(s).or_insert(0) += 1;
    }
    -counts.values()
        .map(|&c| {
            let p = c as f32 / n;
            if p > 0.0 { p * p.log2() } else { 0.0 }
        })
        .sum::<f32>()
}

/// Доля элементов mid-буфера, входящих в каскады ≥3 строго различных подсистем.
///
/// Каскад: цепочка активаций где каждая следующая подсистема ещё не встречалась в текущей цепочке.
/// Именно такая цепочка ≥3 элементов и считается каскадом.
/// Score = (число элементов в таких каскадах) / total.
fn compute_cascade_score(mid: &RingBuf) -> f32 {
    let items: Vec<SubsystemId> = mid.iter().map(|&(s, _)| s).collect();
    let n = items.len();
    if n < 3 {
        return 0.0;
    }

    let mut cascade_elements = 0usize;
    let mut current_len = 1usize;
    let mut seen_in_run: HashSet<SubsystemId> = HashSet::new();
    seen_in_run.insert(items[0]);

    for i in 1..n {
        if items[i] != items[i - 1] && !seen_in_run.contains(&items[i]) {
            current_len += 1;
            seen_in_run.insert(items[i]);
        } else {
            if current_len >= 3 {
                cascade_elements += current_len;
            }
            current_len = 1;
            seen_in_run.clear();
            seen_in_run.insert(items[i]);
        }
    }
    if current_len >= 3 {
        cascade_elements += current_len;
    }

    if cascade_elements == 0 {
        return 0.0;
    }
    cascade_elements as f32 / n as f32
}

// ── Directed cascade (V7-C1) ─────────────────────────────────────────────────

type TMatrix = crate::over_domain::context_recognizer::transitions::TransitionMatrix;

fn compute_directed_cascade_impl(items: &[SubsystemId], matrix: &TMatrix, threshold: f32) -> f32 {
    let n = items.len();
    if n < 3 {
        return 0.0;
    }

    let mut cascade_elements = 0usize;
    let mut chain_len = 1usize;
    let mut seen: HashSet<SubsystemId> = HashSet::new();
    seen.insert(items[0]);

    for i in 1..n {
        let prev = items[i - 1];
        let curr = items[i];

        if curr != prev
            && !seen.contains(&curr)
            && matrix.probability_of(prev, curr) >= threshold
        {
            chain_len += 1;
            seen.insert(curr);
        } else {
            if chain_len >= 3 {
                cascade_elements += chain_len;
            }
            chain_len = 1;
            seen.clear();
            seen.insert(curr);
        }
    }
    if chain_len >= 3 {
        cascade_elements += chain_len;
    }

    if cascade_elements == 0 {
        return 0.0;
    }
    cascade_elements as f32 / n as f32
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_trace(subs: &[SubsystemId]) -> ActivityTrace {
        let mut t = ActivityTrace::new();
        for (i, &s) in subs.iter().enumerate() {
            t.push(s, i as u64);
        }
        t
    }

    // ── cold start ──────────────────────────────────────────────────────────

    #[test]
    fn test_cold_start_uncertain() {
        let t = ActivityTrace::new();
        let d = t.compute_dynamics();
        assert_eq!(classify(&d), vec![ActivitySignature::Uncertain]);
    }

    #[test]
    fn test_cold_start_15_entries_still_uncertain() {
        let subs: Vec<SubsystemId> = (0..15).map(|_| SubsystemId::Mathematics).collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert_eq!(d.fill_count, 15);
        assert_eq!(classify(&d), vec![ActivitySignature::Uncertain]);
    }

    #[test]
    fn test_exactly_16_entries_not_uncertain() {
        let subs: Vec<SubsystemId> = (0..16).map(|_| SubsystemId::Mathematics).collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(classify(&d) != vec![ActivitySignature::Uncertain]);
    }

    // ── Steady ──────────────────────────────────────────────────────────────

    #[test]
    fn test_steady_single_subsystem() {
        let subs: Vec<SubsystemId> = (0..32).map(|_| SubsystemId::Mathematics).collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.dominant_persistence > 0.7);
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Steady), "expected Steady, got {:?}", sigs);
    }

    #[test]
    fn test_steady_dominant_persistence_is_one() {
        let subs: Vec<SubsystemId> = (0..20).map(|_| SubsystemId::Writing).collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!((d.dominant_persistence - 1.0).abs() < 1e-5);
    }

    // ── Oscillating ─────────────────────────────────────────────────────────

    #[test]
    fn test_oscillating_two_alternating() {
        let mut subs = Vec::new();
        for _ in 0..16 {
            subs.push(SubsystemId::Mathematics);
            subs.push(SubsystemId::Writing);
        }
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.oscillation_score > 0.5, "oscillation_score={}", d.oscillation_score);
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Oscillating), "expected Oscillating, got {:?}", sigs);
    }

    #[test]
    fn test_no_oscillation_single_subsystem() {
        let subs: Vec<SubsystemId> = (0..32).map(|_| SubsystemId::Music).collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.oscillation_score < 0.5);
    }

    // ── Cascading ───────────────────────────────────────────────────────────

    #[test]
    fn test_cascading_sequential_distinct() {
        use SubsystemId::*;
        let pattern = [Mathematics, Writing, Music, Time, Logic];
        let mut subs = Vec::new();
        for _ in 0..16 {
            subs.extend_from_slice(&pattern);
        }
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.cascade_score > 0.4, "cascade_score={}", d.cascade_score);
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Cascading), "expected Cascading, got {:?}", sigs);
    }

    #[test]
    fn test_no_cascade_two_subsystems() {
        let subs: Vec<SubsystemId> = (0..32)
            .map(|i| if i % 2 == 0 { SubsystemId::Mathematics } else { SubsystemId::Writing })
            .collect();
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.cascade_score < 0.4, "cascade_score={}", d.cascade_score);
    }

    // ── Converging ──────────────────────────────────────────────────────────

    #[test]
    fn test_converging_entropy_drops() {
        use SubsystemId::*;
        // Сначала много разных, потом только одна
        let mut subs: Vec<SubsystemId> = vec![
            Mathematics, Writing, Music, Time, Logic, Mathematics, Writing, Music, Time, Logic,
            Mathematics, Writing, Music, Time, Logic, Mathematics, Writing, Music, Time, Logic,
            Mathematics, Writing, Music, Time, Logic, Mathematics, Writing, Music, Time, Logic,
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
        ];
        // Убедиться что >= 16 разных в начале mid и >= 16 одинаковых в конце
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.entropy_gradient < -0.15, "entropy_gradient={}", d.entropy_gradient);
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Converging), "expected Converging, got {:?}", sigs);
        let _ = subs; // suppress warning
    }

    // ── Diverging ───────────────────────────────────────────────────────────

    #[test]
    fn test_diverging_entropy_rises() {
        use SubsystemId::*;
        // Сначала одна подсистема, потом много разных
        let mut subs: Vec<SubsystemId> = vec![
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
            Mathematics, Mathematics, Mathematics, Mathematics, Mathematics, Mathematics,
            Writing, Music, Time, Logic, Writing, Music, Time, Logic,
            Writing, Music, Time, Logic, Writing, Music, Time, Logic,
            Writing, Music, Time, Logic, Writing, Music, Time, Logic,
            Writing, Music, Time, Logic, Writing, Music, Time, Logic,
        ];
        let t = fill_trace(&subs);
        let d = t.compute_dynamics();
        assert!(d.entropy_gradient > 0.15, "entropy_gradient={}", d.entropy_gradient);
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Diverging), "expected Diverging, got {:?}", sigs);
        let _ = subs;
    }

    // ── Buffer eviction ──────────────────────────────────────────────────────

    #[test]
    fn test_short_buffer_eviction() {
        let mut t = ActivityTrace::new();
        // Заполнить больше SHORT_CAP элементов
        for i in 0..32u64 {
            t.push(SubsystemId::Mathematics, i);
        }
        assert_eq!(t.fill_count(), SHORT_CAP);
    }

    #[test]
    fn test_long_buffer_eviction() {
        let mut t = ActivityTrace::new();
        for i in 0..(LONG_CAP + 10) as u64 {
            t.push(SubsystemId::Mathematics, i);
        }
        // long буфер не превышает LONG_CAP
        assert_eq!(t.long.len(), LONG_CAP);
    }

    // ── Signature change on-the-fly ──────────────────────────────────────────

    #[test]
    fn test_signature_changes_from_steady_to_oscillating() {
        let subs_steady: Vec<SubsystemId> = (0..32).map(|_| SubsystemId::Mathematics).collect();
        let t = fill_trace(&subs_steady);
        let d = t.compute_dynamics();
        assert!(classify(&d).contains(&ActivitySignature::Steady));

        // Добавляем 40 осцилляций — достаточно чтобы разбавить доминанту Math ниже 0.7.
        // После 32 + 40 = 72 записей mid-буфер (64) содержит последние 64:
        //   24 Math (steady) + 40 чередующихся (20 Math + 20 Writing) → Math 44/64 ≈ 0.69 < 0.7.
        let mut t2 = t;
        for i in 0..40u64 {
            let s = if i % 2 == 0 { SubsystemId::Mathematics } else { SubsystemId::Writing };
            t2.push(s, 1000 + i);
        }
        let d2 = t2.compute_dynamics();
        let sigs2 = classify(&d2);
        assert!(
            sigs2.contains(&ActivitySignature::Oscillating),
            "expected Oscillating after 40 alternating inputs, got {:?}", sigs2
        );
    }

    // ── Unknown subsystem ignored ────────────────────────────────────────────

    #[test]
    fn test_unknown_not_pushed() {
        let mut t = ActivityTrace::new();
        for i in 0..100u64 {
            t.push(SubsystemId::Unknown, i);
        }
        assert_eq!(t.fill_count(), 0);
    }

    // ── Directed cascade (V7-C1) ─────────────────────────────────────────────

    fn make_matrix_with_chain() -> TMatrix {
        let mut m = TMatrix::new();
        // Записываем цепочку: Writing→Mathematics→Music с хорошей вероятностью
        for _ in 0..8 {
            m.record(SubsystemId::Writing, SubsystemId::Mathematics);
        }
        for _ in 0..8 {
            m.record(SubsystemId::Mathematics, SubsystemId::Music);
        }
        m
    }

    #[test]
    fn test_directed_cascade_zero_when_matrix_empty() {
        let t = fill_trace(&(0..64).map(|_| SubsystemId::Mathematics).collect::<Vec<_>>());
        let m = TMatrix::new();
        assert_eq!(t.directed_cascade_score(&m, DIRECTED_CASCADE_THRESHOLD), 0.0);
    }

    #[test]
    fn test_directed_cascade_detects_chain() {
        use SubsystemId::*;
        // Паттерн Writing→Mathematics→Music, повторяется много раз
        let pattern = [Writing, Mathematics, Music];
        let mut subs = Vec::new();
        for _ in 0..24 {
            subs.extend_from_slice(&pattern);
        }
        let t = fill_trace(&subs);
        let m = make_matrix_with_chain();
        let score = t.directed_cascade_score(&m, DIRECTED_CASCADE_THRESHOLD);
        assert!(score > 0.4, "directed cascade score should be high for known chain, got {score}");
    }

    #[test]
    fn test_directed_cascade_zero_for_random_sequence() {
        use SubsystemId::*;
        // Случайная последовательность без обученных переходов в matrix
        let subs = [Writing, Mathematics, Music, Time, Logic, Values, Writing, Music];
        let t = fill_trace(&subs.repeat(8));
        let m = TMatrix::new(); // пустая матрица — всё вероятности 0
        let score = t.directed_cascade_score(&m, DIRECTED_CASCADE_THRESHOLD);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_classify_uses_directed_cascade_score() {
        // Если directed_cascade_score > 0.4 и persistence <= 0.7 → Cascading
        let mut d = ActivityDynamics {
            entropy_gradient: 0.0,
            oscillation_score: 0.0,
            cascade_score: 0.0,
            directed_cascade_score: 0.5, // направленный каскад
            dominant_persistence: 0.5,
            fill_count: MIN_WINDOW_FILL,
        };
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Cascading), "expected Cascading via directed_cascade_score, got {:?}", sigs);

        // Если directed_cascade_score = 0 и cascade_score = 0 → нет Cascading
        d.directed_cascade_score = 0.0;
        d.cascade_score = 0.0;
        let sigs2 = classify(&d);
        assert!(!sigs2.contains(&ActivitySignature::Cascading), "should not be Cascading without scores");
    }

    #[test]
    fn test_directed_cascade_fallback_to_cascade_score() {
        // cascade_score > 0.4 AND directed_cascade_score = 0 → Cascading (backward compat)
        let d = ActivityDynamics {
            entropy_gradient: 0.0,
            oscillation_score: 0.0,
            cascade_score: 0.6,
            directed_cascade_score: 0.0,
            dominant_persistence: 0.5,
            fill_count: MIN_WINDOW_FILL,
        };
        let sigs = classify(&d);
        assert!(sigs.contains(&ActivitySignature::Cascading), "backward compat: cascade_score fallback failed, got {:?}", sigs);
    }
}
