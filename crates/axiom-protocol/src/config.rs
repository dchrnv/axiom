/// C2: schema-driven Configuration UI types.
/// Engine returns ConfigSchema in response to GetConfigSchema; Workstation
/// renders the Configuration tab entirely from this schema — no hardcoded UI.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigSchema {
    pub sections: Vec<ConfigSection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigSection {
    /// Stable dot-separated id, e.g. "engine.dream_phase".
    pub id: String,
    pub label: String,
    pub category: ConfigCategory,
    pub fields: Vec<ConfigField>,
    /// Nested subsections, e.g. Engine → Domains → LOGIC.
    pub subsections: Vec<ConfigSection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigCategory {
    Engine,
    Workstation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigField {
    /// Stable id within the section, maps to the actual config key in Engine.
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub field_type: ConfigFieldType,
    pub current_value: ConfigValue,
    pub default_value: ConfigValue,
    pub hot_reloadable: bool,
    /// GENOME fields are readonly — cannot be changed at runtime.
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConfigFieldType {
    Bool,
    Integer { min: i64, max: i64 },
    UInt { min: u64, max: u64 },
    /// `step` is optional; if None, UI uses a plain numeric input field.
    Float { min: f64, max: f64, step: Option<f64> },
    String { max_length: u32 },
    Enum { variants: Vec<String> },
    /// Value is stored as tick count or nanoseconds depending on context.
    Duration,
    /// Domain id selector in range 100-110.
    Domain,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Bool(bool),
    Integer(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumVariant(String),
    Duration(u64),
    Domain(u16),
}
