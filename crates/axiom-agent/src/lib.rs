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
/// Standalone-функции мета-команд CLI (Phase 0B)
pub mod meta_commands;
/// Протокол обмена tick_loop ↔ адаптеры (Phase 0C)
pub mod protocol;
/// Команды адаптеров в tick_loop (Phase 0C)
pub mod adapter_command;
/// Конфигурация tick_loop и адаптеров (Phase 0C)
pub mod adapters_config;
/// Главный цикл — единственный writer AxiomEngine (Phase 0C)
pub mod tick_loop;
/// WebSocket-адаптер (Phase 1)
pub mod ws;
/// REST-адаптер (Phase 2)
pub mod rest;
