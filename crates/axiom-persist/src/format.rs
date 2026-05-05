// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Сериализуемые структуры данных хранилища.
//
// Отдельный слой между внутренними типами ядра и форматом на диске.
// Позволяет эволюционировать формат независимо от структур ядра.

use axiom_arbiter::{ExperienceTrace, TensionTrace};
use axiom_config::DomainConfig;
use axiom_core::{Connection, Token};
use serde::{Deserialize, Serialize};

/// Состояние одного домена на диске.
///
/// `config` опционален для обратной совместимости: файлы записанные до
/// Memory Persistence V1.1 не содержат это поле → десериализуется как `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredDomain {
    pub domain_id: u32,
    pub tokens: Vec<Token>,
    pub connections: Vec<Connection>,
    /// DomainConfig домена на момент сохранения (None в старых файлах)
    #[serde(default)]
    pub config: Option<DomainConfig>,
}

/// Experience trace на диске.
/// Зеркало ExperienceTrace с weight уже применённым при загрузке.
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredTrace {
    pub pattern: Token,
    pub weight: f32,
    pub created_at: u64,
    pub last_used: u64,
    pub success_count: u32,
    pub pattern_hash: u64,
}

impl From<&ExperienceTrace> for StoredTrace {
    fn from(t: &ExperienceTrace) -> Self {
        Self {
            pattern: t.pattern,
            weight: t.weight,
            created_at: t.created_at,
            last_used: t.last_used,
            success_count: t.success_count,
            pattern_hash: t.pattern_hash,
        }
    }
}

impl From<StoredTrace> for ExperienceTrace {
    fn from(s: StoredTrace) -> Self {
        Self {
            pattern: s.pattern,
            weight: s.weight,
            created_at: s.created_at,
            last_used: s.last_used,
            success_count: s.success_count,
            pattern_hash: s.pattern_hash,
        }
    }
}

/// Tension trace на диске.
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredTensionTrace {
    pub pattern: Token,
    pub temperature: u8,
    pub created_at: u64,
}

impl From<&TensionTrace> for StoredTensionTrace {
    fn from(t: &TensionTrace) -> Self {
        Self {
            pattern: t.pattern,
            temperature: t.temperature,
            created_at: t.created_at,
        }
    }
}

impl From<StoredTensionTrace> for TensionTrace {
    fn from(s: StoredTensionTrace) -> Self {
        Self {
            pattern: s.pattern,
            temperature: s.temperature,
            created_at: s.created_at,
        }
    }
}

/// Полное состояние Engine на диске.
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredEngineState {
    pub tick_count: u64,
    pub com_next_id: u64,
    pub domains: Vec<StoredDomain>,
    pub traces: Vec<StoredTrace>,
    pub tension: Vec<StoredTensionTrace>,
}
