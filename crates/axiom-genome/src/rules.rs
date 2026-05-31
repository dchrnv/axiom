// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::types::{DataType, ModuleId, Permission, ResourceId};
use serde::{Deserialize, Serialize};

/// Структурные инварианты системы — физические и архитектурные ограничения.
/// Эти значения никогда не меняются в рантайме.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenomeInvariants {
    /// Размеры core-структур в байтах
    pub token_size: u16, // Всегда 64
    pub connection_size: u16,    // Всегда 64
    pub event_size: u16,         // Всегда 32
    pub domain_config_size: u16, // Всегда 128

    /// Фундаментальные ограничения
    pub max_domains: u8, // 11 доменов в одном уровне Ashti_Core
    pub min_intensity_nonzero: bool, // min_intensity > 0 для EXPERIENCE(9)
    pub sutra_write_exclusive: bool, // Только SUTRA(0) имеет право WRITE на токены

    /// Временная модель
    pub no_wall_clock_in_core: bool, // Запрет std::time внутри ядра — всегда true
    pub event_id_monotonic: bool, // event_id строго возрастает — всегда true
}

impl GenomeInvariants {
    pub fn ashti_core_v2() -> Self {
        Self {
            token_size: 64,
            connection_size: 64,
            event_size: 32,
            domain_config_size: 128,
            max_domains: 11,
            min_intensity_nonzero: true,
            sutra_write_exclusive: true,
            no_wall_clock_in_core: true,
            event_id_monotonic: true,
        }
    }
}

/// Правило доступа: кто, к чему, что может.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AccessRule {
    pub module: ModuleId,
    pub resource: ResourceId,
    pub permission: Permission,
}

/// Правило протокола: допустимый маршрут данных между модулями.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ProtocolRule {
    pub source: ModuleId,
    pub target: ModuleId,
    pub data_type: DataType,
    pub mandatory: bool,
}

// serde default helpers
fn default_min_primitives() -> usize { 2 }
fn default_min_evidence() -> f32 { 0.3 }
fn default_require_review() -> bool { true }
fn default_min_co_activation() -> u32 { 50 }
fn default_max_bonds_total() -> Option<u32> { None }
fn default_allow_binding() -> bool { true }
fn default_allow_revocation() -> bool { true }
fn default_require_approval() -> bool { true }

/// Конфигурация cross-modal binding (CMB-TD-02).
///
/// Параметры из спеки Cross_Modal_Binding_V1_0.md §5.
/// Дефолты совпадают с константами в `cross_modal/candidate.rs`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrossModalConfig {
    /// Разрешить создание cross-modal bonds.
    #[serde(default = "default_allow_binding")]
    pub allow_binding: bool,
    /// Минимум ко-активаций пары Frame перед предложением bond.
    #[serde(default = "default_min_co_activation")]
    pub min_co_activation: u32,
    /// chrnv должен явно одобрить bond (всегда true в V1).
    #[serde(default = "default_require_approval")]
    pub require_chrnv_approval: bool,
    /// Максимум одновременных bonds (None = без ограничений).
    #[serde(default = "default_max_bonds_total")]
    pub max_bonds_total: Option<u32>,
    /// Разрешить отзыв bonds по stress-сигналу (stress-driven revocation — CMB-TD-01).
    #[serde(default = "default_allow_revocation")]
    pub allow_revocation: bool,
}

impl Default for CrossModalConfig {
    fn default() -> Self {
        Self {
            allow_binding: default_allow_binding(),
            min_co_activation: default_min_co_activation(),
            require_chrnv_approval: default_require_approval(),
            max_bonds_total: default_max_bonds_total(),
            allow_revocation: default_allow_revocation(),
        }
    }
}

/// Правила для emergent subsystem lifecycle (V7-D4).
///
/// GUARDIAN проверяет эти правила при одобрении SubsystemCandidate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmergentSubsystemRules {
    /// Минимум примитивов в кластере для создания кандидата.
    #[serde(default = "default_min_primitives")]
    pub min_primitives: usize,
    /// Минимальная evidence_strength для одобрения кандидата.
    #[serde(default = "default_min_evidence")]
    pub min_evidence_strength: f32,
    /// Оператор должен явно одобрить кандидата (всегда true в V7).
    #[serde(default = "default_require_review")]
    pub require_review: bool,
    /// Максимум одновременных активных кандидатов (None = без ограничений).
    #[serde(default)]
    pub max_active_candidates: Option<usize>,
}

impl Default for EmergentSubsystemRules {
    fn default() -> Self {
        Self {
            min_primitives: default_min_primitives(),
            min_evidence_strength: default_min_evidence(),
            require_review: default_require_review(),
            max_active_candidates: None,
        }
    }
}

/// Глобальная конфигурация Ashti_Core уровня.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenomeConfig {
    /// Arbiter
    pub arbiter_response_timeout: u64,
    pub arbiter_storm_threshold: u32,

    /// Frontier
    pub default_max_events_per_cycle: u32,
    pub default_storm_threshold: u32,

    /// Heartbeat
    pub default_heartbeat_interval: u32,

    /// Ashti_Core
    pub ashti_domain_count: u8,
}

impl GenomeConfig {
    pub fn ashti_core_v2() -> Self {
        Self {
            arbiter_response_timeout: 10_000,
            arbiter_storm_threshold: 100,
            default_max_events_per_cycle: 1_000,
            default_storm_threshold: 5_000,
            default_heartbeat_interval: 1_024,
            ashti_domain_count: 11,
        }
    }
}
