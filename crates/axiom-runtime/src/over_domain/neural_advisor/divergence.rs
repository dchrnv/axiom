// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DivergenceLog — G1 NeuralAdvisor V3.
// Записывает расхождения advisory_octant ↔ analytic_octant при Hamming distance ≥ 2.
// Источник: docs/ROADMAP.md §Phase G / G1

use axiom_experience::Octant;

const DIVERGENCE_LOG_CAPACITY: usize = 256;

/// Запись расхождения октантов.
#[derive(Debug, Clone)]
pub struct DivergenceEntry {
    pub event_id: u64,
    pub sutra_id: u32,
    pub analytic_octant: Octant,
    pub advisory_octant: Octant,
    /// Hamming distance по осям (0–3).
    pub distance: usize,
    pub advisor_confidence: f32,
}

/// Скользящий буфер последних 256 расхождений октантов.
pub struct DivergenceLog {
    entries: std::collections::VecDeque<DivergenceEntry>,
    total_recorded: u64,
}

impl DivergenceLog {
    pub fn new() -> Self {
        Self {
            entries: std::collections::VecDeque::with_capacity(DIVERGENCE_LOG_CAPACITY),
            total_recorded: 0,
        }
    }

    pub fn push(&mut self, entry: DivergenceEntry) {
        if self.entries.len() >= DIVERGENCE_LOG_CAPACITY {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
        self.total_recorded += 1;
    }

    pub fn entries(&self) -> impl Iterator<Item = &DivergenceEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Всего записано расхождений за время жизни лога (не сбрасывается при вытеснении).
    pub fn total_recorded(&self) -> u64 {
        self.total_recorded
    }
}

impl Default for DivergenceLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Hamming distance между двумя октантами (количество отличающихся осей, 0–3).
pub fn octant_hamming_distance(a: Octant, b: Octant) -> usize {
    (a.index() ^ b.index()).count_ones() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_same_octant_is_zero() {
        assert_eq!(octant_hamming_distance(Octant::CreativeAffirmation, Octant::CreativeAffirmation), 0);
    }

    #[test]
    fn test_hamming_adjacent_octant_is_one() {
        // CreativeAffirmation (0=000) vs EcstaticAffirmation (1=001): XOR=001 → 1 bit
        assert_eq!(octant_hamming_distance(Octant::CreativeAffirmation, Octant::EcstaticAffirmation), 1);
    }

    #[test]
    fn test_hamming_opposite_octant_is_three() {
        // CreativeAffirmation (0=000) vs SelfDestructiveApathic (7=111): XOR=111 → 3 bits
        assert_eq!(octant_hamming_distance(Octant::CreativeAffirmation, Octant::SelfDestructiveApathic), 3);
    }

    #[test]
    fn test_hamming_two_axes_differ() {
        // CreativeAffirmation (0=000) vs DestructiveActivating (3=011): XOR=011 → 2 bits
        assert_eq!(octant_hamming_distance(Octant::CreativeAffirmation, Octant::DestructiveActivating), 2);
    }

    #[test]
    fn test_log_push_and_len() {
        let mut log = DivergenceLog::new();
        assert!(log.is_empty());
        log.push(DivergenceEntry {
            event_id: 1,
            sutra_id: 42,
            analytic_octant: Octant::CreativeAffirmation,
            advisory_octant: Octant::SelfDestructiveApathic,
            distance: 3,
            advisor_confidence: 0.75,
        });
        assert_eq!(log.len(), 1);
        assert_eq!(log.total_recorded(), 1);
    }

    #[test]
    fn test_log_capacity_ring() {
        let mut log = DivergenceLog::new();
        for i in 0..300u64 {
            log.push(DivergenceEntry {
                event_id: i,
                sutra_id: 1,
                analytic_octant: Octant::CreativeAffirmation,
                advisory_octant: Octant::SelfDestructiveApathic,
                distance: 3,
                advisor_confidence: 0.5,
            });
        }
        assert_eq!(log.len(), 256);
        assert_eq!(log.total_recorded(), 300);
        // Oldest entries evicted: first entry should have event_id = 44 (300 - 256)
        assert_eq!(log.entries().next().unwrap().event_id, 44);
    }
}
