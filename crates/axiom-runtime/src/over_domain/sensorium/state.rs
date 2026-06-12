// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use axiom_experience::SubsystemId;
use serde::Serialize;

/// Краткая сводка одного домена (11 доменов SUTRA–MAYA).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SensoriumDomainSummary {
    pub domain_id: u16,
    pub token_count: usize,
    pub connection_count: usize,
    /// Средняя температура активных токенов (0 если домен пуст).
    pub temperature_avg: u8,
}

/// Краткая сводка последнего завершённого dream-цикла.
#[derive(Clone, Debug, Serialize)]
pub struct SensoriumDreamSummary {
    pub cycle_id: u64,
    pub started_at_tick: u64,
    pub ended_at_tick: u64,
    pub proposals_accepted: u32,
    pub proposals_rejected: u32,
    pub sutra_written: u32,
}

/// Активность подсистемы в текущем срезе.
#[derive(Clone, Debug, Serialize)]
pub struct SubsystemActivity {
    pub id: SubsystemId,
    pub energy: u8,
    /// activation_load из SubsystemFatigue (0.0..10.0).
    pub fatigue_load: f32,
}

/// Активная дилемма в уровне State+.
#[derive(Clone, Debug, Serialize)]
pub struct ActiveDilemmaEntry {
    pub id: u64,
    /// DilemmaType as u8 (0=DataConflict..4=Axiogenic).
    pub dilemma_type: u8,
    pub intensity: f32,
    pub detected_at_tick: u64,
}

/// Эмерджентный кандидат в уровне Full+.
#[derive(Clone, Debug, Serialize)]
pub struct EmergentEntry {
    pub sutra_id: u32,
    pub depth_avg: u16,
    pub reactivations: u32,
}

/// Статус нейронного depth-советника (Neural Integration Этап 1).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NeuralDepthStatus {
    /// Режим: "rule" | "neural".
    pub mode: String,
    /// Время последнего inference (наносекунды). 0 = ни одного.
    pub last_infer_ns: u64,
    /// Тик последнего inference.
    pub last_infer_tick: u64,
    /// Кешированные веса реактивации [8 октантов] (0.0..1.0).
    /// Высокое значение = октант нуждается в реактивации.
    pub cached_weights: [f32; 8],
    /// Загружен ли .bin файл весов (false = нулевые веса).
    pub weights_loaded: bool,
}

/// Полный внутренний срез системы — Sensorium V2.0.
///
/// Разбит на группы по §2 спеки Sensorium_V1_0.md.
/// V2.0: поглощает поля BroadcastSnapshot (Фаза A), публикуется через BroadcastHandle (Фаза B).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SensoriumState {
    /// Тик сборки среза.
    pub collected_at_tick: u64,
    /// COM event_id на момент сборки (причинное время).
    pub causal_time: u64,

    // — ВОСПРИЯТИЕ СЕЙЧАС —
    /// Активные подсистемы с энергиями (все у кого energy > 0).
    pub active_subsystems: Vec<SubsystemActivity>,
    /// Доминирующая подсистема.
    pub dominant_subsystem: Option<SubsystemId>,
    /// Классификатор динамики активности ("Cascading" / "Steady" / "Oscillating" / ...).
    pub activity_signature: String,

    // — ОЦЕНКА СЕЙЧАС (AxialEvaluator) —
    /// Доминирующий октант (0..7, None если данных нет).
    pub dominant_octant: Option<u8>,
    /// Corpus Callosum: есть ли конфликт analytic/synthetic.
    pub corpus_callosum_active: bool,

    // — НАПРЯЖЕНИЯ СЕЙЧАС —
    /// Число активных дилемм.
    pub active_dilemma_count: usize,
    /// Детали активных дилемм (только уровень State+).
    pub active_dilemmas: Vec<ActiveDilemmaEntry>,
    /// Есть ли незакристаллизованные дилеммы.
    pub has_pending_crystallization: bool,

    // — ГЛУБИНА И ПАМЯТЬ (FrameWeaver) —
    /// Число Frame-кандидатов в MAYA.
    pub candidates_count: usize,
    /// Среднее shell-сходство кандидатов (0.0..=1.0).
    pub avg_shell_similarity: f32,
    /// Эмерджентные кандидаты (только уровень Full+).
    pub emergent_candidates: Vec<EmergentEntry>,

    // — СОСТОЯНИЕ ОРГАНИЗМА —
    /// Фаза DREAM: 0=Wake 1=FallingAsleep 2=Dreaming 3=Waking.
    pub dream_phase_raw: u8,
    /// Подсистемы с высокой усталостью (activation_load > 5.0).
    pub fatigued_subsystems: Vec<SubsystemId>,
    /// Число composite suspects (Calculus, Rhythm, ...).
    pub composite_suspect_count: usize,

    // — ЭМЕРДЖЕНТНОЕ —
    /// Pending advisories в OverDomainArbiter.
    pub pending_advisories: usize,
    /// Cross-modal bonds накоплено.
    pub cross_modal_bonds: usize,

    // — ВНУТРЕННИЙ ИМПУЛЬС (заполняется Waves V1.0) —
    /// Насколько система живёт изнутри vs реакция на вход (0.0..1.0).
    pub internal_dominance_factor: f32,
    /// Число активных импульсов от Waves.
    pub active_impulse_count: usize,
    /// Источники активных импульсов (краткий тег: "Dilemma"/"Resonance"/"Unfinished").
    pub impulse_sources: Vec<&'static str>,

    // — NEURAL INTEGRATION (Этап 1) —
    /// Статус нейронного depth-советника.
    pub neural_depth: NeuralDepthStatus,

    // — ДВИЖОК (добавлено в V2.0, Фаза A — поглощение BroadcastSnapshot) —
    /// Общее число следов опыта (Experience.trace_count).
    pub trace_count: usize,
    /// Число активных tension traces.
    pub tension_count: usize,
    /// Краткая сводка по каждому из 11 доменов.
    pub domain_summaries: Vec<SensoriumDomainSummary>,
    /// Тик последней кристаллизации Frame (0 — ни одной).
    pub last_crystallization_tick: u64,
    /// Вето Guardian с момента последнего Wake.
    pub guardian_vetoes_since_wake: u64,
    /// Cross-modal кандидаты (не достигшие порога BondTokens).
    pub cross_modal_candidates: usize,
    /// Последний завершённый dream-цикл (None до первого сна).
    pub last_dream_summary: Option<SensoriumDreamSummary>,
}
