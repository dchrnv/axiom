// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Явные null-реализации советников.
//
// Разница между None в registry и NullAdvisor:
// - None: слот незаполнен (советник не сконфигурирован)
// - NullAdvisor: слот занят, советник есть, но намеренно ничего не советует
//
// NullAdvisor нужен когда хочется зафиксировать что советник "присутствует"
// в конфигурации (виден в геноме, логируется) но не влияет на результат.

use crate::over_domain::neural_advisor::traits::{
    ConflictAdvisorInput, ConflictDiagnosis, ConflictResolutionHint, CorpusCallosumResolver,
    DepthAdvisorInput, DepthHint, DepthPredictionAdvisor, EmergentAdvisorInput,
    EmergentDetectionResult, EmergentPatternAdvisor, OctantAdvisorInput, OctantCorrectionAdvisor,
    OctantSuggestion, SubsystemAdvisorInput, SubsystemAttributionAdvisor, SubsystemSuggestion,
};

pub struct NullDepthAdvisor;
pub struct NullOctantAdvisor;
pub struct NullConflictResolver;
pub struct NullSubsystemAdvisor;
pub struct NullEmergentAdvisor;

impl DepthPredictionAdvisor for NullDepthAdvisor {
    fn predict_depth(&self, _input: &DepthAdvisorInput) -> Option<DepthHint> {
        None
    }
}

impl OctantCorrectionAdvisor for NullOctantAdvisor {
    fn suggest_octant(&self, _input: &OctantAdvisorInput) -> Option<OctantSuggestion> {
        None
    }
}

impl CorpusCallosumResolver for NullConflictResolver {
    fn resolve(&self, _input: &ConflictAdvisorInput) -> ConflictResolutionHint {
        ConflictResolutionHint {
            diagnosis: ConflictDiagnosis::Unresolved,
            confidence: 0.0,
        }
    }
}

impl SubsystemAttributionAdvisor for NullSubsystemAdvisor {
    fn suggest_subsystem(&self, _input: &SubsystemAdvisorInput) -> Option<SubsystemSuggestion> {
        None
    }
}

impl EmergentPatternAdvisor for NullEmergentAdvisor {
    fn detect(&self, _input: &EmergentAdvisorInput) -> EmergentDetectionResult {
        EmergentDetectionResult {
            is_candidate: false,
            confidence: 0.0,
        }
    }
}
