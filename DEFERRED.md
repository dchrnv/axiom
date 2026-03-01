# Axiom - Отложенные задачи

**Версия:** 1.0
**Создан:** 2026-02-11
**Обновлен:** 2026-02-12

---

## Принцип ведения

Систематический учет всех заглушек и отложенных функций для будущих версий.
Каждая запись: где находится, что отложено, почему, когда планируется.

---

## 1. Портирование старых benches

**Где:** `runtime/benches/`  
**Что отложено:** token_bench, connection_v3_bench, grid_bench, graph_bench, intuition_bench, experience_stream_bench, system_integration_bench, token_1m_bench  
**Почему:** используют старый API (CoordinateSpace, EntityType, token::flags и др.), несовместимый с UPO v2.1  
**Когда:** после стабилизации нового API Token/Connection

---

**Последнее обновление:** 2026-03-01
**Создано в рамках:** Axiom
