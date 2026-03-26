// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// GUARDIAN — над-доменный контроль соблюдения CODEX-правил

use axiom_core::{Token, STATE_LOCKED};
use axiom_domain::DomainState;

/// GUARDIAN — проверяет рефлексы и домены на соответствие CODEX.
///
/// CODEX — набор инвариантов, которые система никогда не должна нарушать:
/// - Рефлекс не может исходить от заблокированного токена
/// - Токен с ненулевой валентностью должен иметь ненулевую массу
/// - Температура должна быть в допустимом диапазоне (проверяется через конфигурацию)
pub struct Guardian {
    violation_count: u32,
}

impl Guardian {
    /// Создать новый Guardian
    pub fn new() -> Self {
        Self { violation_count: 0 }
    }

    /// Проверить рефлексный токен на соответствие CODEX.
    ///
    /// Возвращает `true` если рефлекс допустим, `false` — если ингибируется.
    pub fn validate_reflex(&mut self, token: &Token) -> bool {
        // Правило 1: заблокированный токен не может порождать рефлекс
        if token.state == STATE_LOCKED {
            self.violation_count += 1;
            return false;
        }

        // Правило 2: токен с валентностью должен иметь массу
        if token.valence != 0 && token.mass == 0 {
            self.violation_count += 1;
            return false;
        }

        // Правило 3: нулевой sutra_id — недопустимый токен
        if token.sutra_id == 0 {
            self.violation_count += 1;
            return false;
        }

        true
    }

    /// Сканировать состояние домена на нарушения CODEX.
    ///
    /// Возвращает число найденных нарушений (0 = чисто).
    pub fn scan_domain(&mut self, state: &DomainState) -> u32 {
        let mut local_violations = 0u32;

        for token in &state.tokens {
            // Нарушение: валентный токен без массы
            if token.valence != 0 && token.mass == 0 {
                local_violations += 1;
            }
        }

        self.violation_count += local_violations;
        local_violations
    }

    /// Получить общее число нарушений CODEX с момента создания Guardian
    pub fn violation_count(&self) -> u32 {
        self.violation_count
    }

    /// Сбросить счётчик нарушений
    pub fn reset_violations(&mut self) {
        self.violation_count = 0;
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self::new()
    }
}
