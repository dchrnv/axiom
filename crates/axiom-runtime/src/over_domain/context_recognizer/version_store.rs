// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SubsystemVersionStore (V7-D1) — отслеживание версий подсистем.
//
// Когда AnchorSet перезагружается (hot-reload), CR проверяет изменились ли версии.
// Изменившиеся подсистемы помечаются как stale — DREAM или chrnv решают что делать с профилями.

use std::collections::HashMap;

/// Запись версии подсистемы.
#[derive(Debug, Clone)]
pub struct SubsystemVersionEntry {
    pub name: String,
    pub version: String,
}

/// Хранилище версий подсистем с migration trace (V7-D1).
///
/// Позволяет обнаружить когда версия yaml-примитивов изменилась после hot-reload.
/// InterpretationProfiles для изменившихся подсистем помечаются как stale.
#[derive(Debug, Default)]
pub struct SubsystemVersionStore {
    known: HashMap<String, String>,
    /// Подсистемы, версия которых изменилась с последнего check_migration().
    stale: Vec<String>,
}

impl SubsystemVersionStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Инициализировать из первичной загрузки AnchorSet.
    /// Не генерирует stale — это первичный snapshot.
    pub fn init(&mut self, versions: &HashMap<String, String>) {
        self.known.clone_from(versions);
        self.stale.clear();
    }

    /// Проверить новые версии на изменение. Возвращает имена изменившихся подсистем.
    ///
    /// Добавляет к stale: подсистемы с изменённой версией или вновь появившиеся.
    pub fn check_migration(&mut self, new_versions: &HashMap<String, String>) -> Vec<String> {
        let mut changed = Vec::new();
        for (name, new_ver) in new_versions {
            match self.known.get(name.as_str()) {
                Some(old_ver) if old_ver != new_ver => {
                    changed.push(name.clone());
                }
                None => {
                    changed.push(name.clone()); // новая подсистема
                }
                _ => {}
            }
        }
        self.known.clone_from(new_versions);
        for name in &changed {
            if !self.stale.contains(name) {
                self.stale.push(name.clone());
            }
        }
        changed
    }

    /// Список подсистем, помеченных stale с момента последнего drain.
    pub fn stale_subsystems(&self) -> &[String] {
        &self.stale
    }

    /// Очистить список stale (вызывается после того как DREAM/chrnv обработал миграцию).
    pub fn drain_stale(&mut self) -> Vec<String> {
        std::mem::take(&mut self.stale)
    }

    /// Текущая известная версия подсистемы.
    pub fn known_version(&self, name: &str) -> Option<&str> {
        self.known.get(name).map(|s| s.as_str())
    }

    /// Все известные подсистемы с версиями.
    pub fn all_entries(&self) -> Vec<SubsystemVersionEntry> {
        self.known
            .iter()
            .map(|(name, version)| SubsystemVersionEntry {
                name: name.clone(),
                version: version.clone(),
            })
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.known.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_versions(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_new_store_is_empty() {
        let s = SubsystemVersionStore::new();
        assert!(s.is_empty());
        assert!(s.stale_subsystems().is_empty());
    }

    #[test]
    fn test_init_no_stale() {
        let mut s = SubsystemVersionStore::new();
        let v = make_versions(&[("writing", "1.0"), ("mathematics", "1.0")]);
        s.init(&v);
        assert!(!s.is_empty());
        assert!(s.stale_subsystems().is_empty(), "init should not produce stale");
    }

    #[test]
    fn test_check_migration_same_version_no_change() {
        let mut s = SubsystemVersionStore::new();
        let v1 = make_versions(&[("writing", "1.0")]);
        s.init(&v1);
        let changed = s.check_migration(&v1);
        assert!(changed.is_empty());
        assert!(s.stale_subsystems().is_empty());
    }

    #[test]
    fn test_check_migration_detects_version_change() {
        let mut s = SubsystemVersionStore::new();
        s.init(&make_versions(&[("writing", "1.0")]));
        let v2 = make_versions(&[("writing", "1.1")]);
        let changed = s.check_migration(&v2);
        assert_eq!(changed, vec!["writing"]);
        assert_eq!(s.stale_subsystems(), &["writing"]);
        assert_eq!(s.known_version("writing"), Some("1.1"));
    }

    #[test]
    fn test_check_migration_detects_new_subsystem() {
        let mut s = SubsystemVersionStore::new();
        s.init(&make_versions(&[("writing", "1.0")]));
        let v2 = make_versions(&[("writing", "1.0"), ("music", "1.0")]);
        let changed = s.check_migration(&v2);
        assert!(changed.contains(&"music".to_string()), "new subsystem → changed");
    }

    #[test]
    fn test_drain_stale_clears_stale() {
        let mut s = SubsystemVersionStore::new();
        s.init(&make_versions(&[("writing", "1.0")]));
        s.check_migration(&make_versions(&[("writing", "1.1")]));
        let drained = s.drain_stale();
        assert_eq!(drained, vec!["writing"]);
        assert!(s.stale_subsystems().is_empty(), "stale should be empty after drain");
    }

    #[test]
    fn test_stale_not_duplicated() {
        let mut s = SubsystemVersionStore::new();
        s.init(&make_versions(&[("writing", "1.0")]));
        s.check_migration(&make_versions(&[("writing", "1.1")]));
        s.check_migration(&make_versions(&[("writing", "1.2")]));
        // "writing" должна быть в stale только один раз
        let stale = s.stale_subsystems();
        assert_eq!(stale.iter().filter(|n| n.as_str() == "writing").count(), 1);
    }

    #[test]
    fn test_all_entries_returns_known_versions() {
        let mut s = SubsystemVersionStore::new();
        s.init(&make_versions(&[("writing", "1.0"), ("mathematics", "2.0")]));
        let entries = s.all_entries();
        assert_eq!(entries.len(), 2);
        let math = entries.iter().find(|e| e.name == "mathematics").unwrap();
        assert_eq!(math.version, "2.0");
    }
}
