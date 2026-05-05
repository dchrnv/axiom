// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use std::io;

/// Ошибки персистентного хранилища.
#[derive(Debug)]
pub enum PersistError {
    /// Директория или файл не найдены
    NotFound(String),
    /// manifest.yaml повреждён или отсутствует
    CorruptManifest(String),
    /// Версия формата несовместима
    VersionMismatch {
        expected: &'static str,
        found: String,
    },
    /// Ошибка I/O
    Io(io::Error),
    /// Ошибка bincode-декодирования
    Decode(String),
    /// Ошибка bincode-кодирования
    Encode(String),
}

impl std::fmt::Display for PersistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistError::NotFound(p) => write!(f, "persist: not found: {p}"),
            PersistError::CorruptManifest(msg) => write!(f, "persist: corrupt manifest: {msg}"),
            PersistError::VersionMismatch { expected, found } => write!(
                f,
                "persist: version mismatch (expected={expected}, found={found})"
            ),
            PersistError::Io(e) => write!(f, "persist: io error: {e}"),
            PersistError::Decode(msg) => write!(f, "persist: decode error: {msg}"),
            PersistError::Encode(msg) => write!(f, "persist: encode error: {msg}"),
        }
    }
}

impl From<io::Error> for PersistError {
    fn from(e: io::Error) -> Self {
        PersistError::Io(e)
    }
}
