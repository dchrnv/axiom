//! AXIOM Config — система конфигурации
//!
//! Загрузка, парсинг и валидация конфигураций из YAML.
//!
//! # Модули
//!
//! - `domain_config` — DomainConfig (128 байт) с 5 блоками конфигурации
//! - `heartbeat_config` — HeartbeatConfig для периодической активации
//! - `loader` — ConfigLoader для загрузки и валидации YAML
//!
//! # Примеры
//!
//! ```
//! use axiom_config::{DomainConfig, HeartbeatConfig, StructuralRole, DomainType};
//!
//! // Создание DomainConfig
//! let config = DomainConfig::new(1, DomainType::Logic, StructuralRole::Logic);
//! assert_eq!(config.domain_id, 1);
//!
//! // Создание HeartbeatConfig
//! let heartbeat = HeartbeatConfig::medium();
//! assert_eq!(heartbeat.interval, 1024);
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod domain_config;
pub mod heartbeat_config;
pub mod loader;
/// Пресеты токенов и связей
pub mod preset;

pub use domain_config::{
    DomainConfig, DomainType, StructuralRole,
    DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY,
    PROCESSING_ACTIVE, PROCESSING_FROZEN, PROCESSING_IDLE,
    MEMBRANE_OPEN, MEMBRANE_SEMI, MEMBRANE_CLOSED, MEMBRANE_ADAPTIVE,
    GUARDIAN_CHECK_REQUIRED,
};
pub use heartbeat_config::HeartbeatConfig;
pub use loader::{
    AxiomConfig, ConfigError, ConfigLoader, LoaderConfig, LoadedAxiomConfig,
    PresetsConfig, RuntimeConfig, SchemaConfig,
};
pub use preset::{TokenPreset, ConnectionPreset};
