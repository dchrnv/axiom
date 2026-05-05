// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AXIOM: GENOME V1.0 — Неизменяемый конституционный слой системы
//
// Загружается первым (до COM, до доменов, до любых событий).
// После валидации замораживается в Arc<Genome> — никто не получает &mut Genome.
//
// Связанные спецификации:
//   - docs/spec/GENOME_V1_0.md (каноническая)
//   - docs/spec/GUARDIAN_V1_0.md
//   - docs/spec/Ashti_Core_V2_1.md

pub mod genome;
pub mod index;
pub mod rules;
pub mod subscriber;
pub mod types;

pub use genome::{Genome, GenomeError};
pub use index::GenomeIndex;
pub use rules::{AccessRule, GenomeConfig, GenomeInvariants, ProtocolRule};
pub use subscriber::GenomeSubscriber;
pub use types::{DataType, ModuleId, Permission, ResourceId, MAX_MODULES, MAX_RESOURCES};
