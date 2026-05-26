// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Пять трейтов NeuralAdvisor + их input/output типы.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §4

use axiom_experience::{EvaluationLevel, Octant, SubsystemId};

use crate::over_domain::neural_advisor::history::AdvisoryHistoryEntry;

// ─── Shared output types ─────────────────────────────────────────────────────

/// Подсказка о целевой глубине Frame в конкретном октанте.
#[derive(Debug, Clone)]
pub struct DepthHint {
    pub target_octant: Octant,
    pub suggested_depth: u16,
    /// 0.0..1.0
    pub confidence: f32,
}

/// Предложение октанта от советника.
#[derive(Debug, Clone)]
pub struct OctantSuggestion {
    pub octant: Octant,
    /// 0.0..1.0
    pub confidence: f32,
    pub reason: OctantSuggestionReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctantSuggestionReason {
    /// V9: ML-модель поняла содержание
    SemanticContent,
    /// Историческая активность в этом октанте перевешивает аналитику
    DepthHistoryBias,
    /// Подсистема Frame исторически тяготеет к этому октанту
    SubsystemAffinity,
    /// Разрешение конфликта двух октантов в пользу одного
    BoundaryResolution,
}

/// Диагноз конфликта Corpus Callosum.
///
/// Отличие от `ConflictResolution` в axiom-experience: та описывает *что система делает*
/// с конфликтом. `ConflictDiagnosis` — *почему конфликт существует*.
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictDiagnosis {
    /// Не удалось диагностировать
    Unresolved,
    /// Frame живёт на границе двух октантов — структурно нормально
    BoundaryFrame,
    /// Frame между состояниями — временно (молодой Frame)
    TransitionState,
    /// Frame стабильно принадлежит обоим октантам (зрелый, часто реактивируется)
    DualNature,
    /// Один октант явно доминирует по истории активности
    DominantOctant(Octant),
}

/// Результат диагностики конфликта.
#[derive(Debug, Clone)]
pub struct ConflictResolutionHint {
    pub diagnosis: ConflictDiagnosis,
    /// 0.0..1.0
    pub confidence: f32,
}

/// Предложение подсистемы от советника.
#[derive(Debug, Clone)]
pub struct SubsystemSuggestion {
    pub primary: SubsystemId,
    /// Вторичная подсистема если Frame двойственен
    pub secondary: Option<SubsystemId>,
    /// 0.0..1.0
    pub confidence: f32,
}

/// Результат детектирования эмерджентного примитива.
#[derive(Debug, Clone)]
pub struct EmergentDetectionResult {
    pub is_candidate: bool,
    /// 0.0..1.0
    pub confidence: f32,
    // suggested_name отсутствует в V1 — требует семантического понимания
}

// ─── Input types ─────────────────────────────────────────────────────────────

/// Вход для DepthPredictionAdvisor.
#[derive(Debug, Clone)]
pub struct DepthAdvisorInput {
    pub sutra_id: u32,
    pub subsystem: SubsystemId,
    /// Текущие глубины по октантам
    pub current_depth_per_octant: [u16; 8],
    pub reactivation_count: u32,
    pub frame_age_ticks: u64,
    pub primary_octant: Octant,
    pub event_id: u64,
}

/// Вход для OctantCorrectionAdvisor.
#[derive(Debug, Clone)]
pub struct OctantAdvisorInput {
    pub sutra_id: u32,
    /// Октант по аналитическим метрикам (от AxialEvaluator)
    pub analytic_octant: Octant,
    /// Октант по целостному синтезу позиций
    pub synthetic_octant: Octant,
    pub evaluation_level: EvaluationLevel,
    /// X-ось: Apollo (pos) / Dionysus (neg)
    pub x_positive_pole: u8,
    pub x_negative_pole: u8,
    /// Y-ось: Eros (pos) / Thanatos (neg)
    pub y_positive_pole: u8,
    pub y_negative_pole: u8,
    /// Z-ось: Will (pos) / Nothing (neg)
    pub z_positive_pole: u8,
    pub z_negative_pole: u8,
    /// Доминирующая подсистема Frame (из InterpretationProfile)
    pub primary_subsystem: SubsystemId,
    pub event_id: u64,
    /// Текущие глубины Frame по октантам (из SutraDepthStore snapshot).
    /// Используется DepthHistoryBiasAdvisor.
    pub depth_per_octant: [u16; 8],
    /// Число DREAM-циклов с активностью (из SutraDepthStore snapshot).
    pub reactivation_count: u32,
}

/// Вход для CorpusCallosumResolver.
#[derive(Debug, Clone)]
pub struct ConflictAdvisorInput {
    pub sutra_id: u32,
    pub analytic_octant: Octant,
    pub synthetic_octant: Octant,
    /// Сила конфликта: 1 ось = 85, 2 оси = 170, 3 оси = 255
    pub conflict_strength: u8,
    /// Возраст Frame в тиках (event_id - crystallization_event)
    pub frame_age_ticks: u64,
    pub reactivation_count: u32,
    pub primary_subsystem: SubsystemId,
    pub event_id: u64,
    /// G2: снапшот истории советов для этого Frame (PatternLearningResolver).
    /// None в V1/V2 конфигурации или если история пуста.
    pub history: Option<Vec<AdvisoryHistoryEntry>>,
}

/// Вход для SubsystemAttributionAdvisor.
#[derive(Debug, Clone)]
pub struct SubsystemAdvisorInput {
    pub sutra_id: u32,
    /// Энергетические веса подсистем от ContextRecognizer (0..255)
    pub energy_weights: Vec<(SubsystemId, u8)>,
    pub primary_octant: Octant,
    pub depth_per_octant: [u16; 8],
    pub reactivation_count: u32,
    pub event_id: u64,
}

/// Вход для EmergentPatternAdvisor.
#[derive(Debug, Clone)]
pub struct EmergentAdvisorInput {
    pub sutra_id: u32,
    pub octant: Octant,
    pub depth_per_octant: [u16; 8],
    pub reactivation_count: u32,
    pub frame_age_ticks: u64,
    /// sutra_id уже зарегистрированных примитивов
    pub known_primitive_ids: Vec<u32>,
    pub event_id: u64,
}

// ─── Traits ──────────────────────────────────────────────────────────────────

/// Предсказание целевой глубины для нового Frame.
pub trait DepthPredictionAdvisor: Send + Sync {
    fn predict_depth(&self, input: &DepthAdvisorInput) -> Option<DepthHint>;
}

/// Коррекция аналитически вычисленного октанта.
///
/// Advisory-only: не перезаписывает `AxialEvaluation.octant`.
/// Результат хранится в `AdvisoryResult.octant_suggestion`.
pub trait OctantCorrectionAdvisor: Send + Sync {
    fn suggest_octant(&self, input: &OctantAdvisorInput) -> Option<OctantSuggestion>;
}

/// Диагностика конфликта Corpus Callosum.
///
/// Advisory-only: не меняет `AxialConflict.resolution`.
/// Результат хранится в `AdvisoryResult.conflict_diagnosis`.
pub trait CorpusCallosumResolver: Send + Sync {
    fn resolve(&self, input: &ConflictAdvisorInput) -> ConflictResolutionHint;
}

/// Уточнение атрибуции подсистемы.
///
/// Advisory-only: не меняет `InterpretationProfile.primary`.
/// Результат хранится в `AdvisoryResult.subsystem_suggestion`.
pub trait SubsystemAttributionAdvisor: Send + Sync {
    fn suggest_subsystem(&self, input: &SubsystemAdvisorInput) -> Option<SubsystemSuggestion>;
}

/// Детектирование кандидатов в эмерджентные примитивы.
///
/// Если `is_candidate = true` → NeuralAdvisor добавляет в EmergentPrimitiveStore
/// и посылает `NotifyEmergentCandidate` UCL-команду.
pub trait EmergentPatternAdvisor: Send + Sync {
    fn detect(&self, input: &EmergentAdvisorInput) -> EmergentDetectionResult;
}
