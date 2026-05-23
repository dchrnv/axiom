// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxialEvaluator — пятый над-доменный модуль.
// Оценивает Frame по трём философским осям (X/Y/Z) на 8 уровнях абстракции.
// Результаты хранятся в EvaluatorStorage (AxialStore).
//
// Источник: docs/architecture/AxialEvaluator_V2_0.md

use std::collections::HashSet;
use std::sync::Arc;

use axiom_core::{Token, STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_domain::AshtiCore;
use axiom_experience::{AxialEvaluation, AxialScore, SubsystemId};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::UclCommand;

use crate::over_domain::arbiter::source::{Advisory, AdvisoryAction, AdvisoryType, SourceId};
use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod conflict;
pub mod levels;
pub mod metrics;
pub mod persistence;
pub mod stability;
pub mod storage;
pub mod synthesis;

pub use storage::EvaluatorStorage;
use persistence::ConflictPersistenceTracker;
use stability::OctantStabilityTracker;

/// Интервал срабатывания: каждые 5 тиков.
pub const AXIAL_EVALUATOR_TICK_INTERVAL: u32 = 5;

/// source_id для OverDomainArbiter (AxialEvaluator — второй источник, id=1).
pub const AXIAL_EVALUATOR_SOURCE_ID: SourceId = 1;

/// AxialEvaluator — над-доменный оценщик философских осей.
pub struct AxialEvaluator {
    storage: EvaluatorStorage,
    /// sutra_id Frame-анкеров, уже оценённых (не оцениваем повторно без события).
    evaluated_frames: HashSet<u32>,
    stability_tracker: OctantStabilityTracker,
    conflict_tracker: ConflictPersistenceTracker,
    pending_advisories: Vec<Advisory>,
    /// Доминирующая подсистема из ContextRecognizer; синхронизируется через sync_primary_subsystem.
    primary_subsystem: Option<SubsystemId>,
    next_advisory_id: u64,
}

impl AxialEvaluator {
    pub fn new() -> Self {
        Self {
            storage: EvaluatorStorage::new(),
            evaluated_frames: HashSet::new(),
            stability_tracker: OctantStabilityTracker::new(),
            conflict_tracker: ConflictPersistenceTracker::new(),
            pending_advisories: Vec::new(),
            primary_subsystem: None,
            next_advisory_id: 0,
        }
    }

    pub fn storage(&self) -> &EvaluatorStorage {
        &self.storage
    }

    /// Пометить Frame как требующий переоценки (например, после реактивации).
    pub fn invalidate(&mut self, sutra_id: u32) {
        self.evaluated_frames.remove(&sutra_id);
    }

    /// V2: обновить доминирующую подсистему перед следующим on_tick.
    pub fn sync_primary_subsystem(&mut self, s: Option<SubsystemId>) {
        self.primary_subsystem = s;
    }

    /// V2: забрать накопленные рекомендации (очищает буфер).
    pub fn drain_pending_advisories(&mut self) -> Vec<Advisory> {
        std::mem::take(&mut self.pending_advisories)
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_advisory_id;
        self.next_advisory_id += 1;
        id
    }

    /// Оценить один Frame по всем применимым уровням.
    fn evaluate_frame(
        &mut self,
        anchor: &Token,
        participants: &[Token],
        all_connections: &[axiom_core::Connection],
        event_id: u64,
    ) {
        // Shell-профиль из связей анкера; V2: учитываем доминирующую подсистему
        let shell_profile = levels::build_shell_from_connections(anchor.sutra_id, all_connections);
        let applicable_levels = levels::determine_applicable_levels_with_subsystem(
            &shell_profile,
            self.primary_subsystem,
        );

        // Позиции участников для метрик
        let positions: Vec<[i16; 3]> = participants.iter().map(|t| t.position).collect();
        let participant_ids: Vec<u32> = participants.iter().map(|t| t.sutra_id).collect();

        // X axis: Apollo / Dionysus (энтропия)
        let entropy = metrics::entropy_score(&positions);
        let apollo = 255u8.saturating_sub(entropy);
        let x_score = AxialScore::new(apollo, entropy);

        // Y axis: Eros / Thanatos (связность + валентность)
        let density = metrics::graph_density(&participant_ids, all_connections);
        let (pos_val, neg_val) = metrics::valence_score(participants);
        let eros = density.saturating_add(pos_val).min(255);
        let thanatos = (255u8.saturating_sub(density)).saturating_add(neg_val).min(255);
        let y_score = AxialScore::new(eros, thanatos);

        // Z axis: Will / Nothing (энергетика)
        let will = metrics::will_score(participants);
        let nothing = 255u8.saturating_sub(will);
        let z_score = AxialScore::new(will, nothing);

        // Синтетический октант через центр масс позиций
        let synthetic_octant = synthesis::synthesize_octant(participants, anchor);

        let mut last_analytic_octant = None;
        let mut last_conflict = None;

        for level in applicable_levels {
            let eval = AxialEvaluation::new(
                anchor.sutra_id,
                level,
                x_score,
                y_score,
                z_score,
                event_id,
            );
            let analytic_octant = eval.octant;
            last_analytic_octant = Some(analytic_octant);
            // Conflict detection requires ≥ 2 participants for non-degenerate metrics.
            // With < 2 participants, entropy=0 and density=0 by formula, forcing analytic
            // to FormalDenying regardless of the frame's actual semantic position.
            let eval = if participants.len() >= 2 {
                match conflict::detect_conflict(analytic_octant, synthetic_octant) {
                    Some(c) => {
                        last_conflict = Some(c.clone());
                        eval.with_conflict(c)
                    }
                    None => eval,
                }
            } else {
                eval
            };
            self.storage.record(eval);
        }

        // V2: OctantStabilityTracker → OctantCorrection advisory
        if let Some(octant) = last_analytic_octant {
            if let Some((stable_octant, confidence)) =
                self.stability_tracker.push(anchor.sutra_id, octant)
            {
                let id = self.next_id();
                self.pending_advisories.push(Advisory {
                    id,
                    source: AXIAL_EVALUATOR_SOURCE_ID,
                    advisory_type: AdvisoryType::OctantCorrection,
                    subject_id: anchor.sutra_id,
                    confidence,
                    action: AdvisoryAction::NotifyWorkstation {
                        label: format!(
                            "#{} stable octant → {:?} ({:.2})",
                            anchor.sutra_id, stable_octant, confidence
                        ),
                    },
                    created_at_event: event_id,
                    octant_hint: Some(stable_octant.index()),
                });
            }
        }

        // V2: ConflictPersistenceTracker → ConflictDiagnosis advisory
        if let Some(pc) = self.conflict_tracker.push(anchor.sutra_id, last_conflict.as_ref()) {
            let id = self.next_id();
            self.pending_advisories.push(Advisory {
                id,
                source: AXIAL_EVALUATOR_SOURCE_ID,
                advisory_type: AdvisoryType::ConflictDiagnosis,
                subject_id: anchor.sutra_id,
                confidence: pc.confidence,
                action: AdvisoryAction::NotifyWorkstation {
                    label: format!(
                        "#{} conflict {:?}↔{:?} streak={} ({:.2})",
                        anchor.sutra_id,
                        pc.analytic_octant,
                        pc.synthetic_octant,
                        pc.streak,
                        pc.confidence
                    ),
                },
                created_at_event: event_id,
                octant_hint: None,
            });
        }

        self.evaluated_frames.insert(anchor.sutra_id);
    }
}

impl Default for AxialEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl OverDomainComponent for AxialEvaluator {
    fn name(&self) -> &'static str {
        "AxialEvaluator"
    }

    fn module_id(&self) -> ModuleId {
        ModuleId::AxialEvaluator
    }

    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
        use axiom_genome::types::{Permission, ResourceId};
        use axiom_genome::GenomeIndex;
        let index = GenomeIndex::build(genome);
        if !index.check_access(
            ModuleId::AxialEvaluator,
            ResourceId::ExperienceMemory,
            Permission::Read,
        ) {
            return Err(OverDomainError::GenomeDenied);
        }
        Ok(())
    }

    fn on_tick_interval(&self) -> u32 {
        AXIAL_EVALUATOR_TICK_INTERVAL
    }

    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<Vec<UclCommand>, OverDomainError> {
        let level = ashti.level_id();
        let exp_domain_id = level * 100 + 9;

        let exp_state = match ashti.index_of(exp_domain_id).and_then(|i| ashti.state(i)) {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        // Найти Frame-анкеры ещё не оценённые
        let to_evaluate: Vec<Token> = exp_state
            .tokens
            .iter()
            .filter(|t| {
                (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0
                    && t.state == STATE_ACTIVE
                    && !self.evaluated_frames.contains(&t.sutra_id)
            })
            .cloned()
            .collect();

        for anchor in to_evaluate {
            // Участники: токены, соединённые с анкером
            let participant_ids: Vec<u32> = exp_state
                .connections
                .iter()
                .filter(|c| c.source_id == anchor.sutra_id)
                .map(|c| c.target_id)
                .collect();

            let participants: Vec<Token> = exp_state
                .tokens
                .iter()
                .filter(|t| participant_ids.contains(&t.sutra_id))
                .cloned()
                .collect();

            self.evaluate_frame(&anchor, &participants, &exp_state.connections, tick);
        }

        Ok(vec![])
    }

    fn on_shutdown(&mut self) -> Vec<UclCommand> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_experience::EvaluationLevel;

    fn make_anchor(sutra_id: u32, pos: [i16; 3]) -> Token {
        let mut t = Token::new(sutra_id, 109, pos, 0);
        t.type_flags = TOKEN_FLAG_FRAME_ANCHOR;
        // state = STATE_ACTIVE by default from Token::new
        t
    }

    fn make_participant(sutra_id: u32, pos: [i16; 3]) -> Token {
        Token::new(sutra_id, 109, pos, 0)
    }

    #[test]
    fn test_evaluate_frame_stores_result() {
        let mut evaluator = AxialEvaluator::new();
        let anchor = make_anchor(1, [20000, 20000, 20000]);
        let participants = vec![
            make_participant(2, [19000, 21000, 19000]),
            make_participant(3, [21000, 19000, 21000]),
        ];
        evaluator.evaluate_frame(&anchor, &participants, &[], 100);
        // At least one evaluation stored
        assert!(!evaluator.storage.store().is_empty());
        assert_eq!(evaluator.storage.total_evaluated, 1);
        // Marked as evaluated
        assert!(evaluator.evaluated_frames.contains(&1));
    }

    #[test]
    fn test_invalidate_allows_re_evaluation() {
        let mut evaluator = AxialEvaluator::new();
        let anchor = make_anchor(5, [15000, 15000, 15000]);
        evaluator.evaluate_frame(&anchor, &[], &[], 10);
        assert!(evaluator.evaluated_frames.contains(&5));

        evaluator.invalidate(5);
        assert!(!evaluator.evaluated_frames.contains(&5));
    }

    #[test]
    fn test_frame_not_re_evaluated_without_invalidate() {
        let mut evaluator = AxialEvaluator::new();
        let anchor = make_anchor(7, [10000, 10000, 10000]);
        evaluator.evaluate_frame(&anchor, &[], &[], 1);
        let count_after_first = evaluator.storage.total_evaluated;

        // Same anchor without invalidate — evaluate_frame would still run if called directly,
        // but on_tick skips frames in evaluated_frames set.
        assert!(evaluator.evaluated_frames.contains(&7));
        assert_eq!(count_after_first, 1);
    }

    #[test]
    fn test_empty_participants_fallback_to_conceptual() {
        let mut evaluator = AxialEvaluator::new();
        let anchor = make_anchor(10, [5000, 5000, 5000]);
        // No participants, no connections → shell is all zero → fallback Conceptual
        evaluator.evaluate_frame(&anchor, &[], &[], 1);
        let evals = evaluator.storage.store().get_all(10);
        assert_eq!(evals.len(), 1);
        assert_eq!(evals[0].level, EvaluationLevel::Conceptual);
    }
}
