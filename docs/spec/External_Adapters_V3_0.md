# External Adapters V3.0 — WebSocket + REST + egui Dashboard + Telegram + OpenSearch

**Версия:** 3.0  
**Дата:** 2026-04-15  
**Для:** Claude Sonnet (реализация)  
**Контекст:** 932 теста, CLI Extended V1.0, Anchor Tokens V1.0, Memory Persistence V1.0 — всё работает.

---

## Изменения V3.0 vs V2.0

- **Исправлено:** `drain_pending_impulses()` / `pending_impulses` — поле не существует. Импульсы обрабатываются *внутри* `handle_tick_forward` через `TickSchedule`. Лишний шаг из tick loop убран.
- **Исправлено:** Все пути к полям верифицированы по реальному коду (`ashti.experience()`, `last_traces_matched.get()`, `tension_count()`). `last_routing` на Arbiter отсутствует — использован правильный путь.
- **Исправлено:** Порядок clone/write для `BroadcastSnapshot` (нельзя move значение до clone).
- **Исправлено:** `handle_meta_command` — это метод `&mut self` на `CliChannel`. Рефактор разделяет её на две standalone-функции с конкретными сигнатурами.
- **Исправлено:** `:load` заменяет `engine` целиком — не может быть просто мутирующей командой без специального handling в tick loop.
- **Добавлено:** Graceful shutdown в headless-режиме через SIGTERM.
- **Добавлено:** `tick_hz` в `AdaptersConfig`.
- **Добавлено:** Полный список команд по категориям (read-only vs mutable).
- **Добавлено:** Раздел о подводных камнях (Section 11).

---

## 1. Архитектурное решение: Engine Access Pattern

**Одно правило:** AxiomEngine мутирует только tick loop. Все остальные — через каналы или snapshot.

```
                    ┌──────────────────────────┐
  CLI stdin ────→   │                          │
  WebSocket  ───→   │  command_tx (mpsc)       │ ──→ tick loop ──→ AxiomEngine (единственный writer)
  REST POST  ───→   │                          │         │
  Telegram   ───→   │                          │         │ обновляет каждые N тиков
                    └──────────────────────────┘         ↓
                                                  ┌──────────────────┐
  CLI stdout ←───                                 │ BroadcastSnapshot│
  WebSocket  ←─── broadcast_tx ←─────────────────│ Arc<RwLock<...>> │
  REST GET   ←─── Arc<RwLock<BroadcastSnapshot>> │                  │
  Telegram   ←───                                 └──────────────────┘
```

### 1.1 Каналы

```rust
// Входящие команды от любого адаптера
let (command_tx, command_rx) = mpsc::channel::<AdapterCommand>(256);

// Исходящие события для всех подписчиков (broadcast — каждый получает копию)
let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(1024);

// Snapshot для read-only доступа (REST GET, dashboard polling)
let snapshot: Arc<RwLock<BroadcastSnapshot>> = Arc::new(RwLock::new(BroadcastSnapshot::default()));
```

### 1.2 AdapterCommand

```rust
pub struct AdapterCommand {
    /// Уникальный ID для корреляции ответа (UUID v4 или монотонный счётчик)
    pub id: String,
    /// Откуда пришла команда
    pub source: AdapterSource,
    /// Что делать
    pub payload: AdapterPayload,
}

pub enum AdapterSource {
    Cli,
    WebSocket(u64),   // connection_id
    Rest,
    Telegram(i64),    // chat_id
}

pub enum AdapterPayload {
    /// Пользовательский текст → TextPerceptor → InjectToken
    Inject { text: String },
    /// Мета-команда только для чтения: :status, :traces, :domains, :anchors...
    MetaRead { cmd: String },
    /// Мета-команда с мутацией: :save, :load, :autosave, :tick, :export, :import
    MetaMutate { cmd: String },
    /// Подписка на канал broadcast
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    /// Запрос детального snapshot одного домена
    DomainSnapshot { domain_id: u16 },
}
```

**Почему два типа MetaCommand?**  
Read-команды можно обработать без Engine lock (в tick loop они всё равно идут туда,
но логически они не мутируют состояние). Мутирующие команды требуют строгой сериализации
через tick loop — и некоторые (`:load`) требуют специального handling.

---

## 2. Рефактор handle_meta_command

### 2.1 Текущее состояние

В коде `crates/axiom-agent/src/channels/cli.rs:759` находится:
```rust
fn handle_meta_command(&mut self, line: &str) -> bool  // метод &mut CliChannel, ~700 строк
```

Возвращает `false` только для `:quit`. Всё печатает через `println!`.

### 2.2 Цель рефактора

Разделить на две standalone-функции в отдельном модуле `crates/axiom-agent/src/meta_commands.rs`:

```rust
/// Команды только для чтения — не мутируют Engine.
/// Возвращают строку для отправки через любой транспорт.
///
/// Принимает &AxiomEngine (не &mut), поэтому безопасна для вызова
/// из любого контекста, в том числе в tick loop без необходимости
/// получать &mut в момент, когда другая ссылка уже существует.
pub fn handle_meta_read(
    cmd: &str,
    engine: &AxiomEngine,
    anchor_set: Option<&AnchorSet>,
    perceptor: &TextPerceptor,
    config: &CliConfig,
) -> String

/// Команды с мутацией — вызываются только из tick loop.
/// Возвращает MetaMutateResult (не просто String — некоторые команды
/// изменяют не только engine, но и auto_saver, tick_schedule и т.д.).
pub fn handle_meta_mutate(
    cmd: &str,
    engine: &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    config: &CliConfig,
) -> MetaMutateResult

pub struct MetaMutateResult {
    pub output: String,
    pub action: MetaAction,
}

pub enum MetaAction {
    None,
    Quit,                      // :quit — остановить всё
    EngineReplaced,            // :load — engine заменён, tick loop должен сбросить состояние
    AutosaveChanged(u32),      // :autosave on N — новый интервал
}
```

### 2.3 Разделение команд

**Read-only** (→ `handle_meta_read`):
`:status`, `:domains`, `:tokens`, `:traces`, `:tension`, `:depth`, `:arbiter`,
`:frontier`, `:guardian`, `:perf`, `:events`, `:watch`, `:config`,
`:trace <id>`, `:connections <id>`, `:dream`, `:multipass`, `:reflector`,
`:impulses`, `:schema`, `:anchors`, `:match`, `:help`, `:snapshot`

**Mutating** (→ `handle_meta_mutate`):
`:save`, `:load`, `:autosave`, `:tick N`, `:export`, `:import`, `:reset`, `:quit`

**Внимание: `:watch`** — команда включает/выключает поле в `HashSet<String>` внутри
`CliChannel`. После рефактора `handle_meta_read` получает immutable watch_fields,
а изменение набора происходит через отдельный механизм в tick loop. Детали — в Section 11.

### 2.4 Специальный случай: `:load`

`:load` заменяет `self.engine` полностью (строка 888 в cli.rs):
```rust
self.engine = r.engine;
self.engine.tick_schedule = self.config.tick_schedule;
self.auto_saver.reset_save_tick(self.engine.tick_count);
```

В tick loop это означает: после получения `MetaAction::EngineReplaced`, tick loop
должен также переинициализировать `perceptor` (если anchor_set изменился) и сбросить
все счётчики состояния (`last_traces`, `last_tension`, `multipass_count`).

### 2.5 CLI-адаптер после рефактора

```rust
// В CliChannel::run() — read-only команды
AdapterPayload::MetaRead { cmd } => {
    let output = handle_meta_read(&cmd, &engine, anchor_set.as_deref(), &perceptor, &config);
    print!("{}", output);  // CLI печатает напрямую
    // В tick loop — отправить через broadcast_tx если source != Cli
}

// В CliChannel::run() — мутирующие команды
AdapterPayload::MetaMutate { cmd } => {
    let result = handle_meta_mutate(&cmd, &mut engine, &mut auto_saver, &config);
    print!("{}", result.output);
    match result.action {
        MetaAction::Quit => return,
        MetaAction::EngineReplaced => { /* reset счётчики */ }
        MetaAction::AutosaveChanged(n) => { /* обновить интервал */ }
        MetaAction::None => {}
    }
}
```

---

## 3. Convenience-методы на AxiomEngine

Все пути верифицированы по реальному коду.

```rust
impl AxiomEngine {
    // ── Read-only accessors ──────────────────────────────────────

    /// Число следов опыта.
    /// Путь: ashti.experience() → ExperienceModule::trace_count()
    /// (crates/axiom-arbiter/src/experience.rs:347)
    pub fn trace_count(&self) -> usize {
        self.ashti.experience().trace_count()
    }

    /// Число активных tension traces.
    /// Путь: ashti.experience() → ExperienceModule::tension_count()
    /// (crates/axiom-arbiter/src/experience.rs:551)
    pub fn tension_count(&self) -> usize {
        self.ashti.experience().tension_count()
    }

    /// Число следов, совпавших при последней маршрутизации.
    /// Путь: ashti.experience() → ExperienceModule::last_traces_matched (Cell<u32>)
    /// (crates/axiom-arbiter/src/experience.rs:71)
    ///
    /// ВНИМАНИЕ: last_traces_matched — это Cell<u32>, значит .get() без &mut.
    /// НЕТ last_routing на AshtiCore — arbiter приватный, прямого доступа нет.
    pub fn last_matched(&self) -> u32 {
        self.ashti.experience().last_traces_matched.get()
    }

    /// Лёгкий snapshot для broadcast — только числа, без клонирования токенов.
    pub fn snapshot_for_broadcast(&self) -> BroadcastSnapshot {
        BroadcastSnapshot {
            tick_count:    self.tick_count,
            com_next_id:   self.com_next_id,
            trace_count:   self.trace_count(),
            tension_count: self.tension_count(),
            domain_summaries: self.domain_summaries(),
        }
    }

    /// Snapshot отдельного домена для детального просмотра (dashboard).
    /// Путь: ashti.index_of(domain_id) → ashti.state(idx)
    /// (crates/axiom-domain/src/ashti_core.rs:155-178)
    pub fn domain_detail_snapshot(&self, domain_id: u16) -> Option<DomainDetailSnapshot> {
        let idx = self.ashti.index_of(domain_id)?;
        let state = self.ashti.state(idx)?;
        Some(DomainDetailSnapshot {
            domain_id,
            tokens: state.tokens.iter().map(TokenSnapshot::from).collect(),
            connections: state.connections.iter().map(ConnectionSnapshot::from).collect(),
        })
    }

    fn domain_summaries(&self) -> Vec<DomainSummary> {
        (0u16..=10).map(|offset| {
            let id = 100 + offset;
            let count = self.ashti.token_count(id);   // уже существует
            let conn_count = self.ashti.index_of(id)
                .and_then(|i| self.ashti.state(i))
                .map_or(0, |s| s.connections.len());
            DomainSummary {
                domain_id: id,
                name: domain_name(id).to_string(),
                token_count: count,
                connection_count: conn_count,
            }
        }).collect()
    }
}
```

**Чего нет и не нужно:**

`drain_pending_impulses()` / `pending_impulses` — **не существует** и **не нужно**.

Импульсы (TensionTrace + Goal) генерируются и обрабатываются *внутри*
`handle_tick_forward` через `TickSchedule` (engine.rs:446-462):
```rust
// Уже в handle_tick_forward:
if s.tension_check_interval > 0 && t % s.tension_check_interval as u64 == 0 {
    let impulses = self.ashti.arbiter_heartbeat_pulse(t, true);
    for mut token in impulses { ... route_token ... }
}
```
Tick loop адаптеров не должен повторять это — достаточно вызвать `engine.process_command(&tick_cmd)`.

---

## 4. Tick Loop

```rust
/// Главный цикл обработки — единственный writer AxiomEngine.
///
/// Принимает AxiomEngine по значению (владеет им).
/// Все адаптеры взаимодействуют только через command_rx и broadcast_tx/snapshot.
async fn tick_loop(
    mut engine: AxiomEngine,
    mut command_rx: mpsc::Receiver<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    mut auto_saver: AutoSaver,
    anchor_set: Option<Arc<AnchorSet>>,
    config: AdaptersConfig,
) {
    let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
    let tick_ms = 1000u64 / config.tick_hz.max(1) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(tick_ms));
    let mut perceptor = make_perceptor(&anchor_set);

    loop {
        interval.tick().await;

        // 1. Обработать все входящие команды (non-blocking drain)
        while let Ok(cmd) = command_rx.try_recv() {
            let response = process_adapter_command(
                cmd.id.clone(),
                cmd.payload,
                &mut engine,
                &mut auto_saver,
                &mut perceptor,
                &anchor_set,
                &config,
            );

            match response {
                // process_adapter_command сам строит нужный ServerMessage —
                // tick loop не знает деталей преобразования ProcessingResult → поля протокола.
                CommandResponse::Message(msg) => {
                    let _ = broadcast_tx.send(msg);
                }
                CommandResponse::Quit => {
                    // Автосохранение перед выходом
                    if auto_saver.config.enabled {
                        let _ = auto_saver.force_save(&engine, Path::new(&config.data_dir));
                    }
                    return; // Завершаем tick loop → все задачи в tokio завершатся
                }
                CommandResponse::None => {}
            }
        }

        // 2. Tick ядра.
        //    handle_tick_forward уже обрабатывает tension + goal impulses через TickSchedule.
        //    Ничего дополнительного здесь не нужно.
        engine.process_command(&tick_cmd);
        let t = engine.tick_count;

        // 3. Broadcast тиков (каждые tick_broadcast_interval тиков)
        if t % config.websocket.tick_broadcast_interval as u64 == 0 {
            let _ = broadcast_tx.send(ServerMessage::Tick {
                tick_count:   t,
                traces:       engine.trace_count() as u32,
                tension:      engine.tension_count() as u32,
                last_matched: engine.last_matched(), // last-seen, не sum за интервал
            });
        }

        // 4. Обновить snapshot (каждые state_broadcast_interval тиков).
        //    ВАЖНО: сначала clone (для broadcast), потом write (чтобы не move значение).
        if t % config.websocket.state_broadcast_interval as u64 == 0 {
            let new_snapshot = engine.snapshot_for_broadcast();
            let for_broadcast = new_snapshot.clone();  // clone ДО write
            *snapshot.write().await = new_snapshot;
            let _ = broadcast_tx.send(ServerMessage::State {
                tick_count: t,
                snapshot: for_broadcast,
            });
        }

        // 5. Автосохранение (если включено через TickSchedule)
        if let Some(interval) = auto_saver.check_tick(t) {
            let _ = auto_saver.try_save(&engine, Path::new(&config.data_dir), interval);
        }
    }
}

/// CommandResponse — результат обработки одной AdapterCommand в tick loop.
///
/// Вариант Message(ServerMessage) несёт готовое сообщение для broadcast.
/// process_adapter_command сам отвечает за сборку ServerMessage из ProcessingResult —
/// tick loop об этом ничего не знает (разделение ответственности).
pub enum CommandResponse {
    Message(ServerMessage),  // готово к отправке в broadcast_tx
    Quit,                    // :quit — завершить tick loop после автосохранения
    None,                    // Subscribe/Unsubscribe — обработано на уровне адаптера
}

fn process_adapter_command(
    id: String,
    payload: AdapterPayload,
    engine: &mut AxiomEngine,
    auto_saver: &mut AutoSaver,
    perceptor: &mut TextPerceptor,
    anchor_set: &Option<Arc<AnchorSet>>,
    config: &AdaptersConfig,
) -> CommandResponse {
    match payload {
        AdapterPayload::Inject { text } => {
            let ucl_cmd = perceptor.perceive(&text);
            let r = engine.process_and_observe(&ucl_cmd);
            // Конвертация ProcessingResult → ServerMessage::Result здесь,
            // а не в tick loop. Tick loop получает готовый ServerMessage.
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
            let output = handle_meta_read(&cmd, engine, anchor_set.as_deref(), perceptor, &config.cli);
            CommandResponse::Message(ServerMessage::CommandResult { command_id: id, output })
        }
        AdapterPayload::MetaMutate { cmd } => {
            let result = handle_meta_mutate(&cmd, engine, auto_saver, &config.cli);
            match result.action {
                MetaAction::Quit => CommandResponse::Quit,
                MetaAction::EngineReplaced => {
                    *perceptor = make_perceptor(anchor_set);
                    CommandResponse::Message(ServerMessage::CommandResult {
                        command_id: id,
                        output: result.output,
                    })
                }
                _ => CommandResponse::Message(ServerMessage::CommandResult {
                    command_id: id,
                    output: result.output,
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
            CommandResponse::None
        }
    }
}

fn make_perceptor(anchor_set: &Option<Arc<AnchorSet>>) -> TextPerceptor {
    match anchor_set {
        Some(a) => TextPerceptor::with_anchors(Arc::clone(a)),
        None => TextPerceptor::new(),
    }
}
```

---

## 5. Протокол (protocol.rs)

### 5.1 ClientMessage

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "inject")]
    Inject { text: String },

    /// Команда только для чтения: ":status", ":traces", ":domains"...
    #[serde(rename = "read_command")]
    ReadCommand { cmd: String },

    /// Мутирующая команда: ":save", ":load", ":tick N"...
    #[serde(rename = "mutate_command")]
    MutateCommand { cmd: String },

    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },

    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },

    #[serde(rename = "domain_snapshot")]
    DomainSnapshot { domain_id: u16 },
}
```

**Почему два типа команд вместо одного `command`?**  
Клиент (dashboard, внешняя система) должен явно знать семантику запроса.
Это предотвращает случайные мутации при опечатке в команде.

### 5.2 ServerMessage

```rust
#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    /// Результат inject (ProcessingResult в JSON)
    #[serde(rename = "result")]
    Result {
        command_id: String,
        path: String,         // "fast_path" | "slow_path" | "multi_pass(N)"
        domain_id: u16,
        domain_name: String,
        coherence: f32,
        reflex_hit: bool,
        traces_matched: u32,
        position: [i16; 3],
        shell: [u8; 8],
        event_id: u64,
    },

    /// Периодический broadcast тика (каждые tick_broadcast_interval тиков)
    #[serde(rename = "tick")]
    Tick {
        tick_count: u64,
        traces:     u32,
        tension:    u32,
        /// Число трейсов совпавших при *последнем* inject до момента этого broadcast.
        /// Семантика: last-seen, не сумма и не среднее за интервал.
        /// Источник: ExperienceModule::last_traces_matched (Cell<u32>) — перезаписывается
        /// при каждом route_token. Если между двумя broadcast не было inject — значение
        /// то же что в предыдущем Tick.
        last_matched: u32,
    },

    /// Полный snapshot состояния (каждые state_broadcast_interval тиков)
    #[serde(rename = "state")]
    State {
        tick_count: u64,
        snapshot: BroadcastSnapshot,
    },

    /// Ответ на любую команду (:status, :save, :load...)
    #[serde(rename = "command_result")]
    CommandResult {
        command_id: String,
        output: String,
    },

    /// Детальный snapshot одного домена (ответ на DomainSnapshot request)
    #[serde(rename = "domain_detail")]
    DomainDetail(DomainDetailSnapshot),

    /// Ошибка обработки команды
    #[serde(rename = "error")]
    Error {
        command_id: Option<String>,
        message: String,
    },
}
```

### 5.3 Snapshot-типы

```rust
/// Лёгкий snapshot для периодического broadcast.
/// Только числа — без клонирования токенов и связей.
#[derive(Serialize, Clone, Default)]
pub struct BroadcastSnapshot {
    pub tick_count:       u64,
    pub com_next_id:      u64,
    pub trace_count:      usize,
    pub tension_count:    usize,
    pub domain_summaries: Vec<DomainSummary>,
}

#[derive(Serialize, Clone)]
pub struct DomainSummary {
    pub domain_id:        u16,
    pub name:             String,
    pub token_count:      usize,
    pub connection_count: usize,
}

/// Детальный snapshot одного домена — только по явному запросу.
/// Может быть большим (сотни токенов) — не для периодического broadcast.
#[derive(Serialize, Clone)]
pub struct DomainDetailSnapshot {
    pub domain_id:   u16,
    pub tokens:      Vec<TokenSnapshot>,
    pub connections: Vec<ConnectionSnapshot>,
}

/// Компактное представление Token для передачи через JSON.
/// Не 64-байтовый Token — только поля нужные для визуализации.
#[derive(Serialize, Clone)]
pub struct TokenSnapshot {
    pub sutra_id:    u32,
    pub position:    [i16; 3],
    pub shell:       [u8; 8],
    pub mass:        u8,
    pub temperature: u8,
    pub valence:     i8,
    pub origin:      u16,
    pub is_anchor:   bool,    // mass==255 && temperature==0 && state==STATE_LOCKED
}

#[derive(Serialize, Clone)]
pub struct ConnectionSnapshot {
    pub source_id: u32,
    pub target_id: u32,
    pub weight:    f32,
}
```

**Об `is_anchor` в TokenSnapshot:** нельзя просто проверить `mass==255 && temperature==0`,
потому что обычные тяжёлые токены могут остыть. Надёжный способ — `state == STATE_LOCKED`.
Поле `state` есть в `Token` (u8, STATE_LOCKED=3, инвариант 9.7).

---

## 6. Зависимости

```toml
# crates/axiom-agent/Cargo.toml

[dependencies]
axiom-runtime = { path = "../axiom-runtime" }
axiom-core    = { path = "../axiom-core" }
axiom-config  = { path = "../axiom-config" }
axiom-persist = { path = "../axiom-persist" }
axiom-ucl     = { path = "../axiom-ucl" }

tokio = { version = "1", features = ["rt-multi-thread", "io-util", "macros", "sync", "time", "signal"] }

# Adapters (Phase 1-2)
axum         = { version = "0.8", features = ["ws"] }
tower-http   = { version = "0.6", features = ["cors"] }
serde_json   = "1"

# Telegram (Phase 4, optional)
[dependencies.teloxide]
version  = "0.13"
features = ["macros"]
optional = true

# OpenSearch (Phase 5, optional)
[dependencies.reqwest]
version  = "0.12"
features = ["json"]
optional = true

[features]
default    = []
telegram   = ["teloxide"]
opensearch = ["reqwest"]
```

**Примечание о tokio features:**
- `rt-multi-thread` (не просто `rt`) — нужен для `tokio::spawn` из нескольких потоков.
- `signal` — нужен для `tokio::signal::ctrl_c()` и SIGTERM в headless-режиме.

```toml
# tools/axiom-dashboard/Cargo.toml
[package]
name    = "axiom-dashboard"
version = "0.1.0"

[dependencies]
eframe      = { version = "0.29", default-features = false, features = ["glow"] }
egui_plot   = "0.29"
tungstenite = { version = "0.24", features = ["native-tls"] }
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
```

---

## 7. Конфигурация

```yaml
# В axiom-cli.yaml
adapters:
  tick_hz: 100    # Частота главного цикла в Hz (1-1000)

  cli:
    enabled: true
    detail_level: 1   # 0=minimal, 1=normal, 2=verbose

  websocket:
    enabled: true
    host: "0.0.0.0"
    port: 8080
    tick_broadcast_interval: 10    # каждые N тиков → ServerMessage::Tick
    state_broadcast_interval: 100  # каждые N тиков → ServerMessage::State
    max_connections: 10

  rest:
    enabled: true
    # REST использует тот же порт и тот же axum router что WebSocket

  telegram:
    enabled: false
    bot_token: "${TELEGRAM_BOT_TOKEN}"
    allowed_users: []   # пустой список = нет ограничений (небезопасно в prod!)

  opensearch:
    enabled: false
    url: "http://localhost:9200"
    index_prefix: "axiom-"
    index_traces: true
    index_events: true
    index_skills: true
    batch_size: 100
    flush_interval_ms: 5000
```

Структура в коде:

```rust
#[derive(Deserialize, Clone)]
pub struct AdaptersConfig {
    pub tick_hz:   u32,
    pub cli:       CliAdapterConfig,
    pub websocket: WebSocketConfig,
    pub rest:      RestConfig,
    pub telegram:  TelegramConfig,
    pub opensearch: OpenSearchConfig,
}

#[derive(Deserialize, Clone)]
pub struct WebSocketConfig {
    pub enabled:                bool,
    pub host:                   String,
    pub port:                   u16,
    pub tick_broadcast_interval: u32,
    pub state_broadcast_interval: u32,
    pub max_connections:        u32,
}
```

---

## 8. Режимы запуска и Graceful Shutdown

```bash
# CLI only (по умолчанию)
cargo run --bin axiom-cli

# CLI + WebSocket + REST
cargo run --bin axiom-cli -- --server

# CLI + WebSocket + REST, порт 3000
cargo run --bin axiom-cli -- --server --port 3000

# Headless (WebSocket + REST, без stdin)
cargo run --bin axiom-cli -- --server --no-cli

# + Telegram
cargo run --bin axiom-cli --features telegram -- --server --telegram

# + OpenSearch
cargo run --bin axiom-cli --features opensearch -- --server

# Dashboard (отдельный процесс)
cargo run -p axiom-dashboard
cargo run -p axiom-dashboard -- --url ws://192.168.1.100:8080/ws
```

### 8.1 Graceful Shutdown

В `--no-cli` режиме `:quit` недоступен. Завершение — через сигнал ОС.

```rust
// В main() или точке запуска
let shutdown_signal = async {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint  = signal(SignalKind::interrupt()).unwrap();
        tokio::select! {
            _ = sigterm.recv() => {},
            _ = sigint.recv()  => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.ok();
    }
};

tokio::select! {
    _ = tick_loop_handle => {},
    _ = shutdown_signal  => {
        // Сигнализируем tick loop завершиться через специальный AdapterCommand
        let _ = command_tx.send(AdapterCommand::shutdown()).await;
        // Дать tick loop время автосохраниться (таймаут 5s)
        tokio::time::timeout(Duration::from_secs(5), tick_loop_handle).await.ok();
    }
}
```

**AdapterCommand::shutdown()** — конструктор для команды завершения:
```rust
impl AdapterCommand {
    pub fn shutdown() -> Self {
        Self {
            id: "shutdown".to_string(),
            source: AdapterSource::Cli,
            payload: AdapterPayload::MetaMutate { cmd: ":quit".to_string() },
        }
    }
}
```

---

## 9. REST API

REST использует тот же axum-router что WebSocket (один порт, разные пути).

```
GET  /api/status          → BroadcastSnapshot (из Arc<RwLock<...>>, не блокирует Engine)
GET  /api/domain/:id      → запрашивает через command_tx → ответ через response_tx
POST /api/inject          → { "text": "..." } → command_tx
POST /api/command         → { "cmd": ":status" } → command_tx
```

**Проблема: как REST GET /domain/:id получает ответ?**

`BroadcastSnapshot` не содержит деталей доменов. Нужен механизм request-response.
Варианты:

**Вариант A: One-shot channel в команде**
```rust
pub enum AdapterPayload {
    DomainSnapshot {
        domain_id: u16,
        // Канал для ответа — tick loop отправляет туда, REST ждёт
        response_tx: oneshot::Sender<DomainDetailSnapshot>,
    },
    // ...
}
```
Минус: `oneshot::Sender` не `Clone` — `AdapterCommand` перестаёт быть `Clone`.

**Вариант B: Polling через snapshot + invalidation**
Tick loop хранит `HashMap<u16, Arc<RwLock<Option<DomainDetailSnapshot>>>>`.
REST POST /api/domain/:id/refresh → tick loop обновляет snapshot домена.
REST GET /api/domain/:id → читает из HashMap.
Минус: eventual consistency, REST не знает когда данные свежие.

**Вариант C: Response через broadcast с фильтрацией по command_id (рекомендуется)**
REST отправляет DomainSnapshot через command_tx с уникальным command_id,
подписывается на broadcast_rx, ждёт `ServerMessage::DomainDetail` с matching command_id.
```rust
// В REST handler:
let id = Uuid::new_v4().to_string();
command_tx.send(AdapterCommand { id: id.clone(), payload: AdapterPayload::DomainSnapshot { domain_id } }).await?;
let mut rx = broadcast_tx.subscribe();
let timeout = tokio::time::timeout(Duration::from_secs(5), async move {
    loop {
        match rx.recv().await? {
            ServerMessage::DomainDetail(snap) if snap.domain_id == domain_id => return Ok(snap),
            ServerMessage::CommandResult { command_id, .. } if command_id == id => {
                // Команда обработана, но ответ другой — ошибка
                return Err(anyhow!("unexpected response"));
            }
            _ => continue,
        }
    }
});
```
Плюс: нет лишних структур, использует уже существующий broadcast.
Минус: при высокой нагрузке broadcast может переполниться (1024 буфер).

**Решение по умолчанию:** Вариант C для Phase 2. При необходимости можно перейти на A/B.

---

## 10. Порядок реализации

### Phase 0: Подготовка (обязательно перед остальным)

**0.1. Рефактор handle_meta_command**

1. Создать `crates/axiom-agent/src/meta_commands.rs`
2. Перенести все read-only команды в `handle_meta_read(cmd, engine, ...) -> String`
3. Перенести мутирующие команды в `handle_meta_mutate(cmd, engine, ...) -> MetaMutateResult`
4. `CliChannel::handle_meta_command` превращается в тонкую обёртку вокруг этих функций
5. Все тесты `CliChannel` должны пройти без изменений

**0.2. Convenience-методы на AxiomEngine**

В `crates/axiom-runtime/src/engine.rs`:
- `trace_count(&self) -> usize`
- `tension_count(&self) -> usize`
- `last_matched(&self) -> u32`
- `snapshot_for_broadcast(&self) -> BroadcastSnapshot`
- `domain_detail_snapshot(&self, domain_id: u16) -> Option<DomainDetailSnapshot>`
- `domain_summaries(&self) -> Vec<DomainSummary>` (приватный)

Snapshot-типы (`BroadcastSnapshot`, `DomainSummary`, `DomainDetailSnapshot`,
`TokenSnapshot`, `ConnectionSnapshot`) добавить в `crates/axiom-runtime/src/broadcast.rs`.

**0.3. AdapterCommand и связанные типы**

В `crates/axiom-agent/src/adapter_command.rs`:
- `AdapterCommand`, `AdapterSource`, `AdapterPayload`
- `CommandResponse`, `MetaMutateResult`, `MetaAction`
- `AdaptersConfig` (с JSON-схемой через schemars)

**0.4. Тесты Phase 0**

```rust
#[test]
fn test_handle_meta_read_status_returns_nonempty() {
    let engine = AxiomEngine::new();
    let output = handle_meta_read(":status", &engine, None, &TextPerceptor::new(), &CliConfig::default());
    assert!(!output.is_empty());
    assert!(output.contains("tick_count"));
}

#[test]
fn test_snapshot_for_broadcast_matches_engine_state() {
    let engine = AxiomEngine::new();
    let snap = engine.snapshot_for_broadcast();
    assert_eq!(snap.trace_count, engine.trace_count());
    assert_eq!(snap.domain_summaries.len(), 11);
}

#[test]
fn test_meta_mutate_quit_returns_quit_action() {
    let mut engine = AxiomEngine::new();
    let mut saver = AutoSaver::disabled();
    let result = handle_meta_mutate(":quit", &mut engine, &mut saver, &CliConfig::default());
    assert!(matches!(result.action, MetaAction::Quit));
}
```

### Phase 1: WebSocket

1. `crates/axiom-agent/src/ws/` — модуль WebSocket
2. `protocol.rs` — `ClientMessage`, `ServerMessage` с `#[derive(Serialize, Deserialize)]`
3. `handler.rs` — axum WebSocket handler: принимает `ClientMessage`, отправляет в `command_tx`
4. `server.rs` — запуск axum, регистрация routes (`/ws`, `/api/*`)
5. Рефактор `CliChannel::run()` → tick loop выносится в `tick_loop()` из Section 4
6. `--server` флаг: запустить axum параллельно с tick loop
7. Тесты: подключение, inject, subscribe, tick broadcast

**Важно о subscribe/unsubscribe:** Каждое WebSocket-соединение имеет своё состояние
(на какие каналы подписано). Это состояние хранится *вне* tick loop, в handler.
Subscribe/Unsubscribe — это фильтрация broadcast_rx в per-connection горутине, не команда Engine.

### Phase 2: REST

1. `crates/axiom-agent/src/rest/` — модуль REST
2. Routes: `GET /api/status`, `GET /api/domain/:id`, `POST /api/inject`, `POST /api/command`
3. `GET /api/status` → `snapshot.read().await` — нет lock на Engine
4. `GET /api/domain/:id` → Вариант C (broadcast с command_id)
5. CORS через tower-http: разрешить localhost для dashboard в dev-режиме
6. Тесты: curl /api/status, POST /api/inject, проверка что Engine не лочится при GET

### Phase 3: egui Dashboard (отдельный crate)

1. `tools/axiom-dashboard/` — отдельный crate, не зависит от axiom-*
2. Единственная зависимость на axiom: JSON-протокол (скопировать структуры или использовать общий crate)
3. WebSocket клиент в отдельном потоке (не tokio — eframe синхронный)
4. State: хранит последний `BroadcastSnapshot` + `HashMap<u16, DomainDetailSnapshot>`
5. Panels: Status, Traces, Space View (2D проекция XY), Input

**Space View:** каждый токен = точка на плоскости (x, y). Цвет = domain_id.
Якорные токены (is_anchor=true) — отмечены крестом, не кружком.
Якоря всегда на переднем плане (z-order).

### Phase 4: Telegram (feature-gated)

1. `crates/axiom-agent/src/telegram/` — feature "telegram"
2. Таблица `pending: HashMap<String, i64>` — command_id → chat_id
3. При получении ответа из broadcast_rx по command_id — отправить пользователю
4. Команды: `/start`, `/status`, текстовый ввод → inject
5. `allowed_users: Vec<i64>` — whitelist chat_id (пустой = всем, небезопасно)
6. Тесты: mock bot (teloxide имеет `MockBot` в тестах)

### Phase 5: OpenSearch (feature-gated)

1. `crates/axiom-agent/src/opensearch/` — feature "opensearch"
2. `OpenSearchWriter` — подписчик на `broadcast_rx`
3. Буфер `Vec<serde_json::Value>` накапливает документы
4. Flush: `let docs = std::mem::take(&mut self.buffer); let client = self.client.clone();`
   затем `tokio::spawn(async move { send_bulk(client, docs).await })` — owned, не borrow
5. `@timestamp` — `chrono::Utc::now().to_rfc3339()` в адаптере, не в ядре (COM инвариант)
6. Index templates при старте (опционально)
7. `docker-compose.opensearch.yaml`

---

## 11. Подводные камни

### 11.1 broadcast::Receiver и lagging

`tokio::sync::broadcast` не блокирует sender при медленных receiver.
Если WebSocket-клиент медленный, его `broadcast::Receiver` начнёт лагировать.
При отставании > 1024 сообщений — `RecvError::Lagged(n)` — клиент пропускает N сообщений.

**Решение:** Обрабатывать `RecvError::Lagged` в per-connection goroutine:
```rust
loop {
    match rx.recv().await {
        Ok(msg) => { /* отправить */ }
        Err(broadcast::error::RecvError::Lagged(n)) => {
            // Клиент отстал, пропустили N сообщений — можно отправить warning
            let _ = ws_sender.send(Message::Text(
                serde_json::to_string(&ServerMessage::Error {
                    command_id: None,
                    message: format!("lagged by {} messages", n),
                }).unwrap()
            )).await;
            continue; // продолжаем, не разрываем соединение
        }
        Err(broadcast::error::RecvError::Closed) => break,
    }
}
```

### 11.2 :watch после рефактора

`:watch` включает/выключает `HashSet<String>` в CliChannel.
После рефактора эта логика остаётся в CliChannel (для CLI-режима).
Для WebSocket-клиентов watch-подписки — это просто `Subscribe { channels }` — фильтрация
в per-connection goroutine. Не нужно тащить watch_fields в tick loop.

### 11.3 process_and_observe требует &mut

`engine.process_and_observe(&ucl_cmd)` принимает `&mut self`.
В tick loop это нормально — Engine всегда `&mut`.
Но в тестах и handle_meta_read Engine должен быть `&` — конфликт.

`process_and_observe` — это диагностическая обёртка над `process_command`.
`handle_meta_read` не вызывает inject, поэтому здесь проблемы нет.
Проблема возникнет только если кто-то попытается вызвать `process_and_observe`
из read-only контекста — компилятор это запретит.

### 11.4 AutoSaver в tick loop

`AutoSaver` должен быть в tick loop (он имеет `&mut engine` для force_save).
Нельзя держать его снаружи tick loop и передавать по &mut в команды.

При Phase 0 AutoSaver перемещается в tick_loop как owned value (не в CliChannel).
`CliChannel` остаётся только как точка входа для CLI-команд.

### 11.5 Порядок инициализации

Текущий порядок в `CliChannel::new()`:
1. Применить `tick_schedule` из конфига к engine
2. Инициализировать ConfigWatcher
3. Загрузить якоря → inject_anchor_tokens → создать TextPerceptor

Этот порядок должен сохраниться в новом `tick_loop_init()`:
- Если anchor загрузка падает — fallback на FNV-1a (инвариант 9.8)
- inject_anchor_tokens до первого тика (якоря нужны как ориентиры с самого начала)

### 11.6 Разрастание broadcast_tx

При Phase 1 в tick loop появятся send(broadcast_tx, ...) в трёх местах:
тик, state, результат команды. Убедиться что все `broadcast_tx.send(...)` возвращают
`Result` который мы явно игнорируем через `let _ = ...` — не через `.unwrap()`.
Если получателей нет — `send` возвращает `Err(SendError)` — это нормально, не паника.

### 11.7 DomainState видимость

`DomainState.connections` — нужно проверить что это `pub` поле.
В `snapshot()` используется `state.connections.clone()`, значит поле доступно из engine.rs.
Для `ConnectionSnapshot::from` нужно будет добавить impl в `broadcast.rs`
или в `axiom-core` — с минимальным импортом, без зависимости на serde в axiom-core.

---

## 12. Инварианты

1. **Ядро изолировано.** `axiom-runtime` не импортирует axum, serde_json, teloxide, reqwest.
2. **UCL — единственный способ мутировать.** JSON → адаптер → `UclCommand` (64B) → Engine.
3. **COM не нарушен.** `@timestamp` генерируется адаптером (wall-clock). Ядро знает только `event_id`.
4. **Один Engine, один writer.** Tick loop — единственный кто вызывает `process_command` и `process_and_observe`.
5. **Snapshot для чтения.** REST GET / dashboard → `Arc<RwLock<BroadcastSnapshot>>`, обновляется tick loop. Не лочит Engine.
6. **Impulses не дублируются.** Tick loop вызывает `engine.process_command(&tick_cmd)` — импульсы обрабатываются внутри через `TickSchedule`. Никакого дополнительного drain.
7. **Graceful shutdown.** `:quit` / SIGTERM → автосохранение → выход из tick loop → tokio runtime завершается.

---

## 13. История изменений

- **V3.1**: `CommandResponse` упрощён до `Message(ServerMessage) | Quit | None` — process_adapter_command строит ServerMessage сам, tick loop не знает деталей протокола. `matched` → `last_matched` с явной документацией семантики (last-seen, не sum).
- **V3.0**: Верификация всех путей по реальному коду. Убран несуществующий `drain_pending_impulses`. Исправлен порядок clone/write в snapshot. Рефактор handle_meta_command детализирован с сигнатурами и MetaAction. Добавлен раздел подводных камней (11). Добавлен graceful shutdown. tick_hz в AdaptersConfig. REST request-response паттерн (Section 9).
- **V2.0**: Engine access через snapshot + command channel. handle_meta_command → String. Convenience-методы. OpenSearch flush fix. Telegram pending table. egui = отдельный crate.
- **V1.2**: egui вместо browser, OpenSearch добавлен.
- **V1.0**: Первая версия.
