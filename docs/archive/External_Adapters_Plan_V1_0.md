# External Adapters — План реализации V1.2

**Версия:** 1.2  
**Дата:** 2026-04-18  
**Спецификация:** [External_Adapters_V3_0.md](External_Adapters_V3_0.md) (актуальная версия V3.1)  
**Статус:** Phase 0A/0B/0C завершены

---

## Обзор

| Фаза | Название | Crates | Тесты | Статус |
|------|----------|--------|-------|--------|
| 0A | Convenience-методы + Snapshot-типы | axiom-runtime | +18 | ✅ завершена |
| 0B | Рефактор handle_meta_command | axiom-agent | +7 | ✅ завершена |
| 0C | AdapterCommand + tick_loop | axiom-agent | +6 | ✅ завершена |
| 1 | WebSocket | axiom-agent | +12 | не начата |
| 2 | REST | axiom-agent | +8 | не начата |
| 3 | egui Dashboard | axiom-dashboard (новый) | +5 | не начата |
| 4 | Telegram | axiom-agent (feature) | +6 | не начата |
| 5 | OpenSearch | axiom-agent (feature) | +8 | не начата |

**Инвариант:** после каждой фазы `cargo test --workspace` — зелёный, `cargo clippy` — без warnings.

---

## Phase 0A — Convenience-методы + Snapshot-типы ✅

**Результат:** 932 → 950 тестов (`--features adapters`). Коммит: в работе.

### Что сделано

**`crates/axiom-runtime/Cargo.toml`**
```toml
serde = { version = "1", features = ["derive"], optional = true }

[features]
default = []
adapters = ["serde"]
```

**`crates/axiom-runtime/src/broadcast.rs`** (новый)
- `BroadcastSnapshot` — tick_count, com_next_id, trace_count, tension_count, domain_summaries
- `DomainSummary` — domain_id, name, token_count, connection_count
- `DomainDetailSnapshot` — domain_id, tokens, connections
- `TokenSnapshot` — sutra_id, position, shell, mass, temperature, valence, origin, is_anchor
- `ConnectionSnapshot` — source_id, target_id, weight
- `From<&Token>` и `From<&Connection>` impl

**`crates/axiom-runtime/src/engine.rs`** (добавлены методы)
- `trace_count(&self) -> usize` — `ashti.experience().trace_count()`
- `tension_count(&self) -> usize` — `ashti.experience().tension_count()`
- `last_matched(&self) -> u32` — `ashti.experience().last_traces_matched.get()`
- `snapshot_for_broadcast(&self)` — под `#[cfg(feature = "adapters")]`
- `domain_detail_snapshot(&self, domain_id)` — под `#[cfg(feature = "adapters")]`
- `domain_summaries(&self)` — приватный, под `#[cfg(feature = "adapters")]`

**`crates/axiom-runtime/tests/broadcast_tests.rs`** — 18 тестов

### Технический долг из Phase 0A → DEFERRED

1. **`domain_name` дублируется**: функция существует в `axiom-agent/src/effectors/message.rs`
   и теперь продублирована в `engine.rs` под `#[cfg(feature = "adapters")]`.
   Правильное место — вынести в `axiom-runtime` как `pub fn domain_name(id: u16) -> &'static str`
   без feature gate (это просто const-функция, не зависит от serde).

2. **`shell` в TokenSnapshot — приближение**: реальный семантический профиль Shell (`[u8; 8]`)
   не хранится в `Token` — он вычисляется в `axiom-shell`. Сейчас используем
   `[0,0,0, |valence|, temperature, mass, 0, 0]` как диагностическое приближение.
   Для egui Space View этого достаточно. Для точного профиля нужно либо хранить shell
   в Token (меняет размер структуры — нарушение инварианта 9.1), либо вычислять отдельно.

---

## Phase 0B — Рефактор handle_meta_command ✅

**Цель:** разделить 700-строчный `&mut self` метод `CliChannel::handle_meta_command`
на две standalone-функции. Поведение CLI не меняется — только внутренняя организация.
Это обязательное условие для Phase 0C (tick loop не может держать `&mut CliChannel`).

### Новый файл: `crates/axiom-agent/src/meta_commands.rs`

```rust
use std::collections::HashSet;
use std::fmt::Write;
use axiom_runtime::AxiomEngine;
use axiom_config::AnchorSet;
use axiom_persist::AutoSaver;
use crate::perceptors::text::TextPerceptor;
use crate::config::CliConfig;

/// Результат мутирующей мета-команды.
pub struct MetaMutateResult {
    pub output: String,
    pub action: MetaAction,
}

/// Побочный эффект мутирующей команды — нужен tick loop чтобы реагировать.
pub enum MetaAction {
    None,
    Quit,                   // :quit
    EngineReplaced,         // :load — engine заменён целиком
    AutosaveChanged(u32),   // :autosave on N
    WatchToggle(String),    // :watch traces|tension|tps
    WatchClear,             // :watch off
}

/// Команды только для чтения — не мутируют Engine.
///
/// Принимает &AxiomEngine (не &mut).
/// Возвращает строку готовую к печати или отправке через любой транспорт.
pub fn handle_meta_read(
    cmd: &str,
    engine: &AxiomEngine,
    anchor_set: Option<&AnchorSet>,
    perceptor: &TextPerceptor,
    config: &CliConfig,
    watch_fields: &HashSet<String>,
    // + read-only поля CliChannel нужные для вывода:
    event_log: &std::collections::VecDeque<axiom_core::Event>,
    perf: &crate::perf::PerfTracker,
    multipass_count: u64,
    last_multipass_n: u8,
) -> String

/// Команды с мутацией Engine — вызываются только из tick loop.
pub fn handle_meta_mutate(
    cmd: &str,
    engine: &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    config: &CliConfig,
) -> MetaMutateResult
```

### Разделение команд

**Read-only** → `handle_meta_read`:
`:status`, `:domains`, `:tokens`, `:traces`, `:tension`, `:depth`, `:arbiter`,
`:frontier`, `:guardian`, `:perf`, `:events`, `:config`, `:trace`, `:connections`,
`:dream`, `:multipass`, `:reflector`, `:impulses`, `:schema`, `:anchors`, `:match`,
`:help`, `:snapshot`

**Mutating** → `handle_meta_mutate`:
`:save`, `:load`, `:autosave`, `:tick`, `:export`, `:import`, `:reset`, `:quit`

**Особый случай `:watch`**: `handle_meta_read` получает `&watch_fields` для вывода текущего
состояния. Изменение набора — через `MetaAction::WatchToggle` / `WatchClear`, которые
`CliChannel::handle_meta_command` применяет к `self.watch_fields`.

### Изменения в `crates/axiom-agent/src/channels/cli.rs`

`CliChannel::handle_meta_command` превращается в тонкую обёртку:

```rust
fn handle_meta_command(&mut self, line: &str) -> bool {
    let cmd = line.splitn(2, ' ').next().unwrap_or("");

    let is_mutating = matches!(cmd,
        ":save" | ":load" | ":autosave" | ":tick" |
        ":export" | ":import" | ":reset" | ":quit"
    );

    if is_mutating {
        let result = handle_meta_mutate(line, &mut self.engine, &mut self.auto_saver, &self.config);
        print!("{}", result.output);
        match result.action {
            MetaAction::Quit => return false,
            MetaAction::EngineReplaced => {
                self.engine.tick_schedule = self.config.tick_schedule;
                self.perceptor = make_perceptor(&self.anchor_set);
                self.last_traces = 0;
                self.last_tension = 0;
                self.multipass_count = 0;
                self.auto_saver.reset_save_tick(self.engine.tick_count);
            }
            MetaAction::AutosaveChanged(n) => {
                self.engine.tick_schedule.persist_check_interval = n;
            }
            _ => {}
        }
    } else {
        let output = handle_meta_read(
            line, &self.engine, self.anchor_set.as_deref(), &self.perceptor,
            &self.config, &self.watch_fields, &self.event_log, &self.perf,
            self.multipass_count, self.last_multipass_n,
        );
        print!("{}", output);

        // :watch изменяет self.watch_fields — это не может сделать handle_meta_read
        if cmd == ":watch" {
            let arg = line.splitn(3, ' ').nth(1).unwrap_or("");
            match arg {
                "off" => self.watch_fields.clear(),
                field if !field.is_empty() => { self.watch_fields.insert(field.to_string()); }
                _ => {}
            }
        }
    }
    true
}
```

### Стратегия переноса (пошагово)

1. Создать `meta_commands.rs` с пустыми заглушками `handle_meta_read` / `handle_meta_mutate`
2. Перенести read-only ветки одну за одной: заменить `println!` на `writeln!(out, ...)`,
   убрать `self.` — заменить на параметры функции
3. Перенести mutating ветки: заменить `self.engine` на `engine: &mut AxiomEngine`
4. Обновить `CliChannel::handle_meta_command` → тонкая обёртка
5. Запустить тесты после каждого переноса — не в конце

### Тесты

```rust
test_handle_meta_read_status_nonempty            // :status → содержит "tick_count"
test_handle_meta_read_domains_lists_11           // :domains → 11 строк
test_handle_meta_read_unknown_cmd_hint           // неизвестная → содержит ":help"
test_handle_meta_mutate_quit_returns_quit_action // :quit → MetaAction::Quit
test_handle_meta_mutate_tick_advances_engine     // :tick 5 → tick_count += 5
test_handle_meta_mutate_save_creates_file        // :save /tmp/... → файл создан
test_handle_meta_mutate_load_replaces_engine     // :load → MetaAction::EngineReplaced
```

### Критерий готовности

`cargo test -p axiom-agent` — все 97 существующих тестов зелёные + 7 новых.  
Ручная проверка: `:status`, `:save`, `:load`, `:quit` — поведение CLI не изменилось.

---

## Phase 0C — AdapterCommand + tick_loop ✅

**Цель:** выделить tick loop из `CliChannel::run()` в standalone async-функцию.
`CliChannel` становится тонкой обёрткой: инициализация + stdin reader.
После этой фазы можно добавлять адаптеры не трогая CliChannel.

### Новый файл: `crates/axiom-agent/src/adapter_command.rs`

```rust
/// Команда от любого адаптера в tick loop.
pub struct AdapterCommand {
    pub id:      String,           // UUID для корреляции ответа
    pub source:  AdapterSource,
    pub payload: AdapterPayload,
}

pub enum AdapterSource {
    Cli,
    WebSocket(u64),  // connection_id
    Rest,
    Telegram(i64),   // chat_id
}

pub enum AdapterPayload {
    Inject         { text: String },
    MetaRead       { cmd: String },
    MetaMutate     { cmd: String },
    Subscribe      { channels: Vec<String> },
    Unsubscribe    { channels: Vec<String> },
    DomainSnapshot { domain_id: u16 },
}

impl AdapterCommand {
    /// Команда завершения — для graceful shutdown из SIGTERM.
    pub fn shutdown() -> Self {
        Self {
            id: "shutdown".to_string(),
            source: AdapterSource::Cli,
            payload: AdapterPayload::MetaMutate { cmd: ":quit".to_string() },
        }
    }
}

/// Результат обработки одной AdapterCommand в tick loop.
///
/// Message(ServerMessage) — готово к отправке в broadcast_tx.
/// process_adapter_command сам строит ServerMessage из ProcessingResult —
/// tick loop об этом деталях не знает.
pub enum CommandResponse {
    Message(ServerMessage),
    Quit,   // :quit → автосохранение → выход из tick loop
    None,   // Subscribe/Unsubscribe — обработано на уровне адаптера
}
```

### Новый файл: `crates/axiom-agent/src/tick_loop.rs`

```rust
/// Главный цикл — единственный writer AxiomEngine.
///
/// Принимает engine по значению (владеет им).
/// Все адаптеры взаимодействуют через command_rx и broadcast_tx/snapshot.
pub async fn tick_loop(
    mut engine:      AxiomEngine,
    mut command_rx:  mpsc::Receiver<AdapterCommand>,
    broadcast_tx:    broadcast::Sender<ServerMessage>,
    snapshot:        Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver:  AutoSaver,
    anchor_set:      Option<Arc<AnchorSet>>,
    config:          AdaptersConfig,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let tick_ms = 1000u64 / config.tick_hz.max(1) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(tick_ms));
    let mut perceptor = make_perceptor(&anchor_set);

    loop {
        interval.tick().await;

        // 1. Drain входящих команд (non-blocking)
        while let Ok(cmd) = command_rx.try_recv() {
            match process_adapter_command(cmd.payload, cmd.id, &mut engine,
                                          &mut auto_saver, &mut perceptor,
                                          &anchor_set, &config) {
                CommandResponse::Message(msg) => { let _ = broadcast_tx.send(msg); }
                CommandResponse::Quit => {
                    if auto_saver.config.enabled {
                        let _ = auto_saver.force_save(&engine, Path::new(&config.data_dir));
                    }
                    return;
                }
                CommandResponse::None => {}
            }
        }

        // 2. Tick ядра (handle_tick_forward внутри обрабатывает impulses через TickSchedule)
        engine.process_command(&tick_cmd);
        let t = engine.tick_count;

        // 3. Broadcast тиков
        if t % config.websocket.tick_broadcast_interval as u64 == 0 {
            let _ = broadcast_tx.send(ServerMessage::Tick {
                tick_count:   t,
                traces:       engine.trace_count() as u32,
                tension:      engine.tension_count() as u32,
                last_matched: engine.last_matched(),
            });
        }

        // 4. Обновить snapshot (clone ДО write — нельзя move до clone)
        if t % config.websocket.state_broadcast_interval as u64 == 0 {
            let snap = engine.snapshot_for_broadcast();
            let for_broadcast = snap.clone();
            *snapshot.write().await = snap;
            let _ = broadcast_tx.send(ServerMessage::State { tick_count: t, snapshot: for_broadcast });
        }

        // 5. Автосохранение
        if let Some(n) = auto_saver.check_tick(t) {
            let _ = auto_saver.try_save(&engine, Path::new(&config.data_dir), n);
        }
    }
}

fn process_adapter_command(
    payload:    AdapterPayload,
    id:         String,
    engine:     &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    perceptor:  &mut TextPerceptor,
    anchor_set: &Option<Arc<AnchorSet>>,
    config:     &AdaptersConfig,
) -> CommandResponse {
    match payload {
        AdapterPayload::Inject { text } => {
            let ucl = perceptor.perceive(&text);
            let r = engine.process_and_observe(&ucl);
            CommandResponse::Message(ServerMessage::Result {
                command_id:     id,
                path:           format!("{:?}", r.path),
                domain_id:      r.dominant_domain_id,
                domain_name:    domain_name(r.dominant_domain_id).to_string(),
                coherence:      r.coherence_score.unwrap_or(0.0),
                reflex_hit:     r.reflex_hit,
                traces_matched: r.traces_matched,
                position:       r.output_position,
                shell:          r.output_shell,
                event_id:       r.event_id,
            })
        }
        AdapterPayload::MetaRead { cmd } => {
            let output = handle_meta_read(&cmd, engine, anchor_set.as_deref(),
                                          perceptor, &config.cli,
                                          &HashSet::new(), // watch_fields не нужны вне CLI
                                          &VecDeque::new(), &PerfTracker::new(0), 0, 0);
            CommandResponse::Message(ServerMessage::CommandResult { command_id: id, output })
        }
        AdapterPayload::MetaMutate { cmd } => {
            let result = handle_meta_mutate(&cmd, engine, auto_saver, &config.cli);
            match result.action {
                MetaAction::Quit => CommandResponse::Quit,
                MetaAction::EngineReplaced => {
                    *perceptor = make_perceptor(anchor_set);
                    CommandResponse::Message(ServerMessage::CommandResult {
                        command_id: id, output: result.output,
                    })
                }
                _ => CommandResponse::Message(ServerMessage::CommandResult {
                    command_id: id, output: result.output,
                }),
            }
        }
        AdapterPayload::DomainSnapshot { domain_id } => {
            match engine.domain_detail_snapshot(domain_id) {
                Some(snap) => CommandResponse::Message(ServerMessage::DomainDetail(snap)),
                None => CommandResponse::Message(ServerMessage::Error {
                    command_id: Some(id),
                    message: format!("domain {} not found", domain_id),
                }),
            }
        }
        AdapterPayload::Subscribe { .. } | AdapterPayload::Unsubscribe { .. } => {
            CommandResponse::None // обрабатывается per-connection в WebSocket handler
        }
    }
}
```

### Замечание о watch_fields в tick loop

`handle_meta_read` принимает `watch_fields: &HashSet<String>` для вывода текущего
состояния watch. В tick loop (не CLI) watch не нужен — передаём пустой `HashSet::new()`.
Это честнее чем прятать проблему: watch — CLI-фича, не протокольная.
Для WebSocket аналог — это `Subscribe { channels }`.

### Изменения в `crates/axiom-agent/src/channels/cli.rs`

`CliChannel::run()` рефакторится:

```rust
pub async fn run(&mut self) {
    let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(256);
    let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(1024);
    let snapshot = Arc::new(RwLock::new(BroadcastSnapshot::default()));

    // Инициализируем AdaptersConfig из CliConfig
    // (на этом этапе только CLI enabled, остальные disabled)
    let adapters_config = AdaptersConfig::from_cli_config(&self.config);

    // Stdin reader — отправляет AdapterCommand в command_tx
    let tx = command_tx.clone();
    tokio::spawn(async move {
        // ... читаем stdin, парсим строки в AdapterCommand::MetaRead/MetaMutate/Inject
    });

    // AutoSaver переезжает в tick_loop (владеет engine)
    let auto_saver = std::mem::take(&mut self.auto_saver);
    let engine = std::mem::replace(&mut self.engine, AxiomEngine::new()); // placeholder

    tick_loop(engine, command_rx, broadcast_tx, snapshot, auto_saver,
              self.anchor_set.clone(), adapters_config).await;
}
```

`CliChannel` после Phase 0C больше не владеет `engine` во время `run()` — он в tick loop.

### Конфигурация

`AdaptersConfig` добавляется в `axiom-cli.yaml`. По умолчанию:
```yaml
adapters:
  tick_hz: 100
  cli:
    enabled: true
  websocket:
    enabled: false
  rest:
    enabled: false
  telegram:
    enabled: false
  opensearch:
    enabled: false
```

### Тесты

```rust
test_tick_loop_processes_inject_command        // inject → broadcast::Result
test_tick_loop_updates_snapshot_after_interval // после N тиков snapshot обновлён
test_tick_loop_terminates_on_quit_command      // :quit → tick loop завершается
test_adapter_command_shutdown_constructor      // AdapterCommand::shutdown() корректен
test_command_response_message_variant         // process_adapter_command возвращает Message
test_process_inject_builds_server_message     // Inject → ServerMessage::Result с полями
```

### Критерий готовности

`cargo test -p axiom-agent` — зелёный.  
`cargo run --bin axiom-cli` — поведение CLI идентично Phase 0B.

---

## Phase 1 — WebSocket

**Цель:** WebSocket-клиент может отправлять команды и получать события.

### Структура модуля

```
crates/axiom-agent/src/ws/
    mod.rs
    protocol.rs   — ClientMessage, ServerMessage с serde
    handler.rs    — axum WebSocket per-connection handler
    server.rs     — axum router, AppState, запуск
```

### protocol.rs

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "inject")]         Inject         { text: String },
    #[serde(rename = "read_command")]   ReadCommand    { cmd: String },
    #[serde(rename = "mutate_command")] MutateCommand  { cmd: String },
    #[serde(rename = "subscribe")]      Subscribe      { channels: Vec<String> },
    #[serde(rename = "unsubscribe")]    Unsubscribe    { channels: Vec<String> },
    #[serde(rename = "domain_snapshot")]DomainSnapshot { domain_id: u16 },
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "result")]
    Result { command_id: String, path: String, domain_id: u16, domain_name: String,
             coherence: f32, reflex_hit: bool, traces_matched: u32,
             position: [i16; 3], shell: [u8; 8], event_id: u64 },

    #[serde(rename = "tick")]
    Tick { tick_count: u64, traces: u32, tension: u32, last_matched: u32 },

    #[serde(rename = "state")]
    State { tick_count: u64, snapshot: BroadcastSnapshot },

    #[serde(rename = "command_result")]
    CommandResult { command_id: String, output: String },

    #[serde(rename = "domain_detail")]
    DomainDetail(DomainDetailSnapshot),

    #[serde(rename = "error")]
    Error { command_id: Option<String>, message: String },
}
```

### handler.rs — per-connection

```rust
pub struct AppState {
    pub command_tx:   mpsc::Sender<AdapterCommand>,
    pub broadcast_tx: broadcast::Sender<ServerMessage>,
    pub snapshot:     Arc<RwLock<BroadcastSnapshot>>,
    pub next_conn_id: Arc<AtomicU64>,
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();
    let conn_id = state.next_conn_id.fetch_add(1, Ordering::Relaxed);
    let mut subscriptions: HashSet<String> = HashSet::new();

    // reader: WebSocket → command_tx
    let tx = state.command_tx.clone();
    let read_task = tokio::spawn(async move { ... });

    // writer: broadcast_rx → WebSocket (с фильтрацией по subscriptions)
    let write_task = tokio::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(msg) => {
                    // фильтруем: Tick только если "ticks" в subscriptions и т.д.
                    if should_send(&msg, &subscriptions) {
                        let json = serde_json::to_string(&msg).unwrap();
                        if ws_tx.send(Message::Text(json)).await.is_err() { break; }
                    }
                }
                Err(RecvError::Lagged(n)) => {
                    let warn = serde_json::to_string(&ServerMessage::Error {
                        command_id: None,
                        message: format!("lagged by {} messages", n),
                    }).unwrap();
                    let _ = ws_tx.send(Message::Text(warn)).await;
                }
                Err(RecvError::Closed) => break,
            }
        }
    });

    tokio::select! { _ = read_task => {}, _ = write_task => {} }
}
```

### Зависимости (добавить в axiom-agent/Cargo.toml)

```toml
axum       = { version = "0.8", features = ["ws"] }
tower-http = { version = "0.6", features = ["cors"] }
serde_json = "1"
tokio      = { version = "1", features = ["rt-multi-thread", "io-util", "macros", "sync", "time", "signal"] }
```

### Запуск

```bash
cargo run --bin axiom-cli -- --server         # порт 8080
cargo run --bin axiom-cli -- --server --port 3000
cargo run --bin axiom-cli -- --server --no-cli
```

### Тесты

```rust
test_ws_connect_no_error
test_ws_inject_returns_result
test_ws_tick_broadcast_arrives
test_ws_subscribe_ticks_only
test_ws_read_command_status_returns_output
test_ws_multiple_clients_all_receive_tick
test_ws_lagged_client_receives_warning
test_ws_disconnect_no_panic
```

Dev-dependency: `tokio-tungstenite` как WS клиент в тестах.

### Критерий готовности

`wscat -c ws://localhost:8080/ws` — подключается.  
`{"type":"read_command","cmd":":status"}` — получаем `command_result`.

---

## Phase 2 — REST

**Цель:** HTTP API поверх того же axum router что WebSocket.

### Маршруты

| Method | Path | Реализация |
|--------|------|-----------|
| GET | `/api/status` | `snapshot.read().await` → JSON (не лочит Engine) |
| GET | `/api/domains` | то же, поле `domain_summaries` |
| GET | `/api/domain/:id` | command_tx + broadcast correlation (Вариант C) |
| POST | `/api/inject` | `{"text":"..."}` → command_tx |
| POST | `/api/command` | `{"cmd":"...","type":"read"/"mutate"}` → command_tx |

### GET /api/domain/:id — Вариант C (correlation id)

```rust
async fn rest_domain(Path(id): Path<u16>, State(state): State<AppState>) -> impl IntoResponse {
    let cmd_id = Uuid::new_v4().to_string();
    let mut rx = state.broadcast_tx.subscribe();
    let _ = state.command_tx.send(AdapterCommand {
        id: cmd_id.clone(), source: AdapterSource::Rest,
        payload: AdapterPayload::DomainSnapshot { domain_id: id },
    }).await;

    match tokio::time::timeout(Duration::from_secs(5), async move {
        loop {
            match rx.recv().await {
                Ok(ServerMessage::DomainDetail(snap)) if snap.domain_id == id => {
                    return Ok(Json(snap));
                }
                Ok(_) => continue,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }).await {
        Ok(Ok(r)) => r.into_response(),
        _ => StatusCode::REQUEST_TIMEOUT.into_response(),
    }
}
```

### Тесты

```rust
test_rest_get_status_200
test_rest_get_domains_11_entries
test_rest_post_inject_returns_result
test_rest_post_invalid_json_returns_400
test_rest_get_domain_valid_id
test_rest_get_domain_invalid_id_404
test_rest_get_status_no_engine_lock  // Engine не блокируется при GET
```

### Критерий готовности

`curl http://localhost:8080/api/status` → JSON с tick_count.

---

## Phase 3 — egui Dashboard

**Цель:** визуализация в реальном времени. Отдельный crate — не зависит от axiom-*.

### Структура

```
tools/axiom-dashboard/
    Cargo.toml
    src/
        main.rs
        app.rs         — eframe::App
        ws_client.rs   — std::thread (не tokio — несовместим с eframe)
        protocol.rs    — копия ServerMessage/BroadcastSnapshot без axiom зависимостей
        panels/
            status.rs
            traces.rs
            space_view.rs  — 2D проекция XY
            input.rs
```

### Space View

- Токены = кружки, размер ~ `log(mass + 1)`, цвет = domain_id
- Якоря (is_anchor=true) = крест, поверх кружков
- Клик → детали токена в боковой панели
- Zoom и pan через egui_plot::Plot

### Зависимости

```toml
eframe      = { version = "0.29", default-features = false, features = ["glow"] }
egui_plot   = "0.29"
tungstenite = { version = "0.24", features = ["native-tls"] }
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
```

### Тесты

```rust
test_ws_client_receives_tick    // std::thread WS клиент
test_protocol_deserialize_tick  // JSON → ServerMessage::Tick
test_protocol_deserialize_state // JSON → ServerMessage::State
```

### Критерий готовности

`cargo run -p axiom-dashboard` при запущенном `--server` — Status panel показывает tick_count.

---

## Phase 4 — Telegram (feature "telegram")

**Цель:** управление через Telegram-бот.

### Архитектура

Два concurrent loop:
1. `get_updates()` → парсить → `command_tx.send(AdapterCommand)`
2. `broadcast_rx.recv()` → найти `command_id` в `pending: HashMap<String, i64>` → `send_message`

### Команды

| Telegram | Axiom |
|----------|-------|
| `/start` | приветствие + :status |
| `/status` | MetaRead ":status" |
| любой текст | Inject |
| `:traces`, `:domains`... | MetaRead |
| `:save`, `:tick N` | MetaMutate |

**Безопасность:** `allowed_users: Vec<i64>` — при непустом списке отклонять чужие chat_id.

### Критерий готовности

`cargo build --features telegram` — компилируется.

---

## Phase 5 — OpenSearch (feature "opensearch")

**Цель:** индексация событий для наблюдаемости.

### Flush без borrow conflict

```rust
async fn flush(&mut self) {
    let docs   = std::mem::take(&mut self.buffer);   // owned, не borrow
    let client = self.client.clone();                 // Clone, не borrow
    let url    = format!("{}/_bulk", self.base_url);
    let prefix = self.index_prefix.clone();
    self.last_flush_at = std::time::Instant::now();

    tokio::spawn(async move {                         // owned captures
        let body = build_bulk_body(&prefix, &docs);
        let _ = client.post(&url).body(body).send().await;
    });
}
```

`@timestamp` = `chrono::Utc::now().to_rfc3339()` — в адаптере, не в ядре (COM инвариант).

### Критерий готовности

`cargo build --features opensearch` — компилируется.  
С запущенным OpenSearch: документы появляются в `axiom-traces-*`.

---

## Чеклист перед началом каждой фазы

- [ ] `cargo test --workspace` — зелёный
- [ ] Прочитать соответствующий раздел спецификации V3.x
- [ ] После фазы: `cargo test --workspace` — зелёный
- [ ] После фазы: `cargo clippy --workspace` — без warnings
- [ ] Если появился технический долг — внести в DEFERRED.md до коммита

---

## Итоговая структура файлов

```
crates/axiom-runtime/src/
    broadcast.rs                 ✅ Phase 0A
    engine.rs                    ✅ Phase 0A (методы)

crates/axiom-agent/src/
    meta_commands.rs             ✅ Phase 0B
    adapter_command.rs           ✅ Phase 0C
    adapters_config.rs           ✅ Phase 0C
    protocol.rs                  ✅ Phase 0C
    tick_loop.rs                 ✅ Phase 0C
    ws/
        mod.rs, protocol.rs      ← Phase 1
        handler.rs, server.rs    ← Phase 1
    rest/
        mod.rs, handlers.rs      ← Phase 2
    telegram/mod.rs              ← Phase 4 (feature)
    opensearch/mod.rs            ← Phase 5 (feature)
    channels/cli.rs              ✅ Phase 0B/0C (рефактор)

tools/axiom-dashboard/src/       ← Phase 3 (новый crate)
```

---

## История изменений плана

- **V1.2** (2026-04-18): Phase 0B/0C отмечены завершёнными. Добавлены
  `adapters_config.rs` и `protocol.rs` в итоговую структуру файлов.
  Счётчик тестов 0C исправлен: +6 (не +10).
- **V1.1** (2026-04-17): Phase 0A отмечена завершённой (+18 тестов, 950 total с feature).
  `CommandResponse` уточнён: `Message(ServerMessage) | Quit | None` (по V3.1 спека).
  `matched` → `last_matched` везде. Технический долг из Phase 0A задокументирован.
  Phase 0C: уточнён `process_adapter_command` с полной сборкой `ServerMessage`.
- **V1.0** (2026-04-15): первая версия.
