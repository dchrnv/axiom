# PLAN: DREAM Phase V1.0 Implementation

**Дата:** 2026-04-26
**Контекст:** Спецификация `DREAM_Phase_V1_0.md` утверждена. Этот план — пошаговая инструкция реализации V1.0 минимального вертикального среза. После V1.0 — наблюдение в живую, потом V2.0 с Recombination.
**Связанные документы:** `DREAM_Phase_V1_0.md`, `FrameWeaver_V1_1.md`, `Over_Domain_Layer_V1_1.md`, `BLUEPRINT.md`.

**Состояние входа:** FrameWeaver V1.1 стабилизирован. 1030 тестов, 0 failures. Hot path 238 ns/tick.

---

## Скоуп V1.0

**Реализуется:**
- Четыре формальных состояния: WAKE / FALLING_ASLEEP / DREAMING / WAKING
- DreamScheduler с тремя триггерами (idle, fatigue, explicit command)
- DreamCycle с этапами Stabilization, Processing, Consolidation
- Двухпутевая кристаллизация для FrameWeaver: hot path в EXPERIENCE остаётся, промоция в SUTRA уходит в DREAM-фазу
- Wake mechanism с поддержкой priority interrupt
- DreamReport как токен в EXPERIENCE
- Метрики наблюдаемости (DreamPhaseStats) в BroadcastSnapshot
- CLI команда для просмотра статистики (`dream-stats`)
- CLI команда для явного засыпания (`force-sleep`) и пробуждения (`wake-up`)

**НЕ реализуется (отложено):**
- Этап Recombination (заглушка с пустым return, сам замысел зафиксирован в спеке 4.4.1 для V2.0)
- Curiosity impulses
- Skill condensation
- Адаптивные параметры DreamScheduler
- Активное использование DreamReport-токенов (в V1.0 только запись)
- Уровни сна REM/non-REM
- Сны на разных уровнях FractalChain

---

## Этап 0 — подготовка документации (≈30 минут)

**Цель:** не накапливать дрейф спека-код, чётко обозначить что меняется.

### 0.1 Создать errata-документ

Создать `docs/specs/erratas/DREAM_Phase_V1_0_errata.md`:

```markdown
# DREAM Phase V1.0 — Errata

**Назначение:** фиксация неточностей и пробелов спецификации V1.0,
обнаруженных в процессе реализации. Все правки войдут в V1.1 после
стабилизации.

(пустой раздел — заполняется по ходу реализации)
```

### 0.2 Создать deferred-документ

Создать `docs/specs/deferred/DreamPhase_V2_plus.md`:

```markdown
# Deferred: вне DREAM Phase V1.0

Эти задачи **не выполняются** в V1.0 реализации. См. также раздел 10
спецификации DREAM_Phase_V1_0.md.

## V2.0 кандидаты
- **Recombination этап.** Полная реализация описана в спеке V1.0,
  раздел 4.4.1 (фиксация замысла). Требует отдельной спеки
  `DREAM_Phase_V2_0_Recombination.md`.
- **Curiosity impulses** — генерация в DREAM-фазе вопросов/гипотез.
- **Skill condensation** — обобщение CodexAction proposals и других видов скиллов.
- **Адаптивные параметры DreamScheduler** — fatigue_threshold подстраивается.
- **Активное использование DreamReport-токенов** — анализ снов системой.

## Deferred (без сроков)
- **Межсистемный обмен опытом во сне.**
- **Уровни сна (REM/non-REM аналог).**
- **Сны на разных уровнях FractalChain.**
- **Принудительное обнуление EXPERIENCE через сон.**
```

### Критерий готовности этапа 0
- [ ] errata и deferred файлы созданы
- [ ] Тесты остаются зелёными (изменений в коде нет)

---

## Этап 1 — состояния и Gateway priority (≈3 часа)

**Цель:** ввести машину состояний и расширить Gateway для поддержки priority,
без какой-либо логики DreamCycle. Подготовка фундамента.

### 1.1 Ввести enum DreamPhaseState

Создать `axiom-runtime/src/over_domain/dream_phase/mod.rs`:

```rust
pub mod state;
pub mod scheduler;       // создаётся в этапе 2
pub mod cycle;           // создаётся в этапе 3
pub mod fatigue;         // создаётся в этапе 2

pub use state::*;
```

Создать `axiom-runtime/src/over_domain/dream_phase/state.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DreamPhaseState {
    Wake = 0,
    FallingAsleep = 1,
    Dreaming = 2,
    Waking = 3,
}

impl Default for DreamPhaseState {
    fn default() -> Self { DreamPhaseState::Wake }
}

#[derive(Debug, Clone, Copy)]
pub enum SleepTrigger {
    Idle { idle_ticks: u32 },
    Fatigue { fatigue_score: u8 },
    ExplicitCommand { source: u16 },
}

#[derive(Debug, Clone, Copy)]
pub enum WakeReason {
    CycleComplete,
    CriticalSignal { source: u16 },
    Timeout { max_dream_duration: u32 },
    GuardianOverride,
}

#[derive(Debug, Clone)]
pub enum DreamPhaseEvent {
    WakeToFallingAsleep { trigger: SleepTrigger, fatigue: u8 },
    FallingAsleepToDreaming { drained_operations: u32 },
    DreamingToWaking { cycle_complete: bool, reason: WakeReason },
    WakingToWake { resumed_at_event: u64 },
}
```

Все enum derive `Debug, Clone`. SleepTrigger и WakeReason также `Copy`. Тесты сериализации/десериализации в этой итерации не нужны.

### 1.2 Расширить GatewayCommand priority

В `axiom-runtime/src/gateway.rs` (или где живёт Gateway) добавить поле priority в команду.

**ВАЖНО для имплементера:** перед изменением сигнатуры — посмотри, **используется ли GatewayCommand в публичном API** (через адаптеры, JSON-сериализацию). Если да — добавление поля **не должно ломать существующий вход**. Решение: значение по умолчанию `Normal`, при десериализации без поля используется дефолт.

```rust
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum GatewayPriority {
    #[default]
    Normal = 0,
    Critical = 1,
    Emergency = 2,    // V1.0 ведёт себя так же как Critical, флаг на будущее
}

pub struct GatewayCommand {
    // ... существующие поля
    pub priority: GatewayPriority,
}
```

Все места создания GatewayCommand — обновить так, чтобы по умолчанию ставился `Normal`. CLI команды без явного флага — Normal. Команды от GUARDIAN — Critical (см. этап 5).

### 1.3 Добавить хранение состояния DreamPhase в AxiomEngine

В `AxiomEngine`:

```rust
pub struct AxiomEngine {
    // ...
    dream_phase_state: DreamPhaseState,
    dream_phase_stats: DreamPhaseStats,
    pending_priority_intakes: VecDeque<GatewayCommand>, // буфер во сне
}
```

`DreamPhaseStats` — пока пустая структура (поля заполнятся в этапах 2-4):

```rust
#[derive(Debug, Default, Clone)]
pub struct DreamPhaseStats {
    pub total_sleeps: u64,
    pub total_dream_ticks: u64,
    pub interrupted_dreams: u64,
    // дополнится в этапах 2-4
}
```

### 1.4 Тесты этапа 1

**Test 1.4.a — `gateway_command_default_priority_is_normal`:**
```
Создать GatewayCommand через существующий конструктор/Default.
Проверить, что priority == Normal.
```

**Test 1.4.b — `dream_phase_state_starts_as_wake`:**
```
Создать AxiomEngine.
Проверить, что dream_phase_state == DreamPhaseState::Wake.
```

**Test 1.4.c — `gateway_priority_serialization_backward_compat`:**
```
Если GatewayCommand сериализуется (JSON/binary):
1. Сериализовать без поля priority (старый формат)
2. Десериализовать
3. Проверить, что priority == Normal по умолчанию
```

### Критерий готовности этапа 1
- [ ] Модуль `dream_phase/` создан с пустым state.rs
- [ ] DreamPhaseState, SleepTrigger, WakeReason, DreamPhaseEvent определены
- [ ] GatewayCommand расширен полем priority с дефолтом Normal
- [ ] AxiomEngine хранит dream_phase_state и пустой DreamPhaseStats
- [ ] Тесты 1.4.a-c проходят
- [ ] Существующие 1030 тестов остаются зелёными
- [ ] Hot path не просел больше чем на 5 ns (просто из-за добавления поля в struct)

---

## Этап 2 — DreamScheduler (≈4-6 часов)

**Цель:** компонент, который **только решает** когда спать, ничего не делает.

### 2.1 FatigueTracker

Создать `axiom-runtime/src/over_domain/dream_phase/fatigue.rs`:

```rust
#[derive(Debug, Clone, Copy)]
pub struct FatigueWeights {
    pub uncrystallized_candidates: u8,
    pub experience_pressure: u8,
    pub pending_heavy_proposals: u8,
    pub causal_horizon_growth_rate: u8,
}

impl Default for FatigueWeights {
    fn default() -> Self {
        // Дефолты для V1.0 — экспериментальные, подлежат настройке.
        // Сумма не обязана быть 255: нормировка происходит при делении на total.
        Self {
            uncrystallized_candidates: 80,
            experience_pressure: 100,
            pending_heavy_proposals: 60,
            causal_horizon_growth_rate: 30,
        }
    }
}

impl FatigueWeights {
    pub fn total(&self) -> u32 {
        self.uncrystallized_candidates as u32
        + self.experience_pressure as u32
        + self.pending_heavy_proposals as u32
        + self.causal_horizon_growth_rate as u32
    }
}

#[derive(Debug, Default)]
pub struct FatigueSnapshot {
    pub uncrystallized_candidates: u32,
    pub experience_token_count: u32,
    pub experience_capacity: u32,
    pub pending_heavy_proposals: u32,
    pub causal_horizon_delta: u64,    // delta от прошлого замера
    pub ticks_since_last_check: u32,
}

pub struct FatigueTracker {
    weights: FatigueWeights,
    last_snapshot: FatigueSnapshot,
    last_score: u8,
    last_horizon: u64,
}

impl FatigueTracker {
    pub fn new(weights: FatigueWeights) -> Self {
        Self {
            weights,
            last_snapshot: FatigueSnapshot::default(),
            last_score: 0,
            last_horizon: 0,
        }
    }
    
    pub fn update(&mut self, snapshot: FatigueSnapshot) {
        self.last_snapshot = snapshot;
        self.last_score = self.compute_score();
    }
    
    pub fn score(&self) -> u8 { self.last_score }
    
    fn compute_score(&self) -> u8 {
        let total_weight = self.weights.total();
        if total_weight == 0 { return 0; }
        
        let raw = self.candidates_factor() * self.weights.uncrystallized_candidates as u32
                + self.pressure_factor() * self.weights.experience_pressure as u32
                + self.proposals_factor() * self.weights.pending_heavy_proposals as u32
                + self.horizon_factor() * self.weights.causal_horizon_growth_rate as u32;
        
        ((raw / total_weight).min(255)) as u8
    }
    
    fn candidates_factor(&self) -> u32 {
        // 0..=255 — насколько "плохо" с накопленными кандидатами
        // Дефолт: каждые 10 кандидатов = +25 очков, потолок 255
        (self.last_snapshot.uncrystallized_candidates * 25 / 10).min(255)
    }
    
    fn pressure_factor(&self) -> u32 {
        // Доля заполнения EXPERIENCE
        if self.last_snapshot.experience_capacity == 0 { return 0; }
        (self.last_snapshot.experience_token_count * 255 
         / self.last_snapshot.experience_capacity).min(255)
    }
    
    fn proposals_factor(&self) -> u32 {
        // Каждое тяжёлое предложение = +30 очков
        (self.last_snapshot.pending_heavy_proposals * 30).min(255)
    }
    
    fn horizon_factor(&self) -> u32 {
        // Скорость роста горизонта без переработки
        if self.last_snapshot.ticks_since_last_check == 0 { return 0; }
        let rate = self.last_snapshot.causal_horizon_delta 
                 / (self.last_snapshot.ticks_since_last_check as u64).max(1);
        (rate * 5).min(255) as u32
    }
}
```

### 2.2 IdleTracker

```rust
// в том же fatigue.rs

#[derive(Debug, Default)]
pub struct IdleTracker {
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
    pub fn reset(&mut self) { self.consecutive_idle_ticks = 0; }
}
```

### 2.3 DreamScheduler

Создать `axiom-runtime/src/over_domain/dream_phase/scheduler.rs`:

```rust
use super::fatigue::*;
use super::state::*;

#[derive(Debug, Clone)]
pub struct DreamSchedulerConfig {
    pub min_wake_duration_ticks: u32,
    pub idle_threshold_ticks: u32,
    pub fatigue_threshold: u8,
    pub fatigue_weights: FatigueWeights,
    pub allow_explicit_command: bool,
}

impl Default for DreamSchedulerConfig {
    fn default() -> Self {
        Self {
            min_wake_duration_ticks: 1000,
            idle_threshold_ticks: 200,
            fatigue_threshold: 180,
            fatigue_weights: FatigueWeights::default(),
            allow_explicit_command: true,
        }
    }
}

pub enum SleepDecision {
    StayAwake,
    FallAsleep(SleepTrigger),
}

pub struct DreamScheduler {
    config: DreamSchedulerConfig,
    fatigue: FatigueTracker,
    idle_tracker: IdleTracker,
    last_dream_at_tick: u64,
    explicit_command_pending: Option<u16>, // source id
    pub stats: DreamSchedulerStats,
}

#[derive(Debug, Default)]
pub struct DreamSchedulerStats {
    pub idle_triggered: u64,
    pub fatigue_triggered: u64,
    pub explicit_triggered: u64,
    pub denied_by_min_wake: u64,
}

impl DreamScheduler {
    pub fn new(config: DreamSchedulerConfig) -> Self {
        let weights = config.fatigue_weights;
        Self {
            config,
            fatigue: FatigueTracker::new(weights),
            idle_tracker: IdleTracker::default(),
            last_dream_at_tick: 0,
            explicit_command_pending: None,
            stats: DreamSchedulerStats::default(),
        }
    }
    
    pub fn submit_explicit_command(&mut self, source: u16) {
        self.explicit_command_pending = Some(source);
    }
    
    pub fn on_wake_tick(&mut self, tick: u64, snapshot: FatigueSnapshot, intake_present: bool) 
        -> SleepDecision 
    {
        // 1. Min wake duration check
        if tick - self.last_dream_at_tick < self.config.min_wake_duration_ticks as u64 {
            self.stats.denied_by_min_wake += 1;
            return SleepDecision::StayAwake;
        }
        
        // 2. Update trackers
        self.fatigue.update(snapshot);
        self.idle_tracker.update(intake_present);
        
        // 3. Triggers (priority order)
        
        // 3a. Explicit command
        if self.config.allow_explicit_command {
            if let Some(source) = self.explicit_command_pending.take() {
                self.stats.explicit_triggered += 1;
                return SleepDecision::FallAsleep(SleepTrigger::ExplicitCommand { source });
            }
        }
        
        // 3b. Fatigue
        let fatigue_score = self.fatigue.score();
        if fatigue_score >= self.config.fatigue_threshold {
            self.stats.fatigue_triggered += 1;
            return SleepDecision::FallAsleep(SleepTrigger::Fatigue { fatigue_score });
        }
        
        // 3c. Idle
        let idle_ticks = self.idle_tracker.idle_ticks();
        if idle_ticks >= self.config.idle_threshold_ticks {
            self.stats.idle_triggered += 1;
            return SleepDecision::FallAsleep(SleepTrigger::Idle { idle_ticks });
        }
        
        SleepDecision::StayAwake
    }
    
    pub fn on_dream_finished(&mut self, tick: u64) {
        self.last_dream_at_tick = tick;
        self.idle_tracker.reset();
    }
    
    pub fn current_fatigue(&self) -> u8 { self.fatigue.score() }
}
```

### 2.4 Сбор FatigueSnapshot

Это **точка интеграции** с FrameWeaver, EXPERIENCE и COM. Метод `AxiomEngine::collect_fatigue_snapshot(tick)` собирает все нужные числа:

```rust
impl AxiomEngine {
    fn collect_fatigue_snapshot(&self, tick: u64) -> FatigueSnapshot {
        let frame_weaver_candidates = self.frame_weaver
            .as_ref()
            .map(|fw| fw.candidates_count() as u32)
            .unwrap_or(0);
        
        let experience_state = self.ashti.state(self.ashti.index_of(109).unwrap()).unwrap();
        let experience_token_count = experience_state.tokens.iter()
            .filter(|t| t.state != STATE_REMOVED)
            .count() as u32;
        let experience_capacity = experience_state.token_capacity as u32;
        
        let pending_heavy_proposals = self.dream_proposal_queue.len() as u32;
        
        let current_horizon = self.com.causal_horizon();
        let causal_horizon_delta = current_horizon.saturating_sub(self.last_horizon_check);
        let ticks_since = tick.saturating_sub(self.last_horizon_tick) as u32;
        
        FatigueSnapshot {
            uncrystallized_candidates: frame_weaver_candidates,
            experience_token_count,
            experience_capacity,
            pending_heavy_proposals,
            causal_horizon_delta,
            ticks_since_last_check: ticks_since,
        }
    }
}
```

**ВАЖНО:** имена методов выше (`candidates_count`, `causal_horizon`) — гипотетические. Проверь по реальной кодовой базе. Если нужно их добавить — добавь как `pub fn` соответствующих структур.

### 2.5 Тесты этапа 2

**Test 2.5.a — `idle_trigger_fires_after_threshold`:**
```
1. Scheduler с idle_threshold_ticks = 50, min_wake = 0
2. Вызвать on_wake_tick(tick=10, ..., intake_present=false) ← idle_ticks=1
3. ... 50 раз с intake_present=false ...
4. На 50-м вызове ожидать SleepDecision::FallAsleep(Idle)
```

**Test 2.5.b — `idle_resets_on_intake`:**
```
1. 30 тиков без intake
2. Один тик с intake_present=true
3. Снова 49 тиков без intake
4. Не должно быть FallAsleep (idle_ticks обнулился)
```

**Test 2.5.c — `fatigue_trigger_fires_when_threshold_exceeded`:**
```
1. Scheduler с fatigue_threshold = 100
2. Snapshot, дающий score >= 100 (например, EXPERIENCE заполнен на 50%)
3. Ожидать FallAsleep(Fatigue)
```

**Test 2.5.d — `min_wake_duration_blocks_immediate_sleep`:**
```
1. Scheduler с min_wake_duration_ticks = 1000
2. Сразу попытка засыпания (last_dream_at_tick=0, tick=500)
3. Ожидать StayAwake даже при высокой fatigue
4. stats.denied_by_min_wake == 1
```

**Test 2.5.e — `explicit_command_priority_over_others`:**
```
1. Snapshot с высокой fatigue + idle уже превышен
2. submit_explicit_command(source=999)
3. on_wake_tick → ожидать FallAsleep(ExplicitCommand { source: 999 })
4. stats.explicit_triggered == 1, остальные == 0
```

**Test 2.5.f — `priority_order_fatigue_before_idle`:**
```
Если одновременно fatigue >= threshold И idle >= threshold:
ожидать FallAsleep(Fatigue), не Idle
(см. порядок в on_wake_tick)
```

### Критерий готовности этапа 2
- [ ] FatigueTracker, IdleTracker, DreamScheduler реализованы
- [ ] `collect_fatigue_snapshot` интегрирован в AxiomEngine
- [ ] DreamScheduler создаётся в `AxiomEngine::new()` после GUARDIAN и FrameWeaver
- [ ] Тесты 2.5.a-f проходят
- [ ] DreamScheduler пока **не вызывается** в hot path — только определён и тестируется изолированно
- [ ] 1030+ существующих тестов остаются зелёными

---

## Этап 3 — DreamCycle с Stabilization, Processing, Consolidation (≈6-8 часов)

**Цель:** реализовать саму машину сна. Этап 4 (Recombination) — заглушка.

### 3.1 Структура DreamCycle

Создать `axiom-runtime/src/over_domain/dream_phase/cycle.rs`:

```rust
use crate::ucl::UclCommand;
use super::state::*;

pub struct DreamCycle {
    config: DreamCycleConfig,
    queue: Vec<DreamProposal>,
    current_cycle: Option<ActiveCycle>,
    pub stats: DreamCycleStats,
}

#[derive(Debug, Clone)]
pub struct DreamCycleConfig {
    pub max_dream_duration_ticks: u32,
    pub max_proposals_per_cycle: usize,
    pub enable_recombination: bool,    // V1.0: false
}

impl Default for DreamCycleConfig {
    fn default() -> Self {
        Self {
            max_dream_duration_ticks: 50000,
            max_proposals_per_cycle: 100,
            enable_recombination: false,
        }
    }
}

#[derive(Debug)]
struct ActiveCycle {
    started_at_tick: u64,
    started_at_event: u64,
    trigger: SleepTrigger,
    fatigue_before: u8,
    stage: CycleStage,
    processed: u32,
    approved: u32,
    vetoed: u32,
    deferred: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleStage {
    Stabilization,
    Processing,
    Recombination,
    Consolidation,
}

#[derive(Debug, Clone)]
pub struct DreamProposal {
    pub source: WeaverId,
    pub kind: DreamProposalKind,
    pub priority: u8,
    pub created_at_event: u64,
}

#[derive(Debug, Clone)]
pub enum DreamProposalKind {
    Promotion {
        anchor_id: u32,
        source_domain: u16,
        target_domain: u16,
        rule_id: String,
    },
    HeavyCrystallization {
        // V1.0: пока пустой вариант, place-holder для будущих weavers
    },
}
```

`WeaverId` — уже существует, или добавить если нет.

### 3.2 Алгоритм цикла — пошагово

DreamCycle вызывается из AxiomEngine на каждом тике в состоянии DREAMING:

```rust
impl DreamCycle {
    /// Начать новый цикл сна.
    pub fn start_cycle(&mut self, tick: u64, event: u64, 
                       trigger: SleepTrigger, fatigue: u8) {
        debug_assert!(self.current_cycle.is_none(), "Cycle already active");
        self.current_cycle = Some(ActiveCycle {
            started_at_tick: tick,
            started_at_event: event,
            trigger,
            fatigue_before: fatigue,
            stage: CycleStage::Stabilization,
            processed: 0,
            approved: 0,
            vetoed: 0,
            deferred: 0,
        });
    }
    
    /// Продвинуть цикл на один тик. Возвращает true, если цикл завершён.
    pub fn advance(&mut self, tick: u64, ashti: &AshtiCore, com: &mut COM, 
                   guardian: &mut Guardian) -> CycleAdvanceResult {
        let cycle = match &mut self.current_cycle {
            Some(c) => c,
            None => return CycleAdvanceResult::NotActive,
        };
        
        // Timeout check
        if tick - cycle.started_at_tick >= self.config.max_dream_duration_ticks as u64 {
            return CycleAdvanceResult::Timeout;
        }
        
        match cycle.stage {
            CycleStage::Stabilization => {
                // Один тик: дренируем незавершённое, переходим дальше
                self.do_stabilization(ashti, com);
                cycle.stage = CycleStage::Processing;
                CycleAdvanceResult::InProgress
            }
            CycleStage::Processing => {
                // Обрабатываем по N proposals за тик (чтобы не делать всё за один tick)
                let processed_this_tick = self.process_some(
                    ashti, com, guardian, /* batch_size */ 8
                );
                if self.queue.is_empty() || cycle.processed >= self.config.max_proposals_per_cycle as u32 {
                    cycle.stage = if self.config.enable_recombination {
                        CycleStage::Recombination
                    } else {
                        CycleStage::Consolidation
                    };
                }
                CycleAdvanceResult::InProgress
            }
            CycleStage::Recombination => {
                // V1.0: заглушка, сразу переход
                self.do_recombination_stub();
                cycle.stage = CycleStage::Consolidation;
                CycleAdvanceResult::InProgress
            }
            CycleStage::Consolidation => {
                // Финализация: запись DreamReport, обновление stats
                self.do_consolidation(tick, com);
                CycleAdvanceResult::Complete
            }
        }
    }
    
    pub fn submit(&mut self, proposal: DreamProposal) {
        self.queue.push(proposal);
    }
    
    pub fn drain_active_cycle(&mut self) -> Option<DreamReport> {
        let cycle = self.current_cycle.take()?;
        // ...построение DreamReport...
        Some(DreamReport { /* ... */ })
    }
}

pub enum CycleAdvanceResult {
    NotActive,
    InProgress,
    Complete,
    Timeout,
}
```

### 3.3 Stabilization — что именно дренировать

**Stabilization этап** короткий (1 тик в V1.0, в спеке упомянуто 10-50 — это для будущего, когда там будет реальная работа). В V1.0:

```rust
fn do_stabilization(&mut self, ashti: &AshtiCore, com: &mut COM) {
    // Снимок состояния перед сном — для DreamReport
    // Никаких UCL-команд здесь не генерируется в V1.0.
    // В будущих версиях здесь будет дренирование незавершённых FALLING_ASLEEP операций.
}
```

### 3.4 Processing — обработка очереди

Сортировка по приоритету (стабильная — для воспроизводимости):

```rust
fn process_some(&mut self, ashti: &AshtiCore, com: &mut COM, guardian: &mut Guardian,
                batch_size: usize) -> u32 {
    // Сортировка по убыванию priority (один раз в начале Processing)
    // ВАЖНО: пересортировывать каждый тик дорого. Делаем один раз при переходе в Processing.
    // Это требует флага "sorted" в ActiveCycle. Опускаю детали — Sonnet решит.
    
    let cycle = self.current_cycle.as_mut().unwrap();
    let mut count = 0;
    
    while count < batch_size && !self.queue.is_empty() {
        let proposal = self.queue.pop().unwrap(); // pop() — самый высокий priority
        cycle.processed += 1;
        count += 1;
        
        match self.process_proposal(&proposal, ashti, guardian) {
            ProposalResult::Approved(commands) => {
                for cmd in commands {
                    com.submit(cmd);
                }
                cycle.approved += 1;
            }
            ProposalResult::Vetoed(reason) => {
                // log
                cycle.vetoed += 1;
            }
            ProposalResult::Deferred => {
                // Откладываем — например, если Frame в EXPERIENCE временно не доступен
                self.queue.push(proposal); // в конец очереди
                cycle.deferred += 1;
                // Защита от бесконечного цикла:
                if cycle.deferred >= 10 {
                    break; // прекращаем processing на этом тике
                }
            }
        }
    }
    
    count as u32
}
```

### 3.5 Обработка отдельного proposal

```rust
fn process_proposal(&self, proposal: &DreamProposal, ashti: &AshtiCore, 
                    guardian: &mut Guardian) -> ProposalResult {
    match &proposal.kind {
        DreamProposalKind::Promotion { anchor_id, source_domain, target_domain, .. } => {
            // 1. Восстановить Frame из EXPERIENCE (используя restore_frame_from_anchor из FW V1.1)
            let source_state = match ashti.state(ashti.index_of(*source_domain).unwrap()) {
                Some(s) => s,
                None => return ProposalResult::Vetoed("source domain unavailable".into()),
            };
            
            let restored = match crate::over_domain::weavers::frame::restore_frame_from_anchor(
                *anchor_id, source_state
            ) {
                Ok(r) => r,
                Err(e) => return ProposalResult::Vetoed(format!("restore failed: {:?}", e)),
            };
            
            // 2. Построить UCL-команды для копирования в SUTRA
            let commands = build_promotion_commands(&restored, *target_domain);
            
            // 3. GUARDIAN проверит каждую команду на этапе com.submit().
            //    Здесь только проверяем: имеем ли мы права на target_domain в текущем состоянии.
            //    Запись в SUTRA в DREAMING разрешена (см. раздел 2.3 спеки).
            
            ProposalResult::Approved(commands)
        }
        DreamProposalKind::HeavyCrystallization { .. } => {
            // V1.0: пока никто не использует. Заглушка.
            ProposalResult::Vetoed("HeavyCrystallization not implemented in V1.0".into())
        }
    }
}

enum ProposalResult {
    Approved(Vec<UclCommand>),
    Vetoed(String),
    Deferred,
}
```

### 3.6 Recombination — заглушка

```rust
fn do_recombination_stub(&mut self) {
    // V1.0: реализация отложена, см. раздел 4.4.1 спецификации DREAM_Phase_V1_0.md
    // и документ deferred/DreamPhase_V2_plus.md
}
```

### 3.7 Consolidation и DreamReport

```rust
fn do_consolidation(&mut self, tick: u64, com: &mut COM) {
    let cycle = self.current_cycle.as_ref().unwrap();
    let report = DreamReport {
        started_at_event: cycle.started_at_event,
        ended_at_event: com.current_event(),
        duration_ticks: (tick - cycle.started_at_tick) as u32,
        trigger: cycle.trigger,
        wake_reason: WakeReason::CycleComplete,
        proposals_processed: cycle.processed,
        proposals_approved: cycle.approved,
        proposals_vetoed: cycle.vetoed,
        proposals_deferred: cycle.deferred,
        promotions_applied: cycle.approved, // в V1.0 единственный тип
        heavy_crystallizations_applied: 0,
        fatigue_before: cycle.fatigue_before,
        fatigue_after: 0, // будет обновлено позже после пробуждения
    };
    
    self.stats.record_completed(&report);
    
    // Запись DreamReport как токена в EXPERIENCE
    let report_token_cmd = build_dream_report_token(&report);
    com.submit(report_token_cmd);
}
```

### 3.8 DreamReport как токен

В `axiom-core/src/token.rs` добавить:

```rust
pub const TOKEN_FLAG_DREAM_REPORT: u16 = 0x0040;
```

Функция построения:

```rust
fn build_dream_report_token(report: &DreamReport) -> UclCommand {
    // Кодируем поля DreamReport в payload токена.
    // Точный формат — на усмотрение Sonnet, но должен быть детерминированным
    // и обратимым (отдельная функция parse_dream_report_from_token).
    
    UclCommand::InjectToken {
        target_domain: 109,  // EXPERIENCE
        type_flags: TOKEN_FLAG_DREAM_REPORT,
        position: [/* timestamped */],
        // ... остальные поля Token V5.2 ...
        state: STATE_ACTIVE,
        // payload содержит сериализованный DreamReport
    }
}
```

**ВАЖНО:** Token V5.2 ограничен 64 байтами. DreamReport может не поместиться целиком. Решение для V1.0:

- Hot-полей хватит на токен: trigger (1 byte enum), wake_reason (1 byte enum), duration_ticks (4 bytes), processed/approved/vetoed (3×4=12 bytes), fatigue before/after (2 bytes) = 20 bytes.
- Полный DreamReport в JSON/binary хранить в **отдельном append-only файле** `dreams.log` (через axiom-persist). Token несёт только сводку и offset/event_id для поиска полной записи.

Если это слишком сложно для V1.0 — упрощение: **в V1.0 храним только сводку в токене**, полный лог — в V1.1.

### 3.9 Тесты этапа 3

**Test 3.9.a — `cycle_starts_in_stabilization`:**
```
DreamCycle::start_cycle → проверить current_cycle.stage == Stabilization
```

**Test 3.9.b — `stages_advance_correctly`:**
```
1. start_cycle, очередь пуста
2. advance() → stage=Processing
3. advance() → stage=Consolidation (т.к. enable_recombination=false и очередь пуста)
4. advance() → CycleAdvanceResult::Complete
```

**Test 3.9.c — `processing_handles_promotion_proposal`:**
```
1. Подготовить EXPERIENCE с Frame-анкером (использовать утилиты из FW тестов)
2. Submit DreamProposal { Promotion { anchor_id, source=109, target=100, rule_id } }
3. start_cycle, advance до Processing
4. advance() — должен обработать proposal
5. Проверить что в COM submitted UCL-команды для копирования Frame в SUTRA
6. cycle.approved == 1
```

**Test 3.9.d — `processing_vetoes_unrestorable_anchor`:**
```
1. Submit Promotion с несуществующим anchor_id
2. Цикл завершается, cycle.vetoed == 1, в COM ничего не submitted
```

**Test 3.9.e — `priority_ordering_in_processing`:**
```
1. Submit три proposals с priority 50, 200, 100
2. Прогнать processing
3. Проверить порядок обработки: 200 → 100 → 50
   (Можно через моки или проверкой порядка submitted команд)
```

**Test 3.9.f — `timeout_aborts_cycle`:**
```
1. config.max_dream_duration_ticks = 100
2. start_cycle на тике 0
3. advance(tick=99) → InProgress
4. advance(tick=100) → Timeout
5. drain_active_cycle вернёт DreamReport с wake_reason=Timeout
```

**Test 3.9.g — `dream_report_token_appears_in_experience`:**
```
1. Полный цикл: start_cycle → advance до Complete
2. После Consolidation проверить:
   - В COM submitted команда InjectToken с TOKEN_FLAG_DREAM_REPORT
   - target_domain == 109
```

**Test 3.9.h — `deferred_proposals_dont_loop_forever`:**
```
1. Submit proposal, который всегда возвращает Deferred (через мок)
2. Прогнать processing
3. После 10 итераций deferred — processing прерывается
```

### Критерий готовности этапа 3
- [ ] DreamCycle, ActiveCycle, DreamProposal реализованы
- [ ] Stabilization — пустая (V1.0)
- [ ] Processing — работает с Promotion proposal через restore_frame_from_anchor
- [ ] Recombination — заглушка с комментарием
- [ ] Consolidation — пишет DreamReport-токен
- [ ] TOKEN_FLAG_DREAM_REPORT добавлен
- [ ] DreamCycle пока **не интегрирован** в hot path — определён и тестируется изолированно
- [ ] Тесты 3.9.a-h проходят
- [ ] Существующие тесты остаются зелёными

---

## Этап 4 — интеграция в AxiomEngine (≈4-6 часов)

**Цель:** связать DreamScheduler + DreamCycle с реальным потоком тиков. Здесь
система **впервые начинает спать**.

### 4.1 Машина состояний в AxiomEngine

В `AxiomEngine::tick()`:

```rust
pub fn tick(&mut self) -> ProcessingResult {
    let current_tick = self.current_tick();
    
    match self.dream_phase_state {
        DreamPhaseState::Wake => {
            self.tick_wake(current_tick)
        }
        DreamPhaseState::FallingAsleep => {
            self.tick_falling_asleep(current_tick)
        }
        DreamPhaseState::Dreaming => {
            self.tick_dreaming(current_tick)
        }
        DreamPhaseState::Waking => {
            self.tick_waking(current_tick)
        }
    }
}
```

### 4.2 tick_wake — обычный тик + проверка триггеров сна

```rust
fn tick_wake(&mut self, tick: u64) -> ProcessingResult {
    // 1. Обычная обработка (как раньше)
    let result = self.process_normal_tick();
    
    // 2. Проверка, не пора ли засыпать
    let snapshot = self.collect_fatigue_snapshot(tick);
    let intake_present = self.gateway.intake_in_last_tick(); // или эквивалент
    
    let decision = self.dream_scheduler.on_wake_tick(tick, snapshot, intake_present);
    
    if let SleepDecision::FallAsleep(trigger) = decision {
        self.transition_to_falling_asleep(tick, trigger);
    }
    
    result
}

fn transition_to_falling_asleep(&mut self, tick: u64, trigger: SleepTrigger) {
    let fatigue = self.dream_scheduler.current_fatigue();
    self.dream_phase_state = DreamPhaseState::FallingAsleep;
    self.falling_asleep_started_at = tick;
    self.falling_asleep_trigger = Some(trigger);
    self.falling_asleep_fatigue = fatigue;
    
    self.com.submit_event(DreamPhaseEvent::WakeToFallingAsleep { trigger, fatigue });
    self.dream_phase_stats.total_sleeps += 1;
}
```

### 4.3 tick_falling_asleep — короткая фаза перехода

В V1.0 эта фаза тривиальная — продолжается ровно 1 тик (можно увеличить позже):

```rust
fn tick_falling_asleep(&mut self, tick: u64) -> ProcessingResult {
    // 1. Финализация горячего пути: ASHTI и MAYA ещё работают
    self.ashti.tick();
    
    // 2. Перевод в Dreaming: запустить DreamCycle
    let trigger = self.falling_asleep_trigger.unwrap();
    let event = self.com.current_event();
    
    // Перед стартом цикла — собрать proposals от Weavers
    self.collect_dream_proposals(tick);
    
    self.dream_cycle.start_cycle(tick, event, trigger, self.falling_asleep_fatigue);
    
    self.dream_phase_state = DreamPhaseState::Dreaming;
    self.com.submit_event(DreamPhaseEvent::FallingAsleepToDreaming { 
        drained_operations: 0  // V1.0: пока не дренируем ничего особого
    });
    
    ProcessingResult::default()
}

fn collect_dream_proposals(&mut self, tick: u64) {
    // Собираем proposals от каждого Weaver через trait-метод dream_propose
    // V1.0: только FrameWeaver
    if let Some(fw) = &self.frame_weaver {
        let proposals = fw.dream_propose(&self.ashti, tick);
        for p in proposals {
            self.dream_cycle.submit(p);
        }
    }
}
```

### 4.4 tick_dreaming — основной цикл сна

```rust
fn tick_dreaming(&mut self, tick: u64) -> ProcessingResult {
    // 1. Проверка priority interrupt
    if self.has_critical_intake_pending() {
        self.transition_to_waking(tick, WakeReason::CriticalSignal { 
            source: 0  // TODO: реальный source из priority команды
        });
        return ProcessingResult::default();
    }
    
    // 2. Reduced heartbeat: только DREAM(107) и EXPERIENCE(109) тикают физикой
    self.tick_dream_domain();      // DREAM(107).tick()
    self.tick_experience_domain(); // EXPERIENCE(109).tick() — для естественного старения
    
    // 3. GUARDIAN продолжает работать
    self.guardian.scan_all(&self.ashti);
    
    // 4. Продвинуть DreamCycle
    let advance_result = self.dream_cycle.advance(
        tick, &self.ashti, &mut self.com, &mut self.guardian
    );
    
    match advance_result {
        CycleAdvanceResult::InProgress => ProcessingResult::default(),
        CycleAdvanceResult::Complete => {
            self.transition_to_waking(tick, WakeReason::CycleComplete);
            ProcessingResult::default()
        }
        CycleAdvanceResult::Timeout => {
            self.transition_to_waking(tick, WakeReason::Timeout { 
                max_dream_duration: self.dream_cycle_config.max_dream_duration_ticks 
            });
            ProcessingResult::default()
        }
        CycleAdvanceResult::NotActive => {
            // Не должно случиться — но защитимся
            self.transition_to_waking(tick, WakeReason::CycleComplete);
            ProcessingResult::default()
        }
    }
}

fn transition_to_waking(&mut self, tick: u64, reason: WakeReason) {
    let report = self.dream_cycle.drain_active_cycle();
    
    self.dream_phase_state = DreamPhaseState::Waking;
    self.waking_started_at = tick;
    
    self.com.submit_event(DreamPhaseEvent::DreamingToWaking {
        cycle_complete: matches!(reason, WakeReason::CycleComplete),
        reason,
    });
    
    if matches!(reason, WakeReason::CycleComplete | WakeReason::Timeout { .. }) == false {
        self.dream_phase_stats.interrupted_dreams += 1;
    }
    
    if let Some(report) = report {
        self.last_dream_report = Some(report);
    }
}
```

### 4.5 tick_waking — короткая фаза восстановления

```rust
fn tick_waking(&mut self, tick: u64) -> ProcessingResult {
    // 1. Восстанавливаем все домены
    self.ashti.tick();
    
    // 2. Размораживаем gateway
    while let Some(buffered_cmd) = self.pending_priority_intakes.pop_front() {
        self.gateway.process_command(buffered_cmd);
    }
    
    // 3. После 1 тика возвращаемся в Wake
    // (в V2.0 можно сделать постепенный ramp-up на 5-20 тиков)
    self.dream_scheduler.on_dream_finished(tick);
    self.dream_phase_state = DreamPhaseState::Wake;
    
    self.com.submit_event(DreamPhaseEvent::WakingToWake { 
        resumed_at_event: self.com.current_event() 
    });
    
    ProcessingResult::default()
}
```

### 4.6 Buffer-логика для intake в DREAMING

Gateway нужно научить различать обработку команд в зависимости от состояния системы:

```rust
impl Gateway {
    pub fn submit_command(&mut self, cmd: GatewayCommand, current_state: DreamPhaseState) {
        match current_state {
            DreamPhaseState::Wake | DreamPhaseState::FallingAsleep => {
                self.process_command(cmd);
            }
            DreamPhaseState::Dreaming => {
                match cmd.priority {
                    GatewayPriority::Critical | GatewayPriority::Emergency => {
                        // Помечаем как pending priority — система проснётся
                        self.pending_priority_intakes.push_back(cmd);
                    }
                    GatewayPriority::Normal => {
                        // Просто буферизуем
                        self.normal_intake_buffer.push_back(cmd);
                    }
                }
            }
            DreamPhaseState::Waking => {
                // Тут уже размораживаемся — буферизуем нормально
                self.normal_intake_buffer.push_back(cmd);
            }
        }
    }
}
```

### 4.7 Двухпутевая кристаллизация для FrameWeaver

В FrameWeaver V1.1 сейчас в `on_tick` есть вызов `check_promotion`. Нужно его **убрать**: промоция теперь только через DREAM-фазу.

В `frame.rs` (axiom-runtime/src/over_domain/weavers/frame.rs):

```rust
impl OverDomainComponent for FrameWeaver {
    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore, com: &mut COM) {
        if tick % self.config.scan_interval_ticks != 0 { return; }
        
        let maya_state = ashti.peek_state(110);
        
        // 1. Сканировать MAYA на узоры
        self.scan_internal(tick, maya_state);
        
        // 2. Кристаллизовать стабильных кандидатов в EXPERIENCE
        self.crystallize_stable_candidates(ashti, com);
        
        // 3. БЫЛО: check_promotion — теперь УБРАНО.
        //    Промоция идёт через dream_propose() во сне.
    }
}

impl Weaver for FrameWeaver {
    // ... существующие методы ...
    
    fn dream_propose(&self, ashti: &AshtiCore, tick: u64) -> Vec<DreamProposal> {
        let experience_state = match ashti.state(ashti.index_of(109).unwrap()) {
            Some(s) => s,
            None => return Vec::new(),
        };
        
        let mut proposals = Vec::new();
        
        for token in &experience_state.tokens {
            if token.type_flags & TOKEN_FLAG_FRAME_ANCHOR == 0 { continue; }
            if token.state == STATE_REMOVED { continue; }
            
            // Применить promotion_rules (см. FrameWeaver V1.1 раздел 5.4)
            for rule in &self.config.promotion_rules {
                if self.frame_qualifies_for_promotion(token, rule, tick, experience_state) {
                    proposals.push(DreamProposal {
                        source: WeaverId::Frame,
                        kind: DreamProposalKind::Promotion {
                            anchor_id: token.sutra_id,
                            source_domain: 109,
                            target_domain: 100,
                            rule_id: rule.id.clone(),
                        },
                        priority: rule.priority.unwrap_or(100),
                        created_at_event: 0, // заполнится при submit
                    });
                    break; // одна промоция на frame за вызов
                }
            }
        }
        
        proposals
    }
}
```

**ВАЖНО:** это изменение FrameWeaver V1.1. После реализации — нужно создать
`docs/specs/FrameWeaver_V1_2.md` (минимальная ревизия) и пометить V1.1 как
superseded. Это часть этапа 6 (документация).

### 4.8 Тесты этапа 4

**Test 4.8.a — `engine_falls_asleep_when_idle`:**
```
1. Создать AxiomEngine, idle_threshold_ticks=10
2. Прогнать 12 тиков без gateway-входа
3. Проверить: dream_phase_state == Dreaming (или FallingAsleep на промежуточном тике)
4. dream_phase_stats.total_sleeps == 1
```

**Test 4.8.b — `frame_promotion_happens_only_in_dream`:**
```
1. Создать Frame в EXPERIENCE, состарить чтобы промоция была вызвана
2. Прогнать 100 тиков в WAKE — Frame в SUTRA НЕ должен появиться
3. Заставить заснуть (force-sleep команда)
4. После пробуждения — проверить, что Frame появился в SUTRA
   с TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE
```

**Test 4.8.c — `critical_signal_wakes_system`:**
```
1. Заставить заснуть (idle trigger)
2. В состоянии Dreaming подать GatewayCommand с priority=Critical
3. На следующем тике система должна перейти в Waking
4. После Waking — Wake, и команда обработана
5. dream_phase_stats.interrupted_dreams == 1
```

**Test 4.8.d — `normal_command_buffered_during_sleep`:**
```
1. Заснуть
2. Подать Normal команду
3. Система не просыпается
4. После естественного завершения цикла — команда обработана в Waking
```

**Test 4.8.e — `dream_report_token_persists_in_experience`:**
```
1. Полный цикл сна
2. После Wake — проверить, что в EXPERIENCE появился токен 
   с TOKEN_FLAG_DREAM_REPORT
3. Расшифровать payload — найти duration_ticks > 0
```

**Test 4.8.f — `multiple_sleep_cycles`:**
```
1. Прогнать систему 10000 тиков с генерацией fatigue
2. Должно произойти несколько снов (≥2)
3. dream_phase_stats.total_sleeps >= 2
4. min_wake_duration_ticks соблюдается между ними
```

**Test 4.8.g — `hot_path_in_wake_unchanged_significantly`:**
```
Бенчмарк: tick в WAKE с FrameWeaver и DreamScheduler
ожидать ≤ 280 ns/tick (текущие 238 ns + не более 50 ns на DreamScheduler)
```

### Критерий готовности этапа 4
- [ ] Все 4 состояния реализованы и переходят между собой
- [ ] FrameWeaver больше не делает промоцию в `on_tick`
- [ ] FrameWeaver реализует `dream_propose()`
- [ ] Promotion реально работает только в DREAMING
- [ ] Critical priority пробуждает, Normal — буферизуется
- [ ] DreamReport сохраняется в EXPERIENCE
- [ ] Тесты 4.8.a-g проходят
- [ ] Hot path в WAKE: ≤ 280 ns/tick
- [ ] Все существующие тесты остаются зелёными

---

## Этап 5 — GUARDIAN integration и safety (≈2-3 часа)

**Цель:** GUARDIAN понимает состояния и enforce-ит инварианты.

### 5.1 Состояние-зависимые проверки

В `Guardian::validate_command()` добавить проверку состояния системы:

```rust
impl Guardian {
    pub fn validate_command(&self, cmd: &UclCommand, state: DreamPhaseState) 
        -> ValidationResult 
    {
        // Существующие проверки (CODEX, GENOME)...
        
        // Новая проверка: запись в SUTRA только в DREAMING
        if let UclCommand::InjectToken { target_domain, .. } = cmd {
            if *target_domain == 100 && state != DreamPhaseState::Dreaming {
                return ValidationResult::Veto {
                    reason: "SUTRA write requires DREAMING state".into(),
                };
            }
        }
        
        // Аналогично для CreateConnection в SUTRA
        if let UclCommand::CreateConnection { domain_id, .. } = cmd {
            if *domain_id == 100 && state != DreamPhaseState::Dreaming {
                return ValidationResult::Veto {
                    reason: "SUTRA connection requires DREAMING state".into(),
                };
            }
        }
        
        ValidationResult::Allow
    }
}
```

### 5.2 Передача состояния в COM submit

В точке, где `com.submit(cmd)` валидируется, нужно передать текущее dream_phase_state:

```rust
impl COM {
    pub fn submit(&mut self, cmd: UclCommand, state: DreamPhaseState) -> Result<...> {
        let validation = self.guardian.validate_command(&cmd, state);
        match validation {
            ValidationResult::Allow => self.execute(cmd),
            ValidationResult::Veto { reason } => Err(...),
        }
    }
}
```

Это **подразумевает изменение сигнатуры COM::submit** — нужно проверить все места вызова и передать туда state из AxiomEngine.

### 5.3 Тесты этапа 5

**Test 5.3.a — `guardian_blocks_sutra_write_in_wake`:**
```
1. В состоянии Wake submit InjectToken { target_domain: 100, ... }
2. Получить Veto с понятным reason
```

**Test 5.3.b — `guardian_allows_sutra_write_in_dreaming`:**
```
1. В состоянии Dreaming submit InjectToken { target_domain: 100, ... }
2. Команда проходит
```

**Test 5.3.c — `existing_guardian_checks_still_work`:**
```
Регрессия: все существующие GUARDIAN-тесты проходят с новой сигнатурой
```

### Критерий готовности этапа 5
- [ ] Guardian.validate_command принимает DreamPhaseState
- [ ] SUTRA write/connection в WAKE отклоняется
- [ ] SUTRA write/connection в DREAMING разрешается
- [ ] Все вызовы com.submit обновлены
- [ ] Тесты 5.3.a-c проходят
- [ ] Существующие тесты остаются зелёными

---

## Этап 6 — CLI команды и наблюдаемость (≈2-3 часа)

**Цель:** возможность управлять сном вручную и видеть статистику.

### 6.1 CLI команды

В CLI добавить три команды:

**`dream-stats`** — показывает текущее состояние и статистику:
```
> dream-stats
Current state: Wake
Current fatigue: 87/255 (34%)
Idle ticks: 23
Total sleeps: 4
Total dream ticks: 12450
Avg dream duration: 3112 ticks
By trigger: Idle=2, Fatigue=2, Explicit=0
By wake reason: CycleComplete=4, Timeout=0, Critical=0
Last dream report: 1234 ticks ago, processed 7 proposals (5 approved, 2 vetoed)
```

**`force-sleep`** — отправляет explicit command в DreamScheduler:
```
> force-sleep
Submitted explicit sleep command. System will fall asleep on next tick.
```

**`wake-up`** — отправляет priority Critical команду через Gateway:
```
> wake-up
Sending critical wake signal...
System state: Dreaming → Waking
```

### 6.2 BroadcastSnapshot

В `BroadcastSnapshot` (feature `adapters`) добавить:

```rust
pub struct BroadcastSnapshot {
    // ... существующие поля
    pub dream_phase: Option<DreamPhaseSnapshot>,
}

pub struct DreamPhaseSnapshot {
    pub state: DreamPhaseState,
    pub current_fatigue: u8,
    pub idle_ticks: u32,
    pub stats: DreamPhaseStats,
    pub current_cycle: Option<ActiveCycleSnapshot>,
}

pub struct ActiveCycleSnapshot {
    pub stage: CycleStage,
    pub started_at_event: u64,
    pub queue_size: usize,
    pub processed: u32,
}
```

### 6.3 Тесты этапа 6

**Test 6.3.a — `cli_dream_stats_returns_valid_data`:**
```
Вызвать команду, проверить что в выводе есть все ключевые поля
```

**Test 6.3.b — `cli_force_sleep_triggers_sleep`:**
```
1. Команда force-sleep
2. Прогнать 2 тика
3. dream_phase_state in {FallingAsleep, Dreaming}
```

**Test 6.3.c — `cli_wake_up_triggers_wake`:**
```
1. Заснуть (idle)
2. Команда wake-up
3. На следующем тике dream_phase_state == Waking
```

**Test 6.3.d — `broadcast_snapshot_includes_dream_phase`:**
```
Создать snapshot, проверить наличие поля dream_phase
```

### Критерий готовности этапа 6
- [ ] Три CLI команды работают: dream-stats, force-sleep, wake-up
- [ ] BroadcastSnapshot включает DreamPhaseSnapshot
- [ ] Тесты 6.3.a-d проходят

---

## Этап 7 — финальная валидация и документация (≈2 часа)

**Цель:** убедиться, что V1.0 целостен, документация обновлена.

### 7.1 Smoke test end-to-end

Создать integration test `crates/axiom-runtime/tests/dream_phase_smoke.rs`:

```
sleep_cycle_full_smoke():
1. Запустить AxiomEngine со специальной конфигурацией:
   - low promotion thresholds (чтобы промоция реально сработала)
   - low fatigue threshold
2. Создать в EXPERIENCE 5 Frame с разной устойчивостью
3. Прогнать 5000 тиков с эпизодическим интейком, симулирующим idle
4. Проверки:
   - Произошло ≥1 засыпания
   - Хотя бы 1 Frame промоутирован в SUTRA
   - В EXPERIENCE появился DreamReport-токен
   - Hot path tick в Wake: ≤ 280 ns
   - В состоянии Dreaming: tick дороже (это ОК)
   - Никаких ошибок и паник
   - Все события в COM присутствуют (Wake→FallingAsleep→Dreaming→Waking→Wake)
```

### 7.2 Полный прогон тестов и бенчмарков

```bash
cargo test --workspace --all-features
cargo bench --bench frameweaver_overhead
cargo bench --bench hot_path_regression
cargo bench --bench dream_phase_overhead   # новый
```

### 7.3 Обновление документации

**Создать FrameWeaver V1.2:**
- Скопировать `FrameWeaver_V1_1.md` в `FrameWeaver_V1_2.md`
- В заголовке: `**Версия:** 1.2.0`, `**Дата:** 2026-04-XX`
- Добавить раздел 0 "Изменения относительно V1.1":
  - Промоция перенесена из `on_tick` в `dream_propose`
  - Раздел 5.4 (правила промоции) теперь срабатывает только в DREAM-фазе
  - Реализация раздела 4.6 (реактивация) не изменилась
- Пометить V1.1 как Superseded

**Обновить Over-Domain Layer V1.1 → V1.2:**
- Добавить раздел про DreamPhase как третью категорию (наряду с Guardians и Weavers)
- Расширить trait Weaver методом `dream_propose`
- Описать связь с состояниями системы

**Обновить BLUEPRINT.md:**
- Добавить раздел "DREAM Phase" в секцию Over-Domain Layer
- Кратко описать четыре состояния и потоки

### 7.4 Заполнить errata

В `docs/specs/erratas/DREAM_Phase_V1_0_errata.md` записать всё, что было замечено
в реализации:
- Расхождения между спекой и кодом
- Открытые вопросы, получившие ответ в практике
- Идеи для V1.1 patch-а

### Критерий готовности этапа 7
- [ ] Smoke test проходит
- [ ] Все тесты зелёные
- [ ] Все бенчмарки в норме
- [ ] FrameWeaver V1.2 создан
- [ ] Over-Domain Layer V1.2 создан
- [ ] BLUEPRINT обновлён
- [ ] Errata заполнена

---

## Резюме плана

| Этап | Что делаем                                                           | Время    | Блокер для следующего |
|------|----------------------------------------------------------------------|----------|------------------------|
| 0    | Гигиена документов                                                   | 30 мин   | нет                    |
| 1    | Состояния + Gateway priority                                         | 3 ч      | нет                    |
| 2    | DreamScheduler + FatigueTracker (изолированно)                       | 4-6 ч    | этап 1                 |
| 3    | DreamCycle + этапы (изолированно)                                    | 6-8 ч    | этап 1 (для типов)     |
| 4    | Интеграция в AxiomEngine + двухпутевой FrameWeaver                   | 4-6 ч    | этапы 2 и 3            |
| 5    | GUARDIAN state-aware                                                 | 2-3 ч    | этап 4                 |
| 6    | CLI команды и BroadcastSnapshot                                      | 2-3 ч    | этап 4                 |
| 7    | Smoke test + документация (FW V1.2, OverDomain V1.2)                 | 2 ч      | все предыдущие         |

**Итого:** 23-31 час. Делать строго по порядку, после каждого этапа — отчёт chrnv.

---

## Что строго НЕ делаем

- Не реализуем Recombination (только заглушка)
- Не реализуем Curiosity impulses
- Не реализуем Skill condensation
- Не делаем активное чтение DreamReport-токенов
- Не делаем адаптивные параметры DreamScheduler
- Не пытаемся оптимизировать Hot path в Dreaming (там высокая стоимость допустима)
- Не вводим уровни сна (REM/non-REM)
- Не трогаем FractalChain
- Не делаем межсистемный обмен опытом во сне

Если в процессе обнаружится, что что-то из этого **критично** для работы V1.0 — занести в errata и обсудить с chrnv до начала реализации.

---

## Обнаруженные расхождения

При реализации Sonnet может обнаружить, что какие-то имена методов или поля,
упомянутые в этом плане (например, `ashti.peek_state()`, `gateway.intake_in_last_tick()`,
`com.causal_horizon()`), существуют под другими именами или не существуют вовсе.

**Действие:** не пытаться угадать. Зафиксировать в errata раздел "Naming
discrepancies", затем решить с chrnv: переименовать в коде или адаптировать план.

Имена в спеке и плане — это **наша договорённость**, не реальный API.
Реальный API в коде. При расхождении побеждает код, но фиксируется в errata.
