# AXIOM Migration Status

**baseline_test_count:** 0
**current_test_count:** 200
**date_started:** 2026-03-21

---

## Фазы миграции

| Фаза | Crate | Статус | Дата | Тесты | Комментарий |
|-------|-------|--------|------|-------|-------------|
| 0 | workspace setup | ✅ | 2026-03-21 | 0 | Завершено: 11 crates, justfile, scripts |
| 1 | axiom-core | ✅ | 2026-03-21 | 24 | Token, Connection, Event (zero deps) |
| 2 | axiom-frontier | ✅ | 2026-03-21 | 22 | Frontier, storm, budget, processor |
| 3 | axiom-config | ✅ | 2026-03-21 | 17 | DomainConfig, HeartbeatConfig, ConfigLoader |
| 4 | axiom-space | ✅ | 2026-03-21 | 83 | SpatialHashGrid, координаты, физика (1983 строки) |
| 5 | axiom-shell | ✅ | 2026-03-21 | 43 | Shell V3.0, семантические профили (1365 строк) |
| 6 | axiom-arbiter | ⏸️ | — | — | Пропущена (зависит от непереведённых модулей) |
| 7 | axiom-heartbeat | ✅ | 2026-03-21 | 11 | Heartbeat V2.0, периодическая активация (413 строк) |
| 8 | axiom-upo + axiom-ucl | ⬜ | — | — | — |
| 9 | axiom-domain | ⬜ | — | — | — |
| 10 | axiom-runtime | ⬜ | — | — | — |

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

**Последнее обновление:** 2026-03-21 (Фаза 7 ✅ завершена с Heartbeat V2.0, 200 тестов)
