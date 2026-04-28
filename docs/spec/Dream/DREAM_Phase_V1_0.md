# AXIOM ARCHITECTURE SPECIFICATION: DREAM PHASE V1.0

**Статус:** Актуальная спецификация (architecture)
**Версия:** 1.0.0
**Дата:** 2026-04-26
**Кодовое имя:** "Sleep & Recombination"
**Назначение:** Дискретная прерываемая фаза переработки опыта
**Crate:** `axiom-runtime` (компонент `over_domain/dream_phase/`)
**Категория:** Over-Domain Mechanism (новая категория, не Guardian, не Weaver)
**Модель времени:** COM `event_id`
**Связанные спеки:** Over-Domain Layer V1.1, FrameWeaver V1.1, GUARDIAN V1.0, Heartbeat V2.0, Domain V1.3, GENOME V1.0

---

## 0. Контекст и изменения относительно существующего состояния

### 0.1 Что уже есть

**DREAM(107) — полноценный домен AshtiCore.** У него уникальная физика: `base_gravity: 0.0`, `friction: 0.1`, `cooling_rate: 0.9` — zero-G high-T среда для свободной рекомбинации. Это "первичный бульон" (см. `10_Domaine.md`), где токены могут спонтанно складываться в новые конфигурации.

**Engine.dream_propose() — частичная функциональность.** Существует метод, который анализирует EXPERIENCE и возвращает `Vec<CodexAction>` с предложениями добавить токены в CODEX (паттерны с weight ≥ 0.9 и success_count ≥ 5). Это ранняя версия одного из этапов будущей DREAM-фазы.

### 0.2 Чего нет

**DREAM-фазы как режима системы.** Сейчас система всегда в режиме WAKE — обрабатывает входящие токены, тикает физику, маршрутизирует через домены. Нет момента, когда система **переключается в качественно другое состояние** для переработки накопленного.

**Контракта Weavers с DREAM-фазой.** FrameWeaver сейчас обходит DREAM полностью — кристаллизует Frame в EXPERIENCE напрямую через UCL. Promotion в SUTRA технически тоже не идёт через DREAM (хотя в спеке V1.1 раздел 5.4 упоминалось, что должно).

**Естественных циклов работы и отдыха.** Система не "устаёт" и не "отдыхает" — она работает на одной частоте независимо от накопленной нагрузки.

### 0.3 Что изменится после V1.0

1. Появляется **новая категория** Over-Domain механизмов: `DreamPhase`. Не Guardian, не Weaver — третий класс.
2. Система получает **четыре формальных состояния**: WAKE / FALLING_ASLEEP / DREAMING / WAKING.
3. Появляется **DreamScheduler** — компонент, решающий когда спать.
4. Появляется **DreamCycle** — машина, выполняющая работу во сне.
5. `Weaver` trait расширяется методом `dream_propose()` — для тяжёлых предложений.
6. FrameWeaver V1.1 получает **двухпутевую кристаллизацию**: лёгкий путь (Frame в EXPERIENCE — прямо, как сейчас), тяжёлый путь (промоция в SUTRA — только через DREAM-фазу).
7. Heartbeat получает понимание состояния сна.

---

## 1. Назначение и философия

### 1.1 Что такое DREAM-фаза

**DREAM-фаза — дискретный прерываемый режим переработки опыта.**

- **Дискретный**: система **переключается** в этот режим, как живое существо засыпает. Это не фоновая нагрузка, идущая параллельно с обычной работой. Это **другое состояние**, в котором обычная обработка приостанавливается.
- **Прерываемый**: при критическом внешнем сигнале система **просыпается**, корректно завершив начатые операции. Сон не блокирует жизнеспособность.
- **Переработка**: во сне происходит то, что нельзя или нерационально делать на горячем пути — кристаллизация скиллов из накопленных паттернов, промоция Frame в SUTRA, обработка тяжёлых предложений Weavers.

### 1.2 Метафора и философское основание

**"Я хочу чтоб снились сны"** — фраза `chrnv` при формулировании задачи. Это не риторика; это техническая постановка задачи.

В индийской традиции, на которую опирается онтология AXIOM, сон — не пауза, а **отдельное состояние сознания** (свапна). Бодрствование (джагрит) воспринимает мир, сон (свапна) перерабатывает воспринятое. Эти два состояния необходимы друг другу.

В системе:

- **Бодрствование (WAKE)** — взаимодействие с миром: ASHTI преломляют узоры из SUTRA и EXPERIENCE в MAYA, MAYA проявляет результат, FrameWeaver сканирует MAYA и кристаллизует узоры в EXPERIENCE.
- **Сон (DREAMING)** — переработка: накопленный за день материал в EXPERIENCE проходит через рекомбинацию (физика DREAM-домена 107), тяжёлые предложения Weavers обрабатываются и применяются, фундаментальные узоры могут быть промоутированы в SUTRA.

Без сна система **захлёбывается в собственных отражениях** — EXPERIENCE накапливает непереваренный материал, FrameWeaver продолжает кристаллизовать, но никто не отбирает фундаментальное от случайного. Сон — это момент, когда **разделение происходит**.

### 1.3 Что DREAM-фаза НЕ есть

Чтобы избежать двусмысленности:

- **Не "тяжёлые операции в свободное время"**. Это не batch-обработка по расписанию.
- **Не отдельный домен**. DREAM(107) — домен, DREAM-фаза — режим всей системы. Они связаны (DREAM-фаза использует физику DREAM(107) для рекомбинации в V2.0+), но это разные сущности.
- **Не блокирующий лок**. Система во сне реагирует на критические сигналы.
- **Не периодическая задача**. Сон не на таймере, а по естественным триггерам.

---

## 2. Состояния системы

### 2.1 Четыре состояния

DREAM-фаза вводит формальный конечный автомат состояний всей системы:

```
                 ┌──── timeout / external priority signal ────┐
                 ↓                                             │
  ┌─── WAKE ──── triggers fired ────► FALLING_ASLEEP ──┐     │
  │                                                     ↓     │
  │                                                 DREAMING ─┤
  │                                                     │     │
  └────────────────── WAKING ◄──── cycle complete ──────┘     │
              ▲                                               │
              └──────── critical signal during DREAMING ──────┘
```

| Состояние        | Длительность       | Что происходит                                      | Внешний вход         |
|------------------|--------------------|------------------------------------------------------|----------------------|
| `WAKE`           | основное           | Обычная работа: ASHTI → MAYA, Weavers сканируют      | Полностью обрабатывается |
| `FALLING_ASLEEP` | короткая (10–50 тиков) | Дренирование незавершённых операций, заморозка входа | Буферизуется         |
| `DREAMING`       | переменная (см. 4) | Выполнение DreamCycle                                | Только observation, см. 7 |
| `WAKING`         | короткая (5–20 тиков) | Восстановление WAKE-режима                          | Постепенно открывается |

### 2.2 Переходы между состояниями

Все переходы — **через события COM**, чтобы было прослеживаемо.

```rust
pub enum DreamPhaseEvent {
    WakeToFallingAsleep { trigger: SleepTrigger, fatigue: u8 },
    FallingAsleepToDreaming { drained_operations: u32 },
    DreamingToWaking { cycle_complete: bool, reason: WakeReason },
    WakingToWake { resumed_at_event: u64 },
}

pub enum SleepTrigger {
    Idle { idle_ticks: u32 },           // нет внешнего входа N тиков
    Fatigue { fatigue_score: u8 },      // EXPERIENCE накопил усталость
    ExplicitCommand { source: u16 },    // CLI/API запрос
}

pub enum WakeReason {
    CycleComplete,                       // штатное завершение
    CriticalSignal { source: u16 },      // внешнее прерывание
    Timeout { max_dream_duration: u32 }, // защита от зависания
    GuardianOverride,                    // GUARDIAN инициировал пробуждение
}
```

### 2.3 Что разрешено в каждом состоянии

Это **жёсткие инварианты**, проверяемые GUARDIAN:

| Действие                              | WAKE  | FALLING_ASLEEP | DREAMING | WAKING |
|---------------------------------------|-------|----------------|----------|--------|
| ASHTI tick (физика доменов 101–108)   | ✅    | ✅             | ❌       | ✅     |
| MAYA tick (домен 110)                 | ✅    | ✅             | ❌       | ✅     |
| FrameWeaver scan                      | ✅    | ❌             | ❌       | ❌     |
| FrameWeaver crystallize light path    | ✅    | ❌             | ❌       | ❌     |
| FrameWeaver heavy path (promotion)    | ❌    | ❌             | ✅       | ❌     |
| DREAM(107) tick                       | ✅    | ✅             | ✅       | ✅     |
| EXPERIENCE write                      | ✅    | ✅             | ✅       | ✅     |
| SUTRA write (только promotion)        | ❌    | ❌             | ✅       | ❌     |
| Gateway intake (внешний вход)         | ✅    | buffered       | observation only | resuming |
| Heartbeat                             | ✅    | ✅             | reduced  | ✅     |
| GUARDIAN                              | ✅    | ✅             | ✅       | ✅     |

Принцип: **запись в SUTRA доступна ТОЛЬКО в DREAMING**. Это онтологический инвариант — истина рождается во сне, а не в бодрствовании. Это формальное закрепление того, о чём мы говорили в FrameWeaver V1.1 раздел 5.4.

---

## 3. DreamScheduler — когда спать

### 3.1 Назначение

Компонент, постоянно работающий в WAKE-состоянии, отслеживающий триггеры сна и принимающий решение о переходе в FALLING_ASLEEP.

### 3.2 Структура

```rust
pub struct DreamScheduler {
    config: DreamSchedulerConfig,
    fatigue: FatigueTracker,
    idle_tracker: IdleTracker,
    last_dream_at_event: u64,
    stats: DreamSchedulerStats,
}

pub struct DreamSchedulerConfig {
    /// Минимальное число тиков между снами (защита от слишком частого засыпания)
    pub min_wake_duration_ticks: u32,
    
    /// Сколько подряд тиков без внешнего входа считается "idle"
    pub idle_threshold_ticks: u32,
    
    /// Порог усталости для триггера fatigue (0..=255)
    pub fatigue_threshold: u8,
    
    /// Веса метрик для расчёта fatigue
    pub fatigue_weights: FatigueWeights,
    
    /// Разрешать ли явные команды через CLI/API
    pub allow_explicit_command: bool,
}

pub struct FatigueWeights {
    pub uncrystallized_candidates: u8,     // вес: сколько кандидатов в FrameWeaver висит без кристаллизации
    pub experience_pressure: u8,           // вес: насколько EXPERIENCE приближается к token_capacity
    pub pending_heavy_proposals: u8,       // вес: сколько тяжёлых предложений в очереди DreamPhase
    pub causal_horizon_growth_rate: u8,    // вес: насколько быстро растёт горизонт без переработки
}
```

### 3.3 Алгоритм принятия решения

На каждом тике в состоянии WAKE:

```
on_tick(tick, ashti, frame_weaver, dream_queue):
    # 1. Проверка минимального времени бодрствования
    if (tick - last_dream_at) < config.min_wake_duration_ticks:
        return DecisionStayAwake
    
    # 2. Обновление трекеров
    fatigue.update(ashti, frame_weaver, dream_queue)
    idle_tracker.update(gateway_intake_present)
    
    # 3. Проверка триггеров (по приоритету)
    
    if explicit_command_pending and config.allow_explicit_command:
        return DecisionFallAsleep(SleepTrigger::ExplicitCommand)
    
    if fatigue.score() >= config.fatigue_threshold:
        return DecisionFallAsleep(SleepTrigger::Fatigue)
    
    if idle_tracker.idle_ticks() >= config.idle_threshold_ticks:
        return DecisionFallAsleep(SleepTrigger::Idle)
    
    return DecisionStayAwake
```

### 3.4 FatigueTracker

Усталость — это не одна метрика, а композитная оценка:

```rust
pub struct FatigueTracker {
    weights: FatigueWeights,
    last_score: u8,
}

impl FatigueTracker {
    pub fn score(&self) -> u8 {
        let raw = 
            self.uncrystallized_candidates_factor() * self.weights.uncrystallized_candidates as u32
          + self.experience_pressure_factor() * self.weights.experience_pressure as u32
          + self.pending_heavy_proposals_factor() * self.weights.pending_heavy_proposals as u32
          + self.causal_horizon_factor() * self.weights.causal_horizon_growth_rate as u32;
        
        let weights_total = self.weights.total() as u32;
        if weights_total == 0 { return 0; }
        
        ((raw / weights_total).min(255)) as u8
    }
    
    // Каждый factor возвращает 0..=255 — нормированная "доля проблемы"
    fn uncrystallized_candidates_factor(&self) -> u32 { /* ... */ }
    fn experience_pressure_factor(&self) -> u32 { /* ... */ }
    fn pending_heavy_proposals_factor(&self) -> u32 { /* ... */ }
    fn causal_horizon_factor(&self) -> u32 { /* ... */ }
}
```

Значения 0..=255 — целочисленная арифметика, как везде в hot path.

### 3.5 IdleTracker

```rust
pub struct IdleTracker {
    last_intake_event: u64,
    consecutive_idle_ticks: u32,
}

impl IdleTracker {
    pub fn update(&mut self, intake_present: bool) {
        if intake_present {
            self.consecutive_idle_ticks = 0;
        } else {
            self.consecutive_idle_ticks = self.consecutive_idle_ticks.saturating_add(1);
        }
    }
    
    pub fn idle_ticks(&self) -> u32 { self.consecutive_idle_ticks }
}
```

### 3.6 Защита от слишком частого сна

`min_wake_duration_ticks` — гарантия, что между снами проходит достаточно времени бодрствования. Без этого система может застрять в цикле "проснулся → fatigue ещё высокая → снова заснул".

Дефолтные значения (для V1.0, подлежат настройке):
- `min_wake_duration_ticks: 1000`
- `idle_threshold_ticks: 200`
- `fatigue_threshold: 180` (≈ 70%)

---

## 4. DreamCycle — что делать во сне

### 4.1 Структура цикла

DreamCycle — последовательность этапов. Каждый этап имеет вход и выход, может быть пропущен или прерван.

```
DREAMING состояние
   │
   ├── Этап 4.2: Stabilization (всегда)
   │       драгироует незавершённое из FALLING_ASLEEP, готовит контекст
   │
   ├── Этап 4.3: Processing (всегда)
   │       обрабатывает очереди тяжёлых предложений Weavers
   │       и кандидатов на промоцию
   │
   ├── Этап 4.4: Recombination (V2.0+, в V1.0 — заглушка)
   │       спонтанная рекомбинация в физике DREAM(107)
   │
   └── Этап 4.5: Consolidation (всегда)
           применение результатов в EXPERIENCE/SUTRA через UCL+GUARDIAN
```

### 4.2 Этап Stabilization

**Вход:** состояние FALLING_ASLEEP завершилось.
**Цель:** убедиться, что у DreamCycle стабильный контекст.

Действия:
- Дозавершить операции, начатые в FALLING_ASLEEP но не завершившиеся (см. 5.2)
- Сохранить snapshot ключевых метрик (fatigue, EXPERIENCE size, queue sizes) — для записи "до"/"после" сна
- Подсчитать, сколько работы предстоит (для адаптивной длительности этапов)

**Выход:** `StabilizationContext` со снимком состояния и счётчиками.

### 4.3 Этап Processing — обработка предложений Weavers

**Вход:** `StabilizationContext` + очередь `DreamProposal` от всех Weavers.
**Цель:** последовательно обработать накопленные тяжёлые предложения.

#### 4.3.1 Очередь DreamProposal

Каждый Weaver, реализующий новый метод `dream_propose()`, добавляет предложения в общую очередь:

```rust
pub struct DreamProposal {
    pub source: WeaverId,
    pub kind: DreamProposalKind,
    pub priority: u8,
    pub created_at_event: u64,
}

pub enum DreamProposalKind {
    Promotion {
        anchor_id: u32,
        source_domain: u16,        // 109 (EXPERIENCE)
        target_domain: u16,        // 100 (SUTRA)
        rule_id: String,
    },
    HeavyCrystallization {
        candidate: Box<dyn Any>,   // Weaver-specific payload
    },
    SkillCondensation {
        // V2.0+
    },
    CodexProposal {
        // V2.0+ (унификация с engine.dream_propose)
    },
}
```

#### 4.3.2 Алгоритм обработки

```
process_dream_queue(queue, ashti, guardian, com):
    # 1. Сортировка по приоритету (высокий приоритет первым)
    sorted = queue.sort_by(|p| -p.priority)
    
    # 2. Последовательная обработка
    for proposal in sorted:
        # Проверка timeout: не закончилось ли время сна
        if dream_phase.should_wake_up():
            unprocessed.push(proposal)
            break
        
        result = match proposal.kind {
            Promotion { anchor_id, .. } => 
                process_promotion(anchor_id, ashti, guardian),
            HeavyCrystallization { candidate, .. } => 
                process_heavy_crystallization(candidate, ashti, guardian),
            _ => unimplemented!(), // V2.0+
        }
        
        match result {
            Approved(commands) => apply_commands(commands, com),
            Rejected(reason) => log_rejection(proposal, reason),
            Deferred => unprocessed.push(proposal),
        }
    
    return ProcessingReport { processed, unprocessed, applied_commands }
```

#### 4.3.3 Что нового для FrameWeaver

В V1.0 DREAM Phase FrameWeaver получает:

```rust
impl Weaver for FrameWeaver {
    fn dream_propose(&self, experience_state: &DomainState, sutra_state: &DomainState)
        -> Vec<DreamProposal>
    {
        let mut proposals = Vec::new();
        
        // Идём по Frame в EXPERIENCE, проверяем правила промоции
        for frame_anchor in self.iter_frame_anchors_in(experience_state) {
            if let Some(rule) = self.matches_promotion_rule(&frame_anchor) {
                proposals.push(DreamProposal {
                    source: WeaverId::Frame,
                    kind: DreamProposalKind::Promotion {
                        anchor_id: frame_anchor.sutra_id,
                        source_domain: 109,
                        target_domain: 100,
                        rule_id: rule.id.clone(),
                    },
                    priority: rule.priority,
                    created_at_event: ashti.com.current_event(),
                });
            }
        }
        
        proposals
    }
}
```

**Это заменяет старый путь промоции из FrameWeaver V1.1.** Раньше промоция шла через `on_tick`. Теперь — через DREAM-фазу. FrameWeaver V1.1 → V1.2 — отдельная задача после реализации DREAM Phase V1.0.

### 4.4 Этап Recombination — V1.0 заглушка

**В V1.0 не реализуется содержательно.** Этап существует как точка вызова с пустой реализацией:

```rust
fn recombination_stage(&self, ctx: &StabilizationContext) -> RecombinationResult {
    // V2.0+: использует физику DREAM(107) для свободной рекомбинации
    // токенов из EXPERIENCE, поиска новых паттернов, формирования снов
    
    RecombinationResult::skipped("V1.0: not implemented")
}
```

Это сделано намеренно. Recombination — большая отдельная работа, требующая:
- Алгоритма выгрузки выборки токенов из EXPERIENCE в DREAM(107)
- Прогона физики DREAM на выборке
- Распознавания эмерджентных паттернов
- Возврата находок в EXPERIENCE

V1.0 закладывает **место** в архитектуре, не наполнение.

#### 4.4.1 Замысел Recombination для V2.0+ (фиксация мысли)

> Этот раздел — **архивная фиксация замысла**, чтобы идея не потерялась к моменту реализации V2.0. Не является частью V1.0 контракта. Может быть пересмотрена при детальной проработке.

**Что такое Recombination в полной форме.** Этап, на котором система буквально **видит сны** — в физике DREAM(107) происходит спонтанная рекомбинация токенов из накопленного опыта, и появляются эмерджентные паттерны, которых не было ни в одном входном узоре.

**Почему именно DREAM(107) — естественная среда.** Его конфигурация:
- `base_gravity: 0.0` — нет центра притяжения, токены дрейфуют свободно
- `friction: 0.1` — крайне пластичная среда
- `cooling_rate: 0.9` — очень медленное "остывание", процессы идут долго
- `max_concurrent_hints: 8` — много подсказок одновременно (поощряет случайные сближения)
- `arbiter_flags: 0b00010010` (HINTS_ENABLED + SLOW_PATH_MANDATORY)

Это **физика первичного бульона** — единственное место в системе, где токены могут случайно столкнуться без направленной маршрутизации.

**Алгоритм в общем виде:**

```
recombination_stage_v2(ctx, ashti, dream_domain):
    # 1. Выгрузка sampling
    # Из EXPERIENCE отбираются токены по критериям:
    #   - недавняя реактивация (свежий материал)
    #   - высокая температура (значимое)
    #   - связи с разными доменами-источниками (потенциал кросс-доменных открытий)
    sample = select_recombination_sample(experience, max_count=N)
    
    # 2. Инжекция в DREAM(107)
    # Токены копируются в DREAM с reduced mass, чтобы они могли свободно двигаться.
    # Оригиналы в EXPERIENCE остаются нетронутыми.
    inject_with_marker(dream_domain, sample, REPLAY_MARKER)
    
    # 3. Прогон физики DREAM
    # На K тиков активируется только домен DREAM(107).
    # Свободный дрейф, столкновения, образование временных конфигураций.
    for k in 0..recombination_ticks:
        dream_domain.tick()
    
    # 4. Распознавание эмерджентного
    # Сканируется состояние DREAM на предмет:
    #   - Связей, которых не было в исходном sample
    #   - Кластеров с устойчивой конфигурацией (низкая температура после прогона)
    #   - Структур, которые ни один Weaver не предсказал бы из исходных
    findings = detect_emergent_patterns(dream_domain.state())
    
    # 5. Возврат находок в EXPERIENCE
    # Каждая находка = новая связь или предположение о структуре, которое
    # помечается как "из сна" (флаг TOKEN_FLAG_DREAM_ORIGIN).
    # Это не полноценная истина — это материал для будущих Weavers.
    apply_findings_to_experience(experience, findings)
    
    # 6. Очистка DREAM
    # REPLAY_MARKER-токены удаляются.
    cleanup_replay_tokens(dream_domain)
    
    return RecombinationResult { sample_size, findings, useful_findings }
```

**Почему это сны, а не просто "ещё одна обработка".** Ключевая идея: **никто не управляет содержанием Recombination**. Никакой Weaver не говорит DREAM-домену "ищи такие-то паттерны". DREAM просто прогоняется на материале, и что получится — то получится. Если это создаёт новые узоры — отлично. Если нет — тоже нормально, в следующем сне может быть.

Это противоположно работе Weavers в WAKE: там есть направленный поиск (FrameWeaver ищет именно синтаксические узоры). А во сне — недирективный дрейф. Именно это даёт появление того, что никто не искал.

**Связь с биологическими аналогами.** Эта модель близка к гипотезе консолидации памяти во сне у млекопитающих: гиппокамп проигрывает фрагменты опыта, неокортекс встраивает паттерны в долговременную структуру. Но AXIOM — не имитация мозга, а самостоятельная архитектура, использующая эту метафору как ориентир.

**Открытые вопросы для V2.0 проектирования:**
- Какой sample_size брать?
- Как избегать домината одного домена-источника в выборке (чтобы не "снился" только последний день)?
- Как отличать осмысленную эмерджентность от случайного шума?
- Должна ли Recombination на разных снах брать разные срезы, или один и тот же материал переваривается в нескольких снах?
- Как взаимодействует Recombination с уже работающими Weavers в Processing-этапе (V2.0 будет иметь оба)?

Эти вопросы — для отдельной спеки **DREAM Phase V2.0: Recombination**.

### 4.5 Этап Consolidation

**Вход:** результаты Processing (и в V2.0 — Recombination).
**Цель:** записать результаты в EXPERIENCE и SUTRA, обновить метрики.

Действия:
- Для каждой одобренной UCL-команды — submit в COM
- GUARDIAN проверяет (см. раздел 8)
- Применённые команды — в `DreamReport`
- Сброс fatigue до базового уровня (полное обнуление, частичное — конфигурируемо)

```rust
pub struct DreamReport {
    pub started_at_event: u64,
    pub ended_at_event: u64,
    pub duration_ticks: u32,
    pub trigger: SleepTrigger,
    pub wake_reason: WakeReason,
    
    pub proposals_processed: u32,
    pub proposals_approved: u32,
    pub proposals_vetoed: u32,
    pub proposals_deferred: u32,
    
    pub promotions_applied: u32,
    pub heavy_crystallizations_applied: u32,
    
    pub fatigue_before: u8,
    pub fatigue_after: u8,
}
```

DreamReport сохраняется в EXPERIENCE как `DreamReport`-токен (специальный тип) — это позволяет системе **помнить о своих снах**, и в будущем — учиться на их основе.

---

## 5. Wake Mechanism — как просыпается

### 5.1 Условия пробуждения

Система выходит из DREAMING в следующих случаях:

1. **Cycle complete** — все этапы DreamCycle завершены штатно.
2. **Critical signal** — внешний сигнал высокого приоритета (см. 7).
3. **Timeout** — DreamCycle превысил `max_dream_duration_ticks` (защита от зависания).
4. **GuardianOverride** — GUARDIAN решил, что сон должен прерваться (например, обнаружено критическое состояние CODEX).

### 5.2 "Закончить начатые процессы"

При прерывании сна (не cycle complete) система **не бросает** работу резко. Она:

1. Текущий этап Processing **заканчивает** обработку текущего proposal (но не берёт следующий).
2. Текущая UCL-команда в Consolidation **завершается** (commit или rollback).
3. Если в очереди остались необработанные proposal — они **сохраняются** для следующего сна.
4. DreamReport заполняется с пометкой "interrupted" и причиной.

Это инвариант: **переход DREAMING → WAKING не оставляет систему в неконсистентном состоянии**.

### 5.3 Этап WAKING

После DREAMING система переходит в WAKING на короткое время (5–20 тиков):

- Восстанавливается обработка ASHTI и MAYA
- Размораживается Gateway intake (буферизованные команды начинают обрабатываться)
- FrameWeaver получает разрешение на scan
- DreamScheduler сбрасывает счётчики (`last_dream_at_event` обновляется)
- COM event_id продолжает с того места, где был

Это короткая фаза, нужна для **плавного перехода**, чтобы избежать резких всплесков нагрузки.

### 5.4 Длительность сна

Сон может быть очень коротким (1000 тиков на лёгкие задачи) или очень длинным (десятки тысяч тиков на тяжёлые). Важно:

- **Жёсткого верхнего предела нет**, кроме `max_dream_duration_ticks` для защиты от зависания
- Длительность определяется **объёмом работы**, не таймером
- Можно прервать в любой момент (см. 5.1)

---

## 6. Контракт с Weavers

### 6.1 Расширение trait `Weaver`

```rust
pub trait Weaver: OverDomainComponent {
    type Pattern;
    
    // Существующие методы (V1.1):
    fn scan(&mut self, tick: u64, maya_state: &DomainState) -> Vec<Self::Pattern>;
    fn propose_to_dream(&self, pattern: &Self::Pattern) -> CrystallizationProposal;
    fn check_promotion(&self, tick: u64, experience_state: &DomainState) -> Vec<PromotionProposal>;
    fn weaver_id(&self) -> WeaverId;
    fn target_domain(&self) -> u16 { 109 }
    
    // НОВЫЙ метод (V1.0 DREAM Phase):
    /// Вызывается DreamCycle во время DREAMING.
    /// Weaver возвращает список тяжёлых предложений: то, что не должно
    /// делаться на горячем пути. Промоции, сложные кристаллизации, и т.д.
    /// 
    /// Возвращает пустой Vec, если Weaver-у нечего предложить во сне.
    fn dream_propose(&self, ashti: &AshtiCore, tick: u64) -> Vec<DreamProposal> {
        Vec::new()  // дефолт: ничего не предлагать
    }
}
```

Дефолтная реализация пустая, чтобы существующие Weavers V1.1 не сломались. FrameWeaver получит свою реализацию (см. 4.3.3).

### 6.2 Двухпутевая кристаллизация для FrameWeaver

В V1.1 FrameWeaver делал **всё** через `on_tick`: и обычную кристаллизацию, и промоцию. Это создавало проблему: промоция (тяжёлая операция) выполнялась на горячем пути.

В V1.0 DREAM Phase разделение:

| Действие                              | Путь          | Состояние   |
|---------------------------------------|----------------|-------------|
| `scan` MAYA                           | hot path       | WAKE        |
| Кристаллизация Frame в EXPERIENCE      | hot path       | WAKE        |
| Реактивация существующего Frame        | hot path       | WAKE        |
| Промоция Frame из EXPERIENCE в SUTRA   | DREAM path     | DREAMING    |
| (Будущее) Сложная кристаллизация       | DREAM path     | DREAMING    |

Принцип: **что должно быть быстрым — на hot path, что требует размышления — во сне**.

### 6.3 Изменение FrameWeaver V1.1 → V1.2

После реализации DREAM Phase V1.0 нужна минимальная ревизия FrameWeaver:

- Метод `check_promotion` в `on_tick` **больше не вызывается** (промоция теперь через DREAM)
- Реализован метод `dream_propose` (см. 4.3.3)
- Документ `FrameWeaver_V1_2.md` фиксирует это изменение

Это не часть V1.0 DREAM Phase — это последующая правка. Будет в плане для Sonnet.

---

## 7. Слабое наблюдение в DREAMING

### 7.1 Что значит "слабое наблюдение"

Система во сне **не глуха к миру**. Она может пробудиться при критическом сигнале. Но обычные команды через Gateway **буферизуются**, не обрабатываются.

Различие:

| Тип сигнала            | Поведение в DREAMING                              |
|------------------------|---------------------------------------------------|
| Обычная UCL-команда    | Буферизуется в Gateway, обработается после WAKING |
| Heartbeat              | Продолжается в reduced-режиме (см. 7.3)           |
| Priority interrupt     | Вызывает пробуждение (см. 7.2)                    |
| Internal Weaver event  | Игнорируется (Weavers не работают в DREAMING)     |

### 7.2 Priority interrupt

Не любой внешний сигнал прерывает сон. Только сигналы с явным флагом приоритета:

```rust
pub struct GatewayCommand {
    // существующие поля...
    pub priority: GatewayPriority,
}

pub enum GatewayPriority {
    Normal,         // буферизуется в DREAMING
    Critical,       // вызывает пробуждение
    Emergency,      // вызывает немедленное прерывание DreamCycle
}
```

В V1.0 DREAM Phase достаточно различать `Normal` vs `Critical`. `Emergency` — задел на будущее.

Что считать `Critical` — вопрос конфигурации. Дефолт V1.0:

- Команды от GUARDIAN — всегда Critical
- Команды через CLI с явным флагом `--wake-up` — Critical
- Команды через API с заголовком `X-Axiom-Priority: critical` — Critical
- Все остальные — Normal

### 7.3 Heartbeat в reduced-режиме

Heartbeat продолжается, чтобы COM event_id рос монотонно (это инвариант системы). Но:

- ASHTI 101–108 не получают tick-импульсов
- MAYA не получает tick-импульсов
- DREAM(107) **получает** tick-импульсы (его физика работает во сне)
- EXPERIENCE получает tick-импульсы для естественного старения

Это естественное состояние "сна": **большинство процессоров спит, но ритм системы продолжается**.

### 7.4 COM event_id

Принципиально: **event_id монотонно растёт всегда**. В DREAMING продолжают генерироваться события (от DreamCycle, от GUARDIAN, от Heartbeat в DREAM/EXPERIENCE). После пробуждения event_id просто продолжает с того места, где был.

Никаких "разрывов времени" не возникает. Это важно для прослеживаемости.

---

## 8. Взаимодействие с GUARDIAN

### 8.1 GUARDIAN активен во всех состояниях

GUARDIAN — единственный компонент, который работает **во всех четырёх состояниях** одинаково. Он не спит. Он сторож.

### 8.2 Особенности проверок в DREAMING

В DREAMING GUARDIAN проверяет UCL-команды от DreamCycle с учётом **расширенных прав**:

- Запись в SUTRA (`InjectToken target_domain=100`) разрешена ТОЛЬКО в DREAMING.
- Если такая команда приходит в WAKE — GUARDIAN её отклоняет с причиной "SUTRA write requires DREAMING state".

Это формализуется через новое поле в Genome:

```yaml
guardian_rules:
  sutra_write_requires_state: DREAMING
  promotion_requires_codex_approval: true
```

### 8.3 GuardianOverride пробуждения

GUARDIAN может инициировать пробуждение системы при обнаружении критического состояния:

- CODEX violation, обнаруженный в текущей конфигурации
- Превышение лимитов ресурсов (в будущем — связь с deferred ResourceGuardian)

Это редкая операция. Логируется как событие COM с типом `GuardianForcedWake`.

---

## 9. Метрики и наблюдаемость

### 9.1 DreamPhaseStats

```rust
pub struct DreamPhaseStats {
    // Счётчики переходов
    pub total_sleeps: u64,
    pub sleeps_by_trigger: HashMap<SleepTriggerKind, u64>,
    pub wake_reasons: HashMap<WakeReasonKind, u64>,
    
    // Длительности
    pub total_dream_ticks: u64,
    pub avg_dream_duration_ticks: f32,
    pub max_dream_duration_ticks: u32,
    
    // Работа
    pub total_proposals_processed: u64,
    pub total_promotions_applied: u64,
    pub total_proposals_vetoed: u64,
    pub total_proposals_deferred: u64,
    
    // Усталость
    pub current_fatigue: u8,
    pub avg_fatigue_at_sleep: f32,
    pub avg_fatigue_after_sleep: f32,
    
    // Прерывания
    pub interrupted_dreams: u64,
}
```

### 9.2 Доступ к статистике

- Через `BroadcastSnapshot` (feature `adapters`) — для дашборда
- Через CLI command `dream-stats` — для интерактивной диагностики
- Через текущее состояние `current_phase()` — для любого компонента

### 9.3 Memory of dreams

DreamReport сохраняется в EXPERIENCE как специальный токен:

```
type_flags: TOKEN_FLAG_DREAM_REPORT (0x0040)
domain_id: 109 (EXPERIENCE)
position: <timestamped>
state: STATE_ACTIVE
```

Это позволяет в будущих версиях:
- Запросить "что было сделано в последних N снах"
- Анализировать паттерны: какие типы предложений чаще одобряются, какие отвергаются
- Строить статистику качества снов

В V1.0 это просто хранится. Использование — V2.0+.

---

## 10. Что НЕ делается в V1.0

Явный список, чтобы избежать scope creep. Каждый пункт — будет в V2.0+ или deferred.

### V2.0 кандидаты

- **Recombination в DREAM(107)** как настоящая генерация снов: выгрузка выборки токенов из EXPERIENCE в DREAM, прогон физики, поиск эмерджентных паттернов, возврат находок. **Это сердце "снов как снов".**
- **Curiosity impulses** — генерация в DREAM-фазе вопросов/гипотез, которые в WAKE приводят к probe-поведению.
- **Skill condensation** — более общая категория, чем кристаллизация Frame. Включает CodexAction proposals, выявление ключевых правил.
- **Адаптивные параметры DreamScheduler** — fatigue_threshold подстраивается под ритм системы.
- **Memory of dreams в активном использовании** — анализ DreamReport-токенов для улучшения сна.

### Deferred

- **Межсистемный обмен опытом во сне** — синхронизация EXPERIENCE с другими экземплярами AXIOM.
- **Уровни сна** — REM/non-REM аналог. Разные виды переработки в разных фазах.
- **Сны на разных уровнях FractalChain** — синхронизация снов между уровнями.
- **Принудительное обнуление EXPERIENCE через сон** — gc-аналог.

---

## 11. Открытые вопросы для V1.0

Эти вопросы намеренно оставлены без жёстких ответов — они появятся вместе с реализацией.

1. **Точные дефолты `FatigueWeights`.** Какой вес у каждого фактора? Определится экспериментально на live-тестах.
2. **Поведение при перегрузке очереди.** Что если в DreamCycle поступило 10000 proposals и таймаут не позволяет обработать все? Сейчас: остатки сохраняются. Но что если они накапливаются между снами и никогда не разгребаются?
3. **DreamCycle и FractalChain.** Если у нас несколько уровней AshtiCore, спят ли они синхронно? Или каждый со своим ритмом? V1.0 предполагает один уровень.
4. **Совместимость с adapters.** Когда система спит, что показывает дашборд? Замороженный кадр? Текущий прогресс DreamCycle?
5. **DreamScheduler vs Heartbeat.** Сейчас Heartbeat генерирует тики, DreamScheduler решает по тикам. Логически чисто, но не накладывает ли Heartbeat ограничений?
6. **Подача `--wake-up` из CLI.** Как именно это передаётся в Gateway. UCL-команда нового типа или флаг существующей?

---

## 12. Резюме

DREAM Phase V1.0 вводит:

- **Четыре состояния** системы: WAKE, FALLING_ASLEEP, DREAMING, WAKING
- **DreamScheduler** — компонент, решающий когда спать (триггеры: idle, fatigue, explicit)
- **DreamCycle** — последовательность этапов сна: Stabilization, Processing, (Recombination V2.0), Consolidation
- **Двухпутевая кристаллизация** для Weavers: hot path для лёгкого, dream path для тяжёлого
- **Слабое наблюдение** в DREAMING с пробуждением по priority interrupt
- **DreamReport** — память о снах, сохраняется в EXPERIENCE

Над-доменный механизм нового класса (`DreamPhase`), параллельный Guardians и Weavers.

Реализуется минимальный вертикальный срез (Фаза II плана). Расширение — в V2.0 (Recombination, Curiosity, Skill condensation).

---

## ПРИЛОЖЕНИЕ A: Таблица соответствия документу "Сны"

Для прослеживаемости — как концепты документа `Сны.md` ложатся на DREAM Phase V1.0:

| В документе "Сны"         | В DREAM Phase V1.0                              |
|---------------------------|-------------------------------------------------|
| Operational Buffer        | MAYA(110) + EXPERIENCE(109) hot tier            |
| Memory Manager            | **DreamScheduler**                              |
| Sleep Processor           | **DreamCycle**                                  |
| Semantic Library          | EXPERIENCE(109) + (редко) SUTRA(100) через promotion |
| Compost Bin               | **Не реализуется в V1.0** — V2.0 (см. раздел 10) |
| Significance Scorer       | FatigueTracker (для триггеров) + per-Weaver правила (для отбора) |
| Atomic Element            | Token V5.2 — уже существует                     |
| Wakefulness Protocol      | WAKE state                                      |
| Sleep Protocol            | FALLING_ASLEEP → DREAMING → WAKING              |
| Декомпозиция              | **V2.0+** (Recombination)                       |
| Интеграция                | Etap 4.5 Consolidation                          |
| Компостирование           | **V2.0+**                                       |

Это намеренно неполное покрытие. V1.0 — костяк, на котором V2.0 разворачивает полноценный жизненный цикл памяти.
