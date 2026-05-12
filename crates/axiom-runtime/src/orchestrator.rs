// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Orchestrator — 12-шаговый цикл обработки токена через Arbiter
//
// Маршрутизация: SUTRA(0) → EXPERIENCE(9) → [reflex OR ASHTI(1-8)] → MAYA(10)

use crate::engine::AxiomEngine;
use axiom_arbiter::RoutingResult;
use axiom_config::GUARDIAN_CHECK_REQUIRED;
use axiom_core::Token;

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
    // Шаги 1-4: AshtiCore выполняет dual-path routing.
    // Используем параллельный поиск (Sentinel Фаза 2) — при traces < PARALLEL_THRESHOLD
    // автоматически деградирует до последовательного без накладных расходов.
    //
    // Split borrow: engine.ashti (&mut) и engine.thread_pool (&) — разные поля, ОК.
    let pool = engine.thread_pool.as_ref();
    let mut result = engine.ashti.process_parallel(token, pool);

    // Шаг 3 (fast path): Guardian проверяет рефлекс если GUARDIAN_CHECK_REQUIRED бит установлен
    if let Some(ref reflex_token) = result.reflex {
        // Проверяем arbiter_flags домена-источника (SUTRA = 100)
        let check_required = engine
            .ashti
            .config_of(token.sutra_id as u16)
            .map(|cfg| cfg.arbiter_flags & GUARDIAN_CHECK_REQUIRED != 0)
            .unwrap_or(false);

        if check_required && !engine.guardian.validate_reflex(reflex_token).is_allowed() {
            // Рефлекс ингибирован — убираем его из результата
            result.reflex = None;
        }
    }

    // Шаг 5: Финализация — обратная связь в EXPERIENCE (если есть event_id)
    if result.event_id > 0 {
        // Ошибки финализации не являются фатальными (trace может не существовать)
        let _ = engine.ashti.apply_feedback(result.event_id);
    }

    result
}
