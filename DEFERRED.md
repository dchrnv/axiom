# Axiom - Отложенные задачи

**Версия:** 3.6
**Создан:** 2026-02-11
**Обновлен:** 2026-03-21

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

### axiom-arbiter (Arbiter V1.0) — Stub-модули требуют замены

**Файлы:**
- `crates/axiom-arbiter/src/experience.rs` (84 строки)
- `crates/axiom-arbiter/src/ashti_processor.rs` (24 строки)
- `crates/axiom-arbiter/src/maya_processor.rs` (40 строк)
- `crates/axiom-arbiter/src/com.rs` (32 строки)

**Статус:** Модуль мигрирован со stub-реализациями зависимостей
**Дата:** 2026-03-21 (Фаза 6 миграции)

**Что нужно заменить:**

**1. Experience Module (`experience.rs` stub → полноценная реализация):**
- [ ] `Experience::resonance_search()` — реальный алгоритм резонансного поиска
  - Threshold-based classification (reflex_threshold, association_threshold)
  - Weight-based trace selection
  - Distance metrics для pattern matching
- [ ] `Experience::add_trace()` — полная логика ассоциативной памяти
  - Trace deduplication
  - Weight normalization
  - Memory capacity limits
- [ ] Trace strengthening/weakening based on feedback
  - `strengthen_trace(trace_index, delta)`
  - `weaken_trace(trace_index, delta)`
  - Feedback loop integration
- [ ] `ExperienceTrace` расширение:
  - Добавить `last_used: u64` (для LRU eviction)
  - Добавить `success_count: u32` (для reinforcement)
  - Добавить `pattern_hash: u64` (для быстрого поиска)

**2. ASHTI Processor (`ashti_processor.rs` stub → реальная обработка):**
- [ ] `AshtiProcessor::process_token()` — маршрутизация через ASHTI 1-8
  - Параллельная обработка через 8 специализированных доменов
  - Hint propagation из Experience
  - Domain-specific processing rules
- [ ] Интеграция с DomainConfig:
  - Использование `arbiter_flags` для REFLEX_ENABLED, HINTS_ENABLED
  - Применение `max_concurrent_hints` ограничения
  - Учет `reflex_cooldown` per domain
- [ ] Result aggregation:
  - Сбор результатов от всех 8 доменов
  - Timeout handling для медленных доменов
  - Partial result handling

**3. MAYA Processor (`maya_processor.rs` stub → полная консолидация):**
- [ ] `MayaProcessor::consolidate_results()` — алгоритм консолидации
  - Weighted averaging по результатам от ASHTI 1-8
  - Confidence scoring для каждого результата
  - Conflict resolution при противоречивых результатах
- [ ] Интеграция с DomainConfig:
  - Использование `comparison_strategy` (FirstMatch, BestMatch, Consensus)
  - Применение `response_timeout` для отбрасывания медленных результатов
- [ ] Quality metrics:
  - Consistency score между ASHTI результатами
  - Confidence threshold для принятия решения

**4. COM Module (`com.rs` stub → полный Causal Order Model):**
- [ ] `COM::next_event_id()` — реальное отслеживание причинного порядка
  - Domain-specific event ID allocation
  - Causal ordering guarantees
  - Event timestamp management
- [ ] Интеграция с CausalFrontier:
  - Event tracking для всех 11 доменов
  - Parent-child event relationships
  - Causal consistency validation
- [ ] Event storage:
  - Event history для replay/debugging
  - Pruning старых событий (retention policy)

**Причина stub-реализации:** Модули experience, ashti_processor, maya_processor, com еще не мигрированы. Arbiter мигрирован с минимальными заглушками для компиляции и тестирования основной логики маршрутизации.

**Приоритет:** Средний (требуется после миграции соответствующих модулей)

**Зависимости:**
- Experience module миграция (пока не начата)
- ASHTI domains implementation (требует axiom-domain)
- MAYA consolidation logic (требует axiom-domain)
- COM полная реализация (может быть отдельным crate)

**Критерий готовности:** Все stub-модули заменены, 9 существующих тестов остаются зелеными, добавлены интеграционные тесты для полной функциональности.

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

### 3.3 Events System Integration

**Где:** `runtime/src/`
**Что отложено:** Система событий для COM интеграции
**Почему:** UCL команды должны генерировать события
**Когда планируется:** По требованию

**Требуется:**
- [ ] Event структуры для DomainCreated, TokenInjected, ForceApplied
- [ ] Event bus для подписки и обработки
- [ ] Интеграция с PhysicsProcessor

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
1. **axiom-arbiter stub-модули** - Замена заглушек на полноценные реализации (Фаза 6)
2. SPACE V6.0 - YAML Configuration (Phase 1.10-1.11)
3. Configuration System - Preset Loading
4. Events System Integration
5. Configuration Advanced Features
6. Shell V3.0 - YAML Configuration (Phase 2.3)
7. Shell V3.0 - Runtime Configuration (Phase 2.9)

### 🟢 НИЗКИЙ:
4. Python Adapter
5. REST API
6. Performance Benchmarks

### 📦 АРХИВНЫЕ:
- v0.3.0, v0.4.0 планы (отложено)

---

## 📝 История изменений

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

**Версия:** 3.6
**Последнее обновление:** 2026-03-21 (Migration Phase 6)
**Создано в рамках:** Axiom Project
**Статус:** Активный учет технического долга и отложенных планов
