// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SutraDepthStore — хранилище глубины укоренённости Frame в SUTRA.
//
// SutraDepth — четвёртая ось (не координата токена!). Измеряет насколько
// Frame "укоренён" в системе, отдельно по каждому из 8 октантов.
//
// Источник: `docs/architecture/ContextRecognizer_V5_0.md §3`

use std::collections::HashMap;

/// Максимальный прирост глубины за один DREAM-цикл.
pub const MAX_GROWTH_PER_CYCLE: u16 = 100;

/// Убывание глубины за один DREAM-цикл (без активности).
pub const DECAY_PER_CYCLE: u16 = 5;

/// Глубина примитивов из YAML — максимальная, фиксированная.
pub const PRIMITIVE_DEPTH: u16 = u16::MAX; // 65535

/// Глубина Promoted Frame при переходе в SUTRA через CODEX.
pub const PROMOTED_DEPTH: u16 = 30000;

/// Запись глубины для одного Frame.
///
/// `depth_per_octant[i]` — глубина в октанте `i` (0..7).
/// 0 = поверхностный / новый. 65535 = примитив-якорь, неизменяем.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SutraDepthEntry {
    pub sutra_id: u32,
    pub depth_per_octant: [u16; 8],
    pub last_settle_event: u64,
    pub reactivation_count: u32,
}

impl SutraDepthEntry {
    pub fn new(sutra_id: u32) -> Self {
        Self {
            sutra_id,
            depth_per_octant: [0; 8],
            last_settle_event: 0,
            reactivation_count: 0,
        }
    }

    /// Примитив — максимальная глубина во всех октантах, не меняется.
    pub fn new_primitive(sutra_id: u32) -> Self {
        Self {
            sutra_id,
            depth_per_octant: [PRIMITIVE_DEPTH; 8],
            last_settle_event: 0,
            reactivation_count: 0,
        }
    }

    /// Максимальная глубина среди всех октантов.
    pub fn max_depth(&self) -> u16 {
        *self.depth_per_octant.iter().max().unwrap_or(&0)
    }

    /// Средняя глубина (целочисленная).
    pub fn avg_depth(&self) -> u16 {
        let sum: u32 = self.depth_per_octant.iter().map(|&d| d as u32).sum();
        (sum / 8) as u16
    }

    /// Является ли запись примитивом (все октанты на максимуме).
    pub fn is_primitive(&self) -> bool {
        self.depth_per_octant.iter().all(|&d| d == PRIMITIVE_DEPTH)
    }
}

/// Хранилище глубины укоренённости.
///
/// Обновляется только в DREAMING — горячий путь не трогает.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SutraDepthStore {
    entries: HashMap<u32, SutraDepthEntry>,
}

impl SutraDepthStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, sutra_id: u32) -> Option<&SutraDepthEntry> {
        self.entries.get(&sutra_id)
    }

    pub fn get_mut(&mut self, sutra_id: u32) -> Option<&mut SutraDepthEntry> {
        self.entries.get_mut(&sutra_id)
    }

    /// Создать новую запись с нулевой глубиной. Если уже есть — вернуть существующую.
    pub fn get_or_create(&mut self, sutra_id: u32) -> &mut SutraDepthEntry {
        self.entries
            .entry(sutra_id)
            .or_insert_with(|| SutraDepthEntry::new(sutra_id))
    }

    /// Зарегистрировать примитив (глубина 65535 во всех октантах, не меняется).
    pub fn register_primitive(&mut self, sutra_id: u32) {
        self.entries
            .entry(sutra_id)
            .or_insert_with(|| SutraDepthEntry::new_primitive(sutra_id));
    }

    /// Обновить глубину в одном октанте на основе числа активаций за DREAM.
    /// Примитивы (depth == PRIMITIVE_DEPTH) не меняются.
    pub fn apply_evidence(
        &mut self,
        sutra_id: u32,
        octant: usize,
        evidence: u16,
        current_event: u64,
    ) {
        debug_assert!(octant < 8, "octant must be 0..7");
        let entry = self.get_or_create(sutra_id);
        if entry.depth_per_octant[octant] == PRIMITIVE_DEPTH {
            return;
        }
        entry.last_settle_event = current_event;
        if evidence > 0 {
            let growth = evidence.min(MAX_GROWTH_PER_CYCLE);
            entry.depth_per_octant[octant] =
                entry.depth_per_octant[octant].saturating_add(growth);
            entry.reactivation_count = entry.reactivation_count.saturating_add(1);
        } else {
            entry.depth_per_octant[octant] =
                entry.depth_per_octant[octant].saturating_sub(DECAY_PER_CYCLE);
        }
    }

    /// Применить decay ко всем не-примитивным записям (вызывается в DREAMING).
    pub fn apply_global_decay(&mut self, current_event: u64) {
        for entry in self.entries.values_mut() {
            if entry.is_primitive() {
                continue;
            }
            for depth in &mut entry.depth_per_octant {
                *depth = depth.saturating_sub(DECAY_PER_CYCLE);
            }
            entry.last_settle_event = current_event;
        }
    }

    /// Установить глубину Promoted Frame (скачок до PROMOTED_DEPTH в указанном октанте).
    pub fn set_promoted_depth(&mut self, sutra_id: u32, octant: usize, current_event: u64) {
        debug_assert!(octant < 8);
        let entry = self.get_or_create(sutra_id);
        entry.depth_per_octant[octant] = PROMOTED_DEPTH;
        entry.last_settle_event = current_event;
    }

    pub fn remove(&mut self, sutra_id: u32) -> Option<SutraDepthEntry> {
        self.entries.remove(&sutra_id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Average SutraDepth per octant across all entries (non-primitives included).
    /// Returns [0; 8] if the store is empty.
    pub fn avg_depths(&self) -> [u32; 8] {
        if self.entries.is_empty() {
            return [0; 8];
        }
        let mut sums = [0u64; 8];
        let count = self.entries.len() as u64;
        for entry in self.entries.values() {
            for i in 0..8 {
                sums[i] += entry.depth_per_octant[i] as u64;
            }
        }
        let mut out = [0u32; 8];
        for i in 0..8 {
            out[i] = (sums[i] / count) as u32;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry_zero_depth() {
        let mut store = SutraDepthStore::new();
        let e = store.get_or_create(1);
        assert_eq!(e.depth_per_octant, [0; 8]);
        assert_eq!(e.max_depth(), 0);
    }

    #[test]
    fn test_primitive_all_max() {
        let mut store = SutraDepthStore::new();
        store.register_primitive(42);
        let e = store.get(42).unwrap();
        assert!(e.is_primitive());
        assert_eq!(e.max_depth(), PRIMITIVE_DEPTH);
    }

    #[test]
    fn test_primitive_depth_immutable() {
        let mut store = SutraDepthStore::new();
        store.register_primitive(1);
        store.apply_evidence(1, 0, 50, 100);
        assert_eq!(store.get(1).unwrap().depth_per_octant[0], PRIMITIVE_DEPTH);
    }

    #[test]
    fn test_apply_evidence_grows() {
        let mut store = SutraDepthStore::new();
        store.apply_evidence(1, 3, 200, 1);
        // evidence=200 > MAX_GROWTH_PER_CYCLE=100 → clamp к 100
        assert_eq!(store.get(1).unwrap().depth_per_octant[3], 100);
    }

    #[test]
    fn test_apply_evidence_capped_at_max_growth() {
        let mut store = SutraDepthStore::new();
        store.apply_evidence(1, 0, 500, 1);
        assert_eq!(store.get(1).unwrap().depth_per_octant[0], MAX_GROWTH_PER_CYCLE);
    }

    #[test]
    fn test_apply_evidence_decay_when_no_evidence() {
        let mut store = SutraDepthStore::new();
        store.apply_evidence(1, 0, 50, 1);
        store.apply_evidence(1, 0, 0, 2);
        assert_eq!(store.get(1).unwrap().depth_per_octant[0], 50 - DECAY_PER_CYCLE);
    }

    #[test]
    fn test_decay_does_not_underflow() {
        let mut store = SutraDepthStore::new();
        store.apply_evidence(1, 0, 0, 1);
        assert_eq!(store.get(1).unwrap().depth_per_octant[0], 0);
    }

    #[test]
    fn test_global_decay_skips_primitives() {
        let mut store = SutraDepthStore::new();
        store.register_primitive(1);
        store.apply_evidence(2, 0, 50, 1);
        store.apply_global_decay(2);
        assert_eq!(store.get(1).unwrap().depth_per_octant[0], PRIMITIVE_DEPTH);
        assert_eq!(store.get(2).unwrap().depth_per_octant[0], 50 - DECAY_PER_CYCLE);
    }

    #[test]
    fn test_set_promoted_depth() {
        let mut store = SutraDepthStore::new();
        store.set_promoted_depth(5, 2, 1000);
        assert_eq!(store.get(5).unwrap().depth_per_octant[2], PROMOTED_DEPTH);
        assert_eq!(store.get(5).unwrap().depth_per_octant[0], 0);
    }

    #[test]
    fn test_remove() {
        let mut store = SutraDepthStore::new();
        store.get_or_create(1);
        assert_eq!(store.len(), 1);
        store.remove(1);
        assert_eq!(store.len(), 0);
    }
}
