# External Adapters — План реализации V1.0

**Версия:** 1.0  
**Дата:** 2026-04-15  
**Спецификация:** [External_Adapters_V3_0.md](External_Adapters_V3_0.md)  
**Статус:** готов к реализации

---

## Обзор

Шесть фаз. Каждая фаза — независимый коммит, все тесты зелёные после каждой.

| Фаза | Название | Изменённые crates | Тесты |
|------|----------|-------------------|-------|
| 0A | Convenience-методы + Snapshot-типы | axiom-runtime | +15 |
| 0B | Рефактор handle_meta_command | axiom-agent | 0 (все старые проходят) |
| 0C | AdapterCommand + tick_loop | axiom-agent | +10 |
| 1 | WebSocket | axiom-agent | +12 |
| 2 | REST | axiom-agent | +8 |
| 3 | egui Dashboard | axiom-dashboard (новый) | +5 |
| 4 | Telegram | axiom-agent (feature) | +6 |
| 5 | OpenSearch | axiom-agent (feature) | +8 |

**Инвариант:** после каждой фазы `cargo test --workspace` — зелёный.

---

## Phase 0A — Convenience-методы + Snapshot-типы

**Цель:** добавить методы и типы которые понадобятся всем следующим фазам.
Ничего не ломает — только добавление.

### Файлы

#### `crates/axiom-runtime/src/broadcast.rs` — новый файл

```rust
// Типы для broadcast через адаптеры.
// Намеренно не зависят от axiom-core напрямую — только примитивы.

#[derive(serde::Serialize, Clone, Default)]
pub struct BroadcastSnapshot { ... }   // см. спецификацию §5.3

#[derive(serde::Serialize, Clone)]
pub struct DomainSummary { ... }

#[derive(serde::Serialize, Clone)]
pub struct DomainDetailSnapshot { ... }

#[derive(serde::Serialize, Clone)]
pub struct TokenSnapshot { ... }

#[derive(serde::Serialize, Clone)]
pub struct ConnectionSnapshot { ... }
```

Добавить `serde` как опциональную зависимость в `axiom-runtime/Cargo.toml`:
```toml
[dependencies]
serde = { version = "1", features = ["derive"], optional = true }

[features]
default = []
adapters = ["serde"]
```

Все broadcast-типы под `#[cfg(feature = "adapters")]`.

**Почему feature-gate?**  
axiom-runtime — это ядро. serde как обязательная зависимость нарушает инвариант "ядро не зависит от внешних протоколов". Feature "adapters" явно означает "я собираю для внешней интеграции".

#### `crates/axiom-runtime/src/engine.rs` — добавить методы

```rust
// В блок impl AxiomEngine:

pub fn trace_count(&self) -> usize
pub fn tension_count(&self) -> usize
pub fn last_matched(&self) -> u32
pub fn snapshot_for_broadcast(&self) -> BroadcastSnapshot   // только под feature "adapters"
pub fn domain_detail_snapshot(&self, domain_id: u16) -> Option<DomainDetailSnapshot>
fn domain_summaries(&self) -> Vec<DomainSummary>  // приватный
```

`domain_name(id)` — взять из `crates/axiom-agent/src/effectors/message.rs` (там уже есть).
Вынести в `axiom-runtime/src/engine.rs` или продублировать inline.

#### `crates/axiom-runtime/src/lib.rs`

```rust
pub mod broadcast;
pub use broadcast::{BroadcastSnapshot, DomainSummary, DomainDetailSnapshot, TokenSnapshot, ConnectionSnapshot};
```

### Тесты (axiom-runtime/tests/)

```rust
test_trace_count_zero_on_new_engine
test_tension_count_zero_on_new_engine
test_last_matched_zero_on_new_engine
test_snapshot_for_broadcast_has_11_domains
test_snapshot_for_broadcast_tick_matches_engine
test_domain_detail_snapshot_returns_none_for_unknown_id
test_domain_detail_snapshot_sutra_domain_exists
test_token_snapshot_is_anchor_flag_for_locked_token
// + интеграционные (inject → trace_count увеличивается)
```

### Критерий готовности

`cargo test -p axiom-runtime` — зелёный.
`cargo clippy -p axiom-runtime --features adapters` — без warnings.

---

## Phase 0B — Рефактор handle_meta_command

**Цель:** разделить 700-строчный метод на две standalone-функции.
Поведение CLI не меняется — только внутренняя организация.

### Файлы

#### `crates/axiom-agent/src/meta_commands.rs` — новый файл

Содержит:
- `pub fn handle_meta_read(cmd, engine, anchor_set, perceptor, config) -> String`
- `pub fn handle_meta_mutate(cmd, engine, auto_saver, config) -> MetaMutateResult`
- `pub struct MetaMutateResult { output: String, action: MetaAction }`
- `pub enum MetaAction { None, Quit, EngineReplaced, AutosaveChanged(u32) }`

**Стратегия переноса:**

1. Скопировать весь `match parts[0]` из `handle_meta_command` в `handle_meta_read`
2. Убрать мутирующие ветки (`:save`, `:load`, `:autosave`, `:tick`, `:export`, `:import`, `:reset`, `:quit`)
3. Убрать все `println!` / `print!` — заменить на `writeln!(out, ...)` / `write!(out, ...)`
4. Мутирующие ветки перенести в `handle_meta_mutate`
5. `:watch` остаётся специальным случаем — `handle_meta_read` принимает `watch_fields: &HashSet<String>` для отображения, но *изменение* watch_fields происходит через специальный return в `MetaAction::WatchToggled(String)`

Расширить `MetaAction`:
```rust
pub enum MetaAction {
    None,
    Quit,
    EngineReplaced,
    AutosaveChanged(u32),
    WatchToggle(String),     // имя поля для toggle в watch_fields
    WatchClear,              // :watch off
}
```

#### `crates/axiom-agent/src/channels/cli.rs`

`handle_meta_command(&mut self, line)` превращается в:
```rust
fn handle_meta_command(&mut self, line: &str) -> bool {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();

    // Определить категорию команды
    let is_mutating = matches!(
        parts[0],
        ":save" | ":load" | ":autosave" | ":tick" | ":export" | ":import" | ":reset" | ":quit"
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
            }
            MetaAction::AutosaveChanged(n) => {
                self.engine.tick_schedule.persist_check_interval = n;
            }
            MetaAction::None => {}
            _ => {}
        }
    } else {
        let output = handle_meta_read(
            line,
            &self.engine,
            self.anchor_set.as_deref(),
            &self.perceptor,
            &self.config,
            &self.watch_fields,
            // + другие read-only поля: event_log, perf, multipass_count...
        );
        print!("{}", output);

        // handle_meta_read не может менять watch_fields — это делаем здесь
        if parts[0] == ":watch" {
            // парсим аргумент и обновляем self.watch_fields
        }
    }
    true
}
```

### Тесты

```rust
test_handle_meta_read_status_nonempty
test_handle_meta_read_domains_lists_11
test_handle_meta_read_traces_format
test_handle_meta_read_unknown_cmd_hint
test_handle_meta_mutate_quit_action
test_handle_meta_mutate_tick_advances_engine
test_handle_meta_mutate_save_creates_file    // интеграционный
```

### Критерий готовности

`cargo test -p axiom-agent` — все 97 тестов зелёные (не упал ни один существующий).
`cargo run --bin axiom-cli` — интерактивная сессия работает идентично до рефактора.

---

## Phase 0C — AdapterCommand + tick_loop

**Цель:** выделить tick loop из CliChannel в standalone-функцию.
CliChannel становится тонкой обёрткой. Готовность к Phase 1.

### Файлы

#### `crates/axiom-agent/src/adapter_command.rs` — новый файл

```rust
pub struct AdapterCommand { ... }
pub enum AdapterSource { ... }
pub enum AdapterPayload { ... }
pub enum CommandResponse { ... }
```

#### `crates/axiom-agent/src/tick_loop.rs` — новый файл

```rust
pub async fn tick_loop(
    mut engine: AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver: AutoSaver,
    anchor_set: Option<Arc<AnchorSet>>,
    config: AdaptersConfig,
)
```

Функция `process_adapter_command(...)` — тоже здесь.

#### `crates/axiom-agent/src/channels/cli.rs`

`CliChannel::run()` рефакторится:
1. Создаёт `(command_tx, command_rx)` пару
2. Запускает stdin reader → отправляет `AdapterCommand` в `command_tx`
3. Запускает `tick_loop(engine, command_rx, broadcast_tx, snapshot, ...)`
4. Ожидает `tick_loop` (он завершается при `:quit` или EOF)

`CliChannel` больше не владеет `engine` напрямую после `run()` — engine передаётся в tick_loop.

### Конфигурация

`AdaptersConfig` добавляется в `axiom-cli.yaml`.
По умолчанию — только CLI включён, остальное `enabled: false`.

### Тесты

```rust
test_tick_loop_processes_inject_command
test_tick_loop_updates_snapshot_after_state_interval
test_tick_loop_terminates_on_quit_command
test_adapter_command_inject_roundtrip
```

### Критерий готовности

`cargo test -p axiom-agent` — зелёный.
`cargo run --bin axiom-cli` — работает, поведение CLI не изменилось.

---

## Phase 1 — WebSocket

**Цель:** любой WebSocket-клиент может отправлять команды и получать события.

### Файлы

```
crates/axiom-agent/src/ws/
    mod.rs
    protocol.rs      — ClientMessage, ServerMessage (serde)
    handler.rs       — axum WebSocket handler
    server.rs        — axum router, запуск
```

#### `ws/protocol.rs`

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Inject       { text: String },
    ReadCommand  { cmd: String },
    MutateCommand{ cmd: String },
    Subscribe    { channels: Vec<String> },
    Unsubscribe  { channels: Vec<String> },
    DomainSnapshot { domain_id: u16 },
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage { ... }   // полностью из спецификации §5.2
```

#### `ws/handler.rs`

```rust
// Per-connection state
struct WsConnection {
    id:             u64,
    command_tx:     mpsc::Sender<AdapterCommand>,
    subscriptions:  HashSet<String>,  // "ticks", "state", "results"
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();
    let conn_id = state.next_conn_id.fetch_add(1, Ordering::Relaxed);

    // Читаем из WebSocket → command_tx
    let read_task = tokio::spawn(async move { ... });

    // Читаем из broadcast_rx → WebSocket
    let write_task = tokio::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(msg) => {
                    // Проверяем подписки (subscriptions хранятся локально)
                    let json = serde_json::to_string(&msg).unwrap();
                    if sender.send(Message::Text(json)).await.is_err() { break; }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    // Отправить warning, продолжить
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    tokio::select! { _ = read_task => {}, _ = write_task => {} }
}
```

#### `ws/server.rs`

```rust
pub struct AppState {
    pub command_tx:    mpsc::Sender<AdapterCommand>,
    pub broadcast_tx:  broadcast::Sender<ServerMessage>,
    pub snapshot:      Arc<RwLock<BroadcastSnapshot>>,
    pub next_conn_id:  Arc<AtomicU64>,
}

pub async fn start_server(
    host: &str,
    port: u16,
    state: AppState,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/status", get(rest_status))
        .route("/api/inject", post(rest_inject))
        .route("/api/command", post(rest_command))
        .route("/api/domain/:id", get(rest_domain))
        .layer(CorsLayer::permissive())  // tower-http
        .with_state(state);

    let addr = format!("{}:{}", host, port).parse()?;
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
```

#### Флаг запуска

В `main.rs`:
```rust
if args.server {
    let server_handle = tokio::spawn(start_server(host, port, app_state.clone()));
    tokio::select! {
        _ = tick_loop_handle => {}
        _ = server_handle => {}
        _ = shutdown_signal => { /* quit */ }
    }
}
```

### Тесты

```rust
test_ws_client_connect_receives_no_error
test_ws_inject_returns_result_message
test_ws_tick_broadcast_arrives_at_client
test_ws_subscribe_ticks_only
test_ws_read_command_status_returns_output
test_ws_multiple_clients_all_receive_tick
test_ws_lagging_client_receives_lagged_warning
test_ws_disconnect_no_panic_in_tick_loop
```

Для тестов использовать `tokio-tungstenite` как WebSocket клиент в тестах (dev-dependency).

### Критерий готовности

`cargo test -p axiom-agent` — зелёный.
`cargo run --bin axiom-cli -- --server` — подключается wscat, `:status` возвращает JSON.

---

## Phase 2 — REST

**Цель:** HTTP API для интеграции без WebSocket.

### Маршруты

| Method | Path | Handler |
|--------|------|---------|
| GET | `/api/status` | `snapshot.read()` → JSON |
| GET | `/api/domain/:id` | command_tx → broadcast → JSON |
| GET | `/api/domains` | `snapshot.read().domain_summaries` → JSON |
| POST | `/api/inject` | `{ "text": "..." }` → command_tx → broadcast |
| POST | `/api/command` | `{ "cmd": "...", "type": "read"|"mutate" }` → command_tx |

### Детали GET /api/domain/:id

Реализация Варианта C (broadcast с command_id):
```rust
async fn rest_domain(
    Path(domain_id): Path<u16>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let cmd_id = Uuid::new_v4().to_string();
    let mut rx = state.broadcast_tx.subscribe();

    state.command_tx.send(AdapterCommand {
        id: cmd_id.clone(),
        source: AdapterSource::Rest,
        payload: AdapterPayload::DomainSnapshot { domain_id },
    }).await?;

    let result = tokio::time::timeout(Duration::from_secs(5), async move {
        loop {
            match rx.recv().await {
                Ok(ServerMessage::DomainDetail(snap)) if snap.domain_id == domain_id => {
                    return Ok(Json(snap));
                }
                Ok(_) => continue,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }).await;

    match result {
        Ok(Ok(response)) => response.into_response(),
        _ => StatusCode::REQUEST_TIMEOUT.into_response(),
    }
}
```

### Тесты

Использовать `axum::test` helpers (без поднятия реального TCP):
```rust
test_rest_get_status_returns_snapshot
test_rest_get_domains_returns_11_entries
test_rest_post_inject_returns_result
test_rest_post_invalid_json_returns_400
test_rest_get_domain_valid_id_returns_tokens
test_rest_get_domain_invalid_id_returns_404
test_rest_engine_not_locked_during_get_status
```

### Критерий готовности

`curl http://localhost:8080/api/status` → JSON с tick_count и доменами.

---

## Phase 3 — egui Dashboard

**Цель:** визуализация состояния в реальном времени.

### Структура crate

```
tools/axiom-dashboard/
    src/
        main.rs
        app.rs          — eframe::App impl
        ws_client.rs    — WebSocket в отдельном потоке (std::thread, не tokio)
        protocol.rs     — копия ServerMessage / BroadcastSnapshot (без axiom зависимостей)
        panels/
            status.rs
            traces.rs
            space_view.rs
            input.rs
```

**Почему std::thread, не tokio?**  
eframe — синхронный, блокирует главный поток под render loop.
Tokio runtime несовместим с eframe main loop.
Решение: `std::thread::spawn` для WS клиента + `std::sync::mpsc` для передачи событий в UI.

### Space View

2D проекция семантического пространства (оси X, Y из Token.position):
- Токены = кружки, размер ~ `log(mass + 1)`, цвет = domain_id
- Якоря (is_anchor=true) = крест, всегда поверх
- Клик на токен → показать детали (sutra_id, shell, origin)
- Zoom и pan через egui_plot::Plot

### Тесты

Тесты для dashboard минимальные (UI не тестируется автоматически):
```rust
test_ws_client_receives_tick_message
test_protocol_deserialize_server_message
test_protocol_deserialize_broadcast_snapshot
```

### Критерий готовности

`cargo run -p axiom-dashboard` при запущенном `--server` показывает движение тиков в Status panel.

---

## Phase 4 — Telegram

**Цель:** управление системой через Telegram-бота.

### Архитектура

```rust
// crates/axiom-agent/src/telegram/mod.rs
// #[cfg(feature = "telegram")]

struct TelegramAdapter {
    bot:         Bot,
    command_tx:  mpsc::Sender<AdapterCommand>,
    broadcast_rx: broadcast::Receiver<ServerMessage>,
    // command_id → chat_id: чтобы знать куда отправить ответ
    pending:     HashMap<String, i64>,
}
```

Два параллельных loop:
1. **Incoming loop:** `bot.get_updates()` → парсить команду → `command_tx.send()`
2. **Outgoing loop:** `broadcast_rx.recv()` → найти `command_id` в `pending` → `bot.send_message(chat_id, output)`

### Команды

| Telegram | Axiom |
|----------|-------|
| `/start` | приветствие + текущий :status |
| `/status` | MetaRead ":status" |
| любой текст | Inject { text } |
| `:status`, `:traces` и т.д. | MetaRead |
| `:save`, `:tick N` | MetaMutate |

**Безопасность:** `allowed_users: Vec<i64>` — если список не пустой, отклонять чужие chat_id.

### Тесты

```rust
test_telegram_allowed_users_filter
test_telegram_command_to_adapter_command_mapping
test_telegram_pending_table_cleanup_after_response
```

### Критерий готовности

`cargo build --features telegram` — компилируется.

---

## Phase 5 — OpenSearch

**Цель:** индексация событий для наблюдаемости.

### Архитектура

```rust
// crates/axiom-agent/src/opensearch/mod.rs
// #[cfg(feature = "opensearch")]

struct OpenSearchWriter {
    client:        reqwest::Client,
    base_url:      String,
    index_prefix:  String,
    buffer:        Vec<serde_json::Value>,
    config:        OpenSearchConfig,
    last_flush_at: std::time::Instant,
}

impl OpenSearchWriter {
    // Слушает broadcast_rx, накапливает документы
    pub async fn run(mut self, mut rx: broadcast::Receiver<ServerMessage>) {
        loop {
            match rx.recv().await {
                Ok(ServerMessage::Result { .. }) if self.config.index_traces => {
                    self.buffer.push(self.result_to_doc(/* ... */));
                }
                Ok(ServerMessage::Tick { .. }) if self.config.index_events => {
                    self.buffer.push(self.tick_to_doc(/* ... */));
                }
                _ => {}
            }

            let should_flush = self.buffer.len() >= self.config.batch_size
                || self.last_flush_at.elapsed() >= Duration::from_millis(self.config.flush_interval_ms);

            if should_flush && !self.buffer.is_empty() {
                self.flush().await;
            }
        }
    }

    async fn flush(&mut self) {
        let docs = std::mem::take(&mut self.buffer);
        let client = self.client.clone();
        let url = format!("{}/_bulk", self.base_url);
        let prefix = self.index_prefix.clone();
        self.last_flush_at = std::time::Instant::now();

        // tokio::spawn — owned всё, не borrow
        tokio::spawn(async move {
            let body = build_bulk_body(&prefix, &docs);
            let _ = client.post(&url).body(body).send().await;
        });
    }
}
```

**@timestamp:** `chrono::Utc::now().to_rfc3339()` — генерируется здесь, в адаптере.
Не запрашивается у Engine, не из event_id. COM инвариант не нарушен.

### Тесты

```rust
test_opensearch_flush_on_batch_size
test_opensearch_flush_on_interval
test_opensearch_no_borrow_after_flush   // docs взяты через take — нет borrow conflict
test_opensearch_timestamp_is_wall_clock // не event_id
```

### Критерий готовности

`cargo build --features opensearch` — компилируется.
С запущенным OpenSearch: документы появляются в индексе `axiom-traces-*`.

---

## Чеклист перед началом каждой фазы

- [ ] `cargo test --workspace` — зелёный (точка отсчёта)
- [ ] Прочитать спецификацию §3 (Phase 0A) или соответствующий раздел
- [ ] Проверить что новые файлы объявлены в `mod.rs` / `lib.rs`
- [ ] Новые публичные типы: добавить `#[doc]` комментарий
- [ ] После фазы: `cargo test --workspace` — зелёный
- [ ] После фазы: `cargo clippy --workspace` — без warnings
- [ ] Обновить STATUS.md (тесты, описание)

---

## Порядок файлов (итого)

```
crates/axiom-runtime/src/
    broadcast.rs                    ← Phase 0A (новый)
    engine.rs                       ← Phase 0A (методы)

crates/axiom-agent/src/
    meta_commands.rs                ← Phase 0B (новый)
    adapter_command.rs              ← Phase 0C (новый)
    tick_loop.rs                    ← Phase 0C (новый)
    ws/
        mod.rs                      ← Phase 1 (новый)
        protocol.rs                 ← Phase 1 (новый)
        handler.rs                  ← Phase 1 (новый)
        server.rs                   ← Phase 1 (новый)
    rest/
        mod.rs                      ← Phase 2 (новый)
        handlers.rs                 ← Phase 2 (новый)
    telegram/
        mod.rs                      ← Phase 4 (новый, feature-gated)
    opensearch/
        mod.rs                      ← Phase 5 (новый, feature-gated)
    channels/cli.rs                 ← Phase 0B/0C (рефактор)

tools/axiom-dashboard/
    src/
        main.rs                     ← Phase 3 (новый crate)
        app.rs
        ws_client.rs
        protocol.rs
        panels/
```
