// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: Membrane — фильтры входа/выхода токенов

use axiom_core::{Token, STATE_LOCKED};
use axiom_config::{DomainConfig, MEMBRANE_CLOSED};

/// Domain V1.3: может ли токен войти в домен?
///
/// Проверяет:
/// - Мембрана не закрыта
/// - Масса токена выше порога
/// - Bloom filter (если установлен) пропускает токен
pub fn can_enter_domain(token: &Token, config: &DomainConfig) -> bool {
    if config.membrane_state == MEMBRANE_CLOSED {
        return false;
    }
    if (token.mass as u16) < config.threshold_mass {
        return false;
    }
    // Bloom filter: u64::MAX означает «всё разрешено»
    if config.input_filter != u64::MAX {
        let token_hash = token.sutra_id as u64 ^ token.type_flags as u64;
        if config.input_filter & token_hash == 0 {
            return false;
        }
    }
    true
}

/// Domain V1.3: может ли токен покинуть домен?
///
/// Проверяет:
/// - Мембрана не закрыта
/// - Токен не заблокирован
pub fn can_exit_domain(token: &Token, config: &DomainConfig) -> bool {
    if config.membrane_state == MEMBRANE_CLOSED {
        return false;
    }
    if token.state == STATE_LOCKED {
        return false;
    }
    true
}
