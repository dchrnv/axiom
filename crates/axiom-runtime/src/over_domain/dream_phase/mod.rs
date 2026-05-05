// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// DREAM Phase — Over-Domain механизм нового класса.
// Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md
//
// Три подмодуля (добавляются по этапам):
//   state    — машина состояний, события, типы приоритетов (Этап 1)
//   fatigue  — FatigueTracker, IdleTracker (Этап 2)
//   scheduler — DreamScheduler (Этап 2)
//   cycle    — DreamCycle, DreamProposal, DreamReport (Этап 3)

pub mod cycle;
pub mod fatigue;
pub mod scheduler;
pub mod state;

pub use state::{
    DreamPhaseEvent, DreamPhaseState, DreamPhaseStats, GatewayPriority, SleepTrigger, WakeReason,
};

pub use cycle::{
    CycleAdvanceResult, CycleStage, DreamCycle, DreamCycleConfig, DreamCycleStats, DreamProposal,
    DreamProposalKind, DreamReport,
};
pub use fatigue::{FatigueSnapshot, FatigueTracker, FatigueWeights, IdleTracker};
pub use scheduler::{
    DreamScheduler, DreamSchedulerConfig, DreamSchedulerStats, SleepDecision, SleepTriggerKind,
};
