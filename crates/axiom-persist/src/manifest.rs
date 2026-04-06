// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MemoryManifest — YAML-индекс хранилища.
//
// Обновляется ПОСЛЕДНИМ при записи — наличие валидного manifest означает
// что все файлы записаны корректно. При загрузке проверяется первым.

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::PersistError;

/// Текущая версия формата хранилища.
pub const FORMAT_VERSION: &str = "axiom-memory-v1";

/// Статистика содержимого хранилища.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestContents {
    /// Число доменов
    pub domains: u32,
    /// Суммарное число токенов по всем доменам
    pub tokens: u32,
    /// Суммарное число связей по всем доменам
    pub connections: u32,
    /// Число сохранённых Experience traces
    pub traces: u32,
    /// Число tension traces
    pub tension_traces: u32,
}

/// YAML-манифест хранилища.
///
/// Файл `manifest.yaml` — маркер успешного сохранения.
/// Если manifest отсутствует или повреждён → загрузка невозможна → чистый старт.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryManifest {
    /// Версия формата ("axiom-memory-v1")
    pub version: String,
    /// Дата создания хранилища (ISO 8601, human-readable)
    pub created_at: String,
    /// Дата последнего успешного сохранения
    pub last_saved: String,
    /// tick_count на момент сохранения
    pub tick_count: u64,
    /// com_next_id на момент сохранения
    pub com_next_id: u64,
    /// Версия AXIOM
    pub axiom_version: String,
    /// Статистика содержимого
    pub contents: ManifestContents,
}

impl MemoryManifest {
    /// Создать новый manifest с текущим временем.
    pub fn new(tick_count: u64, com_next_id: u64, contents: ManifestContents) -> Self {
        let now = chrono_like_now();
        Self {
            version: FORMAT_VERSION.to_string(),
            created_at: now.clone(),
            last_saved: now,
            tick_count,
            com_next_id,
            axiom_version: env!("CARGO_PKG_VERSION").to_string(),
            contents,
        }
    }

    /// Обновить `last_saved` и счётчики.
    pub fn update(&mut self, tick_count: u64, com_next_id: u64, contents: ManifestContents) {
        self.last_saved  = chrono_like_now();
        self.tick_count  = tick_count;
        self.com_next_id = com_next_id;
        self.contents    = contents;
    }

    /// Записать manifest в файл `<dir>/manifest.yaml`.
    pub fn write_to(&self, dir: &Path) -> Result<(), PersistError> {
        let path = dir.join("manifest.yaml");
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| PersistError::Encode(e.to_string()))?;
        std::fs::write(&path, yaml)?;
        Ok(())
    }

    /// Загрузить manifest из `<dir>/manifest.yaml`.
    pub fn load_from(dir: &Path) -> Result<Self, PersistError> {
        let path = dir.join("manifest.yaml");
        if !path.exists() {
            return Err(PersistError::NotFound(path.display().to_string()));
        }
        let content = std::fs::read_to_string(&path)?;
        let manifest: Self = serde_yaml::from_str(&content)
            .map_err(|e| PersistError::CorruptManifest(e.to_string()))?;

        if manifest.version != FORMAT_VERSION {
            return Err(PersistError::VersionMismatch {
                expected: FORMAT_VERSION,
                found: manifest.version.clone(),
            });
        }
        Ok(manifest)
    }
}

/// Простая метка времени без внешних зависимостей.
/// Формат: "YYYY-MM-DD" (достаточно для диагностики).
fn chrono_like_now() -> String {
    // Используем std::time для получения секунд с эпохи
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Простой расчёт даты из unix timestamp (без внешних зависимостей)
    let days_since_epoch = secs / 86400;
    let (year, month, day) = days_to_ymd(days_since_epoch as u32);
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn days_to_ymd(days: u32) -> (u32, u32, u32) {
    // Алгоритм Лиса (Julian Day)
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
