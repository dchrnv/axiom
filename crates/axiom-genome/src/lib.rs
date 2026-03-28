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

pub mod types;
pub mod rules;
pub mod genome;
pub mod index;
pub mod subscriber;

pub use types::{ModuleId, ResourceId, Permission, DataType, MAX_MODULES, MAX_RESOURCES};
pub use rules::{AccessRule, ProtocolRule, GenomeInvariants, GenomeConfig};
pub use genome::{Genome, GenomeError};
pub use index::GenomeIndex;
pub use subscriber::GenomeSubscriber;
