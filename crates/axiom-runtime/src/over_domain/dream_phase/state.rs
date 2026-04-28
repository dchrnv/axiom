// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Типы состояний DREAM-фазы.
// Спецификация: docs/spec/Dream/DREAM_Phase_V1_0.md, разделы 2, 3, 5.

/// Четыре состояния системы. Переход — через DreamScheduler и DreamCycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum DreamPhaseState {
    #[default]
    Wake          = 0,
    FallingAsleep = 1,
    Dreaming      = 2,
    Waking        = 3,
}

/// Причина засыпания — фиксируется в DreamPhaseEvent и DreamReport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepTrigger {
    Idle     { idle_ticks: u32 },
    Fatigue  { fatigue_score: u8 },
    ExplicitCommand { source: u16 },
}

/// Причина пробуждения — фиксируется в DreamPhaseEvent и DreamReport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeReason {
    CycleComplete,
    CriticalSignal { source: u16 },
    Timeout { max_dream_duration: u32 },
    GuardianOverride,
}

/// События машины состояний DREAM-фазы — отправляются в COM.
#[derive(Debug, Clone)]
pub enum DreamPhaseEvent {
    WakeToFallingAsleep      { trigger: SleepTrigger, fatigue: u8 },
    FallingAsleepToDreaming  { drained_operations: u32 },
    DreamingToWaking         { cycle_complete: bool, reason: WakeReason },
    WakingToWake             { resumed_at_event: u64 },
}

/// Приоритет входящей команды — определяет поведение во время DREAMING.
///
/// В V1.0 используются Normal и Critical. Emergency зарезервирован для V2.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum GatewayPriority {
    #[default]
    Normal    = 0,
    /// Вызывает пробуждение системы из DREAMING.
    Critical  = 1,
    /// V2.0: немедленное прерывание DreamCycle. В V1.0 — ведёт себя как Critical.
    Emergency = 2,
}

/// Счётчики DREAM-фазы для BroadcastSnapshot и CLI :dream-stats.
/// Поля добавляются по мере реализации этапов.
#[derive(Debug, Default, Clone)]
pub struct DreamPhaseStats {
    pub total_sleeps:          u64,
    pub total_dream_ticks:     u64,
    pub interrupted_dreams:    u64,
    // расширяется в этапах 2–4
}
