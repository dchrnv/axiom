// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AdvisoryResult + AdvisoryResultStore — результаты всех советников для одного Frame.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §6

use std::collections::HashMap;

use crate::over_domain::neural_advisor::traits::{
    ConflictResolutionHint, DepthHint, OctantSuggestion, SubsystemSuggestion,
};

/// Совокупный advisory-результат для одного Frame за последний тик NeuralAdvisor.
#[derive(Debug, Clone)]
pub struct AdvisoryResult {
    pub sutra_id: u32,
    pub computed_at_event: u64,
    /// Предложение по октанту (advisory-only, не меняет AxialEvaluation)
    pub octant_suggestion: Option<OctantSuggestion>,
    /// Диагноз конфликта Corpus Callosum (advisory-only, не меняет ConflictResolution)
    pub conflict_diagnosis: Option<ConflictResolutionHint>,
    /// Предложение по подсистеме (advisory-only, не меняет InterpretationProfile.primary)
    pub subsystem_suggestion: Option<SubsystemSuggestion>,
    /// Подсказка по глубине (advisory-only, не меняет SutraDepthStore)
    pub depth_hint: Option<DepthHint>,
    // Emergent кандидаты не хранятся здесь — они идут прямо в EmergentPrimitiveStore
    // и через UCL NotifyEmergentCandidate.
}

impl AdvisoryResult {
    pub fn new(sutra_id: u32, computed_at_event: u64) -> Self {
        Self {
            sutra_id,
            computed_at_event,
            octant_suggestion: None,
            conflict_diagnosis: None,
            subsystem_suggestion: None,
            depth_hint: None,
        }
    }

    /// Есть ли хоть одна рекомендация (не считая None-ов).
    pub fn has_any(&self) -> bool {
        self.octant_suggestion.is_some()
            || self.conflict_diagnosis.is_some()
            || self.subsystem_suggestion.is_some()
            || self.depth_hint.is_some()
    }
}

/// Хранилище последних advisory-результатов — по одному на Frame.
///
/// Overwrite при каждом тике NeuralAdvisor. История не накапливается (V2+).
#[derive(Debug, Default)]
pub struct AdvisoryResultStore {
    results: HashMap<u32, AdvisoryResult>,
}

impl AdvisoryResultStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, result: AdvisoryResult) {
        self.results.insert(result.sutra_id, result);
    }

    pub fn get(&self, sutra_id: u32) -> Option<&AdvisoryResult> {
        self.results.get(&sutra_id)
    }

    pub fn remove(&mut self, sutra_id: u32) -> Option<AdvisoryResult> {
        self.results.remove(&sutra_id)
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Фреймы у которых хотя бы один советник дал рекомендацию.
    pub fn frames_with_advice(&self) -> impl Iterator<Item = &AdvisoryResult> {
        self.results.values().filter(|r| r.has_any())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_store() {
        let store = AdvisoryResultStore::new();
        assert!(store.is_empty());
        assert!(store.get(1).is_none());
    }

    #[test]
    fn test_insert_and_get() {
        let mut store = AdvisoryResultStore::new();
        store.insert(AdvisoryResult::new(42, 100));
        assert_eq!(store.len(), 1);
        assert_eq!(store.get(42).unwrap().sutra_id, 42);
    }

    #[test]
    fn test_overwrite_on_same_id() {
        let mut store = AdvisoryResultStore::new();
        store.insert(AdvisoryResult::new(1, 10));
        store.insert(AdvisoryResult::new(1, 20));
        assert_eq!(store.len(), 1);
        assert_eq!(store.get(1).unwrap().computed_at_event, 20);
    }

    #[test]
    fn test_has_any_on_empty_result() {
        let r = AdvisoryResult::new(5, 0);
        assert!(!r.has_any());
    }
}
