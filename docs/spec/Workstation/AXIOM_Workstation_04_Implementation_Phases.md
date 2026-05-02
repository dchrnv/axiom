# AXIOM WORKSTATION — IMPLEMENTATION PHASES

**Документ:** 4 из 4 (финальный)
**Назначение:** Пошаговый план реализации Workstation V1.0
**Версия:** 1.0
**Статус:** Финальный документ пакета проектирования
**Дата:** 2026-04-30
**Связанные документы:** Vision & Principles (Документ 1), Architecture (Документ 2), Windows Design (Документ 3 части A-D), AXIOM_Workstation_DEFERRED.md

---

## 0. О чём этот документ

Документы 1-3 определили **что** строится и **как** оно должно работать. Документ 4 — **в каком порядке** это строится.

Это план для Sonnet (имплементер). Структура похожа на план DREAM Phase Stabilization, который сработал хорошо: этапы со строгим порядком, конкретные алгоритмы, тесты, критерии готовности.

Главное отличие — этот план **с нуля строит новый проект**, не чинит существующий. Поэтому первые этапы посвящены **фундаменту** (новые crate-ы, базовая инфраструктура), и только потом — окна одно за другим.

### 0.1 Стратегия: снизу вверх

Решение принято: **сначала фундамент, потом окна**. Причины:
- Проект большой, переделки фундамента дорого стоят
- Спецификация детальная — фундамент проектируем с уверенностью
- Готов проектировать сразу как продукт, не как MVP

Альтернатива (вертикальный срез) отклонена.

### 0.2 Грубая оценка времени

8 этапов реализации, плюс этап 0 (подготовка). Каждый этап — от 1 до 3 недель работы Sonnet.

**Полный V1.0 — 3-4 месяца плотной работы.** Это значимый проект. План должен это учитывать.

После каждого этапа — отчёт chrnv, проверка результатов, планирование следующего. Не торопиться, не пытаться сделать два этапа сразу.

### 0.3 Что строго НЕ делаем в этом плане

- Не делаем Vital Signs (это Companion, см. DEFERRED.md)
- Не делаем визуальный дизайн (это отдельный этап после реализации, или параллельно)
- Не делаем localization
- Не делаем dark theme в V1.0
- Не делаем drag-and-drop в Files если iced не поддерживает в текущей версии
- Не делаем сложные responsive layouts для маленьких окон

---

## Этап 0 — подготовка проекта (≈1-2 дня)

**Цель:** настроить рабочую среду, ничего не реализуя.

### 0.1 Создать структуру crate-ов

В существующем `axiom-workspace/` добавить три новых crate-а:

```
crates/
├── axiom-protocol/        ← НОВЫЙ
├── axiom-broadcasting/    ← НОВЫЙ
└── axiom-workstation/     ← НОВЫЙ
```

Каждый со своим `Cargo.toml`, `src/lib.rs` (или `src/main.rs` для workstation), README.md.

В корневом `Cargo.toml` workspace добавить новые crate-ы в members.

### 0.2 Базовые зависимости

**axiom-protocol:**
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
postcard = { version = "1", features = ["use-std"] }
```

**axiom-broadcasting:**
```toml
[dependencies]
axiom-core = { path = "../axiom-core" }
axiom-runtime = { path = "../axiom-runtime" }
axiom-protocol = { path = "../axiom-protocol" }
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.21"
postcard = "1"
```

**axiom-workstation:**
```toml
[dependencies]
axiom-protocol = { path = "../axiom-protocol" }
iced = { version = "0.13" }  # точная версия проверится на этапе 2
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.21"
postcard = "1"
serde = { version = "1", features = ["derive"] }
dirs = "5"
tracing = "0.1"
tracing-subscriber = "0.3"
```

Точные версии — на момент реализации, могут отличаться.

### 0.3 Настроить логирование и базовые тесты

В каждом crate настроить `tracing` для логирования. Минимальные unit-тесты на placeholder-уровне (`#[test] fn it_compiles() { }`) — чтобы CI работал.

### 0.4 Документация README

В каждом crate — простой README с описанием его роли:
- `axiom-protocol/README.md`: общие типы для Engine ↔ Workstation
- `axiom-broadcasting/README.md`: WebSocket сервер для Engine
- `axiom-workstation/README.md`: клиентское приложение iced

### Критерий готовности этапа 0

- [ ] Три crate-а созданы и компилируются
- [ ] Все зависимости разрешаются
- [ ] `cargo build --workspace` проходит
- [ ] `cargo test --workspace` проходит (даже если тесты — пустые)
- [ ] CI зелёный

---

## Этап 1 — axiom-protocol (≈1 неделя)

**Цель:** определить все типы сообщений Engine ↔ Workstation. Без сетевой части. Просто типы и сериализация.

### 1.1 Базовые типы

В `axiom-protocol/src/messages.rs`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EngineMessage {
    Hello { version: u32, capabilities: u64 },
    Snapshot(SystemSnapshot),
    Event(EngineEvent),
    CommandResult { 
        command_id: u64, 
        result: Result<CommandResultData, String> 
    },
    Bye { reason: ShutdownReason },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Hello { version: u32, client_kind: ClientKind },
    RequestSnapshot,
    Subscribe { event_categories: u64 },
    Command { command_id: u64, command: EngineCommand },
    Bye,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClientKind {
    Workstation,
    Companion,    // зарезервировано
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShutdownReason {
    Normal,
    EngineCrashed,
    ClientRequested,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandResultData {
    None,
    AdapterList(Vec<AdapterInfo>),
    Config(ConfigData),
    // ... добавляются по мере реализации команд
}
```

### 1.2 SystemSnapshot

В `axiom-protocol/src/snapshot.rs`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemSnapshot {
    pub engine_state: EngineState,
    pub current_tick: u64,
    pub current_event: u64,
    
    pub domains: Vec<DomainSnapshot>,
    pub over_domain: OverDomainSnapshot,
    pub fatigue: FatigueSnapshot,
    pub last_dream_report: Option<DreamReport>,
    
    pub frame_weaver_stats: Option<FrameWeaverStats>,
    pub guardian_stats: GuardianStats,
    pub dream_phase_stats: DreamPhaseStats,
    
    pub adapter_progress: Vec<AdapterProgress>,
}

// + DomainSnapshot, OverDomainSnapshot, FatigueSnapshot, и т.д.
```

Полный список структур — см. Документ 2 раздел 4.3.

### 1.3 Events

В `axiom-protocol/src/events.rs`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EngineEvent {
    Tick { tick: u64, event: u64, hot_path_ns: u64 },
    DomainActivity { domain_id: u16, ... },
    DreamPhaseTransition { ... },
    FrameCrystallized { ... },
    FrameReactivated { ... },
    FramePromoted { ... },
    GuardianVeto { ... },
    AdapterStarted { ... },
    AdapterProgress { ... },
    AdapterFinished { ... },
    BenchStarted { ... },
    BenchProgress { ... },
    BenchFinished { ... },
    Alert { ... },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
}
```

Полный список — Документ 2 раздел 4.4.

### 1.4 Commands

В `axiom-protocol/src/commands.rs`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EngineCommand {
    ForceSleep,
    ForceWake,
    UpdateConfig { config_section: String, payload: Vec<u8> },
    GetConfig { section: String },
    ListAdapters,
    StartImport { ... },
    CancelImport { import_id: String },
    SubmitText { text: String, target_domain: u16 },
    InjectToken { ... },
    InjectConnection { ... },
    GracefulShutdown,
    ForceShutdown,
    RequestFullSnapshot,
    RequestFrameDetails { anchor_id: u32 },
}
```

### 1.5 Bench-types

В `axiom-protocol/src/bench.rs`:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchSpec {
    pub bench_id: String,
    pub iterations: u32,
    pub options: BenchOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchResults {
    pub bench_id: String,
    pub iterations: u32,
    pub median_ns: f64,
    pub p50_ns: f64,
    pub p99_ns: f64,
    pub std_dev_ns: f64,
    pub environment: BenchEnvironment,
}
```

### 1.6 Категории событий (битовые флаги)

```rust
pub mod event_category {
    pub const TICK: u64           = 1 << 0;
    pub const DOMAIN_ACTIVITY: u64 = 1 << 1;
    pub const DREAM_PHASE: u64     = 1 << 2;
    pub const FRAMES: u64          = 1 << 3;
    pub const GUARDIAN: u64        = 1 << 4;
    pub const ADAPTERS: u64        = 1 << 5;
    pub const BENCHMARKS: u64      = 1 << 6;
    pub const ALERTS: u64          = 1 << 7;
    
    pub const ALL: u64 = !0;
    pub const DEFAULT: u64 = ALL & !TICK;  // всё кроме Tick
}
```

### 1.7 Тесты сериализации

Для каждого основного типа — round-trip тест:

```rust
#[test]
fn engine_message_serializes_and_deserializes() {
    let original = EngineMessage::Hello { version: 1, capabilities: 0 };
    let bytes = postcard::to_stdvec(&original).unwrap();
    let decoded: EngineMessage = postcard::from_bytes(&bytes).unwrap();
    assert!(matches!(decoded, EngineMessage::Hello { version: 1, .. }));
}

#[test]
fn snapshot_serializes_and_deserializes() {
    let snapshot = SystemSnapshot { /* ... */ };
    let bytes = postcard::to_stdvec(&snapshot).unwrap();
    let decoded: SystemSnapshot = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.current_tick, snapshot.current_tick);
}
```

Минимум один тест на каждый вариант enum-ов EngineMessage, ClientMessage, EngineEvent, EngineCommand.

### 1.8 Версионирование протокола

Создать константу `PROTOCOL_VERSION` в `axiom-protocol/src/lib.rs`:

```rust
pub const PROTOCOL_VERSION: u32 = 0x01_00_00_00;  // 1.0.0
```

Документировать правила версионирования из Документа 2 раздел 11.5 в README.

### Критерий готовности этапа 1

- [ ] Все типы сообщений определены
- [ ] postcard сериализация работает для всех вариантов
- [ ] Round-trip тесты проходят (минимум 30 тестов)
- [ ] Документация типов через `///` комментарии
- [ ] axiom-protocol компилируется без warnings
- [ ] `PROTOCOL_VERSION` определён

---

## Этап 2 — axiom-broadcasting + Engine integration (≈2 недели)

**Цель:** WebSocket сервер на стороне Engine. Engine начинает публиковать события, к нему можно подключиться извне.

### 2.1 WebSocket Server

В `axiom-broadcasting/src/server.rs`:

```rust
pub struct BroadcastServer {
    addr: SocketAddr,
    clients: Arc<RwLock<Vec<Client>>>,
    // ...
}

impl BroadcastServer {
    pub async fn run(&self) -> Result<(), BroadcastError> {
        // accept connections
        // handle handshake
        // dispatch messages from engine to clients
        // dispatch commands from clients to engine
    }
    
    pub async fn broadcast(&self, message: EngineMessage) {
        // send to all subscribed clients
    }
}

pub struct Client {
    id: u64,
    kind: ClientKind,
    subscribed_categories: u64,
    sender: mpsc::UnboundedSender<EngineMessage>,
}
```

**Используем tokio + tokio-tungstenite.** Это самые проверенные варианты в Rust для WebSocket.

### 2.2 Handshake protocol

```
Client → Server: ClientMessage::Hello { version, client_kind }
Server: проверяет версию (PROTOCOL_VERSION должна совпадать по major)
Server → Client: EngineMessage::Hello { version, capabilities }
Server: добавляет клиента в список
Server → Client: EngineMessage::Snapshot(<полный snapshot>)
```

При несовпадении версии — Server отвечает `Bye { reason }` и закрывает соединение.

### 2.3 Throttling и dropping

Из Документа 2 раздел 4.7:

```rust
pub struct ClientChannel {
    sender: mpsc::Sender<EngineMessage>,  // bounded channel
    dropped_count: AtomicU64,
}

impl ClientChannel {
    pub fn send(&self, msg: EngineMessage) -> Result<(), SendError> {
        match self.sender.try_send(msg) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => {
                // Очередь полная — дропаем сообщение
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                Err(SendError::Dropped)
            }
            Err(TrySendError::Closed(_)) => Err(SendError::Disconnected),
        }
    }
}
```

При большом количестве дропов — отправляем клиенту полный Snapshot для resync.

### 2.4 Heartbeat (ping/pong)

WebSocket встроенный ping/pong:

```rust
// Сервер периодически отправляет ping
let ping_interval = Duration::from_secs(30);
let pong_timeout = Duration::from_secs(10);

async fn heartbeat_loop(client: &Client) {
    loop {
        tokio::time::sleep(ping_interval).await;
        client.send_ping().await?;
        
        match tokio::time::timeout(pong_timeout, client.recv_pong()).await {
            Ok(Ok(_)) => continue,
            _ => break,  // pong не пришёл — соединение разорвано
        }
    }
}
```

### 2.5 Engine integration

В `axiom-runtime` добавить feature `broadcasting`:

```toml
[features]
broadcasting = ["axiom-broadcasting"]
```

В коде Engine:

```rust
#[cfg(feature = "broadcasting")]
mod broadcasting_integration {
    pub fn start(engine: &Engine, port: u16) -> BroadcastServer {
        let server = BroadcastServer::new(format!("127.0.0.1:{}", port));
        
        // Подключаем engine.events() → server.broadcast()
        engine.subscribe_events(|event| {
            server.broadcast(EngineMessage::Event(event));
        });
        
        // Подключаем server.commands() → engine.process_command()
        server.set_command_handler(|cmd| {
            engine.process_external_command(cmd)
        });
        
        server
    }
}
```

### 2.6 Snapshot generation

Engine должен уметь генерировать полный SystemSnapshot по запросу:

```rust
impl Engine {
    pub fn build_snapshot(&self) -> SystemSnapshot {
        SystemSnapshot {
            engine_state: self.dream_phase_state(),
            current_tick: self.current_tick(),
            // ... собираем со всех компонентов
        }
    }
}
```

Это требует доступа к внутренним состояниям всех компонентов. Может потребоваться добавление методов в существующие structs (Domain, Guardian, FrameWeaver, и т.д.) для возврата своих snapshot-ов.

### 2.7 Тесты

**Test 2.7.a — full handshake:**
```
1. Запустить сервер на test порту
2. Подключиться WebSocket клиентом
3. Отправить Hello с правильной версией
4. Получить Hello + Snapshot
5. Отключиться
```

**Test 2.7.b — version mismatch:**
```
1. Запустить сервер версии X
2. Подключиться с Hello версии Y (не совместимая)
3. Получить Bye, соединение закрыто
```

**Test 2.7.c — multiple clients:**
```
1. Подключить 3 клиента одновременно
2. Engine публикует событие
3. Все 3 клиента получают (если подписаны)
```

**Test 2.7.d — subscription filter:**
```
1. Клиент 1 подписан на ALL
2. Клиент 2 подписан только на FRAMES
3. Engine публикует Tick event
4. Только клиент 1 получает
5. Engine публикует FrameCrystallized
6. Оба получают
```

**Test 2.7.e — heartbeat:**
```
1. Подключиться
2. Не отвечать на ping в течение 15 секунд
3. Сервер закрывает соединение
```

**Test 2.7.f — dropping under load:**
```
1. Клиент медленно читает
2. Engine публикует много событий быстро
3. Клиентская очередь переполняется
4. Старые события дропаются с counting
5. Клиент в итоге получает Snapshot для resync
```

### Критерий готовности этапа 2

- [ ] WebSocket сервер работает, multi-client
- [ ] Handshake правильный
- [ ] Throttling и dropping реализованы
- [ ] Heartbeat работает
- [ ] Engine может генерировать SystemSnapshot
- [ ] Engine публикует Events через сервер
- [ ] Engine принимает Commands через сервер
- [ ] Все тесты 2.7.a-f проходят
- [ ] Существующие тесты Engine остаются зелёными
- [ ] Hot path не просел (broadcasting должен быть **дёшевым** при отсутствии клиентов)

**Hot path budget:** broadcasting должен добавлять не более 5-10 ns на тик когда клиентов нет. Если есть клиенты — затраты пропорциональны числу подписанных и уровню throttling.

---

## Этап 3 — axiom-workstation базовая инфраструктура (≈1.5 недели)

**Цель:** запускаемое приложение iced. Подключается к Engine. Показывает один пустой экран. Без окон, без табов.

### 3.1 main.rs и базовая структура

```rust
fn main() -> iced::Result {
    init_logging();
    let settings = WorkstationApp::settings();
    iced::run("AXIOM Workstation", WorkstationApp::update, WorkstationApp::view)
        .settings(settings)
        .run()
}
```

### 3.2 WorkstationApp (Model)

```rust
pub struct WorkstationApp {
    connection: ConnectionState,
    engine_snapshot: Option<SystemSnapshot>,
    recent_events: VecDeque<EngineEvent>,  // bounded buffer 1000
    settings: UiSettings,
}

pub enum ConnectionState {
    Disconnected,
    Connecting,
    Reconnecting { attempt: u32, next_retry_in: Duration },
    Connected { engine_version: u32, since: Instant },
}

pub struct UiSettings {
    engine_address: String,  // default "127.0.0.1:9876"
    // ... другие поля по мере необходимости
}
```

### 3.3 Connection management

В `axiom-workstation/src/connection/`:

```rust
pub struct WebSocketClient {
    addr: String,
    state: Arc<Mutex<ClientState>>,
    sender: Option<mpsc::UnboundedSender<ClientMessage>>,
}

impl WebSocketClient {
    pub async fn connect(&mut self) -> Result<(), ConnectionError> { ... }
    pub async fn send(&self, msg: ClientMessage) -> Result<(), SendError> { ... }
    pub async fn close(&mut self) { ... }
}
```

Subscriptions в iced для интеграции:

```rust
fn subscription(state: &WorkstationApp) -> Subscription<Message> {
    websocket_subscription(&state.connection)
}
```

### 3.4 Reconnection logic

Backoff retry:

```rust
async fn reconnect_loop(client: &mut WebSocketClient) {
    let intervals = [1, 2, 5, 10, 30];  // секунды
    let mut idx = 0;
    
    loop {
        let delay = Duration::from_secs(intervals[idx.min(intervals.len()-1)]);
        tokio::time::sleep(delay).await;
        
        match client.connect().await {
            Ok(_) => break,
            Err(_) => idx += 1,
        }
    }
}
```

### 3.5 Persistence настроек

```rust
fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("axiom-workstation")
        .join("config.toml")
}

fn load_settings() -> UiSettings {
    fs::read_to_string(config_path())
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_settings(settings: &UiSettings) {
    let toml = toml::to_string(settings).unwrap();
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, toml).unwrap();
}
```

### 3.6 Базовый view

В этом этапе UI крайне примитивный — просто текст "AXIOM Workstation" и индикатор подключения:

```rust
fn view(state: &WorkstationApp) -> Element<Message> {
    column![
        text("AXIOM Workstation").size(24),
        connection_indicator(&state.connection),
    ]
    .padding(20)
    .into()
}

fn connection_indicator(conn: &ConnectionState) -> Element<Message> {
    match conn {
        Connected { engine_version, since } => 
            text(format!("● Connected v{} ({}s ago)", engine_version, since.elapsed().as_secs())),
        Connecting => text("● Connecting..."),
        Disconnected => text("● Disconnected"),
        Reconnecting { attempt, .. } => 
            text(format!("● Reconnecting (attempt {})", attempt)),
    }
    .into()
}
```

### 3.7 Тесты

Для GUI приложения — сложнее тестировать. Минимально:

**Test 3.7.a — config persistence:**
```
1. Создать UiSettings с custom адресом
2. save_settings()
3. load_settings()
4. Проверить что прочитано то же самое
```

**Test 3.7.b — websocket integration:**
```
1. Запустить test broadcasting сервер на random порту
2. Запустить WebSocketClient
3. Connect
4. Проверить что прошёл handshake
5. Disconnect
```

**Test 3.7.c — reconnection backoff:**
```
1. Запустить сервер
2. Подключиться
3. Закрыть сервер
4. Дождаться попытки reconnect (1с)
5. Закрыть всё
6. Проверить логи: 1 попытка через 1с, потом 2с (если ждать)
```

UI тесты в основном — manual.

### Критерий готовности этапа 3

- [ ] Workstation запускается
- [ ] Подключается к работающему Engine
- [ ] Показывает индикатор подключения
- [ ] Reconnect работает при потере связи
- [ ] Settings сохраняются и восстанавливаются
- [ ] Получает Snapshot при подключении (валидно десериализуется)
- [ ] Получает Events (видно в логах)
- [ ] Тесты 3.7.a-c проходят
- [ ] Можно открыть Workstation, увидеть индикатор Connected, закрыть Engine, увидеть Reconnecting, перезапустить Engine, увидеть Connected снова

---

## Этап 4 — Multi-window iced + tabs + первое окно (System Map) (≈3 недели)

**Цель:** базовая навигация работает. System Map показывает живые данные от Engine. Detach работает.

### 4.1 Multi-window setup

Iced multi-window через `iced::window` и `iced::Application`:

```rust
pub struct WorkstationApp {
    // ... existing fields
    main_window: window::Id,
    detached_windows: HashMap<window::Id, TabKind>,
    active_tab_in_main: TabKind,
    pending_attach_to_main: Vec<TabKind>,  // вкладки, доступные но не открытые
}

pub enum TabKind {
    SystemMap,
    LiveField,    // в этом этапе не реализован
    Patterns,
    DreamState,
    Conversation,
    Configuration,
    Files,
    Benchmarks,
}
```

### 4.2 Tabs UI в main window

```rust
fn main_window_view(state: &WorkstationApp) -> Element<Message> {
    column![
        header(state),                         // индикатор подключения
        tabs_bar(&state.active_tab_in_main),   // переключение между вкладками
        active_tab_content(state),             // System Map в этом этапе
    ]
    .into()
}

fn tabs_bar(active: &TabKind) -> Element<Message> {
    row![
        tab_button("Map", TabKind::SystemMap, active),
        tab_button("Field", TabKind::LiveField, active),
        // ... остальные tabs (placeholder в этом этапе)
    ]
    .into()
}
```

### 4.3 Detach implementation

```rust
fn handle_message(state: &mut WorkstationApp, msg: Message) -> Task<Message> {
    match msg {
        Message::DetachTab(tab) => {
            let new_window_id = window::Id::unique();
            state.detached_windows.insert(new_window_id, tab);
            // Активная вкладка main переключается на следующую доступную
            state.active_tab_in_main = next_available_tab(&state, tab);
            Task::done(Message::WindowOpened(new_window_id))
        }
        // ...
    }
}
```

### 4.4 System Map — реализация

Из Документа 3 часть A:

**Композиция:**

```rust
fn system_map_view(snapshot: &Option<SystemSnapshot>) -> Element<Message> {
    match snapshot {
        None => placeholder_loading(),
        Some(snap) => {
            column![
                map_canvas(snap),       // карта с мандалой
                bottom_panel(snap),     // нижняя панель показателей
            ]
            .into()
        }
    }
}
```

**Map canvas** — это самая сложная часть. Используем `iced::widget::canvas::Canvas`:

```rust
struct SystemMapCanvas<'a> {
    snapshot: &'a SystemSnapshot,
}

impl<'a> canvas::Program<Message> for SystemMapCanvas<'a> {
    fn draw(&self, ...) -> Vec<Geometry> {
        let mut frame = Frame::new(...);
        
        // 1. Нарисовать мандалу в центре
        draw_mandala(&mut frame, &self.snapshot);
        
        // 2. Нарисовать домены вокруг
        draw_domains_around(&mut frame, &self.snapshot.domains);
        
        // 3. Нарисовать линии потоков
        draw_flows(&mut frame, &self.snapshot);
        
        // 4. Нарисовать over-domain точки
        draw_over_domain(&mut frame, &self.snapshot.over_domain);
        
        vec![frame.into_geometry()]
    }
}
```

**Мандала — анимированная.** Каждый кадр:
- Считаем текущее значение анимации (фаза от 0 до 1)
- Рисуем концентрические круги с цветом, соответствующим engine_state
- Применяем легкую пульсацию через scale

**Домены вокруг** — статически расположены в layout. Каждый домен — circle с подписью.

**Линии потоков** — на основе recent_events:

```rust
// Если за последние ~ 500ms был EngineEvent::DomainActivity для домена X →
// линия от X к ближайшему соседу подсвечивается
```

### 4.5 Bottom panel

Простая панель с показателями:

```rust
fn bottom_panel(snap: &SystemSnapshot) -> Element<Message> {
    row![
        column![
            text(format!("State: {}", snap.engine_state)),
            sparkline_widget(&snap.fatigue.history),  // если есть
            text(format!("Hot path: {} ns", snap.last_hot_path_ns)),
        ],
        column![
            text(format!("Last dream: {} ago", format_duration(snap.last_dream_ago))),
            text(format!("Frames: {}", snap.frame_weaver_stats.total_frames)),
            text(format!("Promotions today: {}", snap.frame_weaver_stats.promotions_today)),
        ],
    ]
    .into()
}
```

### 4.6 Тесты

**Test 4.6.a — tab switching:**
```
1. Запустить Workstation с подключённым Engine
2. Активная вкладка — System Map
3. Кликнуть на табе "Patterns"
4. Активная вкладка изменилась
5. (Patterns показывает placeholder в этом этапе)
```

**Test 4.6.b — detach + reattach:**
```
1. Detach System Map
2. Появилось второе окно
3. В main активна другая вкладка
4. Закрыть detached (выбрать re-attach)
5. System Map снова в main
```

**Test 4.6.c — system map renders snapshot:**
```
1. Engine публикует Snapshot с конкретным состоянием
2. Workstation получает
3. Canvas пересчитывается (через mock)
4. Проверить что в нарисованной геометрии присутствуют ожидаемые элементы
```

**Test 4.6.d — animation continues during snapshot updates:**
```
1. Запустить, подключиться
2. Engine присылает события каждые 100ms
3. Мандала анимируется плавно (не дёргается)
```

### Критерий готовности этапа 4

- [ ] Multi-window работает (main + detached)
- [ ] Tabs переключаются
- [ ] System Map визуально отображает архитектуру
- [ ] Мандала анимируется
- [ ] Цвета мандалы/доменов меняются в зависимости от состояния
- [ ] Bottom panel показывает живые показатели
- [ ] Detach работает для System Map
- [ ] Тесты 4.6.a-d проходят
- [ ] FPS стабильно ~ 60
- [ ] Можно запустить, увидеть карту, подать текст через CLI engine, увидеть активность на карте

**Это первое значимое visible accomplishment проекта.** Здесь Sonnet и chrnv впервые видят живую систему через UI.

---

## Этап 5 — Configuration tab (≈1.5 недели)

**Цель:** настройки Engine и Workstation работают. Hot-reload работает для поддерживаемых параметров.

### 5.1 Configuration data model

```rust
pub struct ConfigurationState {
    sections: Vec<ConfigSection>,
    active_section: ConfigSectionId,
    pending_changes: HashMap<ConfigSectionId, ConfigData>,
}

pub struct ConfigSection {
    id: ConfigSectionId,
    name: String,
    parent: Option<ConfigSectionId>,
    schema: ConfigSchema,
    current_values: ConfigData,
}
```

### 5.2 Schema-driven UI

Каждая секция конфигурации описывается схемой, которую Engine отдаёт в response на `GetConfig`:

```rust
pub enum ConfigField {
    Integer { 
        key: String, 
        label: String, 
        min: i64, 
        max: i64, 
        default: i64 
    },
    Float { ... },
    String { ... },
    Bool { ... },
    Enum { ... },
    Group { ... },  // вложенная группа полей
}
```

Workstation рендерит UI на основе схемы. Если Engine добавит новый параметр — Workstation покажет его автоматически без изменений.

### 5.3 Apply / Discard / Reset

```rust
fn apply_changes(state: &mut WorkstationApp, section: ConfigSectionId) -> Task<Message> {
    let pending = state.configuration_state.pending_changes.get(&section);
    if let Some(data) = pending {
        // Validate locally first
        if !validate_locally(data, &state.configuration_state.sections[section].schema) {
            return Task::done(Message::ValidationError);
        }
        
        // Send to engine
        let cmd = EngineCommand::UpdateConfig {
            config_section: section.to_string(),
            payload: postcard::to_stdvec(data).unwrap(),
        };
        Task::done(Message::SendCommand(cmd))
    } else {
        Task::none()
    }
}
```

### 5.4 Workstation-локальные секции

Connection settings, UI preferences — это **в Workstation**, не отправляются в Engine. Применяются локально, сохраняются в config файл.

### Критерий готовности этапа 5

- [ ] Tab Configuration открывается
- [ ] Дерево секций отображается
- [ ] Engine секции загружаются по подключению
- [ ] Workstation секции работают и без Engine
- [ ] Изменения помечаются как pending
- [ ] Apply отправляет UpdateConfig
- [ ] Engine применяет hot-reload
- [ ] Discard / Reset работают
- [ ] Validation работает (невалидные значения помечаются)
- [ ] Все 1100+ существующих тестов остаются зелёными

---

## Этап 6 — Conversation tab (≈1 неделя)

**Цель:** простой чат работает. Подача текста, получение системных ответов, видимость в Patterns.

### 6.1 ConversationState

```rust
pub struct ConversationState {
    messages: Vec<ConversationMessage>,
    input_buffer: String,
    target_domain: u16,
    sending: bool,
}

pub enum ConversationMessage {
    User { text: String, target_domain: u16, timestamp: DateTime<Utc> },
    System { text: String, timestamp: DateTime<Utc>, kind: SystemMessageKind },
}

pub enum SystemMessageKind {
    Acknowledgment,    // "Текст обработан..."
    FrameCreated,      // "Создан Frame..."
    FrameReactivated,  // "Frame X реактивирован"
    Error,             // тёплый красный
}
```

### 6.2 UI — лента и форма ввода

Из Документа 3 часть C раздел 6.3.

Лента scrollable, новые сообщения снизу:

```rust
fn view(state: &ConversationState) -> Element<Message> {
    column![
        scrollable(
            column(
                state.messages.iter().rev().map(message_card)
            )
            .spacing(10)
        )
        .height(Length::Fill),
        input_panel(state),
    ]
    .into()
}
```

### 6.3 Submit logic

```rust
fn handle_submit(state: &mut WorkstationApp) -> Task<Message> {
    let text = state.conversation.input_buffer.clone();
    let target = state.conversation.target_domain;
    
    state.conversation.messages.push(ConversationMessage::User {
        text: text.clone(),
        target_domain: target,
        timestamp: Utc::now(),
    });
    
    state.conversation.sending = true;
    
    Task::done(Message::SendCommand(EngineCommand::SubmitText {
        text, target_domain: target,
    }))
}

fn handle_command_result(state: &mut WorkstationApp, result: CommandResultData) {
    state.conversation.sending = false;
    state.conversation.input_buffer.clear();  // только при success
    
    state.conversation.messages.push(ConversationMessage::System {
        text: "Текст обработан.".into(),
        timestamp: Utc::now(),
        kind: SystemMessageKind::Acknowledgment,
    });
}
```

### 6.4 Корреляция событий с Conversation

Когда Engine публикует FrameCrystallized **в результате** недавнего SubmitText — это нужно показать в Conversation:

```rust
fn handle_frame_crystallized(state: &mut WorkstationApp, event: FrameCrystallizedEvent) {
    // Если последний User message был < 5 секунд назад — связываем с ним
    if let Some(last_user) = state.conversation.messages.iter().rev().find(...) {
        if (Utc::now() - last_user.timestamp).num_seconds() < 5 {
            state.conversation.messages.push(ConversationMessage::System {
                text: format!("Создан Frame #{}", event.anchor_id),
                timestamp: Utc::now(),
                kind: SystemMessageKind::FrameCreated,
            });
        }
    }
}
```

### Критерий готовности этапа 6

- [ ] Tab Conversation открывается
- [ ] Подача текста работает
- [ ] Системные ответы появляются
- [ ] Цвет сообщений правильный (тёплый красный для ошибок)
- [ ] Ctrl+Enter отправляет, Enter — новая строка
- [ ] Direction правильный (новые снизу)
- [ ] Корреляция с FrameCrystallized работает
- [ ] Detach работает
- [ ] Существующие тесты остаются зелёными

---

## Этап 7 — Patterns + Dream State tabs (≈2 недели)

**Цель:** окна наблюдения за работой системы.

### 7.1 Patterns

Из Документа 3 часть B раздел 4.

**State:**
```rust
pub struct PatternsState {
    semantic_layer_history: [VecDeque<u8>; 8],  // sparkline data
    syntactic_layer_history: [VecDeque<u8>; 8],
    recent_frames: VecDeque<FrameEvent>,
}

pub enum FrameEvent {
    Crystallized { anchor_id: u32, layers: u8, participants: u8, timestamp: DateTime<Utc> },
    Reactivated { anchor_id: u32, new_temp: u8, timestamp: DateTime<Utc> },
    Vetoed { reason: String, timestamp: DateTime<Utc> },
    Promoted { source_id: u32, sutra_id: u32, rule: String, timestamp: DateTime<Utc> },
}
```

**View:**
```rust
fn view(state: &PatternsState) -> Element<Message> {
    column![
        active_layers_panel(state),    // 8 семантических + 8 синтаксических sparklines
        recent_frames_panel(state),    // лента событий FrameWeaver
    ]
    .into()
}
```

### 7.2 Dream State

Из Документа 3 часть B раздел 5.

**State:**
```rust
pub struct DreamStateWindow {
    fatigue_history: VecDeque<FatigueSnapshot>,
    recent_dreams: Vec<DreamReport>,
}
```

**View:**
```rust
fn view(state: &DreamStateWindow, snap: &Option<SystemSnapshot>) -> Element<Message> {
    column![
        current_state_panel(snap),        // WAKE/FALLING_ASLEEP/DREAMING/WAKING
        fatigue_panel(snap, state),       // breakdown по факторам
        recent_dreams_panel(state),       // лента DreamReports
    ]
    .into()
}
```

### 7.3 Force-sleep / Wake-up

```rust
fn force_sleep(state: &mut WorkstationApp) -> Task<Message> {
    // Open confirmation dialog
    state.dialog = Some(Dialog::ForceSleepConfirmation);
    Task::none()
}

fn wake_up(state: &mut WorkstationApp) -> Task<Message> {
    Task::done(Message::SendCommand(EngineCommand::ForceWake))
}
```

### Критерий готовности этапа 7

- [ ] Patterns отображает sparklines всех 16 слоёв
- [ ] Recent frames лента работает с 4 типами событий
- [ ] Dream State отображает текущее состояние
- [ ] Fatigue breakdown по факторам видим
- [ ] Recent dreams отображаются
- [ ] Force-sleep с подтверждением работает
- [ ] Wake-up работает
- [ ] Detach работает для обоих окон

---

## Этап 8 — Files + Benchmarks tabs (≈2 недели)

**Цель:** knowledge import и benchmarks работают.

### 8.1 Files

Из Документа 3 часть C раздел 7.

```rust
pub struct FilesState {
    available_adapters: Vec<AdapterInfo>,
    new_import: NewImportForm,
    active_imports: Vec<ImportProgress>,
    recent_imports: VecDeque<ImportResult>,
}
```

**File picker integration:**
```rust
async fn pick_file() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|h| h.path().to_path_buf())
}
```

(rfd — стандартный Rust crate для системных диалогов файлов)

### 8.2 Benchmarks

Из Документа 3 часть C раздел 9 + Документ 2 раздел 7.

**Запуск bench-instance:**
```rust
fn run_benchmark(bench_id: String, iterations: u32) -> Task<Message> {
    let port = find_free_port();
    let _child = Command::new("axiom-engine")
        .args(["--bench-mode", &bench_id, "--port", &port.to_string()])
        .args(["--iterations", &iterations.to_string()])
        .spawn()
        .unwrap();
    
    Task::done(Message::ConnectToBenchInstance(port))
}
```

**bench-history.md persistence:**
```rust
fn append_to_bench_history(result: &BenchResults) {
    let path = bench_history_path();
    let entry = format_bench_result_md(result);
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
        .write_all(entry.as_bytes())
        .unwrap();
}
```

### Критерий готовности этапа 8

- [ ] Files: список адаптеров получается от Engine
- [ ] Files: file picker работает
- [ ] Files: импорт запускается
- [ ] Files: progress отображается в реальном времени
- [ ] Files: cancel работает
- [ ] Files: completed imports отображаются
- [ ] Benchmarks: список бенчмарков работает
- [ ] Benchmarks: запуск bench-instance работает
- [ ] Benchmarks: progress отображается
- [ ] Benchmarks: results сохраняются в bench-history.md
- [ ] Benchmarks: export работает
- [ ] Detach работает для обоих окон

---

## Этап 9 — Welcome + общие компоненты (≈1.5 недели)

**Цель:** финализация. Welcome screen, диалоги, error handling, шорткаты, polish.

### 9.1 Welcome screen

Из Документа 3 часть A раздел 1.

Простое окно с fade-in анимацией. Открывается только при первом запуске (нет config файла).

### 9.2 Главное меню

Из Документа 3 часть D раздел 10.4.

```rust
fn menu_bar() -> Element<Message> {
    row![
        menu_item("File", file_menu()),
        menu_item("Engine", engine_menu()),
        menu_item("View", view_menu()),
        menu_item("Configuration", config_menu()),
        menu_item("Help", help_menu()),
    ]
    .into()
}
```

### 9.3 Диалоги подтверждения

Универсальный модальный компонент:

```rust
pub struct ConfirmationDialog {
    title: String,
    description: String,
    confirm_label: String,
    cancel_label: String,
    on_confirm: Message,
    on_cancel: Message,
}
```

### 9.4 Шорткаты

Из Документа 3 часть D раздел 14:

```rust
fn handle_keyboard(state: &mut WorkstationApp, key_event: KeyEvent) -> Task<Message> {
    match key_event {
        KeyEvent { code: KeyCode::Numeric(1), modifiers: Modifiers::CTRL, .. } => {
            Task::done(Message::TabSelected(TabKind::SystemMap))
        }
        // ...
    }
}
```

### 9.5 Error handling system

Унификация обработки ошибок. Алерты в углу окна, не модальные.

### Критерий готовности этапа 9

- [ ] Welcome screen появляется при первом запуске
- [ ] Subsequent runs не показывают Welcome
- [ ] Главное меню работает
- [ ] Диалоги подтверждения работают (правильные кнопки, Esc отменяет)
- [ ] Шорткаты работают
- [ ] Error handling: ошибки показываются inline, не модально
- [ ] Indicator подключения раскрывает диагностический popup при клике

---

## Этап 10 — Live Field (≈3 недели)

**Цель:** самое сложное окно. 3D-визуализация полей доменов.

### 10.1 Технологический выбор

Iced сам по себе не имеет 3D. Варианты:

**(а) iced::widget::canvas с собственным 3D рендером.** Простой проектор 3D в 2D через матрицы. Контроль полный, но реализация трудоёмкая.

**(б) Интеграция с wgpu через iced wgpu backend.** Использовать GPU-acceleration. Сложнее интегрировать, но производительнее.

В этом этапе — выбираем **(а)** для V1.0. Простой 3D через canvas достаточен для visualisations типа точек+линий. wgpu — V2.0 если потребуется большая нагрузка.

### 10.2 Camera

Простая orbit camera:

```rust
pub struct OrbitCamera {
    target: Vector3,
    distance: f32,
    azimuth: f32,
    elevation: f32,
}

impl OrbitCamera {
    pub fn project(&self, point: Vector3) -> Point2D { ... }
    pub fn handle_drag(&mut self, dx: f32, dy: f32) { ... }
    pub fn handle_zoom(&mut self, delta: f32) { ... }
}
```

### 10.3 Rendering

```rust
fn draw_domain_field(frame: &mut Frame, camera: &OrbitCamera, domain: &DomainSnapshot) {
    // 1. Координатная сетка
    draw_axes(frame, camera);
    
    // 2. Точки токенов
    for token in &domain.tokens {
        let projected = camera.project(token.position);
        let color = layer_color(token.dominant_layer);
        let size = mass_to_size(token.mass);
        let alpha = temperature_to_alpha(token.temperature);
        
        draw_token_point(frame, projected, color, size, alpha);
    }
    
    // 3. Связи (если включено)
    if display_options.show_connections {
        for conn in &domain.connections {
            // ... прорисовка линий
        }
    }
}
```

### 10.4 Performance optimizations

Из Документа 3 часть B раздел 3.9:

- Frustum culling
- Connection limit
- Adaptive frame rate
- Tier visualization

### Критерий готовности этапа 10

- [ ] Tab Live Field открывается
- [ ] Domain selector работает
- [ ] 3D-вид рендерится для выбранного домена
- [ ] Камера работает (drag, zoom, reset)
- [ ] Цвет точек по слоям правильный
- [ ] Размер по массе, прозрачность по температуре
- [ ] Display options работают
- [ ] Domain stats отображаются
- [ ] FPS стабильно ≥ 30 для доменов до 1000 токенов
- [ ] Detach работает

---

## Этап 11 — финальная валидация и release prep (≈1 неделя)

**Цель:** убедиться что всё работает целостно. Подготовка к релизу.

### 11.1 End-to-end smoke test

Полный сценарий:

```
1. Запустить engine отдельно
2. Запустить Workstation
3. Welcome → System Map
4. Подать текст через Conversation, увидеть Frame в Patterns
5. Подождать сна, увидеть DreamReport в Dream State
6. Импортировать небольшой файл через Files, увидеть прогресс
7. Запустить бенчмарк, увидеть результат
8. Изменить конфигурацию через Configuration, увидеть hot-reload
9. Detach System Map на отдельный экран, продолжить работу
10. Закрыть Workstation, переоткрыть — топология сохранилась
```

Этот сценарий — **полная проверка всех функций V1.0**.

### 11.2 Полный прогон тестов

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --workspace -- --check
```

### 11.3 Документация для пользователя

Простой README в репозитории:
- Как запустить engine
- Как запустить Workstation
- Базовое описание окон
- Где искать логи и config файлы

### 11.4 Errata

Создать `docs/specs/erratas/Workstation_V1_0_errata.md` со всеми обнаруженными расхождениями между спекой и реализацией. По правилу проекта.

### Критерий готовности этапа 11

- [ ] End-to-end smoke test проходит
- [ ] Все unit тесты зелёные
- [ ] Clippy без warnings
- [ ] Fmt проходит
- [ ] README написан
- [ ] Errata заполнена

---

## Резюме плана

| Этап | Что делаем                                            | Время    | Накопленное |
|------|-------------------------------------------------------|----------|-------------|
| 0    | Структура crate-ов, dependencies                      | 1-2 дня  | 2 дня       |
| 1    | axiom-protocol — типы сообщений                       | 1 неделя | 1.5 нед     |
| 2    | axiom-broadcasting + Engine integration               | 2 недели | 3.5 нед     |
| 3    | axiom-workstation базовая инфраструктура              | 1.5 нед  | 5 нед       |
| 4    | Multi-window, tabs, System Map                        | 3 недели | 8 нед       |
| 5    | Configuration tab                                     | 1.5 нед  | 9.5 нед     |
| 6    | Conversation tab                                      | 1 неделя | 10.5 нед    |
| 7    | Patterns + Dream State tabs                           | 2 недели | 12.5 нед    |
| 8    | Files + Benchmarks tabs                               | 2 недели | 14.5 нед    |
| 9    | Welcome + общие компоненты                            | 1.5 нед  | 16 нед      |
| 10   | Live Field (3D)                                       | 3 недели | 19 нед      |
| 11   | Финальная валидация и release prep                    | 1 неделя | 20 нед      |

**Итого: ~ 20 недель работы Sonnet, или 4-5 месяцев плотной работы.**

Это серьёзная оценка. Возможны отклонения в обе стороны: что-то окажется проще, что-то сложнее. План должен корректироваться по ходу.

---

## Принципы хорошей реализации

В дополнение к конкретным этапам — общие принципы для Sonnet:

### Спека-код согласованность

Это твой постоянный риск. После каждого этапа:
- Сравнить реализацию с соответствующими разделами Документов 1-3
- Зафиксировать расхождения в errata
- Не молча менять одно — обсуждать с chrnv

### Не торопиться

20 недель — это много, но это **необходимо** для проекта такого качества. Не пытаться сжать в 10 недель за счёт MVP-подхода (это исключено выбором 4.б из Документа 1).

### Performance бюджет

После каждого этапа, влияющего на Engine — проверять hot path. После каждого этапа, добавляющего UI — проверять FPS Workstation.

Бюджеты:
- Engine hot path: ≤ 280 ns/tick (с учётом broadcasting)
- Workstation UI: ≥ 30 FPS даже при активной системе

Если просели — расследовать сразу, не откладывать.

### Изоляция изменений

Каждый этап вносит изменения в чётко определённые crate-ы. Не делать "пока я тут — поправлю и это". Это вызывает регрессии.

### Отчёты

После каждого этапа — отчёт chrnv:
- Что сделано
- Что обнаружено (errata)
- Что не сделано из плана и почему
- Performance numbers
- Готовность к следующему этапу

---

## Что делать когда план встретит реальность

План — это план. Реальность скорректирует.

Возможные сценарии и их решения:

**Сценарий 1: iced API оказался другим, чем ожидалось.**
Адаптируем код, обновляем errata, продолжаем. Спецификация архитектуры от деталей iced API не зависит.

**Сценарий 2: Performance проблема с broadcasting.**
Возможно throttling нужно агрессивнее, или формат не postcard а bincode. Решаем по результатам бенчмарков. Контракт сообщений не меняется.

**Сценарий 3: Окно X оказалось сложнее чем планировалось.**
Расширяем сроки этого этапа. Не пытаемся сжать другие.

**Сценарий 4: Обнаружилась архитектурная ошибка в спеке.**
Останавливаемся. Обсуждаем с chrnv. Если требуется — выпускаем V1.1 спецификации, продолжаем по обновлённой.

**Сценарий 5: chrnv видит что-то нужно изменить в дизайне.**
Принимаем правку. Включаем в текущий или следующий этап. Не пытаемся "закончить как было запланировано", если это уже не отражает желаемое.

---

## После V1.0

Когда V1.0 готов — следующие шаги:

1. **Live observation period.** Несколько недель работы с системой через Workstation. Накопление наблюдений, понимания, ошибок.

2. **Workstation V1.1 patch.** Точечные исправления по результатам live observation. Не большая работа.

3. **Workstation V2.0 planning.** На основе errata + V1.1 — что добавить (drag-and-drop если не в V1.0, dark theme, dim темa, конструктор бенчмарков, история чата, etc.).

4. **Companion проектирование начинается.** Это другой проект. Откроем DEFERRED.md, увидим Vital Signs как первое окно Companion, начнём оттуда.

Workstation V1.0 — это **только начало**. Но без него — Companion не построить.

---

## Резюме Документа 4

11 этапов реализации, ~ 20 недель плотной работы Sonnet. Структура:

- **Этапы 0-3** — фундамент (crate-ы, протокол, broadcasting, базовая инфраструктура)
- **Этапы 4-10** — окна одно за другим, в правильном порядке (System Map, Configuration, Conversation, Patterns/Dream State, Files/Benchmarks, Welcome+общее, Live Field)
- **Этап 11** — финальная валидация

После каждого этапа — отчёт, проверка, errata. Не торопиться. Качество выше скорости.

---

## ИТОГ ЧЕТЫРЁХ ДОКУМЕНТОВ

Полный пакет проектирования AXIOM Workstation V1.0 завершён:

- **Документ 1: Vision & Principles** — что строим, почему, в какой эстетике, в каких отношениях с Companion
- **Документ 2: Architecture** — техническое устройство, контракт Engine ↔ Workstation, готовность к сетевому
- **Документ 3: Windows Design (части A-D)** — детальное описание 9 окон и общих компонентов
- **Документ 4: Implementation Phases** (этот) — порядок реализации в 11 этапах

Дополнительно:
- **DEFERRED.md** — список того, что отложено для V2.0 / Companion / неопределённого будущего
- **Глоссарий и язык** — последовательный во всех документах

После согласования Документа 4 — пакет готов для передачи Sonnet.

Спасибо за совместную работу над этим проектом.
