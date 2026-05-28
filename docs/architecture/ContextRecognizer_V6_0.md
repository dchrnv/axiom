# AXIOM — ContextRecognizer V6.0

**Статус:** Реализовано  
**Версия:** 6.0  
**Дата:** 2026-05-27  
**Категория:** Over-Domain Mechanism  
**Crate:** `axiom-runtime` / `over_domain/context_recognizer/`  
**Опирается на:** `INVARIANTS.md`, `ContextRecognizer_V5_0.md`, `FrameWeaver_V1_3`, `DreamPhase_V1_1`, `AxialEvaluator_V3_0`

---

## 1. Что это

ContextRecognizer — над-доменный модуль, отвечающий на вопрос **"что мы сейчас понимаем и в каком режиме"**.

Сканирует MAYA, определяет какие подсистемы понимания активны, на каких **глубинах SUTRA**, в каких **октантах**, в каких **окнах**. Замечает переключения, классифицирует паттерны активности, фиксирует усталость подсистем, детектирует мета-режимы и kompozitные ko-активации. Записывает эмерджентные примитивы.

`ModuleId = 18`, `tick_interval = 7`.

### 1.1 Разделение ролей

| Модуль | Отвечает на вопрос | Природа |
|--------|-------------------|---------|
| **ContextRecognizer** | "**Что** мы понимаем?" | Горизонтальная — какая подсистема активна |
| **AxialEvaluator** | "**Как** мы это оцениваем?" | Вертикальная — по дихотомиям, определяет октант |

### 1.2 Что добавлено в V6 (сводка)

| Фаза | Компонент | Файл |
|------|-----------|------|
| 0 | SyntacticBridge — bridge_to_maya | `orchestrator.rs` |
| A | ActivityTrace / ActivityDynamics / ActivitySignature / classify | `activity_trace.rs` |
| A | ActivityAnalyzer (переименован из TransitionDetector) | `transitions.rs` |
| B | SubsystemFatigue / FatigueStore | `subsystem_fatigue.rs` |
| C | MetaDetector / MetaPrimitive | `meta_detector.rs` |
| D | CompositeSubsystemDef / COMPOSITE_DEFS / detect_composite_suspects | `composite.rs` |

---

## 2. SyntacticBridge (Phase 0)

### 2.1 Проблема

FrameWeaver кристаллизует Frame-анкеры из `0x08`-связей в MAYA domain state. Но routing (`route_token`) записывал consolidated-токен только в память — в `DomainState.connections` MAYA не попадало. Итог: Frames = 0, CR profiles = 0, ActivityDynamics без данных.

### 2.2 Решение

После каждого `orchestrator::route_token` (slow path) инжектируется в MAYA domain state 8 связей:

```
source_id  = stable_id(consolidated_position)   // одинаковый для одного текста
target_id  = stable_id(ashti_result[role])       // один на каждый ASHTI домен (role 1..8)
link_type  = 0x0800 | (role << 4)                // синтаксический тип, слой = role
flags      = FLAG_ACTIVE
```

После `stability_threshold` (по умолчанию 3) повторений одного текста — FrameWeaver видит паттерн и кристаллизует Frame-анкер в EXPERIENCE. С этого момента CR, AE, NA получают данные.

**Место реализации:** `axiom-runtime/src/orchestrator.rs` — функция `bridge_to_maya()`.

### 2.3 Ограничение V6

Stable ID из position-hash; не учитывает семантическую близость токенов. Точная топология MAYA → V7 (TransitionGraph).

---

## 3. ActivityTrace / ActivityDynamics / ActivitySignature (Phase A)

### 3.1 Три кольцевых буфера

```rust
pub const SHORT_CAP: usize = 16;   // детекция осцилляции
pub const MID_CAP:   usize = 64;   // конвергенция, дивергенция, каскад
pub const LONG_CAP:  usize = 256;  // fatigue (Phase B)

pub struct ActivityTrace {
    short: RingBuf,
    mid:   RingBuf,
    long:  RingBuf,
}
```

`push(subsystem, event_id)` — одновременно пишет во все три. `SubsystemId::Unknown` игнорируется.

Холодный старт: классификация не производится пока `fill_count < SHORT_CAP`.

### 3.2 ActivityDynamics — четыре метрики

```rust
pub struct ActivityDynamics {
    pub entropy_gradient:     f32,  // > 0 = Diverging, < 0 = Converging
    pub oscillation_score:    f32,  // доля alt-пар в short-окне
    pub cascade_score:        f32,  // доля элементов в каскадах ≥3 в mid-окне
    pub dominant_persistence: f32,  // доля наиболее частой подсистемы в mid-окне
    pub fill_count:           usize,
}
```

### 3.3 ActivitySignature — классификатор

```rust
pub enum ActivitySignature {
    Uncertain,     // cold start
    Steady,        // dominant_persistence > 0.7
    Oscillating,   // oscillation_score > 0.5 && !Steady
    Cascading,     // cascade_score > 0.4 && !Steady
    Converging,    // entropy_gradient < -0.15
    Diverging,     // entropy_gradient > 0.15
}
```

`classify(dynamics) -> Vec<ActivitySignature>` — может вернуть несколько лейблов одновременно (например Steady + Converging). Fallback: если ни один не сработал → `[Steady]`.

Приоритет проверок: Steady → Oscillating → Cascading → Converging → Diverging.

### 3.4 ActivityAnalyzer (переименован из TransitionDetector)

Лёгкий компонент: фиксирует факт смены доминирующей подсистемы.

```rust
pub struct ActivityAnalyzer {
    last_primary: SubsystemId,
    last_event:   u64,
}
// TransitionDetector — псевдоним для обратной совместимости
pub type TransitionDetector = ActivityAnalyzer;
```

Для анализа паттернов — использовать `ActivityTrace`.

---

## 4. SubsystemFatigue / FatigueStore (Phase B)

### 4.1 Структуры

```rust
pub struct SubsystemFatigue {
    pub activation_load: f32,   // текущая нагрузка, убывает при неактивности
    pub recovery_debt:   f32,   // накопленный долг, убывает медленно
}
```

```rust
pub struct FatigueStore {
    store: HashMap<SubsystemId, SubsystemFatigue>,
}
```

Живёт в `ContextRecognizer` (V6). Перенос в `axiom-experience` как отдельный `FatigueStore` — V7.

### 4.2 Константы

| Константа | Значение | Смысл |
|-----------|----------|-------|
| `MAX_ACTIVATION_LOAD` | 10.0 | equilibrium при непрерывной активности |
| `DECAY_FACTOR` | 0.90 | затухание load за один on_tick |
| `DEBT_RATE` | 0.05 | конвертация load → debt за тик |
| `DEBT_DECAY` | 0.998 | медленное затухание debt |
| `DREAM_RECOVERY` | 0.35 | `activation_load *= 0.35` при DREAM-пробуждении |

### 4.3 Алгоритм

`FatigueStore::update(dominant)` — вызывается на каждом `on_tick`:
1. Все записи: `activation_load *= DECAY_FACTOR`, `recovery_debt *= DEBT_DECAY`
2. Доминирующая подсистема: `activation_load += 1.0`, `recovery_debt += DEBT_RATE`

`effective_weight(base) = base * (1.0 - 0.5 * min(1.0, activation_load / MAX))`

При максимальной усталости — вес снижается вдвое. При `DREAM_RECOVERY` — `activation_load *= 0.35`.

`apply_to_weights(&mut HashMap<SubsystemId, u8>)` — снижает веса уставших подсистем перед классификацией.

---

## 5. MetaDetector / MetaPrimitive (Phase C)

### 5.1 Главная идея

ContextRecognizer V6 узнаёт **мета-режимы** — типы работы с подсистемами, а не сами подсистемы.

```
Мета-режим = HOW мы работаем с подсистемами

"Анализ"       — раскладываем сложное на примитивы (Math, Logic, Steady)
"Синтез"       — собираем новое (Converging)
"Рефлексия"    — система думает о собственной работе (Cascading)
"Восприятие"   — поглощение нового материала через сенсоры
```

### 5.2 Структуры

```rust
pub struct MetaPrimitive {
    pub id:         String,       // "meta_analysis", "meta_synthesis", ...
    pub meta_id:    u16,          // 0x1001..0x1007
    pub triggered_by: Vec<SubsystemActivationPattern>,
}

pub struct SubsystemActivationPattern {
    pub required_subsystems:  Vec<String>,  // имена подсистем; [] = любая
    pub forbidden_subsystems: Vec<String>,
    pub activity_signature:   Option<String>,  // None = любая
}
```

```rust
pub struct MetaDetector {
    primitives: Vec<MetaPrimitive>,
}
```

`MetaDetector::from_yaml(path)` — загрузка из `meta_primitives.yaml`.  
`MetaDetector::detect(dynamics, signatures, dominant, event_id, &mut MetaStore)` — обновляет `MetaStore`.

Confidence = доля совпавших паттернов из `triggered_by`. Только `confidence > 0` записываются в `MetaStore`.

### 5.3 MetaSubsystemId и MetaStore

```rust
pub struct MetaSubsystemId(pub u16);

// Константы в axiom-experience:
pub const META_ANALYSIS:    MetaSubsystemId = MetaSubsystemId(0x1001);
pub const META_SYNTHESIS:   MetaSubsystemId = MetaSubsystemId(0x1002);
pub const META_REFLECTION:  MetaSubsystemId = MetaSubsystemId(0x1003);
pub const META_PERCEPTION:  MetaSubsystemId = MetaSubsystemId(0x1004);
// ... до 0x1007
```

`MetaStore` — хранилище активных мета-режимов в `ContextRecognizer`. Обновляется на каждом тике.

---

## 6. CompositeSubsystems (Phase D)

### 6.1 Пять статических композитов

```rust
pub static COMPOSITE_DEFS: &[CompositeSubsystemDef] = &[
    CompositeSubsystemDef { name: "Calculus",  components: &[Mathematics, Time] },
    CompositeSubsystemDef { name: "Rhythm",    components: &[Music, Time] },
    CompositeSubsystemDef { name: "Geometry",  components: &[Mathematics, Writing] },
    CompositeSubsystemDef { name: "Narrative", components: &[Writing, Time] },
    CompositeSubsystemDef { name: "Ethics",    components: &[Logic] },  // V7: + Values, Dilemmas
];
```

### 6.2 Co-activation сигнал

```rust
pub struct CompositeActivationSuspected {
    pub name:       &'static str,
    pub confidence: f32,
}
```

`detect_composite_suspects(recent_subsystems, signatures) -> Vec<CompositeActivationSuspected>`

`confidence = (доля компонентов в recent-active set) × [1.5 если Converging, cap 1.0]`

**V6 ограничение:** упрощённая детекция по recent-active set. Directed propagation через TransitionGraph — V7.

---

## 7. SutraDepth — четвёртая ось (из V5, без изменений)

### 7.1 Что это

`SutraDepth: u16` — степень укоренённости Frame или токена в SUTRA (0..65535).

```
SutraDepth = 0        — свежее, только появилось
SutraDepth = 1000     — частая константа
SutraDepth = 10000    — глубоко укоренённая структура
SutraDepth = 65535    — примитив-якорь (максимум, не меняется)
```

### 7.2 Хранение

Отдельный storage: `crates/axiom-experience/src/sutra_depth_store.rs`.

```rust
pub struct SutraDepthEntry {
    pub sutra_id:           u32,
    pub depth_per_octant:   [u16; 8],
    pub last_settle_event:  u64,
    pub reactivation_count: u32,
}
```

`reactivation_count` — число DREAM-циклов с `evidence > 0` (исправлено в OBS-02 патче).

### 7.3 Динамика глубины

Меняется только в DREAM Phase (не на горячем пути):
- `evidence > 0` → `depth += min(evidence, MAX_GROWTH_PER_CYCLE=100)`
- `evidence == 0` → `depth = saturating_sub(depth, DECAY_PER_CYCLE=5)`
- Примитивы из yaml → 65535, не меняется
- Promoted Frame → скачок до 30000+

---

## 8. ScanningPlan (из V5, без изменений)

```rust
pub struct ScanningPlan {
    pub active_regions:    Vec<ActiveRegion>,
    pub computed_at_event: u64,
}

pub struct ActiveRegion {
    pub octant:         Octant,
    pub depth_range:    DepthRange,
    pub priority:       u8,
    pub fractal_levels: Vec<FractalLevel>,
}

pub struct DepthRange { pub min: u16, pub max: u16 }
```

ContextRecognizer сканирует только активные октанты × активные диапазоны глубин, определённые из текущего состояния AE и SutraDepthStore. Не сканирует глубину примитивов ([50000+]) если работа идёт на средних глубинах.

---

## 9. Связь с FrameWeaver

При создании нового Frame:
1. `SutraDepthStore.create_entry(frame_id)` — `depth_per_octant = [0; 8]`
2. AxialEvaluator делает первую оценку → определяет октант
3. ContextRecognizer фиксирует контекст создания

Глубина растёт постфактум в DREAM Phase на основе реактиваций.

**ActivityTrace** получает сигнал через `push()` на каждом тике когда SyntacticBridge инжектирует связи в MAYA и FrameWeaver кристаллизует Frame-анкеры.

---

## 10. Диагностические методы

### compute_raw_energies

```rust
pub fn compute_raw_energies(&self, ashti: &AshtiCore) -> HashMap<SubsystemId, u8>
```

Вычисляет снапшот энергий подсистем по токенам MAYA без привязки к ScanningPlan. Для OBS-наблюдений.

### AxiomEngine::snapshot_subsystem_energies

```rust
pub fn snapshot_subsystem_energies(&self) -> HashMap<SubsystemId, u8>
```

Читает MAYA-токены → вызывает `compute_raw_energies`. Зарезервирован для диагностики.

---

## 11. Структуры данных

### InterpretationProfile (V5, без изменений)

```rust
pub struct InterpretationProfile {
    pub frame_anchor_sutra_id: u32,
    pub weights:               HashMap<SubsystemId, u8>,
    pub primary:               SubsystemId,
    pub last_updated_event:    u64,
    pub last_context:          ContextSnapshot,
    pub frame_composition:     FrameComposition,
    pub primary_octant:        Octant,
}
```

### FrameComposition (переименован из FractalDepth в V4)

```rust
pub enum FrameComposition {
    C0_Primitive, C1_Atom, C2_Molecule,
    C3_Structure, C4_Composition, C5_Plus,
}
```

---

## 12. Что в коде

```
crates/axiom-runtime/src/over_domain/context_recognizer/
├── mod.rs             — ContextRecognizer, pub re-exports
├── activity_trace.rs  — ActivityTrace, ActivityDynamics, ActivitySignature, classify()
├── transitions.rs     — ActivityAnalyzer (бывший TransitionDetector), SubsystemTransition
├── subsystem_fatigue.rs — SubsystemFatigue, FatigueStore
├── meta_detector.rs   — MetaDetector, MetaPrimitive, SubsystemActivationPattern
├── composite.rs       — CompositeSubsystemDef, COMPOSITE_DEFS, detect_composite_suspects()
├── scanning_plan.rs   — ScanningPlan, ActiveRegion, DepthRange, FractalLevel
├── scanner.rs         — сканирование по ScanningPlan
├── energy.rs          — SubsystemEnergy, SubsystemShellRefs
├── profile.rs         — InterpretationProfile
├── depth_bridge.rs    — мост к SutraDepthStore
├── axial_bridge.rs    — мост к AxialStore
├── conflicts.rs       — SubsystemConflict
├── learning.rs        — обновление профилей + SutraDepth
├── hot_reload.rs      — hot-reload подсистем через UCL
└── emergent/          — EmergentPrimitiveStore интеграция

crates/axiom-experience/src/
├── sutra_depth_store.rs         — SutraDepthEntry, SutraDepthStore
├── interpretation_profile_store.rs
├── emergent_primitive_store.rs
└── meta_store.rs                — MetaStore, MetaSubsystemId, META_* константы

crates/axiom-runtime/src/orchestrator.rs:
    bridge_to_maya()             — SyntacticBridge (Phase 0)
```

---

## 13. Инварианты

| Правило | Значение |
|---------|----------|
| Чтение состояния | только `&AshtiCore` |
| Запись | только через UCL |
| Где живут якоря подсистем | **SUTRA (100)** |
| State якорей | **STATE_LOCKED** |
| FractalLevel значений | **5** (Symbol, Word, Phrase, Scene, Session) |
| FrameComposition значений | **6** (C0..C5_Plus) |
| SutraDepth диапазон | **0..65535** на октант |
| Где хранится SutraDepth | `axiom-experience/sutra_depth_store.rs` |
| SutraDepth не в Token | да, HARD 64 байта неприкосновенен |
| Обновление SutraDepth | только в DREAMING |
| MAX_GROWTH_PER_CYCLE | **100** |
| DECAY_PER_CYCLE | **5** |
| Примитивы из yaml | depth = 65535, не меняется |
| Promoted Frame | depth скачком до 30000+ |
| Octants | **8** |
| SHORT_CAP / MID_CAP / LONG_CAP | **16 / 64 / 256** |
| FatigueStore | живёт в CR (V6); перенос в axiom-experience — V7 |
| MetaDetector | загружается из `meta_primitives.yaml` или пустой |
| COMPOSITE_DEFS | **5** статических определений |
| Emergent primitives max | **1000** |
| NeuralAdvisor influence max | **50%** |
| Tick interval | **7** |
| ModuleId | **18** |

---

## 14. Известные ограничения V6

- **SyntacticBridge — упрощённая.** Stable ID из position-hash; семантическая близость не учитывается. → V7 (TransitionGraph)
- **MetaPrimitive yaml — вручную.** Автоматическое обнаружение мета-режимов — V7.
- **CompositeSubsystems — статические.** Динамическое создание через stable co-activation topology — V7.
- **FatigueStore — в CR.** Перенос в `axiom-experience` для независимости от CR — V7.
- **Ethics composite — неполный.** Только `Logic`; полный состав (Values, Dilemmas, Morality) — V7 после реализации этих подсистем.

---

## 15. Что в V7 и дальше

- **V7:** TransitionGraph — directed transitions between subsystems, stable co-activation topology для полной CompositeSubsystem детекции; FatigueStore → axiom-experience; SubsystemVersioning/Splitting/Merging
- **V8:** Axiogenesis through Dilemmas — долгоживущие конфликты → новые ценности в Values
- **V9:** Active NeuralAdvisor — обученные модели на всех 5 трейтах; ActivityTrace как observation sequence для HMM

---

## 16. Исправления

- **CR-TD-01** ✅ — sync_axial_store / sync_profile_store / sync_depth_store вызываются из `engine.rs` после каждого соответствующего цикла.
- **CR-TD-02** ✅ — warm start через `all_octants_in_store` при пустом активном окне.
- **CR-TD-03** ✅ — конструктор `from_anchor_set(AnchorSet)`, вызывается в `engine.rs` при инициализации.
- **OBS-02 патч** ✅ — `reactivation_count` в `SutraDepthStore` инкрементируется при `evidence > 0`. Пороги DepthThresholdEmergentDetector снижены: `MIN_DEPTH 8000→1000`, `MIN_REACTIVATIONS 30→5`.

---

## 17. История

- **V1.0**: булевы флаги
- **V2.0**: InterpretationProfile с весами, cross_links, transitions, conflicts, hot reload
- **V3.0**: координация с AxialEvaluator, Corpus Callosum, AxialHint
- **V4.0**: FractalDepth → FrameComposition, emergent primitives, NeuralAdvisor интерфейс
- **V5.0** (2026-05-15): SutraDepth как четвёртая ось (per-octant, axiom-experience storage), ScanningPlan (octant × depth_range × level), опциональный gravity depth_factor
- **V6.0** (2026-05-27): SyntacticBridge (bridge_to_maya); ActivityTrace (3-ring 16/64/256), ActivityDynamics (4 метрики), ActivitySignature classifier (6 лейблов + Uncertain), ActivityAnalyzer (renamed); SubsystemFatigue {activation_load, recovery_debt}, FatigueStore (decay=0.90, DREAM=0.35); MetaDetector + MetaPrimitive + MetaStore (5 мета-режимов из yaml); CompositeSubsystemDef × 5 + detect_composite_suspects(); compute_raw_energies() → HashMap<SubsystemId, u8>; 100% per-text subsystem accuracy (OBS-02)
