# AXIOM WORKSTATION — ARCHITECTURE

**Документ:** 2 из 4
**Назначение:** Техническая архитектура Workstation и его связь с AXIOM Engine
**Версия:** 1.1 (учтены правки chrnv по бенчмаркам и открытым вопросам)
**Статус:** Финал, утверждён
**Дата:** 2026-04-30
**Связанные документы:** Vision & Principles (Документ 1), AXIOM Engine spec, External Adapters V3.0, Memory Persistence V1.0

---

## 0. О чём этот документ

Этот документ — **технический**. Он описывает, как Workstation устроен внутри, как связан с Engine, какие сообщения летают между ними, как обрабатываются ошибки связи, как добавлять новые функции в будущем.

Vision & Principles (Документ 1) задаёт **что** мы строим. Этот документ — **как** мы это строим.

Документ предполагает, что читающий знаком с архитектурой AXIOM Engine: домены, Weavers, GUARDIAN, DREAM Phase, COM, UCL. Без этого многие термины будут непонятны.

---

## 1. Архитектурные принципы

### 1.1 Engine — субъект, Workstation — окно

Это **главный принцип**, наследуемый из Vision-документа. Engine существует независимо: запускается, живёт непрерывно, сохраняет своё состояние, продолжается после перезапусков. Workstation — окно к нему. Открытие Workstation — это **подключение к существующей жизни**, а не запуск программы.

Из этого следует:

- **Engine lifecycle** не управляется Workstation в обычном режиме. Engine стартуется отдельно (через CLI, systemd, или особый режим Workstation для разработки).
- **Workstation lifecycle** — это lifecycle подключения. Запуск → discovery → connect → display → disconnect.
- **Engine ничего не знает про Workstation**. Engine просто публикует свой broadcasting, кто-то слушает или нет — это не его забота.
- **Состояние Engine хранится в Engine.** Workstation хранит только настройки UI (geometry окон, активная вкладка, последние использованные адаптеры, история бенчмарков).

### 1.2 Локально в V1.0, готовность к сетевому

V1.0 работает только локально (Workstation и Engine на одной машине, связь через localhost). Архитектура спроектирована так, что переход на сетевое — **точечное расширение**, не переписывание.

Все места, где локальность зашита в код, помечаются как **точки расширения** (см. раздел 9). При переходе на сетевое к ним добавляется: discovery, handshake, шифрование. Внутренний контракт сообщений между Engine и Workstation остаётся неизменным.

### 1.3 Нативный Rust, без HTML/JS

Это ограничение проекта целиком, не только Workstation. Никаких embedded-браузеров, Tauri, Electron. Только Rust + iced. Внешние данные (PDF, JSON, etc.) — через Engine и его External Adapters, не через Workstation.

### 1.4 Workstation не реализует логику Engine

Workstation **не дублирует** функциональность Engine. Никакого FrameWeaver-в-миниатюре в UI. Никакого "локального кеша состояния, который отображается даже если Engine не доступен". Если Engine не доступен — Workstation показывает это явно как **состояние "не подключён"**, не пытается симулировать живую систему.

### 1.5 Workstation — клиент с малым риском

Workstation не должен быть способен сломать Engine. Любое действие из Workstation проходит через те же UCL-каналы и GUARDIAN, что и команды извне. Workstation не имеет привилегированного доступа.

---

## 2. Топология процессов

### 2.1 Базовая схема

```
┌─────────────────────────────────────┐
│         AXIOM Engine                │
│  (отдельный процесс, всегда живёт)  │
│                                     │
│  ┌──────────────────────────────┐   │
│  │ AshtiCore + Over-Domain Layer│   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │ Broadcasting Layer            │   │
│  │ (feature "adapters")          │   │
│  │                               │   │
│  │  WebSocket Server :PORT       │   │
│  │  (multi-client, postcard)     │   │
│  └──────────────┬────────────────┘   │
└─────────────────┼────────────────────┘
                  │ WebSocket (localhost)
                  │
        ┌─────────┴──────────┐
        │                    │
┌───────▼────────┐  ┌────────▼─────────┐
│  Workstation   │  │  Future          │
│  (iced app)    │  │  Companion       │
│                │  │                  │
│  Multi-window  │  │  (другой проект) │
└────────────────┘  └──────────────────┘
```

Несколько важных моментов:

- Engine — **multi-client server**. К нему может подключаться несколько клиентов одновременно. В V1.0 это будет только Workstation, но архитектура позволяет в будущем подключить ещё одного клиента (например, ранний Companion).
- Workstation **не единственный клиент**. У Engine есть и другие способы взаимодействия (CLI, прямой UCL через файл).
- **Связь — через WebSocket, протокол бинарный**, формат сериализации **postcard** (см. раздел 4.1).

### 2.2 Что происходит при запуске Workstation

```
1. Workstation стартует
2. Загружает свои настройки (геометрия окон, last connection target)
3. Открывает главное окно с состоянием "Connecting..."
4. Пробует подключиться к Engine по известному адресу
   (default: ws://127.0.0.1:9876, конфигурируемо)
5а. Engine доступен:
    → handshake
    → запрос на initial snapshot состояния
    → подключение к broadcasting
    → переход в активный режим
5б. Engine не доступен:
    → переход в режим "Engine not running"
    → явное сообщение пользователю, БЕЗ автоматического запуска engine
    → доступны три действия:
      a) Spawn local engine (для разработки, явное действие пользователя)
      b) Configure connection (другой адрес/порт)
      c) Wait and retry (фоновое ожидание)
```

**Принципиально: при отсутствии engine Workstation не запускает его автоматически**. Это явный выбор пользователя, не сюрприз. Engine — субъект, его запуск — событие.

### 2.3 Что происходит при закрытии Workstation

```
1. Workstation сохраняет настройки UI
2. Отправляет "graceful disconnect" в Engine
3. Закрывает WebSocket
4. Завершает свой процесс

Engine: продолжает работать, как и раньше.
```

### 2.4 Что происходит при потере связи

```
1. Workstation замечает потерю WebSocket
2. Главное окно переходит в режим "Reconnecting..."
3. UI показывает последний известный snapshot с overlay "Соединение потеряно"
   (см. 2.4.1 для случая когда snapshot не было)
4. Backoff retry: попытки переподключения с увеличивающимся интервалом
   (1с, 2с, 5с, 10с, 30с, потом каждые 30с)
5. При успешном переподключении:
   → запрос initial snapshot (синхронизировать состояние)
   → удаление overlay
   → возобновление нормальной работы
6. Если пользователь отменяет ожидание:
   → переход в режим "Disconnected" с теми же тремя действиями (раздел 2.2)
```

### 2.4.1 Frozen snapshot vs blank state

При потере связи поведение зависит от того, **успел ли Workstation получить snapshot** до потери:

**Frozen snapshot (был snapshot):**
- Все окна показывают **последнее известное состояние**
- Поверх — overlay с пометкой "Connection lost — showing last known state"
- Времена устаревания отображаются как "since: HH:MM:SS"
- Попытки переподключения работают в фоне

**Blank state (snapshot не было):**
- Окна показывают свой initial layout без данных
- Заглушки вместо метрик
- Сообщение "Waiting for engine connection..." в каждом окне
- Это случается при первом подключении или после рестарта Workstation, если engine ни разу не успел отправить snapshot

Frozen snapshot — преимущественный режим. Blank — fallback для первого старта.

### 2.5 Spawn local engine (режим разработки)

Когда Workstation сам запускает Engine — это **режим разработки**.

```
1. Workstation проверяет, есть ли engine binary в известных путях
2. Запускает engine как child process
3. Ждёт, пока engine станет принимать соединения (с timeout)
4. Подключается как обычный клиент
5. При закрытии Workstation — child process по умолчанию НЕ убивается
6. В UI явно: "engine запущен этим Workstation",
   доступен пункт "Stop engine on close"
```

В нормальной работе — engine стартуется отдельно (CLI или systemd), Workstation подключается.

### 2.6 Shutdown engine из Workstation

Workstation имеет операцию остановки Engine. Это специальная команда, требующая подтверждения.

```
Меню → Engine → Shutdown engine...
  → Диалог: "Остановить engine?"
    Текущее состояние сохранено через Memory Persistence.
    После остановки engine не будет работать, пока его снова не запустить.
    
    [ Cancel ]  [ Shutdown ]  [ Force shutdown ]
  
  → Shutdown:
    Workstation отправляет команду GracefulShutdown в engine
    Engine завершает текущий tick, finalizes Memory Persistence,
    отправляет "shutting down" event, закрывает соединение
  
  → Force shutdown:
    SIGTERM (если был spawn-нут текущим клиентом) или ForceShutdown command
    Без гарантий сохранения состояния
```

Force shutdown — для случаев зависания. Должен быть редким.

---

## 3. Топология окон Workstation (multi-window)

### 3.1 Single application, tabs + detach

Workstation — одно iced-приложение, в котором живут вкладки (System Map, Live Field, Patterns, Dream State, Conversation, Configuration, Files, **Benchmarks**). Любая вкладка может быть **detached** — вытащена в отдельное системное окно.

```
┌──────────────────────────────────────────┐
│  Main Workstation Window                 │
│  ┌────────────────────────────────────┐  │
│  │ Tabs: [Map][Field][Patterns][...]  │  │
│  └────────────────────────────────────┘  │
│  ┌────────────────────────────────────┐  │
│  │  Active tab content                 │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘

  + (detached) ┌─────────────────────────┐
              │  System Map (detached)   │
              │  Living on second monitor│
              └─────────────────────────┘
```

### 3.2 Iced multi-window поддержка

Iced поддерживает multi-window через `iced::multi_window` API. Точный API проверяется на этапе реализации (он развивается между версиями iced 0.12, 0.13+). Архитектура приложения от деталей API не зависит, только от факта поддержки multi-window.

Базовая структура:
- Состояние приложения **общее** для всех окон (одна Model в Elm-architecture)
- Каждое окно — это `WindowId` + view-функция
- События с привязкой к WindowId

### 3.3 Lifecycle detached-окон

```
Main window: tab "System Map" active
User: drag tab out OR menu "Detach this tab"
  → создаётся новый WindowId
  → tab удаляется из main window
  → новое окно отображает System Map

Detached window: user closes it
  → доступны: Re-attach (вернуть как вкладку) или Full close
```

### 3.4 Persistence топологии

При закрытии Workstation сохраняется: какие вкладки detached, geometry, активная вкладка main, размер main. При следующем запуске — топология восстанавливается.

### 3.5 Какие вкладки имеют разрешение на detach

В V1.0 — все, кроме Configuration и Files. Эти две — служебные, обычно ненадолго. Главные четыре (System Map, Live Field, Patterns, Dream State), Conversation и Benchmarks — могут быть detached.

---

## 4. Контракт Engine ↔ Workstation

### 4.1 Транспорт и сериализация

**Транспорт:** WebSocket binary frames.

**Сериализация:** **postcard**. Не JSON, не bincode.

Причины выбора postcard:
- **Производительность.** При высоком broadcasting rate JSON будет узким местом.
- **Размер сообщений.** Бинарный формат значительно компактнее JSON.
- **Совместимость с embedded.** Postcard заточен под no_std и embedded окружение. Это важно для будущего сценария "engine в светильнике-банере с ARM-процессором" (Companion).
- **Простота.** Стабильный API, минимум зависимостей.

Контракт сообщений (раздел 4.2) не зависит от выбора формата — при необходимости можно переключиться без изменений семантики.

**Heartbeat:** WebSocket built-in **ping/pong frames**. Часть стандарта WebSocket, поддерживается всеми библиотеками. Engine отправляет ping каждые 30 секунд, Workstation отвечает pong. Если pong не пришёл за timeout (5-10 секунд) — соединение считается потерянным. Application-level heartbeat сообщений не делаем — это было бы дублированием.

### 4.2 Структура сообщения

```rust
// В отдельном crate axiom-protocol, общий для engine и workstation

pub enum EngineMessage {
    // Engine → Client
    Hello { version: u32, capabilities: u64 },
    Snapshot(SystemSnapshot),
    Event(EngineEvent),
    CommandResult { command_id: u64, result: Result<CommandResultData, String> },
    Bye { reason: ShutdownReason },
}

pub enum ClientMessage {
    // Client → Engine
    Hello { version: u32, client_kind: ClientKind },
    RequestSnapshot,
    Subscribe { event_categories: u64 },
    Command { command_id: u64, command: EngineCommand },
    Bye,
}
```

### 4.3 SystemSnapshot

```rust
pub struct SystemSnapshot {
    pub engine_state: EngineState,        // WAKE / FALLING_ASLEEP / DREAMING / WAKING
    pub current_tick: u64,
    pub current_event: u64,
    
    pub domains: Vec<DomainSnapshot>,     // 11 доменов с состоянием
    pub over_domain: OverDomainSnapshot,  // Guardian, Weavers, DreamPhase
    pub fatigue: FatigueSnapshot,
    pub last_dream_report: Option<DreamReport>,
    
    pub frame_weaver_stats: Option<FrameWeaverStats>,
    pub guardian_stats: GuardianStats,
    pub dream_phase_stats: DreamPhaseStats,
    
    pub adapter_progress: Vec<AdapterProgress>,
}

pub struct DomainSnapshot {
    pub id: u16,
    pub name: String,
    pub config_summary: DomainConfigSummary,
    pub token_count: u32,
    pub connection_count: u32,
    pub temperature_avg: u8,
    pub recent_activity: u32,
    pub layer_activations: [u8; 8],
}
```

В V1.0 принимаем — отправляется один раз при подключении, потом дельта-обновления через events.

### 4.4 EngineEvent — частые мелкие апдейты

```rust
pub enum EngineEvent {
    // Метрики
    Tick { tick: u64, event: u64, hot_path_ns: u64 },
    DomainActivity { domain_id: u16, recent_activity: u32, layer_activations: [u8; 8] },
    
    // События архитектуры
    DreamPhaseTransition { from: EngineState, to: EngineState, trigger: SleepTrigger },
    FrameCrystallized { anchor_id: u32, layers_present: u8, participant_count: u8 },
    FrameReactivated { anchor_id: u32, new_temperature: u8 },
    FramePromoted { source_anchor_id: u32, sutra_anchor_id: u32 },
    GuardianVeto { reason: String, command_summary: String },
    
    // Knowledge import
    AdapterStarted { adapter_id: String, source: String },
    AdapterProgress { adapter_id: String, processed: u64, total: u64 },
    AdapterFinished { adapter_id: String, tokens_added: u32, errors: u32 },
    
    // Benchmarks (см. раздел 7)
    BenchStarted { bench_id: String, run_id: u64 },
    BenchProgress { run_id: u64, completed: u32, total: u32 },
    BenchFinished { run_id: u64, results: BenchResults },
    
    // Внимание
    Alert { level: AlertLevel, category: String, message: String },
}
```

### 4.5 EngineCommand — что Workstation может попросить

```rust
pub enum EngineCommand {
    // Управление сном
    ForceSleep,
    ForceWake,
    
    // Конфигурация
    UpdateConfig { config_section: String, payload: Vec<u8> },
    GetConfig { section: String },
    
    // Knowledge import
    ListAdapters,
    StartImport { adapter_id: String, source_path: String, options: ImportOptions },
    CancelImport { import_id: String },
    
    // Подача текста (для Conversation)
    SubmitText { text: String, target_domain: u16 },
    
    // Отладочное введение токенов
    InjectToken { ... },
    InjectConnection { ... },
    
    // Lifecycle
    GracefulShutdown,
    ForceShutdown,
    
    // Запросы
    RequestFullSnapshot,
    RequestFrameDetails { anchor_id: u32 },
}
```

Команды бенчмарков идут в bench-instance, не в основной engine. См. раздел 7.3 про двух-инстансовую архитектуру.

### 4.6 Subscription и фильтрация событий

```rust
pub enum EventCategory {
    Tick           = 1 << 0,
    DomainActivity = 1 << 1,
    DreamPhase     = 1 << 2,
    Frames         = 1 << 3,
    Guardian       = 1 << 4,
    Adapters       = 1 << 5,
    Benchmarks     = 1 << 6,
    Alerts         = 1 << 7,
}
```

Workstation подписывается на все, кроме Tick (Tick часто, для UI достаточно периодического snapshot).

### 4.7 Throttling broadcasting

- Tick события — раз в N тиков (конфигурируемо)
- DomainActivity — только при изменениях больше threshold
- Если очередь сообщений к клиенту растёт — Engine **дропает** старые с приоритетом сохранять последние/важные
- При полном переполнении — Engine отправляет `Snapshot` вместо потока events (resync)

### 4.8 Event buffer на стороне Workstation

Workstation хранит **bounded buffer** последних событий:

- **Default size: 1000 событий.** Конфигурируемо вниз для слабого железа.
- При переполнении — старые события выбрасываются (FIFO).
- Только in-memory ring buffer, на диск не сохраняется.

1000 — компромисс: достаточно для просмотра последних активностей в Conversation/Patterns, не настолько много, чтобы создавать давление на память.

---

## 5. Архитектура iced-приложения

### 5.1 Elm-architecture в multi-window

```rust
struct WorkstationApp {
    connection: ConnectionState,
    
    engine_snapshot: Option<SystemSnapshot>,
    recent_events: VecDeque<EngineEvent>,    // bounded buffer (1000 default)
    
    window_topology: WindowTopology,
    active_tab_in_main: TabKind,
    settings: UiSettings,
    
    // Per-window state
    system_map_state: SystemMapState,
    live_field_state: LiveFieldState,
    patterns_state: PatternsState,
    dream_state_window: DreamStateWindow,
    conversation_state: ConversationState,
    configuration_state: ConfigurationState,
    files_state: FilesState,
    benchmarks_state: BenchmarksState,
}

enum ConnectionState {
    Disconnected,
    Connecting,
    Reconnecting { attempt: u32, next_retry_in: Duration },
    Connected { engine_version: u32, since: Instant },
}

enum TabKind {
    SystemMap,
    LiveField,
    Patterns,
    DreamState,
    Conversation,
    Configuration,
    Files,
    Benchmarks,
}
```

### 5.2 Update loop

```rust
enum Message {
    // Connection
    Connected, Disconnected, Reconnect,
    
    // Engine messages
    EngineMessageReceived(EngineMessage),
    SendCommand(EngineCommand),
    
    // UI events
    TabSelected(TabKind),
    TabDetached(TabKind),
    TabReattached(TabKind),
    WindowClosed(WindowId),
    
    // Per-window
    SystemMapMessage(system_map::Message),
    ConversationMessage(conversation::Message),
    BenchmarksMessage(benchmarks::Message),
    
    // Periodic
    Tick,  // animation tick для UI
}
```

### 5.3 View per window

```rust
fn view(state: &WorkstationApp, window: WindowId) -> Element<Message> {
    match state.window_topology.tab_for_window(window) {
        TabKind::SystemMap => system_map::view(&state.system_map_state, &state.engine_snapshot),
        TabKind::Benchmarks => benchmarks::view(&state.benchmarks_state),
        // ...
    }
}
```

### 5.4 Subscriptions

```rust
fn subscription(state: &WorkstationApp) -> Subscription<Message> {
    Subscription::batch([
        websocket_subscription(&state.connection),
        time::every(Duration::from_millis(33)).map(|_| Message::Tick),
        // window-specific subscriptions
    ])
}
```

### 5.5 Управление производительностью UI

- **Не рендерить, что не видно.** Detached окно не активно — не пересчитывать его view.
- **Throttle re-renders.** 30-60 fps максимум.
- **Tier visualization.** Live Field — самое дорогое окно. Снижать качество если FPS падает.

---

## 6. Knowledge Import / External Adapters

### 6.1 Назначение

Workstation предоставляет окно к подсистеме External Adapters. Сам по себе Workstation не реализует адаптеры — это работа Engine.

### 6.2 Что такое External Adapter

Внешний адаптер — компонент в Engine, который умеет:
- Принимать данные в каком-то формате (PDF, plain text, JSON, etc.)
- Преобразовывать их в правильные структуры AXIOM
- Размещать в правильных доменах (обычно EXPERIENCE с `IMPORT_WEIGHT_FACTOR=0.7`)

Адаптеры реализованы в Engine. Workstation только показывает их и управляет запуском/прогрессом/отменой.

### 6.3 Workflow knowledge import

```
1. Пользователь открывает вкладку Files
2. Workstation запрашивает у Engine список адаптеров (ListAdapters command)
3. Engine отвечает списком: ["pdf", "plain_text", "json_dump", ...]
4. Пользователь выбирает источник (через системный диалог)
   Workstation определяет MIME type, предлагает подходящий адаптер
5. Опции импорта (адаптер-специфичные)
6. Пользователь нажимает Start Import
7. Workstation отправляет StartImport command
8. Engine начинает асинхронно, отправляет AdapterStarted event
9. По мере прогресса — AdapterProgress events
10. По завершении — AdapterFinished event
11. Workstation показывает прогресс в окне Files и в System Map
12. После завершения — данные доступны системе для обработки
```

### 6.4 Cancel и pause

- **Cancel** — прерывание импорта. Уже добавленные токены остаются (нет rollback в V1.0).
- **Pause** — в V1.0 не реализуется (deferred).

### 6.5 Параллельные импорты

В V1.0 Engine может запускать несколько импортов параллельно. Workstation поддерживает это в UI: список активных импортов с прогрессом каждого.

### 6.6 Безопасность импорта

Импортируемые данные **уже фильтруются GUARDIAN**. Если PDF содержит то, что нарушает CODEX — GUARDIAN отклоняет соответствующие токены. Workstation видит это через counter `errors > 0` в AdapterFinished event.

---

## 7. Benchmarks — измерение производительности

### 7.1 Назначение

Workstation предоставляет встроенный механизм для запуска бенчмарков из UI без выхода в терминал:
- Запуск стандартных бенчмарков (hot path tick, FrameWeaver overhead, DREAM Phase overhead) одной кнопкой
- Просмотр результатов в реальном времени
- **История замеров с версионированием** в постоянном файле
- Экспорт результатов в файл для внешнего использования

Эта функциональность критична для развития проекта: chrnv может видеть регрессии сразу после изменений, не запуская `cargo bench` вручную.

### 7.2 Основные бенчмарки V1.0

В V1.0 — простые предустановленные бенчмарки, без конструктора сценариев.

| Имя бенчмарка        | Что измеряет                                  | Параметры                      |
|----------------------|-----------------------------------------------|--------------------------------|
| `hot_path_tick`      | Время одного тика без активной нагрузки      | Iterations (default 10000)     |
| `hot_path_50_tokens` | Время тика с 50 токенами в LOGIC             | Iterations (default 10000)     |
| `frameweaver_overhead` | A/B/C/D тест влияния FrameWeaver           | Iterations per phase (5000)    |
| `dream_phase_overhead` | Стоимость Dream Phase в pipeline           | Iterations (default 5000)      |
| `domain_tick`        | Стоимость tick одного домена                  | Domain ID, Iterations          |
| `crystallization_throughput` | Сколько Frame в секунду может кристаллизовать | Duration in seconds      |

Каждый бенчмарк имеет 1-2 параметра. Конструктор кастомных сценариев — deferred (V2.0).

### 7.3 Архитектура: bench-instance Engine

Бенчмарки **не запускаются на основном engine**. Это нарушило бы принцип "Engine — субъект": замер требует приостановки нормальной работы, чистого окружения, контроля над нагрузкой.

Решение: Workstation запускает **отдельный bench-instance engine** — отдельный процесс с особой конфигурацией, специально для бенчмарка. По завершении он закрывается.

```
1. User: нажимает "Run hot_path_tick" в окне Benchmarks
2. Workstation: spawn-ит engine binary с флагом --bench-mode и параметрами
3. Bench-instance engine стартует, инициализирует чистое состояние
4. Bench-instance запускает указанный бенчмарк
5. Прогресс отправляется через WebSocket в Workstation 
   (тот же протокол, но с категорией Benchmarks)
6. По завершении — BenchFinished с результатами
7. Bench-instance shutting down
8. Workstation: записывает результаты в bench-history.md
9. Workstation: отображает в окне с историей предыдущих запусков
```

**Параллельность:** bench-instance работает на отдельном порту (или через UNIX socket). Основной engine продолжает работать как обычно. Workstation может одновременно подключаться к обоим.

### 7.4 Файл истории бенчмарков

**Имя файла:** `bench-history.md` (располагается в `dirs::data_dir()/axiom-workstation/`).

**Формат:** markdown с таблицами и заголовками. Каждый запуск — отдельная секция.

**Пример содержимого:**

```markdown
# AXIOM Workstation — Bench History

## 2026-04-30 14:32:15 | engine 2.0.0 | workstation 1.0.0

### hot_path_tick
- Iterations: 10000
- Median: 238.5 ns
- P50: 235.0 ns
- P99: 290.0 ns
- Std dev: 12.3 ns
- Environment: Linux, x86_64

## 2026-04-30 14:35:42 | engine 2.0.0 | workstation 1.0.0

### frameweaver_overhead
- Iterations: 5000 per phase
- Phase A (no FrameWeaver): 105.2 ns
- Phase B (registered, idle): 112.0 ns
- Phase C (active scan): 145.3 ns
- Phase D (active with patterns): 238.5 ns
```

Преимущества markdown формата:
- Читается человеком без инструментов
- Согласуется с лексикой проекта (всё в markdown)
- При экспорте — это уже готовый документ
- Workstation может рендерить графики, парся свою же markdown-таблицу

**Если файл не существует — Workstation создаёт его при первом запуске бенчмарка.**

### 7.5 UI окна Benchmarks

Окно Benchmarks разделено на две части:

- **Слева — Run panel.** Список доступных бенчмарков с их параметрами и кнопкой [Run]. Пользователь выбирает бенчмарк, может изменить параметры (число итераций), нажимает Run.
- **Справа — Results panel.** Показывает результат текущего/последнего бенчмарка + историю предыдущих запусков. Тренд по последним 10 запускам в виде sparkline. Список всех запусков с возможностью открыть полный текст.

Полная композиция окна — в Документе 3.

### 7.6 Экспорт результатов

В окне Benchmarks — кнопка "Export results to file...". Открывает системный файловый диалог, предлагает сохранить копию `bench-history.md` (или подмножество — выбранные запуски) куда захочет пользователь.

Это позволяет:
- Поделиться результатами без копирования из appdir
- Сохранить snapshot перед большими изменениями
- Прикрепить к коммиту/issue/PR

### 7.7 Что доступно в bench-instance

Bench-instance — это **тот же engine binary**, запущенный с флагом `--bench-mode`. Внутри он:
- Не сохраняет в Memory Persistence
- Использует in-memory storage без persistence
- Имеет минимум external integrations
- Включает `criterion`-подобный код для precise timing

Это **существующий код бенчмарков из `axiom-bench`**, переупакованный для программного вызова через CLI flag и отчётности через WebSocket.

---

## 8. Lifecycle и persistence

### 8.1 Что хранится в Workstation

Workstation хранит только UI-настройки и историю:

- Геометрия и позиции окон (main + detached)
- Активная вкладка в main
- Какие вкладки detached
- Адрес engine для подключения (host:port)
- Last connection mode (connect/spawn)
- UI preferences
- История recent imports
- **bench-history.md** — постоянная история бенчмарков (раздел 7.4)

Хранение — через `dirs::config_dir()` для настроек и `dirs::data_dir()` для bench-history.

### 8.2 Что НЕ хранится в Workstation

- Состояние Engine. Никогда.
- История чата. Conversation в V1.0 — без long-term history.
- Snapshots состояния. Не делаем "save game" режима.

### 8.3 First run vs subsequent runs

```
First run (нет config файла):
1. Создать config файл с дефолтами
2. Открыть Welcome screen (см. Документ 3)
3. После первого подключения — открыть System Map в main

Subsequent runs:
1. Загрузить config
2. Восстановить window topology
3. Подключиться к Engine
4. Восстановить активные вкладки
```

### 8.4 Engine version compatibility

В Hello-сообщении есть `version: u32`. При несовпадении major версии — предупреждение:

```
Engine version: 2.x.x
Workstation version: 1.x.x

Версии могут быть несовместимы.
[ Continue anyway ]  [ Disconnect ]
```

В V1.0 этого достаточно. Полный compatibility matrix — для будущего.

---

## 9. Готовность к сетевому режиму (deferred)

### 9.1 Точки расширения

Помечены в коде как `// SCALE-POINT: networking`:

1. **Discovery.** Сейчас — фиксированный `127.0.0.1:9876`. Будущее — mDNS или DNS-имена.
2. **Handshake.** Сейчас — версия + capabilities. Будущее — pre-shared key, token-based auth.
3. **Шифрование.** Сейчас — ws://. Будущее — wss:// с TLS.
4. **Session resumption.** Сейчас — каждое подключение с полного snapshot. Будущее — incremental resume по event_id.
5. **Latency tolerance.** Сейчас — низкая латентность предполагается. Будущее — буферизация, predictive UI updates.
6. **Connection multiplexing.** Сейчас — одно WebSocket. Будущее — отдельный канал для bulk transfers.

### 9.2 Что менять не нужно

Контракт сообщений (раздел 4) одинаков для локального и сетевого. Семантика взаимодействия переносится без изменений.

### 9.3 Когда переключаться

Сетевой режим V2.0 имеет смысл когда:
- У chrnv появляется отдельное железо для Engine
- Нужно дистанционное наблюдение
- Companion разрабатывается и хочет общаться с Engine на расстоянии

---

## 10. Отказы и observability

### 10.1 Индикатор подключения

В углу main window:

```
● Connected (engine v2.1.0, since 14:32)         [зелёный шалфей]
● Reconnecting (attempt 3, next in 5s)           [тёплый янтарный]
● Disconnected — Engine not running              [тёплый красный]
```

При проблемах — клик на индикатор раскрывает диагностику.

### 10.2 Логирование Workstation

Workstation пишет логи в `dirs::cache_dir()/axiom-workstation/log/`. Через `tracing` crate. Уровни: error, warn, info, debug. Default уровень — info.

Логи **только** Workstation. Логи Engine — отдельные, на стороне Engine.

### 10.3 Что Workstation НЕ делает при ошибках

- **Не рестартует Engine автоматически.** Engine — субъект.
- **Не пытается фиксить состояние Engine.**
- **Не молчит при ошибках.** Любая ошибка должна быть видна.

---

## 11. Расширяемость

### 11.1 Добавить новое окно

```
1. Создать модуль workstation::ui::new_window
2. Реализовать NewWindowState и его update/view
3. Добавить вариант в TabKind
4. Добавить per-window state в WorkstationApp
5. Добавить в меню "Open new window"
```

Никаких глобальных изменений в архитектуре не требуется.

### 11.2 Добавить новый тип EngineEvent

```
1. Расширить enum EngineEvent в axiom-protocol
2. Engine: добавить генерацию события
3. Workstation: handle нового варианта в update loop
```

### 11.3 Добавить новый тип EngineCommand

```
1. Расширить enum EngineCommand
2. Engine: handle нового варианта (через UCL или прямое действие)
3. Workstation: добавить UI для отправки команды
```

### 11.4 Добавить новый бенчмарк

```
1. Реализовать бенчмарк в axiom-bench (как сейчас)
2. Зарегистрировать в bench registry под именем
3. Добавить в дефолтный список Benchmarks UI
4. (Опционально) добавить параметры в UI
```

### 11.5 Версионирование протокола

- Patch (X.Y.**Z**): добавление новых событий с дефолтным поведением
- Minor (X.**Y**.0): добавление команд или полей с обратной совместимостью
- Major (**X**.0.0): несовместимые изменения

В V1.0 — версия 1.0.0.

### 11.6 Подключение Companion в будущем

Companion подключается к тому же Engine как другой клиент. ClientKind в Hello помогает Engine различать Workstation и Companion.

Workstation и Companion **не общаются напрямую**. Координация — через Engine.

---

## 12. Резюме

Workstation — клиентское приложение iced, подключающееся к работающему Engine через локальный WebSocket. Архитектура:

- **Engine — постоянный субъект**, Workstation — окно к нему
- **Локально в V1.0**, готовность к сетевому через точки расширения
- **Multi-window iced** с tabs + detach
- **Бинарный протокол postcard** с типизированными сообщениями
- **WebSocket ping/pong** для heartbeat
- **Knowledge Import** через External Adapters Engine
- **Benchmarks** через bench-instance engine, с историей в `bench-history.md`
- **Lifecycle полностью независим** от Engine
- **Spawn engine локально** — режим разработки, не основной режим
- **Shutdown engine** — с подтверждением
- **Frozen snapshot + overlay** при потере связи; blank state для первого подключения
- **Engine не запускается автоматически** при первом запуске Workstation

Workstation **не реализует** логику Engine, **не дублирует** функциональность, **не пытается чинить** Engine. Принципиальное разделение ответственности.

Эта архитектура определяет дизайн каждого окна (Документ 3) и порядок реализации (Документ 4).

---

## Приложение A: Структура crate-ов

```
axiom-workspace/
├── crates/
│   ├── axiom-core/                  (existing — core types)
│   ├── axiom-runtime/               (existing — engine runtime)
│   ├── axiom-bench/                 (existing — расширяется
│   │                                 поддержкой --bench-mode CLI flag)
│   │
│   ├── axiom-protocol/              ← НОВЫЙ
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── messages.rs          (EngineMessage, ClientMessage)
│   │       ├── snapshot.rs          (SystemSnapshot, ...)
│   │       ├── events.rs            (EngineEvent)
│   │       ├── commands.rs          (EngineCommand)
│   │       └── bench.rs             (BenchResults, BenchSpec)
│   │
│   ├── axiom-broadcasting/          ← НОВЫЙ или расширение существующего
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── server.rs            (WebSocket server в Engine)
│   │       └── throttle.rs
│   │
│   └── axiom-workstation/           ← НОВЫЙ
│       └── src/
│           ├── main.rs
│           ├── app.rs
│           ├── connection/
│           ├── ui/
│           │   ├── system_map/
│           │   ├── live_field/
│           │   ├── patterns/
│           │   ├── dream_state/
│           │   ├── conversation/
│           │   ├── configuration/
│           │   ├── files/
│           │   ├── benchmarks/      ← новое окно
│           │   └── shared/
│           ├── windowing/
│           ├── persistence/
│           ├── bench/               (управление bench-instance engine)
│           └── theme/
```

## Приложение B: Глоссарий

- **WorkstationApp** — главная структура приложения iced
- **WindowId** — идентификатор окна в iced multi-window
- **TabKind** — тип вкладки
- **ConnectionState** — состояние подключения к Engine
- **SystemSnapshot** — полный snapshot состояния Engine
- **EngineEvent / EngineCommand** — сообщения между Engine и Workstation
- **Detached window** — вкладка, вытащенная в отдельное системное окно
- **Spawn local engine** — режим разработки
- **Bench-instance** — отдельный процесс engine для бенчмарка
- **Throttling** — ограничение частоты broadcasting
- **Subscribe** — подписка на категории событий
- **Frozen snapshot** — последнее известное состояние при потере связи
- **Blank state** — пустое UI при первом подключении

## Приложение C: Закрытые вопросы

Все вопросы, бывшие открытыми в первоначальном черновике, теперь решены:

| Вопрос | Резолюция |
|--------|-----------|
| Q1: Бенчмарки из Workstation? | **Да**, отдельная вкладка Benchmarks через bench-instance, с историей в bench-history.md (раздел 7) |
| Q2: bincode vs postcard? | **postcard** — лучшая готовность к будущему embedded engine (раздел 4.1) |
| Q3: Default behavior при engine not found? | **Не запускать автоматически** — явное действие пользователя (раздел 2.2) |
| Q4: Размер event buffer? | **1000 событий** default, конфигурируемо вниз (раздел 4.8) |
| Q5: Heartbeat механизм? | **WebSocket ping/pong** (раздел 4.1) |
| Q6: System Map при Reconnecting? | **Frozen snapshot + overlay**, blank state для первого подключения (раздел 2.4.1) |
| Q7: Iced 0.13+ multi-window? | **Проверяется на этапе реализации**, общая архитектура от деталей API не зависит (раздел 3.2) |

Новые вопросы из реализации — в errata-документе для будущей V1.2.
