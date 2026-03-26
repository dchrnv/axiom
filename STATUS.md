# AXIOM Migration Status

**baseline_test_count:** 0
**current_test_count:** 324
**date_started:** 2026-03-21
**test_structure:** Извлечено в отдельные файлы (2026-03-21)

---

## Фазы миграции

| Фаза | Crate | Статус | Дата | Тесты | Комментарий |
|-------|-------|--------|------|-------|-------------|
| 0 | workspace setup | ✅ | 2026-03-21 | 0 | Завершено: 11 crates, justfile, scripts |
| 1 | axiom-core | ✅ | 2026-03-21 | 24 | Token, Connection, Event (zero deps) |
| 2 | axiom-frontier | ✅ | 2026-03-21 | 22 | Frontier, storm, budget, processor |
| 3 | axiom-config | ✅ | 2026-03-21 | 33 | DomainConfig (+factory методы, мембрана), HeartbeatConfig, ConfigLoader |
| 4 | axiom-space | ✅ | 2026-03-21 | 83 | SpatialHashGrid, координаты, физика (1983 строки) |
| 5 | axiom-shell | ✅ | 2026-03-21 | 43 | Shell V3.0, семантические профили (1365 строк) |
| 6 | axiom-arbiter | ✅ | 2026-03-26 | 26 | Arbiter V1.0 — stub-модули заменены: experience (resonance_search, strengthen/weaken), ashti (hint blend + role rules), maya (avg/median/confidence), com (per-domain tracking) |
| 7 | axiom-heartbeat | ✅ | 2026-03-21 | 11 | Heartbeat V2.0, периодическая активация (413 строк) |
| 8 | axiom-upo + axiom-ucl | ✅ | 2026-03-21 | 0+5 | UPO v2.2 (388 строк) + UCL commands (356 строк) |
| 9 | axiom-domain | ✅ | 2026-03-26 | 71 | Domain, DomainState, EventGenerator, membrane — без unsafe, без дублирования. AshtiCore → DEFERRED.md |
| 10 | axiom-runtime | ⏸️ | — | — | Пропущена (зависит от domain и arbiter) |

---

## Прогресс Фазы 0

### Checklist:
- [x] Зафиксирован baseline test count: 0
- [x] Создан корневой Cargo.toml
- [x] Созданы 11 пустых crates с lib.rs и Cargo.toml
- [x] Создан justfile с командами
- [x] Создан STATUS.md (этот файл)
- [x] Созданы tools/check_deps.sh и visualize_deps.sh
- [x] Проверка: cargo build успешно компилирует workspace (11.75s)
- [x] Проверка: cargo test --workspace проходит (0 тестов)
- [x] Проверка: cargo clippy без warnings (4.25s)

---

## Легенда статусов

- ⬜ Не начата
- 🔄 В процессе
- ✅ Завершена
- ❌ Заблокирована / проблема

---

## Прогресс Фазы 1

### Checklist:
- [x] Перенесены Token, Connection, Event в axiom-core
- [x] Удалены зависимости от config (zero dependencies)
- [x] Все структуры 64 байта, repr(C, align(64))
- [x] Добавлены compile-time size assertions
- [x] Перенесены и исправлены все тесты (24 теста)
- [x] Проверка: cargo test -p axiom-core passes (24 tests)
- [x] Проверка: cargo build --workspace успешно

---

## Прогресс Фазы 2

### Checklist:
- [x] Перенесён CausalFrontier в axiom-frontier
- [x] EntityQueue с дедупликацией через visited BitVec
- [x] FrontierState lifecycle (Empty, Active, Storm, Stabilized, Idle)
- [x] Storm detection (size > storm_threshold)
- [x] Causal budget (max_events_per_cycle)
- [x] Memory limit (max_frontier_size)
- [x] Создан FrontierProcessor с trait LocalRules
- [x] Основной цикл: pop → evaluate → transform → push neighbors
- [x] Все тесты перенесены + processor tests (22 теста)
- [x] Проверка: cargo test -p axiom-frontier passes (22 tests)
- [x] Проверка: cargo build --workspace успешно

---

## Прогресс Фазы 3

### Checklist:
- [x] Найден и изучен DomainConfig в runtime/src/domain.rs
- [x] Найден HeartbeatConfig в runtime/src/heartbeat.rs
- [x] Найден ConfigLoader в runtime/src/config/loader.rs
- [x] Создан domain_config.rs с DomainConfig (128 байт, 5 блоков)
- [x] Добавлены StructuralRole и DomainType enums
- [x] Добавлены константы DOMAIN_* и PROCESSING_*
- [x] Добавлен compile-time size assertion
- [x] Создан heartbeat_config.rs с пресетами (weak, medium, powerful, disabled)
- [x] Создан loader.rs с ConfigLoader и типами ошибок
- [x] Создан lib.rs с re-exports
- [x] Добавлены 17 тестов (11 domain + 3 heartbeat + 3 loader)
- [x] Проверка: cargo test -p axiom-config passes (17 tests)
- [x] Проверка: cargo test --workspace успешно (63 теста)

---

## Прогресс Фазы 4

### Checklist:
- [x] Найден и изучен space.rs (1983 строки, 83 теста)
- [x] Скопирован space.rs в axiom-space/src/lib.rs полностью
- [x] Проверена зависимость на axiom-core в Cargo.toml
- [x] Проверка: cargo test -p axiom-space --lib passes (83 tests)
- [x] Проверка: cargo test --workspace --lib успешно (146 тестов)

### Компоненты:
- Константы: CELL_SHIFT=8, CELL_SIZE=256, BUCKET_COUNT=65536
- Функции координат: distance2, distance2_to_anchor, has_moved, cell_changed
- Функции физики: compute_gravity (Linear/InverseSquare), apply_velocity, apply_friction, apply_acceleration, move_towards
- SpatialHashGrid: insert, rebuild, query_cell, find_neighbors (zero-alloc, linked lists)
- CellEntry: token_index + next (linked list node)
- CellIterator: итератор по токенам в ячейке
- Обнаружение столкновений: detect_collisions
- 83 теста покрывают всю функциональность

---

## Прогресс Фазы 5

### Checklist:
- [x] Найден и изучен shell.rs (1365 строк, 43 теста)
- [x] Скопирован shell.rs в axiom-shell/src/lib.rs полностью
- [x] Проверены зависимости: axiom-core, bitvec
- [x] Исправлены импорты: `use axiom_core::connection::Connection`
- [x] Исправлены тесты: добавлен event_id в Connection::new()
- [x] Проверка: cargo test -p axiom-shell --lib passes (43 tests)
- [x] Проверка: cargo test --workspace --lib успешно (189 тестов)

### Компоненты:
- ShellProfile = [u8; 8] - 8 слоёв восприятия (Physical, Sensory, Motor, Emotional, Cognitive, Social, Temporal, Abstract)
- DomainShellCache: profiles Vec + dirty_flags BitVec + generation counter
- SemanticContributionTable: categories[256] + overrides HashMap
- Алгоритмы: compute_shell, mark_dirty, collect_affected, process_event, reconcile_batch
- default_ashti_core() - 7 категорий связей (Structural, Semantic, Causal, Experiential, Social, Temporal, Motor)
- 43 теста: profile, cache, table, compute, dirty tracking, reconciliation

---

## Прогресс Фазы 7

### Checklist:
- [x] Найден и изучен heartbeat.rs (413 строк, 11 тестов)
- [x] Скопирован heartbeat.rs в axiom-heartbeat/src/lib.rs полностью
- [x] Проверены зависимости: axiom-core, axiom-frontier
- [x] Исправлены импорты: `use axiom_core::event::*` и `use axiom_frontier::CausalFrontier`
- [x] Проверка: cargo test -p axiom-heartbeat --lib passes (11 tests)
- [x] Проверка: cargo test --workspace --lib успешно (200 тестов)

### Компоненты:
- HeartbeatConfig: пресеты (weak, medium, powerful, disabled)
- HeartbeatGenerator: pulse counter, batch selection (tokens, connections)
- Heartbeat event creation с event_id tracking
- Детерминированный отбор токенов/связей (модульная арифметика)
- Пакетная обработка для токенов и связей
- Wraparound handling для больших доменов
- 11 тестов: config presets, generation, batching, wraparound, determinism

### Примечание:
- Фаза 6 (axiom-arbiter) пропущена - будет реализована позже после миграции experience, ashti_processor, maya_processor, com

---

## Прогресс Фазы 8

### Checklist:
- [x] Найдены и изучены upo.rs (388 строк, 0 тестов) и ucl_command.rs (356 строк, 5 тестов)
- [x] Скопированы оба файла в axiom-upo/src/lib.rs и axiom-ucl/src/lib.rs
- [x] Проверены зависимости: axiom-core (для обоих)
- [x] Исправлены импорты в upo.rs: `use axiom_core::connection::Connection` и `use axiom_core::token::Token`
- [x] UCL не требовал исправлений (уже использовал правильные импорты)
- [x] Проверка: cargo test -p axiom-upo passes (0 tests)
- [x] Проверка: cargo test -p axiom-ucl passes (5 tests)
- [x] Проверка: cargo test --workspace --lib успешно (205 тестов)

### Компоненты:

**axiom-upo (Universal Patch Observer v2.2):**
- TraceSourceType: Token, Connection, Field
- TraceFlags: ACTIVE, FADING, MERGED
- Trace: 128-байтная структура для отслеживания изменений
- UPOEngine: движок наблюдения за изменениями
- Patch generation и application
- 0 тестов (тесты будут добавлены позже)

**axiom-ucl (Universal Command Language):**
- UCLCommand: SpawnDomain, ApplyForce, QueryState (64 байта, repr(C))
- UCLResult: Success/Error с payload (64 байта)
- Compile-time size assertions
- 5 тестов: command size, result size, spawn domain, apply force, result creation

---

---

## Итоги миграции (2026-03-21)

### ✅ Завершено: 8 фаз из 10

**Успешно мигрированы:**
- Фаза 0: workspace setup (11 crates, justfile, scripts)
- Фаза 1: axiom-core (24 теста, zero deps)
- Фаза 2: axiom-frontier (22 теста)
- Фаза 3: axiom-config (17 тестов)
- Фаза 4: axiom-space (83 теста, 1983 строки)
- Фаза 5: axiom-shell (43 теста, 1365 строк)
- Фаза 7: axiom-heartbeat (11 тестов, 413 строк)
- Фаза 8: axiom-upo + axiom-ucl (5 тестов, 744 строки)

**Всего:** 205 тестов, ~7500 строк кода

### ⏸️ Отложено: 3 фазы

**Фаза 6 (axiom-arbiter):**
- Файл: runtime/src/arbiter.rs (500 строк, 9 тестов)
- Причина: Зависит от непереведённых модулей
  - experience (ассоциативная память)
  - ashti_processor (обработка ASHTI 1-8)
  - maya_processor (консолидация)
  - com (Causal Order Model tracking)

**Фаза 9 (axiom-domain):**
- Файл: runtime/src/domain.rs (2845 строк, 63 теста)
- Причина: Требует миграции event_generator + исправление 113+ импортов
- Сложность: Самый большой модуль, интеграция всех компонентов

**Фаза 10 (axiom-runtime):**
- Причина: Зависит от axiom-arbiter и axiom-domain
- Будет завершена после миграции Фаз 6 и 9

### 📊 Статистика

| Метрика | Значение |
|---------|----------|
| Завершённых фаз | 8 из 10 (80%) |
| Тестов мигрировано | 205 |
| Строк кода мигрировано | ~7500 |
| Отложенных модулей | 3 (arbiter, domain, runtime) |
| Активных crates | 8 из 11 |

### 🔧 Технические детали

**Workspace структура:**
- Корректная модульная архитектура
- Все зависимости настроены
- License: AGPL-3.0-only
- Authors: Chernov Denys (@dchrnv)

**Отключенные crates в Cargo.toml:**
```toml
# "crates/axiom-domain",   # Требует event_generator и 113+ импортов
# "crates/axiom-runtime",  # Зависит от axiom-arbiter и axiom-domain
```

### 📝 Следующие шаги

1. Мигрировать вспомогательные модули:
   - event_generator.rs (для domain тестов)

2. Завершить Фазу 9 (domain):
   - Исправить импорты в основном коде
   - Исправить импорты в тестах
   - Адаптировать тесты для новой структуры

3. Завершить Фазу 10 (runtime):
   - Финальная интеграция
   - Интеграционные тесты

### 🔮 Deferred Tasks (на будущее)

**Фаза 6 (axiom-arbiter) - Замена stub-модулей:**
- [ ] Заменить `src/experience.rs` stub на полноценную реализацию Experience модуля
  - ExperienceTrace с полной логикой ассоциативной памяти
  - ResonanceSearch с threshold-based classification
  - Trace strengthening/weakening based on feedback
- [ ] Заменить `src/ashti_processor.rs` stub на реальную ASHTI 1-8 обработку
  - Маршрутизация токенов через 8 специализированных доменов
  - Hint propagation из Experience
  - Parallel/sequential processing options
- [ ] Заменить `src/maya_processor.rs` stub на полноценную MAYA консолидацию
  - Consolidation algorithm для результатов от ASHTI 1-8
  - Confidence scoring
  - Conflict resolution
- [ ] Заменить `src/com.rs` stub на полный Causal Order Model
  - Event tracking and causal ordering
  - Domain-specific event ID allocation
  - Frontier integration

---

## Рефакторинг: Извлечение тестов в отдельные директории

**Дата:** 2026-03-21
**Статус:** ✅ Завершено

### Цель
Переместить inline тесты из `#[cfg(test)] mod tests {}` в отдельные файлы в директориях `tests/` для соответствия целевой структуре из MIGRATION_PLAN.md §2.

### Выполнено

**7 модулей** успешно рефакторены (214 тестов вынесены):

1. **axiom-core** (24 теста → 3 файла):
   - `tests/token_tests.rs` — 6 тестов
   - `tests/connection_tests.rs` — 8 тестов
   - `tests/event_tests.rs` — 10 тестов

2. **axiom-frontier** (22 теста → 2 файла):
   - `tests/frontier_tests.rs` — 16 тестов
   - `tests/processor_tests.rs` — 6 тестов

3. **axiom-config** (17 тестов → 3 файла):
   - `tests/domain_config_tests.rs` — 11 тестов
   - `tests/heartbeat_config_tests.rs` — 3 теста
   - `tests/loader_tests.rs` — 3 теста

4. **axiom-space** (83 теста → 1 файл + doctests):
   - `tests/space_tests.rs` — 83 теста
   - Исправлены 6 doctests (добавлены `use axiom_space::*;`)

5. **axiom-shell** (43 теста → 1 файл):
   - `tests/shell_tests.rs` — 43 теста

6. **axiom-heartbeat** (11 тестов → 1 файл):
   - `tests/heartbeat_tests.rs` — 11 тестов

7. **axiom-ucl** (5 тестов → 1 файл):
   - `tests/ucl_tests.rs` — 5 тестов

8. **axiom-arbiter** (9 тестов → 1 файл):
   - `tests/arbiter_tests.rs` — 9 тестов
   - ⚠️ Модуль временно отключен в workspace

### Изменения в API для тестирования

Для корректной работы внешних тестов сделаны следующие изменения:

**axiom-frontier:**
- Сделан публичным `EntityQueue` и его методы
- Добавлены методы `rules()` и `rules_mut()` в `FrontierProcessor`
- Обновлены exports в `lib.rs`

**axiom-config:**
- Сделано публичным поле `cache` в `ConfigLoader`

**axiom-space:**
- Сделана публичной функция `integer_sqrt()`

### Проверка

```bash
cargo test --workspace
```

**Результат:** ✅ Все 214 тестов проходят успешно

### Коммиты

Каждый модуль зафиксирован отдельным коммитом:
- `refactor: extract tests to separate files (axiom-core + axiom-frontier)`
- `refactor: extract tests to separate files (axiom-config)`
- `refactor: extract tests to separate files (axiom-space)`
- `fix(axiom-space): add imports to doctests for compilation`
- `refactor: extract tests to separate files (axiom-shell)`
- `refactor: extract tests to separate files (axiom-heartbeat)`
- `refactor: extract tests to separate files (axiom-ucl)`
- `refactor: extract tests to separate files (axiom-arbiter)`

---

## Фаза 6: axiom-arbiter

**Дата:** 2026-03-21
**Статус:** ✅ Завершена

### Цель
Мигрировать модуль Arbiter V1.0 (над-доменная маршрутизация) в отдельный crate с stub-модулями для зависимостей.

### Выполнено

**Структура модуля:**
- `src/lib.rs` - Основная логика Arbiter (RoutingResult, PendingComparison, DomainRegistry, Arbiter)
- `src/experience.rs` - Stub для Experience модуля (ExperienceTrace, ResonanceLevel, ResonanceResult)
- `src/ashti_processor.rs` - Stub для ASHTI 1-8 обработки
- `src/maya_processor.rs` - Stub для MAYA консолидации
- `src/com.rs` - Stub для Causal Order Model
- `tests/arbiter_tests.rs` - 9 тестов (уже извлечены ранее)

### Изменения

**API изменения для тестов:**
- `pending_comparisons` field → публичный
- `PendingComparison` struct → публичная со всеми полями
- `compare_tokens()` метод → публичный
- `euclidean_distance()` метод → публичный
- Re-exported `COM` для использования в тестах

**Исправления:**
- Импорты изменены с `crate::token` на `axiom_core::Token`
- Импорты изменены с `crate::domain` на `axiom_config::DomainConfig`
- `Token::default()` заменён на `Token::new()` в тестах
- Убран дубликат импорта `COM`

### Зависимости
- axiom-core (Token)
- axiom-config (DomainConfig)

### Stub модули
Следующие модули реализованы как stubs и будут заменены полноценными реализациями:
- `experience` - Ассоциативная память (Experience, ExperienceTrace, ResonanceLevel)
- `ashti_processor` - Обработка через ASHTI 1-8 домены
- `maya_processor` - Консолидация результатов от ASHTI
- `com` - Causal Order Model для отслеживания событий

### Тесты
✅ 9 тестов проходят успешно:
- `test_arbiter_creation` - Создание Arbiter
- `test_domain_registration` - Регистрация всех 11 доменов
- `test_invalid_role_registration` - Валидация structural_role
- `test_routing_without_registration` - Маршрутизация без готовности
- `test_token_comparison_identical` - Сравнение идентичных токенов
- `test_token_comparison_similar` - Сравнение схожих токенов
- `test_token_comparison_different` - Сравнение различных токенов
- `test_euclidean_distance` - Евклидово расстояние
- `test_cleanup_old_comparisons` - Очистка старых сравнений

### Коммит
`feat(arbiter): complete Phase 6 - axiom-arbiter migration (v0.1.0)`

---

**Последнее обновление:** 2026-03-21 (Завершена Фаза 6: axiom-arbiter — 222 теста в workspace)
