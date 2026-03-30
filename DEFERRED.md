# Axiom — Отложенные задачи

**Версия:** 7.0
**Обновлён:** 2026-03-30

---

## Технический долг

### TOKEN_FLAG_GOAL дублируется в трёх местах

**Где:** `crates/axiom-arbiter/src/lib.rs`, `src/experience.rs`, `src/ashti_processor.rs`
**Что:** Константа `TOKEN_FLAG_GOAL: u16 = 0x0001` определена в трёх местах внутри одного крейта.
**Почему не сделано:** Во избежание циклических импортов между подмодулями при реализации 13D.
**Решение:** Вынести в `axiom-core` (туда же, куда входит `Token`) и импортировать единожды.

---

### JSON-schema валидация конфигов

**Где:** `config/`
**Что:** Валидация YAML-конфигов по схеме при загрузке, миграция между версиями
**Почему не сделано:** Нет практической необходимости при текущем масштабе.

---

## Отложенные функции

### REST API

**Точка расширения:** `impl RuntimeAdapter for RestAdapter`
**Требует:** axum или actix-web

---

### gRPC Adapter

**Точка расширения:** `impl RuntimeAdapter for GrpcAdapter`
**Требует:** tonic + protobuf codegen

---

### Python Bindings

**Точка расширения:** `impl RuntimeAdapter for PythonAdapter`
**Требует:** pyo3
