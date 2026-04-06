// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-persist — персистентное хранилище состояния AXIOM.
//
// Граница ответственности: читает из axiom-runtime, пишет на диск.
// axiom-runtime НЕ знает об axiom-persist.
//
// Спецификация: docs/spec/Memory_Persistence_V1_0.md

pub mod error;
pub mod format;
pub mod manifest;
pub mod writer;
pub mod loader;
pub mod auto;
pub mod exchange;

pub use error::PersistError;
pub use manifest::{MemoryManifest, ManifestContents, FORMAT_VERSION};
pub use writer::{save, WriteOptions};
pub use loader::{load, LoadResult, IMPORT_WEIGHT_FACTOR};
pub use auto::{AutoSaver, PersistenceConfig};
pub use exchange::{
    export_traces, export_skills,
    import_traces, import_skills,
    ImportReport, ExportReport, ExchangeKind,
};
