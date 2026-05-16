// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-experience — хранилища семантических данных.
//
// Содержит data-структуры для AxialEvaluator и ContextRecognizer.
// Не содержит логики оценки — только хранение и базовые операции.
//
// Крейты-потребители:
//   axiom-runtime (AxialEvaluator, ContextRecognizer) — пишут сюда
//   axiom-persist — сериализует (через feature "serde")

#![deny(unsafe_code)]

pub mod axial_store;
pub mod emergent_primitive_store;
pub mod interpretation_profile_store;
pub mod sutra_depth_store;
pub mod types;

pub use axial_store::{AxialConflict, AxialEvaluation, AxialStore, ConflictResolution};
pub use emergent_primitive_store::{
    EmergentPrimitive, EmergentPrimitiveStore, MAX_EMERGENT_PRIMITIVES,
};
pub use interpretation_profile_store::{InterpretationProfile, InterpretationProfileStore};
pub use sutra_depth_store::{
    SutraDepthEntry, SutraDepthStore, DECAY_PER_CYCLE, MAX_GROWTH_PER_CYCLE, PRIMITIVE_DEPTH,
    PROMOTED_DEPTH,
};
pub use types::{
    AxialDominant, AxialScore, ContextSnapshot, EvaluationLevel, FrameComposition, Octant,
    SubsystemId,
};
