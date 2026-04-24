// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::types::{ModuleId, ResourceId, Permission, DataType};
use crate::rules::{AccessRule, GenomeConfig, GenomeInvariants, ProtocolRule};

/// Ошибки валидации GENOME.
#[derive(Debug, PartialEq)]
pub enum GenomeError {
    InvariantViolation(&'static str),
    MissingMandatoryProtocol(&'static str),
    InvalidConfig(&'static str),
    MissingGuardianAccess,
}

impl std::fmt::Display for GenomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenomeError::InvariantViolation(field) =>
                write!(f, "Genome invariant violation: {field}"),
            GenomeError::MissingMandatoryProtocol(route) =>
                write!(f, "Missing mandatory protocol: {route}"),
            GenomeError::InvalidConfig(field) =>
                write!(f, "Invalid genome config: {field}"),
            GenomeError::MissingGuardianAccess =>
                write!(f, "GUARDIAN must have ReadWrite access to CODEX"),
        }
    }
}

/// Конституция системы AXIOM.
///
/// Загружается первым, до COM, до доменов, до любых событий.
/// После загрузки и валидации замораживается в `Arc<Genome>`.
/// Никто не может получить `&mut Genome`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub version: u32,
    pub invariants: GenomeInvariants,
    pub access_rules: Vec<AccessRule>,
    pub protocol_rules: Vec<ProtocolRule>,
    pub config: GenomeConfig,
}

impl Genome {
    /// Захардкоженная конфигурация Ashti_Core V2.0 (Фаза A — без serde).
    ///
    /// Соответствует конституции системы как она определена в спеке GENOME V1.0.
    pub fn default_ashti_core() -> Self {
        use ModuleId as M;
        use ResourceId as R;
        use Permission as P;
        use DataType as D;

        let access_rules = vec![
            // Arbiter
            AccessRule { module: M::Arbiter, resource: R::SutraTokens,      permission: P::Read    },
            AccessRule { module: M::Arbiter, resource: R::AshtiField,        permission: P::Execute },
            AccessRule { module: M::Arbiter, resource: R::ExperienceMemory,  permission: P::Read    },
            AccessRule { module: M::Arbiter, resource: R::MayaOutput,        permission: P::Execute },
            AccessRule { module: M::Arbiter, resource: R::CodexRules,        permission: P::Read    },
            AccessRule { module: M::Arbiter, resource: R::GenomeConfig,      permission: P::Read    },
            // Guardian
            AccessRule { module: M::Guardian, resource: R::SutraTokens,      permission: P::Read      },
            AccessRule { module: M::Guardian, resource: R::AshtiField,        permission: P::Control   },
            AccessRule { module: M::Guardian, resource: R::ExperienceMemory,  permission: P::Control   },
            AccessRule { module: M::Guardian, resource: R::MayaOutput,        permission: P::Read      },
            AccessRule { module: M::Guardian, resource: R::CodexRules,        permission: P::ReadWrite },
            AccessRule { module: M::Guardian, resource: R::GenomeConfig,      permission: P::Read      },
            // Heartbeat
            AccessRule { module: M::Heartbeat, resource: R::AshtiField,       permission: P::Read },
            AccessRule { module: M::Heartbeat, resource: R::ExperienceMemory, permission: P::Read },
            AccessRule { module: M::Heartbeat, resource: R::GenomeConfig,     permission: P::Read },
            // Shell
            AccessRule { module: M::Shell, resource: R::AshtiField,           permission: P::Read },
            AccessRule { module: M::Shell, resource: R::ExperienceMemory,     permission: P::Read },
            // Adapters
            AccessRule { module: M::Adapters, resource: R::MayaOutput,        permission: P::Read },
            // FrameWeaver — читает MAYA, пишет в EXPERIENCE; SUTRA только через CODEX (Control)
            AccessRule { module: M::FrameWeaver, resource: R::MayaOutput,        permission: P::Read      },
            AccessRule { module: M::FrameWeaver, resource: R::ExperienceMemory,  permission: P::ReadWrite },
            AccessRule { module: M::FrameWeaver, resource: R::SutraTokens,       permission: P::Control   },
            AccessRule { module: M::FrameWeaver, resource: R::AshtiField,        permission: P::Read      },
            AccessRule { module: M::FrameWeaver, resource: R::GenomeConfig,      permission: P::Read      },
        ];

        let protocol_rules = vec![
            // Обязательные маршруты Ashti_Core
            ProtocolRule { source: M::Sutra,      target: M::Experience, data_type: D::TokenReference,    mandatory: true  },
            ProtocolRule { source: M::Experience, target: M::Arbiter,    data_type: D::ResonanceResponse, mandatory: true  },
            ProtocolRule { source: M::Arbiter,    target: M::Maya,       data_type: D::Reflex,            mandatory: false },
            ProtocolRule { source: M::Arbiter,    target: M::Logic,      data_type: D::PatternHint,       mandatory: true  },
            ProtocolRule { source: M::Logic,      target: M::Maya,       data_type: D::ProcessingResult,  mandatory: true  },
            ProtocolRule { source: M::Logic,      target: M::Experience, data_type: D::NewExperience,     mandatory: true  },
            ProtocolRule { source: M::Maya,       target: M::Arbiter,    data_type: D::ComparisonResult,  mandatory: false },
            ProtocolRule { source: M::Arbiter,    target: M::Experience, data_type: D::Feedback,          mandatory: false },
        ];

        Self {
            version: 1,
            invariants: GenomeInvariants::ashti_core_v2(),
            access_rules,
            protocol_rules,
            config: GenomeConfig::ashti_core_v2(),
        }
    }

    /// Загрузить GENOME из YAML файла.
    ///
    /// После загрузки автоматически вызывается `validate()`.
    /// Невалидный YAML или нарушение инвариантов → `Err(GenomeError)`.
    pub fn from_yaml(path: &Path) -> Result<Self, GenomeError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| GenomeError::InvariantViolation(
                // Нет хорошего способа вернуть динамическую строку через &'static str,
                // поэтому сигнализируем специальным значением и логируем тип ошибки.
                // В Фазе C можно добавить GenomeError::IoError(String).
                if e.kind() == std::io::ErrorKind::NotFound {
                    "genome yaml file not found"
                } else {
                    "failed to read genome yaml file"
                }
            ))?;

        let genome: Self = serde_yaml::from_str(&content)
            .map_err(|_| GenomeError::InvariantViolation("failed to parse genome yaml"))?;

        genome.validate()?;
        Ok(genome)
    }

    /// Валидация GENOME. Невалидный GENOME → система не запускается.
    pub fn validate(&self) -> Result<(), GenomeError> {
        // Инварианты размеров
        if self.invariants.token_size != 64 {
            return Err(GenomeError::InvariantViolation("token_size must be 64"));
        }
        if self.invariants.connection_size != 64 {
            return Err(GenomeError::InvariantViolation("connection_size must be 64"));
        }
        if self.invariants.event_size != 32 {
            return Err(GenomeError::InvariantViolation("event_size must be 32"));
        }
        if self.invariants.domain_config_size != 128 {
            return Err(GenomeError::InvariantViolation("domain_config_size must be 128"));
        }
        if self.invariants.max_domains != 11 {
            return Err(GenomeError::InvariantViolation("max_domains must be 11"));
        }

        // Флаги безопасности
        if !self.invariants.no_wall_clock_in_core {
            return Err(GenomeError::InvariantViolation("no_wall_clock_in_core must be true"));
        }
        if !self.invariants.event_id_monotonic {
            return Err(GenomeError::InvariantViolation("event_id_monotonic must be true"));
        }

        // GUARDIAN должен иметь ReadWrite на CODEX
        let guardian_has_codex_rw = self.access_rules.iter().any(|r|
            r.module == ModuleId::Guardian
            && r.resource == ResourceId::CodexRules
            && r.permission >= Permission::ReadWrite
        );
        if !guardian_has_codex_rw {
            return Err(GenomeError::MissingGuardianAccess);
        }

        // Обязательные протоколы должны присутствовать
        let mandatory = [
            (ModuleId::Sutra,      ModuleId::Experience, "SUTRA→EXPERIENCE"),
            (ModuleId::Experience, ModuleId::Arbiter,    "EXPERIENCE→ARBITER"),
        ];
        for (src, tgt, name) in mandatory {
            let found = self.protocol_rules.iter().any(|r|
                r.source == src && r.target == tgt && r.mandatory
            );
            if !found {
                return Err(GenomeError::MissingMandatoryProtocol(name));
            }
        }

        // Конфиг: базовые диапазоны
        if self.config.ashti_domain_count != 11 {
            return Err(GenomeError::InvalidConfig("ashti_domain_count must be 11"));
        }
        if self.config.default_heartbeat_interval == 0 {
            return Err(GenomeError::InvalidConfig("default_heartbeat_interval must be > 0"));
        }

        Ok(())
    }
}
