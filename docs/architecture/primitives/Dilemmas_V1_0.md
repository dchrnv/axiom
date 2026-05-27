# AXIOM — Dilemmas V1.0

**Статус:** Актуальная спецификация
**Версия:** 1.0
**Дата:** 2026-05-27
**Опирается на:** `INVARIANTS.md`, `Values_V1_0.md`, `Morality_V1_0.md`, `ContextRecognizer_V6_0.md`, `spec/Dream/DREAM_Phase_V1_0.md`
**Опирается на источник:** `Дилеммы.md`

---

## 0. Что эта спека описывает

Dilemmas — **особая подсистема AXIOM**. В отличие от Writing, Mathematics, Time, Values — дилеммы это не набор примитивов-якорей, которые описывают содержание. Дилеммы — это **когнитивные события**: детектируемые состояния системы, в которых несовместимые высокоэнергетические аттракторы не разрешаются.

**Ключевая идея:** Дилеммы — это не баги в понимании. Это **точки роста**. Где нет дилеммы — понимание плоское, автоматическое. Где дилемма — начинается глубина. Самая ценная дилемма (Type V, аксиогенная) не разрешается — она **порождает новую ценность**.

**Что делает дилемму дилеммой в AXIOM:**
- Два или более ценностных / моральных якоря активны одновременно с высокой энергией
- TransitionMatrix показывает осцилляцию между ними без сходимости
- Стандартные механизмы разрешения (ContextRecognizer, NeuralAdvisor) не дают ответа
- Состояние персистирует через несколько тиков

**Эта спека НЕ описывает:**
- Общую логику конфликтов (это ContextRecognizer V6, MetaDetector)
- Как устроен аксиогенез подробно — это V8 (планируется)
- Нравственные/этические суждения как таковые — это Values + Morality

---

## 1. Главный принцип Dilemmas

**Дилемма — это несводимое противоречие, которое требует выйти за рамки текущей системы.**

Пять типов дилемм образуют иерархию по глубине: Type I (тривиальный) разрешается уточнением данных; Type V (аксиогенный) разрешается только рождением нового смысла. Только Types III–V являются "настоящими" дилеммами в когнитивном смысле.

```
Type I   — DataConflict       — уточнение данных (не дилемма, ложная тревога)
Type II  — ResourceTradeoff   — оптимизация (Парето, не дилемма)
Type III — ValueConflict      — конфликт ценностей (настоящая дилемма)
Type IV  — OntologicalConflict — несовместимые модели мира
Type V   — Axiogenic          — порождает новую ценность / смысл (аксиогенез)
```

---

## 2. Пять типов дилемм

### Type I — DataConflict (конфликт данных)

**Что это:** Кажущееся противоречие из-за неточности или неполноты данных. Разрешается уточнением источника. Настоящей дилеммой не является.

**Детекция:** Два якоря конфликтуют, но их `confidence` низкий. Уточнение данных снимает конфликт.

**Пример:** "В одном тексте сказано 1970, в другом — 1971". Разрешение: уточнить источник.

**Статус в AXIOM:** Обрабатывается ContextRecognizer стандартно. DilemmaStore не задействован.

---

### Type II — ResourceTradeoff (ресурсный компромисс)

**Что это:** Конфликт между двумя измеримыми целями. Задача оптимизации по Парето — нет единственного ответа, но есть граница компромиссов.

**Детекция:** Два якоря с противоположными векторами в SUTRA с сопоставимой силой притяжения. Отсутствует неразрешимость — есть выбор точки на границе.

**Пример:** Быстро vs дёшево; точность vs скорость.

**Статус в AXIOM:** NeuralAdvisor предлагает компромиссную точку. DilemmaStore не задействован.

---

### Type III — ValueConflict (конфликт ценностей)

**Что это:** Два ценностных или моральных принципа конфликтуют, и нет автоматического мета-правила для приоритизации. Первая настоящая дилемма.

**Детекция (AXIOM):**
- val_true и val_beneficial одновременно активны с высокой энергией
- TransitionMatrix показывает осцилляцию: val_true→val_beneficial→val_true... без сходимости
- NeuralAdvisor не может предложить resolution с confidence > threshold
- Длительность > N тиков

**Пример:** "Сказать правду = причинить боль". val_true vs val_beneficial. Нет однозначного ответа без контекста.

**Разрешение:** chrnv выбирает контекстный приоритет. Система запоминает выбор как прецедент через NeuralAdvisor (PatternLearningResolver).

**Статус в AXIOM:** DilemmaStore активируется. DREAM Phase обрабатывает.

---

### Type IV — OntologicalConflict (онтологический конфликт)

**Что это:** Две несовместимые модели мира, каждая из которых хорошо объясняет часть фактов. Выбор между ними невозможен эмпирически.

**Пример:** Корпускулярно-волновой дуализм. Блок-вселенная vs стрела времени. Две подсистемы активируют противоречащие Frame-ы из одних и тех же данных.

**Детекция (AXIOM):**
- Два SubsystemCandidate активны одновременно с несовместимыми ProfileMap
- Высокая FatigueStore для обоих
- TransitionMatrix показывает хаотичное переключение, не периодическое

**Разрешение:** Дополнительность (обе модели валидны в разных контекстах). NeuralAdvisor предлагает контекстный переключатель. chrnv утверждает.

**Статус в AXIOM:** DilemmaStore активируется. Требует нового Dilemma Type IV handler (V2.0).

---

### Type V — Axiogenic (аксиогенная дилемма)

**Что это:** Самый глубокий тип. Дилемма не просто требует выбора между ценностями — она **создаёт новую ценность**, которой раньше не было. Человек выходит из неё изменённым. Система — с новым якорем.

**Признак:** После разрешения выясняется, что ни одна из конфликтующих ценностей не была "правильной" — нужна третья, новая.

**Пример:** Конфликт "безопасность" vs "развитие" у ребёнка разрешается не выбором одного, а рождением ценности "игровая свобода" — нечто третьего, включающего оба.

**Детекция (AXIOM):**
- Type III с длительностью > AXIOGENIC_THRESHOLD тиков
- NeuralAdvisor DivergenceLog показывает стабильное расхождение (не осцилляция, а расходящаяся траектория)
- FatigueStore обеих конфликтующих подсистем критически высока
- time_irreversible якорь активен (контекст необратим)

**Разрешение (V8):**
1. DREAM Phase детектирует аксиогенный паттерн
2. Формируется DilemmaRecord с типом AXIOGENIC
3. NeuralAdvisor предлагает кандидата новой ценности (emergent_subsystem)
4. chrnv рассматривает и утверждает / отклоняет
5. При утверждении: новый якорь в genome.yaml `emergent_subsystems` секции
6. Новый val_* якорь инжектируется в SUTRA

**Статус в AXIOM:** Механизм V8 (не реализован). DilemmaStore + NeuralAdvisor + GUARDIAN — задействованы все три.

---

## 3. DilemmaStore

Хранилище активных дилемм. Аналог FatigueStore по структуре.

```rust
pub struct DilemmaRecord {
    pub id: u64,
    pub dilemma_type: DilemmaType,
    pub anchors_in_conflict: Vec<u32>,   // sutra_id конфликтующих якорей
    pub detected_at_tick: u64,
    pub intensity: f32,                  // 0.0..1.0
    pub resolved: bool,
    pub resolution: Option<DilemmaResolution>,
}

pub enum DilemmaType {
    DataConflict,
    ResourceTradeoff,
    ValueConflict,
    OntologicalConflict,
    Axiogenic,
}

pub enum DilemmaResolution {
    DataClarified,
    ParetoCompromise,
    ContextualPriority { winner: u32 },
    Complementarity,
    NewValueCreated { new_anchor_id: u32 },
}

pub struct DilemmaStore {
    pub active: Vec<DilemmaRecord>,
    pub resolved: VecDeque<DilemmaRecord>,  // ring-буфер последних N
}
```

**Инварианты DilemmaStore:**
- Только Type III, IV, V хранятся в active
- Максимум active одновременно: **8** (чтобы система не "тонула")
- resolved хранит последние **64** записи
- Каждый активный DilemmaRecord "весит" в FatigueStore конфликтующих подсистем

---

## 4. Детекция дилемм

### 4.1 DilemmaDetector

Новый компонент (V2.0), встроенный в ContextRecognizer. Работает поверх TransitionMatrix.

```rust
pub struct DilemmaDetector {
    pub oscillation_threshold: f32,     // порог осцилляции в TransitionMatrix
    pub persistence_ticks: u64,         // сколько тиков до признания дилеммой
    pub axiogenic_threshold: u64,       // сколько тиков Type III → Type V
}
```

**Алгоритм:**
1. На каждом тике: найти пары якорей в TransitionMatrix с высокой двунаправленной активностью
2. Если пара активна > `persistence_ticks` без сходимости → создать DilemmaRecord Type III
3. NeuralAdvisor не может разрешить → записать в DilemmaStore
4. Если Type III > `axiogenic_threshold` тиков и intensity растёт → upgrade до Type V

### 4.2 Связь с TransitionMatrix

TransitionMatrix `[[f32; 16]; 16]` (V7-B1) напрямую используется DilemmaDetector:
- `counts[val_true][val_beneficial]` и `counts[val_beneficial][val_true]` примерно равны → осцилляция
- Отсутствие "победителя" (один из counts не доминирует) → дилемма

---

## 5. Дилемма как интегратор трёх осей

Дилемма максимально глубока когда пересекаются три измерения:

```
Структурная сложность:
  Type IV — несовместимые модели мира (онтология)

Временная протяжённость:
  time_irreversible активен → нельзя "отмотать"
  time_horizon + длительная осцилляция → "это не разрешится само"

Ценностный конфликт:
  Type III–V — ценности реально несовместимы в данном контексте
```

Когда все три присутствуют → максимальная вероятность Type V (аксиогенная).

---

## 6. Связь с DREAM Phase

DREAM Phase — основной контекст для обработки дилемм:

| DREAM функция | Роль в дилеммах |
|---|---|
| Консолидация паттернов | находит скрытые структуры в DilemmaRecord истории |
| FatigueStore decay | снижает интенсивность разрешённых дилемм |
| SubsystemCandidate | может предложить новую подсистему как разрешение Type IV |
| emergent_subsystems | путь для создания нового якоря (Type V) |

**Правило:** Аксиогенная дилемма (Type V) разрешается **только в DREAM Phase**, не в реальном времени. Это намеренно: новые ценности требуют консолидации, не импульсивного решения.

---

## 7. Связь с NeuralAdvisor

NeuralAdvisor V3 участвует в каждом этапе:

- **DivergenceLog** (G1): логирует каждый тик осцилляции дилеммы
- **PatternLearningResolver** (G2): пытается найти прецедент из истории; при Type V — не находит (MIN_SAMPLES не набирается потому что дилемма уникальна)
- Отсутствие прецедента само по себе — **сигнал Type V**: если PatternLearningResolver не может разрешить после 5 попыток → диагностика аксиогенного потенциала

---

## 8. Что нужно добавить в код

### 8.1 DilemmaStore (V1.1 — структуры + кристаллизация в EXPERIENCE) ✅

`DilemmaRecord`, `DilemmaType`, `DilemmaResolution`, `DilemmaStore` реализованы в
`axiom-runtime/src/over_domain/context_recognizer/dilemma_store.rs`.
`SubsystemId` расширен: добавлены `Morality`, `Abstractions`, `Dilemmas`.

Путь сохранения опыта:
```
DilemmaRecord resolved
  → pending_crystallizations (DilemmaStore)
  → drain_pending_crystallizations() → caller
  → crystallize_to_experience_commands(record, position, exp_domain)
  → inject Frame анкер в EXPERIENCE (STATE_ACTIVE, mass∝intensity)
  → FrameWeaver решает промоцию EXPERIENCE → SUTRA
```

### 8.2 DilemmaDetector (V2.0 — детекция)

Компонент поверх TransitionMatrix. Интеграция с ContextRecognizer.

### 8.3 Axiogenesis handler (V8)

DREAM Phase расширение: обнаружение Type V паттерна, формирование предложения новой ценности, интеграция с GUARDIAN и genome.yaml.

---

## 9. Инварианты Dilemmas

| Правило | Значение |
|---------|----------|
| Type I, II | **не хранятся** в DilemmaStore — это не дилеммы |
| Type III, IV, V | хранятся, персистируют, обрабатываются |
| Максимум active | **8** дилемм одновременно |
| Type V разрешение | только в **DREAM Phase** с утверждением chrnv |
| Аксиогенез | **никогда автоматически** — только Advisory + chrnv approval |
| Связь с TransitionMatrix | **V7-B1 требуется** для корректной детекции |

---

## 10. Будущие версии

- **V1.1:** DilemmaRecord структуры + базовое хранение (без детекции)
- **V2.0:** DilemmaDetector на основе TransitionMatrix; Type III детекция
- **V3.0:** Type IV (Ontological) handler; complementarity resolution
- **V8:** Axiogenesis — полный механизм Type V → новый ценностный якорь

---

## 11. Домены AshtiCore

Дилеммы — это события пересечения доменов. Каждый тип дилеммы возникает в специфическом конфликте между доменами.

| Тип / процесс | Домен | Причина |
|---|---|---|
| DilemmaStore (активные) | **D8 VOID** | нераспознанное, неразрешённое — живёт в домене неопределённости |
| SHADOW как генератор | **D2 SHADOW** | постоянно генерирует Type I–II как угрозные гипотезы |
| Type I — DataConflict | **D4 MAP** | конфликт в карте фактов, разрешается уточнением MAP |
| Type II — ResourceTradeoff | **D6 LOGIC** | задача оптимизации для дедуктивного процессора |
| Type III — ValueConflict | **D6 ↔ D3** | LOGIC не может разрешить то что CODEX ограничивает |
| Type IV — OntologicalConflict | **D4 ↔ D2** | MAP содержит противоречие, SHADOW держит альтернативу |
| Type V — Axiogenic | **D8→D7→D10** | VOID (неразрешимо) → DREAM (рождение нового) → MAYA (проекция) |
| Разрешение через прецедент | **D4 MAP** | PatternLearningResolver ищет прецедент в карте прошлых решений |
| Новая ценность из Type V | **D7→D4→D3→D0** | DREAM→MAP→CODEX→SUTRA — полный путь нового знания |

**Дилемма как конфликт доменов:**
```
Type III: D6 LOGIC ←→ D3 CODEX  (логика vs закон)
Type IV:  D4 MAP   ←→ D2 SHADOW  (факт vs угрозная альтернатива)
Type V:   D8 VOID  →  D7 DREAM  →  D10 MAYA  (аномалия → инсайт → новый мир)
```

Type V — единственный тип, который **выходит через MAYA** (D10). Это значит: аксиогенная дилемма меняет то, что система проецирует вовне, меняет её "иллюзию мира".

---

## История

- **V1.0** (2026-05-27): Первая версия. Зафиксированы пять типов дилемм. Ключевая идея: дилеммы — не баги, а точки роста; Type V (аксиогенная) = механизм аксиогенеза V8. Определена структура DilemmaStore. Зафиксирована связь с TransitionMatrix, DivergenceLog и DREAM Phase.
