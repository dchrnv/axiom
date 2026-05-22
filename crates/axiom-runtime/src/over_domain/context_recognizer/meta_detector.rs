// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MetaDetector — сопоставление ActivityDynamics с мета-примитивами (CR-V6 Фаза C).
//
// Источник: ContextRecognizer_Roadmap_V6_V9.md §1.3–1.4

use std::path::Path;

use axiom_experience::{MetaStore, MetaSubsystemId, SubsystemId};
use serde::Deserialize;

use super::activity_trace::{ActivityDynamics, ActivitySignature};

/// Один паттерн активации, тригерирующий мета-режим.
///
/// Все условия паттерна должны выполниться одновременно.
/// Пустые списки = любая подсистема.
/// `activity_signature = None` = любая сигнатура.
#[derive(Debug, Clone, Deserialize)]
pub struct SubsystemActivationPattern {
    /// Какие подсистемы должны быть активны (по имени: "mathematics", "writing", …).
    #[serde(default)]
    pub required_subsystems: Vec<String>,
    /// Какие подсистемы НЕ должны быть активны.
    #[serde(default)]
    pub forbidden_subsystems: Vec<String>,
    /// Требуемая сигнатура активности ("Steady", "Converging", …). None — любая.
    pub activity_signature: Option<String>,
}

impl SubsystemActivationPattern {
    /// Проверить совпадение паттерна с текущим контекстом.
    fn matches(&self, dominant: SubsystemId, signatures: &[ActivitySignature]) -> bool {
        let dom_name = dominant.name();

        if !self.required_subsystems.is_empty()
            && !self.required_subsystems.iter().any(|r| r == dom_name)
        {
            return false;
        }

        if self.forbidden_subsystems.iter().any(|f| f == dom_name) {
            return false;
        }

        if let Some(req_sig) = &self.activity_signature {
            if !signatures.iter().any(|s| s.name() == req_sig.as_str()) {
                return false;
            }
        }

        true
    }
}

/// Один мета-примитив из meta_primitives.yaml.
#[derive(Debug, Clone, Deserialize)]
pub struct MetaPrimitive {
    /// Уникальный строковый идентификатор ("meta_analysis", …).
    pub id: String,
    /// Числовой идентификатор мета-подсистемы (0x1001..0x1007).
    pub meta_id: u16,
    /// Паттерны активации. Хотя бы один должен совпасть.
    pub triggered_by: Vec<SubsystemActivationPattern>,
}

impl MetaPrimitive {
    /// Вычислить confidence совпадения (доля совпавших паттернов).
    ///
    /// Возвращает 0.0 если ни один не совпал.
    pub fn match_confidence(
        &self,
        dominant: SubsystemId,
        signatures: &[ActivitySignature],
    ) -> f32 {
        if self.triggered_by.is_empty() {
            return 0.0;
        }
        let matched = self
            .triggered_by
            .iter()
            .filter(|p| p.matches(dominant, signatures))
            .count();
        matched as f32 / self.triggered_by.len() as f32
    }
}

/// Детектор мета-подсистем.
///
/// Загружает список `MetaPrimitive` (из YAML или hardcoded),
/// на каждом `detect()` обновляет `MetaStore`.
#[derive(Debug)]
pub struct MetaDetector {
    primitives: Vec<MetaPrimitive>,
}

impl Default for MetaDetector {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl MetaDetector {
    pub fn new(primitives: Vec<MetaPrimitive>) -> Self {
        Self { primitives }
    }

    /// Загрузить мета-примитивы из YAML-файла.
    pub fn from_yaml(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let primitives: Vec<MetaPrimitive> = serde_yaml::from_str(&content)?;
        Ok(Self::new(primitives))
    }

    /// Загрузить из YAML или вернуть пустой детектор при ошибке.
    pub fn from_yaml_or_default(path: &Path) -> Self {
        Self::from_yaml(path).unwrap_or_default()
    }

    /// Обновить MetaStore: сопоставить ActivityDynamics + dominant с каждым примитивом.
    ///
    /// Только примитивы с confidence > 0 обновляют хранилище.
    pub fn detect(
        &self,
        _dynamics: &ActivityDynamics,
        signatures: &[ActivitySignature],
        dominant: SubsystemId,
        event_id: u64,
        store: &mut MetaStore,
    ) {
        for primitive in &self.primitives {
            let confidence = primitive.match_confidence(dominant, signatures);
            if confidence > 0.0 {
                store.activate(MetaSubsystemId(primitive.meta_id), confidence, event_id);
            }
        }
    }

    /// Число загруженных примитивов.
    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    pub fn is_empty(&self) -> bool {
        self.primitives.is_empty()
    }
}

// ── Тесты ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::{MetaStore, META_ANALYSIS, META_REFLECTION, META_SYNTHESIS};

    fn make_steady_pattern(required: &[&str]) -> SubsystemActivationPattern {
        SubsystemActivationPattern {
            required_subsystems: required.iter().map(|s| s.to_string()).collect(),
            forbidden_subsystems: vec![],
            activity_signature: Some("Steady".to_string()),
        }
    }

    fn make_converging_pattern() -> SubsystemActivationPattern {
        SubsystemActivationPattern {
            required_subsystems: vec![],
            forbidden_subsystems: vec![],
            activity_signature: Some("Converging".to_string()),
        }
    }

    fn dummy_dynamics() -> ActivityDynamics {
        ActivityDynamics {
            entropy_gradient: 0.0,
            oscillation_score: 0.0,
            cascade_score: 0.0,
            dominant_persistence: 0.8,
            fill_count: 16,
        }
    }

    #[test]
    fn test_empty_detector_no_activations() {
        let detector = MetaDetector::default();
        let mut store = MetaStore::new();
        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Mathematics,
            1,
            &mut store,
        );
        assert!(store.is_empty());
    }

    #[test]
    fn test_detect_steady_math_activates_analysis() {
        let primitive = MetaPrimitive {
            id: "meta_analysis".to_string(),
            meta_id: META_ANALYSIS.0,
            triggered_by: vec![make_steady_pattern(&["mathematics"])],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Mathematics,
            100,
            &mut store,
        );
        let act = store.get(META_ANALYSIS).unwrap();
        assert!((act.confidence - 1.0).abs() < 1e-6);
        assert_eq!(act.last_matched_event, 100);
    }

    #[test]
    fn test_detect_wrong_subsystem_no_activation() {
        let primitive = MetaPrimitive {
            id: "meta_analysis".to_string(),
            meta_id: META_ANALYSIS.0,
            triggered_by: vec![make_steady_pattern(&["mathematics"])],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Writing,
            1,
            &mut store,
        );
        assert!(store.is_empty());
    }

    #[test]
    fn test_detect_wrong_signature_no_activation() {
        let primitive = MetaPrimitive {
            id: "meta_analysis".to_string(),
            meta_id: META_ANALYSIS.0,
            triggered_by: vec![make_steady_pattern(&["mathematics"])],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        // Oscillating вместо требуемого Steady
        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Oscillating],
            SubsystemId::Mathematics,
            1,
            &mut store,
        );
        assert!(store.is_empty());
    }

    #[test]
    fn test_detect_converging_activates_synthesis() {
        let primitive = MetaPrimitive {
            id: "meta_synthesis".to_string(),
            meta_id: META_SYNTHESIS.0,
            triggered_by: vec![make_converging_pattern()],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Converging],
            SubsystemId::Unknown,
            200,
            &mut store,
        );
        assert!(store.get(META_SYNTHESIS).is_some());
    }

    #[test]
    fn test_detect_forbidden_subsystem_blocks() {
        let pattern = SubsystemActivationPattern {
            required_subsystems: vec![],
            forbidden_subsystems: vec!["mathematics".to_string()],
            activity_signature: Some("Steady".to_string()),
        };
        let primitive = MetaPrimitive {
            id: "meta_reflection".to_string(),
            meta_id: META_REFLECTION.0,
            triggered_by: vec![pattern],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Mathematics,
            1,
            &mut store,
        );
        assert!(store.is_empty());

        // Writing — не запрещён → активируется
        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Writing,
            2,
            &mut store,
        );
        assert!(store.get(META_REFLECTION).is_some());
    }

    #[test]
    fn test_multiple_primitives_independent() {
        let p1 = MetaPrimitive {
            id: "meta_analysis".to_string(),
            meta_id: META_ANALYSIS.0,
            triggered_by: vec![make_steady_pattern(&["mathematics"])],
        };
        let p2 = MetaPrimitive {
            id: "meta_synthesis".to_string(),
            meta_id: META_SYNTHESIS.0,
            triggered_by: vec![make_converging_pattern()],
        };
        let detector = MetaDetector::new(vec![p1, p2]);
        let mut store = MetaStore::new();

        // Оба паттерна совпадают
        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady, ActivitySignature::Converging],
            SubsystemId::Mathematics,
            300,
            &mut store,
        );
        assert!(store.get(META_ANALYSIS).is_some());
        assert!(store.get(META_SYNTHESIS).is_some());
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_partial_confidence_multiple_patterns() {
        let primitive = MetaPrimitive {
            id: "meta_analysis".to_string(),
            meta_id: META_ANALYSIS.0,
            triggered_by: vec![
                make_steady_pattern(&["mathematics"]),   // совпадёт
                make_converging_pattern(),                // не совпадёт (нет Converging)
            ],
        };
        let detector = MetaDetector::new(vec![primitive]);
        let mut store = MetaStore::new();

        detector.detect(
            &dummy_dynamics(),
            &[ActivitySignature::Steady],
            SubsystemId::Mathematics,
            1,
            &mut store,
        );
        // 1/2 паттернов = confidence 0.5
        let act = store.get(META_ANALYSIS).unwrap();
        assert!((act.confidence - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_no_required_subsystems_matches_any() {
        let pattern = SubsystemActivationPattern {
            required_subsystems: vec![],
            forbidden_subsystems: vec![],
            activity_signature: None,
        };
        let primitive = MetaPrimitive {
            id: "meta_perception".to_string(),
            meta_id: 0x1004,
            triggered_by: vec![pattern],
        };
        let detector = MetaDetector::new(vec![primitive]);

        for dom in [SubsystemId::Writing, SubsystemId::Mathematics, SubsystemId::Music] {
            let mut store = MetaStore::new();
            detector.detect(&dummy_dynamics(), &[], dom, 1, &mut store);
            assert_eq!(store.len(), 1);
        }
    }

    #[test]
    fn test_from_yaml_load() {
        let yaml = r#"
- id: "meta_analysis"
  meta_id: 4097
  triggered_by:
    - required_subsystems: ["mathematics"]
      activity_signature: "Steady"
- id: "meta_synthesis"
  meta_id: 4098
  triggered_by:
    - activity_signature: "Converging"
"#;
        let primitives: Vec<MetaPrimitive> = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(primitives.len(), 2);
        assert_eq!(primitives[0].id, "meta_analysis");
        assert_eq!(primitives[0].meta_id, 0x1001);
        assert_eq!(primitives[1].id, "meta_synthesis");
    }
}
