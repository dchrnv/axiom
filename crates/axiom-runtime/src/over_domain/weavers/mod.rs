// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Over-Domain Layer — Weavers
// Реализованные: FrameWeaver V1.1 (Phase 3)
// Deferred: CausalWeaver, SpatialWeaver, TemporalWeaver, AnalogyWeaver, NarrativeWeaver

pub mod frame;

pub use frame::{
    FrameWeaver, FrameCandidate, Participant,
    FrameWeaverConfig, FrameWeaverStats,
    PromotionRule, CrystallizationRule, RuleTrigger, RuleCondition, RuleAction,
    CycleStrategy, FRAME_WEAVER_ID,
    RestoreError, RestoredFrame, restore_frame_from_anchor,
};
