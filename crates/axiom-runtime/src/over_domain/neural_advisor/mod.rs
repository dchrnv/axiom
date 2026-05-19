// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NeuralAdvisor — шестой над-доменный модуль.
// Advisory-only: читает результаты AxialEvaluator и ContextRecognizer,
// даёт рекомендации не изменяя их выводы.
//
// Источник: docs/architecture/NeuralAdvisor_V1_0.md

use std::sync::Arc;

use axiom_core::{STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_domain::AshtiCore;
use axiom_experience::{
    AxialStore, EmergentPrimitiveStore, InterpretationProfileStore, Octant, SutraDepthStore,
};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::{NotifyEmergentCandidatePayload, OpCode, UclCommand};

use crate::over_domain::arbiter::source::{
    Advisory, AdvisoryAction, AdvisoryId, AdvisoryOutcome, AdvisorySource, AdvisoryType, SourceId,
};
use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod implementations;
pub mod registry;
pub mod results;
pub mod traits;

pub use implementations::{
    DepthThresholdEmergentDetector, NullConflictResolver, NullDepthAdvisor, NullEmergentAdvisor,
    NullOctantAdvisor, NullSubsystemAdvisor, RuleBasedCorpusCallosumResolver,
    EMERGENT_CANDIDATE_MIN_AGE_TICKS, EMERGENT_CANDIDATE_MIN_DEPTH,
    EMERGENT_CANDIDATE_MIN_REACTIVATIONS,
};
pub use registry::NeuralAdvisorRegistry;
pub use results::{AdvisoryResult, AdvisoryResultStore};
pub use traits::{
    ConflictAdvisorInput, ConflictDiagnosis, ConflictResolutionHint, CorpusCallosumResolver,
    DepthAdvisorInput, DepthHint, DepthPredictionAdvisor, EmergentAdvisorInput,
    EmergentDetectionResult, EmergentPatternAdvisor, OctantAdvisorInput, OctantCorrectionAdvisor,
    OctantSuggestion, OctantSuggestionReason, SubsystemAdvisorInput, SubsystemAttributionAdvisor,
    SubsystemSuggestion,
};

/// Интервал: каждые 11 тиков (нечётное, не совпадает с AE=5, CR=7).
pub const NEURAL_ADVISOR_TICK_INTERVAL: u32 = 11;

/// EXPERIENCE domain: level_id * 100 + 9.
const EXPERIENCE_ROLE: u16 = 9;

pub struct NeuralAdvisor {
    registry: NeuralAdvisorRegistry,
    result_store: AdvisoryResultStore,
    emergent_store: EmergentPrimitiveStore,
    /// Снапшот от AxialEvaluator (sync_axial_store).
    axial_store_snapshot: AxialStore,
    /// Снапшот от ContextRecognizer (sync_profile_store).
    profile_store_snapshot: InterpretationProfileStore,
    /// Снапшот от ContextRecognizer (sync_depth_store).
    depth_store_snapshot: SutraDepthStore,
}

impl NeuralAdvisor {
    pub fn new(registry: NeuralAdvisorRegistry) -> Self {
        Self {
            registry,
            result_store: AdvisoryResultStore::new(),
            emergent_store: EmergentPrimitiveStore::new(),
            axial_store_snapshot: AxialStore::new(),
            profile_store_snapshot: InterpretationProfileStore::new(),
            depth_store_snapshot: SutraDepthStore::new(),
        }
    }



    // ─── Синхронизация снапшотов ─────────────────────────────────────────────
    // Вызываются координатором после тиков соответствующих компонентов.
    // Архитектурный долг: CR-TD-01 (DEFERRED.md).

    pub fn sync_axial_store(&mut self, store: &AxialStore) {
        self.axial_store_snapshot = store.clone();
    }

    pub fn sync_profile_store(&mut self, store: &InterpretationProfileStore) {
        self.profile_store_snapshot = store.clone();
    }

    pub fn sync_depth_store(&mut self, store: &SutraDepthStore) {
        self.depth_store_snapshot = store.clone();
    }

    // ─── Публичный доступ к результатам ──────────────────────────────────────

    pub fn result_store(&self) -> &AdvisoryResultStore {
        &self.result_store
    }

    pub fn emergent_store(&self) -> &EmergentPrimitiveStore {
        &self.emergent_store
    }

    /// Одобрить emergent-кандидата (через UCL ApproveEmergentCandidate от chrnv).
    pub fn approve_emergent(&mut self, sutra_id: u32) -> bool {
        self.emergent_store.approve(sutra_id)
    }

    // ─── Внутренняя логика ────────────────────────────────────────────────────

    fn process_frame(
        &self,
        sutra_id: u32,
        frame_age_ticks: u64,
        event_id: u64,
        known_primitive_ids: &[u32],
    ) -> (AdvisoryResult, bool) {
        let mut result = AdvisoryResult::new(sutra_id, event_id);
        let mut is_emergent_candidate = false;

        // Depth snapshot
        let depth_entry = self.depth_store_snapshot.get(sutra_id);
        let depth_per_octant = depth_entry.map(|e| e.depth_per_octant).unwrap_or([0u16; 8]);
        let reactivation_count = depth_entry.map(|e| e.reactivation_count).unwrap_or(0);

        // Profile snapshot
        let profile = self.profile_store_snapshot.get(sutra_id);
        let primary_subsystem = profile
            .map(|p| p.primary)
            .unwrap_or(axiom_experience::SubsystemId::Unknown);
        let primary_octant = profile
            .map(|p| p.primary_octant)
            .unwrap_or(Octant::CreativeAffirmation);

        // ─── OctantCorrectionAdvisor ─────────────────────────────────────────
        if let Some(advisor) = &self.registry.octant {
            // Берём последнюю оценку из AxialStore
            if let Some(eval) = self.axial_store_snapshot.get_latest(sutra_id) {
                let input = traits::OctantAdvisorInput {
                    sutra_id,
                    analytic_octant: eval.octant,
                    synthetic_octant: eval
                        .conflict
                        .as_ref()
                        .map(|c| c.synthetic_octant)
                        .unwrap_or(eval.octant),
                    evaluation_level: eval.level,
                    x_positive_pole: eval.x_axis.positive_pole,
                    x_negative_pole: eval.x_axis.negative_pole,
                    y_positive_pole: eval.y_axis.positive_pole,
                    y_negative_pole: eval.y_axis.negative_pole,
                    z_positive_pole: eval.z_axis.positive_pole,
                    z_negative_pole: eval.z_axis.negative_pole,
                    primary_subsystem,
                    event_id,
                };
                result.octant_suggestion = advisor.suggest_octant(&input);
            }
        }

        // ─── CorpusCallosumResolver ──────────────────────────────────────────
        if let Some(resolver) = &self.registry.conflict {
            if let Some(eval) = self.axial_store_snapshot.get_latest(sutra_id) {
                if let Some(conflict) = &eval.conflict {
                    let input = traits::ConflictAdvisorInput {
                        sutra_id,
                        analytic_octant: conflict.analytic_octant,
                        synthetic_octant: conflict.synthetic_octant,
                        conflict_strength: conflict.conflict_strength,
                        frame_age_ticks,
                        reactivation_count,
                        primary_subsystem,
                        event_id,
                    };
                    result.conflict_diagnosis = Some(resolver.resolve(&input));
                }
            }
        }

        // ─── SubsystemAttributionAdvisor ─────────────────────────────────────
        if let Some(advisor) = &self.registry.subsystem {
            let energy_weights: Vec<(axiom_experience::SubsystemId, u8)> = profile
                .map(|p| p.weights.iter().map(|(&s, &w)| (s, w)).collect())
                .unwrap_or_default();
            let input = traits::SubsystemAdvisorInput {
                sutra_id,
                energy_weights,
                primary_octant,
                depth_per_octant,
                reactivation_count,
                event_id,
            };
            result.subsystem_suggestion = advisor.suggest_subsystem(&input);
        }

        // ─── DepthPredictionAdvisor ──────────────────────────────────────────
        if let Some(advisor) = &self.registry.depth {
            let input = traits::DepthAdvisorInput {
                sutra_id,
                subsystem: primary_subsystem,
                current_depth_per_octant: depth_per_octant,
                reactivation_count,
                frame_age_ticks,
                primary_octant,
                event_id,
            };
            result.depth_hint = advisor.predict_depth(&input);
        }

        // ─── EmergentPatternAdvisor ──────────────────────────────────────────
        if let Some(advisor) = &self.registry.emergent {
            let input = traits::EmergentAdvisorInput {
                sutra_id,
                octant: primary_octant,
                depth_per_octant,
                reactivation_count,
                frame_age_ticks,
                known_primitive_ids: known_primitive_ids.to_vec(),
                event_id,
            };
            let detection = advisor.detect(&input);
            if detection.is_candidate {
                is_emergent_candidate = true;
            }
        }

        (result, is_emergent_candidate)
    }

    fn make_notify_command(&self, sutra_id: u32, octant: Octant, depth: u16, confidence: f32) -> UclCommand {
        let payload = NotifyEmergentCandidatePayload {
            sutra_id,
            octant: octant.index() as u8,
            confidence_scaled: (confidence * 255.0) as u8,
            depth,
            reserved: [0; 40],
        };
        UclCommand::new(OpCode::NotifyEmergentCandidate, sutra_id, 120, 0)
            .with_payload(&payload)
    }

    pub fn with_default_v1() -> Self {
        Self::new(NeuralAdvisorRegistry::default_v1())
    }
}

impl Default for NeuralAdvisor {
    fn default() -> Self {
        Self::with_default_v1()
    }
}

impl OverDomainComponent for NeuralAdvisor {
    fn name(&self) -> &'static str {
        "NeuralAdvisor"
    }

    fn module_id(&self) -> ModuleId {
        ModuleId::NeuralAdvisor
    }

    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
        use axiom_genome::types::{Permission, ResourceId};
        use axiom_genome::GenomeIndex;
        let index = GenomeIndex::build(genome);
        for resource in [ResourceId::ExperienceMemory, ResourceId::AshtiField] {
            if !index.check_access(ModuleId::NeuralAdvisor, resource, Permission::Read) {
                return Err(OverDomainError::GenomeDenied);
            }
        }
        Ok(())
    }

    fn on_tick_interval(&self) -> u32 {
        NEURAL_ADVISOR_TICK_INTERVAL
    }

    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<Vec<UclCommand>, OverDomainError> {
        let level = ashti.level_id();
        let exp_domain_id = level * 100 + EXPERIENCE_ROLE;

        let exp_state = match ashti.index_of(exp_domain_id).and_then(|i| ashti.state(i)) {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        // Список активных Frame-анкеров
        let frame_anchors: Vec<(u32, u64)> = exp_state
            .tokens
            .iter()
            .filter(|t| {
                (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0 && t.state == STATE_ACTIVE
            })
            .map(|t| (t.sutra_id, t.last_event_id))
            .collect();

        // Известные примитивы из depth_store (register_primitive записывает PRIMITIVE_DEPTH)
        let known_primitives: Vec<u32> = frame_anchors
            .iter()
            .filter_map(|&(id, _)| {
                self.depth_store_snapshot
                    .get(id)
                    .filter(|e| e.is_primitive())
                    .map(|_| id)
            })
            .collect();

        let mut ucl_commands = Vec::new();

        for (sutra_id, crystallized_at) in &frame_anchors {
            let frame_age = tick.saturating_sub(*crystallized_at);
            let (result, is_candidate) =
                self.process_frame(*sutra_id, frame_age, tick, &known_primitives);

            if is_candidate && !self.emergent_store.is_at_capacity() {
                // Определить глубину и октант для уведомления
                let depth_entry = self.depth_store_snapshot.get(*sutra_id);
                let primary_octant = self
                    .profile_store_snapshot
                    .get(*sutra_id)
                    .map(|p| p.primary_octant)
                    .unwrap_or(Octant::CreativeAffirmation);
                let depth = depth_entry
                    .map(|e| e.depth_per_octant[primary_octant.index()])
                    .unwrap_or(0);

                // Добавить в emergent store только если ещё не там
                if self.emergent_store.get_all().iter().all(|e| e.sutra_id != *sutra_id) {
                    use axiom_experience::EmergentPrimitive;
                    let primitive = EmergentPrimitive::new(*sutra_id, tick, primary_octant, depth);
                    self.emergent_store.add(primitive);
                    ucl_commands.push(
                        self.make_notify_command(*sutra_id, primary_octant, depth, 0.60),
                    );
                }
            }

            self.result_store.insert(result);
        }

        Ok(ucl_commands)
    }

    fn on_shutdown(&mut self) -> Vec<UclCommand> {
        Vec::new()
    }
}

// ── AdvisorySource для Arbiter ────────────────────────────────────────────────

/// source_id NeuralAdvisor в OverDomainArbiter.
pub const NEURAL_ADVISOR_SOURCE_ID: SourceId = 0;

impl AdvisorySource for NeuralAdvisor {
    fn source_id(&self) -> SourceId {
        NEURAL_ADVISOR_SOURCE_ID
    }

    fn poll_advisories(&self) -> Vec<Advisory> {
        let mut out = Vec::new();
        for result in self.result_store.frames_with_advice() {
            // DepthHint → ApplyDepth
            if let Some(ref hint) = result.depth_hint {
                out.push(Advisory {
                    id: advisory_id(result.sutra_id, AdvisoryType::DepthHint),
                    source: NEURAL_ADVISOR_SOURCE_ID,
                    advisory_type: AdvisoryType::DepthHint,
                    subject_id: result.sutra_id,
                    confidence: hint.confidence,
                    action: AdvisoryAction::ApplyDepth {
                        octant: hint.target_octant.index(),
                        depth: hint.suggested_depth,
                    },
                    created_at_event: result.computed_at_event,
                });
            }
            // OctantCorrection → NotifyWorkstation
            if let Some(ref sug) = result.octant_suggestion {
                out.push(Advisory {
                    id: advisory_id(result.sutra_id, AdvisoryType::OctantCorrection),
                    source: NEURAL_ADVISOR_SOURCE_ID,
                    advisory_type: AdvisoryType::OctantCorrection,
                    subject_id: result.sutra_id,
                    confidence: sug.confidence,
                    action: AdvisoryAction::NotifyWorkstation {
                        label: format!(
                            "#{} octant → {:?} ({})",
                            result.sutra_id, sug.octant, sug.confidence
                        ),
                    },
                    created_at_event: result.computed_at_event,
                });
            }
        }
        out
    }

    fn on_feedback(&mut self, _id: AdvisoryId, _outcome: AdvisoryOutcome) {
        // V2: логировать применение/отклонение для калибровки советников
    }
}

/// Стабильный AdvisoryId: (sutra_id << 8) | type_index.
fn advisory_id(sutra_id: u32, t: AdvisoryType) -> AdvisoryId {
    let type_index = match t {
        AdvisoryType::DepthHint => 0u64,
        AdvisoryType::OctantCorrection => 1,
        AdvisoryType::ConflictDiagnosis => 2,
        AdvisoryType::SubsystemAttribution => 3,
        AdvisoryType::EmergentCandidate => 4,
    };
    ((sutra_id as u64) << 8) | type_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_registry_has_conflict_and_emergent() {
        let na = NeuralAdvisor::with_default_v1();
        assert!(na.registry.conflict.is_some());
        assert!(na.registry.emergent.is_some());
        assert!(na.registry.depth.is_some());
        assert!(na.registry.octant.is_none());
        assert!(na.registry.subsystem.is_none());
    }

    #[test]
    fn test_empty_registry_no_results() {
        let na = NeuralAdvisor::new(NeuralAdvisorRegistry::empty());
        assert!(na.result_store().is_empty());
        assert!(na.emergent_store().is_empty());
    }

    #[test]
    fn test_sync_stores_no_panic() {
        let mut na = NeuralAdvisor::with_default_v1();
        na.sync_axial_store(&AxialStore::new());
        na.sync_depth_store(&SutraDepthStore::new());
        na.sync_profile_store(&InterpretationProfileStore::new());
    }

    #[test]
    fn test_approve_emergent_unknown_returns_false() {
        let mut na = NeuralAdvisor::with_default_v1();
        assert!(!na.approve_emergent(999));
    }

    #[test]
    fn test_tick_interval() {
        let na = NeuralAdvisor::with_default_v1();
        assert_eq!(na.on_tick_interval(), NEURAL_ADVISOR_TICK_INTERVAL);
        assert_eq!(NEURAL_ADVISOR_TICK_INTERVAL, 11);
    }
}
