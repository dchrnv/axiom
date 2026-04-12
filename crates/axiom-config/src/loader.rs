//! Configuration Loader
//!
//! Реализует единую систему загрузки и валидации конфигураций для Axiom

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::domain_config::DomainConfig;
use crate::heartbeat_config::HeartbeatConfig;
use crate::preset::{TokenPreset, ConnectionPreset};

/// Корневая конфигурация Axiom
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AxiomConfig {
    /// Конфигурация runtime
    pub runtime: RuntimeConfig,
    /// Конфигурация схем
    pub schema: SchemaConfig,
    /// Конфигурация загрузчика
    pub loader: LoaderConfig,
    /// Пресеты (опционально — для совместимости со старыми axiom.yaml)
    #[serde(default)]
    pub presets: PresetsConfig,
}

/// Конфигурация пресетов — пути к готовым конфигурациям компонентов
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct PresetsConfig {
    /// Директория YAML пресетов доменов
    pub domains_dir: Option<String>,
    /// Директория YAML пресетов токенов
    pub tokens_dir: Option<String>,
    /// Директория YAML пресетов связей
    pub connections_dir: Option<String>,
    /// Путь к YAML конфигурации пространственного грида
    pub spatial: Option<String>,
    /// Путь к YAML таблице семантических вкладов Shell
    pub semantic_contributions: Option<String>,
    /// Путь к YAML файлу HeartbeatConfig (опционально)
    pub heartbeat_file: Option<String>,
}

/// Результат полной загрузки конфигурации через `ConfigLoader::load_all`
///
/// Содержит корневую конфигурацию и все загруженные компоненты.
/// Конкретные типы (SpatialConfig, SemanticContributionTable) загружаются
/// через методы соответствующих крейтов по путям из `AxiomConfig::presets`.
#[derive(Debug)]
pub struct LoadedAxiomConfig {
    /// Корневая конфигурация из axiom.yaml
    pub root: AxiomConfig,
    /// Загруженные домены (domain_name → raw YAML value)
    pub domains: HashMap<String, DomainConfig>,
    /// Heartbeat конфигурация (если загружена)
    pub heartbeat: Option<HeartbeatConfig>,
}

/// Конфигурация runtime
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RuntimeConfig {
    /// Путь к файлу runtime конфигурации
    pub file: String,
    /// Путь к схеме runtime конфигурации
    pub schema: String,
}

/// Конфигурация схем
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SchemaConfig {
    /// Путь к схеме домена
    pub domain: String,
    /// Путь к схеме токена
    pub token: String,
    /// Путь к схеме связи
    pub connection: String,
    /// Путь к схеме сетки
    pub grid: String,
    /// Путь к схеме UPO
    pub upo: String,
}

/// Конфигурация загрузчика
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoaderConfig {
    /// Формат конфигурационных файлов
    pub format: String,
    /// Режим валидации
    pub validation: String,
    /// Включить кэширование загруженных конфигураций
    pub cache_enabled: bool,
}

/// Ошибки загрузки конфигурации
#[derive(Debug)]
pub enum ConfigError {
    /// Ошибка ввода/вывода при чтении файла
    IoError(std::io::Error),
    /// Ошибка разбора YAML
    ParseError(serde_yaml::Error),
    /// Ошибка валидации конфигурации
    ValidationError(String),
    /// Конфигурационный файл не найден
    MissingFile(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Parse error: {}", e),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ConfigError::MissingFile(file) => write!(f, "Missing file: {}", file),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Основной загрузчик конфигураций
pub struct ConfigLoader {
    /// Кэш загруженных конфигураций
    pub cache: HashMap<String, serde_yaml::Value>,
}

impl ConfigLoader {
    /// Создать новый загрузчик
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Загрузить корневую конфигурацию
    pub fn load_root(&mut self, path: &Path) -> Result<AxiomConfig, ConfigError> {
        let content = fs::read_to_string(path).map_err(ConfigError::IoError)?;

        let config: AxiomConfig = crate::schema::validate_yaml(&content)?;

        // Кэшируем корневую конфигурацию
        let cache_key = path.to_string_lossy().to_string();
        self.cache.insert(
            cache_key,
            serde_yaml::to_value(&config).map_err(ConfigError::ParseError)?,
        );

        Ok(config)
    }

    /// Загрузить runtime конфигурацию
    pub fn load_runtime(&mut self, path: &Path) -> Result<serde_yaml::Value, ConfigError> {
        self.load_yaml_file(path)
    }

    /// Загрузить схему конфигурации
    pub fn load_schema(
        &mut self,
        _schema_type: &str,
        path: &Path,
    ) -> Result<serde_yaml::Value, ConfigError> {
        self.load_yaml_file(path)
    }

    /// Загрузить DomainConfig из YAML
    pub fn load_domain_config(&mut self, path: &Path) -> Result<DomainConfig, ConfigError> {
        let content = fs::read_to_string(path).map_err(ConfigError::IoError)?;
        let config: DomainConfig = crate::schema::validate_yaml(&content)?;

        config.validate().map_err(ConfigError::ValidationError)?;

        Ok(config)
    }

    /// Загрузить HeartbeatConfig из YAML
    pub fn load_heartbeat_config(&mut self, path: &Path) -> Result<HeartbeatConfig, ConfigError> {
        let content = fs::read_to_string(path).map_err(ConfigError::IoError)?;
        let config: HeartbeatConfig = crate::schema::validate_yaml(&content)?;

        config.validate().map_err(ConfigError::ValidationError)?;

        Ok(config)
    }

    /// Загрузить все пресеты токенов из директории.
    ///
    /// Читает все `*.yaml` файлы из `dir`. Имя файла (без расширения) становится
    /// полем `TokenPreset::name`. Возвращает пустой вектор если директория не существует.
    pub fn load_token_presets(&mut self, dir: &Path) -> Result<Vec<TokenPreset>, ConfigError> {
        self.load_presets_from_dir(dir, |name, content| {
            let mut preset: TokenPreset =
                serde_yaml::from_str(content).map_err(ConfigError::ParseError)?;
            preset.name = name.to_string();
            Ok(preset)
        })
    }

    /// Загрузить все пресеты связей из директории.
    ///
    /// Читает все `*.yaml` файлы из `dir`. Имя файла (без расширения) становится
    /// полем `ConnectionPreset::name`. Возвращает пустой вектор если директория не существует.
    pub fn load_connection_presets(&mut self, dir: &Path) -> Result<Vec<ConnectionPreset>, ConfigError> {
        self.load_presets_from_dir(dir, |name, content| {
            let mut preset: ConnectionPreset =
                serde_yaml::from_str(content).map_err(ConfigError::ParseError)?;
            preset.name = name.to_string();
            Ok(preset)
        })
    }

    /// Вспомогательный метод: читает все *.yaml из директории и применяет `parse_fn`.
    fn load_presets_from_dir<T, F>(&mut self, dir: &Path, parse_fn: F) -> Result<Vec<T>, ConfigError>
    where
        F: Fn(&str, &str) -> Result<T, ConfigError>,
    {
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut presets = Vec::new();
        let mut entries: Vec<_> = fs::read_dir(dir)
            .map_err(ConfigError::IoError)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().and_then(|x| x.to_str()) == Some("yaml")
            })
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let content = fs::read_to_string(&path).map_err(ConfigError::IoError)?;
            let preset = parse_fn(&name, &content)?;
            presets.push(preset);
        }
        Ok(presets)
    }

    /// Загрузить YAML файл с кэшированием
    fn load_yaml_file(&mut self, path: &Path) -> Result<serde_yaml::Value, ConfigError> {
        let cache_key = path.to_string_lossy().to_string();

        // Проверяем кэш
        if let Some(value) = self.cache.get(&cache_key) {
            return Ok(value.clone());
        }

        // Загружаем из файла
        let content = fs::read_to_string(path).map_err(ConfigError::IoError)?;

        let value: serde_yaml::Value =
            serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;

        // Кэшируем результат
        self.cache.insert(cache_key, value.clone());

        Ok(value)
    }

    /// Валидация конфигурации против схемы
    pub fn validate(
        &self,
        config: &serde_yaml::Value,
        schema: &serde_yaml::Value,
    ) -> Result<(), ConfigError> {
        // Базовая валидация
        if let (Some(config_obj), Some(schema_obj)) = (config.as_mapping(), schema.as_mapping()) {
            // Проверяем properties в схеме
            if let Some(properties) = schema_obj.get("properties") {
                if let Some(props_obj) = properties.as_mapping() {
                    for (key, schema_value) in props_obj {
                        let Some(key_str) = key.as_str() else { continue; };
                        if let Some(config_value) = config_obj.get(key) {
                            self.validate_field(key_str, config_value, schema_value)?;
                        }
                    }
                }
            } else {
                // Прямая валидация для плоских схем
                for (key, schema_value) in schema_obj {
                    let Some(key_str) = key.as_str() else { continue; };
                    if let Some(config_value) = config_obj.get(key) {
                        self.validate_field(key_str, config_value, schema_value)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Валидация отдельного поля
    fn validate_field(
        &self,
        field: &str,
        value: &serde_yaml::Value,
        schema: &serde_yaml::Value,
    ) -> Result<(), ConfigError> {
        // Проверка required
        if let Some(required) = schema.get("required") {
            if required.as_bool().unwrap_or(false) && value.is_null() {
                return Err(ConfigError::ValidationError(format!(
                    "Field '{}' is required",
                    field
                )));
            }
        }

        // Проверка типа
        if let Some(type_) = schema.get("type") {
            let expected_type = type_.as_str().unwrap_or("string");
            if !self.check_type(value, expected_type) {
                return Err(ConfigError::ValidationError(format!(
                    "Field '{}' must be of type {}",
                    field, expected_type
                )));
            }
        }

        // Проверка minimum/maximum для чисел
        if value.is_number() {
            if let Some(minimum) = schema.get("minimum") {
                if let Some(min_val) = minimum.as_i64() {
                    if let Some(val) = value.as_i64() {
                        if val < min_val {
                            return Err(ConfigError::ValidationError(format!(
                                "Field '{}' must be >= {}",
                                field, min_val
                            )));
                        }
                    }
                }
            }

            if let Some(maximum) = schema.get("maximum") {
                if let Some(max_val) = maximum.as_i64() {
                    if let Some(val) = value.as_i64() {
                        if val > max_val {
                            return Err(ConfigError::ValidationError(format!(
                                "Field '{}' must be <= {}",
                                field, max_val
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Проверка типа YAML значения
    fn check_type(&self, value: &serde_yaml::Value, expected: &str) -> bool {
        match expected {
            "string" => value.is_string(),
            "integer" => value.is_i64(),
            "number" => value.is_number(),
            "boolean" => value.is_bool(),
            "array" => value.is_sequence(),
            "object" => value.is_mapping(),
            _ => true,
        }
    }

    /// Загрузить все конфигурации из корневого axiom.yaml
    ///
    /// Читает `axiom.yaml`, загружает все компоненты которые в нём указаны.
    /// Если `presets.domains_dir` задан — загружает все YAML из этой директории.
    /// Все файлы кэшируются.
    ///
    /// # Arguments
    /// * `root_path` — путь к axiom.yaml
    ///
    /// # Returns
    /// `LoadedAxiomConfig` со всеми загруженными данными.
    pub fn load_all(&mut self, root_path: &Path) -> Result<LoadedAxiomConfig, ConfigError> {
        let root = self.load_root(root_path)?;

        let base = root_path.parent().unwrap_or(Path::new("."));

        // Загружаем домены из директории пресетов
        let mut domains = HashMap::new();
        if let Some(ref dir) = root.presets.domains_dir {
            let dir_path = base.join(dir);
            if dir_path.exists() {
                for entry in fs::read_dir(&dir_path).map_err(ConfigError::IoError)? {
                    let entry = entry.map_err(ConfigError::IoError)?;
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();
                        let config = self.load_domain_config(&path)?;
                        domains.insert(name, config);
                    }
                }
            }
        }

        // Верифицируем путь к spatial (если задан) — загружаем в кэш
        if let Some(ref spatial_path) = root.presets.spatial {
            let path = base.join(spatial_path);
            if path.exists() {
                self.load_yaml_file(&path)?;
            }
        }

        // Верифицируем путь к semantic_contributions (если задан) — загружаем в кэш
        if let Some(ref semantic_path) = root.presets.semantic_contributions {
            let path = base.join(semantic_path);
            if path.exists() {
                self.load_yaml_file(&path)?;
            }
        }

        // Загружаем HeartbeatConfig если путь задан
        let heartbeat = if let Some(ref hb_path) = root.presets.heartbeat_file.clone() {
            let path = base.join(hb_path);
            if path.exists() {
                Some(self.load_heartbeat_config(&path)?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(LoadedAxiomConfig {
            root,
            domains,
            heartbeat,
        })
    }

    /// Получить путь к spatial конфигурации из LoadedAxiomConfig
    ///
    /// Возвращает абсолютный путь для последующей загрузки через `SpatialConfig::from_yaml`.
    pub fn spatial_config_path<'a>(
        &self,
        loaded: &'a LoadedAxiomConfig,
        base: &Path,
    ) -> Option<std::path::PathBuf> {
        loaded
            .root
            .presets
            .spatial
            .as_ref()
            .map(|p| base.join(p))
    }

    /// Получить путь к semantic_contributions конфигурации из LoadedAxiomConfig
    ///
    /// Возвращает абсолютный путь для последующей загрузки через
    /// `SemanticContributionTable::from_yaml`.
    pub fn semantic_contributions_path<'a>(
        &self,
        loaded: &'a LoadedAxiomConfig,
        base: &Path,
    ) -> Option<std::path::PathBuf> {
        loaded
            .root
            .presets
            .semantic_contributions
            .as_ref()
            .map(|p| base.join(p))
    }

    /// Очистить кэш
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
