// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use axiom_experience::SubsystemId;

/// Активность подсистемы в текущем срезе.
#[derive(Clone, Debug)]
pub struct SubsystemActivity {
    pub id: SubsystemId,
    pub energy: u8,
    /// activation_load из SubsystemFatigue (0.0..10.0).
    pub fatigue_load: f32,
}

/// Активная дилемма в уровне State+.
#[derive(Clone, Debug)]
pub struct ActiveDilemmaEntry {
    pub id: u64,
    /// DilemmaType as u8 (0=DataConflict..4=Axiogenic).
    pub dilemma_type: u8,
    pub intensity: f32,
    pub detected_at_tick: u64,
}

/// Эмерджентный кандидат в уровне Full+.
#[derive(Clone, Debug)]
pub struct EmergentEntry {
    pub sutra_id: u32,
    pub depth_avg: u16,
    pub reactivations: u32,
}

/// Полный внутренний срез системы — Sensorium V1.0.
///
/// Разбит на группы по §2 спеки Sensorium_V1_0.md.
/// Родной формат: бинарный (Clone + Debug). Адаптеры переводят во внешние протоколы.
///
/// Поля под внутренний импульс зарезервированы, пусты до Волн (V2.0).
#[derive(Clone, Debug, Default)]
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

    // — ВНУТРЕННИЙ ИМПУЛЬС (зарезервировано под ВОЛНЫ V2.0) —
    // internal_drive: Vec<Impulse>    // пусто в V1
    // unfinished: Vec<u32>            // пусто в V1
    // curiosity_targets: Vec<u32>     // пусто в V1
}
