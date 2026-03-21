// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM (Causal Order Model) stub
// TODO: Replace with full implementation when COM tracking is migrated

/// COM - Causal Order Model для отслеживания причинного порядка (stub)
#[derive(Debug, Clone)]
pub struct COM {
    next_event_id: u64,
}

impl COM {
    /// Создать новый COM
    pub fn new() -> Self {
        Self {
            next_event_id: 1,
        }
    }

    /// Получить следующий event_id для домена
    pub fn next_event_id(&mut self, _domain_id: u16) -> u64 {
        let id = self.next_event_id;
        self.next_event_id += 1;
        id
    }
}

impl Default for COM {
    fn default() -> Self {
        Self::new()
    }
}
