# Axiom Roadmap

**Версия:** 7.5
**Дата:** 2026-03-21
**Статус:** v0.7.0 завершена (100%), планирование v0.8.0

---

## 🎯 План реализации SPACE V6.0 + Shell V3.0

### PHASE 1: SPACE V6.0 - Пространственная модель (v0.7.0) ✅ ЗАВЕРШЕНО

**✅ 1.1 Базовые структуры данных** (ЗАВЕРШЕНО)
- ✅ `runtime/src/space.rs` - новый модуль (1447 строк)
- ✅ `SpatialHashGrid` структура (bucket_heads, entries, entry_count)
- ✅ `CellEntry` структура (token_index, next)
- ✅ Константы: `CELL_SHIFT`, `CELL_SIZE`, `BUCKET_COUNT`, `BUCKET_MASK`
- ✅ `cell_key()` - хэш-функция координат → bucket
- ✅ Тесты: хэш-функция, распределение по корзинам

**✅ 1.2 Spatial Hash Grid операции** (ЗАВЕРШЕНО)
- ✅ `rebuild()` - полная перестройка индекса O(N)
- ✅ `find_neighbors()` - поиск токенов в радиусе
- ✅ Тесты: перестройка, поиск соседей, граничные случаи

**✅ 1.3 Вычисление расстояний** (ЗАВЕРШЕНО)
- ✅ `distance2()` - квадрат расстояния (i64, без корня)
- ✅ Целочисленная арифметика с переполнением
- ✅ Тесты: максимальные расстояния, точность

**✅ 1.4 Гравитация** (ЗАВЕРШЕНО)
- ✅ `compute_gravity()` - сила притяжения к Anchor(0,0,0)
- ✅ Линейная модель + Inverse_square модель
- ✅ Конфигурируемый `gravity_scale_shift`
- ✅ `integer_sqrt()` - алгоритм Ньютона
- ✅ Тесты: разные массы, расстояния, модели (12 тестов)

**✅ 1.5 Инерция и движение** (ЗАВЕРШЕНО)
- ✅ `apply_velocity()` - обновление position
- ✅ `apply_friction()` - замедление из DomainConfig
- ✅ `apply_acceleration()` - изменение velocity
- ✅ `clamp_i16()` - предотвращение переполнения
- ✅ `move_towards()` - движение к target (аттрактор)
- ✅ Тесты: движение, трение, переполнение (24 теста)

**✅ 1.6 Пространственные события** (ЗАВЕРШЕНО)
- ✅ Расширить `EventType`: TokenMoved, TokenCollision, TokenEnteredCell
- ✅ `has_moved()` - детекция движения
- ✅ `cell_changed()` - детекция смены ячейки
- ✅ `detect_collisions()` - обнаружение столкновений через spatial hash
- ✅ Тесты: генерация событий, collision detection (17 тестов)

**✅ 1.7 Интеграция с Domain** (ЗАВЕРШЕНО)
- ✅ Добавить `SpatialHashGrid` в `Domain`
- ✅ Перестройка индекса после batch событий
- ✅ Конфигурация `rebuild_frequency` в DomainConfig
- ✅ Методы: `rebuild_spatial_grid()`, `should_rebuild_spatial_grid()`, `find_neighbors()`
- ✅ Тесты: domain + spatial hash (10 новых тестов)

**✅ 1.8 Интеграция с Causal Frontier** (ЗАВЕРШЕНО)
- ✅ `Domain::process_frontier()` расширено collision detection через spatial hash
- ✅ `EventGenerator::generate_collision()` - генерация TokenCollision событий
- ✅ Collision detection с радиусом 100 квантов
- ✅ Добавление столкнувшихся токенов обратно в frontier
- ✅ Тесты: frontier + space integration (5 новых тестов)

**✅ 1.9 Интеграция с Heartbeat** (ЗАВЕРШЕНО)
- ✅ Добавлен `enable_spatial_collision` флаг в HeartbeatConfig
- ✅ Heartbeat → Frontier → Spatial collision detection полный цикл
- ✅ Конфигурируемое включение/отключение spatial checks
- ✅ Gravity + Spatial collision одновременно
- ✅ Тесты: heartbeat + space integration (4 новых теста)

**✅ 1.10 Конфигурация** (ЗАВЕРШЕНО - минимум)
- ✅ Hardcoded константы: `collision_radius: 100`, `gravity_scale_shift: 8`, `rebuild_frequency: 50`
- ⏸️ YAML конфигурация отложена в DEFERRED.md v3.4 (причина: DomainConfig 128-byte constraint)
- ✅ Детерминистичная работа с фиксированными значениями

**✅ 1.11 Финальная валидация** (ЗАВЕРШЕНО)
- ✅ Cross-spec validation: `DomainConfig` размер 128 байт
- ✅ Детерминизм: `distance2()` детерминистичен
- ✅ Zero-alloc: `SpatialHashGrid` переиспользует память
- ✅ Все 285 тестов проходят (100% успех)
- ✅ Документация в коде (комментарии в runtime/src/space.rs)

**Итого Phase 1:**
- ✅ 11 из 11 задач завершено (100%)
- ✅ 285 тестов проходят (+105 новых)
- ✅ Файлы: runtime/src/space.rs (1447 строк), domain.rs (+22 теста), heartbeat.rs, event_generator.rs
- ⏸️ YAML конфигурация отложена (см. DEFERRED.md v3.4)

---

### PHASE 2: Shell V3.0 - Семантический профиль (v0.8.0)

**2.1 Базовые структуры**
- `runtime/src/shell.rs` - новый модуль
- `ShellProfile` type = `[u8; 8]`
- `DomainShellCache` структура (profiles, dirty_flags, generation)
- `ShellContribution` type = `[u8; 8]`
- Тесты: размеры, выравнивание

**2.2 Справочник семантических вкладов**
- `SemanticContributionTable` структура
- `categories: [ShellProfile; 256]`
- `overrides: HashMap<u16, ShellProfile>`
- `get(link_type)` - двухуровневый lookup
- Тесты: категории, переопределения

**2.3 YAML конфигурация**
- Schema: `config/schema/semantic_contributions.yaml`
- Пример с 7 категориями (Structural, Semantic, Causal, Experiential, Social, Temporal, Motor)
- Пример с переопределениями (Emotional_Cause, Physical_Cause, Aesthetic_Feel)
- Загрузка через ConfigLoader
- Валидация схемы
- Тесты: загрузка, валидация

**2.4 Алгоритм вычисления Shell**
- `compute_shell()` - полный пересчёт для токена:
  - Сбор Connection (source_id или target_id)
  - Accumulator `[f32; 8]`
  - Вклад contribution × strength
  - Нормализация (max → 255)
  - Округление до `[u8; 8]`
- Тесты: разные наборы связей, нормализация

**2.5 Инкрементальное обновление**
- Dirty flags (BitVec)
- Триггеры: Connection создана/удалена/изменена
- Mark dirty → recompute → clear dirty
- Тесты: инкрементальное vs полное

**2.6 Интеграция с Causal Frontier**
- `evaluate_local_rules()` расширение:
  - shell update при Connection событиях
- `collect_neighbors()` добавляет source+target в frontier
- Тесты: connection event → shell update

**2.7 Reconciliation через Heartbeat**
- `HeartbeatConfig.enable_shell_reconciliation: bool`
- Shell reconciliation в heartbeat батче:
  - Пересчёт Shell
  - Сравнение с кэшем
  - Запись, если отличается
- Гарантия полного покрытия (все токены проверяются)
- Тесты: reconciliation, drift detection

**2.8 Интеграция с Domain**
- Добавить `DomainShellCache` в `DomainState`
- Инициализация нулевыми профилями
- Первичное вычисление при загрузке
- Тесты: domain + shell cache

**2.9 Runtime конфигурация**
- Runtime configuration YAML:
```yaml
shell_cache:
  enable_shell_reconciliation: true
  reconciliation_log: false
```
- Тесты: разные конфигурации

**2.10 Финальная валидация**
- Все инварианты Shell V3.0
- Shell не генерирует COM-события
- Домен-локальность
- Детерминизм
- Документация в коде
### PHASE 3: Интеграция SPACE ↔ Shell (v0.8.1)

**3.1 Полный цикл взаимодействия**
- SPACE: столкновение → `TokenCollision` событие
- Обработчик столкновения создаёт Connection (резонанс)
- Connection триггерит Shell dirty flag
- Shell пересчитывается для обоих токенов
- Тесты: end-to-end поток

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
- SPACE V6.0: ~8-10 шагов (1.1-1.11) - **✅ 9/11 завершено (82%)**
- Shell V3.0: ~8-10 шагов (2.1-2.10)
- Integration: ~3 шага (3.1-3.3)
- Docs: ~4 шага (4.1-4.4)
- **Всего: ~25-30 шагов** (**9 завершено, ~16-21 осталось**) 

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
