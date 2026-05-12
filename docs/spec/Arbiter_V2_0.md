# AXIOM MODULE SPECIFICATION: Arbiter V2.1

**Статус:** Актуальная спецификация (core)
**Версия:** 2.1.0
**Дата:** 2026-05-12
**Заменяет:** Arbiter V1.0
**Назначение:** Над-доменный модуль маршрутизации, ассоциативной памяти и кристаллизации знания
**Связанные спеки:** Ashti_Core V2.2, DomainConfig V2.1, COM V1.1, Cognitive_Depth V1.0, Axiom_Sentinel_V1_1

---

## 1. Назначение

**Arbiter** — над-доменный модуль, принимающий единственное архитектурное решение: как маршрутизировать сигнал между быстрым путём (рефлекс из EXPERIENCE или SKILLSET) и медленным путём (полная обработка в ASHTI).

Arbiter не интерпретирует смысл. Не генерирует ответы. Он маршрутизирует поток и накапливает опыт в форме следов (ExperienceTrace) и кристаллизованных навыков (Skill).

Arbiter работает в паре с GUARDIAN, но их роли различны:
- GUARDIAN: "можно ли?" (законность по CODEX)
- Arbiter: "куда и как?" (маршрут и обратная связь)

---

## 2. Позиция в потоке

```
SUTRA(100) → [ Arbiter ] → ASHTI(1-8) и/или MAYA(110)
                  ↑
              GUARDIAN (проверка рефлексов)
              SKILLSET (кристаллизованные навыки)
              EXPERIENCE (ассоциативная память)
              REFLECTOR (статистика)
```

Arbiter получает токен от SUTRA (domain_id = level_id * 100 + 0) и принимает решение о маршруте. Реализован в `crates/axiom-arbiter/src/lib.rs`.

---

## 3. Структура модуля

Arbiter состоит из нескольких субмодулей:

| Субмодуль | Файл | Назначение |
|---|---|---|
| `Experience` | `experience.rs` | Ассоциативная память, резонансный поиск, tension traces |
| `SkillSet` | `skillset.rs` | Кристаллизованные навыки (быстрый путь) |
| `Reflector` | `reflector.rs` | Статистика рефлексов по доменам |
| `AshtiProcessor` | `ashti_processor.rs` | Обработка через домены 1-8 |
| `MayaProcessor` | `maya_processor.rs` | Консолидация результатов в MAYA |
| `COM` | `com.rs` | Причинный счётчик event_id |
| `GridHash` | `gridhash.rs` | O(1) пространственный индекс для EXPERIENCE |

### 3.1 Experience API (V2.1: Axiom Sentinel S2)

Добавлены методы для динамической дистилляции памяти (S2):

```rust
// Счётчик всех добавленных трейсов за время жизни экземпляра (не сбрасывается)
pub traces_seen_total: u64

// Проверить, нужно ли запустить export_skills (каждые 5000 трейсов)
pub fn should_trigger_export(&self) -> bool

// Оценка потребления памяти трейсами и tension_traces (байт)
pub fn estimate_memory_bytes(&self) -> usize

// Изменить лимит traces (минимум 1)
pub fn set_max_traces(&mut self, n: usize)
```

`should_trigger_export` возвращает `true` при `traces_seen_total > 0 && traces_seen_total % 5000 == 0`. Вызывается в тик-цикле AxiomEngine для запуска `export_skills` без блокировки горячего пути.

---

## 4. Цикл маршрутизации

### 4.1 Точка входа

```
route_token(token, source_domain=0) → RoutingResult
```

Все входящие токены приходят с `source_domain=0` (SUTRA). Arbiter также может маршрутизировать из других доменов, но этот путь не используется в production flow.

### 4.2 Шаги обработки

**Шаг 0: Проверка SKILLSET (быстрее рефлекса)**

Перед резонансным поиском Arbiter проверяет SkillSet:
```
skillset.find_skill_with_idx(&token)
```
- Если кристаллизованный навык найден → мгновенный ответ (без resonance_search)
- Reflex = skill.pattern, slow_path = ASHTI результаты (всё равно выполняется)
- Возвращает RoutingResult с `confidence = 1.0`, `passes = 1`

**Шаг 1: Резонансный поиск (Experience)**

```
experience.resonance_search(&token)         // sequential, всегда
experience.resonance_search_parallel(&token, pool)  // раздельный, ≥ 512 следов
```

Двухфазный поиск:
- **Phase 1 (GridHash O(1))**: Проверяет GridHash-индекс по grid-ключу токена. При попадании с `score ≥ reflex_threshold` → ранний выход (Reflex).
- **Phase 2 (O(N) с hash-prefilter)**: Полный перебор traces. Предфильтр по Hamming distance (threshold=40 бит). Лучший результат по `score = similarity × weight`.

Классификация результата:
- `score ≥ reflex_threshold (128/255 ≈ 0.5)` → `ResonanceLevel::Reflex`
- `score ≥ association_threshold (64/255 ≈ 0.25)` → `ResonanceLevel::Association`
- `score < association_threshold` → `ResonanceLevel::None`

**Шаг 2: Fast path (рефлекс)**

Если `resonance.level == Reflex`:
- `reflex = Some(trace.pattern)`
- Проверка GUARDIAN происходит в **Orchestrator**, не в Arbiter

**Шаг 3: Slow path (ASHTI 1-8)**

Выполняется **всегда** (SLOW_PATH_MANDATORY), если только не активирован Layer Priority режим (см. §6.1):
- При `Association` → hint (подсказка) передаётся в `route_to_ashti`
- При `None` → обработка без подсказки

`route_to_ashti` принимает параметр `max_role: u8` (обычно 8 — все домены). При `max_role < 8` цикл ограничивает обработку доменами 1..=max_role.

**Шаг 4: Консолидация (MAYA)**

```
route_to_maya_with_confidence(ashti_results) → (Option<Token>, f32)
```
- `confidence` = нормализованная согласованность ASHTI-результатов (0.0..=1.0)
- При `confidence < min_coherence` → создаётся TensionTrace в Experience

**Шаг 5: Сохранение PendingComparison**

```
pending_comparisons.insert(event_id, PendingComparison { ... })
```
Ожидает вызова `finalize_comparison(event_id)` для обратной связи.

---

## 5. Multi-pass (Cognitive Depth V1.0)

`route_with_multipass(token)` — расширенный путь для Cognitive Depth.

Активируется только при прямом вызове (не из production flow через AshtiCore::process).
В production используется только из `arbiter_heartbeat_pulse` и тестов.

```
for pass in 0..max_passes:
    ashti_results = route_to_ashti(current_pattern, hint_if_first_pass)
    (consolidated, confidence) = route_to_maya_with_confidence(ashti_results)
    if confidence >= min_coherence → break
    // иначе: обогащаем паттерн и повторяем
if final_confidence < min_coherence:
    experience.add_tension_trace(token, tension_temp, event_id)
```

- `max_passes` читается из `DomainConfig` MAYA
- `min_coherence` читается из `DomainConfig` MAYA (как `u8 / 255.0`)
- `tension_temp = (1.0 - final_confidence) * 255.0 as u8`

---

## 6. Параллелизм (Axiom Sentinel)

### 6.1 Параллельный поиск (V1.0, Фаза 2)

```rust
pub fn route_token_parallel(token, source_domain, pool: &rayon::ThreadPool) -> RoutingResult
```

При `traces.len() >= PARALLEL_THRESHOLD (512)`:
- Phase 2 выполняется через `pool.install(|| traces.par_iter().fold().reduce())`
- Каждый поток ведёт локальный аккумулятор `(best_score, best_idx, matched_count)`
- Merge без mutex через `reduce`
- `Cell<u32>` (last_traces_matched) не попадает в closure — безопасно для Send

При `traces.len() < 512` → автоматический fallback на sequential (без overhead rayon).

В production flow Orchestrator всегда вызывает `process_parallel`.

### 6.2 Layer Priority Path (V1.1, S5)

```rust
pub fn route_token_limited(token: Token, pool: Option<&rayon::ThreadPool>, max_role: u8) -> RoutingResult
```

Упрощённый путь обработки, используемый когда бюджет тика исчерпан более чем на 80% (TickBudget в AxiomEngine).

Отличия от `route_token_parallel`:
- Выполняет резонансный поиск (параллельный при наличии pool и ≥512 traces).
- Вызывает `route_to_ashti` с ограниченным `max_role` (обычно 3 — только рефлекторные домены EXECUTION/SHADOW/CODEX).
- **Не выполняет** multi-pass и не создаёт tension traces.
- Возвращает `RoutingResult` с `passes = 1`.

Включается через `TickSchedule::enable_layer_priority = true` (по умолчанию `false`). Пока gate выключен, поведение системы идентично V2.0. Orchestrator вызывает `route_token_limited` только когда `engine.budget_used_fraction() > 0.80` и gate включён.

---

## 7. Обратная связь (finalize_comparison)

```
finalize_comparison(event_id) → Result<(), String>
```

Выполняется после завершения маршрутизации (из Orchestrator через `apply_feedback`).

Алгоритм:
1. Найти `PendingComparison` по `event_id`
2. Если не было рефлекса → добавить след в Experience (новый опыт)
3. Если рефлекс был:
   - Сравнить `reflex_prediction` с `consolidated_result` по tolerances:
     - temperature: |diff| < TOKEN_COMPARE_TEMP_TOLERANCE (10)
     - mass: |diff| < TOKEN_COMPARE_MASS_TOLERANCE (5)
     - valence: |diff| < TOKEN_COMPARE_VALENCE_TOLERANCE (2)
   - Совпадение → `experience.strengthen_by_hash(pattern_hash, delta)` + `reflector.record_success`
   - Расхождение → добавить новый след с правильным результатом + `reflector.record_failure`

---

## 8. REFLECTOR

Накапливает статистику рефлексов по доменам (ролям 1-8).

```
reflector.record_success(role, pattern)
reflector.record_failure(role, pattern)
reflector.domain_profile(role) → Option<DomainProfile>
```

Используется GUARDIAN (через `run_adaptation` в AxiomEngine) для адаптации порогов.

---

## 9. Internal Drive (Cognitive Depth V1.0)

### 9.1 HeartBeat Pulse

```
on_heartbeat_pulse(pulse_number, enable_internal_drive) → Vec<Token>
```

При `enable_internal_drive = true` генерирует impulse tokens из:
- TensionTraces: `drain_hot_impulses(threshold)` → горячие следы напряжения
- CuriosityImpulses: `check_curiosity_candidates(threshold)` → следы near crystallization

### 9.2 Goal Impulses

```
generate_goal_impulses(pulse_number, check_interval) → Vec<InternalImpulse>
```

Цели = следы с `TOKEN_FLAG_GOAL` и `weight < GOAL_ACHIEVED_WEIGHT (0.9)`.
Чем дальше от достижения — тем сильнее импульс.

---

## 10. RoutingResult

```rust
pub struct RoutingResult {
    pub event_id: u64,              // COM event_id
    pub reflex: Option<Token>,      // Fast path: паттерн рефлекса
    pub slow_path: Vec<Token>,      // ASHTI 1-8 результаты
    pub consolidated: Option<Token>,// MAYA консолидация
    pub routed_events: Vec<u64>,    // COM tracking
    pub confidence: f32,            // 0.0..=1.0, согласованность ASHTI
    pub passes: u8,                 // 1 = normal, >1 = multi-pass
}
```

---

## 11. PendingComparison

```rust
pub struct PendingComparison {
    pub input_pattern: Token,
    pub reflex_prediction: Option<Token>,
    pub ashti_results: Vec<Token>,
    pub consolidated_result: Option<Token>,
    pub created_at: u64,            // event_id
    pub trace_index: Option<usize>, // индекс в Experience.traces
}
```

Хранится в `pending_comparisons: HashMap<u64, PendingComparison>`.
Очищается при вызове `finalize_comparison(event_id)`.

---

## 12. Инварианты

1. **Slow path обязателен.** ASHTI(1-8) обрабатывает токен всегда, даже при рефлексе. Исключение: при `enable_layer_priority = true` и `budget_used_fraction > 0.80` используется `route_token_limited` с `max_role ≤ 3`.
2. **SkillSet приоритетнее Experience.** Кристаллизованный навык проверяется до resonance_search.
3. **GUARDIAN имеет вето в Orchestrator, не в Arbiter.** Arbiter возвращает reflex; Orchestrator решает применять или нет.
4. **Порог резонанса — глобальный.** Reflex/association threshold применяется один для всего Experience, не per-domain (как описывал V1.0 — это не реализовано).
5. **Параллелизм прозрачен.** Результат `resonance_search_parallel` семантически идентичен `resonance_search`.
6. **TensionTrace создаётся Arbiter, потребляется Heartbeat.** Никто кроме `on_heartbeat_pulse` не дренирует tension traces.

---

## 13. Что НЕ реализовано (vs V1.0 spec)

| Фича из V1.0 | Статус |
|---|---|
| `reflex_cooldown` per domain | Не реализовано |
| `response_timeout` | Не реализовано |
| Per-domain `reflex_threshold` в resonance_search | Не реализовано (глобальный порог) |
| COM events (Reflex_Dispatched, Hint_Attached, ...) | Не реализовано |
| `storm_threshold` в Arbiter | Реализован отдельно в CausalFrontier |

---

## 14. История изменений

- **V2.1 (2026-05-12)**: Axiom Sentinel V1.1. §3.1: Experience S2 API (traces_seen_total, should_trigger_export, estimate_memory_bytes, set_max_traces). §6.2: route_token_limited — Layer Priority путь (S5). §4.2: документирован параметр max_role в route_to_ashti. §12: уточнён инвариант Slow path (исключение для Layer Priority).
- **V2.0 (2026-04-08)**: Полная актуализация. SkillSet fast path. Multi-pass (Cognitive Depth V1.0). TensionTrace / InternalDrive. GridHash двухфазный поиск. Параллельный поиск (Sentinel V1.0). Точки несоответствия с V1.0 задокументированы. GUARDIAN вето перенесено в Orchestrator.
- **V1.0 (2026-03-19)**: Первая версия. Над-доменный модуль маршрутизации.
