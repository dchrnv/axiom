# DREAM Phase — Руководство

**Версия:** V1.0  
**Дата:** 2026-04-29

---

## Что это

DREAM-фаза — дискретный режим системы для переработки накопленного материала. Не фоновая задача, не батч по таймеру, а **другое состояние**, в которое система входит и из которого выходит — как живой организм засыпает и просыпается.

**Зачем это нужно.** В WAKE система принимает входящие токены, тикает физику, строит узоры (FrameWeaver). Постепенно в EXPERIENCE накапливаются кандидаты и паттерны, которые нельзя обработать "на горячем пути" — слишком дорого или онтологически недопустимо (запись в SUTRA требует состояния DREAMING). Без DREAM-фазы система захлёбывается: EXPERIENCE растёт, но не очищается, устойчивые паттерны не промоутируются.

**Главный инвариант.** Запись FRAME_ANCHOR в SUTRA-домены (domain_id кратен 100) — **только в состоянии DREAMING**. GUARDIAN ветирует любую такую запись вне DREAMING. Это не ограничение, а защита онтологической чистоты: SUTRA хранит вечные истины, и кандидат должен пройти полный цикл валидации, прежде чем туда попасть.

---

## Четыре состояния

```
  WAKE ──── (trigger) ──► FALLING_ASLEEP ──► DREAMING ──► WAKING ──► WAKE
                                                  │                    ▲
                                                  └── Critical сигнал ─┘
```

| Состояние       | Что происходит                                                      |
|-----------------|---------------------------------------------------------------------|
| `Wake`          | Обычная работа. Все команды обрабатываются нормально.               |
| `FallingAsleep` | Один тик: FrameWeaver отдаёт промоционные предложения, DreamCycle запускается, система переходит в Dreaming. |
| `Dreaming`      | DreamCycle обрабатывает предложения (Stabilization → Processing → Consolidation). Normal-команды игнорируются. |
| `Waking`        | Один тик: приоритетный буфер дренируется, idle-счётчик сбрасывается, система возвращается в Wake. |

Каждый переход занимает ровно один тик. FallingAsleep → Dreaming → Waking — это переходные состояния, не стабильные режимы.

---

## Кто принимает решение засыпать — DreamScheduler

`DreamScheduler` вызывается каждый WAKE-тик. Он смотрит на три триггера и возвращает `GoToSleep` или `StayAwake`.

### Три триггера

**1. Idle** — система не получала внешний intake слишком долго.

```
idle_ticks >= idle_threshold  →  GoToSleep(Idle)
```

`idle_ticks` — счётчик подряд идущих тиков без вызова `process_and_observe` с InjectToken. Любой такой вызов сбрасывает счётчик в ноль. Дефолтный порог — **200 тиков**.

**2. Fatigue** — накопилась усталость.

```
fatigue_score >= fatigue_threshold  →  GoToSleep(Fatigue)
```

`fatigue_score` — композитная оценка 0..=255, вычисляется каждый тик из четырёх факторов (см. ниже). Дефолтный порог — **180**.

**3. Explicit command** — CLI-команда `:force-sleep`.

```
submit_explicit_command(source_id)  →  GoToSleep(ExplicitCommand) на следующем тике
```

### Защита от rapid cycling

Перед проверкой триггеров Scheduler смотрит на `ticks_awake` — сколько тиков прошло с момента последнего пробуждения. Если `ticks_awake < min_wake_ticks` — любые триггеры игнорируются. Дефолт — **1000 тиков**. Это защита: система не должна засыпать сразу после пробуждения.

---

## Fatigue — оценка усталости

`FatigueTracker` каждый тик вычисляет взвешенную сумму четырёх факторов.

### Четыре фактора

| Фактор                       | Что измеряет                                          | Вес (дефолт) |
|------------------------------|-------------------------------------------------------|-------------|
| `uncrystallized_candidates`  | Сколько кандидатов FrameWeaver висит без кристаллизации | 80          |
| `experience_pressure`        | Насколько EXPERIENCE заполнен (tokens / capacity)      | 100         |
| `pending_heavy_proposals`    | Сколько тяжёлых предложений в очереди DreamCycle       | 60          |
| `causal_horizon_growth_rate` | Скорость роста causal horizon за тик                  | 30          |

### Как считается score

Каждый фактор нормируется в 0..=255 (отдельная логика для каждого), затем:

```
score = Σ(factor_i × weight_i) / Σ(weight_i)
score = clamp(score, 0, 255)
```

**Примеры:**
- EXPERIENCE пустой, кандидаты = 0 → score = 0
- EXPERIENCE заполнен на 80%, кандидатов 30 штук → score ≈ 150–180 (порог 180 → засыпание)
- Каждый pending proposal добавляет +30 к factor, что при весе 60 быстро толкает score вверх

---

## DreamCycle — что происходит во сне

DreamCycle — машина стадий, которая запускается при каждом засыпании. Каждый тик DREAMING — один вызов `advance()`.

### Три стадии

```
Stabilization  →  Processing  →  Consolidation  →  Complete
```

**Stabilization** (1 тик).  
Принять входящий пул предложений. Проверить онтологические инварианты. Перейти в Processing.

**Processing** (≥1 тика).  
Обработать предложения батчами (дефолт: 8 штук за тик).  
- `Promotion` → `build_promotion_commands()`: создать `InjectFrameAnchorPayload` + `BondTokensPayload` для каждого участника Frame.
- `HeavyCrystallization` → ветируется в V1.0 (заглушка).
- Если очередь пуста → сразу Consolidation.

**Consolidation** (1 тик).  
Собрать DreamReport: сколько предложений обработано/одобрено/ветировано, триггер засыпания, причина пробуждения, длительность. Отправить команды через UCL. Завершить цикл.

### DreamProposal

Предложения формирует `FrameWeaver.dream_propose()` — вызывается ровно один раз при `FallingAsleep`. Возвращает `Vec<DreamProposal>` для всех Frame в EXPERIENCE, которые соответствуют `PromotionRule`.

Каждое предложение:
```
DreamProposal {
    source:           WeaverId::Frame,
    kind:             Promotion { anchor_id, source_domain: 109, target_domain: 100, rule_id },
    created_at_event: u64,
}
```

### Прерывание

Если в DREAMING приходит Critical-сигнал (через `submit_priority_command(..., GatewayPriority::Critical)`), на следующем тике DreamCycle прерывается. Система переходит в Waking, незаконченные предложения теряются, `interrupted_dreams += 1`.

---

## Управление из CLI

Все команды доступны через `:help :dream`.

### Мониторинг

```
:dream-stats
```

Выводит текущее состояние, fatigue, idle-тики и накопленную статистику:

```
══ DREAM Phase ══════════════════════════
Current state:      Wake
Current fatigue:    12/255 (5%)
Idle ticks:         47
Total sleeps:       3
Total dream ticks:  18
Interrupted dreams: 0
By trigger:  Idle=3, Fatigue=0, Explicit=0
Cycles:  total=3, complete=3, timeout=0, approved=0, vetoed=0
```

**Что смотреть:**
- `fatigue` близко к 180 → система скоро уснёт
- `Interrupted dreams` > 0 → кто-то посылает Critical в неподходящий момент
- `approved=0` при `total=3` → предложений нет (нет кандидатов в EXPERIENCE) — это нормально на старте

### Принудительное засыпание

```
:force-sleep
```

Регистрирует ExplicitCommand. Система уснёт на следующем тике (если `ticks_awake >= min_wake_ticks`, иначе — при следующем допустимом тике).

### Пробуждение из DREAMING

```
:wake-up
```

Отправляет Critical-сигнал. Если система в DREAMING — на следующем тике перейдёт в Waking, ещё через один — Wake. Если система уже в Wake — команда безвредна (сигнал попадёт в буфер и будет обработан при следующем Waking).

---

## Настройка

### Конфигурация DreamScheduler

```rust
DreamSchedulerConfig {
    min_wake_ticks:    1000,  // минимум тиков в WAKE между снами
    idle_threshold:     200,  // тиков без intake → засыпание
    fatigue_threshold:  180,  // fatigue score (0..255) → засыпание
}
```

**Для тестов** обычно используется:
```rust
DreamSchedulerConfig { min_wake_ticks: 0, idle_threshold: 2, fatigue_threshold: 255 }
```

### Веса fatigue

```rust
FatigueWeights {
    uncrystallized_candidates:  80,   // кандидаты FrameWeaver
    experience_pressure:       100,   // заполненность EXPERIENCE
    pending_heavy_proposals:    60,   // тяжёлые предложения
    causal_horizon_growth_rate: 30,   // скорость роста горизонта
}
```

Уменьшение веса — меньше влияние фактора. Обнуление — фактор отключён.

### Конфигурация DreamCycle

```rust
DreamCycleConfig {
    max_dream_duration_ticks:  50_000,  // защита от зависания
    max_proposals_per_cycle:      100,  // потолок предложений
    enable_recombination:       false,  // V2.0+
    batch_size:                     8,  // предложений за тик Processing
}
```

---

## Статистика — что читать

### engine.dream_phase_stats

| Поле                | Что считает                                  |
|---------------------|----------------------------------------------|
| `total_sleeps`      | Сколько раз система входила в FallingAsleep  |
| `total_dream_ticks` | Суммарно тиков, проведённых в DREAMING       |
| `interrupted_dreams`| Сколько циклов было прервано Critical        |

### engine.dream_scheduler.stats

| Поле               | Что считает                     |
|--------------------|---------------------------------|
| `sleep_decisions`  | Всего решений GoToSleep         |
| `idle_triggers`    | Из них — по idle                |
| `fatigue_triggers` | Из них — по fatigue             |
| `explicit_triggers`| Из них — по явной команде       |

### engine.dream_cycle.stats

| Поле                | Что считает                                              |
|---------------------|----------------------------------------------------------|
| `total_cycles`      | Всего запущенных циклов                                  |
| `completed_cycles`  | Завершились нормально (Consolidation → Complete)         |
| `timed_out_cycles`  | Завершились по таймауту (max_dream_duration_ticks)       |
| `total_processed`   | Суммарно предложений рассмотрено                         |
| `total_approved`    | Одобрено (команды отправлены)                            |
| `total_vetoed`      | Ветировано GUARDIAN                                      |
| `total_promotions`  | Промоций Frame EXPERIENCE → SUTRA                        |

**Инвариант:** `total_cycles == completed_cycles + timed_out_cycles`. Если не выполняется — система сейчас в середине цикла.

---

## Типовые сценарии

### Система не засыпает

Смотреть `:dream-stats`:
- Если `idle_ticks` не растёт — есть постоянный intake. Нормально, если система занята.
- Если `fatigue` не растёт — EXPERIENCE пустой, кандидатов нет. Нормально на старте.
- Если `idle_ticks` растёт, но нет засыпания — проверить `min_wake_ticks`. Возможно, `ticks_awake` ещё не достиг порога.

### Система засыпает слишком часто

Увеличить `idle_threshold` или `min_wake_ticks`. Или увеличить `fatigue_threshold`.

### Много interrupted_dreams

Что-то посылает Critical в DREAMING. Найти кто вызывает `submit_priority_command(..., Critical)`. В V1.0 только `:wake-up` CLI делает это.

### approved=0, vetoed=0 при ненулевом total_cycles

Нет предложений от FrameWeaver — `dream_propose()` возвращает пустой вектор. Означает, что в EXPERIENCE нет Frame, соответствующих `PromotionRule`. Нормально пока система не накопила достаточно стабильных паттернов.

---

## Snapshot через broadcast

При `--features adapters` доступен `engine.snapshot_for_broadcast()` — возвращает `BroadcastSnapshot` с полем `dream_phase: Option<DreamPhaseSnapshot>`.

```rust
pub struct DreamPhaseSnapshot {
    pub state:           DreamPhaseState,
    pub current_fatigue: u8,
    pub idle_ticks:      u32,
    pub stats:           DreamPhaseStats,
    pub current_cycle:   Option<ActiveCycleSnapshot>,
}

pub struct ActiveCycleSnapshot {
    pub stage:      CycleStage,
    pub queue_size: usize,
}
```

`current_cycle` — `Some(...)` только когда `state == Dreaming`. В остальных состояниях — `None`.

---

## Файлы реализации

| Файл | Содержит |
|------|----------|
| [over_domain/dream_phase/state.rs](../../crates/axiom-runtime/src/over_domain/dream_phase/state.rs) | DreamPhaseState, GatewayPriority, DreamPhaseStats, SleepTrigger, WakeReason |
| [over_domain/dream_phase/fatigue.rs](../../crates/axiom-runtime/src/over_domain/dream_phase/fatigue.rs) | FatigueTracker, IdleTracker, FatigueWeights, FatigueSnapshot |
| [over_domain/dream_phase/scheduler.rs](../../crates/axiom-runtime/src/over_domain/dream_phase/scheduler.rs) | DreamScheduler, DreamSchedulerConfig, SleepDecision |
| [over_domain/dream_phase/cycle.rs](../../crates/axiom-runtime/src/over_domain/dream_phase/cycle.rs) | DreamCycle, DreamCycleConfig, DreamProposal, DreamReport, CycleStage |
| [engine.rs](../../crates/axiom-runtime/src/engine.rs) | Стейт-машина: tick_wake / tick_falling_asleep / tick_dreaming / tick_waking |
| [guardian.rs](../../crates/axiom-runtime/src/guardian.rs) | check_frame_anchor_sutra_write — SUTRA-инвариант |
| [broadcast.rs](../../crates/axiom-runtime/src/broadcast.rs) | DreamPhaseSnapshot, ActiveCycleSnapshot |
| [meta_commands.rs](../../crates/axiom-agent/src/meta_commands.rs) | CLI :dream-stats, :force-sleep, :wake-up |
