// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ConfigWatcher — горячая перезагрузка конфигурации через inotify/FSEvents
//
// Следит за axiom.yaml. При изменении перечитывает конфигурацию (домены,
// HeartbeatConfig, пресеты). GENOME не перезагружается никогда — он не является
// частью LoadedAxiomConfig.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use notify::{Watcher, RecommendedWatcher, RecursiveMode};

use crate::loader::{ConfigLoader, LoadedAxiomConfig, ConfigError};

/// Горячая перезагрузка конфигурации.
///
/// Следит за файлом `axiom.yaml`. Когда файл изменяется — перечитывает
/// конфигурацию через [`ConfigLoader::load_all`] и возвращает её через [`poll`].
///
/// **GENOME-инвариант:** `ConfigLoader::load_all` не читает `genome.yaml`,
/// поэтому GENOME никогда не затрагивается при горячей перезагрузке.
///
/// # Пример
///
/// ```rust,ignore
/// let mut watcher = ConfigWatcher::new("config/axiom.yaml")?;
/// loop {
///     if let Some(cfg) = watcher.poll() {
///         // применить новую конфигурацию
///     }
///     std::thread::sleep(Duration::from_millis(500));
/// }
/// ```
pub struct ConfigWatcher {
    /// Notify watcher — держит фоновый поток живым
    _watcher: RecommendedWatcher,
    /// Канал событий от notify
    rx: Receiver<notify::Result<notify::Event>>,
    /// Путь к корневому axiom.yaml
    config_path: PathBuf,
}

impl ConfigWatcher {
    /// Создать наблюдатель за файлом конфигурации.
    ///
    /// Запускает inotify (Linux) / FSEvents (macOS) / ReadDirectoryChanges (Windows)
    /// в фоновом потоке через crate `notify`.
    ///
    /// # Ошибки
    /// Возвращает `ConfigError::ValidationError` если не удалось подписаться на события.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)
            .map_err(|e| ConfigError::ValidationError(format!("notify init: {e}")))?;

        // Следим за родительской директорией — надёжнее чем за файлом напрямую
        // (некоторые редакторы удаляют + пересоздают файл при сохранении)
        let watch_dir = path.parent().unwrap_or(Path::new("."));
        watcher
            .watch(watch_dir, RecursiveMode::NonRecursive)
            .map_err(|e| ConfigError::ValidationError(format!("notify watch: {e}")))?;

        Ok(Self {
            _watcher: watcher,
            rx,
            config_path: path.to_path_buf(),
        })
    }

    /// Проверить наличие изменений (неблокирующий вызов).
    ///
    /// Возвращает новую конфигурацию если файл изменился с последнего вызова.
    /// Возвращает `None` если изменений не было или перезагрузка не удалась.
    ///
    /// **Важно:** множественные события за один интервал сворачиваются в одну перезагрузку.
    pub fn poll(&self) -> Option<LoadedAxiomConfig> {
        let mut changed = false;
        while let Ok(event) = self.rx.try_recv() {
            if let Ok(ev) = event {
                // Интересуют только изменения файла с нашим именем
                let affects_config = ev.paths.iter().any(|p| {
                    p.file_name() == self.config_path.file_name()
                });
                if affects_config {
                    changed = true;
                }
            }
        }
        if changed {
            ConfigLoader::new().load_all(&self.config_path).ok()
        } else {
            None
        }
    }

    /// Путь к отслеживаемому файлу конфигурации.
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}
