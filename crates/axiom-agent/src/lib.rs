//! AXIOM Agent — внешний слой интеграции.
//!
//! Реализует Perceptor/Effector архитектуру для взаимодействия AXIOM с внешним миром:
//! - CLI (stdin/stdout)
//! - Telegram Bot API
//! - Shell Effector (whitelist-защищённый)

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Каналы ввода/вывода агента
pub mod channels;
/// Конфигурация агента из channels.yaml
pub mod config;
/// ML inference (tract-onnx или mock)
pub mod ml;
/// Perceptors — преобразование внешних сигналов в UclCommand
pub mod perceptors;
/// Effectors — форматирование результатов ядра для вывода
pub mod effectors;
