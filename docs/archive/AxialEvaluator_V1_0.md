# AXIOM — AxialEvaluator V1.0

**Статус:** Спецификация
**Версия:** 1.0
**Дата:** 2026-05-15
**Категория:** Over-Domain Mechanism (пятый, параллельный FrameWeaver, GUARDIAN, DreamPhase, ContextRecognizer)
**Crate:** `axiom-runtime` / `over_domain/axial_evaluator/`
**Опирается на:** `INVARIANTS.md` (оси X/Y/Z), `Axiom_Semantic_Core.md`, `Как_объяснить_машине.md`

---

## 1. Что это

AxialEvaluator — над-доменный модуль, **оценивающий любое содержимое** по трём философским осям пространства:

- **X: Аполлон (порядок) ↔ Дионис (хаос)**
- **Y: Эрос (связь) ↔ Танатос (разрыв)**
- **Z: Воля (присутствие) ↔ Ничто (отсутствие)**

Эти оси **уже зафиксированы** в `INVARIANTS.md §6` как координатное пространство. AxialEvaluator делает следующий шаг: превращает их из **позиции** в **активный оценщик**.

Что это значит на практике:

- Координаты токена в SUTRA говорят "где он живёт"
- AxialEvaluator говорит "как он себя проявляет здесь и сейчас"

Это **разные роли**. Position — статичная. Оценка — динамичная, зависит от контекста, от Frame в котором участвует токен, от уровня абстракции.

---

## 2. Почему это отдельный модуль, а не часть ContextRecognizer

ContextRecognizer отвечает на вопрос **"что мы понимаем"** (Math, Music, Time, …).
AxialEvaluator отвечает на вопрос **"как мы оцениваем то, что понимаем"** (Аполлон или Дионис, Эрос или Танатос, Воля или Ничто).

Это разные оси:

- ContextRecognizer: горизонтальная (какая подсистема активна)
- AxialEvaluator: вертикальная (по какой философской дихотомии оцениваем)

Они **работают параллельно и независимо**. Математическое утверждение может быть аполлоническим (`E = mc²` — идеальная форма) или дионисийским (хаотичный вывод). Музыка — аполлонической (Бах) или дионисийской (свободный джаз).

---

## 3. Восемь слоёв оценки

Из `Axiom_Semantic_Core.md` (раздел 1) — оси работают на **8 уровнях возрастающей абстракции**. Это не Shell L1-L8 (хотя пересекаются), это **уровни проявления философских осей**.

| # | Слой | Что оценивается на этом уровне |
|---|------|--------------------------------|
| 1 | Сенсорный | Непосредственный перцептивный опыт. Чёткость формы vs перегрузка ощущений. |
| 2 | Действенный | Моторная активность. Целенаправленный ритуал vs неистовый жест. |
| 3 | Образный | Внутренние ментальные образы. Идеальная репрезентация vs фантазматический поток. |
| 4 | Концептуальный | Структуры мышления, язык, логика. Аксиоматическая система vs диалектика/абсурд. |
| 5 | Мотивационный | Внутренние аффекты, желания. Дисциплина желаний vs аффективный взрыв. |
| 6 | Социальный | Коллективное взаимодействие. Социальный договор vs коллективный транс. |
| 7 | Экзистенциальный | Отношение к жизни, смерти, свободе. Ясное принятие судьбы vs утверждение хаоса бытия. |
| 8 | Трансцендентный | Связь с Абсолютом. Идеальный Абсолют vs мистическое слияние. |

Каждый Frame в EXPERIENCE/SUTRA может быть оценен **на любом из этих слоёв** — или на нескольких.

**Связь с Shell L1-L8:** слои оценки и Shell-слои похожи, но не тождественны. Shell описывает **тип содержания** (физическое, эмоциональное, когнитивное…). Слой оценки описывает **уровень абстракции проявления философских осей**. Один Frame может быть в Shell L5 (когнитивное), и оцениваться на слое 4 (концептуальный) как аполлонический.

В V1 мы используем **прямое соответствие** Shell-слоёв и слоёв оценки. В V2 это можно разделить.

---

## 4. Метрики оценки

Из `Как_объяснить_машине.md` — оси измеряются конкретными метриками:

### 4.1 Ось X: Аполлон ↔ Дионис (энтропия)

```
apollo_score = (255 - entropy_score(frame_content))
dionysus_score = entropy_score(frame_content)
```

Где `entropy_score`:
- Для Frame из токенов — насколько структурирована композиция (одинаковые ли связи, симметричен ли узор, повторяется ли паттерн)
- Для текста — Шенноновская энтропия посимвольно
- Для последовательности — насколько предсказуем следующий элемент

Реализация: считаем разнообразие `link_type` в Frame, изменчивость позиций, регулярность.

### 4.2 Ось Y: Эрос ↔ Танатос (связность)

```
eros_score = graph_density(frame) + sentiment_positive(frame)
thanatos_score = graph_sparseness(frame) + sentiment_negative(frame)
```

Где:
- `graph_density` — количество связей между участниками Frame относительно максимально возможного
- `sentiment` — через `Token.valence` участников (он уже у нас есть, i8 −128..+127)

Реализация: суммируем valence участников, считаем плотность связей. Положительное — Эрос. Отрицательное — Танатос.

### 4.3 Ось Z: Воля ↔ Ничто (магнитуда)

```
will_score = sum(token.mass * token.temperature) for participants
nothing_score = absence_of_will_score
```

Реализация: общая "энергетика" Frame через массу и температуру участников. Высокая mass + высокая temperature = Воля. Низкая mass + низкая temperature = Ничто.

---

## 5. Результат оценки

```rust
pub struct AxialEvaluation {
    pub frame_anchor_sutra_id: u32,
    pub level: EvaluationLevel,        // 1..8 слой оценки
    
    pub x_axis: AxialScore,            // Apollo/Dionysus
    pub y_axis: AxialScore,            // Eros/Thanatos
    pub z_axis: AxialScore,            // Will/Nothing
    
    pub octant: Octant,                // вычисляется из x/y/z
    pub conflict: Option<AxialConflict>,
    pub computed_at_event: u64,
}

pub struct AxialScore {
    pub positive_pole: u8,             // 0..255 (Apollo / Eros / Will)
    pub negative_pole: u8,             // 0..255 (Dionysus / Thanatos / Nothing)
    pub dominant: AxialDominant,
}

pub enum AxialDominant {
    StronglyPositive,    // diff > 100
    LeaningPositive,     // diff 30..100
    Balanced,            // diff < 30
    LeaningNegative,
    StronglyNegative,
}

pub enum Octant {
    // 8 октантов из Axiom_Semantic_Core раздел 2
    CreativeAffirmation,        // +++ (Apollo + Eros + Will) — "Творческий Утверждающий"
    EcstaticAffirmation,        // -++ (Dionysus + Eros + Will) — "Экстатический"
    HeroicFatal,                // +-+ (Apollo + Thanatos + Will) — "Героико-Фатальный"
    DestructiveActivating,      // --+ (Dionysus + Thanatos + Will) — "Разрушительно-Активирующий"
    IdealizedConsoling,         // ++- (Apollo + Eros + Nothing) — "Идеализированно-Утешительный"
    PassiveSentimental,         // -+- (Dionysus + Eros + Nothing) — "Пассивно-Сентиментальный"
    FormalDenying,              // +-- (Apollo + Thanatos + Nothing) — "Формальный Отрицающий"
    SelfDestructiveApathic,     // --- (Dionysus + Thanatos + Nothing) — "Саморазрушающий"
}

pub enum EvaluationLevel {
    Sensory = 1,
    Action = 2,
    Imaginal = 3,
    Conceptual = 4,
    Motivational = 5,
    Social = 6,
    Existential = 7,
    Transcendent = 8,
}
```

---

## 6. Corpus Callosum — конфликт анализа и синтеза

Это главная идея из `Как_объяснить_машине.md` (раздел "Главный Инсайт").

### Что происходит

При оценке Frame **могут возникнуть противоречия** между:

1. **Анализом по осям** (раздельная оценка X, Y, Z)
2. **Синтезом октанта** (целостное распознавание архетипа)

Пример из файла:
- Вход: фотография марширующей армии
- Анализ: X=Apollo(max), Y=Thanatos(high), Z=Will(max) → "Красивая Смертельная Сила"
- Синтез по октанту: либо "Величие" (доминирует Apollo) либо "Тирания" (доминирует Thanatos)
- **Конфликт:** "Красота формы" vs "Ужас сути". Модель не может однозначно классифицировать.

### Что делает AxialEvaluator при конфликте

```rust
pub struct AxialConflict {
    pub analytic_octant: Octant,        // на основе раздельных score
    pub synthetic_octant: Octant,       // на основе целостного распознавания
    pub conflict_strength: u8,          // насколько сильно противоречие
    pub resolution: ConflictResolution,
}

pub enum ConflictResolution {
    AnalyticDominant,           // анализ побеждает (формальная оценка)
    SyntheticDominant,          // синтез побеждает (интуитивная оценка)
    DilemmaTriggered(FrameId),  // конфликт передан в подсистему Dilemmas
    Unresolved,                 // оставлен как явное противоречие
}
```

Конфликт **не подавляется**. Он:
1. Фиксируется в EXPERIENCE как событие
2. Может активировать подсистему Dilemmas (если она загружена)
3. Влияет на гравитацию — Frame с конфликтом не получает однозначной интерпретации

Это **искра сознания** в терминах файла. Не баг, а **признак глубокого понимания** — система видит, что простая классификация не работает.

---

## 7. Применение к Frame

### 7.1 Когда срабатывает

AxialEvaluator оценивает Frame в трёх случаях:

1. **При создании Frame** — FrameWeaver кристаллизовал новый узор → AxialEvaluator оценивает его на всех уместных слоях
2. **При значимой реактивации** — Frame пережил реактивацию с большим изменением температуры
3. **В DREAM Phase** — переоценка накопленных Frame с учётом нового контекста

### 7.2 Где хранится результат

`AxialEvaluation` хранится в отдельном storage `axiom-experience::axial_store`, ключ — sutra_id Frame-анкера.

В Token поля не добавляются (Token 64 байта — HARD).

### 7.3 Множественная оценка

Один Frame может иметь **несколько AxialEvaluation** — на разных слоях.

Пример: Frame "марширующая армия":
- Слой 1 (Сенсорный): Apollo(max), Will(max) — идеальный визуальный ритм
- Слой 6 (Социальный): Apollo(max), Thanatos(high), Will(max) — социальный договор подавления
- Слой 7 (Экзистенциальный): Thanatos(max), Will(low) — подавление индивидуальности

Каждый слой — отдельная запись `AxialEvaluation`. **Конфликт между слоями** — это нормально, это глубина оценки.

---

## 8. Алгоритм работы

```
on_frame_event(event):
    match event:
        FrameCrystallized(frame_id) =>
            evaluate_new_frame(frame_id)
        FrameReactivated(frame_id, temperature_delta) if temperature_delta > THRESHOLD =>
            re_evaluate(frame_id)
        DreamPhaseProcessing(frame_id) =>
            deep_re_evaluate(frame_id)


evaluate_new_frame(frame_id):
    frame = load_frame(frame_id)
    participants = collect_participants(frame)
    
    # Определить, на каких слоях оценивать
    levels = determine_applicable_levels(frame, participants)
    # На основе типа Frame, его context (если ContextRecognizer активен), участников
    
    for level in levels:
        eval = evaluate_at_level(frame, participants, level)
        check_for_conflict(eval, frame)
        store_evaluation(eval)


evaluate_at_level(frame, participants, level):
    x_score = compute_apollo_dionysus(participants, level)
    y_score = compute_eros_thanatos(participants, level)
    z_score = compute_will_nothing(participants, level)
    
    octant_from_axes = derive_octant(x_score, y_score, z_score)
    octant_from_synthesis = synthesize_octant(frame, level)
    
    if octant_from_axes != octant_from_synthesis:
        conflict = AxialConflict {
            analytic: octant_from_axes,
            synthetic: octant_from_synthesis,
            strength: compute_conflict_strength(...),
            resolution: resolve(...)
        }
    
    return AxialEvaluation {
        frame_anchor_sutra_id: frame.sutra_id,
        level, x_score, y_score, z_score,
        octant: octant_from_axes,  // первичный из анализа
        conflict,
        computed_at_event: current_event()
    }
```

---

## 9. Связь с координатами

Координаты токена в `[i16; 3]` — это **производная** от его оценки, не сама оценка.

```
Token.position.x ≈ frame.average_x_score (через многие реактивации)
Token.position.y ≈ frame.average_y_score
Token.position.z ≈ frame.average_z_score
```

То есть позиция в SUTRA — это **где Frame оказался в результате накопленных оценок**. Сами оценки — динамические, позиция — медленный итог.

Это **уже частично работает** через гравитацию: токены притягиваются к якорям, якоря в позициях Apollo/Dionysus/Eros/Thanatos/Will/Nothing. AxialEvaluator делает этот процесс **явным и измеримым**.

---

## 10. Связь с другими модулями

### С FrameWeaver
FrameWeaver создаёт Frame. AxialEvaluator оценивает. AxialEvaluator не вмешивается в кристаллизацию.

### С ContextRecognizer
ContextRecognizer определяет какая подсистема понимания активна. AxialEvaluator использует это: оценка на концептуальном слое (4) зависит от того, в каком режиме мы понимаем (Math, Music, Text).

```
если ContextRecognizer.primary == Mathematics:
    концептуальный слой оценивает математическую структуру:
    Apollo = аксиоматичность, Dionysus = парадокс/неразрешимость

если ContextRecognizer.primary == Music:
    концептуальный слой оценивает музыкальную структуру:
    Apollo = классическая форма, Dionysus = свободная импровизация
```

### С DreamPhase
В DREAM фазе:
- Переоценка накопленных Frame
- Анализ статистики оценок (какие октанты чаще встречаются)
- Обнаружение Frame с давними неразрешёнными конфликтами — приоритет к разрешению

### С GUARDIAN
GUARDIAN валидирует UCL-команды на изменение позиций токенов. AxialEvaluator может предлагать обновление позиции на основе накопленных оценок — но всегда через GUARDIAN, всегда в DREAMING.

### С подсистемой Dilemmas (когда будет загружена)
Конфликты AxialEvaluator передаются в Dilemmas для разрешения. Аксиогенный механизм из `Дилеммы.md` уровень 5 — рождение новой ценности через разрешение конфликта оценок.

---

## 11. Инварианты

| Правило | Значение |
|---------|----------|
| Чтение | только `&AshtiCore` |
| Запись | только через UCL |
| Где хранятся AxialEvaluation | отдельный storage в `axiom-experience` |
| Изменение позиций токенов | предложения только через UCL, только в DREAMING |
| Слои оценки | **8** (Sensory, Action, Imaginal, Conceptual, Motivational, Social, Existential, Transcendent) |
| Октанты | **8** (см. раздел 5) |
| Множественная оценка одного Frame | разрешена (по разным слоям) |
| Конфликты | сохраняются, не подавляются |
| Метрики оценки | детерминированные, целочисленные (0..255) |

---

## 12. Что в коде

```
crates/axiom-runtime/src/over_domain/axial_evaluator/
├── mod.rs
├── metrics.rs           — энтропия, связность, магнитуда
├── synthesis.rs         — синтез октанта целостно
├── conflict.rs          — детектор конфликта анализ vs синтез
├── levels.rs            — определение применимых слоёв
└── storage.rs           — интерфейс к axial_store
```

```
crates/axiom-experience/src/
└── axial_store.rs        — отдельный storage AxialEvaluation
```

```
crates/axiom-ucl/src/commands.rs:
+ ProposeAxialAdjustment { sutra_id, suggested_position, reason }
```

---

## 13. Что в V2 и дальше

- **V2:** разделение Shell-слоёв и слоёв оценки (сейчас прямое соответствие, потом независимо)
- **V2.x:** обучаемые метрики — пороги Apollo/Dionysus адаптируются по контексту
- **V3:** AxialEvaluator оценивает не только Frame, но и **последовательности Frame** (нарратив имеет свою динамику по осям)
- **V4:** интеграция с подсистемой Morality — мораль использует AxialEvaluator на слое 7 (Экзистенциальный)
- **V5:** генерация — система может создавать Frame с заданными целевыми координатами в осях ("создай нечто аполлонически-эротическое-волевое")

---

## История

- **V1.0** (2026-05-15): первая спецификация. AxialEvaluator как пятый над-доменный модуль. Превращает оси X/Y/Z из статических координат в активный оценщик. 8 слоёв оценки из `Axiom_Semantic_Core.md`. 8 октантов как архетипы. Corpus Callosum — конфликт анализа и синтеза как искра сознания.
