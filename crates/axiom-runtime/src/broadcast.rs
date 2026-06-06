// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Адаптерные типы: детальные снапшоты для CLI/REST.
// BroadcastSnapshot удалён в SEN-TD-01 Фаза F — заменён SensoriumState.
// Доступны только при feature "adapters".

use axiom_core::STATE_LOCKED;
use serde::Serialize;

/// Сводка последнего завершённого dream-цикла.
/// Хранится в AxiomEngine.last_dream_summary, используется SensoriumState и reporting.
#[derive(Serialize, Clone)]
pub struct LastDreamSummary {
    pub cycle_id: u64,
    pub started_at_tick: u64,
    pub ended_at_tick: u64,
    pub proposals_accepted: u32,
    pub proposals_rejected: u32,
    pub sutra_written: u32,
    pub fatigue_before: u8,
    pub fatigue_after: u8,
}

/// Детальный snapshot одного домена — только по явному запросу.
///
/// Может быть большим (сотни токенов) — не для периодического broadcast.
/// Отправляется в ответ на `AdapterPayload::DomainSnapshot { domain_id }`.
#[derive(Serialize, Clone)]
pub struct DomainDetailSnapshot {
    /// domain_id запрошенного домена
    pub domain_id: u16,
    /// Все токены домена в компактном формате
    pub tokens: Vec<TokenSnapshot>,
    /// Все связи домена в компактном формате
    pub connections: Vec<ConnectionSnapshot>,
}

/// Компактное представление Token для JSON-передачи.
#[derive(Serialize, Clone)]
pub struct TokenSnapshot {
    pub sutra_id: u32,
    pub position: [i16; 3],
    pub shell: [u8; 8],
    pub mass: u8,
    pub temperature: u8,
    pub valence: i8,
    pub origin: u16,
    pub is_anchor: bool,
}

impl From<&axiom_core::Token> for TokenSnapshot {
    fn from(t: &axiom_core::Token) -> Self {
        Self::from_token_with_shell(t, axiom_shell::EMPTY_SHELL)
    }
}

impl TokenSnapshot {
    pub fn from_token_with_connections(
        t: &axiom_core::Token,
        connections: &[axiom_core::Connection],
        table: &axiom_shell::SemanticContributionTable,
    ) -> Self {
        let shell = axiom_shell::compute_shell(t.sutra_id, connections, table);
        Self::from_token_with_shell(t, shell)
    }

    fn from_token_with_shell(t: &axiom_core::Token, shell: axiom_shell::ShellProfile) -> Self {
        Self {
            sutra_id: t.sutra_id,
            position: t.position,
            shell,
            mass: t.mass,
            temperature: t.temperature,
            valence: t.valence,
            origin: t.origin,
            is_anchor: t.state == STATE_LOCKED,
        }
    }
}

/// Компактное представление Connection для JSON-передачи.
#[derive(Serialize, Clone)]
pub struct ConnectionSnapshot {
    pub source_id: u32,
    pub target_id: u32,
    pub weight: f32,
}

impl From<&axiom_core::Connection> for ConnectionSnapshot {
    fn from(c: &axiom_core::Connection) -> Self {
        Self {
            source_id: c.source_id,
            target_id: c.target_id,
            weight: c.strength,
        }
    }
}
