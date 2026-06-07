// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AxialEvaluator — пятый над-доменный модуль.
// Оценивает Frame по трём философским осям (X/Y/Z) на 8 уровнях абстракции.
// Результаты хранятся в EvaluatorStorage (AxialStore).
//
// Источник: docs/architecture/AxialEvaluator_V3_0.md

use std::collections::HashSet;
use std::sync::Arc;

use axiom_core::{Token, STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_domain::AshtiCore;
use axiom_experience::{AxialEvaluation, AxialScore, SubsystemId};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::UclCommand;

use crate::over_domain::arbiter::source::{
    Advisory, AdvisoryAction, AdvisoryId, AdvisoryOutcome, AdvisoryType, SourceId,
};
use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod conflict;
pub mod levels;
pub mod metrics;
pub mod narrative;
pub mod persistence;
pub mod stability;
pub mod storage;
pub mod synthesis;

pub use storage::EvaluatorStorage;
use narrative::NarrativeOctantTracker;
use persistence::ConflictPersistenceTracker;
use stability::OctantStabilityTracker;

/// Интервал срабатывания: каждые 5 тиков.
pub const AXIAL_EVALUATOR_TICK_INTERVAL: u32 = 5;

/// source_id для OverDomainArbiter (AxialEvaluator — второй источник, id=1).
pub const AXIAL_EVALUATOR_SOURCE_ID: SourceId = 1;

/// Кодирование advisory ID: (sutra_id << 8) | type_byte.
/// type_byte: 0x01 = OctantCorrection, 0x02 = ConflictDiagnosis.
/// NarrativeShift использует простой счётчик (subject_id = 0).
const AE_TYPE_OCTANT_CORRECTION: u64 = 0x01;
const AE_TYPE_CONFLICT_DIAGNOSIS: u64 = 0x02;

/// AxialEvaluator — над-доменный оценщик философских осей.
pub struct AxialEvaluator {
    storage: EvaluatorStorage,
    /// sutra_id Frame-анкеров, уже оценённых (не оцениваем повторно без события).
    evaluated_frames: HashSet<u32>,
    stability_tracker: OctantStabilityTracker,
    conflict_tracker: ConflictPersistenceTracker,
    /// V3: скользящее окно нарративного октанта сессии.
    narrative_tracker: NarrativeOctantTracker,
    pending_advisories: Vec<Advisory>,
    /// Доминирующая подсистема из ContextRecognizer; синхронизируется через sync_primary_subsystem.
    primary_subsystem: Option<SubsystemId>,
    /// Счётчик для NarrativeShift advisory ID (не кодируется через sutra_id).
    next_narrative_id: u64,
}

impl AxialEvaluator {
    pub fn new() -> Self {
        Self {
            storage: EvaluatorStorage::new(),
            evaluated_frames: HashSet::new(),
            stability_tracker: OctantStabilityTracker::new(),
            conflict_tracker: ConflictPersistenceTracker::new(),
            narrative_tracker: NarrativeOctantTracker::new(),
            pending_advisories: Vec::new(),
            primary_subsystem: None,
            next_narrative_id: 0,
        }
    }

    pub fn storage(&self) -> &EvaluatorStorage {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut EvaluatorStorage {
        &mut self.storage
    }

    /// Пометить Frame как требующий переоценки (например, после реактивации).
    /// V3: также сбрасывает advisory octant override для этого Frame.
    pub fn invalidate(&mut self, sutra_id: u32) {
        self.evaluated_frames.remove(&sutra_id);
        self.storage.clear_override(sutra_id);
    }

    /// V2: обновить доминирующую подсистему перед следующим on_tick.
    pub fn sync_primary_subsystem(&mut self, s: Option<SubsystemId>) {
        self.primary_subsystem = s;
    }

    /// V3: обратная связь от Arbiter — маршрутизировать в stability_tracker.
    pub fn on_feedback(&mut self, id: AdvisoryId, outcome: AdvisoryOutcome) {
        let type_byte = id & 0xFF;
        if type_byte == AE_TYPE_OCTANT_CORRECTION {
            let sutra_id = (id >> 8) as u32;
            let accepted = matches!(outcome, AdvisoryOutcome::Confirmed | AdvisoryOutcome::Applied);
            self.stability_tracker.on_feedback(sutra_id, accepted);
        }
        // ConflictDiagnosis и NarrativeShift: no-op
    }

    /// V2: забрать накопленные рекомендации (очищает буфер).
    pub fn drain_pending_advisories(&mut self) -> Vec<Advisory> {
        std::mem::take(&mut self.pending_advisories)
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

        // Axis scores: аналитические метрики если есть участники, иначе позиционный фallback.
        // С < 2 участниками entropy=density=will=0 → всегда FormalDenying (O7) — вырождение.
        // Позиционный фallback использует semantic position анкера (вычислена TextPerceptor
        // из якорных матчей), что корректно распределяет инъекции по всем октантам.
        let (x_score, y_score, z_score) = if participants.len() >= 2 {
            let entropy = metrics::entropy_score(&positions);
            let apollo = 255u8.saturating_sub(entropy);

            let density = metrics::graph_density(&participant_ids, all_connections);
            let (pos_val, neg_val) = metrics::valence_score(participants);

            // Y axis: при density=0 и valence=0 (частый случай) используем среднюю Y-позицию
            // участников как Eros/Thanatos сигнал (по спеке Domain V1.3: Y+ = Eros, Y- = Thanatos).
            // Исправляет проблему OBS-AX-01: thanatos=255-density=255 всегда → Y всегда Thanatos.
            let (eros, thanatos) = if density > 0 || pos_val > 0 || neg_val > 0 {
                let e = density.saturating_add(pos_val).min(255);
                let t = (255u8.saturating_sub(density)).saturating_add(neg_val).min(255);
                (e, t)
            } else {
                let mean_y = positions.iter().map(|p| p[1] as f32).sum::<f32>()
                    / positions.len() as f32;
                let pos_eros    = (mean_y.max(0.0) * 255.0 / 32767.0) as u8;
                let pos_thanatos = ((-mean_y).max(0.0) * 255.0 / 30000.0) as u8;
                (pos_eros, pos_thanatos)
            };

            let will = metrics::will_score(participants);
            let nothing = 255u8.saturating_sub(will);
            (
                AxialScore::new(apollo, entropy),
                AxialScore::new(eros, thanatos),
                AxialScore::new(will, nothing),
            )
        } else {
            synthesis::axis_scores_from_position(anchor.position)
        };

        // Синтетический октант через центр масс позиций
        let synthetic_octant = synthesis::synthesize_octant(participants, anchor);

        // V3: advisory override заменяет вычисленный analytic_octant для advisory-логики.
        // X/Y/Z оценки всё равно пересчитываются; override влияет только на stability_tracker.
        let octant_override = self.storage.get_override(anchor.sutra_id);

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

        // OctantStabilityTracker: если есть advisory override — используем его.
        let effective_octant = octant_override.or(last_analytic_octant);

        // OctantCorrection advisory (V2/V3)
        if let Some(octant) = effective_octant {
            if let Some((stable_octant, confidence)) =
                self.stability_tracker.push(anchor.sutra_id, octant)
            {
                let id = (anchor.sutra_id as u64) << 8 | AE_TYPE_OCTANT_CORRECTION;
                self.pending_advisories.push(Advisory {
                    id,
                    source: AXIAL_EVALUATOR_SOURCE_ID,
                    advisory_type: AdvisoryType::OctantCorrection,
                    subject_id: anchor.sutra_id,
                    confidence,
                    action: AdvisoryAction::OverrideOctant {
                        sutra_id: anchor.sutra_id,
                        octant_idx: stable_octant.index(),
                    },
                    created_at_event: event_id,
                    octant_hint: Some(stable_octant.index()),
                });
            }
        }

        // ConflictDiagnosis advisory (V2)
        if let Some(pc) = self.conflict_tracker.push(anchor.sutra_id, last_conflict.as_ref()) {
            let id = (anchor.sutra_id as u64) << 8 | AE_TYPE_CONFLICT_DIAGNOSIS;
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

        // V3: NarrativeOctantTracker → NarrativeShift advisory
        if let Some(octant) = last_analytic_octant {
            if let Some((narrative_octant, confidence)) = self.narrative_tracker.push(octant) {
                let id = self.next_narrative_id;
                self.next_narrative_id += 1;
                self.pending_advisories.push(Advisory {
                    id,
                    source: AXIAL_EVALUATOR_SOURCE_ID,
                    advisory_type: AdvisoryType::NarrativeShift,
                    subject_id: 0,
                    confidence,
                    action: AdvisoryAction::NotifyWorkstation {
                        label: format!("narrative → {:?} ({:.2})", narrative_octant, confidence),
                    },
                    created_at_event: event_id,
                    octant_hint: Some(narrative_octant.index()),
                });
            }
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

    #[test]
    fn test_y_axis_eros_for_high_y_participants() {
        // OBS-AX-01: Y-ось должна быть Eros для участников с высокой Y-позицией.
        // Ранее thanatos=255-density=255 всегда → Y всегда Thanatos.
        // Теперь при density=0 и valence=0 используется mean_y позиции.
        use axiom_experience::Octant;
        let mut evaluator = AxialEvaluator::new();
        // Участники с высоким Y (≥ 5000) → pos_eros > 30 → LeaningPositive → Eros (Y+)
        let anchor = make_anchor(100, [20000, 15000, 20000]);
        let participants = vec![
            make_participant(101, [20000, 14000, 20000]),
            make_participant(102, [20000, 16000, 20000]),
        ];
        evaluator.evaluate_frame(&anchor, &participants, &[], 1);
        let evals = evaluator.storage.store().get_all(100);
        assert!(!evals.is_empty());
        // Все вычисленные октанты должны иметь Eros (Y+)
        for eval in evals {
            let is_eros_octant = matches!(
                eval.octant,
                Octant::CreativeAffirmation    // (+,+,+)
                | Octant::IdealizedConsoling   // (+,+,-)
                | Octant::EcstaticAffirmation  // (-,+,+)
                | Octant::PassiveSentimental   // (-,+,-)
            );
            assert!(
                is_eros_octant,
                "Y=14000-16000 должен давать Eros-октант, got {:?}",
                eval.octant
            );
        }
    }

    #[test]
    fn test_y_axis_thanatos_for_low_y_participants() {
        // Участники с Y ≈ 0 → pos_eros=0, pos_thanatos=0 → Balanced → NOT Eros → Thanatos.
        use axiom_experience::Octant;
        let mut evaluator = AxialEvaluator::new();
        let anchor = make_anchor(200, [20000, 500, 20000]);
        let participants = vec![
            make_participant(201, [20000, 300, 20000]),
            make_participant(202, [20000, 700, 20000]),
        ];
        evaluator.evaluate_frame(&anchor, &participants, &[], 2);
        let evals = evaluator.storage.store().get_all(200);
        assert!(!evals.is_empty());
        // Y ≈ 500 → pos_eros ≈ 3 → diff=3 → Balanced → NOT positive → Thanatos-октанты
        for eval in evals {
            let is_thanatos_octant = matches!(
                eval.octant,
                Octant::HeroicFatal            // (+,-,+)
                | Octant::FormalDenying        // (+,-,-)
                | Octant::DestructiveActivating // (-,-,+)
                | Octant::SelfDestructiveApathic // (-,-,-)
            );
            assert!(
                is_thanatos_octant,
                "Y=300-700 должен давать Thanatos-октант, got {:?}",
                eval.octant
            );
        }
    }
}
