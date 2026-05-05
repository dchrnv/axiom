// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM (Causal Order Model) - причинный порядок событий

use std::collections::HashMap;

/// COM - Causal Order Model для отслеживания причинного порядка.
///
/// Поддерживает глобальную монотонную нумерацию событий и подсчёт
/// событий на каждый домен для аналитики и диагностики.
#[derive(Debug, Clone)]
pub struct COM {
    /// Глобальный счётчик событий (обеспечивает причинный порядок)
    next_id: u64,
    /// Число событий, выданных каждому домену
    domain_event_counts: HashMap<u16, u64>,
}

impl COM {
    /// Создать новый COM
    pub fn new() -> Self {
        Self {
            next_id: 1,
            domain_event_counts: HashMap::new(),
        }
    }

    /// Получить следующий event_id для домена.
    ///
    /// ID монотонно возрастают глобально, что гарантирует причинный порядок.
    /// Каждый вызов регистрирует событие в счётчике домена.
    pub fn next_event_id(&mut self, domain_id: u16) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        *self.domain_event_counts.entry(domain_id).or_insert(0) += 1;
        id
    }

    /// Получить число событий, выданных конкретному домену
    pub fn domain_event_count(&self, domain_id: u16) -> u64 {
        self.domain_event_counts
            .get(&domain_id)
            .copied()
            .unwrap_or(0)
    }

    /// Получить текущее значение глобального счётчика (следующий ID)
    pub fn current_id(&self) -> u64 {
        self.next_id
    }
}

impl Default for COM {
    fn default() -> Self {
        Self::new()
    }
}
