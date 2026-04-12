# CLI Channel — Расширенные команды и вывод V1.0

**Для:** Claude Sonnet  
**Контекст:** Текущий CLI показывает 4-5 строк на ввод. Нужен полный диагностический вывод и расширенный набор команд.

---

## 1. Расширенный вывод при вводе текста

### Текущий (минимальный):
```
привет
  path:     reflex
  domain:   101 (EXECUTION)
  coherence:1.00
  reflex:   hit
  traces:   46 matched
  position: (1823, 5950, 10514)
```

### Новый (полный):
```
привет
  ── input ──────────────────────────────
  text:       "привет"
  hash:       0xA7F3B2C1E4D5F6A8
  token:      pos=(1937,6061,10335) mass=56 temp=200 valence=0
  shell:      [0,0,0,50,200,180,30,75]
  target:     SUTRA(100)

  ── routing ────────────────────────────
  path:       ⚡ reflex
  reflex_hit: true
  confidence: 0.72
  weight:     0.68 (threshold: 0.498)
  traces:     46 matched (of 47 total)

  ── processing ─────────────────────────
  dominant:   101 (EXECUTION)
  coherence:  1.00 (threshold: 0.60)
  passes:     1 (max: 3)
  tension:    created=false

  ── output ─────────────────────────────
  position:   (1823, 5950, 10514)
  shell:      [0,0,12,40,180,160,30,60]
  Δpos:       (-114, -111, +179)
  event_id:   847305
```

**Управление уровнем детализации:**

`:detail min` — текущий краткий вывод (5 строк)  
`:detail mid` — без секции input, с routing + processing  
`:detail max` — полный вывод как выше  
`:detail off` — только path и domain (2 строки)

---

## 2. Полный список служебных команд

### Информация о системе

| Команда | Описание |
|---|---|
| `:status` | Общее состояние Engine |
| `:domains` | Список доменов с метриками |
| `:domain <id>` | Детальная информация об одном домене |
| `:tokens [domain_id]` | Токены в домене (top-20 по weight) |
| `:connections [domain_id]` | Связи в домене |
| `:memory` | Что в памяти: токены, трейсы, скиллы, на диске |
| `:config` | Текущая конфигурация (tick_hz, schedule, thresholds) |
| `:schedule` | TickSchedule — интервалы периодических задач |
| `:genome` | GENOME инварианты и права доступа |
| `:uptime` | Время работы, тиков/сек реальный vs целевой |

### EXPERIENCE / память

| Команда | Описание |
|---|---|
| `:traces` | Все experience traces (top-20 по weight) |
| `:traces all` | Все трейсы без лимита |
| `:trace <index>` | Детали одного трейса |
| `:tension` | Активные tension traces с temperature |
| `:skills` | Кристаллизованные скиллы |
| `:reflector` | Статистика REFLECTOR: per-domain accuracy |
| `:goals` | Активные цели |

### Cognitive Depth

| Команда | Описание |
|---|---|
| `:depth` | Состояние Cognitive Depth: internal_dominance, pending impulses |
| `:impulses` | Очередь pending_impulses (что система хочет обдумать) |
| `:multipass` | Статистика multi-pass: сколько раз срабатывал, средний coherence |

### Управление

| Команда | Описание |
|---|---|
| `:tick [N]` | Прокрутить N тиков (default: 100) |
| `:inject <domain_id> <text>` | Инжектировать в конкретный домен |
| `:save [path]` | Сохранить состояние |
| `:load [path]` | Загрузить состояние |
| `:export skills [path]` | Экспортировать скиллы |
| `:import skills <path>` | Импортировать скиллы |
| `:reset` | Сбросить к чистому состоянию (с подтверждением) |
| `:snapshot` | Сделать snapshot (без записи на диск) |

### Verbose / отображение

| Команда | Описание |
|---|---|
| `:verbose [on/off]` | Verbose mode (печать при изменениях) |
| `:detail [off/min/mid/max]` | Уровень детализации вывода при вводе |
| `:watch <field>` | Следить за полем (traces/tension/tps) — печатать при изменении |
| `:unwatch <field>` | Перестать следить |

### Диагностика

| Команда | Описание |
|---|---|
| `:perf` | Производительность: ns/тик, тиков/сек, overhead |
| `:events [N]` | Последние N COM-событий |
| `:arbiter` | Состояние Arbiter: last_routing, thresholds per domain |
| `:guardian` | Статистика GUARDIAN: approved/vetoed/inhibited |
| `:frontier` | Состояние Causal Frontier: size, state, budget |
| `:autosave` | Статус AutoSaver: interval, last save, dirty count |
| `:dream` | Состояние DREAM(7): последний анализ, предложения |

### Помощь

| Команда | Описание |
|---|---|
| `:help` | Список всех команд |
| `:help <command>` | Детали конкретной команды |
| `:quit` / `:q` | Выход (с автосохранением) |

---

## 3. Формат вывода команд

### :status (расширенный)
```
  ══ Engine Status ══════════════════════
  tick_count:    154200
  com_next_id:   847305
  uptime:        25.7s
  tick_rate:     100 Hz (actual: 99.8 Hz)
  schedule:      default

  ── memory ─────────────────────────────
  tokens:        0 (across 11 domains)
  connections:   0
  traces:        47 in EXPERIENCE
  skills:        0 crystallized
  tension:       0 active
  goals:         0 active
  impulses:      0 pending

  ── performance ────────────────────────
  avg tick:      96.5 ns
  last tick:     42 ns
  peak tick:     1.2 µs
  budget used:   0.01%
```

### :domains (расширенный)
```
  ══ Domains ════════════════════════════
  ID   Name        Tokens  Conns  Temp  Gravity  Reflex-T
  100  SUTRA          0       0   128   0.50     —
  101  EXECUTION      0       0   200   0.30     127
  102  SHADOW         0       0   180   0.40     180
  103  CODEX          3       5   100   0.60     —
  104  MAP            0       0   150   0.50     200
  105  PROBE          0       0   200   0.20     —
  106  LOGIC          0       0   100   0.80     230
  107  DREAM          0       0   255   0.10     —
  108  ETHICS         0       0   120   0.70     —
  109  EXPERIENCE     0       0   200   0.30     127
  110  MAYA           0       0   150   0.50     —
```

### :traces (расширенный)
```
  ══ Experience Traces ══════════════════
  Total: 47  |  Avg weight: 0.54  |  Max weight: 0.72

  #   Weight  Shell                          Position              Age
  1   0.72    [0,0,0,50,200,180,30,75]       (1823, 5950, 10514)   4281
  2   0.68    [0,0,0,50,200,180,30,75]       (1823, 5950, 10514)   3892
  3   0.61    [0,0,0,130,200,180,180,120]     (21710,28871,12526)   2105
  4   0.55    [0,0,0,50,200,180,30,195]       (25290, 2605,17011)   1560
  ...
  47  0.50    [0,0,0,80,200,180,30,45]        (14026,31056,29977)   42
```

### :tension (расширенный)
```
  ══ Tension Traces ═════════════════════
  Active: 2

  #   Temperature  Pattern-hash      Created-at   Age (ticks)
  1   201          0xA7F3B2C1        154180       20
  2   145          0xC3D4E5F6        154150       50
```

### :depth (Cognitive Depth)
```
  ══ Cognitive Depth ════════════════════
  internal_dominance:  0.3
  min_coherence:       0.60
  max_passes:          3
  tension_threshold:   128

  ── statistics ─────────────────────────
  total impulses generated:   12
  total multi-pass events:    3
  avg passes when triggered:  2.1
  tension traces created:     8
  tension traces expired:     6
```

### :arbiter (Arbiter state)
```
  ══ Arbiter ════════════════════════════
  last_routing:
    was_reflex:      true
    traces_matched:  46
    dominant:        101 (EXECUTION)
    confidence:      0.72

  ── thresholds per domain ──────────────
  Domain   Reflex-T   Assoc-T   Cooldown   GUARDIAN
  101      127        50        0          ✓
  102      180        40        2          ✓
  103      —          —         —          —
  106      230        100       5          ✓
  107      0          25        0          ✗
```

### :perf (Performance)
```
  ══ Performance ════════════════════════
  uptime:          25.7s
  total ticks:     154200
  actual rate:     99.8 Hz (target: 100 Hz)

  ── tick breakdown ─────────────────────
  avg hot path:    42 ns
  avg with schedule: 96.5 ns
  peak tick:       1.2 µs (tick #108542)
  budget (1ms):    0.01% used

  ── periodic tasks ─────────────────────
  adaptation:      308 calls  (every 50 ticks)
  horizon_gc:      30 calls   (every 500 ticks)
  dream:           154 calls  (every 100 ticks)
  reconcile:       77 calls   (every 200 ticks)
  snapshot:        3 calls    (every 5000 ticks)
  tension_check:   1542 calls (every 10 ticks)
```

### :events [N]
```
  ══ Last 5 COM Events ══════════════════
  ID        Type              Subtype    Domain  Source  Target
  847305    TokenCreate       none       100     100     42
  847304    TokenMove         gravity    101     101     42
  847303    ReflexApproved    none       109     109     42
  847302    TokenTransform    none       101     101     41
  847301    InternalImpulse   tension    109     109     —
```

### :frontier
```
  ══ Causal Frontier ════════════════════
  state:           Active
  size:            3 entities (max: 10000)
  budget:          997 remaining (max: 1000)
  storm_threshold: 500
  last_storm:      never
```

### :guardian
```
  ══ GUARDIAN ════════════════════════════
  reflexes approved:    42
  reflexes vetoed:      0
  patterns inhibited:   0
  codex rules updated:  0
  access denied:        0
```

### :watch / :unwatch
```
axiom> :watch tension
  watching: tension (prints on change)

axiom> :watch tps
  watching: tps (prints every 10s)

  [tps=99.8 traces=47 tension=1]     ← появляется при изменении

axiom> :unwatch tension
  unwatched: tension
```

---

## 4. Приоритет реализации

### Фаза 1 (критично — сейчас):
- `:detail [off/min/mid/max]` — уровни детализации
- Расширенный вывод при вводе текста (секции input/routing/processing/output)
- `:status` расширенный
- `:traces` расширенный (с weight, shell, position, age)
- `:tension` расширенный (с temperature, age)
- `:depth` — состояние Cognitive Depth
- `:arbiter` — thresholds и last routing
- `:perf` — производительность

### Фаза 2 (полезно):
- `:domain <id>` — детали одного домена
- `:events [N]` — последние COM события
- `:frontier` — состояние frontier
- `:guardian` — статистика
- `:watch / :unwatch`
- `:config`

### Фаза 3 (когда понадобится):
- `:trace <index>` — один трейс детально
- `:connections`
- `:dream`
- `:multipass`
- `:reflector`
- `:impulses`
- `:help <command>`

---

## 5. Примечания для реализации

- Все команды читают состояние Engine **read-only**. Никаких мутаций кроме `:inject`, `:tick`, `:save`, `:load`, `:reset`.
- Формат чисел: weight как `0.XX` (не u8), position как `(X, Y, Z)`, time как `ns/µs/ms` автоматически.
- Age = `current_tick - created_at_tick` (или `current_event_id - created_at_event_id`).
- `:detail` сохраняется в CliConfig — персистентно между вводами, но не между запусками (или через config file).
- Unicode-рамки (`══`, `──`) — опционально. Если терминал не поддерживает — ASCII fallback (`==`, `--`).
