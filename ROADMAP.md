# Axiom Roadmap

**Версия:** 8.2
**Дата:** 2026-03-21
**Статус:** v0.8.1 завершена ✅ (SPACE ↔ Shell Integration), Phase 3.1 выполнена

---

## 🎯 Текущий план: Shell V3.0 + Integration

### ✅ PHASE 1: SPACE V6.0 (v0.7.0) - ЗАВЕРШЕНО
См. [STATUS.md](STATUS.md) для деталей: 285 тестов, 11/11 задач, коммит 229461a

---

### ✅ PHASE 3: Интеграция SPACE ↔ Shell (v0.8.1) - ЗАВЕРШЕНО (Phase 3.1)

**✅ 3.1 Полный цикл взаимодействия** (v0.8.1, complete)
- ✅ SPACE: столкновение → `TokenCollision` событие (уже было в v0.7.0)
- ✅ Connection триггерит Shell dirty flag (process_connection_event)
- ✅ Shell пересчитывается для затронутых токенов (mark_dirty + reconciliation)
- ✅ Интеграция в Domain::process_frontier (collision + connection maintenance)
- ✅ Тесты: 3 integration tests (process_connection_event, connection_maintenance, end-to-end)
- 336 тестов pass

**3.2 Конфигурация столкновений**
- Стратегии обработки столкновений:
  - Resonance (создать Connection)
  - Repulsion (оттолкнуть)
  - Merge (слить токены)
  - Ignore (только статистика)
- Конфигурация по доменам
- Тесты: разные стратегии

**3.3 Cross-module тесты**
- SPACE + Shell integration tests
- SPACE + Heartbeat + Shell
- SPACE + Frontier + Shell
- Performance benchmarks
### PHASE 4: Документация и финализация (v0.8.2)

**4.1 Обновление STATUS.md**
- v0.7.0 - SPACE V6.0
- v0.8.0 - Shell V3.0
- v0.8.1 - SPACE ↔ Shell integration
- Обновить таблицу модулей

**4.2 Обновление DEFERRED.md**
- Удалить завершённые секции
- Добавить новые открытые вопросы:
  - Shell density field (V3.1)
  - Incremental spatial hash rebuild
  - Spring dynamics (Connection.ideal_dist)

**4.3 Обновление ROADMAP.md**
- Отметить v0.7.0-v0.8.2 как завершённые
- Планирование v0.9.0

**4.4 Спецификации**
- Проверить соответствие кода спекам
- Обновить примеры в спеках (если нужно)

---

## 📊 Порядок выполнения

### Критический путь:
- **SPACE**: базовые структуры → spatial hash → события → интеграция с Domain/Frontier/Heartbeat
- **Shell**: базовые структуры → справочник → вычисление → dirty tracking → интеграция с Domain/Frontier/Heartbeat
- **SPACE ↔ Shell**: полный цикл столкновение → Connection → Shell update

### Зависимости:
- Shell зависит от Connection (уже есть ✅)
- SPACE зависит от Token.position/velocity (уже есть ✅)
- Оба зависят от Causal Frontier (уже есть ✅)
- Оба зависят от Heartbeat (уже есть ✅)

### Оценка:
- ✅ SPACE V6.0: 11 шагов (1.1-1.11) - **ЗАВЕРШЕНО**
- ✅ Shell V3.0: 9 шагов (2.1-2.10, skip 2.9) - **ЗАВЕРШЕНО**
- Integration: ~3 шага (3.1-3.3) - **следующая задача**
- Docs: ~4 шага (4.1-4.4)
- **Всего: ~27 шагов** (**20 завершено, ~7 осталось**) 

---

## 📋 Технический долг

### 🔧 Средний приоритет:
- **Адаптеры и интерфейсы** - CLI, REST, WebSocket (отложено)

**Детали:** См. [DEFERRED.md](DEFERRED.md) для полного списка

---

## 📝 Принципы

- **STATUS.md** - только факты, завершенные релизы
- **ROADMAP.md** - только планы, удалять выполненное
- **DEFERRED.md** - технический долг и отложенные задачи
- **Минимализм** - краткость, структура, порядок

---

**Обновлено:** 2026-03-21
