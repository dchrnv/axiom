// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Загрузка/сохранение весов из .bin файлов (bincode).
// Формат: [ModelMeta (bincode)] ++ [слои (bincode)]

use std::path::Path;
use crate::model::NeuralError;

pub fn read_bin<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<T, NeuralError> {
    let bytes = std::fs::read(path)
        .map_err(|e| NeuralError::IoError(e.to_string()))?;
    bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
        .map(|(v, _)| v)
        .map_err(|e| NeuralError::InvalidWeights(e.to_string()))
}

pub fn write_bin<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), NeuralError> {
    let bytes = bincode::serde::encode_to_vec(value, bincode::config::standard())
        .map_err(|e| NeuralError::InvalidWeights(e.to_string()))?;
    std::fs::write(path, &bytes)
        .map_err(|e| NeuralError::IoError(e.to_string()))
}
