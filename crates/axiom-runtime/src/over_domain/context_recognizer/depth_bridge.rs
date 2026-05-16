// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Мост к SutraDepthStore.
// Источник: ContextRecognizer_V5_0.md §3, §4.2

use axiom_experience::SutraDepthStore;

use crate::over_domain::context_recognizer::scanning_plan::DepthRange;

/// Найти активный диапазон глубин для октанта из списка sutra_ids.
///
/// "Активный диапазон" — перцентили [10%, 90%] ненулевых глубин в данном октанте.
/// Если меньше 2 ненулевых значений — возвращает WORKING диапазон по умолчанию.
pub fn find_active_depth_range(
    store: &SutraDepthStore,
    sutra_ids: &[u32],
    octant_index: usize,
) -> DepthRange {
    let depths: Vec<u16> = sutra_ids
        .iter()
        .filter_map(|&id| store.get(id))
        .map(|entry| entry.depth_per_octant[octant_index])
        .filter(|&d| d > 0)
        .collect();

    if depths.len() < 2 {
        return DepthRange::WORKING;
    }

    let mut sorted = depths.clone();
    sorted.sort_unstable();
    let p10 = sorted[sorted.len() / 10];
    let p90 = sorted[sorted.len() * 9 / 10];
    DepthRange { min: p10, max: p90.max(p10 + 1) }
}

/// Проверить, имеет ли Frame (sutra_id) значительную глубину в данном октанте.
///
/// Порог: depth > threshold (по умолчанию 100).
pub fn is_deep_in_octant(
    store: &SutraDepthStore,
    sutra_id: u32,
    octant_index: usize,
    threshold: u16,
) -> bool {
    store
        .get(sutra_id)
        .map(|e| e.depth_per_octant[octant_index] > threshold)
        .unwrap_or(false)
}

/// Получить средние глубины по всем октантам для набора sutra_ids.
pub fn average_depths_per_octant(store: &SutraDepthStore, sutra_ids: &[u32]) -> [u32; 8] {
    let mut sums = [0u32; 8];
    let mut counts = [0u32; 8];
    for &id in sutra_ids {
        if let Some(entry) = store.get(id) {
            for i in 0..8 {
                sums[i] += entry.depth_per_octant[i] as u32;
                counts[i] += 1;
            }
        }
    }
    let mut result = [0u32; 8];
    for i in 0..8 {
        if counts[i] > 0 {
            result[i] = sums[i] / counts[i];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{Octant, SutraDepthStore};

    fn store_with_entries(entries: &[(u32, u16, usize)]) -> SutraDepthStore {
        let mut store = SutraDepthStore::new();
        for &(id, depth, octant_idx) in entries {
            let entry = store.get_or_create(id);
            entry.depth_per_octant[octant_idx] = depth;
        }
        store
    }

    #[test]
    fn test_find_active_range_empty_returns_working() {
        let store = SutraDepthStore::new();
        let range = find_active_depth_range(&store, &[], 0);
        assert_eq!(range.min, DepthRange::WORKING.min);
        assert_eq!(range.max, DepthRange::WORKING.max);
    }

    #[test]
    fn test_find_active_range_with_single_entry_returns_working() {
        let store = store_with_entries(&[(1, 500, 0)]);
        let range = find_active_depth_range(&store, &[1], 0);
        assert_eq!(range.min, DepthRange::WORKING.min);
    }

    #[test]
    fn test_is_deep_in_octant_true() {
        let store = store_with_entries(&[(1, 5000, 2)]);
        assert!(is_deep_in_octant(&store, 1, 2, 100));
    }

    #[test]
    fn test_is_deep_in_octant_false_below_threshold() {
        let store = store_with_entries(&[(1, 50, 2)]);
        assert!(!is_deep_in_octant(&store, 1, 2, 100));
    }

    #[test]
    fn test_is_deep_in_octant_missing_entry() {
        let store = SutraDepthStore::new();
        assert!(!is_deep_in_octant(&store, 999, 0, 0));
    }

    #[test]
    fn test_average_depths_empty_store() {
        let store = SutraDepthStore::new();
        let avgs = average_depths_per_octant(&store, &[1, 2, 3]);
        assert_eq!(avgs, [0u32; 8]);
    }

    #[test]
    fn test_average_depths_single_entry() {
        let store = store_with_entries(&[(1, 1000, 3)]);
        let avgs = average_depths_per_octant(&store, &[1]);
        assert_eq!(avgs[3], 1000);
        assert_eq!(avgs[0], 0);
    }

    #[test]
    fn test_octant_index_valid_range() {
        // Ensure octant indices 0..7 cover all Octant variants
        let _: [Octant; 8] = [
            Octant::CreativeAffirmation,
            Octant::EcstaticAffirmation,
            Octant::HeroicFatal,
            Octant::DestructiveActivating,
            Octant::IdealizedConsoling,
            Octant::PassiveSentimental,
            Octant::FormalDenying,
            Octant::SelfDestructiveApathic,
        ];
    }
}
