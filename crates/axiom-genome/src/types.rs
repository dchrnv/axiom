// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use serde::{Deserialize, Serialize};

/// Идентификаторы модулей системы.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ModuleId {
    Sutra = 0,
    Execution = 1,
    Shadow = 2,
    Codex = 3,
    Map = 4,
    Probe = 5,
    Logic = 6,
    Dream = 7,
    Void = 8,
    Experience = 9,
    Maya = 10,
    Arbiter = 11,
    Guardian = 12,
    Heartbeat = 13,
    Shell = 14,
    Adapters = 15,
    FrameWeaver = 16,
}

pub const MAX_MODULES: usize = 17;

/// Идентификаторы ресурсов, к которым контролируется доступ.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ResourceId {
    SutraTokens = 0,
    AshtiField = 1,
    ExperienceMemory = 2,
    MayaOutput = 3,
    CodexRules = 4,
    GenomeConfig = 5,
    ArbiterState = 6,
    HeartbeatClock = 7,
}

pub const MAX_RESOURCES: usize = 8;

/// Уровень доступа.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum Permission {
    None = 0,
    Read = 1,
    Execute = 2,
    Control = 3,
    ReadWrite = 4,
}

/// Тип данных, передаваемых по протоколу.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DataType {
    TokenReference = 0,
    ResonanceResponse = 1,
    Reflex = 2,
    PatternHint = 3,
    ProcessingResult = 4,
    NewExperience = 5,
    ComparisonResult = 6,
    Feedback = 7,
}
