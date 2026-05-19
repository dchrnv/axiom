# AXIOM — ContextRecognizer V5.0

**Статус:** Спецификация
**Версия:** 5.0
**Дата:** 2026-05-15
**Категория:** Over-Domain Mechanism (один из пяти: FrameWeaver, GUARDIAN, DreamPhase, ContextRecognizer, AxialEvaluator)
**Crate:** `axiom-runtime` / `over_domain/context_recognizer/`
**Опирается на:** `INVARIANTS.md`, `SPACE_V6_0`, `Shell_V3_0`, `FrameWeaver_V1_3`, `DreamPhase_V1_0`, `AxialEvaluator_V1_0`

---

## 1. Что это

ContextRecognizer — над-доменный модуль, отвечающий на вопрос **"что мы сейчас понимаем и в каком режиме"**.

Сканирует MAYA, определяет какие подсистемы понимания активны, на каких **глубинах SUTRA** (новое в V5), в каких **октантах** (с учётом AxialEvaluator), в каких **окнах** (FractalLevel). Замечает переключения, разрешает конфликты, учится в DREAM Phase. Записывает эмерджентные примитивы. Использует опциональных нейронных советников.

### 1.1 Разделение ролей

| Модуль | Отвечает на вопрос | Природа |
|--------|-------------------|---------|
| **ContextRecognizer** | "**Что** мы понимаем?" | Горизонтальная — какая подсистема активна |
| **AxialEvaluator** | "**Как** мы это оцениваем?" | Вертикальная — по философским дихотомиям, определяет октант |

---

## 2. Архитектурный принцип V5

**Новое:** в SUTRA вводится **четвёртая ось — глубина (SutraDepth)**. Это не координата токена (Token остаётся 64 байта HARD), а **отдельный storage по sutra_id**, описывающий насколько Frame "укоренён" в системе.

Параллельно: **глубина измеряется per-octant**. Один и тот же Frame имеет разную глубину в разных октантах. Это создаёт **слоистую структуру каждого октанта**, где работа может быть сосредоточена на разных уровнях.

И главное: **управление per-octant per-depth-range**. Если в октанте 4 на глубине 500+ всё стабильно — нет смысла сканировать примитивы на глубине 65000+. Экономия CPU и более точная работа.

---

## 3. SutraDepth — четвёртая ось

### 3.1 Что это

`SutraDepth: u16` — целое число 0..65535, измеряющее **степень укоренённости** Frame или токена в SUTRA.

```
SutraDepth = 0        — свежее, только появившееся (поверхность)
SutraDepth = 100      — несколько раз использовалось
SutraDepth = 1000     — частая константа
SutraDepth = 10000    — глубоко укоренённая структура
SutraDepth = 65535    — примитив-якорь (максимум, фиксированно)
```

Это не "сложность Frame" и не "из чего сделан" — это **состояние укоренённости**. Простой Frame (одна буква) может быть очень глубоким, если используется тысячи раз. Сложный Frame (новая теорема) может быть поверхностным, пока не приживётся.

### 3.2 Где хранится

**Отдельный storage**, не в Token (HARD-инвариант 64 байта). Расположение: `crates/axiom-experience/src/sutra_depth_store.rs`.

```rust
pub struct SutraDepthEntry {
    pub sutra_id: u32,
    pub depth_per_octant: [u16; 8],   // глубина в каждом из 8 октантов
    pub last_settle_event: u64,        // когда последний раз менялась
    pub reactivation_count: u32,       // общая активность
}
```

### 3.3 Почему per-octant

Из `Axiom_Semantic_Core.md` (Октантная модель) — каждый октант это **отдельный тип смысла**. Frame "марширующая армия":
- В октанте 4 (Разрушительно-Активирующий) — очень глубокий (часто активируется в этом контексте)
- В октанте 1 (Творческий Утверждающий) — поверхностный (редко интерпретируется так)
- В октанте 7 (Формальный Отрицающий) — средний (иногда)

Глубина в каждом октанте — **самостоятельная величина**. `[u16; 8]` = 16 байт на Frame. На 100k Frame = 1.6 MB. Допустимо.

### 3.4 Динамика глубины

Глубина **меняется только в DREAM Phase**, не на горячем пути:

```
В Processing этапе DREAM:
    for frame in active_frames():
        for octant in 0..8:
            evidence_in_octant = count_activations(frame, octant, since_last_dream)
            
            if evidence_in_octant > 0:
                # медленный рост вниз (укоренение)
                depth_entry.depth_per_octant[octant] += min(evidence, MAX_GROWTH_PER_CYCLE)
            else:
                # медленный подъём вверх (забывание)
                depth_entry.depth_per_octant[octant] = saturating_sub(depth, DECAY_PER_CYCLE)
        
        depth_entry.last_settle_event = current_event
```

Параметры (конфигурируемые):
- `MAX_GROWTH_PER_CYCLE = 100` — максимум прирост за один DREAM
- `DECAY_PER_CYCLE = 5` — забывание медленнее роста
- Промоция в SUTRA через CODEX → скачок глубины до 30000+
- Примитивы из yaml → 65535 при загрузке, не меняется

### 3.5 Гравитация с учётом глубины

В SPACE V6.0 гравитация работает в плоскости XYZ. **Errata к SPACE:** в V5 ContextRecognizer добавляется коэффициент глубины при подсчёте.

```
В compute_gravity_pair(a, b):
    base_force = mass_a * mass_b / distance²    # как в SPACE V6.0
    
    depth_diff = |depth_a - depth_b|
    depth_factor = max(0.1, 1.0 - depth_diff / 30000)
    
    return base_force * depth_factor
```

Эффект: токены **одной глубины** притягиваются сильнее. Разные слои **относительно изолированы**. Это создаёт натуральную стратификацию: поверхностные новые Frame не "затягиваются" в глубокую структуру примитивов.

Это **опциональная** часть. Можно отключить через config — тогда гравитация работает как в SPACE V6.0 без модификации.

---

## 4. Управление per-octant per-depth-range

### 4.1 Принцип

ContextRecognizer не сканирует все октанты на всех глубинах каждый тик. Это было бы дорого. Вместо этого:

- Определяет **активные октанты** через AxialEvaluator для текущего окна MAYA
- Определяет **активный диапазон глубин** в каждом активном октанте
- Сканирует только это пересечение

### 4.2 ScanningPlan

```rust
pub struct ScanningPlan {
    pub active_regions: Vec<ActiveRegion>,
}

pub struct ActiveRegion {
    pub octant: Octant,
    pub depth_range: DepthRange,
    pub priority: u8,                   // 0..255
    pub fractal_levels: Vec<FractalLevel>,
}

pub struct DepthRange {
    pub min: u16,
    pub max: u16,
}
```

### 4.3 Как формируется план

```
on_tick:
    # 1. AxialEvaluator говорит какие октанты активны для текущего MAYA
    active_octants = axial_evaluator.current_active_octants()
    
    # 2. Для каждого активного октанта — определить диапазон глубин
    for octant in active_octants:
        # Смотрим где сейчас плотность активности в этом октанте
        depth_density = analyze_depth_density(octant, recent_events)
        active_range = find_dense_depth_range(depth_density)
        
        # Добавляем регион в план
        plan.add(ActiveRegion {
            octant,
            depth_range: active_range,
            priority: compute_priority(octant, active_range),
            fractal_levels: relevant_levels(active_range)
        })
    
    # 3. Сканирование только по плану
    for region in plan.active_regions:
        scan_region(region)
```

### 4.4 Пример

Система читает математический текст:

```
AxialEvaluator определяет:
  Активные октанты: Heroic-Fatal (3), Formal-Denying (7)

ContextRecognizer формирует план:
  Region 1: Octant Heroic-Fatal (3)
    DepthRange [200, 5000]   # средние глубины — рабочая математика
    FractalLevels [Word, Phrase]
    Priority 200
  
  Region 2: Octant Formal-Denying (7)
    DepthRange [1000, 8000]  # более глубокие — формальная логика
    FractalLevels [Phrase, Scene]
    Priority 180

Не сканируем:
  Octant Creative-Affirmation (1) — не активен
  Octant Self-Destructive (8) — не активен
  Любая глубина [0, 200] — это шумовые поверхностные Frame
  Любая глубина [50000+] — там примитивы, уже работают через стандартную гравитацию
```

**Места где примитивы не нужны** — это случай 4 выше. Если работа идёт на глубине [200, 5000], сканирование диапазона [50000+] (где примитивы) **не делается ContextRecognizer**. Примитивы по-прежнему **притягивают** через обычную гравитацию SPACE, но **отдельный анализ не нужен**.

### 4.5 Когда сканируется глубина примитивов

Иногда нужно — например, при появлении нового Frame на поверхности, который пока не привязался к примитивам:

```
Условия включения D0 в скан:
- Появился новый Frame с unsettled depth (depth < 100 во всех октантах)
- ContextRecognizer не может определить subsystem (energies слабые на средних глубинах)
- DREAM Phase явно запросила переосмысление (re-evaluation cycle)
- chrnv послал UCL команду RefreshPrimitiveScan
```

В остальных случаях — экономия CPU.

---

## 5. Слоистая структура октанта

### 5.1 Что это значит

Каждый октант имеет **внутреннюю стратификацию по глубине**. Распределение Frame по глубинам в каждом октанте **разное**.

Пример (гипотетический):

```
Octant 1 (Creative-Affirmation):
  D[0..200]:    свежие творческие идеи (много)
  D[200..2000]: устоявшиеся паттерны творчества
  D[2000..10000]: глубокие принципы (мало)
  D[10000..65535]: примитивы Apollo + Eros + Will

Octant 8 (Self-Destructive):
  D[0..200]:    редкие проявления (мало)
  D[200..2000]: pattern мало
  D[2000..10000]: пусто
  D[10000..65535]: примитивы Dionysus + Thanatos + Nothing
```

Это **автоматически следует** из работы системы. ChrNV не должен это конфигурировать. Просто статистика активности.

### 5.2 Видимое в Workstation

В System Map (или в отдельной вкладке Octants) можно показать:

```
┌───────────────────────────────────────────────────┐
│ Octant Depth Distribution                         │
│                                                   │
│ Oct 1 Creative-Affirm    ▓▓▓▓▒▒▒░░ (активна 80%) │
│ Oct 2 Ecstatic-Affirm    ▓▓▒░░░░░░ (активна 30%) │
│ Oct 3 Heroic-Fatal       ▓▓▓▓▓▓▒░░ (активна 90%) │
│ Oct 4 Destructive-Activ  ▓▒░░░░░░░ (активна 15%) │
│ Oct 5 Idealized-Console  ▓▓▒░░░░░░ (активна 25%) │
│ Oct 6 Passive-Sentiment  ░░░░░░░░░ (активна  5%) │
│ Oct 7 Formal-Deny        ▓▓▓▒▒░░░░ (активна 60%) │
│ Oct 8 Self-Destruct      ░░░░░░░░░ (активна  3%) │
│                                                   │
│ Каждый бар — глубинная активность в октанте      │
│ от D0 (слева) до примитивов (справа)              │
└───────────────────────────────────────────────────┘
```

Это даёт chrnv **визуальную картину** где живёт мышление системы. В каких октантах активность, на каких глубинах. Можно увидеть аномалии — например, всплеск в октанте 8 (тревожный знак).

---

## 6. Связь с FrameWeaver

FrameWeaver кристаллизует Frame. При создании нового Frame:

```rust
fn on_frame_crystallized(frame_id: SutraId):
    // Новый Frame появляется с depth_per_octant = [0; 8]
    SutraDepthStore.create_entry(frame_id);
    
    // AxialEvaluator делает первую оценку → определяет октант
    axial_evaluator.evaluate(frame_id);
    
    // ContextRecognizer фиксирует контекст создания
    let snapshot = current_context_snapshot();
    write_creation_context(frame_id, snapshot);
```

**Глубина растёт постфактум**, через DREAM Phase, на основе реактиваций.

---

## 7. Эмерджентные примитивы (V4) + глубина (V5)

Когда emergent primitive обнаружен и одобрен chrnv — он получает:

- Стартовая `depth_per_octant` — высокая в octant'е где был обнаружен (например, 30000)
- В остальных октантах — 0
- Со временем может укореняться в других октантах, если активируется там

Это отличает emergent от обычных примитивов из yaml — те 65535 во всех октантах одновременно. Emergent **локально глубок**, постепенно может стать **универсально глубоким**.

---

## 8. NeuralAdvisor (V4) + глубина (V5)

В V5 добавляется ещё одна точка подключения advisor'а:

```rust
pub trait DepthPredictionAdvisor: NeuralAdvisor<
    Input = FrameUsageHistory,
    Output = (Octant, u16),  // куда Frame "хочет" укорениться
> {}
```

Опционально. Если есть — DREAM Phase использует его hint при обновлении глубин. Без него — обычная статистическая логика.

---

## 9. Структуры данных (V5)

### 9.1 SutraDepthEntry (новое в V5)

```rust
pub struct SutraDepthEntry {
    pub sutra_id: u32,
    pub depth_per_octant: [u16; 8],
    pub last_settle_event: u64,
    pub reactivation_count: u32,
}
```

### 9.2 InterpretationProfile (обновлено)

```rust
pub struct InterpretationProfile {
    pub frame_anchor_sutra_id: u32,
    pub weights: HashMap<SubsystemId, u8>,
    pub primary: SubsystemId,
    pub last_updated_event: u64,
    pub last_context: ContextSnapshot,
    pub frame_composition: FrameComposition,   // переименовано из FractalDepth в V4
    pub primary_octant: Octant,                // куда Frame в основном тянется
}
```

### 9.3 FrameComposition (переименовано)

```rust
// Бывший FractalDepth из V4 — теперь FrameComposition,
// чтобы не путать с SutraDepth.
pub enum FrameComposition {
    C0_Primitive,    // якорь-примитив в SUTRA
    C1_Atom,         // Frame из примитивов
    C2_Molecule,     // Frame из Atoms
    C3_Structure,    // Frame из Molecules
    C4_Composition,  // Frame из Structures
    C5_Plus,         // высшие композиции
}
```

### 9.4 ScanningPlan (новое в V5)

```rust
pub struct ScanningPlan {
    pub active_regions: Vec<ActiveRegion>,
    pub computed_at_event: u64,
}

pub struct ActiveRegion {
    pub octant: Octant,
    pub depth_range: DepthRange,
    pub priority: u8,
    pub fractal_levels: Vec<FractalLevel>,
}

pub struct DepthRange {
    pub min: u16,
    pub max: u16,
}
```

---

## 10. Инварианты

| Правило | Значение |
|---------|----------|
| Чтение состояния | только `&AshtiCore` |
| Запись | только через UCL |
| Где живут якоря подсистем | **SUTRA (100)** |
| State якорей | **STATE_LOCKED** |
| FractalLevel значений | **5** (Symbol, Word, Phrase, Scene, Session) |
| FrameComposition значений | **6** (C0..C5_Plus) |
| **SutraDepth диапазон** | **0..65535** на октант |
| **Где хранится SutraDepth** | отдельный storage `axiom-experience/sutra_depth_store.rs` |
| **Не в Token** | да, HARD 64 байта неприкосновенен |
| **Обновление SutraDepth** | только в DREAMING |
| **Max рост за DREAM** | 100 единиц глубины |
| **Decay за DREAM** | 5 единиц |
| **Примитивы из yaml** | depth = 65535 во всех октантах, не меняется |
| **Promoted Frame** | depth скачком до 30000+ |
| **Octants** | 8, из AxialEvaluator |
| **Gravity depth_factor** | опциональный, конфигурируемый в SPACE config |
| Emergent primitives | максимум 1000 (из V4) |
| NeuralAdvisor влияние | максимум 50% (из V4) |
| Hot reload подсистем | через UCL (из V2) |

---

## 11. Что в коде

```
crates/axiom-runtime/src/over_domain/context_recognizer/
├── mod.rs
├── scanner.rs              — сканирование по ScanningPlan (НОВОЕ в V5)
├── scanning_plan.rs        — НОВОЕ В V5
├── energy.rs               — подсчёт энергий
├── profile.rs              — InterpretationProfile
├── transitions.rs
├── conflicts.rs
├── learning.rs             — обновление позиций + НОВОЕ обновление SutraDepth
├── hot_reload.rs
├── axial_bridge.rs
├── depth_bridge.rs         — НОВОЕ В V5 — мост к SutraDepthStore
├── emergent/
├── advisors/
└── snapshot.rs
```

```
crates/axiom-experience/src/
├── interpretation_profile_store.rs
├── emergent_primitive_store.rs
└── sutra_depth_store.rs    — НОВОЕ В V5
```

```
crates/axiom-space/src/
└── gravity.rs              — errata: опциональный depth_factor
```

```
crates/axiom-ucl/src/commands.rs:
+ RefreshPrimitiveScan       — НОВОЕ В V5
+ QueryDepthDistribution     — НОВОЕ В V5 для Workstation
+ ResetDepthForFrame         — debug, через GUARDIAN
```

---

## 12. Известные ограничения

Все TD из V1 закрыты.

- **CR-TD-01** (sync_axial_store требует координатора) — закрыт. `engine.rs` вызывает
  `sync_axial_store` / `sync_profile_store` / `sync_depth_store` после каждого
  соответствующего цикла. Event/bus между over-domain компонентами не нужен при текущем
  масштабе (3 компонента); пересмотреть при расширении до V6+.

- **CR-TD-02** (первые тики слепые) — закрыт. Реализован тёплый старт через
  `all_octants_in_store` при пустом активном окне.

- **CR-TD-03** (subsystem_refs до первого тика) — закрыт. Добавлен конструктор
  `from_anchor_set(AnchorSet)`, используется в `engine.rs` при инициализации.

---

## 13. Что в V6 и дальше

- **V6:** мета-уровень — подсистема "Subsystems" сама становится распознаваемой; ContextRecognizer узнаёт когда система переключается между мета-режимами (научное / художественное / повседневное)
- **V7:** генеративные подсистемы — система предлагает свои `primitives_*.yaml` целиком на основе паттернов emergent primitives, chrnv одобряет
- **V8:** подсистема Dilemmas рождает новые ценности (аксиогенный механизм из `Дилеммы.md` уровень 5)
- **V9:** активная фаза NeuralAdvisor — конкретные модели подключены ко всем over-domain модулям

---

## История

- **V1.0**: булевы флаги
- **V2.0**: InterpretationProfile с весами, cross_links, обучаемые позиции, transitions, conflicts, hot reload
- **V3.0**: координация с AxialEvaluator, Corpus Callosum, AxialHint
- **V4.0**: FractalDepth отдельно от FractalLevel, emergent primitives, NeuralAdvisor интерфейс
- **V5.0** (2026-05-15): **SutraDepth как четвёртая ось**, хранится в отдельном storage (не в Token, HARD сохранён). **Глубина per-octant** — каждый октант имеет свою стратификацию. **ScanningPlan** — управление сканированием по (octant × depth_range × level), позволяет пропускать неактивные регионы включая глубину примитивов когда они не нужны для текущей работы. FractalDepth переименован в FrameComposition чтобы не путать с SutraDepth. Гравитация может учитывать depth_factor (опционально, errata к SPACE).
