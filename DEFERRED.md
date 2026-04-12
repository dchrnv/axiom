# Axiom — Отложенные задачи

**Версия:** 17.0
**Обновлён:** 2026-04-10

---

## Ждут интернета

### D-04 — axiom-persist: serde_json вместо bincode

**Где:** `crates/axiom-persist/src/writer.rs`, `loader.rs`

JSON в 3–5× крупнее и в 2–4× медленнее бинарного. Критично при тысячах traces.

**Как исправить:** `bincode = { version = "2", features = ["serde"] }` в `[workspace.dependencies]`, заменить `serde_json` на `bincode::serde`. Расширение `.json` → `.bin`.

**Когда:** При появлении интернета.

---

### D-07 — JSON-schema валидация конфигов

**Где:** `axiom-cli.yaml`, `crates/axiom-config/src/loader.rs`

Нет чёткой ошибки при невалидном YAML — panic или молчаливый дефолт.

**Что нужно:** `schemars` + `jsonschema`, `#[derive(JsonSchema)]` на `CliConfig`, `TickScheduleConfig`, `DomainConfig`, `PersistenceConfig`. Опционально: `axiom-cli --dump-schema`.

**Когда:** При появлении интернета.

---

## Ждут конкретного триггера

### D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`

Проверка `if *input_size > 0` скрывает ShapeMismatch-ошибки.

**Когда:** При первой реальной ONNX-модели.

---

## Внешние адаптеры (требуют интернета)

**Точка расширения:** `RuntimeAdapter` trait в `axiom-runtime/src/adapters.rs`.

| Адаптер | Требует | Статус |
|---|---|---|
| WebSocket | axum / actix-web | не начат |
| REST API | axum / actix-web | не начат |
| gRPC | tonic + protobuf | не начат |
| Python bindings | pyo3 | не начат |

**Когда:** При появлении интернета и конкретной задачи интеграции.
