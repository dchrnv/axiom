// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DilemmaDetector V2.0 — Сигнал A: конфликт подсистем.
//
// Источник: DilemmaDetector_V2_0.md §4
//
// Алгоритм:
//   1. Взять SubsystemConflict из conflicts::detect_conflict.
//   2. Проверить is_natural_tension по SubsystemDependencies.
//   3. Вычислить tension_score. Если < DILEMMA_THRESHOLD — отбросить.
//   4. Cooldown: не дублировать ту же пару раньше DETECTION_COOLDOWN_TICKS.
//   5. push_active() в DilemmaStore.
//   6. Вернуть UCL InjectToken для кристаллизации Frame в EXPERIENCE.

use std::collections::HashMap;

use axiom_config::SubsystemDependencies;
use axiom_core::{STATE_ACTIVE, TOKEN_FLAG_DILEMMA, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_experience::SubsystemId;
use axiom_ucl::{flags as ucl_flags, InjectFrameAnchorPayload, OpCode, UclCommand};

use crate::over_domain::context_recognizer::conflicts::SubsystemConflict;
use crate::over_domain::context_recognizer::dilemma_store::{DilemmaStore, DilemmaType};

use super::tension::{compute_tension_score, DILEMMA_THRESHOLD};

/// Тики CR между повторными регистрациями одной пары подсистем как дилеммы.
const DETECTION_COOLDOWN_TICKS: u64 = 50;

/// Размер скользящего окна ко-активации (в инъекциях токенов).
const COACTIVATION_WINDOW: usize = 32;

/// Минимальное число появлений КАЖДОЙ подсистемы в окне для детектирования.
const MIN_COACTIVATION_COUNT: usize = 2;

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

/// Детектор дилемм V2.0 — встроен в ContextRecognizer.
pub struct DilemmaDetector {
    deps: SubsystemDependencies,
    /// Последний тик CR детектирования для каждой канонической пары.
    last_detected: HashMap<(SubsystemId, SubsystemId), u64>,
    /// Скользящее окно ко-активации: последние N subsystem-активаций из record_injection_signal.
    /// Каждый элемент: (SubsystemId, tick).
    coactivation_window: std::collections::VecDeque<(SubsystemId, u64)>,
}

impl DilemmaDetector {
    pub fn new() -> Self {
        Self {
            deps: SubsystemDependencies::default(),
            last_detected: HashMap::new(),
            coactivation_window: std::collections::VecDeque::with_capacity(COACTIVATION_WINDOW + 1),
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
}

impl Default for DilemmaDetector {
    fn default() -> Self {
        Self::new()
    }
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
}
