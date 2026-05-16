// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// EmergentPrimitiveStore — хранилище эмерджентных примитивов.
//
// Эмерджентные примитивы обнаруживаются ContextRecognizer как устойчивые паттерны
// активности, не совпадающие с известными примитивами. Требуют одобрения chrnv.
//
// Источник: `docs/architecture/ContextRecognizer_V5_0.md §7, §10 (max 1000)`

use crate::types::Octant;

/// Максимальное число эмерджентных примитивов (инвариант из спеки).
pub const MAX_EMERGENT_PRIMITIVES: usize = 1000;

/// Эмерджентный примитив — новый атомарный паттерн, обнаруженный системой.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmergentPrimitive {
    pub sutra_id: u32,
    pub discovered_at_event: u64,
    /// Октант в котором был обнаружен (там и наибольшая начальная глубина).
    pub discovery_octant: Octant,
    /// Начальная глубина в октанте обнаружения.
    pub initial_depth: u16,
    /// Одобрен chrnv для постоянного включения.
    pub approved: bool,
}

impl EmergentPrimitive {
    pub fn new(sutra_id: u32, discovered_at_event: u64, discovery_octant: Octant, initial_depth: u16) -> Self {
        Self {
            sutra_id,
            discovered_at_event,
            discovery_octant,
            initial_depth,
            approved: false,
        }
    }
}

/// Хранилище эмерджентных примитивов.
///
/// Ограничено MAX_EMERGENT_PRIMITIVES записями — не растёт бесконечно.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmergentPrimitiveStore {
    primitives: Vec<EmergentPrimitive>,
}

impl EmergentPrimitiveStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавить эмерджентный примитив.
    /// Возвращает `false` если хранилище заполнено (MAX_EMERGENT_PRIMITIVES).
    pub fn add(&mut self, primitive: EmergentPrimitive) -> bool {
        if self.primitives.len() >= MAX_EMERGENT_PRIMITIVES {
            return false;
        }
        self.primitives.push(primitive);
        true
    }

    pub fn get_all(&self) -> &[EmergentPrimitive] {
        &self.primitives
    }

    pub fn get_approved(&self) -> impl Iterator<Item = &EmergentPrimitive> {
        self.primitives.iter().filter(|p| p.approved)
    }

    pub fn get_pending(&self) -> impl Iterator<Item = &EmergentPrimitive> {
        self.primitives.iter().filter(|p| !p.approved)
    }

    /// Одобрить примитив по sutra_id. Возвращает `true` если найден и одобрен.
    pub fn approve(&mut self, sutra_id: u32) -> bool {
        if let Some(p) = self.primitives.iter_mut().find(|p| p.sutra_id == sutra_id) {
            p.approved = true;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    pub fn is_empty(&self) -> bool {
        self.primitives.is_empty()
    }

    pub fn is_at_capacity(&self) -> bool {
        self.primitives.len() >= MAX_EMERGENT_PRIMITIVES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prim(sutra_id: u32) -> EmergentPrimitive {
        EmergentPrimitive::new(sutra_id, 100, Octant::CreativeAffirmation, 5000)
    }

    #[test]
    fn test_add_and_get() {
        let mut store = EmergentPrimitiveStore::new();
        assert!(store.add(prim(1)));
        assert_eq!(store.len(), 1);
        assert!(!store.get_all()[0].approved);
    }

    #[test]
    fn test_approve() {
        let mut store = EmergentPrimitiveStore::new();
        store.add(prim(1));
        assert!(store.approve(1));
        assert!(store.get_all()[0].approved);
        assert!(!store.approve(99)); // несуществующий
    }

    #[test]
    fn test_get_approved_and_pending() {
        let mut store = EmergentPrimitiveStore::new();
        store.add(prim(1));
        store.add(prim(2));
        store.approve(1);
        assert_eq!(store.get_approved().count(), 1);
        assert_eq!(store.get_pending().count(), 1);
    }

    #[test]
    fn test_capacity_limit() {
        let mut store = EmergentPrimitiveStore::new();
        for i in 0..MAX_EMERGENT_PRIMITIVES as u32 {
            assert!(store.add(prim(i)));
        }
        assert!(store.is_at_capacity());
        assert!(!store.add(prim(MAX_EMERGENT_PRIMITIVES as u32)));
        assert_eq!(store.len(), MAX_EMERGENT_PRIMITIVES);
    }

    #[test]
    fn test_empty_store() {
        let store = EmergentPrimitiveStore::new();
        assert!(store.is_empty());
        assert!(!store.is_at_capacity());
        assert_eq!(store.get_approved().count(), 0);
    }
}
