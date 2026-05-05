// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Over-Domain Layer — Weavers
// Реализованные: FrameWeaver V1.1 (Phase 3)
// Deferred: CausalWeaver, SpatialWeaver, TemporalWeaver, AnalogyWeaver, NarrativeWeaver

pub mod frame;

pub use frame::{
    restore_frame_from_anchor, CrystallizationRule, CycleStrategy, FrameCandidate, FrameWeaver,
    FrameWeaverConfig, FrameWeaverStats, Participant, PromotionRule, RestoreError, RestoredFrame,
    RuleAction, RuleCondition, RuleTrigger, FRAME_WEAVER_ID,
};
