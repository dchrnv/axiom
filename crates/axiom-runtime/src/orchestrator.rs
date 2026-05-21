// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Orchestrator — 12-шаговый цикл обработки токена через Arbiter
//
// Маршрутизация: SUTRA(0) → EXPERIENCE(9) → [reflex OR ASHTI(1-8)] → MAYA(10)

use crate::engine::AxiomEngine;
use axiom_arbiter::RoutingResult;
use axiom_config::GUARDIAN_CHECK_REQUIRED;
use axiom_core::{Connection, Token};

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

    // Шаг 6 (SyntacticBridge): инжектировать 0x08-связи в MAYA domain state
    // чтобы FrameWeaver мог кристаллизовать Frame-анкеры.
    bridge_to_maya(engine, &result);

    result
}

/// SyntacticBridge (Фаза 0 CR-V6): инжектировать синтаксические связи в MAYA domain state.
///
/// После каждого routing slow-path FrameWeaver получает 0x08-связи в MAYA, из которых
/// может кристаллизовать Frame-анкеры в EXPERIENCE (при stability_count ≥ threshold).
///
/// `source_id` = position-hash консолидированного результата (стабилен для одного паттерна).
/// `target_id` = hash(domain_id, position) для каждого ASHTI-результата — уникален по роли.
/// `link_type`  = 0x0800 | (role << 4) — синтаксический слой.
///
/// Игнорирует ошибки ёмкости (best-effort).
fn bridge_to_maya(engine: &mut AxiomEngine, result: &RoutingResult) {
    // Нечего мостить если slow_path пустой или консолидации нет
    let consolidated = match result.consolidated {
        Some(t) => t,
        None => return,
    };
    if result.slow_path.is_empty() {
        return;
    }

    let level = engine.ashti.level_id();
    let maya_domain_id: u16 = level * 100 + 10;

    // source_id: стабильный хеш консолидированной позиции (один на весь routing)
    let source_id = position_hash(consolidated.position);
    let event_id = engine.next_event_id();

    for (i, ashti_tok) in result.slow_path.iter().enumerate() {
        let role = (i + 1) as u16; // 1..=8
        // target_id включает domain_id → уникален даже когда позиции совпадают.
        // Token::new ставит target=position, поэтому apply_spatial не двигает токен
        // при первом вызове. Включение domain_id гарантирует уникальность по роли.
        let target_id = domain_position_hash(ashti_tok.domain_id, ashti_tok.position);
        if target_id == source_id {
            continue; // коллизия хеша — крайне редко
        }
        let mut conn = Connection::new(source_id, target_id, maya_domain_id, event_id);
        conn.link_type = 0x0800 | (role << 4);
        conn.strength = consolidated.mass as f32 / 255.0;
        let _ = engine.ashti.inject_connection(maya_domain_id, conn);
    }
}

/// Стабильный u32-идентификатор из позиции токена (FNV-1a по трём координатам).
fn position_hash(pos: [i16; 3]) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &v in &pos {
        h ^= (v as u16) as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let id = (h & 0x0FFF_FFFF) as u32; // 28 бит — не конфликтует с SUTRA anchor IDs
    if id == 0 { 1 } else { id }
}

/// Стабильный u32-идентификатор из domain_id + позиции токена (FNV-1a).
///
/// Включение domain_id гарантирует уникальность по ASHTI-роли даже когда позиции совпадают.
fn domain_position_hash(domain_id: u16, pos: [i16; 3]) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= domain_id as u64;
    h = h.wrapping_mul(0x100000001b3);
    for &v in &pos {
        h ^= (v as u16) as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let id = (h & 0x0FFF_FFFF) as u32;
    if id == 0 { 1 } else { id }
}

/// Routing с ограниченным набором ролей (S5: TickBudget layer priority).
///
/// Вызывается из tick_wake когда budget > 80% и enable_layer_priority = true.
/// Выполняет только роли 1..=max_role, пропуская MAP/PROBE/LOGIC/DREAM/VOID.
pub(crate) fn route_token_limited(engine: &mut AxiomEngine, token: Token, max_role: u8) -> RoutingResult {
    let pool = engine.thread_pool.as_ref();
    let mut result = engine.ashti.process_parallel_limited(token, pool, max_role);

    if let Some(ref reflex_token) = result.reflex {
        let check_required = engine
            .ashti
            .config_of(token.sutra_id as u16)
            .map(|cfg| cfg.arbiter_flags & GUARDIAN_CHECK_REQUIRED != 0)
            .unwrap_or(false);
        if check_required && !engine.guardian.validate_reflex(reflex_token).is_allowed() {
            result.reflex = None;
        }
    }

    if result.event_id > 0 {
        let _ = engine.ashti.apply_feedback(result.event_id);
    }

    bridge_to_maya(engine, &result);

    result
}
