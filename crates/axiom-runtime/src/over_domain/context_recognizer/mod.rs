// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ContextRecognizer — четвёртый над-доменный модуль.
// Анализирует MAYA и EXPERIENCE, определяет активные подсистемы и строит
// InterpretationProfile для Frame-анкеров.
//
// Источник: docs/architecture/ContextRecognizer_V5_0.md

use std::collections::HashMap;
use std::sync::Arc;

use axiom_config::AnchorSet;
use axiom_core::{Token, STATE_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_domain::AshtiCore;
use axiom_experience::{
    AxialStore, EmergentPrimitiveStore, FrameComposition, InterpretationProfileStore, MetaStore,
    Octant, SubsystemId, SutraDepthEntry, SutraDepthStore,
};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::UclCommand;

use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod activity_trace;
pub mod axial_bridge;
pub mod composite;
pub mod conflicts;
pub mod depth_bridge;
pub mod emergent;
pub mod energy;
pub mod hot_reload;
pub mod learning;
pub mod meta_detector;
pub mod profile;
pub mod scanner;
pub mod scanning_plan;
pub mod subsystem_fatigue;
pub mod transitions;

pub use activity_trace::{ActivityDynamics, ActivitySignature, ActivityTrace};
pub use composite::{CompositeActivationSuspected, CompositeSubsystemDef, COMPOSITE_DEFS};
pub use conflicts::SubsystemConflict;
pub use energy::SubsystemEnergy;
pub use meta_detector::{MetaDetector, MetaPrimitive, SubsystemActivationPattern};
pub use scanning_plan::{DepthRange, FractalLevel, ScanningPlan};
pub use subsystem_fatigue::{FatigueStore, SubsystemFatigue};
pub use transitions::{ActivityAnalyzer, SubsystemTransition, TransitionDetector};

/// MAYA domain: role 10 → level_id * 100 + 10.
const MAYA_ROLE: u16 = 10;
/// Интервал срабатывания: каждые 7 тиков.
pub const CONTEXT_RECOGNIZER_TICK_INTERVAL: u32 = 7;
/// Окно событий для поиска активных октантов из AxialStore.
const AXIAL_WINDOW: u64 = 100;
/// Порог конфликта подсистем: secondary.energy / primary.energy >= threshold.
const SUBSYSTEM_CONFLICT_THRESHOLD: f32 = 0.75;

/// ContextRecognizer — над-доменный классификатор семантического контекста.
pub struct ContextRecognizer {
    /// Опорные позиции примитивов каждой подсистемы (для расчёта энергии).
    subsystem_refs: HashMap<SubsystemId, Vec<[i16; 3]>>,
    /// Глубины Frame по октантам (обновляет только DREAM Phase).
    depth_store: SutraDepthStore,
    /// InterpretationProfile для каждого активного Frame-анкера.
    profile_store: InterpretationProfileStore,
    /// Эмерджентные примитивы (V1: stub, approve через UCL).
    emergent_store: EmergentPrimitiveStore,
    /// Лёгкий анализатор переключений (CR-V6: переименован из TransitionDetector).
    transition_detector: ActivityAnalyzer,
    /// Три кольцевых буфера активности подсистем (CR-V6 Фаза A).
    activity_trace: ActivityTrace,
    /// Последние вычисленные метрики динамики (обновляются на каждом on_tick).
    activity_dynamics: ActivityDynamics,
    /// Усталость подсистем (CR-V6 Фаза B).
    fatigue_store: FatigueStore,
    /// Детектор мета-подсистем (CR-V6 Фаза C).
    meta_detector: MetaDetector,
    /// Активные мета-подсистемы (CR-V6 Фаза C).
    meta_store: MetaStore,
    /// Подозреваемые composite co-activations (CR-V6 Фаза D).
    composite_suspects: Vec<CompositeActivationSuspected>,
    /// Список известных sutra_id активных Frame-анкеров.
    known_frame_ids: Vec<u32>,
    /// Аккумулятор активаций Frame за текущий Wake-цикл (для DREAM depth update).
    /// Ключ: (sutra_id, Octant), значение: число on_tick вызовов где Frame был активен.
    /// Очищается после каждого apply_dream_update.
    dream_activation_acc: HashMap<(u32, Octant), u32>,
    /// Снапшот AxialStore от AxialEvaluator (обновляется через sync_axial_store).
    axial_store_snapshot: AxialStore,
}

impl ContextRecognizer {
    pub fn new(subsystem_refs: HashMap<SubsystemId, Vec<[i16; 3]>>) -> Self {
        Self {
            subsystem_refs,
            depth_store: SutraDepthStore::new(),
            profile_store: InterpretationProfileStore::new(),
            emergent_store: EmergentPrimitiveStore::new(),
            transition_detector: ActivityAnalyzer::new(),
            activity_trace: ActivityTrace::new(),
            activity_dynamics: ActivityDynamics {
                entropy_gradient: 0.0,
                oscillation_score: 0.0,
                cascade_score: 0.0,
                dominant_persistence: 0.0,
                fill_count: 0,
            },
            fatigue_store: FatigueStore::new(),
            meta_detector: MetaDetector::new(vec![]),
            meta_store: MetaStore::new(),
            composite_suspects: Vec::new(),
            known_frame_ids: Vec::new(),
            dream_activation_acc: HashMap::new(),
            axial_store_snapshot: AxialStore::new(),
        }
    }

    /// Текущие метрики динамики активности (последний on_tick).
    pub fn activity_dynamics(&self) -> &ActivityDynamics {
        &self.activity_dynamics
    }

    /// Текущие лейблы активности (вычисляется из последних dynamics).
    pub fn activity_signatures(&self) -> Vec<ActivitySignature> {
        activity_trace::classify(&self.activity_dynamics)
    }

    /// Доступ к хранилищу усталости (для диагностики и DREAM-интеграции).
    pub fn fatigue_store(&self) -> &FatigueStore {
        &self.fatigue_store
    }

    /// Активные мета-подсистемы (CR-V6 Фаза C).
    pub fn meta_store(&self) -> &MetaStore {
        &self.meta_store
    }

    /// Подозреваемые composite co-activations (CR-V6 Фаза D).
    pub fn composite_suspects(&self) -> &[CompositeActivationSuspected] {
        &self.composite_suspects
    }

    /// Установить MetaDetector с загруженными примитивами (builder).
    pub fn with_meta_detector(mut self, detector: MetaDetector) -> Self {
        self.meta_detector = detector;
        self
    }

    /// Заменить MetaDetector без пересоздания остальных stores.
    pub fn set_meta_detector(&mut self, detector: MetaDetector) {
        self.meta_detector = detector;
    }

    /// Дренировать аккумулятор активаций для DREAM depth update.
    /// Возвращает (sutra_id, octant, count) и очищает аккумулятор.
    pub fn drain_dream_activations(&mut self) -> Vec<(u32, Octant, u32)> {
        self.dream_activation_acc
            .drain()
            .map(|((id, oct), count)| (id, oct, count))
            .collect()
    }

    /// Все известные sutra_id Frame-анкеров (для decay в apply_dream_update).
    pub fn all_known_frame_ids(&self) -> &[u32] {
        &self.known_frame_ids
    }

    /// Построить ContextRecognizer с позициями подсистем из AnchorSet.
    ///
    /// Группирует якоря по имени подсистемы ("writing", "mathematics", ...)
    /// и извлекает их позиции как опорные точки для расчёта SubsystemEnergy.
    /// Подсистемы без якорей в AnchorSet — не добавляются (CR игнорирует их).
    pub fn from_anchor_set(anchors: &AnchorSet) -> Self {
        let subsystem_refs = build_subsystem_refs(anchors);
        Self::new(subsystem_refs)
    }

    /// Синхронизировать снапшот AxialStore с результатами AxialEvaluator.
    ///
    /// Вызывается координатором рантайма после каждого цикла AxialEvaluator.
    pub fn sync_axial_store(&mut self, store: &AxialStore) {
        self.axial_store_snapshot = store.clone();
    }

    pub fn depth_store(&self) -> &SutraDepthStore {
        &self.depth_store
    }

    pub fn depth_store_mut(&mut self) -> &mut SutraDepthStore {
        &mut self.depth_store
    }

    pub fn profile_store(&self) -> &InterpretationProfileStore {
        &self.profile_store
    }

    pub fn emergent_store(&self) -> &EmergentPrimitiveStore {
        &self.emergent_store
    }

    /// Применить обновление глубин (только из DREAM Phase).
    ///
    /// Также применяет частичное восстановление усталости: `activation_load *= 0.35`.
    pub fn apply_dream_update(
        &mut self,
        activations: &[(u32, Octant, u32)],
        all_known_ids: &[u32],
        event_id: u64,
    ) {
        learning::apply_dream_depth_update(
            &mut self.depth_store,
            activations,
            all_known_ids,
            event_id,
        );
        // DREAM wake: частичное восстановление усталости (Фаза B)
        self.fatigue_store.apply_dream_recovery();
    }

    /// Одобрить эмерджентный примитив (через UCL от chrnv).
    pub fn approve_emergent(&mut self, sutra_id: u32) -> bool {
        emergent::approve_emergent(&mut self.emergent_store, sutra_id)
    }

    fn build_depth_cache(&self) -> HashMap<u32, SutraDepthEntry> {
        self.known_frame_ids
            .iter()
            .filter_map(|&id| self.depth_store.get(id).map(|e| (id, e.clone())))
            .collect()
    }
}

fn build_subsystem_refs(anchors: &AnchorSet) -> HashMap<SubsystemId, Vec<[i16; 3]>> {
    let known = [
        SubsystemId::Writing,
        SubsystemId::Mathematics,
        SubsystemId::Music,
        SubsystemId::Time,
        SubsystemId::Logic,
        SubsystemId::Values,
    ];
    let mut refs = HashMap::new();
    for id in known {
        let positions: Vec<[i16; 3]> = anchors
            .get_subsystem(id.name())
            .iter()
            .map(|a| a.position)
            .collect();
        if !positions.is_empty() {
            refs.insert(id, positions);
        }
    }
    refs
}

impl Default for ContextRecognizer {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl OverDomainComponent for ContextRecognizer {
    fn name(&self) -> &'static str {
        "ContextRecognizer"
    }

    fn module_id(&self) -> ModuleId {
        ModuleId::ContextRecognizer
    }

    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError> {
        use axiom_genome::types::{Permission, ResourceId};
        use axiom_genome::GenomeIndex;
        let index = GenomeIndex::build(genome);
        for resource in [
            ResourceId::ExperienceMemory,
            ResourceId::MayaOutput,
            ResourceId::AshtiField,
            ResourceId::SutraTokens,
        ] {
            if !index.check_access(ModuleId::ContextRecognizer, resource, Permission::Read) {
                return Err(OverDomainError::GenomeDenied);
            }
        }
        Ok(())
    }

    fn on_tick_interval(&self) -> u32 {
        CONTEXT_RECOGNIZER_TICK_INTERVAL
    }

    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<Vec<UclCommand>, OverDomainError> {
        let level = ashti.level_id();
        let maya_domain_id = level * 100 + MAYA_ROLE;
        let exp_domain_id = level * 100 + 9;

        let maya_state = match ashti.index_of(maya_domain_id).and_then(|i| ashti.state(i)) {
            Some(s) => s,
            None => return Ok(vec![]),
        };
        let exp_state = match ashti.index_of(exp_domain_id).and_then(|i| ashti.state(i)) {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        // Обновить список Frame-анкеров
        self.known_frame_ids = exp_state
            .tokens
            .iter()
            .filter(|t| {
                (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0 && t.state == STATE_ACTIVE
            })
            .map(|t| t.sutra_id)
            .collect();

        // Клонируем список для безопасного использования ниже
        let frame_ids: Vec<u32> = self.known_frame_ids.clone();

        // Активные октанты из снапшота AxialStore
        let active_octants = axial_bridge::current_active_octants_for(
            &self.axial_store_snapshot,
            &frame_ids,
            tick,
            AXIAL_WINDOW,
        );

        // Warm start: если окно пусто но история есть — использовать все известные октанты.
        // Иначе (истинный холодный старт) — fallback на CreativeAffirmation.
        let effective_octants = if active_octants.is_empty() {
            axial_bridge::all_octants_in_store(&self.axial_store_snapshot, &frame_ids)
        } else {
            active_octants
        };

        let plan = if effective_octants.is_empty() {
            ScanningPlan::empty(tick).with_surface_region(Octant::CreativeAffirmation)
        } else {
            ScanningPlan::from_octants(&effective_octants, tick)
        };

        let depth_cache = self.build_depth_cache();

        // Сканировать MAYA по всем регионам плана
        let all_tokens: Vec<Token> = plan
            .active_regions
            .iter()
            .flat_map(|region| scanner::scan_region(maya_state, region, &depth_cache).tokens)
            .collect();

        // Вычислить энергии подсистем
        let energies = energy::compute_energies(&all_tokens, &self.subsystem_refs);
        let dominant = energy::dominant_subsystem(&energies);
        let weights = energy::energies_to_weights(&energies);

        // Первичный октант: наиболее активный из плана, либо fallback
        let primary_octant = effective_octants
            .first()
            .copied()
            .unwrap_or(Octant::CreativeAffirmation);

        // Детектировать переключение подсистемы
        let _transition = self.transition_detector.update(dominant, tick);

        // Обновить ActivityTrace (CR-V6 Фаза A)
        self.activity_trace.push(dominant, tick);
        self.activity_dynamics = self.activity_trace.compute_dynamics();

        // Обновить усталость подсистем (CR-V6 Фаза B)
        self.fatigue_store.update(dominant);

        // Детектировать мета-подсистемы (CR-V6 Фаза C)
        let signatures = activity_trace::classify(&self.activity_dynamics);
        self.meta_detector.detect(
            &self.activity_dynamics,
            &signatures,
            dominant,
            tick,
            &mut self.meta_store,
        );

        // Детектировать composite co-activations (CR-V6 Фаза D)
        let recent_subs = self.activity_trace.unique_subsystems_in_mid();
        self.composite_suspects =
            composite::detect_composite_suspects(&recent_subs, &signatures);

        // Применить усталость к весам
        let mut fatigued_weights = weights.clone();
        self.fatigue_store.apply_to_weights(&mut fatigued_weights);

        // Детектировать конфликт подсистем (V1: не записываем, только детектируем)
        let _conflict = conflicts::detect_conflict(&energies, SUBSYSTEM_CONFLICT_THRESHOLD);

        // Попытка детектировать эмерджентные примитивы (V1: no-op, всегда false)
        for &frame_id in &frame_ids {
            emergent::try_detect_emergent(
                &mut self.emergent_store,
                frame_id,
                primary_octant,
                tick,
            );
        }

        // Построить снапшот контекста и обновить InterpretationProfile (с учётом усталости)
        let snapshot = profile::build_snapshot(dominant, primary_octant, tick);
        for &frame_id in &frame_ids {
            profile::upsert_profile(
                &mut self.profile_store,
                frame_id,
                fatigued_weights.clone(),
                dominant,
                primary_octant,
                FrameComposition::C1Atom,
                snapshot.clone(),
            );
            // Аккумулировать активацию для DREAM depth update
            *self.dream_activation_acc.entry((frame_id, primary_octant)).or_insert(0) += 1;
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

    #[test]
    fn test_new_empty_stores() {
        let cr = ContextRecognizer::new(HashMap::new());
        assert_eq!(cr.profile_store().len(), 0);
        assert_eq!(cr.emergent_store().len(), 0);
        assert_eq!(cr.transition_detector.current(), SubsystemId::Unknown);
    }

    #[test]
    fn test_sync_axial_store_no_panic() {
        let mut cr = ContextRecognizer::default();
        cr.sync_axial_store(&AxialStore::new());
    }

    #[test]
    fn test_approve_emergent_unknown_returns_false() {
        let mut cr = ContextRecognizer::default();
        assert!(!cr.approve_emergent(42));
    }

    #[test]
    fn test_apply_dream_update_no_panic() {
        let mut cr = ContextRecognizer::default();
        cr.apply_dream_update(&[], &[], 0);
    }

    #[test]
    fn test_tick_interval() {
        let cr = ContextRecognizer::default();
        assert_eq!(cr.on_tick_interval(), CONTEXT_RECOGNIZER_TICK_INTERVAL);
        assert_eq!(CONTEXT_RECOGNIZER_TICK_INTERVAL, 7);
    }

    #[test]
    fn test_depth_store_empty() {
        let cr = ContextRecognizer::default();
        assert!(cr.depth_store().get(1).is_none());
    }
}
