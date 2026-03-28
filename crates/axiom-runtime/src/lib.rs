// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AXIOM Runtime — оркестрация всей системы
//
// Главный цикл: UCL → COM → Frontier → State.
// Интеграция всех компонентов в единый Engine.

//! AXIOM Runtime — оркестрация всей системы: Engine, Guardian, Snapshot, Adapters.

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Engine — центральный оркестратор
pub mod engine;
/// Guardian — надоменный контроль CODEX-правил
pub mod guardian;
/// Snapshot — сохранение и восстановление состояния
pub mod snapshot;
/// Adapters — trait-границы для внешних адаптеров
pub mod adapters;
/// Gateway — единая точка входа для внешних запросов
pub mod gateway;
/// Channel — in-process очередь команд и событий
pub mod channel;
mod orchestrator;

pub use engine::{AxiomEngine, AxiomError};
pub use guardian::{
    Guardian, ReflexDecision, VetoReason,
    InhibitAction, InhibitReason,
    CodexAction, GuardianError, GuardianStats,
    RoleStats,
};
pub use snapshot::{EngineSnapshot, DomainSnapshot};
pub use adapters::{RuntimeAdapter, EventObserver, DirectAdapter};
pub use gateway::Gateway;
pub use channel::{Channel, ChannelBatchResult};
