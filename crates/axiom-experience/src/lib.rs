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
pub mod modality_store;
pub mod emergent_primitive_store;
pub mod fatigue_store;
pub mod interpretation_profile_store;
pub mod meta_store;
pub mod sutra_depth_store;
pub mod types;

pub use axial_store::{AxialConflict, AxialEvaluation, AxialStore, ConflictResolution};
pub use fatigue_store::{
    FatigueStore, SubsystemFatigue,
    DECAY_FACTOR as FATIGUE_DECAY_FACTOR,
    DEBT_DECAY as FATIGUE_DEBT_DECAY,
    DEBT_RATE as FATIGUE_DEBT_RATE,
    DREAM_RECOVERY as FATIGUE_DREAM_RECOVERY,
    MAX_ACTIVATION_LOAD,
};
pub use emergent_primitive_store::{
    EmergentPrimitive, EmergentPrimitiveStore, MAX_EMERGENT_PRIMITIVES,
};
pub use interpretation_profile_store::{InterpretationProfile, InterpretationProfileStore};
pub use sutra_depth_store::{
    SutraDepthEntry, SutraDepthStore, DECAY_PER_CYCLE, MAX_GROWTH_PER_CYCLE, PRIMITIVE_DEPTH,
    PROMOTED_DEPTH,
};
pub use meta_store::{
    MetaActivation, MetaStore, MetaSubsystemId,
    META_ANALYSIS, META_DIALOGUE, META_IMAGINATION, META_PERCEPTION,
    META_RECALL, META_REFLECTION, META_SYNTHESIS,
};
pub use modality_store::{Modality, ModalityStore};
pub use types::{
    AxialDominant, AxialScore, ContextSnapshot, EvaluationLevel, FrameComposition, Octant,
    SubsystemId,
};
