# Axiom Roadmap

**Версия:** 9.6
**Дата:** 2026-03-27

---

## 🔜 Следующая задача: axiom-upo тесты

**Файл:** `crates/axiom-upo/src/lib.rs`
**Объём:** ~8 тестов, один файл `tests/upo_tests.rs`

- `DynamicTrace::new()` / `update()` / `is_active()`
- `UPOEngine::record_token_change()` / `record_connection_change()` / `generate_patch()`
- `TraceSourceType` enum значения (Token=1, Connection=2, Field=3)
- Size assertion: `DynamicTrace` = 128 байт, align 32

---

## 🔮 Долгосрочные цели

### Configuration System
YAML-загрузка пространственных параметров и semantic_contributions. Требует согласования с DomainConfig 128-byte constraint.

### Адаптеры
Python bindings, REST API, gRPC — нужны для внешней интеграции.

### Производительность
SIMD (AVX-512), incremental spatial hash rebuild — после стабилизации архитектуры.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-27
