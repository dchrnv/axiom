// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Определение применимых уровней оценки для Frame.
// V1: прямое соответствие Shell L1..L8 ↔ EvaluationLevel 1..8.
// V2: primary_subsystem из ContextRecognizer влияет на выбор уровня.
//
// Shell Frame — вычисляется из link_type его связей: (link_type & 0x00F0) >> 4 = слой 0..7.
// Синтаксические связи: link_type >> 8 == 0x08.

use axiom_core::Connection;
use axiom_experience::{EvaluationLevel, SubsystemId};

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
        SubsystemId::Writing => Some(EvaluationLevel::Imaginal),
        SubsystemId::Music => Some(EvaluationLevel::Imaginal),
        SubsystemId::Time => Some(EvaluationLevel::Action),
        SubsystemId::Unknown => None,
    }
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
