// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Re-export FatigueStore из axiom-experience (V7-B2).
// Реализация перенесена в axiom-experience/src/fatigue_store.rs.

pub use axiom_experience::{
    FatigueStore, SubsystemFatigue,
    FATIGUE_DECAY_FACTOR as DECAY_FACTOR,
    FATIGUE_DEBT_DECAY as DEBT_DECAY,
    FATIGUE_DEBT_RATE as DEBT_RATE,
    FATIGUE_DREAM_RECOVERY as DREAM_RECOVERY,
    MAX_ACTIVATION_LOAD,
};
