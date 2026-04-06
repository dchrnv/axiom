// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MemoryWriter — сериализация состояния Engine на диск.

use std::path::Path;
use axiom_runtime::AxiomEngine;
use crate::error::PersistError;
use crate::format::{StoredDomain, StoredEngineState, StoredTrace, StoredTensionTrace};
use crate::manifest::{ManifestContents, MemoryManifest};

/// Параметры записи.
pub struct WriteOptions {
    /// Минимальный weight trace для сохранения (0.0 = сохранять все).
    pub trace_weight_threshold: f32,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            trace_weight_threshold: 0.0,
        }
    }
}

/// Записывает состояние Engine в директорию `dir`.
///
/// Создаёт директорию если не существует.
/// Порядок записи: engine_state.json → manifest.yaml (последним).
/// Если запись прервана до manifest → при загрузке чистый старт.
pub fn save(engine: &AxiomEngine, dir: &Path, opts: &WriteOptions) -> Result<MemoryManifest, PersistError> {
    std::fs::create_dir_all(dir)?;

    let snapshot = engine.snapshot();
    let experience = engine.ashti.experience();

    // Домены
    let domains: Vec<StoredDomain> = snapshot.domains.iter().map(|ds| StoredDomain {
        domain_id:   ds.domain_id,
        tokens:      ds.tokens.clone(),
        connections: ds.connections.clone(),
    }).collect();

    // Experience traces с фильтром по threshold
    let traces: Vec<StoredTrace> = experience.traces()
        .iter()
        .filter(|t| t.weight >= opts.trace_weight_threshold)
        .map(StoredTrace::from)
        .collect();

    // Tension traces
    let tension: Vec<StoredTensionTrace> = experience.tension_traces()
        .iter()
        .map(StoredTensionTrace::from)
        .collect();

    // Статистика для manifest
    let contents = ManifestContents {
        domains:        domains.len() as u32,
        tokens:         domains.iter().map(|d| d.tokens.len() as u32).sum(),
        connections:    domains.iter().map(|d| d.connections.len() as u32).sum(),
        traces:         traces.len() as u32,
        tension_traces: tension.len() as u32,
    };

    let state = StoredEngineState {
        tick_count:  snapshot.tick_count,
        com_next_id: snapshot.com_next_id,
        domains,
        traces,
        tension,
    };

    // Атомарная запись engine_state.json:
    //   1. Пишем во временный файл
    //   2. Переименовываем (atomic на Linux)
    //   3. Manifest — последним (маркер успешной записи)
    let json = serde_json::to_string(&state)
        .map_err(|e| PersistError::Encode(e.to_string()))?;
    let tmp_path = dir.join("engine_state.json.tmp");
    std::fs::write(&tmp_path, json.as_bytes())?;
    std::fs::rename(&tmp_path, dir.join("engine_state.json"))?;

    // Manifest обновляется последним — маркер успешной записи
    let manifest = MemoryManifest::new(snapshot.tick_count, snapshot.com_next_id, contents);
    manifest.write_to(dir)?;

    Ok(manifest)
}
