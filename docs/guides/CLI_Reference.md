# AXIOM CLI Reference

Полное описание интерфейса командной строки `axiom-cli`.

---

## Запуск

```bash
cargo run --bin axiom-cli --release
```

При старте система автоматически загружает состояние из `./axiom-data` (если каталог существует).

**Опции запуска:**

| Флаг | По умолчанию | Описание |
|------|-------------|----------|
| `--tick-hz N` | `100` | Частота тиков ядра в Гц |
| `--verbose`, `-v` | выкл | Показывать состояние после каждого ввода |
| `--adaptive` | выкл | Адаптивная частота тиков (Sentinel V1.0) |
| `--detail <level>` | `min` | Уровень детализации: off / min / mid / max |
| `--config <path>` | `./axiom-cli.yaml` | Путь к файлу конфигурации |
| `--data-dir <path>` | `./axiom-data` | Директория хранилища |
| `--no-load` | — | Не загружать состояние при старте |

```bash
cargo run --bin axiom-cli --release -- --verbose
cargo run --bin axiom-cli --release -- --detail max
cargo run --bin axiom-cli --release -- --tick-hz 200 --adaptive
cargo run --bin axiom-cli --release -- --no-load --data-dir /tmp/axiom-test
```

---

## Текстовый ввод

Любая строка без `:` в начале — ввод пользователя. Ядро обрабатывает её через когнитивный пайплайн:

```
SUTRA → EXPERIENCE → ASHTI → MAYA
```

### Уровни детализации вывода

Управляются командой `:detail` или флагом `--detail`.

**`:detail off`** — 2 строки:
```
  path:     reflex
  domain:   101 (EXECUTION)
```

**`:detail min`** — краткий (по умолчанию):
```
  path:     slow-path
  domain:   101 (EXECUTION)
  coherence:0.75
  traces:   47 matched
  position: (3113, 6636, 10985)
```

**`:detail mid`** — routing + output:
```
  ── routing ──────────────────────────
  path:       slow-path
  confidence: 0.75
  traces:     47 matched (of 50 total)
  passes:     1 (max: 3)
  ── output ───────────────────────────
  domain:     101 (EXECUTION)
  coherence:  0.75 (threshold: 0.60)
  position:   (3113, 6636, 10985)
  tension:    created=false
```

**`:detail max`** — полный с секцией input:
```
  ── input ────────────────────────────
  text:       "привет"
  hash:       0x00a7f3b2c1e4d5f6
  token:      pos=(1937,6061,10335) mass=100 temp=200 valence=0
  shell:      [0, 0, 0, 0, 200, 100, 0, 0]
  ── routing ──────────────────────────
  path:       ⚡ reflex
  reflex_hit: true
  confidence: 0.72
  traces:     46 matched (of 47 total)
  passes:     1 (max: 3)
  ── output ───────────────────────────
  dominant:   101 (EXECUTION)
  coherence:  1.00 (threshold: 0.60)
  position:   (1823, 5950, 10514)
  shell:      [0, 0, 0, 0, 180, 100, 0, 0]
  Δpos:       (-114, -111, +179)
  event_id:   847305
  tension:    created=false
```

**Описание полей:**

| Поле | Описание |
|------|----------|
| `path` | `slow-path` — первый раз; `reflex` — повторный паттерн; `multi-pass(N)` — N итераций |
| `confidence` | 0.00–1.00. < threshold → tension trace создаётся |
| `traces matched` | Прошли хэш-фильтр при резонансном поиске |
| `passes` | Число проходов через ASHTI→MAYA |
| `Δpos` | Смещение токена в семантическом пространстве |
| `event_id` | COM event ID — монотонный идентификатор события |
| `tension created` | Был ли создан tension trace в ходе этой маршрутизации |

---

## Служебные команды

Все команды начинаются с `:`.

### Состояние системы

```
:status
```
Расширенный статус: tick_count, com_next_id, uptime, actual Hz, память, когнитивные параметры.

```
:memory
```
Полная статистика: тики, токены, связи, traces, skills, tension.

```
:domains
```
Список всех 11 доменов с числом токенов.

```
:tokens <domain_id>
```
Число токенов в конкретном домене.

```
:snapshot
```
Метаданные текущего снапшота.

```
:schedule
```
Текущие интервалы периодических задач ядра (тики).

```
:tickrate
```
Текущая частота и состояние Adaptive Tick Rate (Sentinel V1.0).

### Experience / Tension

```
:traces
```
Experience traces, отсортированные по weight (top-20). Показывает weight, temperature/mass/valence, позицию, возраст, хэш:
```
  ══ Experience Traces ══════════════════
  Total: 50  |  Avg weight: 0.54  |  Max weight: 0.72
    #  Weight  tmp/mss/val  (x,y,z)              Age       Hash
    1  0.7200  180/100/0  (1823, 5950, 10514)     4281         0  0xf3b2c1e4
  ...
```

```
:tension
```
Активные tension traces с temperature и возрастом:
```
  ══ Tension Traces ═════════════════════
  Active: 2
    #  Temp        Hash  Age (ticks)
    1   201  0x00000028           20
    2   145  0x0000001e           50
```

### Cognitive Depth

```
:depth
```
Параметры когнитивного слоя из конфига MAYA:
```
  ══ Cognitive Depth ════════════════════
  max_passes:          3
  min_coherence:       0.60
  internal_dominance:  0.00
  tension_threshold:   128  (drain at 50% heat)
  ── current state ──────────────────────
  traces:              50
  tension_active:      2
```

### Arbiter

```
:arbiter
```
Пороги Arbiter по доменам + статистика Reflector:
```
  ══ Arbiter — Domain Thresholds ════════
     ID        Name  Reflex-T  Assoc-T  Cooldown  MaxPass
    101   EXECUTION       127       64         0        3
    102      SHADOW       180       64         0        0
  ...
  ── reflector ──────────────────────────
  patterns tracked:  12
  reflex success:    42  fail: 3
```

### Производительность

```
:perf
```
Метрики производительности тиков:
```
  ══ Performance ════════════════════════
  uptime:       25.7s
  total ticks:  2570
  actual rate:  99.8 Hz (target: 100 Hz)
  ── tick breakdown ─────────────────────
  avg tick:     42 ns
  peak tick:    1.2 µs  (tick #1082)
  budget used:  0.42%
  ── periodic tasks (calls) ─────────────
  adaptation:   51 calls (every 50 ticks)
  horizon_gc:   5 calls (every 500 ticks)
  dream:        25 calls (every 100 ticks)
  tension_chk:  257 calls (every 10 ticks)
```

### Управление выводом

```
:detail [off|min|mid|max]
```
Переключить уровень детализации. Без аргумента — показать текущий.

```
:verbose [on|off]
```
Показывать `[tick=N traces=M matched=P tension=Q]` после каждого ввода.

```
:tick [N]
```
Прокрутить N тиков вручную (по умолчанию 1).

### Persistence

```
:save [path]
```
Сохранить полное состояние (default: `./axiom-data`).

```
:load [path]
```
Загрузить состояние. TickSchedule из конфига сохраняется.

```
:autosave [on <N> | off]
```
Автосохранение каждые N тиков.
```bash
:autosave on 1000
:autosave off
:autosave          # показать статус
```

```
:export traces [path]
:export skills [path]
```
Экспорт в JSON (default: `axiom-export-traces.json` / `axiom-export-skills.json`).

```
:import traces [path]
:import skills [path]
```
Импорт из JSON. Веса умножаются на 0.7 (GUARDIAN-валидация).

### Завершение

```
:quit / :q
```
Автосохранение (если включено) + выход.

```
:help
```
Краткий список всех команд в терминале.

---

## Домены

| ID | Имя | Роль |
|----|-----|------|
| 100 | SUTRA | Входная точка всех токенов |
| 101 | EXECUTION | Активное выполнение |
| 102 | SHADOW | Скрытые паттерны |
| 103 | CODEX | Правила и ограничения |
| 104 | MAP | Пространственная карта |
| 105 | PROBE | Зондирование |
| 106 | LOGIC | Логический вывод |
| 107 | DREAM | Dream-цикл |
| 108 | ETHICS | Этические ограничения |
| 109 | EXPERIENCE | Experience traces |
| 110 | MAYA | Финальная классификация |

---

## Конфигурационный файл

```yaml
# axiom-cli.yaml
tick_hz: 100
verbose: false
adaptive_tick_rate: false
detail_level: min      # off | min | mid | max
prompt: "axiom> "

tick_schedule:
  tension_check_interval: 10
  goal_check_interval: 10
  adaptation_interval: 50
  horizon_gc_interval: 500
  dream_interval: 100
  reconcile_interval: 200
  persist_check_interval: 1000  # 0 = autosave выкл

  # Параметры Adaptive Tick Rate (только при adaptive_tick_rate: true)
  adaptive_min_hz: 60
  adaptive_max_hz: 1000
  adaptive_step_up: 200
  adaptive_step_down: 20
  adaptive_cooldown: 50
```

---

## Типичные сценарии

**Диагностика когнитивного слоя:**
```bash
cargo run --bin axiom-cli --release -- --detail max
axiom> :depth
axiom> :arbiter
axiom> :traces
# Введи текст несколько раз, смотри как traces накапливаются
```

**Мониторинг производительности:**
```bash
cargo run --bin axiom-cli --release
axiom> :perf          # после нескольких сотен тиков
axiom> :tickrate      # если включён --adaptive
```

**Первый запуск (чистое состояние):**
```bash
cargo run --bin axiom-cli --release -- --no-load --detail mid
# slow-path → reflex после повторных паттернов
```

**Сброс состояния:**
```bash
rm -rf axiom-data/
cargo run --bin axiom-cli --release -- --no-load
```
