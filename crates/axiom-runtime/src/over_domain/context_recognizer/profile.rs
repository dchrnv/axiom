// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Построение и обновление InterpretationProfile из энергий подсистем.
// Источник: ContextRecognizer_V5_0.md §9.2

use axiom_experience::{
    ContextSnapshot, FrameComposition, InterpretationProfile, InterpretationProfileStore, Octant,
    SubsystemId,
};
use std::collections::HashMap;

/// Создать или обновить InterpretationProfile для Frame-анкера.
pub fn upsert_profile(
    store: &mut InterpretationProfileStore,
    frame_anchor_id: u32,
    weights: HashMap<SubsystemId, u8>,
    primary: SubsystemId,
    primary_octant: Octant,
    composition: FrameComposition,
    context: ContextSnapshot,
) {
    if let Some(profile) = store.get_mut(frame_anchor_id) {
        for (&subsystem, &weight) in &weights {
            profile.update_weight(subsystem, weight, context.event_id);
        }
    } else {
        let mut profile = InterpretationProfile::new(
            frame_anchor_id,
            primary,
            primary_octant,
            composition,
            context.clone(),
        );
        for (&subsystem, &weight) in &weights {
            profile.update_weight(subsystem, weight, context.event_id);
        }
        store.insert(profile);
    }
}

/// Построить ContextSnapshot из текущего состояния.
pub fn build_snapshot(
    primary_subsystem: SubsystemId,
    primary_octant: Octant,
    event_id: u64,
) -> ContextSnapshot {
    ContextSnapshot {
        primary_subsystem,
        primary_octant,
        event_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(event_id: u64) -> ContextSnapshot {
        build_snapshot(SubsystemId::Writing, Octant::CreativeAffirmation, event_id)
    }

    #[test]
    fn test_upsert_creates_new_profile() {
        let mut store = InterpretationProfileStore::new();
        let mut weights = HashMap::new();
        weights.insert(SubsystemId::Writing, 200u8);
        upsert_profile(
            &mut store,
            1,
            weights,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        );
        assert_eq!(store.len(), 1);
        let p = store.get(1).unwrap();
        assert_eq!(p.primary, SubsystemId::Writing);
    }

    #[test]
    fn test_upsert_updates_existing_profile() {
        let mut store = InterpretationProfileStore::new();
        let mut weights = HashMap::new();
        weights.insert(SubsystemId::Writing, 200u8);
        upsert_profile(
            &mut store,
            1,
            weights,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        );
        // Now update with Mathematics taking over
        let mut weights2 = HashMap::new();
        weights2.insert(SubsystemId::Mathematics, 240u8);
        upsert_profile(
            &mut store,
            1,
            weights2,
            SubsystemId::Mathematics,
            Octant::HeroicFatal,
            FrameComposition::C1Atom,
            ctx(2),
        );
        // Store should still have 1 profile (updated)
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_build_snapshot() {
        let snap = build_snapshot(SubsystemId::Mathematics, Octant::FormalDenying, 42);
        assert_eq!(snap.primary_subsystem, SubsystemId::Mathematics);
        assert_eq!(snap.primary_octant, Octant::FormalDenying);
        assert_eq!(snap.event_id, 42);
    }
}
