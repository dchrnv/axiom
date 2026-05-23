// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// OverDomainArbiter — слушатель advisory-источников.
// Читает рекомендации, решает по TrustConfig, действует через SutraDepthStore / UCL.
//
// Источник: docs/architecture/OverDomainArbiter_V1_0.md

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

/// Рекомендация в очереди на подтверждение chrnv.
#[derive(Debug, Clone)]
pub struct PendingAdvisory {
    pub advisory: Advisory,
    pub queued_at_event: u64,
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
        Ok(())
    }

    fn on_tick(&mut self, _tick: u64, _ashti: &AshtiCore) -> Result<Vec<UclCommand>, OverDomainError> {
        // Реальная логика в tick_with_stores — engine.rs вызывает напрямую.
        Ok(Vec::new())
    }

    fn on_shutdown(&mut self) -> Vec<UclCommand> { Vec::new() }
}
