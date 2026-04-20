# External Adapters Guide V1.0

**Версия:** 1.1  
**Дата:** 2026-04-20  
**Спецификация:** [docs/spec/External_Adapters_V3_0.md](../spec/External_Adapters_V3_0.md)  
**План:** [docs/spec/External_Adapters_Plan_V1_0.md](../spec/External_Adapters_Plan_V1_0.md)

---

## Что такое External Adapters

Axiom — когнитивная система с единственным состоянием внутри. До сих пор единственный
способ взаимодействия — CLI (`axiom-cli`, stdin/stdout). External Adapters открывают
систему наружу не нарушая её инварианты.

**Ключевые инварианты, которые адаптеры не нарушают:**

1. **AxiomEngine мутирует только один поток** — tick loop. Адаптеры — только через каналы.
2. **UCL — единственный способ изменить состояние** — JSON в ядро не попадает.
3. **COM не нарушен** — стена времени (`@timestamp` в OpenSearch) генерируется адаптером, не ядром. Ядро знает только `event_id`.
4. **Якоря неизменяемы** — инжектированные при старте `STATE_LOCKED` токены не затрагиваются адаптерами.

---

## Архитектура одним взглядом

```
                     ┌─────────────────────────────────────────────┐
  CLI stdin     ─→   │                                             │
  WebSocket     ─→   │   command_tx   mpsc::channel<AdapterCommand>│
  REST POST     ─→   │                                             │
  Telegram      ─→   │                                             │
                     └──────────────────┬──────────────────────────┘
                                        │ единственный reader
                                        ↓
                              ┌──────────────────┐
                              │    tick_loop()   │ ← единственный writer AxiomEngine
                              │                  │
                              │  1. drain cmds   │
                              │  2. tick engine  │
                              │  3. broadcast    │
                              │  4. snapshot     │
                              └────────┬─────────┘
                                       │
                     ┌─────────────────┴───────────────────────────┐
  CLI stdout    ←─   │                                             │
  WebSocket     ←─   │   broadcast_tx  broadcast::channel          │
  REST GET      ←─   │   + Arc<RwLock<BroadcastSnapshot>>          │
  Telegram      ←─   │                                             │
  OpenSearch    ←─   │                                             │
                     └─────────────────────────────────────────────┘
```

---

## Компоненты

### command_tx — входной канал

```rust
mpsc::channel::<AdapterCommand>(256)
```

Все адаптеры пишут сюда. Tick loop читает отсюда (non-blocking `try_recv` каждый тик).
Буфер 256 — при переполнении sender получает `Err(TrySendError::Full)` и должен
либо отбросить команду, либо вернуть ошибку клиенту.

### broadcast_tx — выходной канал

```rust
broadcast::channel::<ServerMessage>(1024)
```

Tick loop пишет сюда. Все адаптеры читают.
`broadcast` в tokio — это MPSC наоборот: один sender, много receivers.
Каждый receiver получает *свою копию* каждого сообщения.

При отставании > 1024 сообщений receiver получает `RecvError::Lagged(n)`.
Это нормальная ситуация — WebSocket handler отправляет клиенту предупреждение и продолжает.

### snapshot — для read-only запросов

```rust
Arc<RwLock<BroadcastSnapshot>>
```

Обновляется tick loop каждые `state_broadcast_interval` тиков.
REST GET `/api/status` читает отсюда — **без блокировки Engine**.
Это eventual consistency: данные могут быть на N тиков позади.

---

## AdapterCommand — формат входящей команды

```rust
pub struct AdapterCommand {
    pub id:      String,         // UUID для корреляции ответа
    pub source:  AdapterSource,  // откуда пришла команда
    pub payload: AdapterPayload, // что делать
}
```

### AdapterSource

```rust
pub enum AdapterSource {
    Cli,
    WebSocket(u64),   // connection_id — уникален в рамках сессии
    Rest,
    Telegram(i64),    // chat_id Telegram
}
```

### AdapterPayload

| Вариант | Описание | Мутирует Engine |
|---------|----------|-----------------|
| `Inject { text }` | Текст → TextPerceptor → InjectToken UCL | да |
| `MetaRead { cmd }` | `:status`, `:traces`, `:domains`... → строка | нет |
| `MetaMutate { cmd }` | `:save`, `:load`, `:tick N`... | да |
| `DomainSnapshot { domain_id }` | детальный snapshot одного домена | нет |
| `Subscribe { channels }` | фильтрация broadcast (per-connection, не в tick loop) | нет |
| `Unsubscribe { channels }` | аналогично | нет |

---

## ServerMessage — формат исходящего сообщения

```rust
#[serde(tag = "type")]
pub enum ServerMessage {
    Result { command_id, path, domain_id, coherence, traces_matched, position, shell, event_id, ... },
    Tick   { tick_count, traces, tension, matched },
    State  { tick_count, snapshot: BroadcastSnapshot },
    CommandResult { command_id, output: String },
    DomainDetail(DomainDetailSnapshot),
    Error  { command_id, message },
}
```

### Когда что приходит

| Сообщение | Когда |
|-----------|-------|
| `Result` | После каждого `Inject` |
| `CommandResult` | После каждой meta-команды |
| `Tick` | Каждые `tick_broadcast_interval` тиков (по умолчанию 10) |
| `State` | Каждые `state_broadcast_interval` тиков (по умолчанию 100) |
| `DomainDetail` | В ответ на `DomainSnapshot` запрос |
| `Error` | При ошибке разбора команды или таймауте |

---

## handle_meta_read / handle_meta_mutate

После Phase 0B эти функции — главная точка расширения метакоманд.

```rust
// Добавить новую read-only команду:
// crates/axiom-agent/src/meta_commands.rs

pub fn handle_meta_read(cmd: &str, engine: &AxiomEngine, ...) -> String {
    match parts[0] {
        // ... существующие команды ...
        ":my_new_cmd" => {
            let mut out = String::new();
            writeln!(out, "  custom data: {}", engine.trace_count()).ok();
            out
        }
        _ => format!("  Unknown command. :help for list.\n"),
    }
}
```

**Правило:** read-команды принимают `&AxiomEngine` — никаких `&mut`.
Если для команды нужна мутация — это мутирующая команда, она идёт через `handle_meta_mutate`.

---

## WebSocket — протокол

### Подключение

```
ws://host:8080/ws
```

После подключения клиент сразу начинает получать `Tick` сообщения (если подписан на "ticks").
По умолчанию — ничего не приходит до первой подписки.

### Пример сессии

```json
// Клиент → сервер: подписаться на тики и результаты
{"type": "subscribe", "channels": ["ticks", "results"]}

// Клиент → сервер: отправить текст
{"type": "inject", "text": "анализ структуры данных"}

// Сервер → клиент: результат обработки
{"type": "result", "command_id": "abc123", "path": "slow_path",
 "domain_id": 106, "domain_name": "LOGIC", "coherence": 0.74,
 "traces_matched": 3, "position": [4200, -1800, 300], "event_id": 15042}

// Сервер → клиент: периодический тик (каждые 10 тиков)
{"type": "tick", "tick_count": 1540, "traces": 47, "tension": 2, "matched": 3}
```

### Channels

| Channel | Содержит |
|---------|----------|
| `"ticks"` | `ServerMessage::Tick` |
| `"state"` | `ServerMessage::State` (тяжелее, реже) |
| `"results"` | `ServerMessage::Result` за inject этого клиента |
| `"all_results"` | `ServerMessage::Result` за inject любого клиента |

---

## REST API

### GET /api/status

Возвращает `BroadcastSnapshot`. Данные из `Arc<RwLock<...>>` — Engine не блокируется.

```bash
curl http://localhost:8080/api/status
```

```json
{
  "tick_count": 15420,
  "com_next_id": 28841,
  "trace_count": 47,
  "tension_count": 2,
  "domain_summaries": [
    {"domain_id": 100, "name": "SUTRA",     "token_count": 12, "connection_count": 8},
    {"domain_id": 101, "name": "EXECUTION", "token_count": 3,  "connection_count": 1},
    ...
  ]
}
```

### POST /api/inject

```bash
curl -X POST http://localhost:8080/api/inject \
  -H "Content-Type: application/json" \
  -d '{"text": "анализ данных"}'
```

Возвращает `ServerMessage::Result`.

### POST /api/command

```bash
# Read-only команда
curl -X POST http://localhost:8080/api/command \
  -d '{"cmd": ":status", "type": "read"}'

# Мутирующая команда
curl -X POST http://localhost:8080/api/command \
  -d '{"cmd": ":save", "type": "mutate"}'
```

### GET /api/domain/:id

```bash
curl http://localhost:8080/api/domain/106
```

Возвращает `DomainDetailSnapshot` с полным списком токенов и связей.
Таймаут 5 секунд — если tick loop не успевает ответить, возвращает 408.

`DomainDetailSnapshot` содержит `tokens: Vec<TokenSnapshot>`. Поле `shell: [u8; 8]` —
точный семантический профиль, вычисленный через `axiom_shell::compute_shell` из связей
домена и его `SemanticContributionTable`. Каждый байт — вес одного из 8 семантических
слоёв (L1–L8), нормализованный к `[0, 255]`. Пустой shell (`[0;8]`) означает токен без связей.

### GET /api/domains

```bash
curl http://localhost:8080/api/domains
```

Возвращает массив `DomainSummary` из snapshot (без таймаута, мгновенно).

---

## egui Dashboard

Запуск (в отдельном терминале, при запущенном `--server`):

```bash
cargo run -p axiom-dashboard
# или с нестандартным адресом:
cargo run -p axiom-dashboard -- --url ws://192.168.1.100:8080/ws
```

### Панели

**Status** — tick_count, Hz, traces, tension. Обновляется при каждом `Tick` сообщении.

**Traces** — таблица топ-20 следов по weight (из `State` сообщений). Колонки: weight, position, age, hash.

**Space View** — 2D семантическое пространство (оси X, Y из `Token.position`).
- Кружок = обычный токен. Размер ~ `log(mass + 1)`. Цвет = domain_id.
- Крест = якорный токен (`is_anchor=true`). Всегда поверх, полупрозрачный.
- Клик → показать детали токена в боковой панели.
- Zoom/pan через колесо мыши и drag.

**Input** — поле ввода текста → POST /api/inject → отображение результата.

---

## Telegram

Требует feature flag при сборке:

```bash
cargo run --bin axiom-cli --features telegram -- --server --telegram
```

Конфигурация в `axiom-cli.yaml`:
```yaml
adapters:
  telegram:
    enabled: true
    bot_token: "${TELEGRAM_BOT_TOKEN}"  # из env
    allowed_users: [123456789]           # пустой = доступ всем (небезопасно!)
```

| Сообщение | Действие |
|-----------|----------|
| `/start` | Статус системы |
| `/status` | `:status` |
| любой текст | inject → ответ с результатом |
| `:traces`, `:domains`... | метакоманда read |
| `:save` | сохранение (только если chat_id в allowed_users) |

---

## OpenSearch

```bash
cargo run --bin axiom-cli --features opensearch -- --server
```

Конфигурация:
```yaml
adapters:
  opensearch:
    enabled: true
    url: "http://localhost:9200"
    index_prefix: "axiom-"
    index_traces: true
    batch_size: 100
    flush_interval_ms: 5000
```

Создаёт индексы:
- `axiom-traces-YYYY.MM.DD` — результаты inject с `@timestamp`, position, coherence
- `axiom-events-YYYY.MM.DD` — tick broadcast (tick_count, traces, tension)

`@timestamp` — wall-clock время адаптера (`chrono::Utc::now()`), не event_id ядра.

Запустить OpenSearch локально:
```bash
docker-compose -f docker-compose.opensearch.yaml up
```

---

## Запуск

```bash
# Только CLI (по умолчанию)
cargo run --bin axiom-cli

# CLI + WebSocket + REST (порт 8080)
cargo run --bin axiom-cli -- --server

# CLI + сервер на порту 3000
cargo run --bin axiom-cli -- --server --port 3000

# Headless (без stdin CLI)
cargo run --bin axiom-cli -- --server --no-cli

# Все адаптеры
cargo run --bin axiom-cli --features telegram,opensearch -- --server --telegram
```

При `--server` в консоли появится:

```
AXIOM — Cognitive Architecture
───────────────────────────────────────────────
tick_hz: 100 Hz  |  domains: 11  |  :help для команд
  mode: restored (tick=15420, traces=47, tension=2)
  anchors: 6 axes + 10 layer + 6 domain = 22 tokens injected
  WebSocket: ws://0.0.0.0:8080/ws
  REST API:  http://0.0.0.0:8080/api/

axiom>
```

---

## Graceful Shutdown

**С CLI (`--server` без `--no-cli`):** введи `:quit` в консоли.

**Headless (`--no-cli`):** `Ctrl+C` или `kill <pid>` (SIGTERM).
Система поймает сигнал → завершит tick loop → автосохранит состояние → выйдет.

Автосохранение при shutdown: только если `autosave` включён в конфиге.
Принудительное сохранение перед выходом независимо от autosave — конфигурируется флагом
`shutdown_save: true` в секции `adapters`.

---

## Конфигурация (axiom-cli.yaml)

```yaml
adapters:
  tick_hz: 100

  cli:
    enabled: true
    detail_level: 1    # 0=minimal, 1=normal, 2=verbose

  websocket:
    enabled: true
    host: "0.0.0.0"
    port: 8080
    tick_broadcast_interval: 10
    state_broadcast_interval: 100
    max_connections: 10

  rest:
    enabled: true     # использует тот же порт что websocket

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
    flush_interval_ms: 5000
```

---

## Добавление нового адаптера

Новый адаптер должен:

1. Получить `mpsc::Sender<AdapterCommand>` — клонировать из `command_tx`
2. Получить `broadcast::Receiver<ServerMessage>` — подписаться через `broadcast_tx.subscribe()`
3. Получить `Arc<RwLock<BroadcastSnapshot>>` — для read-only доступа
4. **Не** получать ссылку на `AxiomEngine`

Шаблон:

```rust
pub async fn run_my_adapter(
    command_tx: mpsc::Sender<AdapterCommand>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    snapshot: Arc<RwLock<BroadcastSnapshot>>,
    config: MyAdapterConfig,
) {
    let mut rx = broadcast_tx.subscribe();

    // Входящий поток (от внешнего источника)
    tokio::spawn(async move {
        loop {
            // читать из внешнего источника
            let text = read_from_external().await;
            let id = Uuid::new_v4().to_string();
            let _ = command_tx.send(AdapterCommand {
                id: id.clone(),
                source: AdapterSource::Cli,  // или новый вариант
                payload: AdapterPayload::Inject { text },
            }).await;
        }
    });

    // Исходящий поток (к внешнему получателю)
    loop {
        match rx.recv().await {
            Ok(ServerMessage::Result { command_id, .. }) => {
                // отправить ответ внешнему получателю
            }
            Ok(_) => continue,
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}
```

---

## FAQ

**Q: Могу ли я читать Engine напрямую из REST handler?**  
A: Нет. Engine передан в tick_loop по значению. Для чтения — `snapshot.read().await`.
Для деталей конкретного домена — `GET /api/domain/:id` через command_tx.

**Q: Почему broadcast buffer 1024, а command buffer 256?**  
A: Команд приходит меньше (человек или скрипт), broadcast рассылается всем клиентам.
1024 — запас на случай медленных WebSocket-клиентов. 256 — достаточно для burst ввода.

**Q: Что происходит с `broadcast_tx.send()` если нет ни одного подписчика?**  
A: Возвращает `Err(SendError)`. Мы игнорируем это через `let _ = broadcast_tx.send(...)`.
Это ожидаемо — в headless без клиентов broadcast идёт в никуда.

**Q: Как корреляция ответа с запросом (command_id)?**  
A: Каждый `AdapterCommand` содержит `id: String`. Tick loop копирует его в `ServerMessage`.
REST/Telegram ждут конкретный command_id в broadcast. WebSocket клиент коррелирует сам.

**Q: Почему egui dashboard — отдельный crate?**  
A: eframe не совместим с tokio (синхронный render loop). Отдельный crate позволяет иметь
другой runtime (std threads + std::sync::mpsc). И главное — dashboard не зависит от
axiom-core/axiom-runtime: только от JSON-протокола. Можно подключить к любому серверу.

**Q: Что если `:load` вызван из Telegram?**  
A: Tick loop получает `MetaMutate { cmd: ":load" }`, вызывает `handle_meta_mutate`,
получает `MetaAction::EngineReplaced`, перепривязывает perceptor. Telegram получит
`CommandResult` с результатом. Якоря остаются (инжектированы в engine при :load из файла).

**Q: Как TickSchedule взаимодействует с tick_hz в AdaptersConfig?**  
A: `tick_hz` — частота главного цикла (раз в X мс). `TickSchedule` — периодичность
задач *внутри* тика (каждые N тиков). Они независимы. При `tick_hz=100` и
`tension_check_interval=10` — tension проверяется 10 раз в секунду.
