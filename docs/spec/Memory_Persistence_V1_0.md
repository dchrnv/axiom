# AXIOM MODULE SPECIFICATION: Memory Persistence V1.0

**Статус:** Спецификация (design + implementation plan)  
**Версия:** 1.0.0  
**Дата:** 2026-04-06  
**Назначение:** Живая память — персистентность через самоорганизацию, а не через дамп  
**Crate:** `axiom-runtime` (расширение EngineSnapshot) + `axiom-agent` (I/O)  
**Связанные спеки:** Ashti_Core V2.1, EXPERIENCE (Домен 9), SKILLSET, CODEX (Домен 3), MAP (Домен 4), Cognitive Depth V1.0, Shell V3.0, FractalChain

---

## 1. Философия: память как физический процесс

AXIOM не "сохраняет файлы". Токены и связи **оседают** на диск по мере кристаллизации — точно как следы в EXPERIENCE оседают от горячих к холодным.

Три принципа:

1. **Консолидация, не дамп.** Сохраняется только то, что "отстоялось" — высокий weight, низкая temperature. Горячие, нестабильные паттерны живут только в RAM.
2. **Стратификация.** Разные типы памяти хранятся раздельно: навыки, факты, правила, контекст. Каждый тип — свой файл в директории.
3. **Импорт = осторожный.** Загруженные данные начинают с пониженным weight. Система проверяет их собственным опытом перед усилением.

---

## 2. Иерархия памяти

Биологическая аналогия → AXIOM mapping → что персистить:

| Тип памяти | Аналогия | Где в AXIOM | Персистить? | Критерий |
|---|---|---|---|---|
| Рабочая | Сознание сейчас | Токены в ASHTI(1-8) | **Нет** | Эфемерна, восстановится |
| Эпизодическая | "Помню тот случай" | Трейсы в EXPERIENCE(9) | **Выборочно** | weight > trace_persist_threshold |
| Процедурная | "Умею делать X" | SKILLSET в EXPERIENCE(9) | **Да, полностью** | Кристаллизованы, ценны |
| Семантическая | "Знаю что X = Y" | Тяжёлые токены в MAP(4) | **Выборочно** | weight > map_persist_threshold |
| Конституционная | "Правила поведения" | CODEX(3) токены+связи | **Да, полностью** | Пластичный закон |
| Контекст | "О чём думал" | Tension traces, goals | **Да** | Для продолжения мысли |
| Метаданные | "Сколько прожил" | tick_count, com_next_id | **Да** | Причинный порядок |
| REFLECTOR | "Статистика опыта" | ReflexStats, DomainProfile | **Да** | Основа адаптации |

**Что НЕ персистится:**
- Frontier (восстанавливается из event log)
- Shell-кэш (пересчитывается из Connection)
- SpatialHashGrid (пересстраивается из position)
- Arbiter routing cache (пересчитывается из DomainConfig)

---

## 3. Структура на диске

```
axiom-data/
├── manifest.yaml              # Версия формата, checksum, метаданные
├── meta/
│   └── engine_state.bin       # tick_count, com_next_id, snapshot_event_id
├── skills/
│   ├── skill_0001.bin         # Каждый скилл — отдельный файл
│   ├── skill_0002.bin
│   └── index.yaml             # Индекс скиллов: id → описание, weight, created_at
├── traces/
│   └── experience.bin         # Трейсы с weight > threshold (бинарный, компактный)
├── codex/
│   └── rules.bin              # Токены и связи домена 3
├── map/
│   └── facts.bin              # Тяжёлые токены MAP(4) с weight > threshold
├── context/
│   ├── tension.bin            # Активные tension traces
│   └── goals.bin              # Активные цели
├── reflector/
│   └── stats.bin              # ReflexStats + DomainProfile
└── config/
    └── domain_configs.yaml    # Адаптированные DomainConfig'и (пороги, температуры)
```

### 3.1 manifest.yaml

```yaml
version: "1.0"
format: "axiom-memory-v1"
created_at: "2026-04-06T15:30:00Z"
last_saved: "2026-04-06T16:45:00Z"
tick_count: 1542000
com_next_id: 847291
checksum: "sha256:abc123..."
axiom_version: "0.1.0"

# Статистика содержимого
contents:
  skills: 12
  traces: 847           # Из 5200 total — только weight > threshold
  codex_rules: 34
  map_facts: 156
  tension_traces: 3
  active_goals: 1
```

### 3.2 Почему отдельные файлы, а не один blob

- **Гранулярность.** Можно руками посмотреть `skills/` и увидеть что система "умеет".
- **Частичная загрузка.** При старте можно загрузить только skills + codex (быстро), а traces подтянуть позже.
- **Обмен.** Скопировать `skills/` другому экземпляру = передать навыки. Без остального состояния.
- **Отладка.** `manifest.yaml` читается человеком. `index.yaml` показывает что внутри.
- **Версионирование.** Можно обновить формат traces.bin не трогая skills.

---

## 4. Формат сериализации

### 4.1 Бинарные файлы (.bin)

Token и Connection уже `repr(C, align(64))` — фиксированный layout. Можно писать напрямую как raw bytes:

```rust
// Запись
let bytes: &[u8] = unsafe {
    std::slice::from_raw_parts(
        tokens.as_ptr() as *const u8,
        tokens.len() * std::mem::size_of::<Token>(),
    )
};
file.write_all(bytes)?;

// Чтение
let mut tokens = vec![Token::default(); count];
let bytes: &mut [u8] = unsafe {
    std::slice::from_raw_parts_mut(
        tokens.as_mut_ptr() as *mut u8,
        tokens.len() * std::mem::size_of::<Token>(),
    )
};
file.read_exact(bytes)?;
```

**Zero-copy на загрузке.** Token = 64B align(64), Connection = 64B align(64) — можно mmap файл и работать напрямую (будущая оптимизация).

**Опционально:** Для портабельности использовать `bincode` вместо raw bytes. Bincode добавляет ~2ns на токен (negligible). Защищает от endianness проблем.

Рекомендация: **начать с bincode**, перейти на raw bytes если нужна производительность загрузки.

### 4.2 YAML файлы (.yaml)

manifest.yaml, index.yaml, domain_configs.yaml — человекочитаемые. Используют существующий serde_yaml.

---

## 5. Два режима сохранения

### 5.1 Автоматический (кристаллизация)

Привязан к физике системы. Работает через Heartbeat / TickSchedule:

```
Heartbeat пульс (каждые N тиков):
  → Проверить SKILLSET: есть новые кристаллизованные скиллы?
    → Да: записать skill_{id}.bin, обновить index.yaml
  → Проверить EXPERIENCE: трейсы с weight > auto_save_threshold?
    → Да: добавить в буфер на запись
  → Проверить CODEX: есть новые/изменённые правила?
    → Да: пометить codex dirty
  → Если dirty_count > flush_threshold OR elapsed > max_interval:
    → Flush буферов на диск (async, не блокировать ядро)
```

**Конфигурация автосохранения:**

```yaml
persistence:
  enabled: true
  data_dir: "axiom-data"

  # Пороги для автоматического сохранения
  auto_save:
    trace_weight_threshold: 128    # weight 0-255, сохранять если > 128 (50%)
    map_weight_threshold: 180      # факты MAP — только уверенные
    check_interval: 1000           # каждые 1000 тиков проверять
    flush_threshold: 50            # накопилось 50 dirty записей → flush
    max_interval: 60000            # максимум 60к тиков между flush (~60с при 1000Hz)

  # Скиллы записываются сразу при кристаллизации
  skills:
    write_on_crystallize: true     # Скилл → файл немедленно
```

### 5.2 Ручной (команды CLI)

Через CLI Channel:

| Команда | Действие |
|---|---|
| `:save` | Полное сохранение: все dirty буферы + все подпороговые |
| `:save skills` | Только скиллы |
| `:save traces` | Только трейсы > threshold |
| `:save all` | Всё включая подпороговые (полный дамп) |
| `:load [path]` | Загрузить состояние из директории |
| `:memory` | Показать что в памяти: сколько скиллов, трейсов, dirty записей |

При `:quit` — автоматический `:save` если persistence.enabled.

### 5.3 Приоритет записи

При flush на диск — не всё пишется одновременно. Порядок:

1. **meta/engine_state.bin** — tick_count, com_next_id (критично для причинного порядка)
2. **skills/** — новые кристаллизованные скиллы (ценнее всего)
3. **codex/rules.bin** — изменённые правила
4. **context/** — tension + goals (для продолжения мысли)
5. **reflector/stats.bin** — статистика
6. **traces/experience.bin** — трейсы (самый большой файл, пишется последним)
7. **map/facts.bin** — факты MAP
8. **config/domain_configs.yaml** — адаптированные пороги
9. **manifest.yaml** — обновляется последним (маркер успешного сохранения)

Если запись прервана — manifest.yaml не обновлён → при загрузке система знает что данные могут быть неконсистентны → откат к предыдущему manifest.

---

## 6. Загрузка при старте

### 6.1 Boot sequence с персистентностью

```
1. Загрузить GENOME (genome.yaml — неизменяемый)
2. Создать AxiomEngine (пустой)
3. Проверить axiom-data/manifest.yaml:
   a. Есть и валидный → загрузить состояние (§6.2)
   b. Нет или повреждён → чистый старт
4. Пересчитать Shell-кэш из Connection
5. Перестроить SpatialHashGrid из position
6. Запустить CLI Channel / Gateway
```

### 6.2 Порядок загрузки

```
1. meta/engine_state.bin → tick_count, com_next_id
2. codex/rules.bin → домен 3 (правила — до всего остального)
3. skills/ → EXPERIENCE(9) SKILLSET (скиллы с оригинальным weight)
4. traces/experience.bin → EXPERIENCE(9) трейсы (с пониженным weight при import)
5. map/facts.bin → MAP(4) (факты)
6. reflector/stats.bin → REFLECTOR статистика
7. context/tension.bin → tension traces (Cognitive Depth)
8. context/goals.bin → active goals
9. config/domain_configs.yaml → DomainConfig обновления
```

### 6.3 Импорт с осторожностью

При загрузке трейсов из файла — weight **понижается**:

```rust
const IMPORT_WEIGHT_FACTOR: f32 = 0.7; // 70% от сохранённого weight

fn load_traces(path: &Path, domain_state: &mut DomainState) -> Result<u32> {
    let traces = read_traces(path)?;
    let mut imported = 0;

    for mut trace in traces {
        // Понизить weight — система должна подтвердить опыт собственной обработкой
        trace.weight = (trace.weight as f32 * IMPORT_WEIGHT_FACTOR) as u8;

        // Но не ниже min_intensity
        trace.weight = trace.weight.max(MIN_INTENSITY);

        domain_state.inject_trace(trace);
        imported += 1;
    }

    Ok(imported)
}
```

Скиллы загружаются **с оригинальным weight** — они уже кристаллизованы и подтверждены.

CODEX правила загружаются **как есть** — это конституция (пластичная часть).

---

## 7. Фрактальная глубина и разделение по уровням

FractalChain даёт естественное разделение памяти по уровням абстракции:

```
Уровень 1 (быстрый): конкретные случаи, рефлексы
  → traces/level_1/experience.bin
  → skills/level_1/skill_*.bin

Уровень 2 (глубокий): обобщения, мета-паттерны
  → traces/level_2/experience.bin
  → skills/level_2/skill_*.bin

Уровень N: ещё более абстрактные связи
```

**Для MVP (один уровень):** Достаточно плоской структуры (без level_N/). Добавить уровни когда FractalChain используется для реальной многоуровневой обработки.

**Структура с уровнями (будущее):**

```
axiom-data/
├── manifest.yaml
├── meta/
├── level_1/
│   ├── skills/
│   ├── traces/
│   ├── codex/
│   └── map/
├── level_2/
│   ├── skills/
│   └── traces/
├── context/
└── reflector/
```

---

## 8. Обмен знаниями между экземплярами

Директорная структура позволяет:

**Передать навыки:**
```bash
cp -r axiom-data-instance-A/skills/ axiom-data-instance-B/skills/imported/
```

**Передать картину мира:**
```bash
cp axiom-data-instance-A/map/facts.bin axiom-data-instance-B/map/imported_facts.bin
```

При загрузке — система видит `imported/` и применяет IMPORT_WEIGHT_FACTOR. GUARDIAN проверяет что импортированные данные не нарушают GENOME.

---

## 9. Влияние на производительность

| Операция | Ожидаемое время | Когда |
|---|---|---|
| Запись одного скилла | ~100 µs | При кристаллизации |
| Flush experience.bin (1000 трейсов) | ~1-5 ms | Раз в max_interval |
| Запись engine_state.bin | ~10 µs | При flush |
| Загрузка всего состояния | ~10-50 ms | При старте |
| Пересчёт Shell после загрузки | ~3 ms (1000 токенов) | При старте |
| Перестройка SpatialHashGrid | ~15 µs (1000 токенов) | При старте |

**Автосохранение НЕ блокирует ядро.** Запись на диск — async (через tokio в axiom-agent). Ядро помечает dirty буферы, agent пишет в фоне.

Для MVP (без tokio в persistence): запись синхронная, но только при `:save` и `:quit`. Автоматическая кристаллизация пишет скилл-файл сразу (~100 µs, допустимо в холодном пути).

---

## 10. План реализации

### Фаза 1: Формат + запись (`:save`)

1. Определить `MemoryManifest` struct (serde + yaml).
2. Определить формат бинарных файлов (bincode для Token, Connection, Trace, Skill).
3. Реализовать `MemoryWriter` — сериализация каждого типа данных.
4. Реализовать `:save` команду в CLI Channel.
5. Тесты: запись → чтение → сравнение = идентично.

### Фаза 2: Загрузка (`:load` + boot)

1. Реализовать `MemoryLoader` — десериализация каждого типа.
2. Реализовать загрузку при старте (manifest.yaml → load sequence).
3. Import weight factor для трейсов.
4. Пересчёт Shell + SpatialHashGrid после загрузки.
5. Тесты: save → quit → load → состояние восстановлено. Рефлексы работают. Скиллы активируются.

### Фаза 3: Автосохранение (кристаллизация)

1. Добавить `PersistenceConfig` в runtime config.
2. Интегрировать check в Heartbeat / TickSchedule.
3. Dirty buffer tracking: пометка изменённых трейсов/правил.
4. Flush по порогу или интервалу.
5. Тесты: кристаллизация скилла → файл появляется на диске. Рестарт → скилл загружается.

### Фаза 4: Обмен знаниями

1. `:export skills <path>` — экспорт скиллов в отдельную директорию.
2. `:import skills <path>` — импорт с IMPORT_WEIGHT_FACTOR.
3. GUARDIAN проверка импорта.
4. Тесты: экспорт из A → импорт в B → скиллы работают с пониженным weight.

---

## 11. Инварианты

1. **Атомарность manifest.** Manifest обновляется последним. Если файл повреждён — загрузка откатывается к чистому старту.
2. **Причинный порядок.** `com_next_id` сохраняется и восстанавливается. После загрузки новые события имеют строго большие event_id.
3. **GENOME неизменяем.** Персистентность НЕ сохраняет GENOME — он всегда загружается из genome.yaml.
4. **Импорт осторожен.** Трейсы загружаются с пониженным weight. Скиллы — с полным (уже проверены).
5. **Ядро не знает о файлах.** Ядро работает с DomainState. Запись/чтение файлов — ответственность axiom-agent или отдельного persistence модуля.
6. **Shell пересчитывается.** Shell не сохраняется (он кэш). После загрузки — полный reconcile.

---

## 12. Зависимости

**Новые зависимости:**
- `bincode` (version 2, serde-compatible) — для бинарной сериализации
- `sha2` (для checksum в manifest) — опционально для MVP

**Не нужны:**
- SQLite, rocksdb — overkill для текущего масштаба
- mmap — будущая оптимизация, не MVP
- Сжатие (zstd) — будущее, при больших traces

---

## 13. Что НЕ входит в V1.0

- WAL (Write-Ahead Log) — для crash safety. MVP: потеря данных при kill -9 допустима.
- Инкрементальная запись трейсов (append-only log) — будущее.
- Версионирование данных (миграция v1 → v2) — при изменении формата пересохранить.
- mmap zero-copy загрузка — оптимизация, не MVP.
- Сжатие файлов — будущее.
- Шифрование — будущее.

---

## 14. История изменений

- **V1.0**: Первая версия. Иерархия памяти. Директорная структура. Два режима (авто + ручной). Import с осторожностью. Фрактальные уровни (перспектива). Обмен знаниями.
