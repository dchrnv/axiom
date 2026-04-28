// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Over-Domain Layer (axiom-runtime::over_domain)
//
// Архитектурный слой компонентов над доменами.
// Спецификация: docs/spec/Weaver/Over_Domain_Layer_V1_1.md
//
// Две категории:
//   Guardians — контроль допустимости, veto-логика (существующий: guardian.rs)
//   Weavers   — кристаллизация узоров в EXPERIENCE, промоция в SUTRA через CODEX

/// Базовые trait'ы Over-Domain компонентов
pub mod traits;
/// Weavers — кристаллизация реляционных структур
pub mod weavers;

pub use traits::{
    WeaverId,
    OverDomainError,
    OverDomainComponent,
    Weaver,
    CrystallizationProposal,
    PromotionProposal,
};

pub use weavers::{
    FrameWeaver, FrameCandidate, Participant,
    FrameWeaverConfig, FrameWeaverStats,
    PromotionRule, CrystallizationRule, RuleTrigger, RuleCondition, RuleAction,
    CycleStrategy, FRAME_WEAVER_ID,
    RestoreError, RestoredFrame, restore_frame_from_anchor,
};
