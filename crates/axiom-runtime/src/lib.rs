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
/// ProcessingResult — диагностический результат process_and_observe()
pub mod result;
/// AdaptiveTickRate — Variable Tick Rate (Axiom Sentinel V1.0, Фаза 3)
pub mod adaptive;
/// Over-Domain Layer: Guardians + Weavers (Over_Domain_Layer_V1_1.md)
pub mod over_domain;
/// Broadcast-типы для внешних адаптеров (WebSocket, REST, egui).
/// Доступны только при feature "adapters".
#[cfg(feature = "adapters")]
pub mod broadcast;

pub use engine::{AxiomEngine, AxiomError, TickSchedule, domain_name};
#[cfg(feature = "adapters")]
pub use broadcast::{BroadcastSnapshot, DomainSummary, DomainDetailSnapshot, TokenSnapshot, ConnectionSnapshot};
pub use result::{ProcessingResult, ProcessingPath};
pub use guardian::{
    Guardian, GuardianConfig, ReflexDecision, VetoReason,
    InhibitAction, InhibitReason,
    CodexAction, GuardianError, GuardianStats,
    RoleStats,
};
pub use snapshot::{EngineSnapshot, DomainSnapshot};
pub use adapters::{RuntimeAdapter, EventObserver, DirectAdapter, EventBus, Perceptor, Effector};
pub use gateway::Gateway;
pub use channel::{Channel, ChannelBatchResult};
pub use adaptive::{AdaptiveTickRate, TickRateReason};
pub use over_domain::{WeaverId, OverDomainError, OverDomainComponent, Weaver, CrystallizationProposal, PromotionProposal};
pub use over_domain::{FrameWeaver, FrameWeaverStats, FrameWeaverConfig};
