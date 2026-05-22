// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MetaSubsystemId + MetaStore — мета-режимы второго порядка (CR-V6 Фаза C).
//
// Источник: ContextRecognizer_Roadmap_V6_V9.md §1.3

use std::collections::HashMap;

/// Идентификатор мета-подсистемы (режима второго порядка).
///
/// Диапазон 0x1001–0x1007 зарезервирован для стандартных мета-режимов.
/// Пользовательские мета-режимы: 0x1100+.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetaSubsystemId(pub u16);

pub const META_ANALYSIS: MetaSubsystemId = MetaSubsystemId(0x1001);
pub const META_SYNTHESIS: MetaSubsystemId = MetaSubsystemId(0x1002);
pub const META_REFLECTION: MetaSubsystemId = MetaSubsystemId(0x1003);
pub const META_PERCEPTION: MetaSubsystemId = MetaSubsystemId(0x1004);
pub const META_RECALL: MetaSubsystemId = MetaSubsystemId(0x1005);
pub const META_IMAGINATION: MetaSubsystemId = MetaSubsystemId(0x1006);
pub const META_DIALOGUE: MetaSubsystemId = MetaSubsystemId(0x1007);

impl MetaSubsystemId {
    pub fn name(self) -> &'static str {
        match self.0 {
            0x1001 => "meta_analysis",
            0x1002 => "meta_synthesis",
            0x1003 => "meta_reflection",
            0x1004 => "meta_perception",
            0x1005 => "meta_recall",
            0x1006 => "meta_imagination",
            0x1007 => "meta_dialogue",
            _ => "meta_unknown",
        }
    }
}

/// Текущая активация мета-подсистемы.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetaActivation {
    /// Степень уверенности совпадения паттерна (0.0..1.0).
    pub confidence: f32,
    /// event_id последнего матча.
    pub last_matched_event: u64,
}

/// Хранилище активных мета-подсистем.
///
/// Хранится в ContextRecognizer. Переносится в axiom-experience — V7 (tech debt).
#[derive(Debug, Default)]
pub struct MetaStore {
    store: HashMap<MetaSubsystemId, MetaActivation>,
}

impl MetaStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Активировать или обновить мета-подсистему.
    pub fn activate(&mut self, meta_id: MetaSubsystemId, confidence: f32, event_id: u64) {
        self.store.insert(meta_id, MetaActivation { confidence, last_matched_event: event_id });
    }

    /// Получить текущую активацию (None если не активна).
    pub fn get(&self, meta_id: MetaSubsystemId) -> Option<&MetaActivation> {
        self.store.get(&meta_id)
    }

    /// Число активных мета-подсистем.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Итератор по всем активным записям.
    pub fn iter(&self) -> impl Iterator<Item = (&MetaSubsystemId, &MetaActivation)> {
        self.store.iter()
    }

    /// Наиболее уверенная активная мета-подсистема.
    pub fn dominant(&self) -> Option<MetaSubsystemId> {
        self.store
            .iter()
            .max_by(|a, b| a.1.confidence.partial_cmp(&b.1.confidence).unwrap())
            .map(|(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_subsystem_id_names() {
        assert_eq!(META_ANALYSIS.name(), "meta_analysis");
        assert_eq!(META_SYNTHESIS.name(), "meta_synthesis");
        assert_eq!(META_REFLECTION.name(), "meta_reflection");
        assert_eq!(MetaSubsystemId(0xFFFF).name(), "meta_unknown");
    }

    #[test]
    fn test_meta_store_empty() {
        let store = MetaStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.dominant().is_none());
    }

    #[test]
    fn test_meta_store_activate_and_get() {
        let mut store = MetaStore::new();
        store.activate(META_ANALYSIS, 0.8, 100);
        let act = store.get(META_ANALYSIS).unwrap();
        assert!((act.confidence - 0.8).abs() < 1e-6);
        assert_eq!(act.last_matched_event, 100);
    }

    #[test]
    fn test_meta_store_dominant() {
        let mut store = MetaStore::new();
        store.activate(META_ANALYSIS, 0.6, 10);
        store.activate(META_SYNTHESIS, 0.9, 20);
        store.activate(META_REFLECTION, 0.4, 30);
        assert_eq!(store.dominant(), Some(META_SYNTHESIS));
    }

    #[test]
    fn test_meta_store_update_overwrites() {
        let mut store = MetaStore::new();
        store.activate(META_ANALYSIS, 0.5, 10);
        store.activate(META_ANALYSIS, 0.9, 50);
        assert_eq!(store.len(), 1);
        assert_eq!(store.get(META_ANALYSIS).unwrap().last_matched_event, 50);
    }
}
