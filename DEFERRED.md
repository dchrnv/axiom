# Axiom — Отложенные задачи

**Версия:** 7.0
**Обновлён:** 2026-03-30

---

## Технический долг

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
