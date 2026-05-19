// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ArbiterLog — кольцевой буфер решений OverDomainArbiter.
// Источник: docs/architecture/OverDomainArbiter_V1_0.md §9

use std::collections::VecDeque;

use super::source::{AdvisoryId, AdvisoryType, SourceId};

const LOG_CAPACITY: usize = 500;

/// Исход решения Arbiter по рекомендации.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArbiterOutcome {
    /// Применено автономно (AutoApply).
    Applied,
    /// Помещено в очередь Workstation (RequireConfirmation).
    Queued,
    /// Пропущено — ниже порога confidence или режим Ignore.
    Skipped,
    /// Подтверждено chrnv из очереди.
    Confirmed,
    /// Отклонено chrnv из очереди.
    Rejected,
}

/// Запись лога одного решения.
#[derive(Debug, Clone)]
pub struct ArbiterLogEntry {
    pub event_id: u64,
    pub advisory_id: AdvisoryId,
    pub source: SourceId,
    pub advisory_type: AdvisoryType,
    pub subject_id: u32,
    pub confidence: f32,
    pub outcome: ArbiterOutcome,
}

/// Кольцевой буфер последних 500 решений.
/// Не персистируется в V1.
#[derive(Debug, Default)]
pub struct ArbiterLog {
    entries: VecDeque<ArbiterLogEntry>,
}

impl ArbiterLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: ArbiterLogEntry) {
        if self.entries.len() >= LOG_CAPACITY {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ArbiterLogEntry> {
        self.entries.iter()
    }

    /// Число Applied для пары (source, type) — числитель качества советника.
    pub fn count_outcome(
        &self,
        source: SourceId,
        advisory_type: AdvisoryType,
        outcome: ArbiterOutcome,
    ) -> usize {
        self.entries
            .iter()
            .filter(|e| e.source == source && e.advisory_type == advisory_type && e.outcome == outcome)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(outcome: ArbiterOutcome) -> ArbiterLogEntry {
        ArbiterLogEntry {
            event_id: 1,
            advisory_id: 0,
            source: 0,
            advisory_type: AdvisoryType::DepthHint,
            subject_id: 1,
            confidence: 0.8,
            outcome,
        }
    }

    #[test]
    fn test_log_capacity_capped() {
        let mut log = ArbiterLog::new();
        for _ in 0..600 {
            log.push(entry(ArbiterOutcome::Applied));
        }
        assert_eq!(log.len(), 500);
    }

    #[test]
    fn test_count_outcome() {
        let mut log = ArbiterLog::new();
        log.push(entry(ArbiterOutcome::Applied));
        log.push(entry(ArbiterOutcome::Applied));
        log.push(entry(ArbiterOutcome::Skipped));
        assert_eq!(log.count_outcome(0, AdvisoryType::DepthHint, ArbiterOutcome::Applied), 2);
        assert_eq!(log.count_outcome(0, AdvisoryType::DepthHint, ArbiterOutcome::Skipped), 1);
    }
}
