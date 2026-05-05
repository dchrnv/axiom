// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-persist — персистентное хранилище состояния AXIOM.
//
// Граница ответственности: читает из axiom-runtime, пишет на диск.
// axiom-runtime НЕ знает об axiom-persist.
//
// Спецификация: docs/spec/Memory_Persistence_V1_0.md

pub mod auto;
pub mod error;
pub mod exchange;
pub mod format;
pub mod loader;
pub mod manifest;
pub mod writer;

pub use auto::{AutoSaver, PersistenceConfig};
pub use error::PersistError;
pub use exchange::{
    export_skills, export_traces, import_skills, import_traces, ExchangeKind, ExportReport,
    ImportReport,
};
pub use loader::{load, LoadResult, IMPORT_WEIGHT_FACTOR};
pub use manifest::{ManifestContents, MemoryManifest, FORMAT_VERSION};
pub use writer::{save, WriteOptions};
