# AXIOM Migration Status

**baseline_test_count:** 0
**current_test_count:** 46
**date_started:** 2026-03-21

---

## Фазы миграции

| Фаза | Crate | Статус | Дата | Тесты | Комментарий |
|-------|-------|--------|------|-------|-------------|
| 0 | workspace setup | ✅ | 2026-03-21 | 0 | Завершено: 11 crates, justfile, scripts |
| 1 | axiom-core | ✅ | 2026-03-21 | 24 | Token, Connection, Event (zero deps) |
| 2 | axiom-frontier | ✅ | 2026-03-21 | 22 | Frontier, storm, budget, processor |
| 3 | axiom-config | ⬜ | — | — | — |
| 4 | axiom-space | ⬜ | — | — | — |
| 5 | axiom-shell | ⬜ | — | — | — |
| 6 | axiom-arbiter | ⬜ | — | — | — |
| 7 | axiom-heartbeat | ⬜ | — | — | — |
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

**Последнее обновление:** 2026-03-21 (Фаза 2 ✅ завершена с processor, 46 тестов)
