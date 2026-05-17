// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

pub mod conflict;
pub mod emergent;
pub mod null;

pub use conflict::RuleBasedCorpusCallosumResolver;
pub use emergent::{
    DepthThresholdEmergentDetector, EMERGENT_CANDIDATE_MIN_AGE_TICKS,
    EMERGENT_CANDIDATE_MIN_DEPTH, EMERGENT_CANDIDATE_MIN_REACTIVATIONS,
};
pub use null::{
    NullConflictResolver, NullDepthAdvisor, NullEmergentAdvisor, NullOctantAdvisor,
    NullSubsystemAdvisor,
};
