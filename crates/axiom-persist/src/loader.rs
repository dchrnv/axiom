// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MemoryLoader — десериализация состояния Engine с диска.

use std::path::Path;
use axiom_runtime::AxiomEngine;
use crate::error::PersistError;
use crate::format::{StoredEngineState, StoredTrace};
use crate::manifest::MemoryManifest;

/// Фактор понижения weight при импорте traces.
///
/// Загруженные traces получают weight × 0.7 — система должна подтвердить
/// опыт собственной обработкой перед полным усилением.
pub const IMPORT_WEIGHT_FACTOR: f32 = 0.7;

/// Результат загрузки.
pub struct LoadResult {
    /// Восстановленный engine
    pub engine: AxiomEngine,
    /// Manifest (статистика)
    pub manifest: MemoryManifest,
    /// Число импортированных traces
    pub traces_imported: u32,
    /// Число импортированных tension traces
    pub tension_imported: u32,
}

/// Загрузить состояние Engine из директории `dir`.
///
/// Алгоритм:
/// 1. Проверить manifest.yaml (наличие, версия)
/// 2. Прочитать engine_state.bin
/// 3. Восстановить Engine через `restore_from(snapshot)`
/// 4. Импортировать traces с weight × IMPORT_WEIGHT_FACTOR
/// 5. Импортировать tension traces
pub fn load(dir: &Path) -> Result<LoadResult, PersistError> {
    // 1. Manifest
    let manifest = MemoryManifest::load_from(dir)?;

    // 2. engine_state.bin
    let state_path = dir.join("engine_state.bin");
    if !state_path.exists() {
        return Err(PersistError::NotFound(state_path.display().to_string()));
    }
    let bytes = std::fs::read(&state_path)?;
    let (state, _): (StoredEngineState, _) = bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
        .map_err(|e| PersistError::Decode(e.to_string()))?;

    // 3. Восстановить токены/связи через EngineSnapshot
    let snapshot = state_to_snapshot(&state);
    let mut engine = AxiomEngine::restore_from(&snapshot);

    // 4. Импортировать Experience traces (с понижением weight)
    let traces_imported = state.traces.len() as u32;
    {
        let exp = engine.ashti.experience_mut();
        for stored in state.traces {
            let trace = apply_import_factor(stored);
            exp.import_trace(trace.into());
        }
    }

    // 5. Tension traces
    let tension_imported = state.tension.len() as u32;
    {
        let exp = engine.ashti.experience_mut();
        for stored in state.tension {
            exp.import_tension_trace(stored.into());
        }
    }

    Ok(LoadResult {
        engine,
        manifest,
        traces_imported,
        tension_imported,
    })
}

/// Конвертировать StoredEngineState → EngineSnapshot (без traces).
fn state_to_snapshot(state: &StoredEngineState) -> axiom_runtime::EngineSnapshot {
    use axiom_runtime::DomainSnapshot;
    use axiom_config::DomainConfig;

    let domains = state.domains.iter().map(|sd| DomainSnapshot {
        domain_id:   sd.domain_id as u16,
        config:      sd.config.unwrap_or_else(|| DomainConfig::factory_void(sd.domain_id as u16, 0)),
        tokens:      sd.tokens.clone(),
        connections: sd.connections.clone(),
    }).collect();

    axiom_runtime::EngineSnapshot {
        domains,
        com_next_id: state.com_next_id,
        tick_count:  state.tick_count,
        created_at:  0,
    }
}

/// Применить IMPORT_WEIGHT_FACTOR к весу следа.
fn apply_import_factor(mut stored: StoredTrace) -> StoredTrace {
    stored.weight = (stored.weight * IMPORT_WEIGHT_FACTOR).max(0.001);
    stored
}
