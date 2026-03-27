# Axiom - Отложенные задачи

**Версия:** 4.0
**Создан:** 2026-02-11
**Обновлен:** 2026-03-27

---

## Принципы разработки

**Правила и стандарты проекта:** см. [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

Ключевые принципы (краткое напоминание):
- Асимметрия Token (бытие) и Connection (действие) — поля не дублируются
- Время только через `event_id` — никакого wall-clock в ядре
- Спецификации в `docs/spec/` — единственный источник правды, не изменять
- 100% тест coverage для критических модулей
- `#![deny(unsafe_code)]` и `#![warn(missing_docs)]` во всех crates

---

## Принцип ведения

Систематический учет всех заглушек, отложенных функций и старых планов для будущих версий.
Каждая запись: где находится, что отложено, почему, когда планируется.

---

## 1. 🔥 КРИТИЧЕСКИЕ ПРОБЛЕМЫ - ТРЕБУЮТ ВНИМАНИЯ

_(Нет критических проблем на данный момент)_

---

## 1.5. 🧪 ОТСУТСТВУЮЩИЕ ТЕСТЫ

### axiom-upo (UPO v2.2) — 0 тестов

**Файл:** `crates/axiom-upo/src/lib.rs` (388 строк)
**Статус:** Перенесен из runtime/src/upo.rs без тестов
**Дата:** 2026-03-21 (Фаза 8 миграции)

**Что нужно протестировать:**
- [ ] `DynamicTrace::new()` — создание следа с начальными значениями
- [ ] `DynamicTrace::update()` — обновление полей (position, temperature, mass, valence)
- [ ] `DynamicTrace::is_active()` — проверка флагов активности
- [ ] `UPOEngine::record_token_change()` — запись изменения токена
- [ ] `UPOEngine::record_connection_change()` — запись изменения связи
- [ ] `UPOEngine::generate_patch()` — генерация патча изменений
- [ ] `TraceSourceType` enum — корректность значений (Token=1, Connection=2, Field=3)
- [ ] Size assertions для DynamicTrace (128 байт, align 32)

**Причина отсутствия:** Исходный файл не содержал тестов. UPO тестируется как часть интеграционных тестов domain.

**Приоритет:** Низкий (можно добавить после завершения миграции)

---

### ~~axiom-arbiter (Arbiter V1.0) — Stub-модули~~ ✅ ЗАВЕРШЕНО 2026-03-26

**Статус:** ✅ Все stub-модули заменены реальными реализациями
**Дата завершения:** 2026-03-26 (Option A — перед Фазой 10)

**Что реализовано:**
- ✅ `experience.rs` — `resonance_search()` с threshold-based classification, weight-based trace selection, pattern distance (temperature/mass/valence/position); `strengthen_trace()`, `weaken_trace()`; `ExperienceTrace` расширен: `last_used`, `success_count`, `pattern_hash`; capacity eviction по минимальному весу
- ✅ `ashti_processor.rs` — `process_token()` с hint blending (blend_alpha = trace.weight × feedback_weight_delta) + 8 domain-specific transformers по structural_role (spatial/temporal/logical/semantic/thermal/causal/resonant/meta)
- ✅ `maya_processor.rs` — `consolidate_results()` с weighted averaging, confidence scoring (4 поля, tolerance-based agreement), median fallback при confidence < 0.5, `arbiter_flags` bit 0 для force-median
- ✅ `com.rs` — `next_event_id()` с глобальным монотонным счётчиком + per-domain event count tracking; методы `domain_event_count()`, `current_id()`

**Тесты:** 9 существующих ✅ + 12 experience tests + 5 COM tests = **26 итого**

---

### ~~axiom-domain — AshtiCore~~ ✅ ЗАВЕРШЕНО 2026-03-27

**Статус:** ✅ Реализовано — `crates/axiom-domain/src/ashti_core.rs`, 13 тестов
**AxiomEngine рефакторен** — `ashti: AshtiCore` заменяет HashMap-based domains+arbiter

---

## 2. 📋 СТАРЫЕ ПЛАНЫ ИЗ ROADMAP (ОТЛОЖЕНО)

Весь контент из предыдущего ROADMAP v2.3 перемещен сюда как отложенные задачи.

### 2.1 v0.3.0 - Внутренние интерфейсы и адаптеры (ОТЛОЖЕНО)

**Статус:** Отложено на неопределенный срок
**Причина:** Изменение приоритетов проекта

#### Внутренние интерфейсы (Commands, Queries, Events)
- [ ] **Определить Domain Commands** (CreateDomain, UpdateDomain, DeleteDomain)
- [ ] **Определить Domain Queries** (GetDomain, ListDomains, ValidateDomain)
- [ ] **Определить Domain Events** (DomainCreated, DomainUpdated, DomainDeleted)
- [ ] **Создать структуры данных** для внутренних операций
- [ ] **Реализовать обработку Commands** в Domain модуле
- [ ] **Реализовать обработку Queries** в Domain модуле
- [ ] **Реализовать генерацию Events** в Domain модуле
- [ ] **Написать unit тесты** для внутренних интерфейсов

#### Адаптеры (Transport Layer)
- [ ] **CLI адаптер** - трансляция CLI → внутренние Commands/Queries
- [ ] **REST адаптер** - эндпоинты для Commands/Queries
- [ ] **WebSocket адаптер** - потоковые Events
- [ ] **Тестирование адаптеров** - unit и интеграционные тесты
- [ ] **Документация адаптеров** - примеры использования

#### Интеграция и документация
- [ ] **Интеграционные тесты** всех адаптеров
- [ ] **Бенчмарки производительности** внутренних интерфейсов
- [ ] **Документация внутренних интерфейсов** (Commands, Queries, Events)
- [ ] **CI/CD pipeline** для автоматического тестирования
- [ ] **Мониторинг покрытия** - метрики в STATUS.md

---

### 2.2 v0.3.1 - Расширение и оптимизация (ОТЛОЖЕНО)

**Статус:** Отложено
**Задачи:**
- [ ] **gRPC адаптер** - межсервисное взаимодействие
- [ ] **Hot reload** - перезагрузка конфигураций без перезапуска
- [ ] **Бенчмарки** - производительность всех адаптеров
- [ ] **Мониторинг** - метрики производительности адаптеров
- [ ] **Документация** - полная документация всех интерфейсов

---

### 2.3 v0.4.0 - Performance Optimization (ОТЛОЖЕНО)

**Статус:** Отложено
**Задачи:**
- [ ] **SIMD пакетная обработка** - использование AVX-512
- [ ] **Мониторинг производительности** - метрики и дашборд
- [ ] **Масштабирование адаптеров** - поддержка высокой нагрузки

**Завершено в v0.6.2:**
- ✅ Event оптимизация до 64 байт (COM V1.1)
- ✅ DynamicTrace оптимизация до 32 байт (UPO V2.3)

---

## 3. 🔧 ЗАДАЧИ ПО УЛУЧШЕНИЮ - СРЕДНИЙ ПРИОРИТЕТ

### 3.1 SPACE V6.0 - YAML Configuration (Phase 1.10-1.11)

**Где:** `runtime/src/space.rs`, `runtime/src/domain.rs`, `runtime/src/config/mod.rs`
**Что отложено:** YAML конфигурация пространственных параметров
**Текущий статус:** Hardcoded константы (collision_radius: 100, gravity_scale_shift: 8)
**Почему отложено:** Конфликт с DomainConfig 128-byte constraint + требует ConfigLoader интеграции
**Когда планируется:** После завершения базовой Configuration System

**Hardcoded параметры (временное решение):**
- `collision_radius`: 100 (в функции `detect_collisions`)
- `gravity_scale_shift`: 8 (в GravityModel::InverseSquare)
- `rebuild_frequency`: 50 (в Domain::should_rebuild_spatial_grid)

**Требуется для полной реализации:**
- [ ] Добавить SpatialConfig структуру (collision_radius, gravity_scale_shift)
- [ ] Расширить DomainConfig (сейчас 128b, нужны +3-4 байта) ИЛИ
- [ ] Создать отдельную конфигурацию для spatial параметров
- [ ] YAML схема для spatial.yml (пресеты: tight, medium, loose)
- [ ] Интеграция с ConfigLoader::load_config()
- [ ] Валидация spatial параметров
- [ ] Тесты загрузки spatial конфигураций

**Альтернативы:**
1. Оставить hardcoded (текущее решение, детерминистично)
2. Передавать параметры через HeartbeatConfig (но там тоже размер ограничен)
3. Создать отдельный SpatialConfig вне DomainConfig

**Связано с:**
- Section 3.2: Configuration System - Preset Loading
- Section 3.4: Configuration System - Advanced Features

---

### 3.2 Configuration System - Preset Loading

**Где:** `runtime/src/config/mod.rs`, `runtime/src/token.rs`, `runtime/src/connection.rs`
**Что отложено:** Загрузка пресетов через ConfigLoader
**Текущий статус:** Структура готова, импорты сохранены, загрузка не реализована
**Когда планируется:** Когда потребуется динамическая конфигурация

**Требуется:**
- [ ] Token::load_presets() - загрузка пресетов токенов из YAML
- [ ] Connection::load_presets() - загрузка пресетов связей из YAML
- [ ] Валидация пресетов
- [ ] Тесты загрузки

---

### 3.3 Event Bus (pub/sub)

**Что отложено:** Подписочная модель для внешних потребителей событий
**Почему:** Нужна только при появлении внешних потребителей (REST, gRPC, Python bindings)
**Когда планируется:** Неопределённо — после реализации адаптеров

**Что потребуется:**
- [ ] Trait `EventSubscriber` / механизм подписки
- [ ] Роутинг `pending_events` по подписчикам
- [ ] Интеграция с адаптерами (REST, WebSocket)

---

### 3.4 Configuration System - Advanced Features

**Что осталось (низкий приоритет):**
- [ ] JSON схемы валидация
- [ ] Hot reload конфигураций
- [ ] Миграция конфигураций между версиями

---

### 3.5 Shell V3.0 - YAML Configuration (Phase 2.3)

**Где:** `runtime/src/shell.rs`, `runtime/src/config/mod.rs`
**Что отложено:** YAML конфигурация семантических вкладов (semantic_contributions.yaml)
**Текущий статус:** Hardcoded `default_ashti_core()` с 7 категориями
**Почему отложено:** Требует ConfigLoader интеграции (см. Section 3.2)
**Когда планируется:** После завершения базовой Configuration System

**Hardcoded категории (временное решение):**
- `0x01` Structural: [20, 5, 0, 0, 5, 0, 0, 0] - Physical, Sensory, Cognitive layers
- `0x02` Semantic: [0, 0, 0, 0, 15, 0, 0, 10] - Cognitive, Abstract layers
- `0x03` Causal: [0, 0, 5, 0, 15, 0, 10, 8] - Motor, Cognitive, Temporal, Abstract
- `0x04` Experiential: [5, 20, 0, 15, 0, 0, 0, 0] - Physical, Sensory, Emotional
- `0x05` Social: [0, 0, 0, 5, 0, 25, 0, 0] - Emotional, Social layers
- `0x06` Temporal: [0, 0, 0, 0, 5, 0, 25, 0] - Cognitive, Temporal layers
- `0x07` Motor: [10, 0, 25, 0, 5, 0, 0, 0] - Physical, Motor, Cognitive layers

**Требуется для полной реализации:**
- [ ] YAML schema: `config/schema/semantic_contributions.yaml`
- [ ] Формат описания категорий и переопределений:
  ```yaml
  categories:
    0x01:  # Structural
      name: "Structural"
      layers: [20, 5, 0, 0, 5, 0, 0, 0]
    0x02:  # Semantic
      name: "Semantic"
      layers: [0, 0, 0, 0, 15, 0, 0, 10]
    0x03:  # Causal
      name: "Causal"
      layers: [0, 0, 5, 0, 15, 0, 10, 8]
    0x04:  # Experiential
      name: "Experiential"
      layers: [5, 20, 0, 15, 0, 0, 0, 0]
    0x05:  # Social
      name: "Social"
      layers: [0, 0, 0, 5, 0, 25, 0, 0]
    0x06:  # Temporal
      name: "Temporal"
      layers: [0, 0, 0, 0, 5, 0, 25, 0]
    0x07:  # Motor
      name: "Motor"
      layers: [10, 0, 25, 0, 5, 0, 0, 0]
  overrides:
    0x0310:  # Emotional_Cause (Causal category override)
      name: "Emotional_Cause"
      layers: [0, 0, 0, 20, 10, 5, 8, 5]
    0x0311:  # Physical_Cause
      name: "Physical_Cause"
      layers: [15, 5, 10, 0, 8, 0, 10, 0]
    0x0412:  # Aesthetic_Feel (Experiential override)
      name: "Aesthetic_Feel"
      layers: [2, 15, 0, 18, 5, 0, 0, 12]
  ```
- [ ] Пресеты: `ashti_core.yaml`, `custom.yaml`
- [ ] Интеграция с ConfigLoader::load_config()
- [ ] Валидация схемы: проверка диапазонов [0-255], суммы вкладов
- [ ] Тесты: загрузка, валидация пресетов

**Альтернативы:**
1. Оставить hardcoded `default_ashti_core()` (текущее решение, детерминистично)
2. Добавить runtime конфигурацию через HeartbeatConfig/DomainConfig (требует расширения структур)
3. Создать отдельный ShellConfig файл (semantic_contributions.yaml)

**Связано с:**
- Section 3.2: Configuration System - Preset Loading
- Section 3.1: SPACE V6.0 - YAML Configuration (аналогичный случай)

---

### 3.6 Shell V3.0 - Runtime Configuration (Phase 2.9)

**Где:** `runtime/src/heartbeat.rs`, `runtime/src/config/mod.rs`
**Что отложено:** Runtime YAML конфигурация для Shell cache параметров
**Текущий статус:** Hardcoded флаг `enable_shell_reconciliation` в HeartbeatConfig пресетах
**Почему отложено:** Требует ConfigLoader интеграции (см. Section 3.2)
**Когда планируется:** После завершения базовой Configuration System

**Текущее решение (hardcoded):**
```rust
// HeartbeatConfig presets
weak:     enable_shell_reconciliation = false  // Disabled for weak hardware
medium:   enable_shell_reconciliation = true   // Enabled for medium+ hardware
powerful: enable_shell_reconciliation = true   // Enabled for medium+ hardware
disabled: enable_shell_reconciliation = false  // Disabled when heartbeat disabled
```

**Требуется для полной реализации:**
- [ ] YAML schema: `config/runtime/shell_cache.yaml`
- [ ] Формат конфигурации:
  ```yaml
  shell_cache:
    enable_shell_reconciliation: true
    reconciliation_log: false  # Опционально: логировать drift detection
    reconciliation_batch_size: 10  # Опционально: сколько токенов проверять за раз
  ```
- [ ] Интеграция с ConfigLoader::load_runtime_config()
- [ ] Валидация параметров
- [ ] Тесты: разные конфигурации shell_cache

**Альтернативы:**
1. Оставить hardcoded в HeartbeatConfig пресетах (текущее решение)
2. Добавить ShellCacheConfig структуру отдельно от HeartbeatConfig
3. Расширить HeartbeatConfig дополнительными полями (reconciliation_batch_size, etc.)

**Связано с:**
- Section 3.2: Configuration System - Preset Loading
- Section 3.5: Shell V3.0 - YAML Configuration (semantic_contributions.yaml)

---

## 4. 🟢 НИЗКИЙ ПРИОРИТЕТ - ДОЛГОСРОЧНЫЕ ЦЕЛИ

### 4.1 Python Adapter

**Где:** `runtime/src/python_adapter.rs`
**Что отложено:** Python bindings для UCL V2.0
**Почему:** Внешняя интеграция и CLI
**Когда планируется:** Неопределенно

---

### 4.2 REST API

**Где:** `runtime/src/rest_api.rs`
**Что отложено:** HTTP endpoints для доменов
**Почему:** Веб-интерфейс и удаленное управление
**Когда планируется:** Неопределенно

---

### 4.3 Performance Benchmarks

**Где:** `runtime/benches/`
**Что отложено:** Бенчмарки UCL V2.0 производительности
**Почему:** Измерение эффективности zero-allocation
**Когда планируется:** Когда будет базовый функционал

---



## 📊 СВОДКА ПО ПРИОРИТЕТАМ

### 🔥 КРИТИЧЕСКИЕ:
_(Нет критических проблем)_

### 🔧 СРЕДНИЙ:
1. SPACE V6.0 - YAML Configuration (Section 3.1)
2. Configuration System - Preset Loading (Section 3.2)
3. Events System Integration (Section 3.3)
4. Configuration Advanced Features (Section 3.4)
5. Shell V3.0 - YAML Configuration (Section 3.5)
6. Shell V3.0 - Runtime Configuration (Section 3.6)

### 🟢 НИЗКИЙ:
1. **axiom-upo тесты** — 0 тестов, можно добавить в любое время
2. Python Adapter
3. REST API

### 📦 АРХИВНЫЕ:
- ~~axiom-arbiter stub-модули~~ ✅ Завершено 2026-03-26
- v0.3.0, v0.4.0 планы (отложено)

---

## 📝 История изменений

**2026-03-27 (Events System audit):**
- Секция 3.3 переписана: удалено устаревшее (EventType уже есть, COM реализован)
- Оставлена только реальная отложенная задача: Event Bus (pub/sub) для внешних потребителей
- Задача "proброс физических событий в pending_events" перенесена в ROADMAP как следующая

**2026-03-27 (Causal Frontier V2.0):**
- ✅ Causal Frontier V2.0 реализован: `FrontierConfig` + presets, `FrontierEntity` enum, `begin_cycle/end_cycle`, `frontier_growth_rate`, `StormMetrics`, BitVec dedup, `FrontierProcessor` обновлён
- ✅ axiom-domain, axiom-heartbeat обновлены под V2.0 API
- 372 тестов (+6 к 366)

**2026-03-27 (AshtiCore complete):**
- ✅ AshtiCore реализован: `ashti_core.rs` (13 тестов), AxiomEngine рефакторен
- ✅ axiom-bench: первые результаты зафиксированы в `docs/bench/RESULTS.md`
- Удалены из НИЗКОГО: AshtiCore ✅, axiom-bench ✅
- Обновлена версия (3.9)

**2026-03-27 (Migration complete):**
- ✅ Закрыта секция 1.5: axiom-arbiter stub-модули (все 4 реализованы, 26 тестов)
- Обновлена сводка: убран "axiom-arbiter" из СРЕДНИХ приоритетов
- Добавлены в НИЗКИЙ: AshtiCore (разблокировано), axiom-bench (создан, нужен запуск)

**2026-03-26 (Option A + Phase 9 + Phase 10):**
- ✅ Option A: все 4 stub-модуля axiom-arbiter заменены (experience, ashti_processor, maya_processor, com)
- ✅ Phase 9: axiom-domain мигрирован (71 тест), AshtiCore перенесён в DEFERRED с отметкой "блокировка снята"
- ✅ Phase 10: axiom-runtime реализован полностью (30 тестов): AxiomEngine, Guardian, Snapshot, adapters, orchestrator
- ✅ Cleanup: 0 warnings, 0 dead code по всему workspace
- Создан axiom-bench crate (в процессе запуска)

**2026-03-21 (Migration Phase 6):**
- Добавлено: Секция 1.5 (axiom-arbiter stub-модули требуют замены)
- Обновлена сводка по приоритетам (добавлен пункт 1 в СРЕДНИЙ приоритет)
- Причина: Фаза 6 миграции завершена со stub-реализациями для experience, ashti_processor, maya_processor, com
- Детализированы требования для замены каждого stub-модуля

**2026-03-21 (v0.8.0 Phase 2.9):**
- Добавлено: Секция 3.6 (Shell V3.0 - Runtime Configuration)
- Обновлена сводка по приоритетам (добавлен пункт 6)
- Причина: Отложена Runtime YAML конфигурация shell_cache (требует ConfigLoader)

**2026-03-21 (v0.8.0 Phase 2.3):**
- Добавлено: Секция 3.5 (Shell V3.0 - YAML Configuration)
- Обновлена сводка по приоритетам (добавлен пункт 5)
- Причина: Отложена YAML конфигурация semantic_contributions.yaml (требует ConfigLoader)

**2026-03-21 (v0.7.0 Phase 1.10-1.11):**
- Добавлено: Секция 3.1 (SPACE V6.0 - YAML Configuration)
- Перенумерованы секции 3.2-3.4
- Обновлена сводка по приоритетам (добавлен пункт 1)
- Причина: Отложена YAML конфигурация из-за DomainConfig 128-byte constraint

**2026-03-20 (v0.6.2 завершена):**
- ✅ Удалено: Секция 1.1 (Compiler warnings - решено в v0.6.2)
- ✅ Удалено: Секция 3.1 (DomainConfig V2.1 примеры - завершено в v0.6.2)
- ✅ Обновлено: Секция 2.3 (Struct optimization - завершена в v0.6.2)
- Добавлено: Секция 3.1 (Configuration System - Preset Loading)
- Обновлены приоритеты: критические проблемы отсутствуют

**2026-03-20 (v0.6.1 завершена):**
- ✅ Удалено: Секция 0.1 (v0.4.0 Causal Time System - завершено в v0.6.0)
- ✅ Удалено: Секция 1.1 (Падающие тесты - решено в v0.6.1)
- ✅ Удалено: Секция 3.2 (Factory Methods - реализовано в v0.6.1)
- Обновлены приоритеты: убраны завершённые задачи
- Обновлена секция 1.1: предупреждения компиляции (~15 warnings)

**2026-03-20 (v0.6.0 завершена):**
- ✅ Решены: Domain Configuration, Configuration System Integration
- Обновлен статус тестов: 168 pass, 5 fail

**2026-03-19 (DomainConfig V2.1):**
- Добавлен раздел 3.1: DomainConfig V2.1 - Примеры конфигураций для всех доменов
- Обновлен раздел 3.2: Factory Methods с маркировкой V2.1 Arbiter настроек
- Перенумерованы секции 3.3 и 3.4
- Обновлена сводка по приоритетам (добавлен пункт 4)

**2026-03-19 (Phase 3):**
- Обновлен статус падающих тестов (6 fail вместо 5)
- Отмечен factory_experience как реализованный (v0.5.0)
- Архивирован v0.5.0 как завершенный
- Очищен ROADMAP от выполненных задач

**2026-03-19 (Phase 1-2):**
- Перемещен весь контент из ROADMAP.md v2.3
- Добавлены текущие проблемы (падающие тесты, предупреждения)
- Реструктуризация по приоритетам
- Архивированы детальные планы

**2026-03-08:**
- Добавлены UCL V2.0 задачи
- Обновлена критическая секция

**2026-02-11:**
- Первая версия файла

---

**Версия:** 4.0
**Последнее обновление:** 2026-03-27
**Создано в рамках:** Axiom Project
**Статус:** Активный учет технического долга и отложенных планов
