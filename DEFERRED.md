# Axiom — Отложенные задачи

**Версия:** 9.0
**Обновлён:** 2026-04-02

---

## В работе (план в ROADMAP.md)

| Пункт | Фаза |
|-------|------|
| `partial_cmp().unwrap()` в experience.rs | Фаза 1 |
| `resonance.trace.unwrap()` в arbiter/lib.rs | Фаза 1 |
| `key.as_str().unwrap()` в loader.rs | Фаза 1 |
| `EventType` panic на неизвестном типе | Фаза 2 |
| Event: добавить source_domain, payload[8], snapshot_event_id | Фаза 3 |
| ShellEffector: extract_command всегда None | Фаза 3 |
| EngineSnapshot: com_next_id = 0 | Фаза 4 |
| Magic numbers в compare_tokens | Фаза 5 |
| structural_role = factory_preset в UclBuilder | Фаза 5 |
| Tick Scheduling (TickSchedule + tick_count) | Фаза 6 |

---

## Справка: MLEngine Real — input_size/output_size = 0

**Где:** `crates/axiom-agent/src/ml/engine.rs:120-123`
**Статус:** Не критично. Реальных ONNX-моделей нет, фича спекулятивная. Отложено на неопределённый срок.

При загрузке модели через `tract` размеры тензоров не извлекаются из model facts — оба поля остаются 0. Проверка `if *input_size > 0` никогда не срабатывает, что скрывает ShapeMismatch-ошибки при реальном использовании.

```rust
Ok(MLEngine::Real {
    model: Box::new(model),
    input_size: 0,  // Должно быть: из model.input_fact(0)
    output_size: 0, // Должно быть: из model.output_fact(0)
})
```

**Как исправить когда понадобится:**
```rust
let input_fact = model.input_fact(0)
    .map_err(|e| MLError::ModelLoad(format!("No input fact: {}", e)))?;
let input_size = input_fact.shape.as_concrete()
    .map(|s| s.iter().product::<usize>())
    .unwrap_or(0);
// Аналогично для output_fact(0)
```

---

## Отложенные функции (без срока)

### WebSocket и REST адаптеры

**Точка расширения:** `impl RuntimeAdapter for WebSocketAdapter` / `RestAdapter`
**Требует:** axum или actix-web
**Описание:** Архитектура готова (`RuntimeAdapter` trait в `axiom-runtime/src/adapters.rs`), реализаций нет.

---

### gRPC адаптер

**Точка расширения:** `impl RuntimeAdapter for GrpcAdapter`
**Требует:** tonic + protobuf codegen

---

### Python Bindings

**Точка расширения:** `impl RuntimeAdapter for PythonAdapter`
**Требует:** pyo3

---

### JSON-schema валидация конфигов

**Где:** `axiom-config/`
**Что:** Валидация YAML-конфигов по схеме при загрузке, миграция между версиями.
**Почему не сделано:** Нет практической необходимости при текущем масштабе.
