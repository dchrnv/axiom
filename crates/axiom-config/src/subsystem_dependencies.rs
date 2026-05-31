// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SubsystemDependencies — загрузчик статического графа зависимостей подсистем.
//
// Источник: docs/architecture/ContextRecognizer_Roadmap_V6_V9.md §2.7 (Variant C+)
// Файл:    config/subsystem_dependencies.yaml

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::loader::ConfigError;

/// Запись для одной подсистемы в графе зависимостей.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsystemDep {
    /// Архитектурные зависимости: B builds_on A → A инициализируется раньше B.
    #[serde(default)]
    pub builds_on: Vec<String>,
    /// Ожидаемые продуктивные напряжения: коактивация порождает параллельные гипотезы.
    /// НЕ является блоком — сигнал для DilemmaDetector.
    #[serde(default)]
    pub natural_tensions: Vec<NaturalTension>,
}

/// Описание одного ожидаемого напряжения между подсистемами.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalTension {
    /// Имя подсистемы на другом конце напряжения.
    pub target: String,
    /// Человекочитаемое объяснение природы напряжения.
    #[serde(default)]
    pub reason: String,
}

/// Статический граф зависимостей когнитивных подсистем.
///
/// Загружается из `config/subsystem_dependencies.yaml`.
/// Используется ContextRecognizer и DilemmaDetector.
#[derive(Debug, Clone, Default)]
pub struct SubsystemDependencies {
    /// Карта: имя подсистемы → её зависимости и напряжения.
    pub subsystems: HashMap<String, SubsystemDep>,
}

impl SubsystemDependencies {
    /// Загрузить из `<config_dir>/subsystem_dependencies.yaml`.
    ///
    /// Возвращает `empty()` если файл не существует (graceful degradation).
    pub fn load_or_empty(config_dir: &Path) -> Self {
        let path = config_dir.join("subsystem_dependencies.yaml");
        if !path.exists() {
            return Self::default();
        }
        match Self::load(&path) {
            Ok(deps) => deps,
            Err(e) => {
                eprintln!("[subsystem_dependencies] load failed: {e}, using empty");
                Self::default()
            }
        }
    }

    /// Загрузить из явного пути.
    ///
    /// Поддерживает два формата YAML:
    /// - Плоский: `writing: { ... }` (используется в тестах)
    /// - С оберткой: `subsystems: { writing: { ... } }` (production-файл)
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(ConfigError::IoError)?;

        // Попробовать формат с оберткой `subsystems:`
        #[derive(serde::Deserialize)]
        struct Wrapped {
            subsystems: HashMap<String, SubsystemDep>,
        }
        if let Ok(w) = serde_yaml::from_str::<Wrapped>(&content) {
            return Ok(Self { subsystems: w.subsystems });
        }

        // Fallback: плоский формат (имена подсистем на верхнем уровне)
        let raw: HashMap<String, SubsystemDep> = serde_yaml::from_str(&content)
            .map_err(ConfigError::ParseError)?;
        Ok(Self { subsystems: raw })
    }

    /// Получить зависимости конкретной подсистемы.
    pub fn get(&self, name: &str) -> Option<&SubsystemDep> {
        self.subsystems.get(name)
    }

    /// Проверить: является ли пара подсистем ожидаемым напряжением.
    ///
    /// Симметрично: (a, b) == (b, a).
    pub fn is_natural_tension(&self, a: &str, b: &str) -> bool {
        self.subsystems
            .get(a)
            .map(|dep| dep.natural_tensions.iter().any(|t| t.target == b))
            .unwrap_or(false)
            || self
                .subsystems
                .get(b)
                .map(|dep| dep.natural_tensions.iter().any(|t| t.target == a))
                .unwrap_or(false)
    }

    /// Топологически отсортированный порядок загрузки по `builds_on`.
    ///
    /// Подсистемы без зависимостей идут первыми.
    /// Возвращает ошибку при обнаружении цикла.
    pub fn load_order(&self) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut visited = HashMap::new();
        for name in self.subsystems.keys() {
            self.visit(name, &mut visited, &mut result)?;
        }
        Ok(result)
    }

    fn visit(
        &self,
        name: &str,
        visited: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<(), String> {
        match visited.get(name) {
            Some(true) => return Ok(()),
            Some(false) => return Err(format!("cycle detected involving '{name}'")),
            None => {}
        }
        visited.insert(name.to_string(), false);
        if let Some(dep) = self.subsystems.get(name) {
            for dep_name in &dep.builds_on {
                self.visit(dep_name, visited, result)?;
            }
        }
        visited.insert(name.to_string(), true);
        result.push(name.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_yaml(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    const MINIMAL_YAML: &str = r#"
writing:
  builds_on: []
  natural_tensions:
    - target: mathematics
      reason: "форма vs структура"

mathematics:
  builds_on: ["writing"]
  natural_tensions:
    - target: writing
      reason: "структура vs форма"
    - target: abstractions
      reason: "конкретная система vs мета-уровень"

abstractions:
  builds_on: []
  natural_tensions: []

dilemmas:
  builds_on: ["values", "morality"]
  natural_tensions: []
"#;

    #[test]
    fn test_load_parses_builds_on() {
        let f = write_yaml(MINIMAL_YAML);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        assert!(deps.get("mathematics").unwrap().builds_on.contains(&"writing".to_string()));
        assert!(deps.get("writing").unwrap().builds_on.is_empty());
    }

    #[test]
    fn test_load_parses_natural_tensions() {
        let f = write_yaml(MINIMAL_YAML);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        let tensions = &deps.get("writing").unwrap().natural_tensions;
        assert_eq!(tensions.len(), 1);
        assert_eq!(tensions[0].target, "mathematics");
    }

    #[test]
    fn test_is_natural_tension_symmetric() {
        let f = write_yaml(MINIMAL_YAML);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        assert!(deps.is_natural_tension("writing", "mathematics"));
        assert!(deps.is_natural_tension("mathematics", "writing"));
        assert!(!deps.is_natural_tension("writing", "abstractions"));
    }

    #[test]
    fn test_load_order_respects_builds_on() {
        let f = write_yaml(MINIMAL_YAML);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        let order = deps.load_order().unwrap();
        let writing_pos = order.iter().position(|s| s == "writing").unwrap();
        let math_pos = order.iter().position(|s| s == "mathematics").unwrap();
        assert!(writing_pos < math_pos, "writing must come before mathematics");
    }

    #[test]
    fn test_load_order_detects_cycle() {
        let cyclic = r#"
a:
  builds_on: ["b"]
b:
  builds_on: ["a"]
"#;
        let f = write_yaml(cyclic);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        assert!(deps.load_order().is_err());
    }

    #[test]
    fn test_missing_file_returns_empty() {
        let deps = SubsystemDependencies::load_or_empty(Path::new("/nonexistent/path"));
        assert!(deps.subsystems.is_empty());
    }

    #[test]
    fn test_dilemmas_builds_on_values_and_morality() {
        let f = write_yaml(MINIMAL_YAML);
        let deps = SubsystemDependencies::load(f.path()).unwrap();
        let d = deps.get("dilemmas").unwrap();
        assert!(d.builds_on.contains(&"values".to_string()));
        assert!(d.builds_on.contains(&"morality".to_string()));
        assert!(d.natural_tensions.is_empty());
    }
}
