// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-domain — Домены и Ashti_Core
//
// Domain V1.3 + Event-Driven V1 + SPACE V6.0

pub mod ashti_core;
pub mod causal_horizon;
pub mod domain;
pub mod domain_state;
pub mod fractal_chain;
pub mod membrane;
pub mod physics;

pub use ashti_core::AshtiCore;
pub use causal_horizon::CausalHorizon;
pub use domain::Domain;
pub use domain_state::{CapacityExceeded, DomainState};
pub use fractal_chain::FractalChain;
pub use membrane::{can_enter_domain, can_exit_domain};
pub use physics::EventGenerator;

// Re-export из axiom-config для удобства пользователей axiom-domain
pub use axiom_config::{DomainConfig, DomainType, StructuralRole};
// HeartbeatConfig экспортируется из axiom_heartbeat (там живёт реализация)
pub use axiom_config::{DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY};
pub use axiom_config::{MEMBRANE_ADAPTIVE, MEMBRANE_CLOSED, MEMBRANE_OPEN, MEMBRANE_SEMI};
pub use axiom_config::{PROCESSING_ACTIVE, PROCESSING_FROZEN, PROCESSING_IDLE};
pub use axiom_heartbeat::HeartbeatConfig;
