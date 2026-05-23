// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AdvisorySource — трейт для источников рекомендаций OverDomainArbiter.
// Источник: docs/architecture/OverDomainArbiter_V1_0.md §3–4

/// Идентификатор источника рекомендаций (назначается при регистрации).
pub type SourceId = u8;

/// Уникальный ID одной рекомендации в рамках источника.
pub type AdvisoryId = u64;

/// Тип рекомендации.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdvisoryType {
    DepthHint,
    OctantCorrection,
    ConflictDiagnosis,
    SubsystemAttribution,
    EmergentCandidate,
    /// V3: смена доминирующего нарративного октанта в скользящем окне сессии.
    NarrativeShift,
}

/// Действие которое Arbiter может выполнить при принятии рекомендации.
#[derive(Debug, Clone)]
pub enum AdvisoryAction {
    /// Установить глубину Frame в заданном октанте.
    ApplyDepth { octant: usize, depth: u16 },
    /// Поставить уведомление в очередь Workstation (для RequireConfirmation).
    NotifyWorkstation { label: String },
    /// V3: установить advisory override октанта для Frame в AxialEvaluatorStorage.
    OverrideOctant { sutra_id: u32, octant_idx: usize },
}

/// Единица рекомендации от AdvisorySource.
#[derive(Debug, Clone)]
pub struct Advisory {
    pub id: AdvisoryId,
    pub source: SourceId,
    pub advisory_type: AdvisoryType,
    /// sutra_id Frame о котором рекомендация.
    pub subject_id: u32,
    /// 0.0..1.0
    pub confidence: f32,
    pub action: AdvisoryAction,
    pub created_at_event: u64,
    /// Индекс октанта (0..7) для OctantCorrection — используется CognitiveProfile scaling.
    /// None для остальных типов advisory.
    pub octant_hint: Option<usize>,
}

/// Исход обработки рекомендации — передаётся источнику через on_feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvisoryOutcome {
    /// Применено автономно (AutoApply).
    Applied,
    /// Помещено в очередь Workstation.
    Queued,
    /// Пропущено (ниже порога confidence или режим Ignore).
    Skipped,
    /// Подтверждено chrnv из очереди Workstation.
    Confirmed,
    /// Отклонено chrnv из очереди Workstation.
    Rejected,
}

/// Трейт для источников рекомендаций.
///
/// В V1 единственный источник — NeuralAdvisor.
/// Будущие источники (PatternAdvisor и др.) реализуют тот же трейт.
pub trait AdvisorySource: Send {
    fn source_id(&self) -> SourceId;

    /// Вернуть все активные рекомендации этого источника.
    /// Вызывается Arbiter каждый тик.
    fn poll_advisories(&self) -> Vec<Advisory>;

    /// Обратная связь: что случилось с рекомендацией.
    /// В V1 реализации могут игнорировать (no-op).
    fn on_feedback(&mut self, id: AdvisoryId, outcome: AdvisoryOutcome);
}
