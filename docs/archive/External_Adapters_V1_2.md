# External Adapters V1.0 — WebSocket + REST + Dashboard + Telegram

**Версия:** 1.0  
**Дата:** 2026-04-14  
**Назначение:** Открыть AXIOM миру через сетевые адаптеры  
**Для:** Claude Sonnet (реализация)  
**Контекст:** 919 тестов, CLI Channel работает, Memory Persistence реализована, Anchor Tokens загружаются. RuntimeAdapter trait и EventBus уже существуют. Ядро не трогаем — всё в axiom-agent.

---

## 1. Архитектурное решение

**Один сервер (axum), все протоколы, один порт.**

```
Browser (dashboard)  ←→  WebSocket /ws     ←→  axiom-agent  ←→  Engine
Browser (API)        ←→  REST /api/*       ←→  axiom-agent  ←→  Engine
Telegram             ←→  TelegramAdapter   ←→  axiom-agent  ←→  Engine
CLI (stdin/stdout)   ←→  CliChannel        ←→  axiom-agent  ←→  Engine
```

Все адаптеры подключены к одному AxiomEngine. EventBus — мост между Engine и адаптерами. Адаптеры подписываются на события, не опрашивают Engine.

**Ядро не знает об адаптерах.** Ядро не импортирует axum, tokio-tungstenite, teloxide, serde_json. Граница = UclCommand(64B) → UclResult(32B).

---

## 2. Зависимости

В `crates/axiom-agent/Cargo.toml`:

```toml
[dependencies]
# Существующие
axiom-runtime = { path = "../axiom-runtime" }
axiom-core = { path = "../axiom-core" }
axiom-persist = { path = "../axiom-persist" }
tokio = { version = "1", features = ["rt", "io-util", "macros", "sync", "time"] }

# Новые — Phase 1-3
axum = { version = "0.8", features = ["ws"] }
tower-http = { version = "0.6", features = ["cors", "fs"] }
serde_json = "1"

# Telegram — Phase 4, за feature flag
[dependencies.teloxide]
version = "0.13"
features = ["macros"]
optional = true

[features]
default = []
telegram = ["teloxide"]
```

**Почему axum:** Легче actix-web, tokio-native (уже в проекте), tower экосистема, WebSocket + REST + статика в одном. Хорошо документирован.

---

## 3. Структура файлов

```
crates/axiom-agent/
├── src/
│   ├── adapters/
│   │   ├── mod.rs              # pub mod websocket, rest, telegram
│   │   ├── websocket.rs        # WebSocket adapter
│   │   ├── rest.rs             # REST endpoints
│   │   ├── telegram.rs         # Telegram bot (feature-gated)
│   │   └── protocol.rs         # Общий протокол сообщений (JSON types)
│   ├── channels/
│   │   └── cli.rs              # Существующий CLI
│   ├── dashboard/
│   │   └── mod.rs              # Serve static files
│   └── ...
├── assets/
│   └── dashboard/
│       ├── index.html          # Dashboard UI
│       ├── app.js              # WebSocket client + Canvas rendering
│       └── style.css           # Стили
└── bin/
    └── axiom-cli.rs            # Обновить: --mode cli|server|both
```

---

## 4. Конфигурация

В `axiom-cli.yaml` (или отдельный `config/adapters.yaml`):

```yaml
adapters:
  cli:
    enabled: true

  websocket:
    enabled: true
    host: "0.0.0.0"
    port: 8080
    # Интервалы broadcast (в тиках Engine)
    tick_broadcast_interval: 10     # Отправлять tick-сводку каждые 10 тиков
    state_broadcast_interval: 100   # Полный snapshot каждые 100 тиков (для dashboard)
    max_connections: 10             # Лимит одновременных подключений

  rest:
    enabled: true                   # На том же порту что WebSocket (axum routes)

  telegram:
    enabled: false
    bot_token: "${TELEGRAM_BOT_TOKEN}"   # Из переменной окружения
    allowed_users: []                     # Пустой = все могут писать

  dashboard:
    enabled: true                   # Статические файлы dashboard
    path: "assets/dashboard"        # Относительно рабочей директории
```

---

## 5. Протокол (protocol.rs)

Общие типы для WebSocket и REST. Сериализация через serde_json.

### 5.1 Входящие сообщения (клиент → AXIOM)

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    /// Инжектировать текст в ядро
    #[serde(rename = "inject")]
    Inject { text: String },

    /// Выполнить CLI команду
    #[serde(rename = "command")]
    Command { cmd: String },

    /// Подписаться на каналы событий
    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },

    /// Отписаться от каналов
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },
}
```

Примеры JSON:
```json
{"type": "inject", "text": "привет"}
{"type": "command", "cmd": "status"}
{"type": "subscribe", "channels": ["ticks", "traces", "tension", "state"]}
{"type": "unsubscribe", "channels": ["ticks"]}
```

### 5.2 Исходящие сообщения (AXIOM → клиент)

```rust
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    /// Результат обработки inject
    #[serde(rename = "result")]
    Result {
        path: String,           // "reflex" | "slow-path" | "multi-pass(N)"
        domain_id: u16,
        domain_name: String,
        coherence: f32,
        reflex_hit: bool,
        traces_matched: u32,
        position: [i16; 3],
        shell: [u8; 8],
        event_id: u64,
        // Полный вывод для detail=max
        input_hash: Option<String>,
        input_shell: Option<[u8; 8]>,
        confidence: Option<f32>,
        weight: Option<f32>,
        passes: Option<u8>,
        tension_created: Option<bool>,
    },

    /// Периодическая сводка по тикам
    #[serde(rename = "tick")]
    Tick {
        tick_count: u64,
        traces: u32,
        tension: u32,
        matched: u32,
        impulses_pending: u32,
    },

    /// Полный snapshot состояния (для dashboard визуализации)
    #[serde(rename = "state")]
    State {
        tick_count: u64,
        domains: Vec<DomainSnapshot>,
    },

    /// Ответ на команду
    #[serde(rename = "command_result")]
    CommandResult {
        cmd: String,
        output: String,
    },

    /// Ошибка
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Serialize)]
pub struct DomainSnapshot {
    pub domain_id: u16,
    pub name: String,
    pub tokens: Vec<TokenSnapshot>,
    pub connections: Vec<ConnectionSnapshot>,
    pub anchor_count: u32,
}

#[derive(Serialize)]
pub struct TokenSnapshot {
    pub sutra_id: u32,
    pub position: [i16; 3],
    pub shell: [u8; 8],
    pub mass: u8,
    pub temperature: u8,
    pub valence: i8,
    pub state: u8,
    pub origin: u16,
    pub is_anchor: bool,
}

#[derive(Serialize)]
pub struct ConnectionSnapshot {
    pub source_idx: u32,
    pub target_idx: u32,
    pub weight: f32,
    pub link_type: u8,
}
```

### 5.3 Каналы подписки

| Канал | Что отправляется | Частота |
|---|---|---|
| `ticks` | ServerMessage::Tick | Каждые tick_broadcast_interval тиков |
| `traces` | Tick с обновлённым traces count | При изменении |
| `tension` | Tick с обновлённым tension | При изменении |
| `state` | ServerMessage::State (полный snapshot) | Каждые state_broadcast_interval тиков |
| `results` | ServerMessage::Result | При каждом inject (от любого клиента) |
| `events` | COM events (последние N) | При генерации |

---

## 6. Фаза 1: WebSocket сервер

### 6.1 websocket.rs

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    extract::State,
    routing::get,
    Router,
};
use tokio::sync::{broadcast, mpsc};

/// Состояние, разделяемое между всеми WebSocket подключениями
pub struct WsState {
    /// Broadcast канал для исходящих сообщений (Engine → все клиенты)
    pub broadcast_tx: broadcast::Sender<ServerMessage>,
    /// Канал для входящих сообщений (клиент → Engine)
    pub command_tx: mpsc::Sender<ClientMessage>,
}

/// Axum handler для WebSocket upgrade
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Обработка одного WebSocket подключения
async fn handle_socket(socket: WebSocket, state: Arc<WsState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    // Подписки этого клиента (по умолчанию — ничего)
    let subscriptions: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    // Задача: получать broadcast и отправлять клиенту (если подписан)
    let subs_clone = subscriptions.clone();
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            let channel = match &msg {
                ServerMessage::Tick { .. } => "ticks",
                ServerMessage::State { .. } => "state",
                ServerMessage::Result { .. } => "results",
                _ => "other",
            };
            if subs_clone.lock().await.contains(channel) || channel == "other" {
                let json = serde_json::to_string(&msg).unwrap();
                if sender.send(Message::Text(json)).await.is_err() {
                    break; // Клиент отключился
                }
            }
        }
    });

    // Задача: получать сообщения от клиента
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::Subscribe { channels }) => {
                        subscriptions.lock().await.extend(channels);
                    }
                    Ok(ClientMessage::Unsubscribe { channels }) => {
                        let mut subs = subscriptions.lock().await;
                        for ch in channels { subs.remove(&ch); }
                    }
                    Ok(client_msg) => {
                        let _ = state.command_tx.send(client_msg).await;
                    }
                    Err(e) => {
                        // Отправить ошибку клиенту (через broadcast нельзя — нужен direct send)
                    }
                }
            }
        }
    });

    // Ждать завершения любой задачи (клиент отключился)
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}
```

### 6.2 Интеграция с tick loop

В основном tick loop (рядом с CLI try_recv):

```rust
// Проверить WebSocket команды
while let Ok(msg) = ws_command_rx.try_recv() {
    match msg {
        ClientMessage::Inject { text } => {
            let cmd = perceptor.perceive(&text);
            let result = engine.process_and_observe(&cmd);
            let server_msg = format_result_for_ws(&result);
            let _ = broadcast_tx.send(ServerMessage::Result { ... });
        }
        ClientMessage::Command { cmd } => {
            let output = handle_meta_command(&cmd, engine, config);
            let _ = broadcast_tx.send(ServerMessage::CommandResult { cmd, output });
        }
        _ => {} // Subscribe/Unsubscribe handled in ws handler
    }
}

// Broadcast тиков (каждые N тиков)
if tick_count % ws_config.tick_broadcast_interval as u64 == 0 {
    let _ = broadcast_tx.send(ServerMessage::Tick {
        tick_count,
        traces: engine.trace_count(),
        tension: engine.tension_count(),
        matched: last_matched,
        impulses_pending: engine.impulse_count(),
    });
}

// Broadcast state (каждые M тиков) — для dashboard
if tick_count % ws_config.state_broadcast_interval as u64 == 0 {
    let snapshot = engine.snapshot_for_dashboard();
    let _ = broadcast_tx.send(ServerMessage::State { ... });
}
```

### 6.3 Запуск сервера

В `main()` axiom-cli:

```rust
// Если WebSocket включён — запустить axum сервер
if config.adapters.websocket.enabled {
    let (broadcast_tx, _) = broadcast::channel(1024);
    let (command_tx, command_rx) = mpsc::channel(256);

    let ws_state = Arc::new(WsState { broadcast_tx: broadcast_tx.clone(), command_tx });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(ws_state);

    let addr = format!("{}:{}", config.adapters.websocket.host, config.adapters.websocket.port);
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        println!("  WebSocket server: ws://{}/ws", addr);
        axum::serve(listener, app).await.unwrap();
    });

    // Передать broadcast_tx и command_rx в tick loop
}
```

### 6.4 Тесты

- [ ] WebSocket подключение к `/ws` → handshake успешен
- [ ] Отправка `{"type": "inject", "text": "привет"}` → получение `{"type": "result", ...}`
- [ ] Subscribe на `"ticks"` → получение tick-сообщений
- [ ] Subscribe на `"state"` → получение полного snapshot
- [ ] Два клиента одновременно → оба получают broadcast
- [ ] Отключение клиента → без паник, без утечек
- [ ] `{"type": "command", "cmd": "status"}` → получение `{"type": "command_result", ...}`
- [ ] Невалидный JSON → `{"type": "error", "message": "..."}`
- [ ] max_connections → отказ при превышении

---

## 7. Фаза 2: REST endpoints

### 7.1 rest.rs

```rust
use axum::{routing::{get, post}, Json, Router, extract::State, extract::Query};

pub fn rest_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Информация
        .route("/api/status", get(get_status))
        .route("/api/domains", get(get_domains))
        .route("/api/domain/:id", get(get_domain))
        .route("/api/traces", get(get_traces))
        .route("/api/tension", get(get_tension))
        .route("/api/skills", get(get_skills))
        .route("/api/anchors", get(get_anchors))
        .route("/api/match", get(get_match))          // ?text=лёд
        .route("/api/perf", get(get_perf))
        .route("/api/depth", get(get_depth))
        .route("/api/frontier", get(get_frontier))
        .route("/api/guardian", get(get_guardian))
        .route("/api/events", get(get_events))         // ?n=10

        // Действия
        .route("/api/inject", post(post_inject))        // {"text": "привет"}
        .route("/api/command", post(post_command))       // {"cmd": "save"}
        .route("/api/tick", post(post_tick))             // {"n": 100}

        .with_state(state)
}

// Примеры handlers
async fn get_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().await;
    Json(serde_json::json!({
        "tick_count": engine.tick_count(),
        "traces": engine.trace_count(),
        "tension": engine.tension_count(),
        "domains": 11,
        // ...
    }))
}

#[derive(Deserialize)]
pub struct InjectRequest {
    text: String,
}

async fn post_inject(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InjectRequest>,
) -> Json<ServerMessage> {
    let mut engine = state.engine.lock().await;
    let cmd = state.perceptor.perceive(&req.text);
    let result = engine.process_and_observe(&cmd);
    Json(format_result(&result))
}

#[derive(Deserialize)]
pub struct MatchQuery {
    text: String,
}

async fn get_match(
    State(state): State<Arc<AppState>>,
    Query(q): Query<MatchQuery>,
) -> Json<serde_json::Value> {
    let matches = state.anchors.match_text(&q.text);
    let position = state.anchors.compute_position(&matches);
    let shell = state.anchors.compute_shell(&matches);
    Json(serde_json::json!({
        "text": q.text,
        "matches": matches.iter().map(|m| {
            serde_json::json!({
                "word": m.anchor.word,
                "score": m.score,
                "match_type": format!("{:?}", m.match_type),
            })
        }).collect::<Vec<_>>(),
        "position": position,
        "shell": shell,
    }))
}
```

### 7.2 Добавить к axum серверу

```rust
let app = Router::new()
    .route("/ws", get(ws_handler))
    .merge(rest_routes(app_state.clone()))
    .layer(CorsLayer::permissive())  // Для dashboard из другого origin
    .with_state(ws_state);
```

### 7.3 Тесты

- [ ] `GET /api/status` → 200, JSON с tick_count
- [ ] `POST /api/inject {"text": "привет"}` → 200, JSON с result
- [ ] `GET /api/traces` → 200, JSON со списком трейсов
- [ ] `GET /api/match?text=лёд` → 200, JSON с якорными совпадениями
- [ ] `GET /api/anchors` → 200, JSON со всеми якорями
- [ ] `POST /api/tick {"n": 100}` → 200, тики прокручены
- [ ] CORS headers присутствуют

---

## 8. Фаза 3: egui Dashboard (отдельный бинарник через WebSocket)

### 8.1 Архитектура: Вариант B

Dashboard — отдельный процесс. Подключается к Engine через WebSocket. Два бинарника:

```
Процесс 1:  axiom-cli --server          (ядро + WebSocket + REST + CLI)
Процесс 2:  axiom-dashboard              (egui + WebSocket клиент)

axiom-dashboard ←──WebSocket──→ axiom-cli --server
```

**Преимущества:**
- Dashboard можно закрыть/открыть без остановки ядра
- Можно подключиться с другой машины в сети
- Ядро не зависит от GUI зависимостей (eframe, wgpu/glow)
- Можно запустить несколько dashboard одновременно (разные домены)

**Следствие:** Фаза 1 (WebSocket) обязательна для dashboard. Протокол `ServerMessage::State` — основной канал данных.

### 8.2 Отдельный crate

Dashboard живёт в **отдельном crate** `axiom-dashboard`, не в axiom-agent. Это изолирует GUI зависимости от ядра.

```
axiom/
├── crates/
│   ├── axiom-core/
│   ├── axiom-runtime/
│   ├── axiom-agent/          # Ядро + CLI + WebSocket + REST
│   └── ...
├── tools/
│   └── axiom-dashboard/      # Отдельный crate — GUI
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app.rs          # DashboardApp, eframe::App impl
│           ├── ws_client.rs    # WebSocket клиент (tungstenite)
│           ├── space_view.rs   # 2D визуализация пространства
│           ├── traces_panel.rs
│           ├── status_panel.rs
│           ├── input_panel.rs
│           ├── result_panel.rs
│           ├── tension_view.rs
│           └── theme.rs
```

```toml
# tools/axiom-dashboard/Cargo.toml
[package]
name = "axiom-dashboard"
version = "0.1.0"

[dependencies]
eframe = { version = "0.29", default-features = false, features = ["glow"] }
egui_plot = "0.29"
tungstenite = "0.24"       # WebSocket клиент (sync, для простоты)
serde = { version = "1", features = ["derive"] }
serde_json = "1"
# НЕ зависит от axiom-core, axiom-runtime — только от протокола (JSON)
```

**Ключевое:** axiom-dashboard НЕ зависит от axiom-core/axiom-runtime. Единственный интерфейс — JSON через WebSocket. Это позволяет dashboard работать с любым AXIOM сервером, в том числе удалённым.

### 8.3 Протокол (re-use из Фазы 1)

Dashboard использует тот же протокол что любой WebSocket клиент:

```
Подключение:  ws://localhost:8080/ws

Отправить:    {"type": "subscribe", "channels": ["ticks", "state", "results"]}
Получать:     {"type": "state", "tick_count": ..., "domains": [...]}
              {"type": "tick", "tick_count": ..., "traces": ..., "tension": ...}
              {"type": "result", "path": "reflex", ...}

Inject:       {"type": "inject", "text": "привет"}
Команды:      {"type": "command", "cmd": "status"}
```

### 8.4 Компоненты dashboard

**1. Space View (центральная область)**

2D проекция пространства выбранного домена:
- Точки = токены
  - Цвет: temperature (красный=горячий, синий=холодный) через HSL
  - Размер: mass (тяжёлый = большой)
  - Прозрачность: weight (слабый = полупрозрачный)
- Звёзды/ромбы = якоря (mass=255, state=Locked, золотой цвет)
  - Подпись якоря рядом (word из State snapshot)
- Линии = связи (толщина по weight, цвет по link_type)
- Оси X/Y с подписями (Аполлон↔Дионис, Эрос↔Танатос)
- Z = яркость или отдельный слайдер проекции
- При inject — анимация: вспышка → точка появляется → связи протягиваются
- Zoom + pan (egui input handling)

**2. Status Bar (верх)**

```
tick: 154,200 | tps: 99.8 Hz | traces: 47 | tension: 2 | impulses: 0
ws: connected ● | server: localhost:8080
```

**3. Domain Selector (слева)**

Список 11 доменов, кликабельный. Выбранный рисуется в Space View.
```
▸ SUTRA(100)       0 tokens, 6 anchors
▸ EXECUTION(101)   0 tokens, 5 anchors
  ...
▸ EXPERIENCE(109)  0 tokens, 47 traces
▸ MAYA(110)        0 tokens
```

**4. Traces Panel (справа-сверху)**

Top-20 трейсов по weight:
```
0.72 ████████████░░░ (1823, 5950, 10514) age=4281
0.68 ███████████░░░░ (1823, 5950, 10514) age=3892
```

**5. Input + Result (снизу)**

Текстовое поле ввода. Результат последнего inject:
```
> привет
⚡ reflex | EXECUTION(101) | coherence: 1.00 | traces: 46 matched
```

**6. Tension View (справа-снизу)**

egui_plot: tension over time.

**7. Connection indicator**

Зелёный/красный индикатор WebSocket подключения. Авто-переподключение при обрыве.

### 8.5 WebSocket клиент в egui

```rust
// ws_client.rs
use tungstenite::{connect, Message};
use std::sync::mpsc;

pub struct WsClient {
    /// Получать серверные сообщения
    pub rx: mpsc::Receiver<ServerMessage>,
    /// Отправлять клиентские сообщения
    pub tx: mpsc::Sender<ClientMessage>,
    pub connected: Arc<AtomicBool>,
}

impl WsClient {
    pub fn connect(url: &str) -> Self {
        let (server_tx, server_rx) = mpsc::channel();
        let (client_tx, client_rx) = mpsc::channel();
        let connected = Arc::new(AtomicBool::new(false));
        let conn = connected.clone();

        // WebSocket в отдельном потоке (не блокирует GUI)
        std::thread::spawn(move || {
            loop {
                match connect(url) {
                    Ok((mut socket, _)) => {
                        conn.store(true, Ordering::Relaxed);

                        // Подписаться на всё
                        let sub = serde_json::to_string(&ClientMessage::Subscribe {
                            channels: vec!["ticks", "state", "results"]
                                .into_iter().map(String::from).collect()
                        }).unwrap();
                        let _ = socket.send(Message::Text(sub));

                        loop {
                            // Отправить клиентские сообщения
                            while let Ok(msg) = client_rx.try_recv() {
                                let json = serde_json::to_string(&msg).unwrap();
                                if socket.send(Message::Text(json)).is_err() { break; }
                            }

                            // Получить серверные сообщения
                            match socket.read() {
                                Ok(Message::Text(text)) => {
                                    if let Ok(msg) = serde_json::from_str(&text) {
                                        let _ = server_tx.send(msg);
                                    }
                                }
                                Err(_) => break, // Disconnected
                                _ => {}
                            }
                        }

                        conn.store(false, Ordering::Relaxed);
                    }
                    Err(_) => {
                        // Retry через 2 секунды
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        });

        Self { rx: server_rx, tx: client_tx, connected }
    }
}
```

### 8.6 Запуск

```bash
# Терминал 1: запустить ядро с WebSocket
cargo run --bin axiom-cli -- --server

# Терминал 2: запустить dashboard
cargo run -p axiom-dashboard

# Или с указанием адреса (если удалённый сервер)
cargo run -p axiom-dashboard -- --url ws://192.168.1.100:8080/ws
```

При запуске dashboard:
```
AXIOM Dashboard
Connecting to ws://localhost:8080/ws...
Connected ●
```

### 8.7 Dashboard read-only + inject

Dashboard может: визуализировать, inject текст, выполнять read-only команды (status, traces, domains).
Dashboard НЕ может: :save, :reset, :load, :quit — только через CLI.

---

## 9. Фаза 4: Telegram bot

(содержание без изменений)

---

## 10. Фаза 5: OpenSearch adapter (опциональный)

### 10.1 Назначение

OpenSearch — для полнотекстового поиска по памяти AXIOM и аналитики через OpenSearch Dashboards. Не обязательный компонент — система работает полностью без него.

**Что даёт:**
- Поиск по трейсам: "покажи все трейсы связанные с температурой" — через Shell-профиль и якорные теги
- Аналитика: распределение coherence за последний час, heatmap tension, timeline рефлексов
- OpenSearch Dashboards (бывший Kibana): готовые визуализации без написания UI
- История: все COM-события индексированы, можно искать по event_type, subtype, domain

**Что не даёт:**
- Не заменяет egui Dashboard (разные задачи: egui = real-time, OpenSearch = аналитика)
- Не заменяет persistence (OpenSearch = search index, axiom-persist = source of truth)

### 10.2 Архитектура

AXIOM → OpenSearch — однонаправленный export. AXIOM не читает из OpenSearch (не зависит от него).

```
Engine → EventBus → OpenSearchAdapter → HTTP bulk API → OpenSearch
                                                              ↓
                                                    OpenSearch Dashboards
```

Если OpenSearch недоступен — адаптер буферизирует события и ретрайит. Потеря событий допустима (это index, не source of truth).

### 10.3 Зависимости

```toml
# В axiom-agent/Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["json"], optional = true }

[features]
opensearch = ["reqwest"]
```

Без внешних OpenSearch SDK — просто HTTP REST API через reqwest. OpenSearch Bulk API достаточно прост.

### 10.4 Конфигурация

```yaml
adapters:
  opensearch:
    enabled: false
    url: "http://localhost:9200"
    index_prefix: "axiom-"
    # Что индексировать
    index_traces: true        # Experience traces → axiom-traces
    index_events: true        # COM events → axiom-events
    index_skills: true        # Crystallized skills → axiom-skills
    index_tension: true       # Tension traces → axiom-tension
    # Батчинг
    batch_size: 100           # Отправлять пакетами по 100 документов
    flush_interval: 5000      # Flush каждые 5000 тиков (или при заполнении batch)
    # Retention
    event_retention_days: 7   # Удалять старые события через N дней (ILM policy)
```

### 10.5 Индексы

**axiom-traces-YYYY.MM.DD:**
```json
{
  "trace_id": 42,
  "weight": 0.72,
  "position": [1823, 5950, 10514],
  "shell": [0, 0, 0, 50, 200, 180, 30, 75],
  "created_at_tick": 150000,
  "last_event_id": 847291,
  "matched_anchors": ["привет", "социальность"],
  "domain_id": 109,
  "@timestamp": "2026-04-14T12:00:00Z"
}
```

**axiom-events-YYYY.MM.DD:**
```json
{
  "event_id": 847305,
  "event_type": "TokenCreate",
  "event_subtype": "none",
  "domain_id": 100,
  "source_domain": 100,
  "target_id": 42,
  "tick_count": 154200,
  "@timestamp": "2026-04-14T12:00:00Z"
}
```

**axiom-skills:**
```json
{
  "skill_id": 1,
  "weight": 0.85,
  "token_count": 5,
  "connection_count": 8,
  "created_at_tick": 120000,
  "description": "pattern: greeting response"
}
```

### 10.6 Реализация

```rust
#[cfg(feature = "opensearch")]
pub struct OpenSearchAdapter {
    client: reqwest::Client,
    config: OpenSearchConfig,
    buffer: Vec<serde_json::Value>,
}

#[cfg(feature = "opensearch")]
impl OpenSearchAdapter {
    pub fn new(config: OpenSearchConfig) -> Self { ... }

    /// Добавить документ в буфер
    pub fn index_trace(&mut self, trace: &ExperienceTrace) { ... }
    pub fn index_event(&mut self, event: &Event) { ... }
    pub fn index_skill(&mut self, skill: &Skill) { ... }

    /// Отправить буфер в OpenSearch (Bulk API)
    pub async fn flush(&mut self) -> Result<(), reqwest::Error> {
        if self.buffer.is_empty() { return Ok(()); }

        let mut body = String::new();
        for doc in &self.buffer {
            // Bulk API format: action + document, newline-delimited
            body.push_str(&format!(
                "{{\"index\":{{\"_index\":\"{}-{}\"}}}}\n{}\n",
                self.config.index_prefix,
                index_name_for(doc),
                serde_json::to_string(doc).unwrap()
            ));
        }

        self.client
            .post(&format!("{}/_bulk", self.config.url))
            .header("Content-Type", "application/x-ndjson")
            .body(body)
            .send()
            .await?;

        self.buffer.clear();
        Ok(())
    }
}
```

### 10.7 Интеграция с tick loop

```rust
// В tick loop, рядом с WebSocket broadcast
if let Some(os) = &mut opensearch_adapter {
    // При обработке inject — индексировать trace
    if new_trace_created {
        os.index_trace(&trace);
    }

    // Периодический flush
    if tick_count % os.config.flush_interval as u64 == 0 {
        tokio::spawn(os.flush()); // Async, не блокирует tick
    }
}
```

### 10.8 Index templates (создаются при старте)

При первом подключении к OpenSearch — создать index templates:

```json
PUT _index_template/axiom-traces
{
  "index_patterns": ["axiom-traces-*"],
  "template": {
    "mappings": {
      "properties": {
        "weight": { "type": "float" },
        "position": { "type": "integer" },
        "shell": { "type": "integer" },
        "matched_anchors": { "type": "keyword" },
        "@timestamp": { "type": "date" }
      }
    }
  }
}
```

### 10.9 Тесты

- [ ] Feature gate: без `--features opensearch` код не компилируется
- [ ] Buffer заполняется при index_trace/index_event
- [ ] Flush отправляет bulk request (mock HTTP server)
- [ ] При недоступности OpenSearch — buffer сохраняется, retry при следующем flush
- [ ] Index template создаётся при первом подключении

### 10.10 Требования к окружению

OpenSearch не входит в AXIOM. Пользователь устанавливает отдельно:

```bash
# Docker (рекомендуемый)
docker run -d --name opensearch -p 9200:9200 -e "discovery.type=single-node" opensearchproject/opensearch:2.14.0
docker run -d --name dashboards -p 5601:5601 --link opensearch opensearchproject/opensearch-dashboards:2.14.0
```

Или через docker-compose. Axiom предоставит `docker-compose.opensearch.yaml` в корне проекта.

---

### 9.1 telegram.rs (feature-gated)

```rust
#[cfg(feature = "telegram")]
pub mod telegram {
    use teloxide::prelude::*;

    pub async fn start_telegram_bot(
        token: String,
        command_tx: mpsc::Sender<ClientMessage>,
        mut result_rx: broadcast::Receiver<ServerMessage>,
    ) {
        let bot = Bot::new(token);

        teloxide::repl(bot, move |bot: Bot, msg: Message| {
            let command_tx = command_tx.clone();
            async move {
                let text = msg.text().unwrap_or("");

                if text.starts_with('/') {
                    // Команды бота
                    match text {
                        "/start" => {
                            bot.send_message(msg.chat.id, 
                                "AXIOM Cognitive Architecture\nОтправь любой текст для обработки\n/status — состояние\n/traces — top трейсы"
                            ).await?;
                        }
                        "/status" => {
                            command_tx.send(ClientMessage::Command { cmd: "status".into() }).await.ok();
                            // Ответ придёт через result_rx → отправить в чат
                        }
                        "/traces" => {
                            command_tx.send(ClientMessage::Command { cmd: "traces".into() }).await.ok();
                        }
                        _ => {
                            bot.send_message(msg.chat.id, "Неизвестная команда").await?;
                        }
                    }
                } else {
                    // Обычный текст → inject
                    command_tx.send(ClientMessage::Inject { text: text.into() }).await.ok();
                    // Результат придёт через broadcast → отправить в чат
                }
                Ok(())
            }
        }).await;
    }
}
```

### 9.2 Telegram formatting

Результат обработки форматируется для Telegram (markdown):

```
⚡ *Reflex*
Domain: EXECUTION (101)
Coherence: 1.00
Traces: 46 matched
Position: (1823, 5950, 10514)
```

### 9.3 Тесты

- [ ] Bot запускается с валидным токеном
- [ ] /start → приветствие
- [ ] Текст → inject → результат в чат
- [ ] /status → состояние Engine
- [ ] Feature gate: без `--features telegram` код не компилируется

---

## 10. Режимы запуска

Обновить `bin/axiom-cli.rs`:

```
cargo run --bin axiom-cli                           # CLI only (default)
cargo run --bin axiom-cli -- --server               # CLI + WebSocket + REST + Dashboard
cargo run --bin axiom-cli -- --server --no-cli      # WebSocket only (headless)
cargo run --bin axiom-cli -- --server --port 3000   # Кастомный порт
cargo run --bin axiom-cli --features telegram -- --telegram  # + Telegram bot
```

При `--server`:
```
AXIOM — Cognitive Architecture
───────────────────────────────
tick_hz: 100 Hz  |  domains: 11  |  :help for commands
  mode: restored from axiom-data (tick=154200, traces=47, tension=0)
  WebSocket: ws://0.0.0.0:8080/ws
  REST API:  http://0.0.0.0:8080/api/
  Dashboard: http://0.0.0.0:8080/dashboard/

axiom>
```

---

## 11. Порядок реализации

| Фаза | Что | Зависимости | Feature flag | Результат |
|------|-----|-------------|-------------|-----------|
| 1 | WebSocket сервер + протокол | axum, serde_json | — (всегда) | `ws://localhost:8080/ws` |
| 2 | REST endpoints | Фаза 1 | — (всегда) | `curl /api/status` |
| 3 | egui Dashboard | Фазы 1-2 | `--features dashboard` | Нативное окно с визуализацией |
| 4 | Telegram bot | Фазы 1-2, teloxide | `--features telegram` | Бот отвечает в Telegram |
| 5 | OpenSearch adapter | reqwest | `--features opensearch` | Аналитика в OpenSearch Dashboards |

**Каждая фаза — отдельный коммит. Тесты после каждой фазы.**
**Фазы 3-5 за feature flags — не увеличивают базовую сборку.**

---

## 12. Инварианты

1. **Ядро изолировано.** axiom-runtime не импортирует axum, serde_json, teloxide.
2. **Один Engine.** Все адаптеры работают с одним AxiomEngine через shared state.
3. **EventBus.** Адаптеры подписываются на события, не опрашивают Engine.
4. **Thread safety.** Engine за `Arc<Mutex<>>` или `Arc<RwLock<>>`. Одна запись за раз.
5. **Graceful shutdown.** `:quit` в CLI → остановка WebSocket сервера → остановка Telegram → автосохранение.
6. **Dashboard read-only.** Inject да, :save/:reset/:load — только через CLI.

---

## 13. История изменений

- **V1.0**: Первая версия. WebSocket + REST + Dashboard + Telegram. Axum как единый сервер. Подписки. Dashboard с Canvas визуализацией.
