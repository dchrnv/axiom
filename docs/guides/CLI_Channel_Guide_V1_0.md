# CLI Channel — Руководство пользователя и оператора

**Версия:** 2.0  
**Дата:** 2026-04-12  
**Реализация:** CLI Channel V1.1 + Config V1.0  
**Запуск:** `cargo run --bin axiom-cli -- [флаги]`

---

## 1. Что это

CLI Channel — терминальный интерфейс к живому ядру AXIOM. Это не чат-бот. Это **диагностическое окно** в когнитивную систему: вы видите как ядро превращает введённый текст в токен, через какой путь он проходит, нашёлся ли рефлекс в EXPERIENCE, сколько следов tension накоплено.

Архитектурная граница жёсткая:

```
stdin → [TextPerceptor] → UclCommand(64B) → [AxiomEngine] → [MessageEffector] → stdout
```

Ядро не знает о терминале. Ядро не импортирует tokio. Всё async — только в `axiom-agent`.

---

## 2. Запуск

```bash
# Стандартный запуск (100 Hz, verbose off)
cargo run --bin axiom-cli

# Медленный режим (1 Hz) — видно каждый тик
cargo run --bin axiom-cli -- --tick-hz 1

# Подробный вывод: tension traces после каждого тика
cargo run --bin axiom-cli -- --verbose

# Комбинация
cargo run --bin axiom-cli -- --tick-hz 10 --verbose

# Справка
cargo run --bin axiom-cli -- --help
```

---

## 3. Параметры запуска (CLI-флаги)

### `--tick-hz N`

| Параметр | Значение по умолчанию | Диапазон | Где применяется |
|----------|----------------------|----------|-----------------|
| `tick_hz` | `100` | `1..u32::MAX` | `CliConfig::tick_hz` |

Частота тиков ядра в герцах. Один тик = один вызов `TickForward` в движке. На каждом тике:
- **Горячий путь (каждый тик):** физика всех 11 доменов — движение токенов, гравитация
- **Тёплый путь (каждые N тиков):** tension_check (каждые 10), goal_check (каждые 10), dream (каждые 100)
- **Холодный путь (редко):** adaptation (каждые 50), horizon_gc (каждые 500), reconcile (каждые 200), snapshot (каждые 5000)

Интервал тика в мс = `1000 / tick_hz`. При `tick_hz=100` → 10 ms/тик. При `tick_hz=1` → 1 сек/тик.

> **Рекомендуемые значения:**  
> `1` — исследование, пошаговый режим  
> `10` — медленная демонстрация  
> `100` — стандартная работа (default)  
> `1000` — стресс-тест, бенчмарк

### `--adaptive`

| Параметр | Значение по умолчанию | Где применяется |
|----------|----------------------|-----------------|
| `adaptive_tick_rate` | `false` | `CliConfig::adaptive_tick_rate` |

Включает Axiom Sentinel V1.0 Phase 3 — адаптивную частоту тиков. Частота автоматически повышается при пользовательском вводе, multipass, активном tension, и снижается во время простоя.

### `--detail <level>`

| Параметр | Значение по умолчанию | Допустимые значения | Где применяется |
|----------|----------------------|---------------------|-----------------|
| `detail_level` | `min` | `off` / `min` / `mid` / `max` | `CliConfig::detail_level` |

Уровень детализации вывода при текстовом вводе (управляет `MessageEffector`):
- `off` — только path + domain (2 строки)
- `min` — краткий вывод: path, domain, coherence, traces, position (default)
- `mid` — routing + output без секции input
- `max` — полный вывод: input / routing / output + Δpos + event_id

### `--hot-reload`

| Параметр | Значение по умолчанию | Где применяется |
|----------|----------------------|-----------------|
| `hot_reload` | `false` | `CliConfig::hot_reload` |

Включает горячую перезагрузку `config/axiom.yaml`. При изменении файла `tick_schedule` применяется к работающему ядру без перезапуска. Остальные параметры (`tick_hz`, `verbose`, `detail_level`) требуют перезапуска.

При старте выводится:
```
[config] hot-reload enabled (watching config/axiom.yaml)
```

При изменении файла:
```
[config] reloaded tick_schedule (tick=12345)
```

Можно также задать в `axiom-cli.yaml`: `hot_reload: true`.

### `--data-dir <path>`

Директория хранилища для `:save` / `:load` (default: `./axiom-data`).

### `--no-load`

Пропустить загрузку из директории данных при старте.

### `--verbose` / `-v`

| Параметр | Значение по умолчанию | Где применяется |
|----------|----------------------|-----------------|
| `verbose` | `false` | `CliConfig::verbose` |

Включает вывод tension traces после каждого тика (если есть активные). Tension trace — незавершённый или низко-когерентный паттерн, который система "держит в уме" для повторной обработки (Cognitive Depth V1.0).

При `--verbose`:
```
  [tension: 3 active]
```

Выводится только если tension_count > 0. Не добавляет шума при пустой памяти.

---

## 3.5 Файл конфигурации (axiom-cli.yaml)

Приоритет источников: **default → файл → CLI-флаги**. Флаги всегда перекрывают файл.

### Расположение

Поиск в следующем порядке:

1. `--config <path>` — явный путь через флаг
2. `./axiom-cli.yaml` — рабочая директория (рядом с бинарником)
3. `~/.config/axiom/cli.yaml` — пользовательский конфиг

Если файл не найден — используются defaults. Ошибка парсинга выводится в stderr и также игнорируется (defaults).

### Структура файла

```yaml
# axiom-cli.yaml

# Основные параметры
tick_hz: 100            # Гц, default: 100
verbose: false          # default: false
# prompt: "axiom> "    # зарезервировано
adaptive_tick_rate: false  # Sentinel V1.0 Phase 3, default: false
detail_level: min       # off|min|mid|max, default: min
hot_reload: false       # горячая перезагрузка config/axiom.yaml, default: false

# Расписание периодических задач ядра
# Значения — интервал в тиках (0 = отключено)
tick_schedule:
  tension_check_interval: 10    # Cognitive Depth tension (default: 10)
  goal_check_interval:    10    # GoalPersistence (default: 10)
  dream_interval:         100   # CODEX dream-паттерны (default: 100)
  adaptation_interval:    50    # адаптация порогов Arbiter (default: 50)
  reconcile_interval:     200   # rebuild spatial grid (default: 200)
  horizon_gc_interval:    500   # GC за causal horizon (default: 500)
  snapshot_interval:      5000  # auto-snapshot+prune (default: 5000)
```

Все поля опциональны — отсутствующее поле = значение по умолчанию, не ошибка.

> **Горячая перезагрузка:** `hot_reload: true` включает слежение за `config/axiom.yaml`.
> При его изменении `tick_schedule` подхватывается на лету. Изменения `tick_hz`, `verbose`,
> `detail_level` требуют перезапуска — они читаются только при старте.

### Пример: только-горячий-путь (максимальная скорость)

```yaml
tick_hz: 1000
tick_schedule:
  tension_check_interval: 0
  goal_check_interval:    0
  dream_interval:         0
  adaptation_interval:    0
  reconcile_interval:     0
  horizon_gc_interval:    0
  snapshot_interval:      0
```

### Пример: максимальная нагрузка (все задачи каждый тик)

```yaml
tick_hz: 10
verbose: true
tick_schedule:
  tension_check_interval: 1
  goal_check_interval:    1
  dream_interval:         1
  adaptation_interval:    1
  reconcile_interval:     1
  horizon_gc_interval:    1
  snapshot_interval:      0  # snapshot дорого каждый тик
```

### Пример: медленное исследование

```yaml
tick_hz: 1
verbose: true
tick_schedule:
  tension_check_interval: 1
  goal_check_interval:    1
  dream_interval:         5
  adaptation_interval:    10
  reconcile_interval:     20
  horizon_gc_interval:    50
  snapshot_interval:      0
```

### Флаг `--config`

```bash
cargo run --bin axiom-cli -- --config /path/to/my-config.yaml
cargo run --bin axiom-cli -- --config configs/stress.yaml --tick-hz 1000
```

CLI-флаги применяются поверх загруженного файла. В последнем примере `tick_hz` из `stress.yaml` будет перекрыт на 1000.

---

## 4. Ввод текста

Любая строка не начинающаяся с `:` — **пользовательский ввод**. Обрабатывается через `TextPerceptor`:

1. `TextPerceptor::perceive(text)` → `UclCommand(InjectToken, SUTRA=100)`
2. `AxiomEngine::process_and_observe(cmd)` → `ProcessingResult`
3. `MessageEffector::format_result(result)` → текст на stdout

### Вывод результата

```
  path:     slow-path
  domain:   110 (MAYA)
  coherence:0.78
  position: (12345, -8901, 23456)
```

| Поле | Описание |
|------|----------|
| `path` | Путь обработки: `reflex`, `slow-path`, `multi-pass(N)` |
| `domain` | ID и имя доминирующего домена (откуда вышел консолидированный токен) |
| `coherence` | Оценка согласованности ASHTI 1-8 (0.0..1.0). Если < порога → multi-pass |
| `reflex: hit` | Показывается только при fast path из EXPERIENCE |
| `traces: N matched` | Сколько следов прошли хэш-фильтр при поиске в EXPERIENCE |
| `tension: N active` | Появляется если в EXPERIENCE есть незавершённые паттерны |
| `position` | Позиция выходного токена в семантическом пространстве (i16 × 3) |

### Как TextPerceptor кодирует текст

Детерминированный MVP без ML:

| Поле токена | Откуда берётся |
|-------------|----------------|
| `position` (x, y, z) | FNV-1a hash текста → 3 × i16 (0..32767) |
| `mass` | `50 + len.min(200)` байт — длина текста |
| `temperature` | Базовый 150 + 15 × (кол-во `!`) + 10 × (кол-во `?`) |
| `semantic_weight` | Константа 0.8 (высокое когнитивное значение) |
| `target_domain` | Всегда SUTRA = 100 |

Одинаковый текст → одинаковый токен (детерминизм). Разные тексты → разные позиции.

---

## 5. Служебные команды (`:команда`)

### `:quit` / `:q`
Завершить работу. Graceful shutdown — ядро дропается, память освобождается.

---

### `:help`
Показать список всех команд.

---

### `:status`
Текущее состояние ядра:
```
  tick_count: 1450
  tension:    2
```
| Поле | Откуда | Описание |
|------|--------|----------|
| `tick_count` | `engine.tick_count` | Число прошедших тиков с запуска |
| `tension` | `experience().tension_count()` | Незавершённые паттерны в EXPERIENCE |

---

### `:domains`
Список всех 11 доменов с числом токенов:
```
  100 (SUTRA) — 3 tokens
  101 (EXECUTION) — 0 tokens
  ...
  110 (MAYA) — 1 tokens
```

Маппинг `domain_id → имя` по формуле `id % 100`:

| offset | Имя | ID (level 1) |
|--------|-----|--------------|
| 0 | SUTRA | 100 |
| 1 | EXECUTION | 101 |
| 2 | SHADOW | 102 |
| 3 | CODEX | 103 |
| 4 | MAP | 104 |
| 5 | PROBE | 105 |
| 6 | LOGIC | 106 |
| 7 | DREAM | 107 |
| 8 | ETHICS | 108 |
| 9 | EXPERIENCE | 109 |
| 10 | MAYA | 110 |

---

### `:tokens <domain_id>`
Число токенов в конкретном домене:
```
:tokens 106
  domain 106: 5 tokens
```

---

### `:traces` / `:tension`
Число активных tension traces в EXPERIENCE:
```
  tension traces: 3
```
Tension trace создаётся когда когерентность ASHTI-результатов < порога (Cognitive Depth). Система держит паттерн в буфере для повторной обработки на следующем цикле.

---

### `:verbose [on/off]`
Переключить подробный вывод без перезапуска:
```
:verbose on
  verbose: on

:verbose off
  verbose: off

:verbose          ← без аргумента — показать текущее значение
  verbose: on
```

---

### `:tick [N]`
Прокрутить N тиков синхронно (без ожидания интервала):
```
:tick 1000
  ticked 1000 times. tick_count=2450
```
Полезно для разогрева системы или форсированной прокрутки периодических задач.

При N не указан — прокручивает 1 тик.

---

### `:snapshot`
Показать метаданные текущего снапшота:
```
  snapshot: tick_count=2450 domains=11
```
Снапшот создаётся в памяти — данные не сохраняются на диск (persistent storage вне MVP).

---

### `:schedule`
Текущий TickSchedule — расписание периодических задач:
```
  adaptation:    50
  horizon_gc:    500
  snapshot:      5000
  dream:         100
  tension_check: 10
  goal_check:    10
  reconcile:     200
```

---

## 6. TickSchedule — изменяемые параметры ядра

`TickSchedule` задаётся через `tick_schedule:` в `axiom-cli.yaml` или программно через `engine.tick_schedule`. Значения — интервал в тиках (0 = отключено).

| Поле | Default | Что делает при срабатывании |
|------|---------|----------------------------|
| `adaptation_interval` | `50` | `run_adaptation()` — адаптация порогов Arbiter по статистике рефлексов |
| `horizon_gc_interval` | `500` | `run_horizon_gc()` — удаление трейсов за causal horizon, освобождение памяти |
| `snapshot_interval` | `5000` | `snapshot_and_prune()` — авто-снапшот + prune Experience |
| `dream_interval` | `100` | `dream_propose()` — CODEX предлагает DREAM-паттерны для ассоциации |
| `tension_check_interval` | `10` | `arbiter_heartbeat_pulse()` — обработка TensionTraces из Experience |
| `goal_check_interval` | `10` | `generate_goal_impulses()` — GoalPersistence: инжект goal-импульсов если цель не достигнута |
| `reconcile_interval` | `200` | `reconcile_all()` — rebuild spatial grid, prune orphaned connections, fix domain_id |

### Производительность (из RESULTS.md v6)

| Задача | Время/вызов | Рекомендуемый интервал |
|--------|-------------|------------------------|
| `tension_check` | ~25 µs | каждые 10 тиков |
| `goal_check` | ~25 µs | каждые 10 тиков |
| `dream` | ~20 µs | каждые 100 тиков |
| `adaptation` | ~25–30 µs | каждые 50 тиков |
| `reconcile_all` (t50 c100) | ~48 µs | каждые 200 тиков |
| `horizon_gc` | ~30 µs | каждые 500 тиков |
| `snapshot_and_prune` | ~40 µs | каждые 5000 тиков |

При `tick_hz=100`: 50-интервальная задача срабатывает раз в 0.5 секунды, 500-интервальная — раз в 5 секунд.

### Как изменить ScheduleTimeSchedule в коде

```rust
// В bin/axiom-cli.rs или через отдельный конфиг
let mut engine = AxiomEngine::new();

// Отключить все периодические задачи (только hot path)
engine.tick_schedule.adaptation_interval    = 0;
engine.tick_schedule.horizon_gc_interval    = 0;
engine.tick_schedule.snapshot_interval      = 0;
engine.tick_schedule.dream_interval         = 0;
engine.tick_schedule.tension_check_interval = 0;
engine.tick_schedule.goal_check_interval    = 0;
engine.tick_schedule.reconcile_interval     = 0;

// Или: максимальная нагрузка (все задачи каждый тик)
engine.tick_schedule.tension_check_interval = 1;
engine.tick_schedule.goal_check_interval    = 1;
engine.tick_schedule.dream_interval         = 1;
engine.tick_schedule.adaptation_interval    = 1;
engine.tick_schedule.reconcile_interval     = 1;
// snapshot оставить на большом интервале или 0 — дорого
```

---

## 7. Параметры TextPerceptor (кодирование текста)

Сейчас не изменяются через CLI. Жёстко заданы в [perceptors/text.rs](../crates/axiom-agent/src/perceptors/text.rs):

| Константа | Значение | Описание |
|-----------|---------|----------|
| `SUTRA_DOMAIN_ID` | `100` | Всегда инжектируем в SUTRA первого уровня |
| `semantic_weight` | `0.8` | Вес токена — "насколько важен" для ассоциации |
| `temperature` base | `150.0` | Минимальная пластичность нового ввода |
| temperature + `!` | `+15.0` | Бонус за каждый восклицательный знак |
| temperature + `?` | `+10.0` | Бонус за каждый вопросительный знак |
| `mass` formula | `50 + len.min(200)` | Масса зависит от длины текста (50..250) |

---

## 8. Параметры ProcessingResult — что возвращает ядро

Определены в [axiom-runtime/src/result.rs](../crates/axiom-runtime/src/result.rs).

| Поле | Тип | Источник | Описание |
|------|-----|----------|----------|
| `ucl_result` | `UclResult` | `process_command()` | Статус UCL команды (0=Success) |
| `path` | `ProcessingPath` | `RoutingResult.passes` + `reflex` | Рефлекс / slow-path / multi-pass(N) |
| `dominant_domain_id` | `u16` | `RoutingResult.consolidated.domain_id` | Домен, выдавший консолидированный токен |
| `coherence_score` | `Option<f32>` | `RoutingResult.confidence` | Согласованность ASHTI 1-8 (0.0..1.0) |
| `tension_count` | `u32` | `experience().tension_count()` | Число незавершённых паттернов |
| `output_shell` | `[u8; 8]` | Поля Token (приближение) | L4=valence, L5=temperature, L6=mass |
| `output_position` | `[i16; 3]` | `consolidated.position` | Позиция в семантическом пространстве |
| `reflex_hit` | `bool` | `RoutingResult.reflex.is_some()` | Был ли использован fast path |
| `traces_matched` | `u32` | `experience().last_traces_matched` | Следов прошли хэш-фильтр |

### ProcessingPath — когда что появляется

| Путь | Условие | Что значит |
|------|---------|-----------|
| `Reflex` | `passes == 1`, `reflex.is_some()` | Токен найден в EXPERIENCE с высоким резонансом |
| `SlowPath` | `passes == 1`, `reflex.is_none()` | Обработка через ASHTI 1-8 → MAYA |
| `MultiPass(N)` | `passes > 1` | Coherence < порога → повторная обработка N раз |

---

## 9. Что добавить в следующих версиях

Сейчас не реализовано:

| Функция | Приоритет | Описание |
|---------|-----------|----------|
| Пример `reflex` в действии | Высокий | Повторный ввод одного текста — второй раз должен попасть в рефлекс (если weight > threshold) |
| `:inject <domain_id> <text>` | Средний | Инжект в любой домен, не только SUTRA |
| ANSI цвет | Низкий | `reflex` — зелёный, `multi-pass` — жёлтый, `tension` — красный |
| `:set tick-hz N` | Низкий | Изменение частоты тика без перезапуска (сейчас только через `hot_reload: true` и `tick_hz` в файле не подхватывается — только `tick_schedule`) |

---

## 10. Инварианты (нельзя нарушать)

1. **Ядро не знает о tokio** — `AxiomEngine` синхронный, вызывается только из tick loop
2. **Один поток для Engine** — все обращения к `engine` через `tick_loop`, не из stdin reader
3. **Детерминизм** — одинаковый ввод при одинаковом состоянии → одинаковый результат
4. **Ядро тикает всегда** — даже без ввода `TickForward` вызывается по интервалу
5. **Graceful shutdown** — `:quit` → чистый выход, нет `unwrap` на EOF
