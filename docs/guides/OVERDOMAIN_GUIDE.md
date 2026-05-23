# Axiom — Over-Domain Layer Guide

**Дата:** 2026-05-23  
**Статус:** живой документ, обновляется после каждого значимого OBS-прогона

Это руководство описывает наддоменный пласт: что входит, как взаимодействует, какие параметры откалиброваны и что ещё не сделано.

---

## 1. Зачем нужен over-domain

ASHTI/MAYA/EXPERIENCE — домены данных. Over-domain — **слой смысла поверх них**: он не хранит токены, он их интерпретирует, оценивает и направляет.

```
WAKE tick:
  InjectToken → SUTRA → MAYA
                          ↓
            AxialEvaluator (t%5)   — оценить октант Frame
                          ↓
            ContextRecognizer (t%7) — доминирующая подсистема, профиль
                          ↓
            NeuralAdvisor (t%11)   — глубина, emergent, советы
                          ↓
            Arbiter (t%13)         — исполнить advisory (apply depth, notify)
                          ↓
            FrameWeaver (t%scan_interval=20) — кристаллизовать Frame

SLEEP:
  DreamCycle → proposals → Frame реактивация в MAYA
             → apply_dream_depth_update → SutraDepthStore
```

---

## 2. Компоненты: что делает каждый

### 2.1 AxialEvaluator — `over_domain/axial_evaluator/`
**Тик-интервал:** 5

Оценивает каждый активный Frame по трём философским осям: Apollo/Dionysus (X), Eros/Thanatos (Y), Will/Nothing (Z). Результат — `AxialEvaluation` с октантом и возможным конфликтом.

- Хранит историю в **AxialStore** (снапшот синкается в CR и NA)
- Конфликт = analytic_octant ≠ synthetic_octant (E2 в OBS-01 errata, conflict_rate baseline 0% после фикса)
- Читает позицию Frame из EXPERIENCE, сверяет с опорными полюсами из AnchorSet

**Ключевые файлы:** `mod.rs`, `axial_bridge.rs`  
**Состояние:** работает, conflict_rate 0%

---

### 2.2 ContextRecognizer (CR) — `over_domain/context_recognizer/`
**Тик-интервал:** 7

Классифицирует семантический контекст: какая подсистема сейчас доминирует в MAYA. CR-V6 — текущая версия.

**Субмодули:**

| Файл | Роль |
|---|---|
| `energy.rs` | `compute_energies`: расстояние Frame → subsystem refs → энергия |
| `scanner.rs` | Сканирует MAYA-токены в регионе ScanningPlan |
| `scanning_plan.rs` | ScanningPlan: октанты → регионы MAYA для сканирования |
| `axial_bridge.rs` | `current_active_octants_for`: какие октанты активны сейчас |
| `profile.rs` | `upsert_profile`: обновить InterpretationProfile Frame |
| `learning.rs` | `apply_dream_depth_update`: рост/decay глубин в DREAM |
| `activity_trace.rs` | Кольцевые буферы активности (CR-V6 Фаза A) |
| `subsystem_fatigue.rs` | Усталость подсистем (CR-V6 Фаза B) |
| `meta_detector.rs` | Мета-подсистемы: meta_perception и др. (CR-V6 Фаза C) |
| `composite.rs` | Composite co-activations (CR-V6 Фаза D) |
| `emergent/` | Заглушка (V1: no-op) |
| `transitions.rs` | ActivityAnalyzer — детектор переключений |

**Ключевые поля CR:**
- `subsystem_refs` — позиции subsystem-примитивов (math, writing, logic, time, music, values)
- `depth_store` — SutraDepthStore: глубина каждого Frame по октантам
- `profile_store` — InterpretationProfileStore: dominant subsystem, primary_octant, веса
- `dream_activation_acc` — `HashMap<(sutra_id, Octant), u32>`: счётчик on_tick-активаций за текущий Wake-цикл, передаётся в DREAM

**Важно про depth:**
- Глубина растёт на MAX_GROWTH_PER_CYCLE=100 за DREAM-цикл
- `reactivation_count` в SutraDepthEntry инкрементируется в `apply_evidence(evidence>0)` — один раз за DREAM-цикл на октант
- При 30k тиках и idle_threshold=200/min_wake=1000 получается ~10-15 DREAM-циклов

---

### 2.3 NeuralAdvisor (NA) — `over_domain/neural_advisor/`
**Тик-интервал:** 11

Советник, работает на снапшотах CR. Синк происходит сразу после CR на каждом t%7:
```
t%7: CR.on_tick → NA.sync_profile_store + NA.sync_depth_store
t%11: NA.on_tick (читает свежие снапшоты)
```

**Реестр советников (registry):**

| Советник | Файл | Что делает |
|---|---|---|
| OctantCorrectionAdvisor | — | Корректирует октант по истории AxialStore |
| CorpusCallosumResolver | `conflict.rs` | Разрешает analytic/synthetic конфликт |
| SubsystemAttributionAdvisor | — | Атрибуция подсистемы по весам профиля |
| DepthPredictionAdvisor | `depth.rs` | Рекомендует глубину через REACT_* формулы |
| **DepthThresholdEmergentDetector** | `emergent.rs` | Детектирует emergent кандидатов |

**Emergent detection** (три условия одновременно):
```
passes_depth:         depth_per_octant[primary_octant] >= MIN_DEPTH (текущее: 1000)
passes_reactivations: reactivation_count >= MIN_REACTIVATIONS (текущее: 5)
passes_age:           frame_age_ticks >= MIN_AGE_TICKS (100)
```

Если все три — Frame попадает в `EmergentPrimitiveStore` NA как "pending".  
Approval = 0 до прихода UCL `ApproveEmergentCandidate` (ручная команда от chrnv).

**Калибровка thresholds (OBS-02, 30k тиков):**
- `MIN_DEPTH`: 8000 → 1000 (O7 avg_depth=1198)
- `MIN_REACTIVATIONS`: 30 → 5 (~10 DREAM-циклов за 30k тиков)
- Результат: 312 pending (все Frame), что ожидаемо при однородном корпусе

---

### 2.4 OverDomainArbiter — `over_domain/arbiter/`
**Тик-интервал:** 13

Исполнитель advisory-команд. Собирает `Advisory` из NA и AxialEvaluator, применяет через Arbiter.

- `ApplyDepth` advisory → `depth_store.set_promoted_depth(...)`
- `NotifyWorkstation` advisory → UCL NotifyWorkstation (→ внешний сигнал)
- Подтверждение pending-advisory через `confirm_pending`

---

### 2.5 FrameWeaver — `over_domain/weavers/`
**Тик-интервал:** scan_interval_ticks=20

Формирует Frame-анкеры из кластеров связанных SUTRA-токенов.

**Конфигурация (default):**
```
scan_interval_ticks: 20
stability_threshold: 3   — Frame кристаллизуется после 3 стабильных скан-циклов
crystallization_rules: [] — ShellProximity opt-in, не default
promotion_rules: [PromotionRule::default()]
```

**Важно:** `crystallization_rules: vec![]` оставлен пустым намеренно.  
При непустом списке `evaluate_crystallization_rules()` не фолбэчит на stability_threshold → Frames=0.

**Shell:**
- `shell_similarity: f32` в FrameCandidate — avg pairwise cosine участников
- `avg_candidate_shell_similarity()` — метод для OBS snapshot
- `shell_registry: HashMap<u32,[u8;8]>` — загружается из engine после boot

---

### 2.6 DreamPhase — `over_domain/dream_phase/`

Машина фаз: Wake → FallingAsleep → Dreaming → Waking → Wake.

**Параметры (defaults):**
```
idle_threshold:   200 тиков без intake → переход в FallingAsleep
min_wake_ticks:  1000 тиков — минимальный Wake перед следующим DREAM
```

**Цикл в engine.rs:**
```
tick_falling_asleep → tick_dreaming → DreamCycle (proposals) →
apply_dream_cycle_commands → apply_dream_depth_update → tick_waking
```

**apply_dream_depth_update** (engine.rs:1420):
```rust
let activations = context_recognizer.drain_dream_activations();
context_recognizer.apply_dream_update(&activations, &known_ids, event_id);
```
`drain_dream_activations()` сбрасывает `dream_activation_acc` (накоплено за Wake).

---

## 3. Синхронизации между компонентами

```
engine.tick_wake():
  t%5:  axial_evaluator.on_tick → AxialStore
        CR.sync_axial_store(axial)
        NA.sync_axial_store(axial)
  t%7:  CR.on_tick → profile_store, depth_store, dream_activation_acc
        NA.sync_profile_store(CR.profile_store())
        NA.sync_depth_store(CR.depth_store())
  t%11: NA.on_tick → reads snapshots, populates emergent_store, advisory results
  t%13: Arbiter.tick_with_stores(NA.advisories, AE.advisories, depth_store_mut)
  fw:   FrameWeaver.on_tick (каждые 20 тиков)

engine.apply_dream_depth_update():
  CR.drain_dream_activations() → [(sutra_id, octant, count)]
  CR.depth_store.apply_evidence(sutra_id, octant, evidence)  ← reactivation_count++
  NA.sync_depth_store происходит на следующем t%7 (свежий depth виден NA)
```

---

## 4. Хранилища опыта (axiom-experience)

| Хранилище | Тип | Кто пишет | Кто читает |
|---|---|---|---|
| SutraDepthStore | `HashMap<u32, SutraDepthEntry>` | CR (learning.rs) | NA (depth snapshot) |
| InterpretationProfileStore | `HashMap<u32, InterpretationProfile>` | CR (profile.rs) | NA (profile snapshot) |
| AxialStore | история AxialEvaluation | AxialEvaluator | CR (axial_bridge), NA |
| EmergentPrimitiveStore | Vec<EmergentPrimitive> | NA | Arbiter, report |

**SutraDepthEntry** (на Frame):
```
depth_per_octant: [u16; 8]  — глубина в каждом октанте
reactivation_count: u32      — кол-во DREAM-циклов с активностью (++ в apply_evidence)
last_settle_event: u64
```

---

## 5. Текущие пороги и константы

| Константа | Значение | Файл | Откалибровано |
|---|---|---|---|
| MAX_GROWTH_PER_CYCLE | 100 | axiom-experience/sutra_depth_store.rs | — |
| DECAY_PER_CYCLE | 5 | axiom-experience/sutra_depth_store.rs | — |
| PRIMITIVE_DEPTH | 65535 | axiom-experience | — |
| EMERGENT_CANDIDATE_MIN_DEPTH | 1000 | neural_advisor/implementations/emergent.rs | OBS-02 |
| EMERGENT_CANDIDATE_MIN_REACTIVATIONS | 5 | neural_advisor/implementations/emergent.rs | OBS-02 |
| EMERGENT_CANDIDATE_MIN_AGE_TICKS | 100 | neural_advisor/implementations/emergent.rs | — |
| CONTEXT_RECOGNIZER_TICK_INTERVAL | 7 | context_recognizer/mod.rs | — |
| AXIAL_EVALUATOR_TICK_INTERVAL | 5 | axial_evaluator/mod.rs | — |
| NEURAL_ADVISOR_TICK_INTERVAL | 11 | neural_advisor/mod.rs | — |
| ARBITER_TICK_INTERVAL | 13 | arbiter/mod.rs | — |
| FrameWeaver scan_interval | 20 | weavers/frame.rs (config default) | — |
| FrameWeaver stability_threshold | 3 | weavers/frame.rs (config default) | — |
| DreamScheduler idle_threshold | 200 | dream_phase/scheduler.rs | — |
| DreamScheduler min_wake_ticks | 1000 | dream_phase/scheduler.rs | — |

---

## 6. Что не работает / технический долг

| ID | Компонент | Описание | Приоритет |
|---|---|---|---|
| CR-TD-01 | CR | FatigueStore → axiom-experience (сейчас внутри CR) | Средний |
| CR-TD-02 | CR | TransitionGraph для directed Cascading | Низкий |
| CR-TD-03 | CR | Ethics composite (Values/Dilemmas/Morality) | Средний |
| CR-TD-04 | CR | ActivityTrace сериализация | Низкий |
| AGENT-TD-01 | TextPerceptor | FNV-1a fallback → embeddings | Высокий |
| FW-TD-01 | FrameWeaver | multi-participant routing | Средний |
| Shell-TD-01 | всё | resonance_search shell bonus (axiom-arbiter) | Средний |

---

## 7. Наблюдения по OBS-прогонам

| Прогон | Тиков | Ключевые результаты |
|---|---|---|
| OBS-01 | 3000 | Baseline: E1 energy=0, E2 conflict=100%, E3 depth_store пуст |
| OBS-05..08 | 3000 | После фиксов E1/E2/E3: dominant=Time(3), fill=16, conflict=0%, coherence=0.963 |
| OBS-02 | 30000 | 100% per-text accuracy, 312 frames, O7 avg_depth=1198, 312 emergent pending |

**Вопросы ROADMAP (OBS-02):**
1. ✓ Frame кристаллизуются: 312
2. ✓ SubsystemId корректен: 100% per-text accuracy
3. ✓ Coherence: avg 0.996
4. ✓ Conflict rate: 0.0%
5. ~ Experience traces: 15 (стабилизируются рано и не растут)
6. ~ Depth thresholds: O7=1198, порог снижен до 1000. Для отделения "глубокого" нужен неоднородный корпус

---

## 8. Добавить новый over-domain модуль: чеклист

1. Создать `src/over_domain/<name>/mod.rs`
2. Реализовать трейт `OverDomainComponent` (`name`, `module_id`, `on_boot`, `on_tick_interval`, `on_tick`, `on_shutdown`)
3. Добавить поле в `AxiomEngine` (engine.rs)
4. Инициализировать в `AxiomEngine::new()`
5. Добавить `on_boot` вызов в `AxiomEngine::boot()`
6. Добавить `on_tick` в `tick_wake()` с нужным интервалом (`t % interval == 0`)
7. Если модуль читает данные других компонентов — добавить sync вызов перед on_tick
8. Если выдаёт Advisory — зарегистрировать в Arbiter
9. Если нужен Genome-доступ — добавить Permission в `on_boot`
10. Добавить snapshot в axiom-broadcasting если нужна OBS-видимость
