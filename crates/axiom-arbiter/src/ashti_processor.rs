// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ASHTI Processor stub
// TODO: Replace with full implementation when ASHTI processing is migrated

use axiom_config::DomainConfig;
use axiom_core::Token;
use crate::experience::ExperienceTrace;

/// ASHTI Processor - обработка токенов через ASHTI 1-8 домены (stub)
pub struct AshtiProcessor;

impl AshtiProcessor {
    /// Обработать токен через ASHTI домен
    pub fn process_token(
        token: &Token,
        _domain: &DomainConfig,
        _hint: Option<&ExperienceTrace>,
    ) -> Token {
        // Stub: просто возвращаем копию токена
        token.clone()
    }
}
