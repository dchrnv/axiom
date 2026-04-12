# Axiom — Отложенные задачи

**Версия:** 19.0
**Обновлён:** 2026-04-12

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
