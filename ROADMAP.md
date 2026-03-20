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

### Phase 1: Event-Driven Core (базовая инфраструктура)

**Задачи:**

- [ ] Расширить `Event` структуру для event-driven модели
  - Добавить семантические типы событий (TokenMoved, TokenDecayed, ConnectionWeakened, etc.)
  - Добавить опциональное поле `pulse_id: u64` для Heartbeat
  - Обновить тесты Event

- [ ] Реализовать `EventGenerator` - механизм генерации событий из изменений состояния
  - Детектор трансформаций (collision, merge, decay triggers)
  - Правила генерации событий
  - Тесты детерминизма

- [ ] Обновить `COM` для поддержки event-driven архитектуры
  - Интеграция с EventGenerator
  - Batch event processing
  - Event aggregation для storm mitigation

---

### Phase 2: Causal Frontier System

**Задачи:**

- [ ] Создать `causal_frontier.rs` модуль
  - Структура `CausalFrontier` с типизированными очередями
  - `Queue<TokenId>`, `Queue<ConnectionId>`
  - BitSet для дедупликации (visited tracking)

- [ ] Реализовать основной алгоритм обработки Frontier
  - `push(entity)`, `pop()`, `contains()`, `clear()`, `size()`
  - Специализированные методы: `push_token()`, `push_connection()`
  - Детерминированный порядок обработки

- [ ] Интеграция Frontier с доменами
  - Каждый домен имеет свой frontier
  - Междоменное взаимодействие через COM
  - Lifecycle: empty → active → storm → stabilized → idle

- [ ] Causal Storm mitigation
  - Storm detection (`frontier_size > threshold`)
  - Batch events
  - Event aggregation
  - Causal budget (max_events_per_cycle)

- [ ] Тесты Causal Frontier
  - Локальность вычислений
  - Детерминизм
  - Storm handling
  - Idle state

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
