//! AXIOM Agent — внешний слой интеграции.
//!
//! Реализует Perceptor/Effector архитектуру для взаимодействия AXIOM с внешним миром:
//! - CLI (stdin/stdout)
//! - Telegram Bot API
//! - Shell Effector (whitelist-защищённый)

#![deny(unsafe_code)]
#![allow(missing_docs)]

/// Команды адаптеров в tick_loop (Phase 0C)
pub mod adapter_command;
/// Конфигурация tick_loop и адаптеров (Phase 0C)
pub mod adapters_config;
/// Каналы ввода/вывода агента
pub mod channels;
/// Конфигурация агента из channels.yaml
pub mod config;
/// Effectors — форматирование результатов ядра для вывода
pub mod effectors;
/// Standalone-функции мета-команд CLI (Phase 0B)
pub mod meta_commands;
/// ML inference (tract-onnx или mock)
pub mod ml;
/// OpenSearch-адаптер (Phase 5, feature = "opensearch")
#[cfg(feature = "opensearch")]
pub mod opensearch;
/// Perceptors — преобразование внешних сигналов в UclCommand
pub mod perceptors;
/// Протокол обмена tick_loop ↔ адаптеры (Phase 0C)
pub mod protocol;
/// REST-адаптер (Phase 2)
pub mod rest;
/// Telegram-адаптер (Phase 4, feature = "telegram")
#[cfg(feature = "telegram")]
pub mod telegram;
/// Главный цикл — единственный writer AxiomEngine (Phase 0C)
pub mod tick_loop;
/// WebSocket-адаптер (Phase 1)
pub mod ws;
