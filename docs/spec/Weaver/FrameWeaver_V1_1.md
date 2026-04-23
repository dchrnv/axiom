# AXIOM MODULE SPECIFICATION: FRAMEWEAVER V1.1

**Статус:** Актуальная спецификация (core)
**Версия:** 1.1.0
**Дата:** 2026-04-22
**Codename:** "First Weaver"
**Назначение:** Сборка и кристаллизация реляционных (синтаксических) узоров
**Crate:** `axiom-runtime` (модуль `over_domain/weavers/frame.rs`)
**Категория:** Weaver (Over-Domain Layer)
**Модель времени:** COM `event_id`
**Связанные спеки:** Over-Domain Layer V1.1, Connection V5.0, Token V5.2, Shell V3.0, Ashti_Core V2.1, GENOME V1.0, GUARDIAN V1.0, Memory Persistence V1.0

---

## 0. Изменения относительно V1.0

**V1.1 — онтологическая коррекция.** V1.0 содержала фундаментальную ошибку: Frame кристаллизовался в SUTRA (домен 100). Это противоречит онтологии AXIOM:

- **SUTRA** — нить, вечная истина, первичные сущности (anchor-токены, оси, аксиомы).
- **EXPERIENCE** — накопленный опыт, история проявлений, кристаллизованные скиллы.
- **MAYA** — проявление, "сейчас", живая сборка.

Frame — это **закристаллизованный опыт структурирования**, а не первичная истина. Его естественный дом — EXPERIENCE (домен 109), не SUTRA.

### Что изменилось

1. `domain_id` анкера Frame: **100 → 109** (EXPERIENCE).
2. `state` анкера: **STATE_LOCKED → STATE_ACTIVE** (Frame подчиняется физике EXPERIENCE: живёт, стареет, усиливается при повторном использовании).
3. Жизненный цикл получил **второй этап**: возможная промоция Frame из EXPERIENCE в SUTRA для особо устойчивых узоров через GUARDIAN/CODEX (раздел 5.4).
4. `reserved_gate` Connection для метаданных о доменах участников — более не костыль, а естественное использование (EXPERIENCE по природе помнит историю).
5. DAG-инвариант смягчён: в EXPERIENCE допустимы циклы; `cycle_handling` становится рекомендацией качества, не жёстким запретом.
6. Формулировка "ASHTI как зеркала" уточнена: у зеркал два источника — SUTRA (истина) и EXPERIENCE (опыт).

Всё остальное (структура 8 синтаксических слоёв, `SemanticContributionTable` категория 0x08, traits, правила кристаллизации, метрики) **остаётся без изменений**.

---

## 1. Назначение и философия

### 1.1 Что такое Frame

**Frame** — реляционная структура, фиксирующая "кто-что-кому" в конкретном акте: подлежащее, сказуемое, дополнение, обстоятельство — и любые более глубокие синтаксические/семантические роли.

Frame — это **узор**, не свойство. Один токен ("книга") может играть разные роли в разных Frame: подлежащее в "книга лежит", прямое дополнение в "читаю книгу". Сам токен от этого не меняется. Меняется его роль внутри конкретного Frame.

### 1.2 Почему отдельный компонент

Синтаксическая роль — **реляционна** и не сводится к свойству токена. Поэтому она не может жить в Shell (который описывает свойство). Она может жить только как **граф связей**, оформленный в специальную структуру.

В архитектуре AXIOM домены — зеркала, они не сборщики структур. Сборка реляционных узоров — функция отдельного над-доменного компонента: **FrameWeaver**.

### 1.3 Онтологическое место Frame

Frame — это **закристаллизованный опыт структурирования**.

- Он рождается как живой узор в **MAYA** (место "сейчас").
- Стабилизируется и сохраняется в **EXPERIENCE** (место накопленного опыта).
- В исключительных случаях может промоутироваться в **SUTRA** (место вечной истины), если узор проявил себя как фундаментальная структура, заслуживающая статуса первичной сущности.

Опыт — природная среда Frame. SUTRA — финальная и редкая точка, достижимая только через длительную валидацию (раздел 5.4).

### 1.4 Жизненный цикл Frame

```
[MAYA] узор рождается как живая конфигурация связей
   ↓ (FrameWeaver.scan)
[FrameWeaver] распознаёт кандидата в Frame, формирует CrystallizationProposal
   ↓ (DREAM-инспекция: правила из Genome + Schema Configuration)
[DREAM] решает: кристаллизовать / отклонить / отложить
   ↓ (если кристаллизовать)
[GUARDIAN] финальная проверка на нарушение GENOME/CODEX
   ↓ (если разрешено)
[EXPERIENCE] Frame сохраняется как анкер-токен + типизированные связи
   ↓ (живёт, стареет, усиливается при повторном использовании)
   ↓ (по запросу любого домена)
[любой домен] получает Frame и разворачивает в свою рабочую среду

   [опциональный второй этап]

[EXPERIENCE] Frame проявил длительную устойчивость и универсальность
   ↓ (промоция через GUARDIAN/CODEX)
[SUTRA] Frame становится первичной сущностью (anchor)
```

**Ключевой принцип:** MAYA генерирует, DREAM отбирает, EXPERIENCE хранит опыт, SUTRA хранит истину, любой домен потребляет.

---

## 2. Структура Frame

### 2.1 Frame в EXPERIENCE

Frame в EXPERIENCE — это **анкер-токен** + набор типизированных связей к токенам-участникам.

**Анкер-токен Frame** — обычный `Token V5.2`, отличается значением `type_flags`:

```
TOKEN_FLAG_FRAME_ANCHOR = 0x0010
```

Расшифровка анкера:

| Поле          | Значение для Frame-анкера                                    |
|---------------|--------------------------------------------------------------|
| `sutra_id`    | новый, уникальный (генерируется при кристаллизации)          |
| `domain_id`   | **109 (EXPERIENCE)** — естественный дом накопленного опыта   |
| `type_flags`  | `TOKEN_FLAG_FRAME_ANCHOR` (+ опционально категория Frame)    |
| `position`    | вычисляется как центр масс позиций участников                |
| `lineage_hash`| хэш набора (sutra_id участников) — для дедупликации          |
| `mass`        | равна количеству участников × коэффициент кристаллизации     |
| `temperature` | начальная (по умолчанию 128) — подчиняется физике EXPERIENCE |
| `state`       | **`STATE_ACTIVE` (1)** — Frame живёт, стареет, реактивируется|

**Почему `STATE_ACTIVE`, а не `STATE_LOCKED`:** Frame в EXPERIENCE — это опыт, а опыт не заморожен. Узоры, к которым часто возвращаются, усиливаются (рост температуры при повторной активации). Узоры, не востребованные, затухают. Это и есть нормальная физика опыта — и именно её ожидает EXPERIENCE.

**Связи Frame** — обычные `Connection V5.0`, отличаются значением `link_type`:

- `source_id` = sutra_id анкер-токена Frame
- `target_id` = sutra_id участника
- `domain_id` = **109** (EXPERIENCE — связи живут вместе с анкером)
- `link_type` ∈ синтаксической категории (см. раздел 3)
- `flags` = `FLAG_ACTIVE`

### 2.2 Метаданные о доменах участников

Участники Frame могут принадлежать разным доменам (по их природе). Эту информацию сохраняем в поле `reserved_gate: [u8; 14]` в Connection V5.0:

```
reserved_gate[0..2]   = participant_origin_domain (u16, big-endian)
reserved_gate[2..4]   = participant_role_id (u16, big-endian)
reserved_gate[4..14]  = зарезервировано
```

В V1.0 это выглядело как костыль (мы обходили ограничение `domain_id` у Connection). В V1.1 это **нормальное использование**: EXPERIENCE по природе своей есть история, и хранение информации "откуда пришёл этот участник" — её прямая функция. Не костыль, а штатный механизм памяти.

### 2.3 Категории Frame

Анкер может нести подкатегорию через старшие биты `type_flags`:

```
FRAME_CATEGORY_MASK     = 0xFF00
FRAME_CATEGORY_SYNTAX   = 0x0100  // синтаксический Frame (V1.1 — единственный)
FRAME_CATEGORY_SEMANTIC = 0x0200  // зарезервировано (будущее)
FRAME_CATEGORY_PRAGMA   = 0x0300  // зарезервировано (будущее)
```

В V1.1 реализуется только `FRAME_CATEGORY_SYNTAX`.

### 2.4 Промоутнутые Frame в SUTRA

Редкий путь: Frame, прошедший длительную валидацию (раздел 5.4), может быть **скопирован** из EXPERIENCE в SUTRA как новая первичная сущность. В этом случае:

- В SUTRA создаётся **новый** анкер-токен с новым `sutra_id`, `domain_id = 100`, `state = STATE_LOCKED`.
- Связи копируются с новыми ID, `domain_id = 100`.
- В EXPERIENCE оригинал **не удаляется** — он продолжает жить как след.
- В SUTRA-анкере устанавливается дополнительный флаг `TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE = 0x0020`.

Промоция — однонаправленная. Обратного пути SUTRA → EXPERIENCE не существует: истина не забывается обратно в опыт.

---

## 3. Синтаксическая категория `link_type`

### 3.1 Размещение в пространстве `link_type`

В Connection V5.0 `link_type: u16` уже разделён на 256 категорий × 256 типов. Существующие категории (Shell V3.0):

- 0x01 Structural
- 0x02 Semantic
- 0x03 Causal
- 0x04 Experiential
- 0x05 Social
- 0x06 Temporal
- 0x07 Motor

**Добавляется новая категория:**

```
0x08 Syntactic — синтаксические/реляционные роли
```

### 3.2 Расширенный перечень синтаксических подтипов (8 слоёв)

Синтаксические связи организованы по **8 слоям глубины** — от поверхностной грамматики до глубокой смысловой структуры. Это параллельно 8 семантическим слоям Shell, но **не идентично им**: Shell описывает что токен есть, синтаксические слои — как он соотносится в Frame.

#### Слой S1 — Ядерные роли (Core Arguments)
Базовая структура высказывания: "кто что делает с чем".

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0801      | SUBJECT          | Подлежащее, агенс действия                                      |
| 0x0802      | PREDICATE        | Сказуемое, центральное действие/состояние                       |
| 0x0803      | DIRECT_OBJECT    | Прямое дополнение, пациенс                                      |
| 0x0804      | INDIRECT_OBJECT  | Косвенное дополнение, бенефициар                                |
| 0x0805      | COPULA_LINK      | Связка "быть/являться" (для предикативов)                       |

#### Слой S2 — Атрибутивные связи (Modification)
Уточнения и характеристики.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0810      | ATTRIBUTE        | Определение (прилагательное, причастие)                         |
| 0x0811      | ADVERBIAL        | Обстоятельство (наречие, наречная группа)                       |
| 0x0812      | QUANTIFIER       | Количественное определение                                      |
| 0x0813      | DETERMINER       | Артикль, указательное (this/that, the)                          |
| 0x0814      | INTENSIFIER      | Усилитель/ослабитель (very, slightly)                           |

#### Слой S3 — Структурные связи (Structural)
Сцепка частей сложных конструкций.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0820      | COORDINATOR      | Сочинение (and, or — соединяет равноправные элементы)           |
| 0x0821      | SUBORDINATOR     | Подчинение (because, although — вводит зависимое предложение)   |
| 0x0822      | APPOSITION       | Приложение (уточняющий повтор: "мой друг, доктор Иванов")       |
| 0x0823      | LIST_MEMBER      | Член списка/перечисления                                        |

#### Слой S4 — Прагматические связи (Pragmatic)
Роль в коммуникации, иллокутивная функция.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0830      | ADDRESSEE        | Адресат высказывания                                            |
| 0x0831      | TOPIC_MARKER     | Маркер темы (то, о чём речь)                                    |
| 0x0832      | FOCUS_MARKER     | Маркер ремы (новая информация, фокус внимания)                  |
| 0x0833      | EVIDENTIAL       | Источник знания (по слухам, лично видел, предполагаю)           |
| 0x0834      | MOOD_MARKER      | Модальность (вопрос, приказ, утверждение, гипотеза)             |

#### Слой S5 — Темпоральные связи (Temporal Frame)
Время и порядок внутри Frame (отличается от глобального COM event_id — это внутренняя темпоральность высказывания).

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0840      | TEMPORAL_ANCHOR  | Привязка ко времени ("вчера", "сейчас", "до Нового года")       |
| 0x0841      | DURATION         | Длительность ("два часа", "всё утро")                           |
| 0x0842      | FREQUENCY        | Частотность ("часто", "трижды")                                 |
| 0x0843      | TENSE_CARRIER    | Носитель грамматического времени                                |
| 0x0844      | ASPECT_CARRIER   | Носитель аспекта (совершенный/несовершенный, прогрессивный)     |

#### Слой S6 — Пространственные связи (Spatial Frame)
Локализация и направление.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0850      | LOCATION         | Где (статическая локализация)                                   |
| 0x0851      | SOURCE           | Откуда (исходная точка движения)                                |
| 0x0852      | GOAL             | Куда (целевая точка движения)                                   |
| 0x0853      | PATH             | Через что (траектория)                                          |
| 0x0854      | ORIENTATION      | Направление взгляда/действия (в сторону X)                      |

#### Слой S7 — Каузальные связи внутри Frame (Causal Internal)
Не путать с межсобытийной каузальностью (это работа CausalWeaver, см. Over-Domain Layer V1.1 п.7). Здесь — каузальность внутри одного высказывания.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0860      | INSTRUMENT       | Чем (орудие действия)                                           |
| 0x0861      | PURPOSE          | Зачем (цель действия)                                           |
| 0x0862      | REASON           | Почему (причина)                                                |
| 0x0863      | CONDITION        | При каком условии                                               |
| 0x0864      | RESULT           | С каким результатом                                             |
| 0x0865      | CONCESSION       | Несмотря на что (уступка)                                       |

#### Слой S8 — Метасинтаксические связи (Meta)
Связи между Frame и Frame, или Frame и его собственной структурой.

| `link_type` | Имя              | Назначение                                                      |
|-------------|------------------|-----------------------------------------------------------------|
| 0x0870      | EMBEDDED_FRAME   | Frame, вложенный в позицию участника другого Frame              |
| 0x0871      | FRAME_REFERENCE  | Ссылка на ранее закристаллизованный Frame                       |
| 0x0872      | FRAME_NEGATION   | Отрицание Frame целиком                                         |
| 0x0873      | FRAME_QUOTE      | Цитирование (этот Frame — чужое высказывание)                   |
| 0x0874      | FRAME_HYPOTHESIS | Гипотетический Frame (если бы)                                  |

### 3.3 Распределение по слоям — общий принцип

Слои S1–S2 — обязательны для большинства Frame (без подлежащего и сказуемого Frame не Frame).
Слои S3–S6 — частые, но не обязательные.
Слои S7–S8 — характерны для сложных и абстрактных Frame.

**Глубина Frame** определяется максимальным используемым слоем (Frame с S1+S2 — глубина 2; Frame с S1+S7+S8 — глубина 8).

### 3.4 Расширение `SemanticContributionTable`

Каждый синтаксический подтип добавляется в `SemanticContributionTable` с профилем вклада в Shell. Категория `0x08` получает базовый профиль:

```yaml
categories:
  0x08:
    name: "Syntactic"
    base_profile: [0, 0, 0, 0, 10, 5, 0, 15]
    # вклад в L5 (cognitive) и L8 (abstract)
```

Конкретные подтипы могут переопределять профиль через `overrides`. Пример:

```yaml
overrides:
  0x0830:  # ADDRESSEE
    name: "Addressee"
    profile: [0, 0, 0, 5, 5, 25, 0, 5]   # сильный вклад в L6 (social)

  0x0862:  # REASON
    name: "Reason"
    profile: [0, 0, 0, 0, 20, 0, 0, 15]  # сильный когнитивный + абстрактный

  0x0870:  # EMBEDDED_FRAME
    name: "Embedded Frame"
    profile: [0, 0, 0, 0, 15, 0, 0, 25]  # максимальный абстрактный
```

---

## 4. FrameWeaver — компонент

### 4.1 Структура

```rust
pub struct FrameWeaver {
    module_id: ModuleId,                      // в GENOME
    config: FrameWeaverConfig,                // из Schema Configuration
    candidates: HashMap<u64, FrameCandidate>, // незакристаллизованные кандидаты
    last_scan_event: u64,                     // event_id последнего скана MAYA
    stats: FrameWeaverStats,
}

pub struct FrameCandidate {
    pub anchor_position: [i16; 3],
    pub participants: Vec<Participant>,
    pub detected_at: u64,            // event_id обнаружения
    pub stability_count: u32,        // сколько сканов узор существует без изменений
    pub category: u16,               // FRAME_CATEGORY_SYNTAX и т.д.
}

pub struct Participant {
    pub sutra_id: u32,
    pub origin_domain_id: u16,
    pub role_link_type: u16,         // 0x08XX
    pub layer: u8,                   // S1..S8
}

pub struct FrameWeaverConfig {
    pub scan_interval_ticks: u32,           // как часто сканировать MAYA
    pub stability_threshold: u32,           // сколько сканов для предложения DREAM
    pub min_participants: usize,            // минимум участников (по умолчанию 2)
    pub max_storage_depth: u8,              // 0 = без ограничений
    pub default_unfold_depth: u8,
    pub max_unfold_depth: u8,
    pub cycle_handling: CycleStrategy,      // Break | Mark | Allow
    pub promotion_rules: Vec<PromotionRule>, // правила промоции EXPERIENCE → SUTRA
    pub crystallization_rules: Vec<CrystallizationRule>,
}
```

### 4.2 Расписание

`FrameWeaver` срабатывает каждые `scan_interval_ticks` тиков (по умолчанию 20). Точный момент определяется `TickSchedule.weaver_scan_intervals`.

### 4.3 Цикл работы

```
on_tick(tick, ashti, com):
    if tick % config.scan_interval_ticks != 0:
        return

    maya_state = ashti.peek_state(110)
    experience_state = ashti.peek_state(109)

    # 1. Сканировать MAYA на синтаксические узоры
    new_candidates = self.scan(maya_state)

    # 2. Обновить существующих кандидатов
    for candidate in self.candidates:
        if still_present_in_maya(candidate, maya_state):
            candidate.stability_count += 1
        else:
            self.candidates.remove(candidate.id)

    # 3. Кандидаты, достигшие стабильности, передать DREAM для кристаллизации в EXPERIENCE
    for candidate in self.candidates:
        if candidate.stability_count >= config.stability_threshold:
            proposal = self.propose_to_dream(candidate, target=EXPERIENCE)
            ashti.dream(107).submit_proposal(proposal)

    # 4. Оценить устойчивые Frame в EXPERIENCE на предмет промоции в SUTRA
    for frame in self.iter_frames_in_experience(experience_state):
        if self.qualifies_for_promotion(frame, config.promotion_rules):
            proposal = self.propose_promotion(frame)
            ashti.dream(107).submit_proposal(proposal)

    # 5. Отправить одобренные кристаллизации и промоции через UCL
    for approval in ashti.dream(107).drain_approvals(WeaverId::Frame):
        commands = match approval.kind {
            Crystallization => self.build_crystallization_commands(approval),
            Promotion       => self.build_promotion_commands(approval),
        }
        for cmd in commands:
            com.submit(cmd)  # пройдут через GUARDIAN
```

### 4.4 Распознавание узоров (`scan`)

```
scan(maya_state) -> Vec<FrameCandidate>:
    # Найти связи в MAYA с link_type из категории 0x08
    syntactic_connections = maya_state.connections.iter()
        .filter(|c| (c.link_type >> 8) == 0x08)
        .filter(|c| c.flags & FLAG_ACTIVE != 0)
        .collect()

    # Группировать по предполагаемым центрам Frame
    # Центр Frame — токен, к которому ведут связи как минимум двух разных слоёв
    groups = group_by_potential_anchors(syntactic_connections)

    # Каждая группа — кандидат
    candidates = groups.map(|g| FrameCandidate::from(g))

    return candidates
```

### 4.5 Кристаллизация в EXPERIENCE

После одобрения DREAM, FrameWeaver формирует UCL-команды:

```
build_crystallization_commands(approval) -> Vec<UclCommand>:
    cmds = []

    # 1. Создать анкер-токен в EXPERIENCE (domain_id = 109)
    anchor_cmd = UclCommand::InjectToken {
        target_domain: 109,                              // ← изменение V1.1
        type_flags: TOKEN_FLAG_FRAME_ANCHOR | candidate.category,
        position: candidate.anchor_position,
        lineage_hash: hash_of(candidate.participants),
        state: STATE_ACTIVE,                             // ← изменение V1.1
        ...
    }
    cmds.push(anchor_cmd)
    anchor_id = expected_sutra_id(anchor_cmd)

    # 2. Создать связи к каждому участнику
    for p in candidate.participants:
        conn_cmd = UclCommand::CreateConnection {
            source_id: anchor_id,
            target_id: p.sutra_id,
            domain_id: 109,                              // ← изменение V1.1
            link_type: p.role_link_type,
            reserved_gate: encode_origin(p.origin_domain_id, p.role_id),
            ...
        }
        cmds.push(conn_cmd)

    return cmds
```

### 4.6 Реактивация существующего Frame

Если при сканировании MAYA FrameWeaver обнаруживает узор, структурно совпадающий с уже существующим Frame в EXPERIENCE (по `lineage_hash`), то **новый Frame не создаётся**. Вместо этого:

1. Существующий Frame в EXPERIENCE получает реактивацию — температура повышается, mass может расти.
2. Это естественный механизм обучения: повторяющийся опыт усиливается.
3. Новая UCL-команда: `UclCommand::ReinforceFrame { anchor_id, delta_mass, delta_temperature }`.

Это невозможно было в V1.0 (Frame был `STATE_LOCKED` в SUTRA), и это одна из причин, почему правильный дом — EXPERIENCE.

---

## 5. Правила кристаллизации и промоции

### 5.1 Источник правил

**Genome (frozen после boot):** структурные инварианты.
- Какие категории Frame существуют (V1.1 — только SYNTAX).
- Какие подтипы связей разрешены в категории.
- Какие домены могут быть источниками участников.
- Базовые инварианты (минимум участников, ограничения на роли).
- **Условия промоции EXPERIENCE → SUTRA** (базовые пороги, которые нельзя ослабить конфигурацией).

**Schema Configuration (загружается при старте, можно менять):** правила оценки.
- Конкретные триггеры (стабильность, температура, повторное появление).
- Условия (доминирующие слои, состав участников).
- Действия (полная кристаллизация, частичная, повторная проверка).
- Дополнительные условия промоции (сверх базовых из Genome).

### 5.2 Структура правила кристаллизации

```rust
pub struct CrystallizationRule {
    pub id: String,
    pub priority: u8,
    pub trigger: RuleTrigger,
    pub conditions: Vec<RuleCondition>,
    pub action: RuleAction,
}

pub enum RuleTrigger {
    StabilityReached(u32),                  // candidate.stability_count >= N
    DreamCycle,                              // во время DREAM-фазы
    RepeatedAssembly { window_ticks: u32 },  // повторное появление в окне
    HighConfidence(f32),                     // confidence узора >= X
}

pub enum RuleCondition {
    DominantLayer(u8),                       // в Frame доминирует слой S_N
    MinParticipants(usize),
    RequiresParticipantFromDomain(u16),
    LayerPresent(u8),                        // в Frame присутствует слой S_N
    MaxDepth(u8),
}

pub enum RuleAction {
    CrystallizeFull,                         // создать Frame целиком в EXPERIENCE
    CrystallizeAnchorOnly,                   // только анкер с метаданными,
                                             // связи доразвернутся при первом запросе
    Defer { ticks: u32 },                    // отложить решение
    Reject,                                  // отклонить кандидата
}
```

### 5.3 Пример правил кристаллизации

```yaml
# schema-config: frame_crystallization_rules.yaml

rules:
  - id: "basic_assertion_frame"
    priority: 100
    trigger:
      type: StabilityReached
      value: 3
    conditions:
      - LayerPresent: 1   # должен быть слой S1 (ядерные роли)
      - MinParticipants: 2
    action: CrystallizeFull

  - id: "deep_causal_frame"
    priority: 80
    trigger:
      type: StabilityReached
      value: 5
    conditions:
      - LayerPresent: 7   # каузальные роли
      - MinParticipants: 3
    action: CrystallizeFull

  - id: "embedded_frame"
    priority: 90
    trigger:
      type: StabilityReached
      value: 4
    conditions:
      - LayerPresent: 8   # метасинтаксические роли
    action: CrystallizeFull

  - id: "dream_consolidation"
    priority: 50
    trigger:
      type: DreamCycle
    conditions:
      - MinParticipants: 2
    action: CrystallizeAnchorOnly  # лёгкая консолидация во время сна

  - id: "reject_too_shallow"
    priority: 200   # высокий приоритет — проверяется первым
    trigger:
      type: StabilityReached
      value: 1
    conditions:
      - MinParticipants: 1   # одиночные узлы — не Frame
    action: Reject
```

### 5.4 Правила промоции EXPERIENCE → SUTRA

Промоция — редкий путь, превращающий накопленный опыт в первичную истину. Требует строгих условий.

```rust
pub struct PromotionRule {
    pub id: String,
    pub min_age_events: u64,         // минимальный возраст Frame в EXPERIENCE (в event_id)
    pub min_reactivations: u32,      // минимальное число реактиваций (см. 4.6)
    pub min_temperature: u8,         // минимальная устоявшаяся температура
    pub min_mass: u8,                // минимальная масса (как индикатор значимости)
    pub min_participant_anchors: usize, // минимум участников, которые сами — anchor'ы SUTRA
    pub requires_codex_approval: bool, // требует явного одобрения CODEX (обычно true)
}
```

**Базовые пороги в Genome (нельзя ослабить):**

```yaml
promotion_base:
  min_age_events: 100000           # Frame должен существовать долго
  min_reactivations: 10            # должен повторно проявиться много раз
  requires_codex_approval: true    # только с санкции конституционного фильтра
```

**Пример конфигурируемого правила:**

```yaml
promotion_rules:
  - id: "universal_syntactic_pattern"
    min_age_events: 500000
    min_reactivations: 50
    min_temperature: 200
    min_mass: 100
    min_participant_anchors: 3    # большинство участников уже anchor'ы SUTRA
    requires_codex_approval: true
```

**Поток промоции:**

```
1. FrameWeaver в цикле on_tick перебирает Frame в EXPERIENCE
2. Для каждого проверяет PromotionRule (Genome + Config)
3. Если все условия выполнены — формирует PromotionProposal
4. DREAM рассматривает предложение в контексте других приоритетов
5. Если DREAM одобряет — отправляет в GUARDIAN
6. GUARDIAN проверяет CODEX (конституционный фильтр)
7. Если CODEX одобряет — UCL-команды копируют Frame в SUTRA
   (с новым anchor_id, STATE_LOCKED, флагом TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE)
8. Оригинал в EXPERIENCE сохраняется
```

Промоция не часта — в нормальных условиях за длительный период работы системы их должны быть единицы или десятки, не тысячи. Это механизм роста фундаментальной онтологии системы, не рутинная операция.

---

## 6. Параметры глубины

| Параметр              | Значение по умолчанию | Назначение                                     |
|-----------------------|-----------------------|------------------------------------------------|
| `max_storage_depth`   | 0 (без ограничений)   | Предельная глубина при хранении в EXPERIENCE   |
| `default_unfold_depth`| 3                     | Глубина развёртывания по умолчанию при запросе |
| `max_unfold_depth`    | 8                     | Максимально допустимая глубина по запросу      |
| `cycle_handling`      | `Allow`               | Стратегия обработки циклов при кристаллизации  |

### 6.1 Стратегии обработки циклов

```rust
pub enum CycleStrategy {
    Break,     // Цикл рвётся в произвольном месте, последняя связь помечается флагом
    Mark,      // Цикл сохраняется, на ребре устанавливается флаг CONNECTION_FLAG_CYCLE
    Allow,     // Цикл сохраняется без пометок (опыт может быть противоречивым)
}
```

**Изменение относительно V1.0:** в V1.0 была стратегия `Reject` и инвариант "Frame должен быть DAG". Это было навязано SUTRA-мышлением (истина не должна быть противоречива). В V1.1 Frame живёт в EXPERIENCE, а **опыт по природе может быть противоречивым** — одна и та же структура может повторяться в разных конфигурациях, образуя циклы при наложении.

Поэтому V1.1 добавляет `Allow` как новую стратегию (и делает её дефолтом), и убирает `Reject`. DAG-инвариант сохраняется **только для промоции в SUTRA** — если Frame претендует на статус истины, он должен быть ацикличен. Но внутри EXPERIENCE циклы допустимы.

---

## 7. Развёртывание Frame на запрос

Любой домен может запросить Frame из EXPERIENCE (а для промоутнутых Frame — из SUTRA) через UCL-команду:

```
UclCommand::UnfoldFrame {
    frame_anchor_id: u32,    // sutra_id анкера
    source_domain: u16,      // 109 (EXPERIENCE) по умолчанию; 100 для промоутнутых
    target_domain: u16,      // куда развернуть
    depth: u8,               // глубина (0 = use default)
}
```

Результат — набор токенов и связей в целевом домене, представляющий копию структуры Frame с учётом глубины.

При `depth < max_storage_depth` глубокие участники остаются "свёрнутыми" — представляются ссылками на дальнейший подузор, который можно развернуть отдельным запросом.

**Разрешение источника:** если `source_domain` не указан, FrameWeaver ищет анкер сначала в EXPERIENCE (109), затем в SUTRA (100). Это отражает естественный порядок: опыт проверяется первым, истина — как fallback для глубинных структур.

---

## 8. Интеграция с GUARDIAN

GUARDIAN проверяет каждую UCL-команду от FrameWeaver:

- `InjectToken` в EXPERIENCE (анкер Frame) — проверка на права доступа FrameWeaver к EXPERIENCE (Write).
- `CreateConnection` в EXPERIENCE (связи Frame) — проверка `link_type` (только из категории 0x08), проверка `domain_id = 109`.
- `ReinforceFrame` — проверка того, что целевой анкер действительно является Frame (имеет `TOKEN_FLAG_FRAME_ANCHOR`).
- `UnfoldFrame` — проверка прав запрашивающего домена на чтение EXPERIENCE/SUTRA.
- **Promotion-команды (InjectToken в SUTRA)** — особый режим: GUARDIAN обращается к CODEX для конституционной проверки. Без одобрения CODEX промоция не происходит.

Если действие нарушает GENOME или CODEX — veto, кандидат не кристаллизуется / не промоутируется.

---

## 9. Метрики и наблюдаемость

```rust
pub struct FrameWeaverStats {
    pub scans_performed: u64,
    pub candidates_detected: u64,
    pub candidates_proposed_to_dream: u64,
    pub crystallizations_approved: u64,        // EXPERIENCE
    pub crystallizations_vetoed: u64,
    pub frames_in_experience: u64,
    pub frame_reactivations: u64,               // реактивации существующих
    pub promotions_proposed: u64,
    pub promotions_approved: u64,               // SUTRA
    pub promotions_vetoed: u64,
    pub frames_in_sutra: u64,                   // промоутнутые
    pub unfold_requests: u64,
    pub cycles_handled: u64,
}
```

Доступны через `BroadcastSnapshot` (feature `adapters`) для дашборда.

---

## 10. Открытые вопросы

- **Конфликт правил.** Если несколько правил применимы к одному кандидату — побеждает по `priority`. Что если приоритеты равны? Сейчас: побеждает первое в порядке загрузки. Возможно, нужна более строгая семантика.
- **Эволюция syntactic-категории.** Должен ли FrameWeaver уметь предлагать новые подтипы (через DREAM → CODEX), или 8 слоёв из раздела 3 — окончательный набор?
- **Межоменные участники.** Может ли участник Frame быть сам Frame из другого домена-зеркала? Сейчас — да, через `EMBEDDED_FRAME` (S8). Но не описан случай, когда участник в своём родном домене ещё не имеет sutra_id (только локальный).
- **Производительность сканирования.** При большом MAYA (тысячи токенов) сканирование на каждом интервале может быть дорогим. Возможны оптимизации: incremental scan, реакция только на новые синтаксические связи через EventBus.
- **Демоция SUTRA → EXPERIENCE.** Сейчас путь строго односторонний. Что, если промоутнутый Frame оказался ошибкой? Концептуально: истина не возвращается в опыт. Практически: это может быть проблемой. Оставлено как открытый вопрос; возможна в будущем специальная операция `RevokePromotion` с веским CODEX-обоснованием.
- **Frame и обмен между экземплярами AXIOM.** Memory Persistence V1.0 упоминает `IMPORT_WEIGHT_FACTOR=0.7` для импортируемого опыта. Frame, экспортированный в другую систему, импортируется как опыт (в EXPERIENCE с пониженным весом), не как истина. Детали протокола обмена Frame — отдельная спецификация.

---

## 11. Резюме

FrameWeaver V1.1 — первый Weaver в Over-Domain Layer.

Назначение: распознавать синтаксические/реляционные узоры в MAYA, отбирать стабильные через правила (Genome + Schema Configuration), передавать в DREAM для одобрения, кристаллизовать одобренные в **EXPERIENCE** (накопленный опыт). Устойчивые Frame могут быть промоутнуты в SUTRA (первичная истина) через CODEX.

Структурно: новая категория `link_type` (0x08 Syntactic) с 8 слоями подтипов; анкер-токен с флагом `TOKEN_FLAG_FRAME_ANCHOR`, живущий в EXPERIENCE (domain_id=109) со `STATE_ACTIVE`; метаданные о доменах участников в `reserved_gate` Connection; механизм реактивации для повторяющегося опыта.

Архитектурно: компонент Over-Domain Layer, не домен. Подчиняется trait `Weaver`. Подчиняется GUARDIAN. Промоция в SUTRA — через CODEX.

Онтологически: MAYA генерирует, DREAM отбирает, EXPERIENCE хранит опыт, SUTRA хранит истину. Frame — закристаллизованный опыт структурирования.

Будущее: список будущих Weavers — в Over-Domain Layer V1.1, раздел 7. Текущая реализация — только синтаксический Frame, остальные категории Frame и другие Weavers — после практики V1.1.

---

## ПРИЛОЖЕНИЕ A: Инструкция для имплементера (Sonnet)

### A.1 Что реализовать в этой итерации

1. Создать модуль `axiom-runtime/src/over_domain/`.
2. Определить traits `OverDomainComponent` и `Weaver` (см. Over-Domain Layer V1.1, раздел 8).
3. Реализовать `FrameWeaver` со всеми структурами из раздела 4.
4. Расширить `SemanticContributionTable` категорией `0x08 Syntactic` и подтипами из раздела 3.2.
5. Добавить константы `TOKEN_FLAG_FRAME_ANCHOR` (0x0010), `TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE` (0x0020), `FRAME_CATEGORY_*` в `axiom-core`.
6. Добавить новые UCL OpCodes: `UnfoldFrame` (4000+), `ReinforceFrame` (4001).
7. Расширить `TickSchedule` полем `weaver_scan_intervals: HashMap<WeaverId, u32>`.
8. Интегрировать `FrameWeaver` в `AxiomEngine::new()` — создаётся после GUARDIAN.
9. Frame кристаллизуется в **EXPERIENCE (109)**, не в SUTRA (100). Promotion path — отдельный, через CODEX (раздел 5.4).
10. Добавить FrameWeaver-метрики в `BroadcastSnapshot` (feature `adapters`).
11. Тесты: scan, candidate stability, crystallization commands (в EXPERIENCE), reactivation, promotion, unfold, cycle handling.

### A.2 Что НЕ реализовать (deferred)

Следующие компоненты упоминаются в Over-Domain Layer V1.1, раздел 7, но **не реализуются** в текущем релизе:

- **Будущие Weavers:** CausalWeaver, SpatialWeaver, TemporalWeaver, AnalogyWeaver, NarrativeWeaver.
- **Будущие Guardians:** ResourceGuardian, CoherenceGuardian.
- **Будущие категории Frame:** FRAME_CATEGORY_SEMANTIC (0x0200), FRAME_CATEGORY_PRAGMA (0x0300).
- **Демоция SUTRA → EXPERIENCE** (см. раздел 10, открытые вопросы).
- **Межсистемный обмен Frame** (Memory Persistence интеграция — отдельная спека).

**Действие для имплементера:** создать файл `docs/specs/deferred/FUTURE_WEAVERS.md` со следующим содержимым:

```markdown
# Deferred: Future Over-Domain Components

**Эти компоненты не реализуются в текущей версии. Список — архитектурный задел.
Каждый получит собственную спецификацию на момент реализации.**

## Future Weavers
- **CausalWeaver** — причинные цепочки A → B → C.
- **SpatialWeaver** — пространственные конфигурации.
- **TemporalWeaver** — временные последовательности и ритмы.
- **AnalogyWeaver** — структурные аналогии A:B::C:D.
- **NarrativeWeaver** — нарративные структуры из последовательностей Frame.

## Future Guardians
- **ResourceGuardian** — бюджет тиков, памяти, температуры.
- **CoherenceGuardian** — семантическая когерентность Shell.

## Future Frame Categories
- FRAME_CATEGORY_SEMANTIC (0x0200) — семантические Frame
- FRAME_CATEGORY_PRAGMA (0x0300) — прагматические Frame

## Архитектурный принцип
Текущая реализация (GUARDIAN + FrameWeaver) спроектирована с учётом расширяемости:
- Все компоненты Over-Domain Layer следуют trait `OverDomainComponent`.
- Все Weavers следуют trait `Weaver`.
- Категории Frame расширяются через `FRAME_CATEGORY_MASK` в `type_flags`.
- Категории `link_type` расширяются через старший байт u16.
- Weavers пишут в EXPERIENCE по умолчанию. Промоция в SUTRA — через CODEX.

При появлении нового компонента: создать его в `axiom-runtime/src/over_domain/`
и зарегистрировать в Orchestrator. Внести соответствующую запись в GENOME.
```

### A.3 Согласованность спек и кода

При реализации обновлять одновременно:
- Этот документ (FrameWeaver_V1_1.md)
- BLUEPRINT.md (раздел "Домены как зеркала" обновлён в V1.1)
- Over_Domain_Layer_V1_1.md
- GENOME_V1_0.md (добавить ModuleId::FrameWeaver и его permissions для EXPERIENCE и SUTRA)
- Schema Configuration (добавить frame_weaver_config секцию, crystallization_rules, promotion_rules)

При обнаружении расхождений между спекой и кодом — фиксировать вопрос в раздел "Открытые вопросы" этого документа, не молча менять одно из двух.

### A.4 Миграция с V1.0

Предыдущая версия V1.0 помечена как superseded. Ключевые отличия при чтении кода, написанного по V1.0:
- Любая запись `InjectToken { target_domain: 100, type_flags: TOKEN_FLAG_FRAME_ANCHOR ... }` → изменить на `target_domain: 109`.
- `state: STATE_LOCKED` для Frame-анкеров → `state: STATE_ACTIVE`.
- `CreateConnection { domain_id: 100, link_type: 0x08XX ... }` → `domain_id: 109`.
- Инвариант "Frame в SUTRA должен быть DAG" → инвариант "Frame при промоции в SUTRA должен быть DAG; в EXPERIENCE допустимы циклы".
