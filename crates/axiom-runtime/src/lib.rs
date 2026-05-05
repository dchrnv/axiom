// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// AXIOM Runtime — оркестрация всей системы
//
// Главный цикл: UCL → COM → Frontier → State.
// Интеграция всех компонентов в единый Engine.

//! AXIOM Runtime — оркестрация всей системы: Engine, Guardian, Snapshot, Adapters.

#![deny(unsafe_code)]
#![allow(missing_docs)]

/// Adapters — trait-границы для внешних адаптеров
pub mod adapters;
/// AdaptiveTickRate — Variable Tick Rate (Axiom Sentinel V1.0, Фаза 3)
pub mod adaptive;
/// Broadcast-типы для внешних адаптеров (WebSocket, REST, egui).
/// Доступны только при feature "adapters".
#[cfg(feature = "adapters")]
pub mod broadcast;
/// Channel — in-process очередь команд и событий
pub mod channel;
/// Engine — центральный оркестратор
pub mod engine;
/// Gateway — единая точка входа для внешних запросов
pub mod gateway;
/// Guardian — надоменный контроль CODEX-правил
pub mod guardian;
mod orchestrator;
/// Over-Domain Layer: Guardians + Weavers (Over_Domain_Layer_V1_1.md)
pub mod over_domain;
/// ProcessingResult — диагностический результат process_and_observe()
pub mod result;
/// Snapshot — сохранение и восстановление состояния
pub mod snapshot;

pub use adapters::{DirectAdapter, Effector, EventBus, EventObserver, Perceptor, RuntimeAdapter};
pub use adaptive::{AdaptiveTickRate, TickRateReason};
#[cfg(feature = "adapters")]
pub use broadcast::{
    BroadcastSnapshot, ConnectionSnapshot, DomainDetailSnapshot, DomainSummary, TokenSnapshot,
};
pub use channel::{Channel, ChannelBatchResult};
pub use engine::{domain_name, AxiomEngine, AxiomError, TickSchedule};
pub use gateway::Gateway;
pub use guardian::{
    CodexAction, Guardian, GuardianConfig, GuardianError, GuardianStats, InhibitAction,
    InhibitReason, ReflexDecision, RoleStats, VetoReason,
};
pub use over_domain::{
    CrystallizationProposal, OverDomainComponent, OverDomainError, PromotionProposal, Weaver,
    WeaverId,
};
pub use over_domain::{
    DreamPhaseEvent, DreamPhaseState, DreamPhaseStats, GatewayPriority, SleepTrigger, WakeReason,
};
pub use over_domain::{
    DreamScheduler, DreamSchedulerConfig, DreamSchedulerStats, SleepDecision, SleepTriggerKind,
};
pub use over_domain::{FatigueSnapshot, FatigueTracker, FatigueWeights, IdleTracker};
pub use over_domain::{FrameWeaver, FrameWeaverConfig, FrameWeaverStats};
pub use result::{ProcessingPath, ProcessingResult};
pub use snapshot::{DomainSnapshot, EngineSnapshot};
