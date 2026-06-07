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
    Modality, ModalityStore, Octant, SubsystemId, SutraDepthEntry, SutraDepthStore,
};
use energy::SubsystemShellRefs;
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::UclCommand;

use crate::over_domain::traits::{OverDomainComponent, OverDomainError};

pub mod activity_trace;
pub mod axial_bridge;
pub mod cross_modal;
pub mod composite;
pub mod conflicts;
pub mod depth_bridge;
pub mod dilemma;
pub mod dilemma_store;
pub mod emergent;
pub mod energy;
pub mod hot_reload;
pub mod learning;
pub mod meta_detector;
pub mod moral_signal;
pub mod profile;
pub mod scanner;
pub mod scanning_plan;
pub mod subsystem_fatigue;
pub mod transitions;
pub mod split_merge;
pub mod version_store;

pub use activity_trace::{ActivityDynamics, ActivitySignature, ActivityTrace};
pub use composite::{
    BidirectionalCoupling, CompositeActivationSuspected, CompositeSubsystemDef,
    CompositeSubsystemProfile, BIDIRECTIONAL_COUPLING_THRESHOLD, COMPOSITE_DEFS,
};
pub use conflicts::SubsystemConflict;
pub use dilemma::DilemmaDetector;
pub use cross_modal::{CrossModalDetector, MIN_CROSS_MODAL_COACTIVATION};
pub use dilemma_store::{
    crystallize_to_experience_commands, DilemmaRecord, DilemmaResolution, DilemmaStore, DilemmaType,
};
pub use energy::SubsystemEnergy;
pub use meta_detector::{MetaDetector, MetaPrimitive, SubsystemActivationPattern};
pub use scanning_plan::{DepthRange, FractalLevel, ScanningPlan};
pub use subsystem_fatigue::{FatigueStore, SubsystemFatigue};
pub use transitions::{ActivityAnalyzer, SubsystemTransition, TransitionDetector, TransitionMatrix};
pub use split_merge::{
    MergeCandidate, MergeReason, SplitCandidate, SplitMergeCandidateStore, SplitMergeDetector,
    SplitReason, MERGE_THRESHOLD, SPLIT_ENTROPY_THRESHOLD, SPLIT_LOAD_THRESHOLD,
};
pub use version_store::{SubsystemVersionEntry, SubsystemVersionStore};

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
    /// Полные профили composite подсистем с bidirectional coupling (V7-C2).
    composite_profiles: Vec<CompositeSubsystemProfile>,
    /// Список известных sutra_id активных Frame-анкеров.
    known_frame_ids: Vec<u32>,
    /// Аккумулятор активаций Frame за текущий Wake-цикл (для DREAM depth update).
    /// Ключ: (sutra_id, Octant), значение: число on_tick вызовов где Frame был активен.
    /// Очищается после каждого apply_dream_update.
    dream_activation_acc: HashMap<(u32, Octant), u32>,
    /// Снапшот AxialStore от AxialEvaluator (обновляется через sync_axial_store).
    axial_store_snapshot: AxialStore,
    /// Опорные точки + shell-профили якорей каждой подсистемы.
    /// Заполняется при from_anchor_set + set_subsystem_shell_templates.
    /// Если не пусто — on_tick использует compute_energies_with_shell.
    subsystem_shell_refs: SubsystemShellRefs,
    /// Средний shell-профиль каждой подсистемы (синхронизируется из engine после boot).
    subsystem_shell_templates: HashMap<SubsystemId, [u8; 8]>,
    /// Матрица переходов между подсистемами (V7-B1). Decay на каждом on_tick.
    transition_matrix: TransitionMatrix,
    /// Версии подсистем для migration trace (V7-D1).
    version_store: SubsystemVersionStore,
    /// Split/Merge кандидаты (V7-D2). Обновляются в DREAM-фазе on_tick.
    split_merge_candidates: SplitMergeCandidateStore,
    /// Хранилище активных дилемм (DilemmaDetector V2.0).
    pub dilemma_store: DilemmaStore,
    /// Детектор конфликтов подсистем (DilemmaDetector V2.0).
    dilemma_detector: DilemmaDetector,
    /// Модальности Frame-анкеров (Cross_Modal_Binding_V1_0 §2).
    pub modality_store: ModalityStore,
    /// Детектор cross-modal ко-активации (Cross_Modal_Binding_V1_0 §3).
    cross_modal_detector: CrossModalDetector,
    /// Разрешённые дилеммы ожидающие создания TensionTrace в Experience.
    /// Дрейнится engine.rs после on_tick → add_tension_trace(temperature=255).
    pending_resolution_tensions: Vec<u64>,
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
                directed_cascade_score: 0.0,
                dominant_persistence: 0.0,
                fill_count: 0,
            },
            fatigue_store: FatigueStore::new(),
            meta_detector: MetaDetector::new(vec![]),
            meta_store: MetaStore::new(),
            composite_suspects: Vec::new(),
            composite_profiles: Vec::new(),
            known_frame_ids: Vec::new(),
            dream_activation_acc: HashMap::new(),
            axial_store_snapshot: AxialStore::new(),
            subsystem_shell_refs: HashMap::new(),
            subsystem_shell_templates: HashMap::new(),
            transition_matrix: TransitionMatrix::new(),
            version_store: SubsystemVersionStore::new(),
            split_merge_candidates: SplitMergeCandidateStore::new(),
            dilemma_store: DilemmaStore::new(),
            dilemma_detector: DilemmaDetector::new(),
            modality_store: ModalityStore::new(),
            cross_modal_detector: CrossModalDetector::new(),
            pending_resolution_tensions: Vec::new(),
        }
    }

    pub fn pending_resolution_tensions_len(&self) -> usize {
        self.pending_resolution_tensions.len()
    }

    /// Дрейнить pending TensionTrace от разрешённых дилемм.
    ///
    /// Возвращает tick-метки разрешений. Engine создаёт TensionTrace с temperature=255
    /// для каждого элемента и добавляет в Experience.
    pub fn drain_resolution_tensions(&mut self) -> Vec<u64> {
        std::mem::take(&mut self.pending_resolution_tensions)
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

    /// Матрица переходов между подсистемами (V7-B1).
    pub fn transition_matrix(&self) -> &TransitionMatrix {
        &self.transition_matrix
    }

    /// Хранилище версий подсистем (V7-D1).
    pub fn version_store(&self) -> &SubsystemVersionStore {
        &self.version_store
    }

    /// Split/Merge кандидаты (V7-D2).
    pub fn split_merge_candidates(&self) -> &SplitMergeCandidateStore {
        &self.split_merge_candidates
    }

    /// Активные дилеммы (DilemmaDetector V2.0).
    pub fn dilemma_store(&self) -> &DilemmaStore {
        &self.dilemma_store
    }

    /// Зарегистрировать модальность Frame-анкера.
    ///
    /// Вызывается из engine при обработке InjectFrameAnchorPayload.
    pub fn register_frame_modality(&mut self, sutra_id: u32, modality: Modality) {
        self.modality_store.insert(sutra_id, modality);
    }

    /// Число кандидатов cross-modal ко-активации.
    pub fn cross_modal_candidate_count(&self) -> usize {
        self.cross_modal_detector.candidate_count()
    }

    /// Число cross-modal bond, ожидающих DREAM Phase.
    pub fn cross_modal_pending_count(&self) -> usize {
        self.cross_modal_detector.pending_count()
    }

    /// Число уже созданных cross-modal bond.
    pub fn cross_modal_bond_count(&self) -> usize {
        self.cross_modal_detector.bond_count()
    }

    /// Дрейнировать UCL-команды для cross-modal bond (вызывается после DREAM-цикла).
    pub fn drain_cross_modal_bond_commands(&mut self, exp_domain_id: u16) -> Vec<axiom_ucl::UclCommand> {
        self.cross_modal_detector.drain_pending_bond_commands(exp_domain_id)
    }

    /// Передать граф зависимостей подсистем детектору дилемм.
    pub fn set_subsystem_dependencies(&mut self, deps: axiom_config::SubsystemDependencies) {
        self.dilemma_detector.set_dependencies(deps);
    }

    /// Инициализировать версии подсистем из AnchorSet (первичная загрузка).
    pub fn init_subsystem_versions(&mut self, versions: &std::collections::HashMap<String, String>) {
        self.version_store.init(versions);
    }

    /// Проверить и применить обновление версий (hot-reload). Возвращает stale-подсистемы.
    pub fn update_subsystem_versions(&mut self, versions: &std::collections::HashMap<String, String>) -> Vec<String> {
        self.version_store.check_migration(versions)
    }

    /// Активные мета-подсистемы (CR-V6 Фаза C).
    pub fn meta_store(&self) -> &MetaStore {
        &self.meta_store
    }

    /// Подозреваемые composite co-activations (CR-V6 Фаза D).
    pub fn composite_suspects(&self) -> &[CompositeActivationSuspected] {
        &self.composite_suspects
    }

    /// Полные профили composite подсистем с bidirectional coupling (V7-C2).
    pub fn composite_profiles(&self) -> &[CompositeSubsystemProfile] {
        &self.composite_profiles
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

    /// Снапшот ActivityTrace для персистентности (CR-TD-04).
    pub fn activity_trace_snapshot(&self) -> &ActivityTrace {
        &self.activity_trace
    }

    /// Восстановить ActivityTrace из сохранённого снапшота (CR-TD-04).
    pub fn restore_activity_trace(&mut self, trace: ActivityTrace) {
        self.activity_trace = trace;
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
    /// и извлекает их позиции + shell как опорные точки для расчёта SubsystemEnergy.
    /// Подсистемы без якорей в AnchorSet — не добавляются (CR игнорирует их).
    pub fn from_anchor_set(anchors: &AnchorSet) -> Self {
        let subsystem_refs = build_subsystem_refs(anchors);
        let subsystem_shell_refs = build_subsystem_shell_refs(anchors);
        let mut cr = Self::new(subsystem_refs);
        cr.subsystem_shell_refs = subsystem_shell_refs;
        cr.version_store.init(&anchors.subsystem_versions);
        cr.dilemma_detector.set_dependencies(anchors.subsystem_dependencies.clone());
        cr
    }

    /// Синхронизировать средние shell-профили подсистем из engine (после inject_anchor_tokens).
    pub fn set_subsystem_shell_templates(&mut self, templates: HashMap<SubsystemId, [u8; 8]>) {
        self.subsystem_shell_templates = templates;
    }

    /// Записать прямой сигнал активности подсистемы из позиции входного токена.
    ///
    /// Вызывается из engine::process_command при обработке InjectToken.
    /// Обходит MAYA-накопление: определяет подсистему по позиции (confidence threshold
    /// фильтрует FNV-токены) и напрямую записывает в ActivityTrace и FatigueStore.
    pub fn record_injection_signal(&mut self, position: [i16; 3], event_id: u64) {
        let sub = energy::classify_position(position, &self.subsystem_refs);
        if sub == SubsystemId::Unknown {
            return;
        }
        self.activity_trace.push(sub, event_id);
        let transition = self.transition_detector.update(sub, event_id);
        if let Some(ref t) = transition {
            self.transition_matrix.record(t.from, t.to);
        }
        self.fatigue_store.update(sub);
        self.activity_dynamics = self.activity_trace.compute_dynamics();
        self.dilemma_detector.record_injection(sub, event_id);
    }

    /// Вычислить энергии подсистем прямо сейчас из переданных MAYA-токенов.
    /// Используется для per-text delta-метрики в axiom-observe: до/после инъекции.
    pub fn compute_raw_energies(&self, maya_tokens: &[Token]) -> HashMap<SubsystemId, f32> {
        energy::compute_energies(maya_tokens, &self.subsystem_refs)
            .into_iter()
            .map(|e| (e.subsystem, e.energy))
            .collect()
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
        SubsystemId::Morality,
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

fn build_subsystem_shell_refs(anchors: &AnchorSet) -> SubsystemShellRefs {
    let known = [
        SubsystemId::Writing,
        SubsystemId::Mathematics,
        SubsystemId::Music,
        SubsystemId::Time,
        SubsystemId::Logic,
        SubsystemId::Values,
        SubsystemId::Morality,
    ];
    let mut refs = HashMap::new();
    for id in known {
        let entries: Vec<([i16; 3], [u8; 8])> = anchors
            .get_subsystem(id.name())
            .iter()
            .map(|a| (a.position, a.shell))
            .collect();
        if !entries.is_empty() {
            refs.insert(id, entries);
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

        // Вычислить энергии подсистем.
        //
        // Проблема накопления: MAYA-токены из E1-fix не попадают в frontier и никогда не
        // переходят в STATE_SLEEPING. За долгие прогоны накапливаются тысячи токенов,
        // из которых cumulative compute_energies всегда возвращает одну доминанту
        // → ActivityDynamics замирает.
        //
        // Решение: использовать только САМЫЙ ПОСЛЕДНИЙ токен (по last_event_id).
        // Отражает текущую активность, а не исторический срез.
        let energy_tokens: Vec<Token> = match all_tokens.iter().max_by_key(|t| t.last_event_id) {
            Some(t) => vec![*t],
            None => vec![],
        };
        let energies = if !self.subsystem_shell_refs.is_empty() {
            energy::compute_energies_with_shell(&energy_tokens, &self.subsystem_shell_refs)
        } else {
            energy::compute_energies(&energy_tokens, &self.subsystem_refs)
        };
        let dominant = energy::dominant_subsystem_confident(&energies);
        let weights = energy::energies_to_weights(&energies);

        // Первичный октант: наиболее активный из плана, либо fallback
        let primary_octant = effective_octants
            .first()
            .copied()
            .unwrap_or(Octant::CreativeAffirmation);

        // Детектировать переключение подсистемы + обновить TransitionMatrix (V7-B1)
        let transition = self.transition_detector.update(dominant, tick);
        if let Some(ref t) = transition {
            self.transition_matrix.record(t.from, t.to);
        }
        // Decay TransitionMatrix на каждом тике CR
        self.transition_matrix.decay();

        // Обновить ActivityTrace (CR-V6 Фаза A) + directed cascade (V7-C1)
        self.activity_trace.push(dominant, tick);
        let mut dynamics = self.activity_trace.compute_dynamics();
        dynamics.directed_cascade_score = self.activity_trace.directed_cascade_score(
            &self.transition_matrix,
            activity_trace::DIRECTED_CASCADE_THRESHOLD,
        );
        self.activity_dynamics = dynamics;

        // Обновить усталость подсистем (CR-V6 Фаза B)
        self.fatigue_store.update(dominant);
        // Детектировать split/merge сигналы (V7-D2, обновляется каждый тик CR)
        self.split_merge_candidates =
            split_merge::SplitMergeDetector::detect(&self.fatigue_store, &self.transition_matrix);

        // Детектировать мета-подсистемы (CR-V6 Фаза C)
        let signatures = activity_trace::classify(&self.activity_dynamics);
        self.meta_detector.detect(
            &self.activity_dynamics,
            &signatures,
            dominant,
            tick,
            &mut self.meta_store,
        );

        // Детектировать composite co-activations (CR-V6 Фаза D + V7-C2)
        let recent_subs = self.activity_trace.unique_subsystems_in_mid();
        self.composite_suspects =
            composite::detect_composite_suspects(&recent_subs, &signatures);
        self.composite_profiles = composite::detect_composite_profiles(
            &recent_subs,
            &signatures,
            &self.transition_matrix,
            BIDIRECTIONAL_COUPLING_THRESHOLD,
        );

        // Применить усталость к весам
        let mut fatigued_weights = weights.clone();
        self.fatigue_store.apply_to_weights(&mut fatigued_weights);

        // Детектировать конфликт подсистем + регистрировать дилемму (DilemmaDetector V2.1)
        let conflict = conflicts::detect_conflict(&energies, SUBSYSTEM_CONFLICT_THRESHOLD);
        let mut dilemma_cmds = self.dilemma_detector.detect(
            conflict,
            &mut self.dilemma_store,
            &self.subsystem_refs,
            exp_domain_id,
            tick,
        );
        // Сигнал B: стресс связей MAYA
        let stress_cmds = self.dilemma_detector.detect_signal_b(
            &maya_state.connections,
            dominant,
            &self.subsystem_refs,
            &mut self.dilemma_store,
            exp_domain_id,
            tick,
        );
        dilemma_cmds.extend(stress_cmds);
        // Сигнал C: Corpus Callosum — analytic ≠ synthetic octant из AxialStore
        let cc_cmds = self.dilemma_detector.detect_signal_c(
            &self.axial_store_snapshot,
            &frame_ids,
            &mut self.dilemma_store,
            exp_domain_id,
            tick,
        );
        dilemma_cmds.extend(cc_cmds);

        // — DIL-TD-01 Шаг 1: Разрешение дилемм ———————————————————————————————
        // Type III (ValueConflict): intensity decay + resolve при dominant_persistence > 0.75
        // Type IV (OntologicalConflict): resolve после 500 тиков в стабильном состоянии
        // Type V (Axiogenic): только DREAM Phase — пропускается здесь
        {
            const INTENSITY_DECAY: f32 = 0.997;
            const TYPE_III_PERSISTENCE_THRESHOLD: f32 = 0.75;
            const TYPE_III_INTENSITY_RESOLVE: f32 = 0.15;
            const TYPE_IV_AGE_TICKS: u64 = 500;
            const INTENSITY_FORCE_RESOLVE: f32 = 0.02;

            for rec in &mut self.dilemma_store.active {
                rec.intensity = (rec.intensity * INTENSITY_DECAY).max(0.0);
            }

            let dyn_persistence = self.activity_dynamics.dominant_persistence;
            let dyn_entropy = self.activity_dynamics.entropy_gradient;

            let to_resolve: Vec<(u64, DilemmaResolution)> = self
                .dilemma_store
                .active
                .iter()
                .filter_map(|rec| {
                    let resolution = match rec.dilemma_type {
                        DilemmaType::ValueConflict => {
                            if dominant != SubsystemId::Unknown
                                && dyn_persistence > TYPE_III_PERSISTENCE_THRESHOLD
                                && rec.intensity < TYPE_III_INTENSITY_RESOLVE
                            {
                                let winner =
                                    rec.anchors_in_conflict.first().copied().unwrap_or(0);
                                Some(DilemmaResolution::ContextualPriority { winner })
                            } else if rec.intensity < INTENSITY_FORCE_RESOLVE {
                                let winner =
                                    rec.anchors_in_conflict.first().copied().unwrap_or(0);
                                Some(DilemmaResolution::ContextualPriority { winner })
                            } else {
                                None
                            }
                        }
                        DilemmaType::OntologicalConflict => {
                            let age = tick.saturating_sub(rec.detected_at_tick);
                            if age >= TYPE_IV_AGE_TICKS && dyn_entropy.abs() < 0.05 {
                                Some(DilemmaResolution::Complementarity)
                            } else if rec.intensity < INTENSITY_FORCE_RESOLVE {
                                Some(DilemmaResolution::Complementarity)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    resolution.map(|r| (rec.id, r))
                })
                .collect();

            for (id, resolution) in to_resolve {
                self.dilemma_store.resolve(id, resolution);
            }

            // — DIL-TD-01 Шаг 2: Кристаллизация разрешённых дилемм в EXPERIENCE ——
            let pending = self.dilemma_store.drain_pending_crystallizations();
            for rec in &pending {
                let cmds =
                    crystallize_to_experience_commands(rec, [0i16; 3], exp_domain_id);
                dilemma_cmds.extend(cmds);
                // После кристаллизации регистрируем TensionTrace — след разрешённой дилеммы.
                // temperature=255 → след живёт ~255 тиков (TENSION_DECAY=1), виден OBS-снапшотом.
                self.pending_resolution_tensions.push(tick);
            }
        }

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

        // Обновить модальности известных Frame + cross-modal ко-активация (V1.0)
        self.modality_store.retain_known(&frame_ids);
        self.cross_modal_detector.update(&frame_ids, &self.modality_store, tick);

        Ok(dilemma_cmds)
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

    #[test]
    fn test_transition_matrix_empty_on_new() {
        let cr = ContextRecognizer::default();
        assert!(cr.transition_matrix().is_empty());
    }

    #[test]
    fn test_transition_matrix_probability_zero_no_data() {
        let cr = ContextRecognizer::default();
        let p = cr.transition_matrix().probability_of(SubsystemId::Writing, SubsystemId::Mathematics);
        assert_eq!(p, 0.0);
    }

    #[test]
    fn test_transition_matrix_most_likely_next_none_no_data() {
        let cr = ContextRecognizer::default();
        assert!(cr.transition_matrix().most_likely_next(SubsystemId::Writing).is_none());
    }

    // — Cross-Modal Binding integration —

    #[test]
    fn test_cross_modal_candidate_accumulates_with_two_modalities() {
        // Проверяет что при одновременной активности Text + Vision Frame
        // CrossModalDetector накапливает кандидата.
        use axiom_experience::Modality;
        let mut cr = ContextRecognizer::default();
        let text_frame: u32 = 0x4000_0042; // stable text sutra_id (bit 30)
        let vision_frame: u32 = 0x2000_0099; // stable vision sutra_id (bit 29)
        cr.register_frame_modality(text_frame, Modality::Text);
        cr.register_frame_modality(vision_frame, Modality::Vision);

        for tick in 0..10u64 {
            cr.cross_modal_detector.update(&[text_frame, vision_frame], &cr.modality_store, tick);
        }
        assert_eq!(cr.cross_modal_candidate_count(), 1, "should track one cross-modal candidate");
        assert_eq!(cr.cross_modal_pending_count(), 0, "not yet at threshold (50)");
    }

    #[test]
    fn test_cross_modal_bond_proposal_after_threshold() {
        // Проверяет полный путь: co-activation × 50 → pending bond → drain → UCL BondTokens.
        use axiom_experience::Modality;
        use cross_modal::MIN_CROSS_MODAL_COACTIVATION;
        let mut cr = ContextRecognizer::default();
        let text_frame: u32 = 0x4000_1234;
        let vision_frame: u32 = 0x2000_5678;
        cr.register_frame_modality(text_frame, Modality::Text);
        cr.register_frame_modality(vision_frame, Modality::Vision);

        for tick in 0..=(MIN_CROSS_MODAL_COACTIVATION as u64) {
            cr.cross_modal_detector.update(&[text_frame, vision_frame], &cr.modality_store, tick);
        }
        assert_eq!(cr.cross_modal_pending_count(), 1, "should have one bond pending DREAM");
        assert_eq!(cr.cross_modal_candidate_count(), 0, "candidate moved to pending");

        let cmds = cr.drain_cross_modal_bond_commands(109);
        assert_eq!(cmds.len(), 1, "should produce one BondTokens UCL command");
        assert_eq!(cr.cross_modal_bond_count(), 1, "bond registered as existing");

        // Повторные co-activations не должны создавать дубль
        for tick in 0..=(MIN_CROSS_MODAL_COACTIVATION as u64) {
            cr.cross_modal_detector.update(&[text_frame, vision_frame], &cr.modality_store, tick);
        }
        assert_eq!(cr.cross_modal_pending_count(), 0, "no duplicate bond proposal");
    }

    #[test]
    fn test_same_modality_no_cross_modal_candidate() {
        // Два Text Frame → не создаёт cross-modal кандидата (оба одной модальности).
        use axiom_experience::Modality;
        let mut cr = ContextRecognizer::default();
        let frame_a: u32 = 0x4000_0001;
        let frame_b: u32 = 0x4000_0002;
        cr.register_frame_modality(frame_a, Modality::Text);
        cr.register_frame_modality(frame_b, Modality::Text);

        for tick in 0..60u64 {
            cr.cross_modal_detector.update(&[frame_a, frame_b], &cr.modality_store, tick);
        }
        assert_eq!(cr.cross_modal_candidate_count(), 0, "same modality → no cross-modal candidate");
    }

    #[test]
    fn resolution_tension_emitted_after_dilemma_resolve() {
        // После разрешения дилеммы drain_resolution_tensions() возвращает tick кристаллизации.
        use crate::over_domain::context_recognizer::dilemma_store::{DilemmaResolution, DilemmaType};
        let mut cr = ContextRecognizer::default();
        cr.dilemma_store.push_active(DilemmaType::ValueConflict, vec![1, 2], 0, 0.8);
        let id = cr.dilemma_store.active[0].id;
        cr.dilemma_store.resolve(id, DilemmaResolution::ContextualPriority { winner: 1 });

        // Вручную дрейним pending_crystallizations как это делает on_tick, и заполняем буфер.
        let pending = cr.dilemma_store.drain_pending_crystallizations();
        for _ in &pending {
            cr.pending_resolution_tensions.push(42);
        }

        let tensions = cr.drain_resolution_tensions();
        assert_eq!(tensions.len(), 1, "одна дилемма → один tension tick");
        assert_eq!(tensions[0], 42);
    }

    #[test]
    fn drain_resolution_tensions_empty_initially() {
        let mut cr = ContextRecognizer::default();
        assert!(cr.drain_resolution_tensions().is_empty());
    }

    #[test]
    fn drain_resolution_tensions_clears_buffer() {
        let mut cr = ContextRecognizer::default();
        cr.pending_resolution_tensions.push(100);
        cr.pending_resolution_tensions.push(200);
        let first = cr.drain_resolution_tensions();
        assert_eq!(first.len(), 2);
        let second = cr.drain_resolution_tensions();
        assert!(second.is_empty(), "второй drain должен быть пустым");
    }
}
