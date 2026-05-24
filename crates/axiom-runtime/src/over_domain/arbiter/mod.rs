// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// OverDomainArbiter — слушатель advisory-источников.
// Читает рекомендации, решает по TrustConfig, действует через SutraDepthStore / UCL.
//
// Источник: docs/architecture/OverDomainArbiter_V2_0.md

use std::collections::VecDeque;
use std::sync::Arc;

use axiom_domain::AshtiCore;
use axiom_experience::SutraDepthStore;
use axiom_genome::{Genome, ModuleId, Permission, ResourceId};
use axiom_ucl::UclCommand;

use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod log;
pub mod profile;
pub mod source;
pub mod trust;

pub use log::{ArbiterLog, ArbiterLogEntry, ArbiterOutcome};
pub use profile::CognitiveProfile;
pub use source::{Advisory, AdvisoryAction, AdvisoryId, AdvisoryOutcome, AdvisorySource,
                 AdvisoryType, SourceId};
pub use trust::{TrustConfig, TrustEntry, TrustMode};

/// Тик-интервал: 13 (простое, не совпадает с AE=5, CR=7, NA=11).
pub const ARBITER_TICK_INTERVAL: u32 = 13;

/// V2: TTL очереди в event_id единицах.
pub const PENDING_TTL: u64 = 1000;

/// Рекомендация в очереди на подтверждение chrnv.
#[derive(Debug, Clone)]
pub struct PendingAdvisory {
    pub advisory: Advisory,
    pub queued_at_event: u64,
    /// V2: event_id после которого advisory считается устаревшим.
    pub expires_at_event: u64,
}

pub struct OverDomainArbiter {
    sources: Vec<Box<dyn AdvisorySource>>,
    trust: TrustConfig,
    pending: VecDeque<PendingAdvisory>,
    log: ArbiterLog,
    /// Permission::Control на ExperienceMemory — разрешает AutoApply.
    auto_apply_allowed: bool,
    /// Когнитивный профиль: масштабирует confidence OctantCorrection advisory по октанту.
    cognitive_profile: CognitiveProfile,
    /// V3: pending octant overrides для AxialEvaluatorStorage (sutra_id, octant_idx).
    pending_overrides: Vec<(u32, usize)>,
    /// V3: feedback для незарегистрированных источников (source_id, advisory_id, outcome).
    unrouted_feedback: Vec<(SourceId, AdvisoryId, AdvisoryOutcome)>,
}

fn advisory_type_to_u8(atype: AdvisoryType) -> u8 {
    match atype {
        AdvisoryType::DepthHint            => 0,
        AdvisoryType::OctantCorrection     => 1,
        AdvisoryType::ConflictDiagnosis    => 2,
        AdvisoryType::SubsystemAttribution => 3,
        AdvisoryType::EmergentCandidate    => 4,
        AdvisoryType::NarrativeShift       => 5,
    }
}

fn advisory_type_from_u8(v: u8) -> Option<AdvisoryType> {
    match v {
        0 => Some(AdvisoryType::DepthHint),
        1 => Some(AdvisoryType::OctantCorrection),
        2 => Some(AdvisoryType::ConflictDiagnosis),
        3 => Some(AdvisoryType::SubsystemAttribution),
        4 => Some(AdvisoryType::EmergentCandidate),
        5 => Some(AdvisoryType::NarrativeShift),
        _ => None,
    }
}

impl OverDomainArbiter {
    pub fn new(trust: TrustConfig) -> Self {
        Self {
            sources: Vec::new(),
            trust,
            pending: VecDeque::new(),
            log: ArbiterLog::new(),
            auto_apply_allowed: false,
            cognitive_profile: CognitiveProfile::default(),
            pending_overrides: Vec::new(),
            unrouted_feedback: Vec::new(),
        }
    }

    pub fn cognitive_profile(&self) -> &CognitiveProfile {
        &self.cognitive_profile
    }

    pub fn cognitive_profile_mut(&mut self) -> &mut CognitiveProfile {
        &mut self.cognitive_profile
    }

    /// V3: забрать накопленные octant overrides для AxialEvaluatorStorage.
    pub fn drain_octant_overrides(&mut self) -> Vec<(u32, usize)> {
        std::mem::take(&mut self.pending_overrides)
    }

    /// V3: забрать feedback для незарегистрированных источников.
    pub fn drain_unrouted_feedback(&mut self) -> Vec<(SourceId, AdvisoryId, AdvisoryOutcome)> {
        std::mem::take(&mut self.unrouted_feedback)
    }

    pub fn default_v1() -> Self {
        Self::new(TrustConfig::default_v1(0))
    }

    /// ARB-TD-05: экспортировать калиброванные min_confidence значения TrustConfig.
    pub fn export_trust_calibration(&self) -> Vec<(u8, u8, f32)> {
        self.trust.iter_entries().map(|((src, atype), entry)| {
            (*src, advisory_type_to_u8(*atype), entry.min_confidence)
        }).collect()
    }

    /// ARB-TD-05: восстановить min_confidence значения в TrustConfig.
    pub fn import_trust_calibration(&mut self, data: &[(u8, u8, f32)]) {
        for &(src, atype_u8, min_conf) in data {
            let Some(atype) = advisory_type_from_u8(atype_u8) else { continue };
            self.trust.set_min_confidence(src, atype, min_conf);
        }
    }

    pub fn register_source(&mut self, source: Box<dyn AdvisorySource>) {
        self.sources.push(source);
    }

    // ── Workstation API ───────────────────────────────────────────────────────

    pub fn pending_snapshot(&self) -> &VecDeque<PendingAdvisory> {
        &self.pending
    }

    pub fn log(&self) -> &ArbiterLog {
        &self.log
    }

    pub fn confirm_pending(&mut self, advisory_id: AdvisoryId, depth_store: &mut SutraDepthStore) {
        if let Some(pos) = self.pending.iter().position(|p| p.advisory.id == advisory_id) {
            let pending = self.pending.remove(pos).unwrap();
            Self::execute_with_overrides(&pending.advisory, depth_store, &mut self.pending_overrides);
            // CognitiveProfile: принятие → увеличить вес октанта
            if let Some(idx) = pending.advisory.octant_hint {
                self.cognitive_profile.update(idx, true);
            }
            self.push_log(&pending.advisory, pending.queued_at_event, ArbiterOutcome::Confirmed);
            self.feedback_source(pending.advisory.source, advisory_id, AdvisoryOutcome::Confirmed);
            // V2: автокалибровка после подтверждения
            let (src, atype) = (pending.advisory.source, pending.advisory.advisory_type);
            self.trust.calibrate(src, atype, &self.log);
        }
    }

    pub fn reject_pending(&mut self, advisory_id: AdvisoryId) {
        if let Some(pos) = self.pending.iter().position(|p| p.advisory.id == advisory_id) {
            let pending = self.pending.remove(pos).unwrap();
            // CognitiveProfile: отклонение → уменьшить вес октанта
            if let Some(idx) = pending.advisory.octant_hint {
                self.cognitive_profile.update(idx, false);
            }
            self.push_log(&pending.advisory, pending.queued_at_event, ArbiterOutcome::Rejected);
            self.feedback_source(pending.advisory.source, advisory_id, AdvisoryOutcome::Rejected);
            // V2: автокалибровка после отклонения
            let (src, atype) = (pending.advisory.source, pending.advisory.advisory_type);
            self.trust.calibrate(src, atype, &self.log);
        }
    }

    // ── Основной тик ─────────────────────────────────────────────────────────

    /// Вызывается из engine.rs вместо on_tick (нужен &mut depth_store).
    pub fn tick_with_stores(
        &mut self,
        event_id: u64,
        advisories: &[Advisory],
        depth_store: &mut SutraDepthStore,
    ) {
        // V2: TTL sweep — истечь advisory старше PENDING_TTL event_id.
        let expired_ids: Vec<AdvisoryId> = self.pending
            .iter()
            .filter(|p| event_id >= p.expires_at_event)
            .map(|p| p.advisory.id)
            .collect();
        for id in expired_ids {
            if let Some(pos) = self.pending.iter().position(|p| p.advisory.id == id) {
                let pending = self.pending.remove(pos).unwrap();
                self.push_log(&pending.advisory, pending.queued_at_event, ArbiterOutcome::Expired);
                self.feedback_source(pending.advisory.source, id, AdvisoryOutcome::Expired);
            }
        }

        for advisory in advisories {
            let Some(entry) = self.trust.get(advisory.source, advisory.advisory_type) else {
                continue;
            };

            // CognitiveProfile: масштабировать confidence для OctantCorrection по октанту.
            let effective_confidence = if advisory.advisory_type == AdvisoryType::OctantCorrection {
                if let Some(idx) = advisory.octant_hint {
                    self.cognitive_profile.scale_confidence(idx, advisory.confidence)
                } else {
                    advisory.confidence
                }
            } else {
                advisory.confidence
            };

            if effective_confidence < entry.min_confidence {
                self.push_log(advisory, event_id, ArbiterOutcome::Skipped);
                self.feedback_source(advisory.source, advisory.id, AdvisoryOutcome::Skipped);
                continue;
            }

            match entry.mode {
                TrustMode::Ignore => {}

                TrustMode::AutoApply => {
                    if self.auto_apply_allowed {
                        Self::execute_with_overrides(advisory, depth_store, &mut self.pending_overrides);
                        self.push_log(advisory, event_id, ArbiterOutcome::Applied);
                        self.feedback_source(advisory.source, advisory.id, AdvisoryOutcome::Applied);
                    } else {
                        // Геном не выдал Control → деградируем до RequireConfirmation
                        self.enqueue(advisory, event_id);
                    }
                }

                TrustMode::RequireConfirmation => {
                    self.enqueue(advisory, event_id);
                }
            }
        }
    }

    // ── Внутренние методы ────────────────────────────────────────────────────

    fn execute_with_overrides(
        advisory: &Advisory,
        depth_store: &mut SutraDepthStore,
        pending_overrides: &mut Vec<(u32, usize)>,
    ) {
        match &advisory.action {
            AdvisoryAction::ApplyDepth { octant, depth } => {
                depth_store.set_promoted_depth(
                    advisory.subject_id,
                    *octant,
                    advisory.created_at_event,
                );
                let _ = depth;
            }
            AdvisoryAction::NotifyWorkstation { .. } => {}
            AdvisoryAction::OverrideOctant { sutra_id, octant_idx } => {
                pending_overrides.push((*sutra_id, *octant_idx));
            }
        }
    }

    fn enqueue(&mut self, advisory: &Advisory, event_id: u64) {
        if self.pending.iter().any(|p| p.advisory.id == advisory.id) {
            return;
        }
        self.pending.push_back(PendingAdvisory {
            advisory: advisory.clone(),
            queued_at_event: event_id,
            expires_at_event: event_id + PENDING_TTL,
        });
        self.push_log(advisory, event_id, ArbiterOutcome::Queued);
        self.feedback_source(advisory.source, advisory.id, AdvisoryOutcome::Queued);
    }

    fn push_log(&mut self, advisory: &Advisory, event_id: u64, outcome: ArbiterOutcome) {
        self.log.push(ArbiterLogEntry {
            event_id,
            advisory_id: advisory.id,
            source: advisory.source,
            advisory_type: advisory.advisory_type,
            subject_id: advisory.subject_id,
            confidence: advisory.confidence,
            outcome,
        });
    }

    fn feedback_source(&mut self, source_id: SourceId, advisory_id: AdvisoryId, outcome: AdvisoryOutcome) {
        if let Some(src) = self.sources.iter_mut().find(|s| s.source_id() == source_id) {
            src.on_feedback(advisory_id, outcome);
        } else {
            // V3: незарегистрированные источники (AxialEvaluator) — буферизовать для Engine.
            self.unrouted_feedback.push((source_id, advisory_id, outcome));
        }
    }
}

impl OverDomainComponent for OverDomainArbiter {
    fn name(&self) -> &'static str { "OverDomainArbiter" }

    fn module_id(&self) -> ModuleId { ModuleId::OverDomainArbiter }

    fn on_tick_interval(&self) -> u32 { ARBITER_TICK_INTERVAL }

    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
        use axiom_genome::GenomeIndex;
        let index = GenomeIndex::build(genome);
        self.auto_apply_allowed = index.check_access(
            ModuleId::OverDomainArbiter,
            ResourceId::ExperienceMemory,
            Permission::Control,
        );

        // V2: загрузить TrustConfig из config/genome.yaml, если секция arbiter.trust задана.
        let genome_path = std::path::Path::new("config/genome.yaml");
        self.trust = trust::TrustConfigLoader::from_genome_yaml(genome_path);

        // V2: загрузить CognitiveProfile из config/profiles/<name>.yaml, если задано.
        if let Some(name) = trust::TrustConfigLoader::profile_name_from_genome_yaml(genome_path) {
            let profile_path = format!("config/profiles/{name}.yaml");
            self.cognitive_profile =
                CognitiveProfile::from_yaml_or_default(std::path::Path::new(&profile_path));
        }

        Ok(())
    }

    fn on_tick(&mut self, _tick: u64, _ashti: &AshtiCore) -> Result<Vec<UclCommand>, OverDomainError> {
        // Реальная логика в tick_with_stores — engine.rs вызывает напрямую.
        Ok(Vec::new())
    }

    fn on_shutdown(&mut self) -> Vec<UclCommand> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::SutraDepthStore;

    fn make_advisory(id: AdvisoryId, source: SourceId, atype: AdvisoryType, confidence: f32) -> Advisory {
        Advisory {
            id,
            source,
            advisory_type: atype,
            subject_id: 1,
            confidence,
            action: AdvisoryAction::NotifyWorkstation { label: "test".into() },
            created_at_event: 0,
            octant_hint: None,
        }
    }

    #[test]
    fn test_ttl_sweep_expires_old_advisory() {
        let mut arbiter = OverDomainArbiter::default_v1();
        let mut depth_store = SutraDepthStore::new();

        // Enqueue at event 0 (expires at event PENDING_TTL)
        let adv = make_advisory(1, 0, AdvisoryType::OctantCorrection, 0.85);
        arbiter.tick_with_stores(0, &[adv], &mut depth_store);
        assert_eq!(arbiter.pending.len(), 1);

        // Tick at event PENDING_TTL → should expire
        arbiter.tick_with_stores(PENDING_TTL, &[], &mut depth_store);
        assert_eq!(arbiter.pending.len(), 0, "advisory should be expired");

        // Log should contain Expired entry
        let expired = arbiter.log().iter()
            .filter(|e| e.outcome == ArbiterOutcome::Expired)
            .count();
        assert_eq!(expired, 1);
    }

    #[test]
    fn test_ttl_sweep_not_premature() {
        let mut arbiter = OverDomainArbiter::default_v1();
        let mut depth_store = SutraDepthStore::new();

        let adv = make_advisory(1, 0, AdvisoryType::OctantCorrection, 0.85);
        arbiter.tick_with_stores(0, &[adv], &mut depth_store);

        // One tick before expiry — should still be pending
        arbiter.tick_with_stores(PENDING_TTL - 1, &[], &mut depth_store);
        assert_eq!(arbiter.pending.len(), 1, "advisory should not expire before TTL");
    }

    #[test]
    fn test_expires_at_event_set_correctly() {
        let mut arbiter = OverDomainArbiter::default_v1();
        let mut depth_store = SutraDepthStore::new();

        let adv = make_advisory(42, 0, AdvisoryType::OctantCorrection, 0.85);
        arbiter.tick_with_stores(500, &[adv], &mut depth_store);

        let p = arbiter.pending.front().unwrap();
        assert_eq!(p.expires_at_event, 500 + PENDING_TTL);
    }
}
