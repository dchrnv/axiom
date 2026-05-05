// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Over-Domain Layer (axiom-runtime::over_domain)
//
// Архитектурный слой компонентов над доменами.
// Спецификация: docs/spec/Weaver/Over_Domain_Layer_V1_1.md
//
// Три категории:
//   Guardians   — контроль допустимости, veto-логика (существующий: guardian.rs)
//   Weavers     — кристаллизация узоров в EXPERIENCE, промоция в SUTRA через DREAM
//   DreamPhase  — дискретный прерываемый режим переработки опыта (DREAM Phase V1.0)

/// DREAM Phase — машина состояний сна, DreamScheduler, DreamCycle
pub mod dream_phase;
/// Базовые trait'ы Over-Domain компонентов
pub mod traits;
/// Weavers — кристаллизация реляционных структур
pub mod weavers;

pub use traits::{
    CrystallizationProposal, OverDomainComponent, OverDomainError, PromotionProposal, Weaver,
    WeaverId,
};

pub use weavers::{
    restore_frame_from_anchor, CrystallizationRule, CycleStrategy, FrameCandidate, FrameWeaver,
    FrameWeaverConfig, FrameWeaverStats, Participant, PromotionRule, RestoreError, RestoredFrame,
    RuleAction, RuleCondition, RuleTrigger, FRAME_WEAVER_ID,
};

pub use dream_phase::{
    CycleAdvanceResult, CycleStage, DreamCycle, DreamCycleConfig, DreamCycleStats, DreamPhaseEvent,
    DreamPhaseState, DreamPhaseStats, DreamProposal, DreamProposalKind, DreamReport,
    DreamScheduler, DreamSchedulerConfig, DreamSchedulerStats, FatigueSnapshot, FatigueTracker,
    FatigueWeights, GatewayPriority, IdleTracker, SleepDecision, SleepTrigger, SleepTriggerKind,
    WakeReason,
};
