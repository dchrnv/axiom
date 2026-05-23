// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// InterpretationProfileStore — профили интерпретации Frame по подсистемам.
//
// ContextRecognizer записывает сюда для каждого Frame:
//   какие подсистемы активны, с какими весами, в каком октанте.
//
// Источник: `docs/architecture/ContextRecognizer_V5_0.md §9.2`

use crate::types::{ContextSnapshot, FrameComposition, Octant, SubsystemId};
use std::collections::HashMap;

/// Профиль интерпретации одного Frame.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterpretationProfile {
    pub frame_anchor_sutra_id: u32,
    /// Веса активных подсистем (0..255). SubsystemId → вес.
    pub weights: HashMap<SubsystemId, u8>,
    /// Доминирующая подсистема.
    pub primary: SubsystemId,
    pub last_updated_event: u64,
    /// Снимок контекста в момент последнего обновления.
    pub last_context: ContextSnapshot,
    /// Уровень композиции Frame (примитив / атом / молекула / ...).
    pub frame_composition: FrameComposition,
    /// Октант к которому Frame преимущественно тяготеет.
    pub primary_octant: Octant,
}

impl InterpretationProfile {
    pub fn new(
        frame_anchor_sutra_id: u32,
        primary: SubsystemId,
        primary_octant: Octant,
        frame_composition: FrameComposition,
        context: ContextSnapshot,
    ) -> Self {
        let mut weights = HashMap::new();
        weights.insert(primary, 255);
        Self {
            frame_anchor_sutra_id,
            weights,
            primary,
            last_updated_event: context.event_id,
            last_context: context,
            frame_composition,
            primary_octant,
        }
    }

    /// Обновить вес подсистемы. Если вес доминирующей стал меньше другой — сменить primary.
    pub fn update_weight(&mut self, subsystem: SubsystemId, weight: u8, event_id: u64) {
        self.weights.insert(subsystem, weight);
        self.last_updated_event = event_id;
        // Пересчитать dominant
        if let Some((&new_primary, _)) = self.weights.iter().max_by_key(|(_, &w)| w) {
            self.primary = new_primary;
        }
    }

    /// Вес подсистемы (0 если не зарегистрирована).
    pub fn weight(&self, subsystem: SubsystemId) -> u8 {
        *self.weights.get(&subsystem).unwrap_or(&0)
    }

    /// Признак сильной принадлежности к подсистеме (вес > 200).
    pub fn is_strongly(&self, subsystem: SubsystemId) -> bool {
        self.weight(subsystem) > 200
    }
}

/// Хранилище профилей интерпретации.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterpretationProfileStore {
    profiles: HashMap<u32, InterpretationProfile>,
}

impl InterpretationProfileStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, profile: InterpretationProfile) {
        self.profiles.insert(profile.frame_anchor_sutra_id, profile);
    }

    pub fn get(&self, sutra_id: u32) -> Option<&InterpretationProfile> {
        self.profiles.get(&sutra_id)
    }

    pub fn get_mut(&mut self, sutra_id: u32) -> Option<&mut InterpretationProfile> {
        self.profiles.get_mut(&sutra_id)
    }

    pub fn remove(&mut self, sutra_id: u32) -> Option<InterpretationProfile> {
        self.profiles.remove(&sutra_id)
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    /// Frame с данной доминирующей подсистемой.
    pub fn frames_with_primary<'a>(
        &'a self,
        subsystem: SubsystemId,
    ) -> impl Iterator<Item = &'a InterpretationProfile> {
        self.profiles
            .values()
            .filter(move |p| p.primary == subsystem)
    }

    /// Доминирующая подсистема (по числу Frame с таким primary).
    pub fn dominant_primary(&self) -> Option<SubsystemId> {
        self.dominant_primary_as_u8().map(|v| match v {
            0 => SubsystemId::Writing,
            1 => SubsystemId::Mathematics,
            2 => SubsystemId::Music,
            3 => SubsystemId::Time,
            4 => SubsystemId::Logic,
            5 => SubsystemId::Values,
            _ => SubsystemId::Unknown,
        })
    }

    /// Most common primary subsystem as u8. Encoding: Writing=0 Mathematics=1 Music=2 Time=3 Logic=4 Values=5 Unknown=6.
    pub fn dominant_primary_as_u8(&self) -> Option<u8> {
        if self.profiles.is_empty() {
            return None;
        }
        let mut counts = [0u32; 7];
        for p in self.profiles.values() {
            counts[subsystem_to_u8(p.primary) as usize] += 1;
        }
        counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i as u8)
    }
}

fn subsystem_to_u8(s: SubsystemId) -> u8 {
    match s {
        SubsystemId::Writing => 0,
        SubsystemId::Mathematics => 1,
        SubsystemId::Music => 2,
        SubsystemId::Time => 3,
        SubsystemId::Logic => 4,
        SubsystemId::Values => 5,
        SubsystemId::Unknown => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(event_id: u64) -> ContextSnapshot {
        ContextSnapshot {
            primary_subsystem: SubsystemId::Writing,
            primary_octant: Octant::CreativeAffirmation,
            event_id,
        }
    }

    #[test]
    fn test_new_profile_primary_weight_255() {
        let p = InterpretationProfile::new(
            1,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        );
        assert_eq!(p.weight(SubsystemId::Writing), 255);
        assert_eq!(p.primary, SubsystemId::Writing);
    }

    #[test]
    fn test_update_weight_changes_primary() {
        let mut p = InterpretationProfile::new(
            1,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        );
        p.update_weight(SubsystemId::Mathematics, 200, 2);
        // Writing=255 > Mathematics=200 → primary остаётся Writing
        assert_eq!(p.primary, SubsystemId::Writing);

        p.update_weight(SubsystemId::Mathematics, 255, 3);
        // Теперь оба 255 — один из них победит (HashMap не гарантирует порядок, но оба корректны)
        assert!(
            p.primary == SubsystemId::Writing || p.primary == SubsystemId::Mathematics
        );
    }

    #[test]
    fn test_is_strongly() {
        let p = InterpretationProfile::new(
            1,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        );
        assert!(p.is_strongly(SubsystemId::Writing));
        assert!(!p.is_strongly(SubsystemId::Mathematics));
    }

    #[test]
    fn test_store_insert_get_remove() {
        let mut store = InterpretationProfileStore::new();
        let p = InterpretationProfile::new(
            1,
            SubsystemId::Mathematics,
            Octant::HeroicFatal,
            FrameComposition::C2Molecule,
            ctx(1),
        );
        store.insert(p);
        assert_eq!(store.len(), 1);
        assert!(store.get(1).is_some());
        store.remove(1);
        assert!(store.is_empty());
    }

    #[test]
    fn test_frames_with_primary() {
        let mut store = InterpretationProfileStore::new();
        store.insert(InterpretationProfile::new(
            1,
            SubsystemId::Writing,
            Octant::CreativeAffirmation,
            FrameComposition::C1Atom,
            ctx(1),
        ));
        store.insert(InterpretationProfile::new(
            2,
            SubsystemId::Mathematics,
            Octant::HeroicFatal,
            FrameComposition::C2Molecule,
            ctx(1),
        ));
        let writing: Vec<_> = store.frames_with_primary(SubsystemId::Writing).collect();
        assert_eq!(writing.len(), 1);
        assert_eq!(writing[0].frame_anchor_sutra_id, 1);
    }
}
