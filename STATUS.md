# AXIOM Migration Status

**baseline_test_count:** 0
**current_test_count:** 24
**date_started:** 2026-03-21

---

## Фазы миграции

| Фаза | Crate | Статус | Дата | Тесты | Комментарий |
|-------|-------|--------|------|-------|-------------|
| 0 | workspace setup | ✅ | 2026-03-21 | 0 | Завершено: 11 crates, justfile, scripts |
| 1 | axiom-core | ✅ | 2026-03-21 | 24 | Token, Connection, Event (zero deps) |
| 2 | axiom-frontier | ⬜ | — | — | — |
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

**Последнее обновление:** 2026-03-21 (Фаза 1 ✅ завершена, 24 теста)
