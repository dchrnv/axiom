// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Sensorium V1.0 — полный внутренний срез системы + выражение.
//
// Спецификация: docs/spec/Sensorium_V1_0.md
//
// Природа: только читает, никогда не управляет (&self на весь доступ к данным).
// Это инвариант GENOME: "Sensorium читает — никогда не управляет. Видит. Говорит. Не трогает."
//
// Архитектурное место: получает SensoriumView (ссылки на все OD-компоненты движка),
// хранит текущий SensoriumState, строит SensoriumExpression.
//
// В V1.0 Sensorium НЕ заменяет TickSnapshot — работает параллельно.
// Полное поглощение TickSnapshot → V2.0 (см. DEFERRED.md SEN-TD-01).

pub mod expression;
pub mod levels;
pub mod registry;
pub mod schedule;
pub mod state;

pub use expression::{express, SensoriumExpression};
pub use levels::{CollectionLevel, FULL_INTERVAL, PULSE_INTERVAL, STATE_INTERVAL};
pub use registry::{ConsumerEntry, ConsumerRegistry};
pub use schedule::SensoriumSchedule;
pub use state::{
    ActiveDilemmaEntry, EmergentEntry, SensoriumDomainSummary, SensoriumDreamSummary,
    SensoriumState, SubsystemActivity,
};

use axiom_genome::types::{ModuleId, Permission, ResourceId};
use axiom_genome::{Genome, GenomeIndex};
use std::sync::Arc;

use crate::over_domain::context_recognizer::DilemmaType;
use crate::over_domain::{
    AxialEvaluator, ContextRecognizer, FrameWeaver, NeuralAdvisor, OverDomainArbiter, Waves,
};
use crate::over_domain::dream_phase::{DreamPhaseState, DreamPhaseStats};

/// Снимок движка для чтения Sensorium'ом.
///
/// Строится в engine.rs из конкретных полей AxiomEngine перед вызовом collect().
/// Borrow-checker-safe аналог `&AxiomEngine`: компилятор видит что sensorium
/// заимствует отдельные поля, а не весь self.
pub struct SensoriumView<'a> {
    pub tick: u64,
    pub causal_time: u64,
    pub dream_phase: DreamPhaseState,
    pub dream_stats: &'a DreamPhaseStats,
    pub context_recognizer: &'a ContextRecognizer,
    pub axial_evaluator: &'a AxialEvaluator,
    pub frame_weaver: &'a FrameWeaver,
    pub over_domain_arbiter: &'a OverDomainArbiter,
    #[allow(dead_code)]
    pub neural_advisor: &'a NeuralAdvisor,
    /// Waves — для чтения impulse-полей (internal_dominance_factor, active_impulses).
    pub waves: &'a Waves,
    // — Добавлено в V2.0 (Фаза A) — поля из BroadcastSnapshot —
    /// Experience.trace_count() — вычислено в engine до построения view.
    pub trace_count: usize,
    /// Experience.tension_count() — вычислено в engine до построения view.
    pub tension_count: usize,
    /// Сводка по 11 доменам — None на Pulse тиках, Some только при State/Full.
    /// §4 спеки: domain info не нужна каждый тик, только каждые 8 тиков.
    pub domain_summaries: Option<Vec<SensoriumDomainSummary>>,
    /// AxiomEngine.last_crystallization_tick.
    pub last_crystallization_tick: u64,
    /// Guardian.stats().vetoes_since_wake.
    pub guardian_vetoes_since_wake: u64,
    /// Последний завершённый dream-цикл (None до первого сна).
    pub last_dream_summary: Option<SensoriumDreamSummary>,
}

/// Sensorium — единая точка доступа к внутреннему срезу системы.
///
/// Хранит последний собранный SensoriumState и SensoriumExpression.
/// Тикает в конце каждого тика движка (после всех OD-компонентов).
#[derive(Debug)]
pub struct Sensorium {
    /// Последний собранный срез (None до первого collect).
    pub current_state: Option<SensoriumState>,
    /// Выражение из последнего среза.
    pub current_expression: Option<SensoriumExpression>,
    /// Расписание сборки (большой цикл). pub для peek_level в engine.rs.
    pub schedule: SensoriumSchedule,
    /// Реестр потребителей.
    registry: ConsumerRegistry,
    /// Число вызовов collect() за всё время.
    pub collect_count: u64,
}

impl Default for Sensorium {
    fn default() -> Self {
        Self {
            current_state: None,
            current_expression: None,
            schedule: SensoriumSchedule::new(),
            registry: ConsumerRegistry::v1_defaults(),
            collect_count: 0,
        }
    }
}

impl Sensorium {
    pub fn new() -> Self {
        Self::default()
    }

    /// Проверить права доступа из GENOME при boot.
    pub fn on_boot(&self, genome: &Arc<Genome>) -> Result<(), String> {
        let index = GenomeIndex::build(genome);
        let checks = [
            (ResourceId::ExperienceMemory, Permission::Read),
            (ResourceId::AshtiField, Permission::Read),
            (ResourceId::MayaOutput, Permission::Read),
            (ResourceId::SutraTokens, Permission::Read),
        ];
        for (resource, required) in checks {
            if !index.check_access(ModuleId::Sensorium, resource, required) {
                return Err(format!(
                    "Sensorium: GENOME denied {resource:?}/{required:?}"
                ));
            }
        }
        Ok(())
    }

    /// Собрать срез из текущего состояния движка.
    ///
    /// Вызывается в конце каждого тика engine (после всех OD-компонентов).
    /// Уровень детерминирован по номеру тика (§4 спеки).
    /// Деградация §12: в DREAMING собирается только Pulse.
    pub fn collect(&mut self, view: &SensoriumView<'_>) {
        let mut level = self.schedule.advance(view.tick);

        // Деградация §12: в DREAMING — только Pulse.
        if view.dream_phase == DreamPhaseState::Dreaming && level > CollectionLevel::Pulse {
            level = CollectionLevel::Pulse;
        }

        // Ограничить по реестру потребителей.
        let max_needed = self.registry.max_required_level();
        if level > max_needed {
            level = max_needed;
        }

        let mut state = Self::build_state(view, level);

        // Carry-over domain_summaries: на Pulse тиках перенести из предыдущего State среза.
        // domain_summaries обновляется каждые 8 тиков (State) — для Pulse достаточно свежий.
        if state.domain_summaries.is_empty() {
            if let Some(prev) = &self.current_state {
                if !prev.domain_summaries.is_empty() {
                    state.domain_summaries = prev.domain_summaries.clone();
                }
            }
        }

        let expression = express(&state);
        self.current_state = Some(state);
        self.current_expression = Some(expression);
        self.collect_count += 1;
    }

    /// Уведомить о пробуждении из DREAM — собрать Memory-уровень при следующем collect.
    pub fn on_dream_wake(&mut self) {
        self.schedule.schedule_memory_collection();
    }

    fn build_state(view: &SensoriumView<'_>, level: CollectionLevel) -> SensoriumState {
        let mut state = SensoriumState {
            collected_at_tick: view.tick,
            causal_time: view.causal_time,
            dream_phase_raw: dream_phase_to_u8(view.dream_phase),
            ..Default::default()
        };

        collect_pulse(view, &mut state);

        if level >= CollectionLevel::State {
            collect_state(view, &mut state);
        }

        if level >= CollectionLevel::Full {
            collect_full(view, &mut state);
        }

        state
    }
}

// — Сборщики по уровням —

fn collect_pulse(view: &SensoriumView<'_>, state: &mut SensoriumState) {
    // Dominant subsystem из InterpretationProfileStore.
    state.dominant_subsystem = view
        .context_recognizer
        .profile_store()
        .dominant_primary();

    // Activity signature: берём первую из списка (наивысший приоритет).
    let sigs = view.context_recognizer.activity_signatures();
    state.activity_signature = sigs
        .first()
        .map(|s| format!("{s:?}"))
        .unwrap_or_else(|| "Uncertain".to_string());

    // Dominant octant из AxialEvaluator storage.
    state.dominant_octant = view
        .axial_evaluator
        .storage()
        .store()
        .most_common_octant();

    // Число активных дилемм и pending crystallizations.
    state.active_dilemma_count = view.context_recognizer.dilemma_store().active_count();
    state.has_pending_crystallization = view
        .context_recognizer
        .dilemma_store()
        .has_pending_crystallizations();

    // Frame candidates.
    state.candidates_count = view.frame_weaver.candidates_count();
    state.avg_shell_similarity = view.frame_weaver.avg_candidate_shell_similarity();

    // Pending advisories.
    state.pending_advisories = view.over_domain_arbiter.pending_snapshot().len();

    // — Внутренний импульс (Waves) —
    state.internal_dominance_factor = view.waves.internal_dominance_factor;
    state.active_impulse_count = view.waves.active_impulses.len();
    state.impulse_sources = view
        .waves
        .active_impulses
        .iter()
        .map(|imp| match imp.source {
            crate::over_domain::waves::ImpulseSource::Dilemma => "Dilemma",
            crate::over_domain::waves::ImpulseSource::Resonance => "Resonance",
            crate::over_domain::waves::ImpulseSource::Unfinished => "Unfinished",
        })
        .collect();

    // — Движок (поля пульса: дешёвые скаляры, без итерации доменов) —
    state.trace_count = view.trace_count;
    state.tension_count = view.tension_count;
    // domain_summaries — State level (каждые 8 тиков), не Pulse. §4 спеки.
    state.last_crystallization_tick = view.last_crystallization_tick;
    state.guardian_vetoes_since_wake = view.guardian_vetoes_since_wake;
    state.cross_modal_candidates = view.context_recognizer.cross_modal_candidate_count();
    state.last_dream_summary = view.last_dream_summary.clone();
}

fn collect_state(view: &SensoriumView<'_>, state: &mut SensoriumState) {
    // Corpus Callosum: есть ли OntologicalConflict среди активных дилемм.
    // Signal C (Corpus Callosum) и Signal B (connection stress) — оба OntologicalConflict.
    state.corpus_callosum_active = view
        .context_recognizer
        .dilemma_store()
        .active
        .iter()
        .any(|d| matches!(d.dilemma_type, DilemmaType::OntologicalConflict));

    // Активные подсистемы через FatigueStore (activation_load > 0 → подсистема была активна).
    let fatigue_store = view.context_recognizer.fatigue_store();
    for (&sid, fatigue) in fatigue_store.iter() {
        let load = fatigue.activation_load;
        if load > 0.1 {
            // Масштабируем load (0..10) в energy (0..255).
            let energy = ((load / 10.0) * 255.0).min(255.0) as u8;
            state.active_subsystems.push(SubsystemActivity {
                id: sid,
                energy,
                fatigue_load: load,
            });
            if load > 5.0 {
                state.fatigued_subsystems.push(sid);
            }
        }
    }

    // Детали активных дилемм.
    for record in &view.context_recognizer.dilemma_store().active {
        state.active_dilemmas.push(ActiveDilemmaEntry {
            id: record.id,
            dilemma_type: record.dilemma_type as u8,
            intensity: record.intensity,
            detected_at_tick: record.detected_at_tick,
        });
    }

    // Composite suspects.
    state.composite_suspect_count = view.context_recognizer.composite_suspects().len();

    // Cross-modal bonds.
    state.cross_modal_bonds = view.context_recognizer.cross_modal_bond_count();

    // Domain summaries — только здесь (State level = каждые 8 тиков).
    // §4 спеки: domain info не нужна каждый тик, только для Workstation обновления.
    if let Some(summaries) = &view.domain_summaries {
        state.domain_summaries = summaries.clone();
    }
}

fn collect_full(view: &SensoriumView<'_>, state: &mut SensoriumState) {
    let emergent_store = view.context_recognizer.emergent_store();
    let depth_store = view.context_recognizer.depth_store();

    for ep in emergent_store.get_all() {
        let depth_entry = depth_store.get(ep.sutra_id);
        let depth_avg = depth_entry.map(|e| e.avg_depth()).unwrap_or(ep.initial_depth);
        let reactivations = depth_entry.map(|e| e.reactivation_count).unwrap_or(0);

        state.emergent_candidates.push(EmergentEntry {
            sutra_id: ep.sutra_id,
            depth_avg,
            reactivations,
        });
    }
}

fn dream_phase_to_u8(phase: DreamPhaseState) -> u8 {
    match phase {
        DreamPhaseState::Wake => 0,
        DreamPhaseState::FallingAsleep => 1,
        DreamPhaseState::Dreaming => 2,
        DreamPhaseState::Waking => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensorium_new_has_no_state() {
        let s = Sensorium::new();
        assert!(s.current_state.is_none());
        assert!(s.current_expression.is_none());
        assert_eq!(s.collect_count, 0);
    }

    #[test]
    fn collection_level_pulse_on_odd_tick() {
        assert_eq!(CollectionLevel::for_tick(1), CollectionLevel::Pulse);
        assert_eq!(CollectionLevel::for_tick(3), CollectionLevel::Pulse);
        assert_eq!(CollectionLevel::for_tick(7), CollectionLevel::Pulse);
    }

    #[test]
    fn collection_level_state_on_8() {
        assert_eq!(CollectionLevel::for_tick(8), CollectionLevel::State);
        assert_eq!(CollectionLevel::for_tick(16), CollectionLevel::State);
        assert_eq!(CollectionLevel::for_tick(24), CollectionLevel::State);
    }

    #[test]
    fn collection_level_full_on_32() {
        assert_eq!(CollectionLevel::for_tick(32), CollectionLevel::Full);
        assert_eq!(CollectionLevel::for_tick(64), CollectionLevel::Full);
        assert_eq!(CollectionLevel::for_tick(0), CollectionLevel::Full);
    }

    #[test]
    fn dream_phase_raw_encoding() {
        assert_eq!(dream_phase_to_u8(DreamPhaseState::Wake), 0);
        assert_eq!(dream_phase_to_u8(DreamPhaseState::FallingAsleep), 1);
        assert_eq!(dream_phase_to_u8(DreamPhaseState::Dreaming), 2);
        assert_eq!(dream_phase_to_u8(DreamPhaseState::Waking), 3);
    }

    #[test]
    fn schedule_advance_memory_after_flag() {
        let mut sched = SensoriumSchedule::new();
        sched.schedule_memory_collection();
        let level = sched.advance(1);
        assert_eq!(level, CollectionLevel::Memory);
        let level2 = sched.advance(1);
        assert_eq!(level2, CollectionLevel::Pulse);
    }

    #[test]
    fn on_boot_passes_with_default_genome() {
        let genome = Genome::default_ashti_core();
        let s = Sensorium::new();
        let arc = Arc::new(genome);
        assert!(s.on_boot(&arc).is_ok());
    }

    #[test]
    fn sensorium_domain_summary_default() {
        let s = SensoriumDomainSummary::default();
        assert_eq!(s.domain_id, 0);
        assert_eq!(s.token_count, 0);
        assert_eq!(s.temperature_avg, 0);
    }

    #[test]
    fn sensorium_state_has_phase_a_fields() {
        let state = SensoriumState::default();
        assert_eq!(state.trace_count, 0);
        assert_eq!(state.tension_count, 0);
        assert!(state.domain_summaries.is_empty());
        assert_eq!(state.last_crystallization_tick, 0);
        assert_eq!(state.guardian_vetoes_since_wake, 0);
        assert_eq!(state.cross_modal_candidates, 0);
        assert!(state.last_dream_summary.is_none());
    }
}
