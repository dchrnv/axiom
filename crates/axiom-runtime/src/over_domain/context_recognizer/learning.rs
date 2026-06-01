// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Обновление SutraDepth в DREAM Phase. V1: предоставляет функции для вызова из DreamPhase.
// Источник: ContextRecognizer_V5_0.md §3.4

use axiom_experience::{Octant, SutraDepthStore, MAX_GROWTH_PER_CYCLE};

/// Обновить глубины в DREAM Phase для набора активных Frame.
///
/// Вызывается только из DREAMING фазы.
///
/// `activations_per_frame`: sutra_id → count активаций по октантам с прошлого DREAM.
pub fn apply_dream_depth_update(
    store: &mut SutraDepthStore,
    activations: &[(u32, Octant, u32)], // (sutra_id, octant, count)
    all_known_ids: &[u32],
    event_id: u64,
) {
    // Применить рост для Frame с активностью
    for &(sutra_id, octant, count) in activations {
        let evidence = count.min(MAX_GROWTH_PER_CYCLE as u32) as u16;
        store.apply_evidence(sutra_id, octant.index(), evidence, event_id);
        // EMERGENT-TD-02: гранулярный reactivation_count = число CR-тиков активности.
        // Быстрее растёт чем старый +1 за DREAM-цикл — лучше отражает реальную частоту.
        store.record_reactivations(sutra_id, count);
    }

    // Применить decay для Frame без активности (по всем октантам через apply_evidence с 0)
    let active_ids: std::collections::HashSet<u32> =
        activations.iter().map(|&(id, _, _)| id).collect();
    for &id in all_known_ids {
        if !active_ids.contains(&id) {
            for octant_idx in 0..8 {
                store.apply_evidence(id, octant_idx, 0, event_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{SutraDepthStore, PRIMITIVE_DEPTH};

    #[test]
    fn test_depth_grows_with_activity() {
        let mut store = SutraDepthStore::new();
        store.get_or_create(1); // initial depth = 0

        apply_dream_depth_update(&mut store, &[(1, Octant::CreativeAffirmation, 50)], &[1], 1);

        let entry = store.get(1).unwrap();
        assert!(entry.depth_per_octant[0] > 0);
    }

    #[test]
    fn test_inactive_frame_decays() {
        let mut store = SutraDepthStore::new();
        let entry = store.get_or_create(2);
        entry.depth_per_octant[0] = 100;

        apply_dream_depth_update(&mut store, &[], &[2], 2);

        let entry = store.get(2).unwrap();
        assert!(entry.depth_per_octant[0] < 100);
    }

    #[test]
    fn test_primitive_depth_immutable() {
        let mut store = SutraDepthStore::new();
        store.register_primitive(5);

        // Even with activity, primitives stay at PRIMITIVE_DEPTH
        apply_dream_depth_update(
            &mut store,
            &[(5, Octant::CreativeAffirmation, 1000)],
            &[5],
            1,
        );

        let entry = store.get(5).unwrap();
        assert_eq!(entry.depth_per_octant[0], PRIMITIVE_DEPTH);
    }

    #[test]
    fn test_sutra_depth_only_in_dreaming() {
        // This test documents the invariant:
        // apply_dream_depth_update must ONLY be called from DREAMING phase.
        // The function itself does not enforce this (caller responsibility).
        // Test simply verifies the function exists and works.
        let mut store = SutraDepthStore::new();
        store.get_or_create(10);
        apply_dream_depth_update(&mut store, &[], &[10], 0);
        // No panic = correct
    }
}
