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

### Phase 3: Heartbeat System

**Задачи:**

- [ ] Создать `heartbeat.rs` модуль
  - `HeartbeatGenerator` структура
  - Генерация по счётчику событий (детерминизм)
  - `pulse_number` tracking

- [ ] Добавить `HeartbeatConfig` в DomainConfig
  - `interval`, `batch_size`, `connection_batch_size`
  - Флаги: `enable_decay`, `enable_gravity`, `enable_connection_maintenance`
  - Пресеты для слабого/среднего/мощного оборудования

- [ ] Реализовать `handle_heartbeat()` обработчик
  - Добавление сущностей в Causal Frontier (НЕ выполнение логики)
  - Детерминированный выбор: `(pulse_number * batch_size + offset) % total`
  - Интеграция с Frontier

- [ ] Фоновые процессы через Frontier
  - Decay через причинный возраст (`current_event_id - last_event_id`)
  - Gravity updates
  - Connection stress checks
  - Thermodynamics (температурная адаптация)

- [ ] Тесты Heartbeat
  - Детерминизм генерации
  - COM совместимость
  - Полное покрытие сущностей за N пульсов
  - Idle state (нет событий → нет пульсов)

---

### Phase 4: Time Model Validation

**Задачи:**

- [ ] Audit кодовой базы на соблюдение Time Model V1.0
  - Проверка: нет `std::time`, `SystemTime`, `Instant` в ядре
  - Проверка: нет `timestamp_ms`, `duration_seconds` в core-структурах
  - Проверка: все "длительности" через `event_id` разницу
  - Проверка: нет `sleep()`, `delay()`, таймеров

- [ ] Рефакторинг нарушений (если найдены)
  - Замена wall-clock на причинный возраст
  - Вынос real-time в адаптеры (вне ядра)

- [ ] Cross-spec validation тесты
  - Token, Connection, Domain используют только `event_id` для возраста
  - Все decay/gravity/cooling вычисляются через причинный возраст
  - Heartbeat - легитимная причинность

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
