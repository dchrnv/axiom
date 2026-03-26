// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-domain — Домены и Ashti_Core
//
// Domain V1.3 + Event-Driven V1 + SPACE V6.0

pub mod domain;
pub mod domain_state;
pub mod membrane;
pub mod physics;

pub use domain::Domain;
pub use domain_state::{DomainState, CapacityExceeded};
pub use membrane::{can_enter_domain, can_exit_domain};
pub use physics::EventGenerator;

// Re-export из axiom-config для удобства пользователей axiom-domain
pub use axiom_config::{DomainConfig, StructuralRole, DomainType};
// HeartbeatConfig экспортируется из axiom_heartbeat (там живёт реализация)
pub use axiom_heartbeat::HeartbeatConfig;
pub use axiom_config::{DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY};
pub use axiom_config::{PROCESSING_IDLE, PROCESSING_ACTIVE, PROCESSING_FROZEN};
pub use axiom_config::{MEMBRANE_OPEN, MEMBRANE_SEMI, MEMBRANE_CLOSED, MEMBRANE_ADAPTIVE};
