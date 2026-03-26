//! Configuration Loader for Axiom
//! Implements unified configuration loading and validation

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomConfig {
    pub runtime: RuntimeConfig,
    pub schema: SchemaConfig,
    pub loader: LoaderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub file: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaConfig {
    pub domain: String,
    pub token: String,
    pub connection: String,
    pub grid: String,
    pub upo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderConfig {
    pub format: String,
    pub validation: String,
    pub cache_enabled: bool,
    pub hot_reload: bool,
}

/// Configuration loading errors
#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(serde_yaml::Error),
    ValidationError(String),
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

/// Main configuration loader
pub struct ConfigLoader {
    cache: HashMap<String, serde_yaml::Value>,
}

impl ConfigLoader {
    /// Create new configuration loader
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Load root configuration
    pub fn load_root(&mut self, path: &Path) -> Result<AxiomConfig, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(ConfigError::IoError)?;
        
        let config: AxiomConfig = serde_yaml::from_str(&content)
            .map_err(ConfigError::ParseError)?;

        // Cache root config
        let cache_key = path.to_string_lossy().to_string();
        self.cache.insert(cache_key, serde_yaml::to_value(&config.clone()).unwrap());

        Ok(config)
    }

    /// Load runtime configuration
    pub fn load_runtime(&mut self, path: &Path) -> Result<serde_yaml::Value, ConfigError> {
        self.load_yaml_file(path)
    }

    /// Load schema configuration
    pub fn load_schema(&mut self, _schema_type: &str, path: &Path) -> Result<serde_yaml::Value, ConfigError> {
        self.load_yaml_file(path)
    }

    /// Load YAML file with caching
    fn load_yaml_file(&mut self, path: &Path) -> Result<serde_yaml::Value, ConfigError> {
        let cache_key = path.to_string_lossy().to_string();
        
        // Check cache first
        if let Some(value) = self.cache.get(&cache_key) {
            return Ok(value.clone());
        }

        // Load from file
        let content = fs::read_to_string(path)
            .map_err(ConfigError::IoError)?;
        
        let value: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(ConfigError::ParseError)?;

        // Cache the result
        self.cache.insert(cache_key, value.clone());

        Ok(value)
    }

    /// Validate configuration against schema
    pub fn validate(&self, config: &serde_yaml::Value, schema: &serde_yaml::Value) -> Result<(), ConfigError> {
        // Basic validation - in real implementation, use JSON Schema validation
        if let (Some(config_obj), Some(schema_obj)) = (config.as_mapping(), schema.as_mapping()) {
            // Check if schema has properties
            if let Some(properties) = schema_obj.get("properties") {
                if let Some(props_obj) = properties.as_mapping() {
                    for (key, schema_value) in props_obj {
                        if let Some(config_value) = config_obj.get(key) {
                            self.validate_field(key.as_str().unwrap(), config_value, schema_value)?;
                        }
                    }
                }
            } else {
                // Direct field validation for flat schemas
                for (key, schema_value) in schema_obj {
                    if let Some(config_value) = config_obj.get(key) {
                        self.validate_field(key.as_str().unwrap(), config_value, schema_value)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate individual field
    fn validate_field(&self, field: &str, value: &serde_yaml::Value, schema: &serde_yaml::Value) -> Result<(), ConfigError> {
        // Simple validation - extend with proper schema validation
        if let Some(required) = schema.get("required") {
            if required.as_bool().unwrap_or(false) && value.is_null() {
                return Err(ConfigError::ValidationError(format!("Field '{}' is required", field)));
            }
        }

        if let Some(type_) = schema.get("type") {
            let expected_type = type_.as_str().unwrap_or("string");
            if !self.check_type(value, expected_type) {
                return Err(ConfigError::ValidationError(format!(
                    "Field '{}' must be of type {}", field, expected_type
                )));
            }
        }

        // Check minimum/maximum constraints for numbers
        if value.is_number() {
            if let Some(minimum) = schema.get("minimum") {
                if let Some(min_val) = minimum.as_i64() {
                    if let Some(val) = value.as_i64() {
                        if val < min_val {
                            return Err(ConfigError::ValidationError(format!(
                                "Field '{}' must be >= {}", field, min_val
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
                                "Field '{}' must be <= {}", field, max_val
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check YAML value type
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
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
