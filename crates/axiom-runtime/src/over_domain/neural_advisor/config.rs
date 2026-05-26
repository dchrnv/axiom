// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// G3: genome-per-advisor control.
// Парсит секцию `neural_advisor` из genome.yaml и применяет к NeuralAdvisorRegistry.
// Источник: docs/ROADMAP.md §Phase G / G3

use std::path::Path;

use crate::over_domain::neural_advisor::registry::NeuralAdvisorRegistry;

mod yaml_schema {
    use serde::Deserialize;

    #[derive(Deserialize, Default)]
    pub struct GenomeNeuralAdvisorWrapper {
        pub neural_advisor: Option<NeuralAdvisorSection>,
    }

    #[derive(Deserialize, Default)]
    pub struct NeuralAdvisorSection {
        pub depth: Option<SlotConfig>,
        pub octant: Option<SlotConfig>,
        pub conflict: Option<SlotConfig>,
        pub subsystem: Option<SlotConfig>,
        pub emergent: Option<SlotConfig>,
    }

    #[derive(Deserialize)]
    pub struct SlotConfig {
        pub enabled: bool,
    }
}

/// Конфигурация советников NeuralAdvisor из genome.yaml.
///
/// По умолчанию все слоты включены — отсутствие секции = все работают.
#[derive(Debug, Clone, PartialEq)]
pub struct NeuralAdvisorConfig {
    pub depth_enabled: bool,
    pub octant_enabled: bool,
    pub conflict_enabled: bool,
    pub subsystem_enabled: bool,
    pub emergent_enabled: bool,
}

impl Default for NeuralAdvisorConfig {
    fn default() -> Self {
        Self {
            depth_enabled: true,
            octant_enabled: true,
            conflict_enabled: true,
            subsystem_enabled: true,
            emergent_enabled: true,
        }
    }
}

impl NeuralAdvisorConfig {
    /// Загрузить из секции `neural_advisor` в genome.yaml.
    /// При ошибке или отсутствии секции → все слоты включены.
    pub fn from_genome_yaml(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| Self::from_yaml_str(&s))
            .unwrap_or_default()
    }

    /// Загрузить из YAML-строки (для тестов и встроенной конфигурации).
    pub fn from_yaml_str(s: &str) -> Option<Self> {
        let wrapper: yaml_schema::GenomeNeuralAdvisorWrapper =
            serde_yaml::from_str(s).ok()?;
        let section = wrapper.neural_advisor?;
        Some(Self {
            depth_enabled:     section.depth.map(|s| s.enabled).unwrap_or(true),
            octant_enabled:    section.octant.map(|s| s.enabled).unwrap_or(true),
            conflict_enabled:  section.conflict.map(|s| s.enabled).unwrap_or(true),
            subsystem_enabled: section.subsystem.map(|s| s.enabled).unwrap_or(true),
            emergent_enabled:  section.emergent.map(|s| s.enabled).unwrap_or(true),
        })
    }

    /// Применить конфигурацию: `enabled: false` → `None` в реестре.
    pub fn apply_to_registry(&self, registry: &mut NeuralAdvisorRegistry) {
        if !self.depth_enabled     { registry.depth     = None; }
        if !self.octant_enabled    { registry.octant    = None; }
        if !self.conflict_enabled  { registry.conflict  = None; }
        if !self.subsystem_enabled { registry.subsystem = None; }
        if !self.emergent_enabled  { registry.emergent  = None; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_yaml(depth: bool, octant: bool, conflict: bool, subsystem: bool, emergent: bool) -> String {
        format!(
            "neural_advisor:\n  depth:\n    enabled: {depth}\n  octant:\n    enabled: {octant}\n  conflict:\n    enabled: {conflict}\n  subsystem:\n    enabled: {subsystem}\n  emergent:\n    enabled: {emergent}\n"
        )
    }

    #[test]
    fn test_default_all_enabled() {
        let cfg = NeuralAdvisorConfig::default();
        assert!(cfg.depth_enabled);
        assert!(cfg.octant_enabled);
        assert!(cfg.conflict_enabled);
        assert!(cfg.subsystem_enabled);
        assert!(cfg.emergent_enabled);
    }

    #[test]
    fn test_missing_section_returns_none() {
        let yaml = "version: 1\nconfig:\n  foo: bar\n";
        assert!(NeuralAdvisorConfig::from_yaml_str(yaml).is_none());
    }

    #[test]
    fn test_from_genome_yaml_nonexistent_returns_default() {
        let cfg = NeuralAdvisorConfig::from_genome_yaml(Path::new("/nonexistent/genome.yaml"));
        assert_eq!(cfg, NeuralAdvisorConfig::default());
    }

    #[test]
    fn test_all_enabled_parsed() {
        let yaml = full_yaml(true, true, true, true, true);
        let cfg = NeuralAdvisorConfig::from_yaml_str(&yaml).unwrap();
        assert_eq!(cfg, NeuralAdvisorConfig::default());
    }

    #[test]
    fn test_emergent_disabled() {
        let yaml = full_yaml(true, true, true, true, false);
        let cfg = NeuralAdvisorConfig::from_yaml_str(&yaml).unwrap();
        assert!(cfg.depth_enabled);
        assert!(cfg.octant_enabled);
        assert!(cfg.conflict_enabled);
        assert!(cfg.subsystem_enabled);
        assert!(!cfg.emergent_enabled);
    }

    #[test]
    fn test_conflict_and_depth_disabled() {
        let yaml = full_yaml(false, true, false, true, true);
        let cfg = NeuralAdvisorConfig::from_yaml_str(&yaml).unwrap();
        assert!(!cfg.depth_enabled);
        assert!(cfg.octant_enabled);
        assert!(!cfg.conflict_enabled);
        assert!(cfg.subsystem_enabled);
        assert!(cfg.emergent_enabled);
    }

    #[test]
    fn test_missing_slot_defaults_to_enabled() {
        // Only emergent specified; others absent → default to true
        let yaml = "neural_advisor:\n  emergent:\n    enabled: false\n";
        let cfg = NeuralAdvisorConfig::from_yaml_str(yaml).unwrap();
        assert!(cfg.depth_enabled);
        assert!(cfg.octant_enabled);
        assert!(cfg.conflict_enabled);
        assert!(cfg.subsystem_enabled);
        assert!(!cfg.emergent_enabled);
    }

    #[test]
    fn test_apply_to_registry_disables_emergent() {
        let mut registry = NeuralAdvisorRegistry::default_v3();
        assert!(registry.emergent.is_some());

        let cfg = NeuralAdvisorConfig {
            emergent_enabled: false,
            ..NeuralAdvisorConfig::default()
        };
        cfg.apply_to_registry(&mut registry);

        assert!(registry.emergent.is_none());
        assert!(registry.depth.is_some());
        assert!(registry.octant.is_some());
        assert!(registry.conflict.is_some());
        assert!(registry.subsystem.is_some());
    }

    #[test]
    fn test_apply_to_registry_all_disabled() {
        let mut registry = NeuralAdvisorRegistry::default_v3();
        let cfg = NeuralAdvisorConfig {
            depth_enabled: false,
            octant_enabled: false,
            conflict_enabled: false,
            subsystem_enabled: false,
            emergent_enabled: false,
        };
        cfg.apply_to_registry(&mut registry);
        assert!(registry.depth.is_none());
        assert!(registry.octant.is_none());
        assert!(registry.conflict.is_none());
        assert!(registry.subsystem.is_none());
        assert!(registry.emergent.is_none());
    }

    #[test]
    fn test_apply_to_registry_all_enabled_leaves_intact() {
        let mut registry = NeuralAdvisorRegistry::default_v3();
        NeuralAdvisorConfig::default().apply_to_registry(&mut registry);
        assert!(registry.depth.is_some());
        assert!(registry.octant.is_some());
        assert!(registry.conflict.is_some());
        assert!(registry.subsystem.is_some());
        assert!(registry.emergent.is_some());
    }
}
