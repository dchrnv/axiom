# AXIOM WORKSTATION — CORRECTIONS V1.0

**Назначение:** Точечные правки к пакету проектирования V1.0 на основе обратной связи Sonnet
**Применяется к:** Документы 1-4 V1.0
**Версия:** 1.0
**Дата:** 2026-04-30
**Статус:** Patch-документ для применения до начала реализации

---

## 0. Контекст

После завершения пакета проектирования (Документы 1-4) Sonnet провёл review всех материалов и обнаружил пять конкретных технических расхождений или неполностью проработанных мест. Этот документ их фиксирует точечно.

**Природа правок:**
- Не пересматриваем архитектуру
- Не переписываем существующие документы целиком
- Каждая правка — отдельная корректировка с указанием места применения

**После применения этих правок** пакет готов к реализации без обнаруженных подводных камней.

---

## Правка C1 — Терминология popup

### Источник проблемы

Sonnet обнаружил, что слово "popup" используется в двух местах с разной семантикой:

- **Документ 3A раздел 2.6** — клик на GUARDIAN раскрывает popup со статистикой veto
- **Документ 3B раздел 4.6** — клик на карточку Frame в Patterns раскрывает детальный popup
- **Документ 1 раздел 10** — анти-паттерн "Pop-up уведомления о каждом событии"

Без явного определения "popup" — есть риск дрейфа к anti-pattern.

### Решение

Зафиксировать терминологию в **глоссарии Документа 3 Часть D раздел "Приложение: язык глобальных элементов"**:

**Добавить термины:**

- **Detail panel** — раскрывающаяся секция с подробной информацией, появляющаяся **только по явному клику** пользователя на элемент. Не модальная (можно работать с другими частями интерфейса). Закрывается явным действием (кнопка [×] или клик вне панели). Используется для просмотра деталей Frame, DreamReport, GUARDIAN veto и т.п.

- **Notification popup** — **запрещённый паттерн в Workstation V1.0**. Всплывающее окно, появляющееся автоматически без действия пользователя (например, "у вас новое сообщение"). Не используется ни в одном окне.

**Заменить во всех документах слово "popup" на "detail panel"** там, где оно описывает раскрывающуюся секцию по клику. Конкретные места:

- Документ 3A раздел 2.6 — "раскрывает popup со статистикой" → "раскрывает detail panel со статистикой"
- Документ 3B раздел 4.6 — "детальный popup" → "detail panel"
- Документ 3B раздел 5.6 — "детальный popup" → "detail panel"
- Документ 3C раздел 6.13 — упоминание popup при ошибках → уточнить, что это **inline сообщение в ленте**, не popup
- Документ 3D раздел 10.2 — "диагностический popup" → "diagnostic detail panel"

**В разделе 10 Документа 1 (антипаттерны):**
- Уточнить: "Pop-up notifications уведомляющие о событиях" (вместо общего "Pop-up уведомления") — чтобы не запретить detail panels по клику.

### Применение

Patch применяется при следующей правке Документов 1, 3A, 3B, 3C, 3D или может быть зафиксирован как обязательное соглашение для Sonnet при реализации.

---

## Правка C2 — ConfigSchema в axiom-protocol

### Источник проблемы

Документ 4 этап 5 (Configuration tab) и Документ 3C раздел 8 предполагают **schema-driven UI**: Engine возвращает не только текущие значения параметров, но и **схему** (типы, диапазоны, labels, описания), на основе которой Workstation рендерит UI элементы.

В Документе 2 (раздел 4) этот тип явно не определён. При реализации этапа 5 разработчик упрётся в необходимость добавить ConfigSchema задним числом. Лучше зафиксировать сейчас.

### Решение

**Добавить в Документ 2 раздел 4 определение `ConfigSchema`** и сопутствующих типов в axiom-protocol:

```rust
// В axiom-protocol::config

pub struct ConfigSchema {
    pub sections: Vec<ConfigSection>,
}

pub struct ConfigSection {
    pub id: String,                    // "engine.dream_phase", "workstation.connection"
    pub label: String,                 // "Dream Phase"
    pub category: ConfigCategory,      // Engine | Workstation
    pub fields: Vec<ConfigField>,
    pub subsections: Vec<ConfigSection>, // для иерархии (Engine → Domains → LOGIC)
}

pub enum ConfigCategory {
    Engine,
    Workstation,
}

pub struct ConfigField {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub field_type: ConfigFieldType,
    pub current_value: ConfigValue,
    pub default_value: ConfigValue,
    pub hot_reloadable: bool,
    pub readonly: bool,                // GENOME полей — true
}

pub enum ConfigFieldType {
    Bool,
    Integer { min: i64, max: i64 },
    UInt { min: u64, max: u64 },
    Float { min: f64, max: f64 },
    String { max_length: u32 },
    Enum { variants: Vec<String> },
    Duration,                          // ticks или ns
    Domain,                            // selector домена 100-110
}

pub enum ConfigValue {
    Bool(bool),
    Integer(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumVariant(String),
    Duration(u64),
    Domain(u16),
}
```

**Расширить EngineCommand:**

```rust
// Было:
GetConfig { section: String },

// Стало:
GetConfigSchema,                       // возвращает полную схему всех секций
GetConfigSection { id: String },       // возвращает одну секцию по id
UpdateConfigField { 
    section_id: String, 
    field_id: String, 
    value: ConfigValue,
},
```

**Расширить EngineMessage:**

```rust
// Добавить в CommandResultData:
ConfigSchema(ConfigSchema),
ConfigSection(ConfigSection),
ConfigUpdateApplied { hot_reloaded: bool },
ConfigValidationError { field_id: String, message: String },
```

### Применение

**Этап 1** реализации (axiom-protocol) теперь включает ConfigSchema и связанные типы. Это **не блокирует** этапы 2-4, но критично для этапа 5.

**Документ 4 этап 1** дополняется задачей: "Реализовать ConfigSchema, ConfigField, ConfigValue, ConfigFieldType (см. Корректировку C2)".

**Документ 2 раздел 4** дополняется секцией 4.9 "Configuration schema" с полным определением.

---

## Правка C3 — External Adapters в Engine

### Источник проблемы

Документ 3C раздел 7 (Files tab) предполагает, что Engine отвечает на `ListAdapters` реальным списком адаптеров с метаданными и схемой опций.

Sonnet указал: в существующей архитектуре External Adapters V3.0 — это REST/WS-слой `axiom-agent`, **не подсистема самого Engine**. То есть адаптеры живут в axiom-agent crate, не в axiom-runtime.

При реализации этапа 8 (Files tab) разработчик столкнётся с двумя уровнями архитектурного решения, которые мы должны разделить чётко.

### Решение — два уровня

**Уровень 1 — концептуальное направление (решено сейчас, V1.0):**

Workstation общается с External Adapters **через Engine как прокси-слой**, не напрямую с axiom-agent.

```
Workstation  ⟷  Engine  ⟷  axiom-agent (External Adapters)
              WebSocket    [способ — см. Уровень 2]
```

Обоснование выбора прокси-варианта:
- Один WebSocket клиент в Workstation (не два)
- Одна точка авторизации и одна точка отказа
- Engine знает состояние системы (например, DREAMING) и может корректно паузить адаптеры
- Если Engine упал — Files tab автоматически становится недоступен, что правильно (не должно быть импорта без живого Engine)

Альтернативный вариант (Workstation → axiom-agent напрямую через второй WebSocket) **отклонён** из-за усложнения протокола и точек отказа.

**Уровень 2 — реализационная развилка (решается на этапе 8):**

**Как именно Engine коммуницирует с axiom-agent** — это техническая деталь, которую невозможно решить сейчас без знания состояния axiom-agent на момент этапа 8. Возможные варианты:

- **In-process call** — если axiom-agent окажется встроен в Engine как библиотека (одна и та же программа)
- **Localhost HTTP** — если axiom-agent отдельный процесс с REST API
- **Unix socket** — если оптимизируем под локальную работу без TCP overhead
- **Что-то ещё** — то, что появится по ходу разработки axiom-agent

Решение этой развилки **не влияет** на контракт Engine ↔ Workstation (Документ 2 раздел 4). С точки зрения Workstation — Engine просто отвечает на `ListAdapters`, `StartImport` и т.д. Что Engine делает за этим — его внутреннее дело.

### Что добавляется в документы

**Документ 2 получает новый раздел: "External Adapters integration" (раздел 6.7 или новый):**

```
External Adapters реализованы в crate `axiom-agent`, не в самом Engine.
Workstation обращается к адаптерам через Engine как прокси:

1. Workstation отправляет ListAdapters/StartImport/etc в Engine через WebSocket
2. Engine обрабатывает запрос и обращается к axiom-agent
3. axiom-agent выполняет операцию, возвращает результат Engine
4. Engine оборачивает ответ и возвращает в Workstation

Способ коммуникации между Engine и axiom-agent (in-process call,
localhost HTTP, Unix socket, иное) определяется на этапе 8 реализации
исходя из состояния axiom-agent на тот момент. Этот выбор не влияет
на контракт Workstation ↔ Engine.
```

**Документ 4 этап 8** дополняется новым подэтапом 8.1:

> **8.1.a — Принять решение по способу коммуникации Engine ↔ axiom-agent:**
> - Review текущего состояния axiom-agent crate
> - Выбор: in-process / localhost HTTP / Unix socket / иное
> - Зафиксировать в errata Документа 2 V1.1
> - ~ 0.5-1 день
>
> Концептуальное направление (прокси через Engine) **уже решено** на стадии проектирования.

### Применение

**Сейчас:** концептуальное направление "прокси через Engine" зафиксировано в Документе 2 раздел 6.7.

**На этапе 8:** реализационная деталь (способ Engine ↔ axiom-agent) выбирается и фиксируется в errata.

---

## Правка C4 — Throttling threshold для DomainActivity

### Источник проблемы

Документ 2 раздел 4.7 говорит: "DomainActivity отправляется только при изменениях больше threshold". Но **значение threshold не зафиксировано**, и не сказано, **где оно настраивается**.

При реализации Sonnet увидит: либо threshold захардкожен (плохо), либо вынесен в конфиг (но в какой именно).

### Решение

**Threshold для broadcasting — это часть BroadcastingConfig**, отдельной секции в Engine config:

```rust
// В axiom-runtime или axiom-broadcasting

pub struct BroadcastingConfig {
    pub tick_event_interval: u32,            // отправлять Tick каждые N тиков
    pub domain_activity_threshold: u32,      // отправлять DomainActivity при изменении ≥ N
    pub max_event_queue_per_client: usize,   // дроп старых при переполнении
    pub event_drop_strategy: DropStrategy,   // FifoDropOldest | TailDropNewest
    pub snapshot_resync_threshold: usize,    // если очередь дошла до N — отправить Snapshot
}

pub enum DropStrategy {
    FifoDropOldest,
    TailDropNewest,
}

impl Default for BroadcastingConfig {
    fn default() -> Self {
        Self {
            tick_event_interval: 100,            // раз в 100 тиков
            domain_activity_threshold: 5,        // изменение recent_activity >= 5
            max_event_queue_per_client: 1000,
            event_drop_strategy: DropStrategy::FifoDropOldest,
            snapshot_resync_threshold: 800,
        }
    }
}
```

**Эта секция доступна в Configuration tab** (Workstation/Connection или Engine/Broadcasting).

**Документ 2 раздел 4.7** дополняется:

> Все threshold-параметры throttling-а — настраиваемые через `BroadcastingConfig` в Engine config. Дефолты подбираются эмпирически на этапе реализации и могут быть зафиксированы в errata после первых live-тестов.

**Документ 3C раздел 8** (Configuration tab) — добавить категорию **Engine → Broadcasting** в дерево настроек.

### Применение

**Этап 2** (axiom-broadcasting + Engine integration) включает реализацию `BroadcastingConfig` и его hot-reload.

**Этап 5** (Configuration tab) включает категорию Broadcasting.

---

## Правка C5 — axiom-bench как библиотека и отдельный binary для bench-mode

### Источник проблемы

Документ 4 этап 8 описывает "переупаковку axiom-bench для программного вызова через `--bench-mode` CLI flag". Sonnet указал: сейчас `axiom-bench` — это crate с criterion-бенчмарками, **не имеющий bin target**. Он компилируется только под `cargo bench`, не под обычный `cargo run`.

При реализации этапа 8 разработчик столкнётся с архитектурной развилкой: добавлять bin target в axiom-bench или выносить bench-mode в другой crate.

### Решение

**Архитектурное решение, фиксируемое сейчас:**

Преобразовать `axiom-bench` из чистого benchmark-crate в **dual-purpose crate**:

1. **Library mode** — существующий код для `cargo bench` остаётся (через criterion harness в `[[bench]]` секциях Cargo.toml)
2. **Binary mode** — добавляется новый bin target `axiom-bench-runner`, который:
   - Принимает `--bench-id`, `--iterations`, и другие параметры через CLI
   - Запускается из Workstation как bench-instance
   - Прогоняет указанный бенчмарк программно (через ту же логику, что и criterion harness)
   - Отчитывается через WebSocket о прогрессе и результатах
   - Завершается после прогона

**Cargo.toml:**

```toml
[package]
name = "axiom-bench"

[[bench]]
name = "hot_path_tick"
harness = false  # criterion использует свой harness

# ... другие [[bench]]

[[bin]]
name = "axiom-bench-runner"
path = "src/bin/runner.rs"
```

**Структура:**

```
crates/axiom-bench/
├── benches/                    # для cargo bench
│   ├── hot_path_tick.rs
│   ├── frameweaver_overhead.rs
│   └── ...
├── src/
│   ├── lib.rs                  # общая логика бенчмарков
│   ├── bin/
│   │   └── runner.rs           # bench-instance binary
│   └── runners/                # программные функции запуска бенчмарков
│       ├── hot_path.rs
│       ├── frameweaver.rs
│       └── ...
```

**Альтернатива (вынос в axiom-agent или новый crate)** отклонена, потому что:
- Логика бенчмарков уже в axiom-bench
- Дублирование кода — плохо
- axiom-agent — это совсем другая ответственность

### Применение

**Документ 4 этап 8** дополняется задачей:

> **8.1.b — Преобразование axiom-bench:**
> - Добавить `[[bin]]` секцию для axiom-bench-runner
> - Вынести логику бенчмарков из benches/ в src/runners/ как переиспользуемые функции
> - benches/ становится тонким слоем поверх runners/, использующим criterion harness
> - bin/runner.rs принимает CLI флаги и вызывает соответствующий runner программно

Это работа на ≈ 2-3 дня, **в начале этапа 8**, до реализации UI Benchmarks tab.

**Существующий код axiom-bench не ломается** — `cargo bench` продолжает работать как раньше, потому что benches/ файлы используют ту же логику, только через criterion harness.

---

## Сводка корректировок

| #   | Правка                                          | Источник проблемы              | Где применяется             |
|-----|-------------------------------------------------|--------------------------------|------------------------------|
| C1  | Терминология popup → detail panel               | Двусмысленность с anti-pattern | Документы 1, 3A, 3B, 3C, 3D |
| C2  | ConfigSchema в axiom-protocol                   | Schema-driven UI без типа      | Документы 2, 4 (этап 1)     |
| C3  | External Adapters: прокси concept + детали этапа 8 | Архитектурное расхождение      | Документы 2, 4 (этап 8)     |
| C4  | Threshold throttling — параметризованные        | Хардкод vs config              | Документы 2, 3C, 4 (этап 2,5)|
| C5  | axiom-bench как dual-purpose crate              | Bench-only crate без bin       | Документ 4 (этап 8)         |

---

## Приоритеты применения

**Применить до начала реализации:**
- C1 (терминология) — простая текстовая правка
- C2 (ConfigSchema) — нужно для этапа 1, нельзя откладывать
- C3 концептуальное направление (прокси через Engine) — фиксируется в Документе 2 раздел 6.7
- C4 (BroadcastingConfig дефолты) — нужно для этапа 2

**Применить в начале соответствующего этапа:**
- C3 реализационная деталь (способ Engine ↔ axiom-agent) — начало этапа 8
- C5 (axiom-bench dual-purpose) — начало этапа 8

---

## Что НЕ изменилось после применения корректировок

Чтобы было ясно, что **архитектурно ничего не пересматривается**:

- Стратегия "снизу вверх" — остаётся
- Порядок этапов — остаётся
- Оценка времени (~ 20 недель) — остаётся, возможно +1 неделя на C5 в этапе 8
- Эстетика (школа Cupertino, минимализм) — без изменений
- Состав 9 окон — без изменений
- Принципы Документа 1 — без изменений
- Multi-window iced + tabs + detach — без изменений
- postcard как формат сериализации — без изменений
- Bench-instance как отдельный процесс — без изменений
- Engine как субъект, Workstation как окно — без изменений

Корректировки **точечные** и не задевают onтологию проекта. Это правки на уровне технической специфики, не на уровне видения.

---

## После применения

После того как:
1. Терминология popup исправлена в документах
2. ConfigSchema добавлена в Документ 2
3. Решение по External Adapters зафиксировано
4. BroadcastingConfig добавлен в Документ 2 и Configuration tab
5. axiom-bench dual-purpose решение зафиксировано в Документе 4

— **пакет проектирования готов к передаче Sonnet** для реализации.

---

## Благодарность

Sonnet провёл review с конкретностью, которую сложно переоценить. Все пять замечаний — реальные подводные камни, на которые мы бы наткнулись на этапе реализации с большей стоимостью переработки. Стоимость их исправления **сейчас** — несколько часов на правки в документах. Стоимость исправления **на этапе реализации** была бы дни или недели.

Это правильный паттерн работы: документы → review → правки → реализация. И chrnv его поддерживает с самого начала проекта (errata-документы для FrameWeaver и DREAM Phase были той же природы).
