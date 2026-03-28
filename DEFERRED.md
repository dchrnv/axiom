# Axiom — Отложенные задачи

**Версия:** 5.0
**Обновлён:** 2026-03-28

---

## 1. Технический долг

### Token / Connection preset loading

**Где:** `crates/axiom-config/src/`, `crates/axiom-core/src/`
**Что:** `Token::load_presets()`, `Connection::load_presets()` — загрузка пресетов из YAML
**Почему не сделано:** Нет конкретного потребителя. Все текущие пользователи используют конструкторы напрямую.

---

### Event Bus (pub/sub)

**Где:** `crates/axiom-runtime/src/adapters.rs`
**Что:** Подписочная модель поверх `EventObserver` — роутинг событий по типу, фильтры, приоритеты
**Почему не сделано:** Нет внешних потребителей, которым это требуется прямо сейчас.
**Текущий вариант:** `Gateway::register_observer` покрывает базовые нужды.

---

### Configuration — Advanced Features

**Где:** `config/`
**Что:** JSON-schema валидация, hot reload конфигураций, миграция между версиями
**Почему не сделано:** Нет практической необходимости при текущем масштабе.

---

## 2. Отложенные функции

### Фрактальные уровни (Этап 7 Шаг 3)

**Суть:** Протокол 10→0 — MAYA(10) одного AshtiCore подаёт выход на SUTRA(0) следующего. Цепочка нескольких AshtiCore.

**Почему отложено:** Усложнение архитектуры без конкретного use-case. Базовая система полна без этого.

**Что потребуется:**
- `FractalChain` — связывает два AshtiCore: `maya_output → sutra_input`
- `AshtiCore::set_sutra_input(token)` + `take_maya_output()`
- Обмен SkillSet между уровнями через `export/import_batch`
- Тесты двухуровневой цепочки

---

### REST API

**Точка расширения:** `impl RuntimeAdapter for RestAdapter`
**Требует:** axum или actix-web (внешние crates)

---

### gRPC Adapter

**Точка расширения:** `impl RuntimeAdapter for GrpcAdapter`
**Требует:** tonic + protobuf codegen (внешние crates)

---

### Python Bindings

**Точка расширения:** `impl RuntimeAdapter for PythonAdapter`
**Требует:** pyo3 (внешний crate)

---

### SIMD-оптимизация

**Где:** `crates/axiom-space/src/`, физика поля
**Что:** Пакетная обработка токенов через AVX-512 / AVX2
**Когда:** При появлении конкретных бенчмарков с доказанным bottleneck
