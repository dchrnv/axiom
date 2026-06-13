# Axiom — Справочник инвариантов

**Версия:** 14.0 (2026-06-13)  
**Правило:** Значения помеченные **HARD** менять запрещено — они фиксированы в коде compile-time assert-ами, бинарным форматом или фундаментальной логикой. Всё остальное — soft (настраиваемый дефолт).

---

## 1. Размеры структур

| Структура | Размер | Выравнивание | Проверка |
|-----------|--------|--------------|----------|
| `Token` | **64 байта** | 64 байта | `assert!(size_of::<Token>() == 64)` |
| `Connection` | **64 байта** | 64 байта | compile-time assert |
| `Event` | **64 байта** | 64 байта | compile-time assert |
| `DomainConfig` | **128 байт** | 128 байт | compile-time assert |
| `ShellProfile` | **8 байт** (`[u8; 8]`) | — | тип |
| `UclCommand` | **64 байта** | — | HARD |
| `UclResult` | **32 байта** | — | HARD |

> **НИКОГДА** не добавлять поля в Token/Connection/Event/DomainConfig без пересчёта layout и обновления assert-ов.

---

## 2. Поля Token (`crates/axiom-core/src/token.rs`)

| Поле | Тип | Диапазон / инвариант |
|------|-----|----------------------|
| `sutra_id` | `u32` | **> 0** |
| `domain_id` | `u16` | **> 0** |
| `type_flags` | `u16` | битовая маска (см. §8) |
| `position` | `[i16; 3]` | −32768..+32767 |
| `velocity` | `[i16; 3]` | −32768..+32767 |
| `target` | `[i16; 3]` | −32768..+32767 |
| `origin` | `u16` | 0x0000..0xFFFF |
| `valence` | `i8` | −128..+127 |
| `mass` | `u8` | **> 0** (инвариант) |
| `temperature` | `u8` | 0..255 |
| `state` | `u8` | **1 / 2 / 3** (только три значения) |
| `lineage_hash` | `u64` | любое |
| `momentum` | `[i32; 3]` | −2³¹..+2³¹−1 |
| `resonance` | `u32` | 0..100 |
| `last_event_id` | `u64` | **> 0** |

---

## 3. Поля Connection (`crates/axiom-core/src/connection.rs`)

| Поле | Тип | Инвариант |
|------|-----|-----------|
| `source_id` | `u32` | **> 0** |
| `target_id` | `u32` | **> 0** |
| `domain_id` | `u16` | **> 0** |
| `link_type` | `u16` | старший байт = категория (0x01..0x0A) |
| `flags` | `u32` | битовая маска (см. §8) |
| `strength` | `f32` | **> 0.0** |
| `current_stress` | `f32` | >= 0.0 |
| `elasticity` | `f32` | **> 0.0** |
| `created_at` | `u64` | **> 0** |
| `last_event_id` | `u64` | >= `created_at` (монотонно) |

---

## 4. Поля Event (`crates/axiom-core/src/event.rs`)

| Поле | Тип | Инвариант |
|------|-----|-----------|
| `event_id` | `u64` | **> 0, строго возрастает** |
| `parent_event_id` | `u64` | **< event_id** |
| `payload_hash` | `u64` | **!= 0** |
| `event_type` | `u16` | repr(u16), см. диапазоны в §8 |
| `payload` | `[u8; 8]` | inline data |

---

## 5. Адресация доменов

**Формула:** `domain_id = level_id × 100 + offset`  
**Уровень 1 (единственный в системе):**

| Имя | domain_id | structural_role |
|-----|-----------|-----------------|
| SUTRA | **100** | 0 |
| EXECUTION | **101** | 1 |
| SHADOW | **102** | 2 |
| CODEX | **103** | 3 |
| MAP | **104** | 4 |
| PROBE | **105** | 5 |
| LOGIC | **106** | 6 |
| DREAM | **107** | 7 |
| ETHICS/VOID | **108** | 8 |
| EXPERIENCE | **109** | 9 |
| MAYA | **110** | 10 |

**Доменов ровно 11.** Менять без изменения GENOME — запрещено.

---

## 6. Система координат

| Параметр | Значение |
|----------|----------|
| Тип координат | `[i16; 3]` (X, Y, Z) |
| Диапазон по инварианту N1 | **[0..32767]³** — все координаты положительные |
| Тип допускает | −32768..+32767 (i16), но отрицательные значения нарушают N1 |
| Промежуточные вычисления | **i32 или i64** — переполнение i16 при вычитании! |
| `distance2` возвращает | **i64** (максимум 3 × 32768² = 3 221 225 472) |
| FNV-1a fallback позиции | **[0..32767]³** (`& 0x7FFF`) |
| Глобальный центр притяжения | **НЕТ** (R4/REPAIR-01: глобальная гравитация удалена) |
| Семантическая гравитация | только SubsystemGravity (к позициям val_beneficial/val_harmful/abstraction_*) |

**Параметрические оси (N2, REPAIR-01):**

Природа токена (Аполлон/Дионис и т.д.) — ТОЛЬКО из подписи, НИКОГДА из координат.

| Ось | Параметр | Положительный полюс | Отрицательный полюс |
|-----|----------|---------------------|---------------------|
| X | `mass` (u8) | Аполлон: порядок (высокий mass) | Дионис: хаос (низкий mass) |
| Y | `valence` (i8) | Эрос: жизнь (valence > 0) | Танатос: смерть (valence < 0) |
| Z | `temperature` (u8) | Воля: агентность (высокая temp) | Ничто: тишина (низкая temp) |

> `config/anchors/axes.yaml` — словарь шкал (документация), не список якорей. Осевые токены в SUTRA не инжектируются (R3).

---

## 7. Shell-профиль (`crates/axiom-shell/src/lib.rs`)

| Параметр | Значение |
|----------|----------|
| Тип | `[u8; 8]` |
| Диапазон каждого слоя | **0..255** (0 = не затронут, 255 = максимум) |
| Число слоёв | **8** |
| Пустой Shell | `[0; 8]` — корректное состояние |
| Хранение | **вне Token**, в `DomainShellCache` |

**Слои:**

| Индекс | Имя | Смысл |
|--------|-----|-------|
| L1 | Physical | Материальность, вещественность |
| L2 | Sensory | Ощущения, восприятие |
| L3 | Motor | Движение, действие |
| L4 | Emotional | Чувства, аффект |
| L5 | Cognitive | Мышление, знание |
| L6 | Social | Отношения, роли |
| L7 | Temporal | Время, ритм |
| L8 | Abstract | Символ, чистая идея |

**Базовые Shell-вклады категорий связей:**

| Категория (старший байт `link_type`) | Базовый профиль [L1..L8] |
|--------------------------------------|--------------------------|
| 0x01 Structural | [20, 5, 0, 0, 5, 0, 0, 0] |
| 0x02 Semantic | [0, 0, 0, 0, 15, 0, 0, 10] |
| 0x03 Causal | [0, 0, 5, 0, 15, 0, 10, 8] |
| 0x04 Experiential | [5, 20, 0, 15, 0, 0, 0, 0] |
| 0x05 Social | [0, 0, 0, 5, 0, 25, 0, 0] |
| 0x06 Temporal | [0, 0, 0, 0, 5, 0, 25, 0] |
| 0x07 Motor | [10, 0, 25, 0, 5, 0, 0, 0] |
| 0x08 Syntactic | [0, 0, 0, 0, 10, 5, 0, 15] |
| 0x09 Spatial/Composition | [10, 15, 0, 0, 8, 0, 0, 5] |
| 0x0A CrossModal | [0, 20, 0, 0, 10, 0, 0, 10] |
| 0x0B SemanticAnchor | [0, 0, 0, 0, 20, 0, 0, 15] |

**Именованные link_type константы (axiom-shell):**

| Константа | Значение | Назначение |
|-----------|----------|------------|
| `COMPOSITION_BOND` | `0x0901` | FrameWeaver: дочерний → родительский Frame |
| `CROSS_MODAL_BOND` | `0x0A01` | CrossModalDetector: Text↔Vision grounding |
| `SEMANTIC_ANCHOR_BOND` | `0x0B01` | TextPerceptor: текстовый токен → matched anchor (AE-TD-08) |

**Диапазоны sutra_id (детерминированные):**

| Диапазон | Назначение |
|----------|------------|
| `1..event_id` | Sequential tokens (event_id as u32) |
| `0x0001..0x0FFF_FFFF` | domain_position_hash (28 бит) |
| `0x1000_0001..0x1FFF_FFFF` | temporal_anchor_stable_id (TemporalPerceptor, бит 28) |
| `0x2000_0001..0x3FFF_FFFF` | vision_anchor_stable_id (L0VisionPerceptor, бит 29) |
| `0x4000_0001..0x7FFF_FFFF` | text_stable_id (TextPerceptor, бит 30) |
| `0x8000_0001..0xFFFF_FFFF` | anchor_sutra_id (fnv1a_anchor_id, бит 31) |

---

## 8. Флаги и константы состояний

**Token.state:**

| Константа | Значение |
|-----------|----------|
| `STATE_ACTIVE` | **1** |
| `STATE_SLEEPING` | **2** |
| `STATE_LOCKED` | **3** |

**Token.type_flags:**

| Константа | Значение |
|-----------|----------|
| `TOKEN_FLAG_GOAL` | `0x0001` |
| `TOKEN_FLAG_IMPULSE` | `0x0002` |
| `TOKEN_FLAG_FRAME_ANCHOR` | `0x0010` |
| `TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE` | `0x0020` |
| `TOKEN_FLAG_DREAM_REPORT` | `0x0040` |
| `TOKEN_FLAG_DILEMMA` | `0x0080` |
| `FRAME_CATEGORY_MASK` | `0xFF00` |
| `FRAME_CATEGORY_SYNTAX` | `0x0100` |

**Token.origin:**

| Константа | Значение |
|-----------|----------|
| `TOKEN_ORIGIN_LOCAL` | `0x0000` |
| `TOKEN_ORIGIN_PERSISTED` | `0xFE00` |
| `TOKEN_ORIGIN_EXTERNAL_BASE` | `0xFF00` |

**Connection.flags:**

| Константа | Значение |
|-----------|----------|
| `FLAG_ACTIVE` | `1` |
| `FLAG_INHIBITED` | `2` |
| `FLAG_TEMPORARY` | `4` |
| `FLAG_CRITICAL` | `8` |

**EventPriority (u8):**

| Уровень | Значение |
|---------|----------|
| Low | 0 (0..63) |
| Normal | 128 (64..191) |
| High | 200 (192..254) |
| Critical | **255** |

**EventType диапазоны (u16):**

| Диапазон | Категория |
|----------|-----------|
| 0x0001–0x000C | Token события |
| 0x0010–0x0012 | SPACE / движение |
| 0x1001–0x1008 | Connection события |
| 0x2001–0x2003 | Domain события |
| 0x3001–0x3005 | Physics события |
| 0xE001–0xE002 | Внешние/агентские |
| 0xF001–0xF003 | Системные |
| `0xFFFF` | Unknown — безопасный fallback |

---

## 9. Якорные токены — константы инжекции

| Параметр | Значение |
|----------|----------|
| `mass` | **255** |
| `temperature` | **0** |
| `state` | **STATE_LOCKED (3)** |
| Осевые + слоевые → домен | **SUTRA (100)** |
| Доменные якоря D1..D8 → домен | **101..108** |
| Количество осей | **6** |
| Количество слоёв | **8** |
| Количество доменных групп | **8** |

**Файловая структура якорей:**
```
config/anchors/
├── axes.yaml              — 6 осевых якорей
├── layers/L{1..8}_*.yaml  — по слою
└── domains/D{1..8}_*.yaml — по домену
```

**Frame-анкер в EXPERIENCE:**

| Параметр | Значение |
|----------|----------|
| `domain_id` | **109** |
| `state` | STATE_ACTIVE (1) |
| `type_flags` | `TOKEN_FLAG_FRAME_ANCHOR` (0x0010) |
| `temperature` | начальная 128 |

**Промотированный Frame в SUTRA:**

| Параметр | Значение |
|----------|----------|
| `domain_id` | **100** |
| `state` | STATE_LOCKED (3) |
| `type_flags` | `0x0010 | 0x0020` |
| Путь | **только DREAMING + CODEX** |
| Минимальный возраст | 100 000 event_id |
| Минимум реактиваций | 10 |

---

## 10. Хэши и FNV-1a

| Параметр | Значение |
|----------|----------|
| FNV-1a offset basis | **0xcbf29ce484222325** |
| FNV-1a prime | **0x100000001b3** |
| Операция | `h ^= byte; h = h.wrapping_mul(prime)` |
| Результат | `u64` |
| Spatial hash primes | X: 73856093 / Y: 19349663 / Z: 83492791 |
| TextPerceptor fallback маска | `& 0x7FFF` → **0..32767** |

---

## 11. Временная модель (COM)

| Правило | |
|---------|--|
| Единственная мера времени | `event_id` (u64, монотонно) |
| `std::time` / wall-clock в ядре | **ЗАПРЕЩЕНО** |
| `parent_event_id < event_id` | ОБЯЗАТЕЛЬНО |
| `CausalHorizon` | только растёт, никогда не убывает |

---

## 12. Персистентность

| Параметр | Значение |
|----------|----------|
| Формат | **bincode** (не JSON) |
| Атомарность | temp-файл + rename |
| Путь | `<data_dir>/engine_state.bin` |
| Import weight factor | **0.7** |

---

## 13. DREAM Phase

| Параметр | Значение |
|----------|----------|
| Переходы | `Wake → FallingAsleep → Dreaming → Waking → Wake` |
| SUTRA write с FLAG_FRAME_ANCHOR | **только в DREAMING** |
| Critical-команда в Dreaming | прерывает → Waking |
| Normal-команда в Dreaming | **отклоняется** |

---

## 14. Архитектурные запреты

| Правило |
|---------|
| `#![deny(unsafe_code)]` во всех crates ядра |
| GENOME неизменяем после `Arc::new()` в рантайме |
| Over-Domain читают состояние только через `&AshtiCore` |
| Over-Domain пишут только через UCL-команды |
| Over-Domain не владеют доменными данными |
| `sutra_id` токена — уникальный ID потока, всегда **> 0** (0 зарезервирован как «нет потока») |
| `domain_id` токена — ID домена в котором живёт токен, всегда **> 0** (формула: `level×100 + role`) |
| `event_id` и `last_event_id` — ID событий COM, всегда **> 0** и строго монотонно возрастают |
| `mass` токена — физическая масса, влияет на гравитацию и динамику, всегда **> 0** |
| DAG-инвариант при промоции Frame в SUTRA — обязателен |
| Запись Frame в SUTRA — только через CODEX в состоянии DREAMING |
| **Токены не удаляются физически**. Жизненный цикл: `STATE_ACTIVE → STATE_SLEEPING`. Переход в `STATE_SLEEPING` происходит через `TokenDecayed` событие (причинный возраст > порога). Физическое удаление из `Vec` допустимо **только** для `STATE_SLEEPING` токенов — и только когда домен заполнен и нужен слот для нового активного токена. `AnnihilateToken` (UCL 2002) — **no-op** по дизайну. |
| `STATE_LOCKED` токены (якоря, `temperature=0`) **никогда не затрагиваются** eviction/decay — это конституционные примитивы пространства |
| **Sensorium только читает** — `collect()` принимает `&SensoriumView` (immutable), `on_boot()` принимает `&self`. Никаких `&mut` на данные движка. GENOME: ExperienceMemory/Read, AshtiField/Read, MayaOutput/Read, SutraTokens/Read. Инвариант навсегда. |
| **Waves поднимает импульс, не управляет** — пишет только через UCL (ReinforceFrame в EXPERIENCE). Не пишет в SUTRA напрямую. В DREAMING молчит. MAX_ACTIVE_IMPULSES=4. |

---

## 15. Over-Domain Layer — Phase C модули

### ModuleId (axiom-genome)

`ModuleId` — `#[repr(u8)]`, значения HARD (зафиксированы в GenomeIndex и GENOME rules).  
`MAX_MODULES = 23`.

| ModuleId | u8 | Описание |
|----------|----|----------|
| Sutra..Maya | 0–10 | Доменные роли AshtiCore |
| Arbiter | 11 | Внутренний маршрутизатор AshtiCore |
| Guardian | 12 | Конституционный фильтр |
| Heartbeat | 13 | Генератор импульсов |
| Shell | 14 | Shell-профили |
| Adapters | 15 | Внешние адаптеры |
| FrameWeaver | **16** | Кристаллизация Frame из MAYA → EXPERIENCE |
| AxialEvaluator | **17** | Оценка Frame по осям X/Y/Z (8 октантов); V2: stability/persistence trackers; source_id=1 |
| ContextRecognizer | **18** | SubsystemEnergy, InterpretationProfile, SutraDepthStore |
| NeuralAdvisor | **19** | Advisory-only; 5 советников; poll_advisories() |
| OverDomainArbiter | **20** | Координатор advisory-источников; TrustConfig; PendingQueue |
| Sensorium | **21** | Полный внутренний срез + выражение; только чтение (&self); GENOME-инвариант навсегда |
| Waves | **22** | Внутренний ветер; поднимает импульс изнутри; не пишет SUTRA напрямую |

### Тик-интервалы Phase C (on_tick_interval)

Простые числа — не совпадают между собой и не создают кратных пиков нагрузки.

| Модуль | Интервал | Константа |
|--------|----------|-----------|
| AxialEvaluator | **5** тиков | — |
| ContextRecognizer | **7** тиков | — |
| NeuralAdvisor | **11** тиков | — |
| OverDomainArbiter | **13** тиков | `ARBITER_TICK_INTERVAL` |

### SutraDepthStore — константы глубины

Хранит `depth_per_octant: [u16; 8]` на каждый Frame (sutra_id).

| Константа | Значение | Смысл |
|-----------|----------|-------|
| `PRIMITIVE_DEPTH` | **65535** (`u16::MAX`) | Frame является зарегистрированным примитивом — не меняется советниками |
| `PROMOTED_DEPTH` | **30000** | Глубина при промоции Frame в SUTRA |
| `DEPTH_FLOOR` | **50** | Минимальный пол для «мёртвых» фреймов (AgeDecayAdvisor) — не обнуляет |

> **Правило:** советники НЕ трогают Frame с `depth == PRIMITIVE_DEPTH`.  
> Советники предлагают глубину, Arbiter применяет через `set_promoted_depth`.

### DepthHint советники (NeuralAdvisor)

| Советник | Условие срабатывания | Suggested depth |
|----------|----------------------|-----------------|
| `ReactivationDepthAdvisor` | reactivations ≥ 20, age > 50, current_depth < 500 | min(count×15, 3000) |
| `SubsystemAffinityDepthAdvisor` | affinity_octant_depth < 800 | 1500 |
| `AgeDecayAdvisor` | age > 200, reactivations == 0, current > DEPTH_FLOOR | DEPTH_FLOOR (50) |

### OverDomainArbiter — константы и SourceId

| Параметр | Значение |
|----------|----------|
| `NEURAL_ADVISOR_SOURCE_ID` | **0** |
| `AXIAL_EVALUATOR_SOURCE_ID` | **1** |
| `ARBITER_TICK_INTERVAL` | **13** |
| `ArbiterLog` max entries | **500** (ring buffer) |
| Advisory ID scheme | `(sutra_id as u64) << 8 \| type_index` |
| `auto_apply_allowed` default | **false** до `on_boot` |

### TrustConfig default V1 (NeuralAdvisor source=0, AxialEvaluator source=1)

**NeuralAdvisor (source=0):**

| AdvisoryType | Режим | min_confidence |
|--------------|-------|----------------|
| DepthHint | **AutoApply** | 0.75 |
| OctantCorrection | RequireConfirmation | 0.60 |
| ConflictDiagnosis | Ignore | — |
| SubsystemAttribution | Ignore | — |
| EmergentCandidate | RequireConfirmation | 0.60 |

**AxialEvaluator (source=1):**

| AdvisoryType | Режим | min_confidence |
|--------------|-------|----------------|
| OctantCorrection | RequireConfirmation | 0.70 |
| ConflictDiagnosis | RequireConfirmation | 0.60 |

### AxialEvaluator V2 — трекеры и константы

| Параметр | Значение | Смысл |
|----------|----------|-------|
| `AXIAL_EVALUATOR_SOURCE_ID` | **1** | SourceId для OverDomainArbiter |
| `STABILITY_HISTORY_DEPTH` | **10** | Длина кольцевого буфера OctantStabilityTracker |
| `STABILITY_THRESHOLD` | **0.70** | Порог доли доминирующего октанта для fire |
| `STABILITY_MIN_HISTORY` | **5** | Минимум записей перед проверкой стабильности |
| `CONFLICT_PERSISTENCE_THRESHOLD` | **5** | Подряд-конфликтов до ConflictDiagnosis advisory |
| `MAX_EVALUATIONS_PER_FRAME` | **20** | Ограничение истории AxialStore на один Frame |

> После fire OctantStabilityTracker очищает историю Frame.  
> После fire ConflictPersistenceTracker сбрасывает streak в 0.

### SubsystemLifecycleState (Phase H2)

Жизненный цикл кандидата в подсистемы (`dream_phase/subsystem.rs`).

| Variant | Value | Смысл |
|---------|-------|-------|
| `Proposed` | **0** | Автоматически обнаружен (кластер ≥2 approved primitives) |
| `Candidate` | **1** | Оператор одобрил для изучения |
| `InReview` | **2** | Детальный анализ оператором |
| `Active` | **3** | Интегрирован в таксономию подсистем |
| `Mature` | **4** | Долгосрочно устойчивый |
| `Deprecated` | **5** | Выводится из использования |
| `Archived` | **6** | Удалён из активного использования |

Допустимые переходы: `Proposed→Candidate→InReview→Active→(Mature|Deprecated)→Archived`.  
`ApproveSubsystemCandidate` (UCL 5301) переводит `Proposed→Candidate`.

### DivergenceLog (Phase G1)

| Константа | Значение | Смысл |
|-----------|----------|-------|
| `DIVERGENCE_LOG_CAPACITY` | **256** | Размер скользящего буфера расхождений |
| Порог записи | Hamming distance ≥ 2 | Фиксируются только существенные расхождения |

`octant_hamming_distance(a, b)` — число отличающихся осей (0–3).

### EmergentPatternAdvisor пороги (DepthThresholdEmergentDetector)

| Константа | Значение | Откалибровано |
|-----------|----------|---------------|
| `EMERGENT_CANDIDATE_MIN_DEPTH` | **1000** | OBS-02 (O7 avg_depth=1198 за 30k тиков) |
| `EMERGENT_CANDIDATE_MIN_REACTIVATIONS` | **5** | OBS-02 (~10-15 DREAM-циклов за 30k тиков) |
| `EMERGENT_CANDIDATE_MIN_AGE_TICKS` | **100** | OBS-02 |

### ContextRecognizer V6 — ActivityTrace + SubsystemFatigue

| Константа | Значение | Смысл |
|-----------|----------|-------|
| `SHORT_CAP` | **16** | Размер короткого кольцевого буфера (oscillation window) |
| `MID_CAP` | **64** | Размер среднего буфера (convergence / cascade window) |
| `LONG_CAP` | **256** | Размер длинного буфера (fatigue / persistence window) |
| `MIN_WINDOW_FILL` | **16** | Минимум записей до выхода из холодного старта (Uncertain) |
| `MAX_ACTIVATION_LOAD` | **10.0** | Равновесная нагрузка при непрерывной активности (1/(1-DECAY)) |
| `DECAY_FACTOR` | **0.90** | Затухание `activation_load` за один on_tick |
| `DEBT_RATE` | **0.05** | Прирост `recovery_debt` за один on_tick при активной подсистеме |
| `DEBT_DECAY` | **0.998** | Затухание `recovery_debt` (медленно, lingers при смене primary) |
| `DREAM_RECOVERY` | **0.35** | Коэффициент частичного восстановления при DREAM-пробуждении |

> `effective_weight = base * (1.0 - 0.5 * min(1.0, activation_load / MAX_ACTIVATION_LOAD))`  
> При max load вес уполовинивается. `recovery_debt` не сбрасывается DREAM-ом.

---

## 16. Известные расхождения спека vs код

| Место | Расхождение | Источник истины |
|-------|-------------|-----------------|
| `GENOME_V1_0.md` пишет `event_size: 32` | Реальный Event = **64 байта** (compile-time assert в `event.rs`) | **Код** |

---

## 17. Инварианты пространства и природы (REPAIR-01, 2026-06-13)

Зафиксированы после аудита R1 — источник: `docs/spec/Foundation/REPAIR-01_Coordinates_vs_Nature.md`.

| ID | Правило | Статус |
|----|---------|--------|
| **N1** | Координаты `[0..32767]` положительные. Позиция несёт ТОЛЬКО близость (гравитация, грид, резонанс). Позиция НИКОГДА не несёт философию. | **HARD** |
| **N2** | Октант/природа токена и Frame — ТОЛЬКО из `mass` / `valence` / `temperature` + `origin_domain`. НИКОГДА из знаков или величин координат. | **HARD** |
| **N3** | Природу (+/-,+/-,+/-) несут только 8 ASHTI-доменов (101–108) как внешнее свойство домена целиком (мембранные профили). Природа НЕ распространяется внутрь куба домена. SUTRA (100) / EXPERIENCE (109) / MAYA (110) — вне октантной философии. | **HARD** |
| **N4** | Центр куба = (16384, 16384, 16384). Точка (0,0,0) — угол, не центр. Глобальных центров притяжения нет. | **HARD** |
| **N5** | EXECUTION (101) — основной/главный домен (primary processing). | soft |
| **N6** | DREAM наступает по СОСТОЯНИЮ (простой/idle). Интервальный запуск сна (`dream_interval > 0`) допустим только в OBS/тестах под явным флагом, из боевого пути убран (default = 0). | **HARD** |

**Что сделано (REPAIR-01):**
- R2: `synthesis.rs` переписан на `axis_scores_from_signature(token)` — октант из подписи, не позиции
- R3: `axes.yaml` → словарь параметрических шкал; осевые токены не инжектируются в SUTRA; все отрицательные координаты доменных/слойных якорей сдвинуты в положительные (+20500, clamp 32767)
- R4: глобальная гравитация к (0,0,0) удалена (`ANCHOR_X/Y/Z`, `GravityModel`, `apply_gravity_batch*`, `enable_gravity`, `gravity_strength`, `GravityUpdate`) — это был мёртвый код
- R5: `dream_interval` default 0; idle-путь (`DreamScheduler`) — единственный боевой триггер

---

## 18. Инварианты Crystal C0/C1 и Seed Injection (Foundation, 2026-06-14)

| ID | Правило | Статус |
|----|---------|--------|
| **CR1** | Crystal регион: `origin=[26500,26500,26500]`, `size=[4000,4000,1600]`. 8 слоёв по 200 ед. (1600/8). Слой C0 на поверхности (d≈0, z≈26600), C1 = C0+200, ..., C7 = C0+1400. | **HARD** |
| **CR2** | Crystal якоря НЕ участвуют в `match_text()` и subsystem detection. Только position fallback (Path 3 TextPerceptor). | **HARD** |
| **CR3** | Crystal загружается из `seeds/crystal_c0.yaml` при `AnchorSet::load()`. НЕ инжектируются как живые токены — только позиционный справочник. | soft |
| **CR4** | C1 биграммный seed: `token_type=1`, `mass=120`, `temperature=200`, `semantic_weight=0.65`. `stable_id` в диапазоне `0x4800_0001+` (бит 30+27). | soft |
| **CR5** | Биграммы — только алфавитные пары (пробелы/пунктуация разрывают биграмму). Дубликаты объединяются (centroid). | soft |
| **CR6** | `process_command(InjectToken)` = токен в SUTRA без Arbiter routing. Experience накапливается только через `process_and_observe`. Для FileIngester InjectToken всегда идёт через `process_and_observe`. | **HARD** |

**Что сделано (Foundation Фазы 1–2, 2026-06-13/14):**
- C0: 107 фиксированных графем в crystal регионе, детерминированная геометрия
- SEED-TD-01: TextPerceptor Path 3 (crystal_position, char→centroid, semantic_weight=0.75)
- INGEST-01: FileIngester (.md→секции, .axiom.yaml датасет, grow=дефолт, :ingest CLI)
- C1: crystal_bigrams() → C1_z=C0_z+200, InjectToken через process_and_observe
- Проверено на «Стихи 2025.md»: 971 traces, 98.7% match, 8.4× рост vs C0-only
