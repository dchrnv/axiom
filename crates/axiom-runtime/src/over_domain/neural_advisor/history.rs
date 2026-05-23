// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AdvisoryHistory — ring-буфер последних результатов советников per sutra_id.
// Используется DepthHistoryBiasAdvisor (Фаза 2) и калибровкой CognitiveProfile (Фаза 4).

use std::collections::HashMap;
use std::collections::VecDeque;

use axiom_experience::{Octant, SubsystemId};

#[inline]
fn octant_from_idx(i: usize) -> Octant {
    match i {
        0 => Octant::CreativeAffirmation,
        1 => Octant::EcstaticAffirmation,
        2 => Octant::HeroicFatal,
        3 => Octant::DestructiveActivating,
        4 => Octant::IdealizedConsoling,
        5 => Octant::PassiveSentimental,
        6 => Octant::FormalDenying,
        _ => Octant::SelfDestructiveApathic,
    }
}

/// Исход обработки рекомендации (заполняется через on_feedback).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvisoryHistoryOutcome {
    /// Ещё не получен feedback от Arbiter.
    Pending,
    /// Применено автономно (AutoApply).
    Applied,
    /// Подтверждено chrnv.
    Confirmed,
    /// Отклонено chrnv.
    Rejected,
    /// Пропущено (ниже порога или Ignore).
    Skipped,
}

/// Одна запись в истории советов per Frame.
#[derive(Debug, Clone)]
pub struct AdvisoryHistoryEntry {
    pub computed_at_event: u64,
    pub octant_suggestion: Option<Octant>,
    pub octant_confidence: f32,
    pub subsystem_suggestion: Option<SubsystemId>,
    pub subsystem_confidence: f32,
    pub outcome: AdvisoryHistoryOutcome,
}

/// Ring-буфер истории советов для одного Frame.
pub struct AdvisoryRingBuffer {
    entries: VecDeque<AdvisoryHistoryEntry>,
    cap: usize,
}

impl AdvisoryRingBuffer {
    pub fn new(cap: usize) -> Self {
        Self { entries: VecDeque::with_capacity(cap), cap }
    }

    pub fn push(&mut self, entry: AdvisoryHistoryEntry) {
        if self.entries.len() == self.cap {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn iter(&self) -> impl Iterator<Item = &AdvisoryHistoryEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Доля принятых советов (Applied + Confirmed) среди тех, по которым есть ответ.
    pub fn acceptance_rate_octant(&self) -> f32 {
        let decided: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.octant_suggestion.is_some())
            .filter(|e| e.outcome != AdvisoryHistoryOutcome::Pending)
            .collect();
        if decided.is_empty() {
            return 0.0;
        }
        let accepted = decided
            .iter()
            .filter(|e| {
                e.outcome == AdvisoryHistoryOutcome::Applied
                    || e.outcome == AdvisoryHistoryOutcome::Confirmed
            })
            .count();
        accepted as f32 / decided.len() as f32
    }

    /// Октант, который чаще всего принимался (Applied/Confirmed) в истории.
    pub fn dominant_accepted_octant(&self) -> Option<Octant> {
        let mut counts = [0u32; 8];
        for entry in &self.entries {
            if let Some(oct) = entry.octant_suggestion {
                if entry.outcome == AdvisoryHistoryOutcome::Applied
                    || entry.outcome == AdvisoryHistoryOutcome::Confirmed
                {
                    counts[oct.index()] += 1;
                }
            }
        }
        let max = *counts.iter().max().unwrap_or(&0);
        if max == 0 {
            return None;
        }
        let idx = counts.iter().position(|&c| c == max)?;
        Some(octant_from_idx(idx))
    }

    /// Обновить исход последней записи со статусом Pending для данного event_id.
    /// Если точного совпадения по event_id нет — обновляет последнюю Pending запись.
    pub fn update_outcome(&mut self, event_id: u64, outcome: AdvisoryHistoryOutcome) {
        // Попробовать точный матч по event_id
        if let Some(entry) = self
            .entries
            .iter_mut()
            .rev()
            .find(|e| e.computed_at_event == event_id && e.outcome == AdvisoryHistoryOutcome::Pending)
        {
            entry.outcome = outcome;
            return;
        }
        // Fallback: последняя Pending запись
        if let Some(entry) = self
            .entries
            .iter_mut()
            .rev()
            .find(|e| e.outcome == AdvisoryHistoryOutcome::Pending)
        {
            entry.outcome = outcome;
        }
    }
}

/// Хранилище истории советов: sutra_id → ring-буфер.
pub struct AdvisoryHistory {
    per_sutra: HashMap<u32, AdvisoryRingBuffer>,
    cap_per_sutra: usize,
}

impl AdvisoryHistory {
    pub const DEFAULT_CAP: usize = 32;

    pub fn new() -> Self {
        Self::with_cap(Self::DEFAULT_CAP)
    }

    pub fn with_cap(cap: usize) -> Self {
        Self { per_sutra: HashMap::new(), cap_per_sutra: cap }
    }

    pub fn record(&mut self, sutra_id: u32, entry: AdvisoryHistoryEntry) {
        self.per_sutra
            .entry(sutra_id)
            .or_insert_with(|| AdvisoryRingBuffer::new(self.cap_per_sutra))
            .push(entry);
    }

    /// Обновить исход для sutra_id по event_id.
    pub fn update_outcome(
        &mut self,
        sutra_id: u32,
        event_id: u64,
        outcome: AdvisoryHistoryOutcome,
    ) {
        if let Some(buf) = self.per_sutra.get_mut(&sutra_id) {
            buf.update_outcome(event_id, outcome);
        }
    }

    pub fn get(&self, sutra_id: u32) -> Option<&AdvisoryRingBuffer> {
        self.per_sutra.get(&sutra_id)
    }

    pub fn len(&self) -> usize {
        self.per_sutra.len()
    }

    pub fn is_empty(&self) -> bool {
        self.per_sutra.is_empty()
    }
}

impl Default for AdvisoryHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(event_id: u64, oct: Option<Octant>) -> AdvisoryHistoryEntry {
        AdvisoryHistoryEntry {
            computed_at_event: event_id,
            octant_suggestion: oct,
            octant_confidence: 0.7,
            subsystem_suggestion: None,
            subsystem_confidence: 0.0,
            outcome: AdvisoryHistoryOutcome::Pending,
        }
    }

    #[test]
    fn test_ring_buffer_cap_evicts_oldest() {
        let mut buf = AdvisoryRingBuffer::new(3);
        buf.push(entry(1, Some(Octant::CreativeAffirmation)));
        buf.push(entry(2, Some(Octant::EcstaticAffirmation)));
        buf.push(entry(3, Some(Octant::HeroicFatal)));
        buf.push(entry(4, Some(Octant::DestructiveActivating)));
        assert_eq!(buf.len(), 3);
        // Oldest (event_id=1) evicted
        assert!(buf.iter().all(|e| e.computed_at_event != 1));
    }

    #[test]
    fn test_update_outcome_finds_by_event_id() {
        let mut buf = AdvisoryRingBuffer::new(32);
        buf.push(entry(100, Some(Octant::CreativeAffirmation)));
        buf.push(entry(200, Some(Octant::EcstaticAffirmation)));
        buf.update_outcome(100, AdvisoryHistoryOutcome::Confirmed);
        let e100 = buf.iter().find(|e| e.computed_at_event == 100).unwrap();
        assert_eq!(e100.outcome, AdvisoryHistoryOutcome::Confirmed);
        let e200 = buf.iter().find(|e| e.computed_at_event == 200).unwrap();
        assert_eq!(e200.outcome, AdvisoryHistoryOutcome::Pending);
    }

    #[test]
    fn test_update_outcome_fallback_last_pending() {
        let mut buf = AdvisoryRingBuffer::new(32);
        buf.push(entry(100, Some(Octant::CreativeAffirmation)));
        // event_id=999 не совпадает → fallback
        buf.update_outcome(999, AdvisoryHistoryOutcome::Rejected);
        assert_eq!(buf.iter().last().unwrap().outcome, AdvisoryHistoryOutcome::Rejected);
    }

    #[test]
    fn test_acceptance_rate_empty_returns_zero() {
        let buf = AdvisoryRingBuffer::new(32);
        assert_eq!(buf.acceptance_rate_octant(), 0.0);
    }

    #[test]
    fn test_acceptance_rate_all_pending_returns_zero() {
        let mut buf = AdvisoryRingBuffer::new(32);
        buf.push(entry(1, Some(Octant::CreativeAffirmation)));
        assert_eq!(buf.acceptance_rate_octant(), 0.0);
    }

    #[test]
    fn test_acceptance_rate_mixed() {
        let mut buf = AdvisoryRingBuffer::new(32);
        let mut e1 = entry(1, Some(Octant::CreativeAffirmation));
        e1.outcome = AdvisoryHistoryOutcome::Confirmed;
        let mut e2 = entry(2, Some(Octant::EcstaticAffirmation));
        e2.outcome = AdvisoryHistoryOutcome::Rejected;
        buf.push(e1);
        buf.push(e2);
        assert!((buf.acceptance_rate_octant() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dominant_accepted_octant_votes_correctly() {
        let mut buf = AdvisoryRingBuffer::new(32);
        let mut e1 = entry(1, Some(Octant::HeroicFatal));
        e1.outcome = AdvisoryHistoryOutcome::Applied;
        let mut e2 = entry(2, Some(Octant::HeroicFatal));
        e2.outcome = AdvisoryHistoryOutcome::Applied;
        let mut e3 = entry(3, Some(Octant::CreativeAffirmation));
        e3.outcome = AdvisoryHistoryOutcome::Applied;
        buf.push(e1);
        buf.push(e2);
        buf.push(e3);
        assert_eq!(buf.dominant_accepted_octant(), Some(Octant::HeroicFatal));
    }

    #[test]
    fn test_dominant_accepted_octant_none_when_all_rejected() {
        let mut buf = AdvisoryRingBuffer::new(32);
        let mut e = entry(1, Some(Octant::CreativeAffirmation));
        e.outcome = AdvisoryHistoryOutcome::Rejected;
        buf.push(e);
        assert_eq!(buf.dominant_accepted_octant(), None);
    }

    #[test]
    fn test_advisory_history_record_and_get() {
        let mut hist = AdvisoryHistory::new();
        hist.record(42, entry(100, Some(Octant::CreativeAffirmation)));
        assert!(hist.get(42).is_some());
        assert_eq!(hist.get(42).unwrap().len(), 1);
        assert!(hist.get(99).is_none());
    }

    #[test]
    fn test_advisory_history_update_outcome() {
        let mut hist = AdvisoryHistory::new();
        hist.record(42, entry(100, Some(Octant::CreativeAffirmation)));
        hist.update_outcome(42, 100, AdvisoryHistoryOutcome::Confirmed);
        let entry = hist.get(42).unwrap().iter().next().unwrap();
        assert_eq!(entry.outcome, AdvisoryHistoryOutcome::Confirmed);
    }
}
