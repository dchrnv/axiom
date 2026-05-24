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
    /// V2: истёк TTL в очереди (PENDING_TTL event_id).
    Expired,
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
/// Не персистируется в V1/V2 (ARB-TD-06).
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

    /// V2: Доля Confirmed среди (Confirmed + Rejected) за последние `window` записей
    /// для пары (source, advisory_type). None если нет данных.
    pub fn quality_window(
        &self,
        source: SourceId,
        advisory_type: AdvisoryType,
        window: usize,
    ) -> Option<f32> {
        let relevant: Vec<_> = self.entries.iter()
            .filter(|e| e.source == source && e.advisory_type == advisory_type)
            .filter(|e| matches!(e.outcome, ArbiterOutcome::Confirmed | ArbiterOutcome::Rejected))
            .rev()
            .take(window)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let confirmed = relevant.iter()
            .filter(|e| e.outcome == ArbiterOutcome::Confirmed)
            .count();
        Some(confirmed as f32 / relevant.len() as f32)
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

    #[test]
    fn test_quality_window_empty() {
        let log = ArbiterLog::new();
        assert!(log.quality_window(0, AdvisoryType::OctantCorrection, 20).is_none());
    }

    #[test]
    fn test_quality_window_all_confirmed() {
        let mut log = ArbiterLog::new();
        for _ in 0..5 {
            log.push(ArbiterLogEntry {
                event_id: 1, advisory_id: 0, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Confirmed,
            });
        }
        let q = log.quality_window(0, AdvisoryType::OctantCorrection, 20).unwrap();
        assert!((q - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_quality_window_half() {
        let mut log = ArbiterLog::new();
        for i in 0..4 {
            let outcome = if i % 2 == 0 { ArbiterOutcome::Confirmed } else { ArbiterOutcome::Rejected };
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8, outcome,
            });
        }
        let q = log.quality_window(0, AdvisoryType::OctantCorrection, 20).unwrap();
        assert!((q - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_quality_window_respects_window_size() {
        let mut log = ArbiterLog::new();
        // 10 Rejected
        for i in 0..10 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Rejected,
            });
        }
        // 5 Confirmed (most recent)
        for i in 10..15 {
            log.push(ArbiterLogEntry {
                event_id: i, advisory_id: i as u64, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8,
                outcome: ArbiterOutcome::Confirmed,
            });
        }
        // Window=5 → only the last 5 (all Confirmed) → quality=1.0
        let q = log.quality_window(0, AdvisoryType::OctantCorrection, 5).unwrap();
        assert!((q - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_quality_window_skips_other_outcomes() {
        let mut log = ArbiterLog::new();
        // Applied/Queued/Skipped не считаются
        for outcome in [ArbiterOutcome::Applied, ArbiterOutcome::Queued, ArbiterOutcome::Skipped] {
            log.push(ArbiterLogEntry {
                event_id: 1, advisory_id: 0, source: 0,
                advisory_type: AdvisoryType::OctantCorrection,
                subject_id: 1, confidence: 0.8, outcome,
            });
        }
        log.push(ArbiterLogEntry {
            event_id: 2, advisory_id: 1, source: 0,
            advisory_type: AdvisoryType::OctantCorrection,
            subject_id: 1, confidence: 0.8,
            outcome: ArbiterOutcome::Confirmed,
        });
        let q = log.quality_window(0, AdvisoryType::OctantCorrection, 20).unwrap();
        // Only 1 entry counted (Confirmed), quality = 1.0
        assert!((q - 1.0).abs() < 1e-5);
    }
}
