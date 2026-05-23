// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Определение применимых уровней оценки для Frame.
// V1: прямое соответствие Shell L1..L8 ↔ EvaluationLevel 1..8.
// V2: primary_subsystem из ContextRecognizer влияет на выбор уровня.
// V3: AxialEvaluatorConfig — override таблица subsystem→level из genome.yaml.
//
// Shell Frame — вычисляется из link_type его связей: (link_type & 0x00F0) >> 4 = слой 0..7.
// Синтаксические связи: link_type >> 8 == 0x08.

use std::collections::HashMap;

use axiom_core::Connection;
use axiom_experience::{EvaluationLevel, SubsystemId};

/// V3: конфигурация AxialEvaluator из genome.yaml.
///
/// Позволяет переопределить встроенную таблицу SubsystemId → EvaluationLevel.
/// Встроенный дефолт используется как fallback для неупомянутых подсистем.
#[derive(Debug, Default, Clone)]
pub struct AxialEvaluatorConfig {
    pub subsystem_level_overrides: HashMap<SubsystemId, EvaluationLevel>,
}

impl AxialEvaluatorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_override(mut self, subsystem: SubsystemId, level: EvaluationLevel) -> Self {
        self.subsystem_level_overrides.insert(subsystem, level);
        self
    }
}

const SHELL_WEIGHT_THRESHOLD: u8 = 1;

/// Построить Shell-профиль [8] из связей Frame-анкера в домене.
///
/// Для каждой синтаксической связи (link_type >> 8 == 0x08) от anchor_id
/// инкрементирует счётчик соответствующего слоя.
pub fn build_shell_from_connections(anchor_id: u32, connections: &[Connection]) -> [u8; 8] {
    let mut profile = [0u8; 8];
    for conn in connections {
        if conn.source_id != anchor_id {
            continue;
        }
        if (conn.link_type >> 8) != 0x08 {
            continue;
        }
        let layer = ((conn.link_type & 0x00F0) >> 4) as usize;
        if layer < 8 {
            profile[layer] = profile[layer].saturating_add(1);
        }
    }
    profile
}

/// V2: Определить уровень по доминирующей подсистеме ContextRecognizer.
///
/// SubsystemId → EvaluationLevel маппинг по смысловой аффинности.
/// `None` или `Unknown` → Shell-fallback.
pub fn subsystem_to_level(subsystem: SubsystemId) -> Option<EvaluationLevel> {
    match subsystem {
        SubsystemId::Mathematics => Some(EvaluationLevel::Conceptual),
        SubsystemId::Logic => Some(EvaluationLevel::Conceptual),
        SubsystemId::Values => Some(EvaluationLevel::Conceptual),
        SubsystemId::Writing => Some(EvaluationLevel::Imaginal),
        SubsystemId::Music => Some(EvaluationLevel::Imaginal),
        SubsystemId::Time => Some(EvaluationLevel::Action),
        SubsystemId::Unknown => None,
    }
}

/// V3: то же с учётом AxialEvaluatorConfig.overrides — override имеет приоритет над встроенным.
pub fn subsystem_to_level_with_config(
    subsystem: SubsystemId,
    config: Option<&AxialEvaluatorConfig>,
) -> Option<EvaluationLevel> {
    if let Some(cfg) = config {
        if let Some(&level) = cfg.subsystem_level_overrides.get(&subsystem) {
            return Some(level);
        }
    }
    subsystem_to_level(subsystem)
}

/// V2: Определить применимые уровни с учётом подсистемы.
///
/// Если `primary_subsystem` известна и маппируется — возвращает [mapped_level].
/// Иначе — fallback на Shell-профиль.
pub fn determine_applicable_levels_with_subsystem(
    shell_profile: &[u8; 8],
    primary_subsystem: Option<SubsystemId>,
) -> Vec<EvaluationLevel> {
    if let Some(subsystem) = primary_subsystem {
        if let Some(level) = subsystem_to_level(subsystem) {
            return vec![level];
        }
    }
    determine_applicable_levels(shell_profile)
}

/// V3: то же с AxialEvaluatorConfig.
pub fn determine_applicable_levels_with_config(
    shell_profile: &[u8; 8],
    primary_subsystem: Option<SubsystemId>,
    config: Option<&AxialEvaluatorConfig>,
) -> Vec<EvaluationLevel> {
    if let Some(subsystem) = primary_subsystem {
        if let Some(level) = subsystem_to_level_with_config(subsystem, config) {
            return vec![level];
        }
    }
    determine_applicable_levels(shell_profile)
}

/// V1: Определить применимые уровни оценки из Shell-профиля.
///
/// Возвращает уровни где счётчик превышает порог.
/// Fallback: [Conceptual] если ни один слой не активен.
pub fn determine_applicable_levels(shell_profile: &[u8; 8]) -> Vec<EvaluationLevel> {
    const LEVELS: [EvaluationLevel; 8] = [
        EvaluationLevel::Sensory,
        EvaluationLevel::Action,
        EvaluationLevel::Imaginal,
        EvaluationLevel::Conceptual,
        EvaluationLevel::Motivational,
        EvaluationLevel::Social,
        EvaluationLevel::Existential,
        EvaluationLevel::Transcendent,
    ];

    let applicable: Vec<EvaluationLevel> = LEVELS
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, _)| shell_profile[*i] >= SHELL_WEIGHT_THRESHOLD)
        .map(|(_, l)| l)
        .collect();

    if applicable.is_empty() {
        vec![EvaluationLevel::Conceptual]
    } else {
        applicable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn syntactic_conn(source: u32, target: u32, layer: u8) -> Connection {
        let mut conn = Connection::new(source, target, 109, 1);
        conn.link_type = 0x0800 | ((layer as u16) << 4);
        conn
    }

    #[test]
    fn test_empty_shell_gives_conceptual() {
        let levels = determine_applicable_levels(&[0u8; 8]);
        assert_eq!(levels, vec![EvaluationLevel::Conceptual]);
    }

    #[test]
    fn test_layer_0_gives_sensory() {
        let mut shell = [0u8; 8];
        shell[0] = 1;
        let levels = determine_applicable_levels(&shell);
        assert!(levels.contains(&EvaluationLevel::Sensory));
    }

    #[test]
    fn test_build_shell_from_connections_empty() {
        let profile = build_shell_from_connections(1, &[]);
        assert_eq!(profile, [0u8; 8]);
    }

    #[test]
    fn test_build_shell_from_connections_layer3() {
        let conns = vec![syntactic_conn(1, 2, 3)]; // layer 3 = Conceptual
        let profile = build_shell_from_connections(1, &conns);
        assert_eq!(profile[3], 1);
        let levels = determine_applicable_levels(&profile);
        assert!(levels.contains(&EvaluationLevel::Conceptual));
    }

    #[test]
    fn test_config_override_takes_priority() {
        let cfg = AxialEvaluatorConfig::new()
            .with_override(SubsystemId::Music, EvaluationLevel::Conceptual);
        let level = subsystem_to_level_with_config(SubsystemId::Music, Some(&cfg));
        assert_eq!(level, Some(EvaluationLevel::Conceptual));
    }

    #[test]
    fn test_config_fallback_without_override() {
        let cfg = AxialEvaluatorConfig::new();
        let level = subsystem_to_level_with_config(SubsystemId::Music, Some(&cfg));
        // No override → default: Music → Imaginal
        assert_eq!(level, Some(EvaluationLevel::Imaginal));
    }

    #[test]
    fn test_determine_with_config_override() {
        let cfg = AxialEvaluatorConfig::new()
            .with_override(SubsystemId::Time, EvaluationLevel::Transcendent);
        let levels = determine_applicable_levels_with_config(
            &[0u8; 8],
            Some(SubsystemId::Time),
            Some(&cfg),
        );
        assert_eq!(levels, vec![EvaluationLevel::Transcendent]);
    }

    #[test]
    fn test_non_syntactic_connections_ignored() {
        let mut conn = Connection::new(1, 2, 109, 1);
        conn.link_type = 0x0100; // not 0x08XX
        let profile = build_shell_from_connections(1, &[conn]);
        assert_eq!(profile, [0u8; 8]);
    }

    #[test]
    fn test_other_anchor_connections_ignored() {
        let conns = vec![syntactic_conn(999, 2, 0)]; // different source
        let profile = build_shell_from_connections(1, &conns);
        assert_eq!(profile, [0u8; 8]);
    }
}
