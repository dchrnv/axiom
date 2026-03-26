// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Orchestrator — 12-шаговый цикл обработки токена через Arbiter
//
// Маршрутизация: SUTRA(0) → EXPERIENCE(9) → [reflex OR ASHTI(1-8)] → MAYA(10)

use axiom_core::Token;
use axiom_arbiter::RoutingResult;
use crate::engine::AxiomEngine;

/// Выполнить полный цикл маршрутизации токена через Arbiter.
///
/// Шаги:
/// 1.  Токен поступает от SUTRA (source_domain = 0)
/// 2.  Arbiter передаёт в EXPERIENCE(9) → resonance_search
/// 3.  Если Reflex → fast path (Guardian проверяет рефлекс)
/// 4.  Slow path → ASHTI(1-8) → MAYA(10)
/// 5.  Консолидированный результат → finalize_comparison → обратная связь в EXPERIENCE
/// 6.  Возврат RoutingResult
pub(crate) fn route_token(engine: &mut AxiomEngine, token: Token) -> RoutingResult {
    // Шаги 1-4: Arbiter выполняет dual-path routing
    let mut result = engine.arbiter.route_token(token, 0);

    // Шаг 3 (fast path): Guardian проверяет рефлекс
    if let Some(ref reflex_token) = result.reflex.clone() {
        if !engine.guardian.validate_reflex(reflex_token) {
            // Рефлекс ингибирован — убираем его из результата
            result.reflex = None;
        }
    }

    // Шаг 5: Финализация — обратная связь в EXPERIENCE (если есть event_id)
    if result.event_id > 0 {
        // Ошибки финализации не являются фатальными (trace может не существовать)
        let _ = engine.arbiter.finalize_comparison(result.event_id);
    }

    result
}
