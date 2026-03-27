//! AXIOM Frontier — причинная граница
//!
//! Causal Frontier V1: очередь событий, storm detection, causal budget, processor.
//!
//! # Архитектура
//!
//! - `frontier` — CausalFrontier структура с типизированными очередями
//! - `processor` — FrontierProcessor с трейтом LocalRules для обработки
//! - Дедупликация через visited tracking (BitVec)
//! - Storm detection при превышении порога
//! - Causal budget для ограничения обработки за цикл
//!
//! # Storm Detection
//!
//! Когда `frontier.size() > storm_threshold`, активируется Storm режим.
//! Mitigation через causal budget и batch processing.
//!
//! # Processor
//!
//! Основной цикл: `pop → evaluate_local_rules → transform → push affected_neighbors`.
//! Реализация локальных правил через trait `LocalRules`.

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod frontier;
pub mod processor;

pub use frontier::{
    CausalFrontier, EntityQueue, FrontierConfig, FrontierEntity, FrontierState, StormMetrics,
};
pub use processor::{EvaluationResult, FrontierProcessor, LocalRules};
