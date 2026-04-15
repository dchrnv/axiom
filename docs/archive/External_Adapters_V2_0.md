# External Adapters V2.0 — WebSocket + REST + egui Dashboard + Telegram + OpenSearch

**Версия:** 2.0 (ревизия после обратной связи исполнителя)  
**Дата:** 2026-04-15  
**Для:** Claude Sonnet (реализация)  
**Контекст:** 919 тестов, CLI Extended работает, Anchor Tokens загружаются, Memory Persistence реализована. RuntimeAdapter trait и EventBus существуют.

---

## Изменения V2.0 vs V1.2

- **Исправлено:** Engine доступ через snapshot + command channel, НЕ через Mutex напрямую
- **Исправлено:** handle_meta_command возвращает String, не печатает в stdout
- **Добавлено:** Convenience-методы на AxiomEngine (trace_count, tension_count и т.д.)
- **Исправлено:** OpenSearch flush — owned buffer + cloned client для Send + 'static
- **Исправлено:** Telegram — таблица pending command_id → chat_id
- **Убрано:** HTML/JS dashboard — заменён на egui (отдельный crate через WebSocket)
- **Уточнено:** UCL остаётся единственным способом изменить состояние. JSON — только за границей.
- **Уточнено:** `@timestamp` в OpenSearch генерируется адаптером, не ядром (COM не нарушен).

---

## 1. Главное архитектурное решение: Engine Access Pattern

**Ядро трогает ТОЛЬКО tick loop.** Все остальные (CLI, WebSocket, REST, Telegram) — через каналы.

```
                    ┌──────────────────────────┐
  CLI stdin ────→   │                          │
  WebSocket  ───→   │  command_tx (mpsc)       │ ──→ tick loop ──→ Engine (единственный writer)
  REST POST  ───→   │                          │         │
  Telegram   ───→   │                          │         │
                    └──────────────────────────┘         │
                                                          │ обновляет
                    ┌──────────────────────────┐         │
  CLI stdout ←───   │                          │ ←───────┘
  WebSocket  ←───   │  broadcast_tx            │
  REST GET   ←───   │  + snapshot (RwLock)     │
  Telegram   ←───   │                          │
                    └──────────────────────────┘
```

### 1.1 Каналы

```rust
/// Входящие команды от любого адаптера
let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(256);

/// Исходящие события для всех подписчиков
let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(1024);

/// Snapshot для read-only доступа (REST GET, dashboard)
let snapshot: Arc<RwLock<BroadcastSnapshot>> = Arc::new(RwLock::new(BroadcastSnapshot::default()));
```

### 1.2 AdapterCommand (единый тип входящей команды)

```rust
pub struct AdapterCommand {
    pub id: String,                  // Уникальный ID для корреляции ответа
    pub source: AdapterSource,       // CLI | WebSocket(conn_id) | REST | Telegram(chat_id)
    pub payload: AdapterPayload,
}

pub enum AdapterSource {
    Cli,
    WebSocket(u64),          // connection id
    Rest,
    Telegram(i64),           // chat_id
}

pub enum AdapterPayload {
    Inject { text: String },
    MetaCommand { cmd: String },
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
}
```

### 1.3 Tick loop обрабатывает команды

```rust
async fn tick_loop(
    engine: &mut AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    config: &AdaptersConfig,
) {
    let tick_interval = Duration::from_millis(1000 / config.tick_hz as u64);
    let mut interval = tokio::time::interval(tick_interval);
    let tick_cmd = build_tick_forward_command();

    loop {
        interval.tick().await;

        // 1. Обработать все входящие команды (non-blocking drain)
        while let Ok(cmd) = command_rx.try_recv() {
            match cmd.payload {
                AdapterPayload::Inject { text } => {
                    let ucl_cmd = perceptor.perceive(&text);
                    let result = engine.process_and_observe(&ucl_cmd);
                    let msg = ServerMessage::Result {
                        command_id: cmd.id,
                        // ... все поля из ProcessingResult
                    };
                    let _ = broadcast_tx.send(msg);
                }
                AdapterPayload::MetaCommand { cmd: meta } => {
                    let output = handle_meta_command(&meta, engine);
                    let msg = ServerMessage::CommandResult {
                        command_id: cmd.id,
                        cmd: meta,
                        output,
                    };
                    let _ = broadcast_tx.send(msg);
                }
                _ => {} // Subscribe/Unsubscribe — handled per-adapter
            }
        }

        // 2. Обработать pending impulses (Cognitive Depth)
        let impulses: Vec<Token> = engine.drain_pending_impulses();
        for token in impulses {
            let ucl_cmd = build_inject_from_token(100, token);
            let _ = engine.process_command(&ucl_cmd);
        }

        // 3. Tick ядра
        engine.process_command(&tick_cmd);

        // 4. Broadcast тиков (каждые N)
        let t = engine.tick_count();
        if t % config.websocket.tick_broadcast_interval as u64 == 0 {
            let _ = broadcast_tx.send(ServerMessage::Tick {
                tick_count: t,
                traces: engine.trace_count() as u32,
                tension: engine.tension_count() as u32,
                matched: engine.last_matched() as u32,
                impulses_pending: engine.impulse_count() as u32,
            });
        }

        // 5. Обновить snapshot (каждые M тиков)
        if t % config.websocket.state_broadcast_interval as u64 == 0 {
            let new_snapshot = engine.snapshot_for_broadcast();
            *snapshot.write().await = new_snapshot;
            let _ = broadcast_tx.send(ServerMessage::State {
                tick_count: t,
                snapshot: new_snapshot.clone(),
            });
        }
    }
}
```

---

## 2. Рефактор handle_meta_command

**Критический подготовительный шаг.** Все `println!` внутри метакоманд → возврат `String`.

```rust
// Было (CLI-only):
fn handle_meta_command(cmd: &str, engine: &mut AxiomEngine, config: &CliConfig) {
    match cmd {
        ":status" => {
            println!("  tick_count: {}", engine.tick_count());
            println!("  traces:    {}", engine.trace_count());
            // ... ещё 10 println!
        }
        // ... ещё 30 команд
    }
}

// Стало (adapter-agnostic):
fn handle_meta_command(cmd: &str, engine: &AxiomEngine) -> String {
    let mut out = String::new();
    match cmd.split_whitespace().next().unwrap_or("") {
        ":status" | "status" => {
            writeln!(out, "  tick_count: {}", engine.tick_count()).ok();
            writeln!(out, "  traces:    {}", engine.trace_count()).ok();
            // ...
        }
        // ...
    }
    out
}

// CLI adapter:
let output = handle_meta_command(cmd, &engine);
print!("{}", output);  // CLI печатает

// WebSocket/REST adapter:
let output = handle_meta_command(cmd, &engine);
// Отправляет как JSON
```

**Важно:** `handle_meta_command` принимает `&AxiomEngine` (immutable ref), НЕ `&mut`. Все read-only команды (:status, :traces, :domains) не требуют мутации. Мутирующие команды (:inject, :tick, :save) обрабатываются через AdapterCommand → tick loop.

Разделение:
- **Read-only** (:status, :traces, :domains, :tension, :perf, :arbiter, :frontier, :guardian, :config, :anchors, :match) → `handle_meta_command(&engine) -> String`
- **Mutating** (:inject, :tick, :save, :load, :reset, :export, :import) → `AdapterCommand → tick loop`

---

## 3. Convenience-методы на AxiomEngine

```rust
impl AxiomEngine {
    // --- Read-only accessors (для snapshot и метакоманд) ---

    pub fn trace_count(&self) -> usize {
        // Путь к traces через существующие структуры
        self.ashti.experience().traces().len()
    }

    pub fn tension_count(&self) -> usize {
        self.ashti.arbiter().experience.tension_traces.len()
    }

    pub fn impulse_count(&self) -> usize {
        self.pending_impulses.len()
    }

    pub fn last_matched(&self) -> usize {
        self.ashti.arbiter().last_routing
            .as_ref()
            .map(|r| r.traces_matched as usize)
            .unwrap_or(0)
    }

    /// Лёгкий snapshot для broadcast — числа и сводки, НЕ полный clone
    pub fn snapshot_for_broadcast(&self) -> BroadcastSnapshot {
        BroadcastSnapshot {
            tick_count: self.tick_count,
            com_next_id: self.com_next_id,
            trace_count: self.trace_count(),
            tension_count: self.tension_count(),
            impulse_count: self.impulse_count(),
            domain_summaries: self.domain_summaries(),
        }
    }

    /// Snapshot одного домена для dashboard визуализации
    pub fn domain_snapshot(&self, domain_id: u16) -> Option<DomainDetailSnapshot> {
        let state = self.ashti.domain_state(domain_id)?;
        Some(DomainDetailSnapshot {
            domain_id,
            tokens: state.tokens.iter().map(|t| TokenSnapshot::from(t)).collect(),
            connections: state.connections.iter().map(|c| ConnectionSnapshot::from(c)).collect(),
        })
    }

    fn domain_summaries(&self) -> Vec<DomainSummary> {
        (0..=10).map(|offset| {
            let id = 100 + offset as u16;
            let state = self.ashti.domain_state(id);
            DomainSummary {
                domain_id: id,
                name: domain_name(id).to_string(),
                token_count: state.map(|s| s.tokens.len()).unwrap_or(0),
                connection_count: state.map(|s| s.connections.len()).unwrap_or(0),
            }
        }).collect()
    }

    pub fn drain_pending_impulses(&mut self) -> Vec<Token> {
        std::mem::take(&mut self.pending_impulses)
    }
}
```

---

## 4. Зависимости

```toml
# crates/axiom-agent/Cargo.toml

[dependencies]
axiom-runtime = { path = "../axiom-runtime" }
axiom-core = { path = "../axiom-core" }
axiom-persist = { path = "../axiom-persist" }
tokio = { version = "1", features = ["rt", "io-util", "macros", "sync", "time"] }

# Adapters (Phase 1-2)
axum = { version = "0.8", features = ["ws"] }
tower-http = { version = "0.6", features = ["cors"] }
serde_json = "1"

# Telegram (Phase 4, optional)
[dependencies.teloxide]
version = "0.13"
features = ["macros"]
optional = true

# OpenSearch (Phase 5, optional)
[dependencies.reqwest]
version = "0.12"
features = ["json"]
optional = true

[features]
default = []
telegram = ["teloxide"]
opensearch = ["reqwest"]
```

```toml
# tools/axiom-dashboard/Cargo.toml
[package]
name = "axiom-dashboard"
version = "0.1.0"

[dependencies]
eframe = { version = "0.29", default-features = false, features = ["glow"] }
egui_plot = "0.29"
tungstenite = "0.24"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## 5. Конфигурация

```yaml
# В axiom-cli.yaml
adapters:
  cli:
    enabled: true

  websocket:
    enabled: true
    host: "0.0.0.0"
    port: 8080
    tick_broadcast_interval: 10
    state_broadcast_interval: 100
    max_connections: 10

  rest:
    enabled: true    # На том же порту что WebSocket

  telegram:
    enabled: false
    bot_token: "${TELEGRAM_BOT_TOKEN}"
    allowed_users: []

  opensearch:
    enabled: false
    url: "http://localhost:9200"
    index_prefix: "axiom-"
    index_traces: true
    index_events: true
    index_skills: true
    batch_size: 100
    flush_interval: 5000
```

---

## 6. Порядок реализации

### Phase 0: Подготовка (обязательно перед остальными фазами)

1. **Рефактор handle_meta_command** → возвращает `String`, не печатает
2. **Convenience-методы** на AxiomEngine: trace_count, tension_count, impulse_count, snapshot_for_broadcast, domain_snapshot
3. **BroadcastSnapshot, DomainSummary, TokenSnapshot, ConnectionSnapshot** — структуры для сериализации
4. **AdapterCommand, AdapterSource, AdapterPayload** — единый тип входящей команды
5. Тесты: handle_meta_command(":status", &engine) возвращает непустую строку

### Phase 1: WebSocket

1. protocol.rs — ClientMessage, ServerMessage (serde)
2. websocket.rs — axum WebSocket handler, подписки, broadcast
3. Tick loop рефактор: command_rx + broadcast_tx + snapshot update
4. `--server` флаг запускает axum на заданном порту
5. Тесты: подключение, inject, subscribe, broadcast

### Phase 2: REST

1. rest.rs — axum HTTP routes
2. GET endpoints читают `Arc<RwLock<BroadcastSnapshot>>` — НЕ лочат Engine
3. POST endpoints → command_tx.send() — НЕ лочат Engine
4. CORS через tower-http
5. Тесты: curl /api/status, POST /api/inject

### Phase 3: egui Dashboard (отдельный crate)

1. tools/axiom-dashboard/ — отдельный crate
2. WebSocket клиент (tungstenite, отдельный поток)
3. Space View — 2D проекция с якорями
4. Status, Traces, Input, Result panels
5. НЕ зависит от axiom-core — только JSON

### Phase 4: Telegram (feature-gated)

1. telegram.rs — teloxide
2. pending таблица: command_id → chat_id
3. /start, /status, текст → inject
4. Feature flag: `--features telegram`
5. Тесты: mock bot

### Phase 5: OpenSearch (feature-gated)

1. opensearch.rs — reqwest Bulk API
2. Буфер + flush: owned buffer, cloned client → tokio::spawn
3. `@timestamp` генерируется адаптером (wall-clock), не ядром
4. Index templates при первом подключении
5. docker-compose.opensearch.yaml
6. Feature flag: `--features opensearch`
7. Тесты: mock HTTP server

---

## 7. Протокол (protocol.rs)

### 7.1 ClientMessage

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "inject")]
    Inject { text: String },

    #[serde(rename = "command")]
    Command { cmd: String },

    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },

    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },

    #[serde(rename = "domain_snapshot")]
    DomainSnapshot { domain_id: u16 },
}
```

### 7.2 ServerMessage

```rust
#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "result")]
    Result {
        command_id: String,
        path: String,
        domain_id: u16,
        domain_name: String,
        coherence: f32,
        reflex_hit: bool,
        traces_matched: u32,
        position: [i16; 3],
        shell: [u8; 8],
        event_id: u64,
    },

    #[serde(rename = "tick")]
    Tick {
        tick_count: u64,
        traces: u32,
        tension: u32,
        matched: u32,
        impulses_pending: u32,
    },

    #[serde(rename = "state")]
    State {
        tick_count: u64,
        snapshot: BroadcastSnapshot,
    },

    #[serde(rename = "command_result")]
    CommandResult {
        command_id: String,
        cmd: String,
        output: String,
    },

    #[serde(rename = "error")]
    Error {
        command_id: Option<String>,
        message: String,
    },

    #[serde(rename = "domain_detail")]
    DomainDetail {
        domain_id: u16,
        tokens: Vec<TokenSnapshot>,
        connections: Vec<ConnectionSnapshot>,
    },
}
```

### 7.3 Snapshot типы

```rust
#[derive(Serialize, Clone, Default)]
pub struct BroadcastSnapshot {
    pub tick_count: u64,
    pub com_next_id: u64,
    pub trace_count: usize,
    pub tension_count: usize,
    pub impulse_count: usize,
    pub domain_summaries: Vec<DomainSummary>,
}

#[derive(Serialize, Clone)]
pub struct DomainSummary {
    pub domain_id: u16,
    pub name: String,
    pub token_count: usize,
    pub connection_count: usize,
}

#[derive(Serialize, Clone)]
pub struct TokenSnapshot {
    pub sutra_id: u32,
    pub position: [i16; 3],
    pub shell: [u8; 8],
    pub mass: u8,
    pub temperature: u8,
    pub valence: i8,
    pub origin: u16,
    pub is_anchor: bool,
}

#[derive(Serialize, Clone)]
pub struct ConnectionSnapshot {
    pub source_idx: u32,
    pub target_idx: u32,
    pub weight: f32,
}
```

---

## 8. Режимы запуска

```bash
# CLI only (по умолчанию)
cargo run --bin axiom-cli

# CLI + WebSocket + REST
cargo run --bin axiom-cli -- --server

# CLI + WebSocket + REST на порту 3000
cargo run --bin axiom-cli -- --server --port 3000

# Headless (WebSocket + REST, без CLI stdin)
cargo run --bin axiom-cli -- --server --no-cli

# + Telegram
cargo run --bin axiom-cli --features telegram -- --server --telegram

# + OpenSearch
cargo run --bin axiom-cli --features opensearch -- --server

# Dashboard (отдельный терминал)
cargo run -p axiom-dashboard
cargo run -p axiom-dashboard -- --url ws://192.168.1.100:8080/ws
```

При `--server`:
```
AXIOM — Cognitive Architecture
───────────────────────────────
tick_hz: 100 Hz  |  domains: 11  |  :help for commands
  mode: restored from axiom-data (tick=154200, traces=47, tension=0)
  WebSocket: ws://0.0.0.0:8080/ws
  REST API:  http://0.0.0.0:8080/api/
  anchors:   6 axes + 15 layer + 10 domain

axiom>
```

---

## 9. Инварианты

1. **Ядро изолировано.** axiom-runtime не импортирует axum, serde_json, teloxide, reqwest.
2. **UCL — единственный способ мутировать.** JSON → адаптер → UclCommand(64B) → Engine. JSON не проникает в ядро.
3. **COM не нарушен.** `@timestamp` генерируется адаптерами (wall-clock за границей). Ядро знает только event_id.
4. **Один Engine, один writer.** Tick loop — единственный кто мутирует Engine. Адаптеры → command_tx → tick loop.
5. **Snapshot для чтения.** REST GET / dashboard → `Arc<RwLock<BroadcastSnapshot>>`, обновляется tick loop.
6. **Graceful shutdown.** `:quit` → остановка всех адаптеров → автосохранение → выход.

---

## 10. История изменений

- **V2.0**: Ревизия. Engine access через snapshot + command channel. handle_meta_command → String. Convenience-методы. OpenSearch flush fix. Telegram pending table. egui = отдельный crate через WebSocket. HTML/JS убран.
- **V1.2**: egui вместо browser, OpenSearch добавлен.
- **V1.0**: Первая версия.
