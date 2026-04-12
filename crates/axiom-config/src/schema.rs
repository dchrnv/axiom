// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// schema.rs — генерация JSON-схем и валидация конфигов.
//
// Использование:
//   schema::validate_yaml::<AxiomConfig>(yaml_str)?;   // при загрузке
//   schema::axiom_schema_json()                         // для --dump-schema

use serde::de::DeserializeOwned;
use schemars::JsonSchema;
use crate::loader::ConfigError;

/// Сгенерировать JSON-схему для типа `T` в виде строки.
pub fn schema_json<T: JsonSchema>() -> String {
    let schema = schemars::schema_for!(T);
    serde_json::to_string_pretty(&schema)
        .unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

/// JSON-схема корневого конфига axiom.yaml.
pub fn axiom_schema_json() -> String {
    schema_json::<crate::loader::AxiomConfig>()
}

/// JSON-схема DomainConfig (для файлов в presets/domains/).
pub fn domain_schema_json() -> String {
    schema_json::<crate::domain_config::DomainConfig>()
}

/// JSON-схема HeartbeatConfig.
pub fn heartbeat_schema_json() -> String {
    schema_json::<crate::heartbeat_config::HeartbeatConfig>()
}

/// Распарсить YAML-строку как `T`, предварительно провалидировав против JSON-схемы.
///
/// При несоответствии схеме возвращает `ConfigError::ValidationError` с подробным
/// описанием всех нарушений (путь к полю + причина).
///
/// При несоответствии типа/структуры — `ConfigError::ParseError`.
pub fn validate_yaml<T>(content: &str) -> Result<T, ConfigError>
where
    T: JsonSchema + DeserializeOwned,
{
    // 1. Парсим YAML → serde_yaml::Value (базовая структурная проверка)
    let yaml_val: serde_yaml::Value = serde_yaml::from_str(content)
        .map_err(ConfigError::ParseError)?;

    // 2. Конвертируем в serde_json::Value для jsonschema-валидатора
    let json_val: serde_json::Value = serde_json::to_value(&yaml_val)
        .map_err(|e| ConfigError::ValidationError(format!("yaml→json conversion: {e}")))?;

    // 3. Генерируем схему и конвертируем в serde_json::Value
    let schema = schemars::schema_for!(T);
    let schema_val: serde_json::Value = serde_json::to_value(&schema)
        .map_err(|e| ConfigError::ValidationError(format!("schema serialization: {e}")))?;

    // 4. Валидация — собираем все ошибки
    let validator = jsonschema::Validator::new(&schema_val)
        .map_err(|e| ConfigError::ValidationError(format!("schema compile: {e}")))?;

    let errors: Vec<String> = validator
        .iter_errors(&json_val)
        .map(|e| format!("{} (at {})", e, e.instance_path()))
        .collect();

    if !errors.is_empty() {
        return Err(ConfigError::ValidationError(errors.join("; ")));
    }

    // 5. Десериализуем в конкретный тип
    serde_yaml::from_value(yaml_val).map_err(ConfigError::ParseError)
}
