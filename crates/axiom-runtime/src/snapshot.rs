// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Snapshot — сохранение и восстановление состояния Engine

use std::collections::HashMap;
use axiom_core::{Token, Connection};
use axiom_config::DomainConfig;

/// Слепок состояния одного домена
#[derive(Debug, Clone)]
pub struct DomainSnapshot {
    /// ID домена
    pub domain_id: u32,
    /// Конфигурация домена
    pub config: DomainConfig,
    /// Токены домена
    pub tokens: Vec<Token>,
    /// Связи домена
    pub connections: Vec<Connection>,
}

/// Полный слепок состояния Engine.
///
/// Frontier **не** включается — он восстанавливается из последних событий
/// (Causal Frontier V1, §14).
#[derive(Debug, Clone)]
pub struct EngineSnapshot {
    /// Состояния всех доменов
    pub domains: Vec<DomainSnapshot>,
    /// Следующий event_id глобального COM счётчика
    pub com_next_id: u64,
    /// Event_id на момент создания snapshot
    pub created_at: u64,
}

impl EngineSnapshot {
    /// Создать пустой snapshot (для тестов и инициализации)
    pub fn empty() -> Self {
        Self {
            domains: Vec::new(),
            com_next_id: 1,
            created_at: 0,
        }
    }

    /// Число доменов в snapshot
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Суммарное число токенов по всем доменам
    pub fn total_token_count(&self) -> usize {
        self.domains.iter().map(|d| d.tokens.len()).sum()
    }

    /// Найти snapshot домена по ID
    pub fn find_domain(&self, domain_id: u32) -> Option<&DomainSnapshot> {
        self.domains.iter().find(|d| d.domain_id == domain_id)
    }

    /// Получить конфигурации доменов для восстановления
    pub fn domain_configs(&self) -> HashMap<u32, DomainConfig> {
        self.domains.iter()
            .map(|d| (d.domain_id, d.config))
            .collect()
    }
}
