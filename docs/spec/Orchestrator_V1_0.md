# AXIOM MODULE SPECIFICATION: Orchestrator V1.1

**Статус:** Актуальная спецификация (core)
**Версия:** 1.1.0
**Дата:** 2026-05-12
**Назначение:** Тонкий слой маршрутизации токена в axiom-runtime между AxiomEngine и AshtiCore
**Связанные спеки:** Arbiter_V2_1, Ashti_Core V2.2, Axiom_Sentinel_V1_1

---

## 1. Назначение

**Orchestrator** — функциональный модуль в `crates/axiom-runtime/src/orchestrator.rs`, выполняющий полный цикл маршрутизации токена. Он не является отдельным crate или структурой — это `pub(crate)` функция, вызываемая из методов `AxiomEngine`.

Orchestrator изолирует три сквозных ответственности от `engine.rs`:

1. **Параллельная маршрутизация** через Arbiter (AshtiCore)
2. **Проверка GUARDIAN** для рефлексов (fast path)
3. **Финализация** — обратная связь в EXPERIENCE

Без Orchestrator эти три шага были бы рассыпаны по методам `AxiomEngine`, загромождая `engine.rs`.

---

## 2. Позиция в архитектуре

```
AxiomEngine::process_and_observe()
AxiomEngine::handle_dual_path()
        │
        ▼
orchestrator::route_token(engine, token)
        │
        ├─ engine.ashti.process_parallel(token, pool)          → RoutingResult  [обычный путь]
        │   или
        ├─ engine.ashti.process_parallel_limited(token, pool, max_role)          [Layer Priority, S5]
        │           │
        │           └─ Arbiter → Experience → ASHTI(1..=max_role) → MAYA
        │
        ├─ engine.guardian.validate_reflex(reflex_token)   [если нужно]
        │
        └─ engine.ashti.apply_feedback(event_id)
```

Orchestrator не хранит состояния. Он мутирует `engine` транзитно: через `&mut engine.ashti` и `&engine.thread_pool`.

---

## 3. Функции

### 3.0 route_token — основной путь

```rust
pub(crate) fn route_token(engine: &mut AxiomEngine, token: Token) -> RoutingResult
```

Файл: `crates/axiom-runtime/src/orchestrator.rs`

### 3.1 Шаг 1: Параллельная маршрутизация

```rust
let pool = engine.thread_pool.as_ref();
let mut result = engine.ashti.process_parallel(token, pool);
```

Использует `process_parallel` (AshtiCore), который вызывает `Arbiter::route_token_parallel`.

- При `Experience.traces.len() >= PARALLEL_THRESHOLD (512)` → резонансный поиск через rayon
- При `< 512` → автоматически деградирует до последовательного без накладных расходов
- **Split borrow**: `engine.ashti` (`&mut`) и `engine.thread_pool` (`&Arc<rayon::ThreadPool>`) — разные поля структуры, Rust допускает одновременный заём. `engine.thread_pool.as_ref()` разыменовывает `Arc` в `&rayon::ThreadPool`.

### 3.2 Шаг 2: GUARDIAN check (опциональный)

```rust
if let Some(ref reflex_token) = result.reflex {
    let check_required = engine.ashti
        .config_of(token.sutra_id)
        .map(|cfg| cfg.arbiter_flags & GUARDIAN_CHECK_REQUIRED != 0)
        .unwrap_or(false);

    if check_required && !engine.guardian.validate_reflex(reflex_token).is_allowed() {
        result.reflex = None;
    }
}
```

Проверка выполняется только если:
- В результате есть рефлекс (`result.reflex.is_some()`)
- Флаг `GUARDIAN_CHECK_REQUIRED` установлен в `arbiter_flags` домена-источника (SUTRA)

Arbiter возвращает рефлекс безусловно. Именно здесь Orchestrator принимает финальное решение: применять или подавлять. Это соответствует инварианту Arbiter V2.0 §12: *"GUARDIAN имеет вето в Orchestrator, не в Arbiter"*.

Если GUARDIAN блокирует рефлекс → `result.reflex = None`. Slow path (`result.slow_path`) остаётся нетронутым.

### 3.3 Шаг 3: Финализация (apply_feedback)

```rust
if result.event_id > 0 {
    let _ = engine.ashti.apply_feedback(result.event_id);
}
```

Запускает `finalize_comparison(event_id)` в Arbiter — обратную связь в EXPERIENCE:
- Если не было рефлекса → новый ExperienceTrace
- Если рефлекс был → сравнение с consolidated_result, усиление или добавление контртрейса

Ошибки финализации не фатальны (`let _`): trace может отсутствовать (например, при первом токене).

---

### 3.4 route_token_limited — Layer Priority путь (V1.1, S5)

```rust
pub(crate) fn route_token_limited(engine: &mut AxiomEngine, token: Token) -> RoutingResult
```

Используется вместо `route_token` когда `engine.budget_used_fraction() > 0.80` и `TickSchedule::enable_layer_priority = true`.

Алгоритм:
1. Вычисляет `max_role = engine.layer_priority_max_role()` → возвращает 3 при бюджетном ограничении.
2. Вызывает `engine.ashti.process_parallel_limited(token, pool, max_role)`.
3. Выполняет GUARDIAN check и `apply_feedback` аналогично `route_token`.

Семантическая разница: только домены 1–3 (EXECUTION, SHADOW, CODEX) участвуют в slow path. Домены 4–8 пропускаются для экономии времени тика.

---

## 4. Где вызывается

| Caller | Файл |
|---|---|
| `AxiomEngine::process_and_observe()` | `crates/axiom-runtime/src/lib.rs` |
| `AxiomEngine::handle_dual_path()` | `crates/axiom-runtime/src/engine.rs` |

Оба метода передают `self` (как `&mut AxiomEngine`) и токен из входящей UCL-команды.

---

## 5. Связь с RoutingResult

Orchestrator возвращает `RoutingResult` без изменений (кроме возможного обнуления `result.reflex`). Вызывающий код (process_and_observe) интерпретирует поля:

```rust
pub struct RoutingResult {
    pub event_id: u64,
    pub reflex: Option<Token>,        // None если Guardian заблокировал
    pub slow_path: Vec<Token>,
    pub consolidated: Option<Token>,
    pub routed_events: Vec<u64>,
    pub confidence: f32,
    pub passes: u8,
}
```

---

## 6. Инварианты

1. **Orchestrator без состояния.** Не хранит поля между вызовами. Все побочные эффекты — через `engine`.
2. **GUARDIAN — только здесь.** Ни Arbiter, ни AshtiCore не проверяют GUARDIAN. Это единственная точка вето.
3. **apply_feedback — всегда последний.** Финализация вызывается после всех проверок, с финальным состоянием `result`.
4. **Параллелизм прозрачен для вызывающего.** Caller не знает, был ли поиск параллельным — результат семантически идентичен.
5. **event_id = 0 означает отсутствие COM-события.** Финализация не вызывается.

---

## 7. Что не реализовано

| Возможная фича | Статус |
|---|---|
| Таймаут маршрутизации (`response_timeout`) | Не реализован (Arbiter V2.0 §13) |
| Метрики latency per-token | Не реализованы |
| Retry при ошибке AshtiCore | Не нужен (Arbiter возвращает RoutingResult::error без panic) |
| Логирование маршрута (tracing) | Не реализовано |

---

## 8. История изменений

- **V1.1 (2026-05-12)**: Axiom Sentinel V1.1. §3.4: добавлена `route_token_limited` для Layer Priority режима (S5). §3.1: `engine.thread_pool.as_ref()` вместо `&engine.thread_pool` — корректное разыменование `Arc<rayon::ThreadPool>`. §2: диаграмма обновлена с альтернативным путём `process_parallel_limited`.
- **V1.0 (2026-04-08)**: Первая версия. Выделен из engine.rs при реализации Axiom Sentinel V1.0, Фаза 2. Добавлен параллельный поиск через `process_parallel`. GUARDIAN check перенесён из Arbiter. apply_feedback централизован здесь.
