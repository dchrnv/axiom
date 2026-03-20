# Axiom Roadmap

**Версия:** 5.0
**Дата:** 2026-03-20
**Статус:** v0.6.0 - Causal Time System

---

## 🎯 v0.6.0 - Causal Time System

**Цель:** Реализация полной модели времени согласно спецификациям из `docs/spec/time/`

**Спецификации:**
- Time Model V1.0 (конституционный документ)
- COM V1.0 (базовая реализация существует)
- Event-Driven V1
- Causal Frontier System V1
- Heartbeat V2.0

---

### Phase 1: Event-Driven Core ✅ COMPLETED (2026-03-20)

**Реализовано:**

- ✅ Event структура расширена
  - 12 семантических типов событий (TokenDecayed, TokenMerged, ConnectionWeakened, GravityUpdate, CollisionDetected, etc.)
  - Поле `pulse_id: u64` для Heartbeat V2.0
  - Удалено `timestamp` из Snapshot (нарушение Time Model V1.0)
  - 9 unit тестов

- ✅ EventGenerator реализован ([event_generator.rs](runtime/src/event_generator.rs))
  - check_decay() - затухание через причинный возраст
  - check_collision() - детектор столкновений
  - check_connection_stress() - контроль стресса связей
  - generate_gravity_update() - гравитационные обновления
  - Детерминистичные hash функции
  - 5 unit тестов

- ✅ COM обновлен для event-driven архитектуры
  - Интеграция с EventGenerator
  - Batch event processing (apply_batch, buffer_event, flush_batch)
  - Event aggregation (aggregate_events)
  - 7 новых тестов (всего 17)

**Итого:** 21 новый тест, все проходят. Commits: 25e6715, 9c4a9f9

---

### Phase 2: Causal Frontier System ✅ COMPLETED (2026-03-20)

**Реализовано:**

- ✅ Создан `causal_frontier.rs` модуль ([causal_frontier.rs](runtime/src/causal_frontier.rs))
  - Структура `EntityQueue` с дедупликацией через visited BitSet
  - `CausalFrontier` с типизированными очередями для токенов и связей
  - Детерминированный порядок обработки (FIFO)
  - 16 unit тестов

- ✅ Реализован основной алгоритм обработки Frontier
  - `push_token()`, `push_connection()` - добавление в frontier
  - `pop_token()`, `pop_connection()` - извлечение из frontier
  - `contains_token()`, `contains_connection()` - проверка наличия
  - `clear()`, `size()`, `is_empty()` - управление состоянием

- ✅ Интеграция Frontier с доменами
  - Добавлена структура `Domain` с `CausalFrontier` ([domain.rs](runtime/src/domain.rs))
  - Каждый домен имеет свой frontier (Domain isolation, Causal Frontier V1 §12)
  - FrontierState lifecycle: Empty → Active → Storm → Stabilized → Idle
  - 6 integration тестов

- ✅ Causal Storm mitigation
  - Storm detection (`frontier_size > storm_threshold`)
  - Causal budget (max_events_per_cycle)
  - Batch processing (интеграция с COM batch_buffer)
  - Memory limits (max_frontier_size)
  - Event aggregation (используется aggregate_events из COM)

- ✅ Тесты Causal Frontier
  - Локальность вычислений (каждый домен — свой frontier)
  - Детерминизм (FIFO порядок обработки)
  - Storm handling (порог, stabilization)
  - Idle state (frontier пуст)

**Итого:** 22 новых теста (16 causal_frontier + 6 domain). Всего: 130 passed. Commit: a6564de

---

### Phase 3: Heartbeat System ✅ COMPLETED (2026-03-20)

**Реализовано:**

- ✅ Создан `heartbeat.rs` модуль ([heartbeat.rs](runtime/src/heartbeat.rs))
  - `HeartbeatConfig` с пресетами (weak/medium/powerful/disabled)
  - `HeartbeatGenerator` - детерминистичная генерация по счётчику событий
  - `handle_heartbeat()` - добавление сущностей в Causal Frontier
  - 12 unit тестов

- ✅ Интеграция HeartbeatConfig с Domain
  - `Domain::with_heartbeat()` для кастомной конфигурации
  - `Domain::on_event()` - проверка нужен ли пульс
  - `Domain::handle_heartbeat()` - обработка пульса
  - Каждый домен имеет свой HeartbeatGenerator (изоляция)
  - 5 integration тестов

- ✅ Фоновые процессы через Frontier
  - Decay через причинный возраст - уже реализовано в EventGenerator ([event_generator.rs:42](runtime/src/event_generator.rs#L42))
  - Gravity updates - уже реализовано в EventGenerator ([event_generator.rs:110](runtime/src/event_generator.rs#L110))
  - Connection stress checks - уже реализовано в EventGenerator ([event_generator.rs:85](runtime/src/event_generator.rs#L85))
  - Heartbeat только добавляет сущности в Frontier, логика - в EventGenerator

- ✅ Тесты Heartbeat
  - Детерминизм генерации (test_heartbeat_generation_by_event_count)
  - COM совместимость (pulse_id, event_id)
  - Полное покрытие сущностей (test_heartbeat_full_coverage)
  - Idle state (test_heartbeat_idle_state)
  - Domain isolation (test_domain_heartbeat_isolation)

**Итого:** 17 новых тестов (12 heartbeat + 5 domain). Всего: 146 passed. Commit: 9ab5e78

---

### Phase 3.5: Frontier Processing Loop ✅ COMPLETED (2026-03-20)

**Реализовано:**

- ✅ Создан `Domain::process_frontier()` метод ([domain.rs:663](runtime/src/domain.rs#L663))
  - Соединяет все компоненты: Heartbeat → Frontier → EventGenerator → Events
  - Обрабатывает токены из frontier через EventGenerator.check_decay()
  - Генерирует гравитационные обновления через EventGenerator.generate_gravity_update()
  - Проверяет стресс связей через EventGenerator.check_connection_stress()
  - Уважает causal budget (max_events_per_cycle)
  - Обновляет FrontierState после обработки

- ✅ Unit тесты frontier processing
  - test_process_frontier_basic - базовая обработка frontier
  - test_process_frontier_decay - генерация TokenDecayed событий
  - test_process_frontier_gravity - генерация GravityUpdate событий
  - test_process_frontier_connection_stress - обработка стресса связей
  - test_process_frontier_budget_limit - соблюдение лимитов бюджета
  - test_process_frontier_empty - обработка пустого frontier
  - test_process_frontier_state_update - обновление состояния

- ✅ Integration тесты полного потока
  - test_full_heartbeat_to_event_flow - полный цикл: Heartbeat → Frontier → Events
  - test_full_flow_multiple_cycles - множественные циклы обработки
  - Проверка pulse_id в генерируемых событиях
  - Проверка domain isolation

**Архитектура:**

```
HeartbeatEvent → Domain.handle_heartbeat() → adds entities to Frontier
    ↓
Domain.process_frontier() → pop entities from Frontier
    ↓
EventGenerator.check_*() → check if event should be generated
    ↓
Generated Events → returned to caller for COM processing
```

**Итого:** 9 новых тестов. Всего: 155 passed. Commit: [pending]

---

### Phase 4: Time Model Validation ✅ COMPLETED (2026-03-20)

**Реализовано:**

- ✅ Audit кодовой базы на соблюдение Time Model V1.0
  - Проверены все core модули на использование `std::time`
  - Проверены все структуры на наличие wall-clock полей
  - Найдены нарушения в `ucl_command.rs`, `physics_processor.rs`, `domain.rs`

- ✅ Рефакторинг нарушений
  - `ucl_command.rs:generate_command_id()` - заменён `SystemTime` на атомарный счётчик
  - `physics_processor.rs` - добавлены комментарии что `Instant` используется только для метрик адаптера
  - `domain.rs` - исправлена инициализация `created_at`/`last_update` (было Unix timestamp, стало event_id = 0)
  - `domain.rs:validate()` - исправлена валидация для поддержки event_id = 0

- ✅ Cross-spec validation тесты ([lib.rs:169-291](runtime/src/lib.rs#L169-L291))
  - test_time_model_token_uses_event_id_for_age
  - test_time_model_connection_uses_event_id
  - test_time_model_domain_config_event_ids
  - test_time_model_decay_uses_causal_age
  - test_time_model_heartbeat_is_causal
  - test_time_model_no_wall_clock_in_core_structs
  - test_time_model_com_monotonic_causality
  - test_time_model_gravity_uses_causal_age

**Результат:**
- Все core структуры используют только event_id для времени
- Decay/gravity вычисляются через причинный возраст
- Heartbeat - легитимная причинность (count событий)
- Wall-clock time используется только в адаптерах (execution_time_us метрика)

**Итого:** 8 новых validation тестов. Всего: 163 passed. Commit: [pending]

---

### Phase 5: Integration & Testing

**Задачи:**

- [ ] Интеграция всех компонентов
  - COM → Event-Driven → Causal Frontier → Heartbeat
  - Domain lifecycle с новой архитектурой
  - End-to-end flow тесты

- [ ] Performance тесты
  - O(active_entities) сложность подтверждена
  - Нет глобальных проходов по состоянию
  - Масштабирование до 100k+ сущностей
  - Idle state - нулевая нагрузка CPU

- [ ] Детерминизм тесты
  - Воспроизводимость симуляций
  - Event replay
  - Snapshot & restore

---

## 📋 Технический долг (перенесено из v0.4.0)

- Падающие тесты размеров структур (требует внимания)
- Configuration System Integration (частично решено через DomainConfig)
- Factory Methods для остальных доменов
- Адаптеры и интерфейсы (отложено до v0.7.0)

---

## 📝 Принципы

- **STATUS.md** - только факты, завершенные релизы
- **ROADMAP.md** - только планы, удалять выполненное
- **DEFERRED.md** - технический долг и отложенные задачи
- **Минимализм** - краткость, структура, порядок

---

## 📝 Принципы

- **STATUS.md** - только факты, завершенные релизы
- **ROADMAP.md** - только планы, удалять выполненное
- **DEFERRED.md** - технический долг и отложенные задачи
- **Минимализм** - краткость, структура, порядок

---

**Обновлено:** 2026-03-20
