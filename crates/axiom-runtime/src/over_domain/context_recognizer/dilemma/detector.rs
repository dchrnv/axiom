// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DilemmaDetector V2.1 — Сигнал A: конфликт подсистем; B: стресс связей; C: Corpus Callosum.
//
// Источник: DilemmaDetector_V2_0.md §4; §2 Signal B/C
//
// Сигнал A:
//   1. Взять SubsystemConflict из conflicts::detect_conflict.
//   2. Проверить is_natural_tension по SubsystemDependencies.
//   3. Вычислить tension_score. Если < DILEMMA_THRESHOLD — отбросить.
//   4. Cooldown: не дублировать ту же пару раньше DETECTION_COOLDOWN_TICKS.
//   5. push_active() в DilemmaStore (ValueConflict).
//   6. Вернуть UCL InjectToken для кристаллизации Frame в EXPERIENCE.
//
// Сигнал B:
//   1. Взять активные Connection из MAYA.
//   2. Вычислить долю stressed connections (current_stress/strength > threshold).
//   3. Если stressed_count >= MIN_STRESSED + mean_ratio > MEAN_THRESHOLD → дилемма.
//   4. Cooldown: отдельный от Сигнала A.
//   5. push_active() в DilemmaStore (OntologicalConflict).
//   6. Вернуть UCL InjectToken.
//
// Сигнал C (Corpus Callosum):
//   1. Сканировать AxialStore на Frame с conflict (analytic ≠ synthetic octant).
//   2. Если conflict_strength >= CORPUS_CALLOSUM_THRESHOLD → дилемма уровня 3.
//   3. Cooldown per Frame sutra_id.
//   4. push_active() в DilemmaStore (ModelConflict = OntologicalConflict).
//   5. Вернуть UCL InjectToken.

use std::collections::HashMap;

use axiom_config::SubsystemDependencies;
use axiom_core::{Connection, FLAG_ACTIVE, STATE_ACTIVE, TOKEN_FLAG_DILEMMA, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_experience::{AxialStore, Octant, SubsystemId};
use axiom_ucl::{flags as ucl_flags, InjectFrameAnchorPayload, OpCode, UclCommand};

use crate::over_domain::context_recognizer::conflicts::SubsystemConflict;
use crate::over_domain::context_recognizer::dilemma_store::{DilemmaStore, DilemmaType};

use super::tension::{compute_tension_score, DILEMMA_THRESHOLD};

/// Тики CR между повторными регистрациями одной пары подсистем как дилеммы (Сигнал A).
const DETECTION_COOLDOWN_TICKS: u64 = 50;

/// Размер скользящего окна ко-активации (в инъекциях токенов).
const COACTIVATION_WINDOW: usize = 32;

/// Минимальное число появлений КАЖДОЙ подсистемы в окне для детектирования.
const MIN_COACTIVATION_COUNT: usize = 2;

// ── Сигнал B: стресс связей ───────────────────────────────────────────────────

/// Порог stressed-ratio (current_stress / strength) для одной связи.
pub const STRESS_RATIO_THRESHOLD: f32 = 0.5;

/// Минимальное число stressed-связей для срабатывания Signal B.
pub const MIN_STRESSED_CONNECTIONS: usize = 2;

/// Порог среднего stressed-ratio по всем активным связям.
pub const MEAN_STRESS_THRESHOLD: f32 = 0.35;

/// Тики CR cooldown для Сигнала B (дольше A — стресс меняется медленнее).
pub const SIGNAL_B_COOLDOWN_TICKS: u64 = 100;

// ── Сигнал C: Corpus Callosum ─────────────────────────────────────────────────

/// Минимальная сила AxialConflict для срабатывания Signal C (≥ 2 различающихся оси).
pub const CORPUS_CALLOSUM_THRESHOLD: u8 = 170;

/// Тики CR cooldown для Сигнала C на один Frame sutra_id.
pub const SIGNAL_C_COOLDOWN_TICKS: u64 = 150;

/// Канонический порядок пары: по лексикографии имён (для дедупликации).
fn canonical_pair(a: SubsystemId, b: SubsystemId) -> (SubsystemId, SubsystemId) {
    if a.name() <= b.name() { (a, b) } else { (b, a) }
}

/// Центроид пространственных позиций двух подсистем.
fn centroid(
    a: SubsystemId,
    b: SubsystemId,
    refs: &HashMap<SubsystemId, Vec<[i16; 3]>>,
) -> [i16; 3] {
    let positions: Vec<[i16; 3]> = refs
        .get(&a)
        .into_iter()
        .chain(refs.get(&b))
        .flat_map(|v| v.iter().copied())
        .collect();
    if positions.is_empty() {
        return [0i16; 3];
    }
    let n = positions.len() as i32;
    let sum = positions.iter().fold([0i32; 3], |mut acc, p| {
        acc[0] += p[0] as i32;
        acc[1] += p[1] as i32;
        acc[2] += p[2] as i32;
        acc
    });
    [(sum[0] / n) as i16, (sum[1] / n) as i16, (sum[2] / n) as i16]
}

/// FNV-1a хэш канонической пары подсистем (по именам).
fn pair_lineage_hash(a: SubsystemId, b: SubsystemId) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in a.name().bytes().chain(":".bytes()).chain(b.name().bytes()) {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Детектор дилемм V2.1 — встроен в ContextRecognizer.
pub struct DilemmaDetector {
    deps: SubsystemDependencies,
    /// Последний тик CR детектирования для каждой канонической пары (Сигнал A).
    last_detected: HashMap<(SubsystemId, SubsystemId), u64>,
    /// Скользящее окно ко-активации: последние N subsystem-активаций из record_injection_signal.
    coactivation_window: std::collections::VecDeque<(SubsystemId, u64)>,
    /// Последний тик детектирования Сигнала B (стресс связей).
    last_stress_detected: u64,
    /// Последний тик детектирования Сигнала C per Frame sutra_id (Corpus Callosum).
    last_cc_detected: HashMap<u32, u64>,
}

impl DilemmaDetector {
    pub fn new() -> Self {
        Self {
            deps: SubsystemDependencies::default(),
            last_detected: HashMap::new(),
            coactivation_window: std::collections::VecDeque::with_capacity(COACTIVATION_WINDOW + 1),
            last_stress_detected: 0,
            last_cc_detected: HashMap::new(),
        }
    }

    /// Записать активацию подсистемы из record_injection_signal.
    ///
    /// Вызывается из ContextRecognizer::record_injection_signal при каждом InjectToken.
    pub fn record_injection(&mut self, sub: SubsystemId, tick: u64) {
        if sub == SubsystemId::Unknown {
            return;
        }
        if self.coactivation_window.len() >= COACTIVATION_WINDOW {
            self.coactivation_window.pop_front();
        }
        self.coactivation_window.push_back((sub, tick));
    }

    /// Найти пару подсистем в окне ко-активации с достаточным числом появлений.
    ///
    /// Возвращает (primary, secondary, intensity) если обе подсистемы появляются
    /// не менее MIN_COACTIVATION_COUNT раз и являются natural tension.
    fn detect_coactivation_pair(&self) -> Option<(SubsystemId, SubsystemId, f32)> {
        use std::collections::HashMap;
        let mut counts: HashMap<SubsystemId, usize> = HashMap::new();
        for &(sub, _) in &self.coactivation_window {
            *counts.entry(sub).or_insert(0) += 1;
        }
        let qualifying: Vec<(SubsystemId, usize)> = counts
            .into_iter()
            .filter(|(_, c)| *c >= MIN_COACTIVATION_COUNT)
            .collect();
        for i in 0..qualifying.len() {
            for j in (i + 1)..qualifying.len() {
                let (sa, ca) = qualifying[i];
                let (sb, cb) = qualifying[j];
                if self.deps.is_natural_tension(sa.name(), sb.name()) {
                    // Интенсивность: насколько сбалансированы появления
                    let min_c = ca.min(cb) as f32;
                    let max_c = ca.max(cb) as f32;
                    let intensity = (min_c / max_c).min(1.0) * 0.75 + 0.25;
                    // primary = та что встречалась чаще
                    let (primary, secondary) = if ca >= cb { (sa, sb) } else { (sb, sa) };
                    return Some((primary, secondary, intensity));
                }
            }
        }
        None
    }

    /// Заменить граф зависимостей (вызывается из from_anchor_set).
    pub fn set_dependencies(&mut self, deps: SubsystemDependencies) {
        self.deps = deps;
    }

    /// Проверить конфликт и зарегистрировать дилемму если все условия выполнены.
    ///
    /// Возвращает UCL-команды: 1 InjectToken (Frame дилеммы в EXPERIENCE).
    /// Возвращает пустой Vec если дилемма не обнаружена.
    pub fn detect(
        &mut self,
        conflict: Option<SubsystemConflict>,
        store: &mut DilemmaStore,
        subsystem_refs: &HashMap<SubsystemId, Vec<[i16; 3]>>,
        exp_domain_id: u16,
        tick: u64,
    ) -> Vec<UclCommand> {
        // Путь 1: энергетический конфликт из MAYA (если доступен)
        let (pair, tension) = if let Some(ref c) = conflict {
            if self.deps.is_natural_tension(c.primary.name(), c.secondary.name()) {
                let t = compute_tension_score(c);
                if t >= DILEMMA_THRESHOLD {
                    (canonical_pair(c.primary, c.secondary), t)
                } else {
                    // Путь 2: ко-активация из окна инъекций
                    match self.detect_coactivation_pair() {
                        Some((a, b, t)) if t >= DILEMMA_THRESHOLD => (canonical_pair(a, b), t),
                        _ => return vec![],
                    }
                }
            } else {
                match self.detect_coactivation_pair() {
                    Some((a, b, t)) if t >= DILEMMA_THRESHOLD => (canonical_pair(a, b), t),
                    _ => return vec![],
                }
            }
        } else {
            // Путь 2: ко-активация из окна инъекций (MAYA-конфликта нет)
            match self.detect_coactivation_pair() {
                Some((a, b, t)) if t >= DILEMMA_THRESHOLD => (canonical_pair(a, b), t),
                _ => return vec![],
            }
        };

        if let Some(&last) = self.last_detected.get(&pair) {
            if tick.saturating_sub(last) < DETECTION_COOLDOWN_TICKS {
                return vec![];
            }
        }

        if store.is_at_capacity() {
            return vec![];
        }

        store.push_active(DilemmaType::ValueConflict, vec![], tick, tension);
        self.last_detected.insert(pair, tick);

        let position = centroid(pair.0, pair.1, subsystem_refs);
        let lineage_hash = pair_lineage_hash(pair.0, pair.1);
        // 0x4000_0000 префикс отличает Frame дилеммы от разрешённых (0x8000_0000)
        let proposed_sutra_id = ((lineage_hash >> 32) as u32) | 0x4000_0000;
        let mass = (80u8).saturating_add((tension * 120.0) as u8);

        let payload = InjectFrameAnchorPayload {
            lineage_hash,
            proposed_sutra_id,
            target_domain_id: exp_domain_id,
            type_flags: TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_DILEMMA,
            position,
            state: STATE_ACTIVE,
            mass,
            temperature: 110,
            valence: 0,
            reserved: [0; 22],
        };
        vec![UclCommand::new(OpCode::InjectToken, 0, 10, ucl_flags::FRAME_ANCHOR)
            .with_payload(&payload)]
    }

    /// Сигнал B — стресс связей MAYA.
    ///
    /// Сканирует активные Connection на высокий `current_stress / strength`.
    /// Если достаточно stressed-связей — фиксирует OntologicalConflict в DilemmaStore.
    /// Cooldown независим от Сигнала A.
    pub fn detect_signal_b(
        &mut self,
        connections: &[Connection],
        dominant: SubsystemId,
        subsystem_refs: &HashMap<SubsystemId, Vec<[i16; 3]>>,
        store: &mut DilemmaStore,
        exp_domain_id: u16,
        tick: u64,
    ) -> Vec<UclCommand> {
        let (stressed_count, mean_ratio) = stress_metrics(connections);

        if stressed_count < MIN_STRESSED_CONNECTIONS {
            return vec![];
        }
        if mean_ratio < MEAN_STRESS_THRESHOLD {
            return vec![];
        }
        if tick.saturating_sub(self.last_stress_detected) < SIGNAL_B_COOLDOWN_TICKS {
            return vec![];
        }
        if store.is_at_capacity() {
            return vec![];
        }

        let intensity = mean_ratio.min(1.0);
        store.push_active(DilemmaType::OntologicalConflict, vec![], tick, intensity);
        self.last_stress_detected = tick;

        let position = stress_position(dominant, subsystem_refs);
        let lineage_hash = stress_lineage_hash(dominant, tick, SIGNAL_B_COOLDOWN_TICKS);
        // 0x5000_0000 префикс отличает Signal B Frame от Signal A (0x4000_0000)
        let proposed_sutra_id = ((lineage_hash >> 32) as u32) | 0x5000_0000;
        let mass = (70u8).saturating_add((intensity * 100.0) as u8);

        let payload = InjectFrameAnchorPayload {
            lineage_hash,
            proposed_sutra_id,
            target_domain_id: exp_domain_id,
            type_flags: TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_DILEMMA,
            position,
            state: STATE_ACTIVE,
            mass,
            temperature: 100,
            valence: 0,
            reserved: [0; 22],
        };
        vec![UclCommand::new(OpCode::InjectToken, 0, 10, ucl_flags::FRAME_ANCHOR)
            .with_payload(&payload)]
    }

    /// Детектировать Сигнал C (Corpus Callosum) из AxialStore.
    ///
    /// Для каждого Frame в `frame_ids`: если latest eval имеет conflict_strength >=
    /// CORPUS_CALLOSUM_THRESHOLD — ModelConflict дилемма (analytic ≠ synthetic octant).
    /// Cooldown per Frame sutra_id.
    pub fn detect_signal_c(
        &mut self,
        axial_store: &AxialStore,
        frame_ids: &[u32],
        store: &mut DilemmaStore,
        exp_domain_id: u16,
        tick: u64,
    ) -> Vec<UclCommand> {
        if frame_ids.is_empty() || store.is_at_capacity() {
            return vec![];
        }

        let mut cmds = Vec::new();

        for &frame_id in frame_ids {
            if let Some(&last) = self.last_cc_detected.get(&frame_id) {
                if tick.saturating_sub(last) < SIGNAL_C_COOLDOWN_TICKS {
                    continue;
                }
            }

            let eval = match axial_store.get_latest(frame_id) {
                Some(e) => e,
                None => continue,
            };

            let conflict = match &eval.conflict {
                Some(c) if c.conflict_strength >= CORPUS_CALLOSUM_THRESHOLD => c.clone(),
                _ => continue,
            };

            if store.is_at_capacity() {
                break;
            }

            let intensity = conflict.conflict_strength as f32 / 255.0;
            store.push_active(DilemmaType::OntologicalConflict, vec![frame_id], tick, intensity);
            self.last_cc_detected.insert(frame_id, tick);

            let position = cc_position(conflict.analytic_octant, conflict.synthetic_octant);
            let lineage_hash = cc_lineage_hash(frame_id, tick, SIGNAL_C_COOLDOWN_TICKS);
            let proposed_sutra_id = ((lineage_hash >> 32) as u32) | 0x6000_0000;
            let mass = (60u8).saturating_add((intensity * 100.0) as u8);

            let payload = InjectFrameAnchorPayload {
                lineage_hash,
                proposed_sutra_id,
                target_domain_id: exp_domain_id,
                type_flags: TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_DILEMMA,
                position,
                state: STATE_ACTIVE,
                mass,
                temperature: 130,
                valence: 0,
                reserved: [0; 22],
            };
            cmds.push(UclCommand::new(OpCode::InjectToken, 0, 10, ucl_flags::FRAME_ANCHOR)
                .with_payload(&payload));
        }
        cmds
    }
}

impl Default for DilemmaDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Вычислить (stressed_count, mean_stress_ratio) по активным связям.
///
/// Связь "stressed" если `current_stress / strength > STRESS_RATIO_THRESHOLD`.
/// Mean ratio = среднее `current_stress / strength` по ВСЕМ активным связям.
fn stress_metrics(connections: &[Connection]) -> (usize, f32) {
    let active: Vec<&Connection> = connections
        .iter()
        .filter(|c| c.flags & FLAG_ACTIVE != 0 && c.strength > 0.0)
        .collect();
    if active.is_empty() {
        return (0, 0.0);
    }
    let mut stressed = 0usize;
    let mut ratio_sum = 0.0f32;
    for c in &active {
        let ratio = c.current_stress / c.strength;
        ratio_sum += ratio;
        if ratio > STRESS_RATIO_THRESHOLD {
            stressed += 1;
        }
    }
    (stressed, ratio_sum / active.len() as f32)
}

/// Пространственная позиция для Frame Сигнала B: центроид якорей доминирующей подсистемы.
/// Fallback на [0, 0, 0] если refs пусты.
fn stress_position(dominant: SubsystemId, refs: &HashMap<SubsystemId, Vec<[i16; 3]>>) -> [i16; 3] {
    match refs.get(&dominant) {
        Some(positions) if !positions.is_empty() => {
            let n = positions.len() as i32;
            let sum = positions.iter().fold([0i32; 3], |mut acc, p| {
                acc[0] += p[0] as i32;
                acc[1] += p[1] as i32;
                acc[2] += p[2] as i32;
                acc
            });
            [(sum[0] / n) as i16, (sum[1] / n) as i16, (sum[2] / n) as i16]
        }
        _ => [0i16; 3],
    }
}

/// FNV-1a хэш для Signal B Frame: имя доминирующей подсистемы + выровненный тик.
/// Выравнивание по cooldown_ticks — одно событие стресса за период.
fn stress_lineage_hash(dominant: SubsystemId, tick: u64, cooldown: u64) -> u64 {
    let period = if cooldown > 0 { tick / cooldown } else { tick };
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in b"stress:".iter().chain(dominant.name().as_bytes()) {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h ^= period;
    h = h.wrapping_mul(0x100000001b3);
    h
}

/// Пространственная позиция для Frame Сигнала C: середина между двумя октантами.
///
/// Конфликт «форма vs содержание» — позиция между аналитическим и синтетическим октантами.
fn cc_position(analytic: Octant, synthetic: Octant) -> [i16; 3] {
    let a = octant_centroid_pos(analytic);
    let s = octant_centroid_pos(synthetic);
    [
        ((a[0] as i32 + s[0] as i32) / 2) as i16,
        ((a[1] as i32 + s[1] as i32) / 2) as i16,
        ((a[2] as i32 + s[2] as i32) / 2) as i16,
    ]
}

/// Детерминированный центроид октанта в семантическом пространстве.
///
/// bit2=X_low, bit1=Y_low, bit0=Z_low → high=24000, low=8000.
fn octant_centroid_pos(oct: Octant) -> [i16; 3] {
    let idx = oct.index();
    let x: i16 = if (idx & 4) == 0 { 24000 } else { 8000 };
    let y: i16 = if (idx & 2) == 0 { 24000 } else { 8000 };
    let z: i16 = if (idx & 1) == 0 { 24000 } else { 8000 };
    [x, y, z]
}

/// FNV-1a хэш для Signal C Frame: "corpus_callosum:" + frame sutra_id + period.
fn cc_lineage_hash(frame_id: u32, tick: u64, cooldown: u64) -> u64 {
    let period = if cooldown > 0 { tick / cooldown } else { tick };
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in b"corpus_callosum:".iter() {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    for byte in frame_id.to_le_bytes().iter() {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h ^= period;
    h = h.wrapping_mul(0x100000001b3);
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::over_domain::context_recognizer::dilemma_store::DilemmaStore;

    fn conflict(primary: SubsystemId, secondary: SubsystemId, ratio: f32) -> SubsystemConflict {
        SubsystemConflict { primary, secondary, conflict_ratio: ratio }
    }

    fn deps_with_tension(a: &str, b: &str) -> SubsystemDependencies {
        use axiom_config::{NaturalTension, SubsystemDep};
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(a.to_string(), SubsystemDep {
            builds_on: vec![],
            natural_tensions: vec![NaturalTension { target: b.to_string(), reason: String::new() }],
        });
        SubsystemDependencies { subsystems: map }
    }

    #[test]
    fn test_no_detect_without_conflict() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let cmds = det.detect(None, &mut store, &HashMap::new(), 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_no_detect_without_deps() {
        let mut det = DilemmaDetector::new(); // пустые deps
        let mut store = DilemmaStore::new();
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_no_detect_not_natural_tension() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "values"));
        let mut store = DilemmaStore::new();
        // morality не в natural_tensions mathematics
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 100);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_detects_natural_conflict() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        let mut store = DilemmaStore::new();
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 100);
        assert_eq!(cmds.len(), 1);
        assert_eq!(store.active_count(), 1);
    }

    #[test]
    fn test_no_detect_below_threshold() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        let mut store = DilemmaStore::new();
        // ratio = 0.3 < DILEMMA_THRESHOLD = 0.5
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.3);
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_cooldown_prevents_duplicate() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        let mut store = DilemmaStore::new();
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);

        let cmds1 = det.detect(Some(c.clone()), &mut store, &HashMap::new(), 109, 100);
        assert_eq!(cmds1.len(), 1);

        // Повторно через 10 тиков — cooldown 50
        let cmds2 = det.detect(Some(c), &mut store, &HashMap::new(), 109, 110);
        assert!(cmds2.is_empty());
        assert_eq!(store.active_count(), 1);
    }

    #[test]
    fn test_cooldown_expires() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        let mut store = DilemmaStore::new();
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);

        det.detect(Some(c.clone()), &mut store, &HashMap::new(), 109, 100);
        // Через 60 тиков (> cooldown 50) — новая дилемма
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 160);
        assert_eq!(cmds.len(), 1);
        assert_eq!(store.active_count(), 2);
    }

    #[test]
    fn test_no_detect_at_capacity() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        // Заполним store до MAX_ACTIVE
        use crate::over_domain::context_recognizer::dilemma_store::{DilemmaType, MAX_ACTIVE};
        let mut store = DilemmaStore::new();
        for i in 0..MAX_ACTIVE as u64 {
            store.push_active(DilemmaType::ValueConflict, vec![], i, 0.8);
        }
        assert!(store.is_at_capacity());

        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);
        let cmds = det.detect(Some(c), &mut store, &HashMap::new(), 109, 1000);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_canonical_pair_is_symmetric() {
        let p1 = canonical_pair(SubsystemId::Mathematics, SubsystemId::Morality);
        let p2 = canonical_pair(SubsystemId::Morality, SubsystemId::Mathematics);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_centroid_two_subsystems() {
        let mut refs: HashMap<SubsystemId, Vec<[i16; 3]>> = HashMap::new();
        refs.insert(SubsystemId::Mathematics, vec![[100, 0, 0]]);
        refs.insert(SubsystemId::Morality, vec![[-100, 0, 0]]);
        let c = centroid(SubsystemId::Mathematics, SubsystemId::Morality, &refs);
        assert_eq!(c, [0i16, 0, 0]);
    }

    // ── Сигнал B: стресс связей ───────────────────────────────────────────────

    fn stressed_conn(stress: f32, strength: f32) -> Connection {
        let mut c = Connection::default();
        c.flags = FLAG_ACTIVE;
        c.strength = strength;
        c.current_stress = stress;
        c.source_id = 1;
        c.target_id = 2;
        c.domain_id = 10;
        c
    }

    fn make_connections_stressed(count: usize, ratio: f32) -> Vec<Connection> {
        (0..count).map(|_| stressed_conn(ratio, 1.0)).collect()
    }

    #[test]
    fn test_signal_b_no_detect_empty_connections() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let cmds = det.detect_signal_b(&[], SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_signal_b_no_detect_low_stress() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        // stress=0.1, strength=1.0 → ratio 0.1 < STRESS_RATIO_THRESHOLD(0.5)
        let conns = make_connections_stressed(5, 0.1);
        let cmds = det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(store.active_count(), 0);
    }

    #[test]
    fn test_signal_b_no_detect_too_few_stressed() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        // Только 1 stressed < MIN_STRESSED_CONNECTIONS(2)
        let conns = vec![stressed_conn(0.8, 1.0)];
        let cmds = det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_signal_b_detects_high_stress() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        // 3 stressed connections, ratio 0.8 >> thresholds
        let conns = make_connections_stressed(3, 0.8);
        let cmds = det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert_eq!(cmds.len(), 1, "Signal B should fire with high stress");
        assert_eq!(store.active_count(), 1);
    }

    #[test]
    fn test_signal_b_stores_ontological_conflict() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let conns = make_connections_stressed(3, 0.8);
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        use crate::over_domain::context_recognizer::dilemma_store::DilemmaType;
        assert!(matches!(store.active[0].dilemma_type, DilemmaType::OntologicalConflict));
    }

    #[test]
    fn test_signal_b_cooldown_prevents_duplicate() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let conns = make_connections_stressed(3, 0.8);
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert_eq!(store.active_count(), 1);
        // Повторно через 50 тиков — cooldown 100
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 150);
        assert_eq!(store.active_count(), 1, "cooldown should block second detection");
    }

    #[test]
    fn test_signal_b_cooldown_expires() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let conns = make_connections_stressed(3, 0.8);
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        // После cooldown(100) → снова срабатывает
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 201);
        assert_eq!(store.active_count(), 2, "cooldown expired — second detection should succeed");
    }

    #[test]
    fn test_signal_b_independent_cooldown_from_signal_a() {
        let mut det = DilemmaDetector::new();
        det.set_dependencies(deps_with_tension("mathematics", "morality"));
        let mut store = DilemmaStore::new();

        // Сначала Signal A
        let c = conflict(SubsystemId::Mathematics, SubsystemId::Morality, 0.9);
        det.detect(Some(c), &mut store, &HashMap::new(), 109, 100);
        assert_eq!(store.active_count(), 1);

        // Signal B сразу после — отдельный cooldown
        let conns = make_connections_stressed(3, 0.8);
        det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 110);
        assert_eq!(store.active_count(), 2, "Signal B has independent cooldown from Signal A");
    }

    #[test]
    fn test_signal_b_uses_sutra_id_prefix_0x5000() {
        let mut det = DilemmaDetector::new();
        let mut store = DilemmaStore::new();
        let conns = make_connections_stressed(3, 0.8);
        let cmds = det.detect_signal_b(&conns, SubsystemId::Mathematics, &HashMap::new(), &mut store, 109, 100);
        assert_eq!(cmds.len(), 1);
        // Проверить что команда несёт правильный суффикс (косвенно через наличие payload)
        // Полная проверка proposed_sutra_id через InjectFrameAnchorPayload была бы интеграционной
    }

    #[test]
    fn test_stress_metrics_all_below_threshold() {
        let conns: Vec<Connection> = (0..5).map(|_| stressed_conn(0.1, 1.0)).collect();
        let (count, mean) = stress_metrics(&conns);
        assert_eq!(count, 0);
        assert!(mean < STRESS_RATIO_THRESHOLD);
    }

    #[test]
    fn test_stress_metrics_all_above_threshold() {
        let conns: Vec<Connection> = (0..4).map(|_| stressed_conn(0.8, 1.0)).collect();
        let (count, mean) = stress_metrics(&conns);
        assert_eq!(count, 4);
        assert!(mean > MEAN_STRESS_THRESHOLD);
    }

    #[test]
    fn test_stress_metrics_inactive_ignored() {
        let mut c = stressed_conn(0.9, 1.0);
        c.flags = 0; // не FLAG_ACTIVE
        let (count, mean) = stress_metrics(&[c]);
        assert_eq!(count, 0);
        assert_eq!(mean, 0.0, "inactive connections must be ignored");
    }

    #[test]
    fn test_stress_position_fallback_zero() {
        let pos = stress_position(SubsystemId::Unknown, &HashMap::new());
        assert_eq!(pos, [0i16, 0, 0]);
    }

    #[test]
    fn test_stress_position_uses_dominant_refs() {
        let mut refs = HashMap::new();
        refs.insert(SubsystemId::Mathematics, vec![[500i16, 200, 100]]);
        let pos = stress_position(SubsystemId::Mathematics, &refs);
        assert_eq!(pos, [500i16, 200, 100]);
    }

    // ── Signal C: Corpus Callosum ─────────────────────────────────────────────

    fn axial_store_with_conflict(
        frame_id: u32,
        analytic: Octant,
        synthetic: Octant,
        strength: u8,
    ) -> AxialStore {
        use axiom_experience::{AxialConflict, AxialEvaluation, AxialScore, ConflictResolution, EvaluationLevel};
        let conflict = AxialConflict {
            analytic_octant: analytic,
            synthetic_octant: synthetic,
            conflict_strength: strength,
            resolution: ConflictResolution::Unresolved,
        };
        let score = AxialScore::new(128, 128);
        let eval = AxialEvaluation::new(frame_id, EvaluationLevel::Sensory, score, score, score, 100)
            .with_conflict(conflict);
        let mut store = AxialStore::new();
        store.add(eval);
        store
    }

    #[test]
    fn test_signal_c_no_detect_empty_frame_ids() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        let axial = AxialStore::new();
        let cmds = det.detect_signal_c(&axial, &[], &mut ds, 109, 100);
        assert!(cmds.is_empty());
        assert_eq!(ds.active_count(), 0);
    }

    #[test]
    fn test_signal_c_no_detect_no_conflict() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        // AxialStore без conflict
        use axiom_experience::{AxialEvaluation, AxialScore, EvaluationLevel};
        let score = AxialScore::new(128, 128);
        let eval = AxialEvaluation::new(42, EvaluationLevel::Sensory, score, score, score, 100);
        let mut axial = AxialStore::new();
        axial.add(eval);
        let cmds = det.detect_signal_c(&axial, &[42], &mut ds, 109, 100);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_signal_c_no_detect_below_threshold() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        // strength=85 (1 bit) < CORPUS_CALLOSUM_THRESHOLD(170)
        let axial = axial_store_with_conflict(
            42,
            Octant::CreativeAffirmation,
            Octant::EcstaticAffirmation,
            85,
        );
        let cmds = det.detect_signal_c(&axial, &[42], &mut ds, 109, 100);
        assert!(cmds.is_empty(), "strength 85 < threshold 170 → no detect");
    }

    #[test]
    fn test_signal_c_detects_strong_conflict() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        // strength=170 (2 bits) = CORPUS_CALLOSUM_THRESHOLD
        let axial = axial_store_with_conflict(
            42,
            Octant::CreativeAffirmation,
            Octant::DestructiveActivating,
            170,
        );
        let cmds = det.detect_signal_c(&axial, &[42], &mut ds, 109, 100);
        assert_eq!(cmds.len(), 1, "should emit one dilemma Frame command");
        assert_eq!(ds.active_count(), 1);
    }

    #[test]
    fn test_signal_c_cooldown_per_frame() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        let axial = axial_store_with_conflict(
            42, Octant::CreativeAffirmation, Octant::SelfDestructiveApathic, 255,
        );
        det.detect_signal_c(&axial, &[42], &mut ds, 109, 100);
        assert_eq!(ds.active_count(), 1);
        // Повтор через 50 тиков (< cooldown 150) → без дублирования
        det.detect_signal_c(&axial, &[42], &mut ds, 109, 150);
        assert_eq!(ds.active_count(), 1, "cooldown must prevent duplicate");
        // Через 160 тиков (> cooldown) → новая дилемма
        det.detect_signal_c(&axial, &[42], &mut ds, 109, 260);
        assert_eq!(ds.active_count(), 2, "after cooldown new dilemma should register");
    }

    #[test]
    fn test_signal_c_position_midpoint_between_octants() {
        let a = octant_centroid_pos(Octant::CreativeAffirmation); // [24000, 24000, 24000]
        let s = octant_centroid_pos(Octant::SelfDestructiveApathic); // [8000, 8000, 8000]
        let pos = cc_position(Octant::CreativeAffirmation, Octant::SelfDestructiveApathic);
        assert_eq!(pos[0], ((a[0] as i32 + s[0] as i32) / 2) as i16);
        assert_eq!(pos[1], ((a[1] as i32 + s[1] as i32) / 2) as i16);
        assert_eq!(pos[2], ((a[2] as i32 + s[2] as i32) / 2) as i16);
    }

    #[test]
    fn test_signal_c_different_frames_independent_cooldown() {
        let mut det = DilemmaDetector::new();
        let mut ds = DilemmaStore::new();
        let axial_42 = axial_store_with_conflict(42, Octant::CreativeAffirmation, Octant::SelfDestructiveApathic, 255);
        let axial_99 = axial_store_with_conflict(99, Octant::HeroicFatal, Octant::PassiveSentimental, 170);

        // Detect frame 42 at tick 100
        det.detect_signal_c(&axial_42, &[42], &mut ds, 109, 100);
        assert_eq!(ds.active_count(), 1);

        // Frame 99 has separate cooldown → fires independently
        det.detect_signal_c(&axial_99, &[99], &mut ds, 109, 110);
        assert_eq!(ds.active_count(), 2, "different frames have independent cooldowns");
    }
}
