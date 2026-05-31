# AXIOM — DilemmaDetector V2.0

**Статус:** Спецификация
**Версия:** 2.0
**Дата:** 2026-05-29
**Категория:** Под-механизм ContextRecognizer (не отдельный over-domain модуль)
**Crate:** `axiom-runtime` / `over_domain/context_recognizer/dilemma/`
**Опирается на:** `ContextRecognizer_V5_0`, `Дилеммы.md`, `Primitive_Nature_and_Connections_V1_0`, `Connection_V5_0`, `AxialEvaluator`, `INVARIANTS.md`
**Предпосылка:** DilemmaStore существует как инфраструктура, детектора нет. `push_active()` вызывается только в тестах. Эта спека пишет недостающий детектор.

---

## 0. Контекст: инфраструктура без детектора

Диагностика (Sonnet, 2026-05-29) установила:
- `DilemmaStore` существует, `push_active()` — только в unit-тестах
- `dilemma_store` НЕ поле в ContextRecognizer
- `is_natural_tension()` в SubsystemDependencies определена, но не вызывается из runtime
- DilemmaDetector V2.0 упомянут в комментариях как "реализуется в V7" — не написан

Дилеммы не активируются **не из-за стерильного corpus и не из-за застывшего времени** (оно течёт, Cascading 0.97-1.00). Просто нет детектора. Эта спека его определяет.

**Решение по архитектуре (chrnv):** детектор встроен в ContextRecognizer (он уже вычисляет конфликты подсистем), не отдельный модуль. Меньше координации.

---

## 1. Что такое дилемма в AXIOM

Из `Дилеммы.md` — дилемма не баг мышления, а точка роста понимания. Шесть уровней:

| Уровень | Название | Суть | Чем детектируется в коде |
|---------|----------|------|--------------------------|
| 0 | Ложная дилемма | Противоречие в данных, решается уточнением | рассогласование, снимаемое фактом |
| 1 | Тривиальный трейд-офф | Парето-выбор между измеримыми | трейд-офф метрик |
| 2 | Конфликт правил | Два правила без мета-правила | **конфликт подсистем (conflicts_with)** |
| 3 | Несовместимые модели | Конфликт моделей реальности | конфликт интерпретаций + Corpus Callosum |
| 4 | Рефлексивный парадокс | Решение меняет систему | self-reference, fixed point |
| 5 | Аксиогенная | Рождает новую ценность | глубокий неразрешимый конфликт + DREAM |

**Версионный план** (детально §6):
- **V2.0** (эта спека) — уровни 2-3, через конфликт подсистем
- **V2.1** — уровень 0-1 (ложные/трейд-офф), отсеивание + Corpus Callosum для уровня 3
- **V2.2** — уровень 4 (рефлексивные парадоксы)
- **V3.0** — уровень 5 (аксиогенез, рождение ценностей) — смыкается с V8 roadmap

---

## 2. Три сигнала дилеммы

В коде уже есть три источника, по которым детектируется напряжение. V2.0 использует **первый**, остальные намечены.

### Сигнал A — конфликт подсистем ✅ V2.0

Две подсистемы активны с высоким весом одновременно, и они в `conflicts_with` по SubsystemDependencies.

Готовая опора: `is_natural_tension()` (определена, не вызывается) + `conflicts_with` граф. ContextRecognizer уже считает `ContextConflict` (V5 §3.2). Дилемма — надстройка над этим.

```
Math активна (0.7) + Morality активна (0.6)
  и [mathematics, morality] в conflicts_with
  → конфликт подсистем
  → кандидат в дилемму уровня 2-3
```

### Сигнал B — напряжение связей ⏳ V2.1

`current_stress` на Connection растёт (поле уже есть в Connection V5.0). Высокий stress = противоречие в отношениях между токенами. Из `Primitive_Nature_and_Connections` — связь живёт, stress показывает конфликт.

```
Связь A--[supports]-->B со stress > threshold
  одновременно A--[conflicts]-->B активна
  → противоречивые отношения → дилемма
```

### Сигнал C — Corpus Callosum ⏳ V2.1

AxialEvaluator: analytic octant ≠ synthetic octant (уже фиксируется как `AxialConflict`). Конфликт оценки формы и сути. "Красивая смертельная сила" из `Как_объяснить_машине.md`.

```
AxialEvaluator.conflict_present == true для Frame
  → дилемма оценки (уровень 3)
```

V2.0 реализует A. B и C — точки роста, инфраструктура для них уже есть, подключаются в V2.1.

---

## 3. Структуры данных

### 3.1 DilemmaCandidate

```rust
pub struct DilemmaCandidate {
    pub dilemma_type: DilemmaType,
    pub level: DilemmaLevel,              // 0..5
    pub source_subsystems: Vec<SubsystemId>,
    pub involved_frames: Vec<u32>,        // sutra_id Frame в конфликте
    pub tension_score: u8,                // 0..255 сила напряжения
    pub detected_at_event: u64,
    pub signal: DilemmaSignal,            // какой сигнал сработал
    pub resolution: DilemmaResolution,
}

pub enum DilemmaLevel {
    FalseDilemma = 0,
    TradeOff = 1,
    RuleConflict = 2,         // V2.0
    ModelConflict = 3,        // V2.0 (частично), V2.1 (полно через Corpus Callosum)
    ReflexiveParadox = 4,     // V2.2
    Axiogenic = 5,            // V3.0
}

pub enum DilemmaSignal {
    SubsystemConflict,        // сигнал A — V2.0
    ConnectionStress,         // сигнал B — V2.1
    CorpusCallosum,           // сигнал C — V2.1
    SelfReference,            // V2.2
}

pub enum DilemmaResolution {
    Detected,                 // V2.0: только зафиксирован
    Crystallized(u32),        // V2.0: Frame дилеммы создан в EXPERIENCE (sutra_id)
    HeldInTension,            // V2.1: удерживается без разрешения (уровень 3)
    Transcended(u32),         // V3.0: породил новую ценность (sutra_id ценности)
    Dissolved,                // V2.1: оказался ложным (уровень 0), снят
}
```

### 3.2 Поле в ContextRecognizer

```rust
pub struct ContextRecognizer {
    // ... существующие поля V5 ...
    pub dilemma_store: DilemmaStore,      // НОВОЕ — было упущено
    dilemma_detector: DilemmaDetector,    // НОВОЕ
}
```

Это закрывает первую часть диагноза: `dilemma_store` теперь поле в CR.

---

## 4. Алгоритм V2.0 (встроен в CR on_tick)

ContextRecognizer уже вычисляет `ContextConflict` на каждом уровне сканирования. Дилемма-детект — под-шаг после этого.

```
on_tick (внутри ContextRecognizer, после detect_conflicts):

    for conflict in current_state.conflicts:
        # Конфликт подсистем уже найден CR (две подсистемы, вес >0.4 каждая)
        
        # Проверка: это natural tension по графу зависимостей?
        if is_natural_tension(conflict.subsystems):    # ← наконец вызываем
            
            tension = compute_tension_score(conflict)
            
            if tension > DILEMMA_THRESHOLD:             # 128/255 default
                candidate = DilemmaCandidate {
                    dilemma_type: classify(conflict),
                    level: RuleConflict,                # V2.0 = уровень 2
                    source_subsystems: conflict.subsystems,
                    involved_frames: frames_in_conflict(conflict),
                    tension_score: tension,
                    signal: SubsystemConflict,
                    resolution: Detected,
                }
                
                # Фиксация
                dilemma_store.push_active(candidate);   # ← наконец вызываем из runtime
                
                # Кристаллизация Frame дилеммы в EXPERIENCE (решение chrnv)
                crystallize_dilemma_frame(candidate);
```

### 4.1 compute_tension_score

```
tension = f(
    вес обеих подсистем,            # обе сильные → выше
    длительность конфликта,         # дольше держится → выше
    острота по conflicts_with графу # насколько фундаментально противоречие
)
```

### 4.2 crystallize_dilemma_frame

Из решения chrnv: фиксировать + кристаллизовать Frame в EXPERIENCE (дилемма становится памятью).

```
Frame-анкер дилеммы:
  domain_id = 109 (EXPERIENCE)
  type_flags = TOKEN_FLAG_FRAME_ANCHOR | TOKEN_FLAG_DILEMMA
  composed_of: [frames_in_conflict]      # из Primitive_Nature — дилемма это
                                          # примитив-ОТНОШЕНИЕ, composed of конфликтующих
  position: между конфликтующими подсистемами (центроид)
  через UCL InjectFrameAnchor (как обычный Frame)
```

Дилемма-Frame — это **примитив-отношение** в терминах `Primitive_Nature`: конфликт двух ценностей/подсистем, реализованный как Frame со связями к участникам. Не вещь — отношение, ставшее памятью.

Новый флаг: `TOKEN_FLAG_DILEMMA = 0x0080` (свободный бит в type_flags, проверить INVARIANTS §8).

---

## 5. Что V2.0 НЕ делает (и как это решается дальше)

Из решения chrnv: пометить частичное решение, указать как и во сколько шагов наращивается.

| Не делает в V2.0 | Решается в | Как |
|------------------|-----------|-----|
| Не отсеивает ложные дилеммы (уровень 0) | V2.1 | проверка снимаемости фактом → Dissolved |
| Не ловит трейд-оффы (уровень 1) | V2.1 | детект Парето-выбора метрик |
| Не использует stress связей (сигнал B) | V2.1 | сканировать current_stress на связях |
| Не использует Corpus Callosum (сигнал C) | V2.1 | читать AxialConflict из AxialEvaluator |
| Не удерживает в напряжении (уровень 3) | V2.1 | HeldInTension — не разрешать, хранить оба |
| Не ловит рефлексивные парадоксы (уровень 4) | V2.2 | детект self-reference, fixed point |
| **Не разрешает дилемму** | V3.0 | аксиогенез — рождение новой ценности |

V2.0 = **обнаружить и запомнить**. Разрешение — отдельная ветка, наращивается версиями. Это правильно: сначала научиться видеть дилемму, потом — что с ней делать.

---

## 6. Версионный план до 6 уровней

Полное покрытие `Дилеммы.md` за четыре версии.

### V2.0 (эта спека) — Обнаружение конфликтов
- Сигнал A (конфликт подсистем)
- Уровни 2-3 (частично)
- Resolution: Detected + Crystallized
- Встроен в CR, вызывает is_natural_tension + push_active
- **Цель: дилеммы начинают фиксироваться и становиться памятью**

### V2.1 — Полнота сигналов и удержание
- Сигналы B (stress связей) + C (Corpus Callosum)
- Уровни 0-1 (отсев ложных, трейд-оффы) → Dissolved
- Уровень 3 полно: HeldInTension — удерживать несовместимые модели параллельно (из `Дилеммы.md` уровень 3: "оперировать обеими моделями как взаимодополнительными")
- **Цель: система различает виды дилемм, не путает ложную с настоящей**

### V2.2 — Рефлексивные парадоксы
- Сигнал SelfReference
- Уровень 4 (решение меняет систему, fixed point)
- Детект когда дилемма ссылается на саму систему оценки
- **Цель: система замечает парадоксы самоотнесения**

### V3.0 — Аксиогенез (смыкается с ContextRecognizer V8 roadmap)
- Уровень 5: рождение новой ценности
- Resolution: Transcended — глубокий неразрешимый конфликт через DREAM Phase порождает новый якорь в Values
- Transcendence logic (из roadmap V8): не среднее между ценностями, а новое измерение
- Требует wisdom_store, длительной работы системы
- **Цель: дилемма как источник роста — система создаёт ценности, которых не было**

```
V2.0 ──→ V2.1 ──→ V2.2 ──→ V3.0
видеть   различать  парадоксы  разрешать через
конфликт  виды                 рождение ценности
```

---

## 7. Инварианты

| Правило | Значение |
|---------|----------|
| Где живёт детектор | под-механизм ContextRecognizer, не отдельный модуль |
| Когда тикает | внутри CR on_tick, после detect_conflicts (не нужен свой tick) |
| Чтение | через CR — только `&AshtiCore` |
| Запись | через UCL (crystallize via InjectFrameAnchor) |
| DILEMMA_THRESHOLD | 128/255 (soft, конфигурируемый) |
| Frame дилеммы | EXPERIENCE (109), TOKEN_FLAG_DILEMMA |
| Дилемма как примитив | примитив-отношение (composed_of конфликтующих) |
| Разрешение в V2.0 | нет (только Detected + Crystallized) |
| Аксиогенез | только V3.0, требует chrnv approval (как все ценности) |

---

## 8. Что в коде V2.0

```
crates/axiom-runtime/src/over_domain/context_recognizer/
├── ... существующее V5 ...
└── dilemma/                      — НОВОЕ
    ├── mod.rs                    — DilemmaDetector
    ├── detector.rs               — алгоритм §4, сигнал A
    ├── tension.rs                — compute_tension_score
    └── crystallize.rs            — crystallize_dilemma_frame

crates/axiom-runtime/src/over_domain/context_recognizer/mod.rs:
+ dilemma_store: DilemmaStore       — поле (было упущено)
+ dilemma_detector: DilemmaDetector
+ вызов detector после detect_conflicts в on_tick

crates/axiom-core/src/token.rs:
+ TOKEN_FLAG_DILEMMA: u16 = 0x0080  — проверить свободность в INVARIANTS §8

SubsystemDependencies:
+ is_natural_tension() — наконец вызывается из detector (был мёртвый код)
```

Минимально. DilemmaStore уже есть. is_natural_tension есть. conflicts_with есть. Пишем **связку** между готовыми частями + кристаллизацию.

---

## 9. Проверка успеха V2.0

После реализации, на corpus с конфликтами (тип В из Task_Dynamics — "запереть двери: спасти / лишить свободы"):

- `dilemma_store` непустой после прогона
- Frame дилемм в EXPERIENCE с TOKEN_FLAG_DILEMMA
- OBS runner измеряет дилеммы (сейчас не измеряет — нечего было)
- tension_score коррелирует с остротой конфликта в тексте

Если дилеммы пошли — V2.0 закрыта, можно к V2.1 (полнота сигналов) или к cross-modal binding (UGS).

---

## 10. Связь с другими спеками

- **Primitive_Nature_and_Connections**: дилемма = примитив-отношение. Эта спека — первое практическое применение того принципа.
- **ContextRecognizer V8 roadmap**: аксиогенез был намечен там, теперь конкретизирован как DilemmaDetector V3.0.
- **AxialEvaluator**: Corpus Callosum (сигнал C) — мост в V2.1.
- **Universal Grounding Stack**: дилемма уровня 5 — высший уровень UGS (рождение ценностей из неразрешимых конфликтов).
- **Дилеммы.md** (chrnv): первоисточник 6 уровней, полностью покрывается планом §6.

---

## История

- **V2.0** (2026-05-29): первая реализуемая спека детектора. Закрывает диагноз "инфраструктура без детектора". Сигнал A (конфликт подсистем) через готовые is_natural_tension + conflicts_with. Resolution: Detected + Crystallized (дилемма становится памятью в EXPERIENCE). Встроен в ContextRecognizer. Версионный план §6 покрывает все 6 уровней Дилеммы.md за V2.0→V2.1→V2.2→V3.0. Дилемма понята как примитив-отношение (связь с Primitive_Nature). V1.0 не существовало — номер 2.0 взят из комментариев в коде где детектор был обещан как "V7/V2.0".
