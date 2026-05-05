# AXIOM Quick Start

---

## Prerequisites

- **OS:** Linux (Arch Linux recommended)
- **Rust:** 1.75+

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Installation

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom

cargo test --workspace   # 1174 тестов, 0 failures
cargo build --release    # production build
```

Полное руководство по установке: [Installation Guide.md](Installation%20Guide.md)

---

## Запуск CLI

```bash
cargo run --bin axiom-cli --release
```

При старте система:
1. Загружает якоря из `config/anchors/` (если есть) и инжектирует их в домены
2. Загружает состояние из `./axiom-data/` (если есть)

```bash
# Полезные флаги
cargo run --bin axiom-cli --release -- --verbose        # состояние после каждого ввода
cargo run --bin axiom-cli --release -- --adaptive       # адаптивная частота тиков
cargo run --bin axiom-cli --release -- --no-load        # чистый старт
cargo run --bin axiom-cli --release -- --detail max     # подробный вывод
cargo run --bin axiom-cli --release -- --hot-reload     # следить за config/axiom.yaml
```

---

## Работа в CLI

**Текстовый ввод** — любая строка без `:`:
```
axiom> порядок структуры

  [Direct] → EXECUTION | coh=0.75 matched=3 pos=(22000,1500,500)
```

Если в `config/anchors/` есть якоря — TextPerceptor определяет позицию через совпадение,
иначе — через FNV-1a хэш.

---

## Ключевые команды

**Состояние:**
```
:status          — tick_count, uptime, Hz, memory summary
:memory          — токены, связи, traces, tension, skills
:domains         — список доменов с числом токенов
:perf            — avg/peak ns/тик, actual Hz, budget
```

**Якоря:**
```
:anchors              — загруженные якоря (axes/layers/domains)
:anchors axes         — 6 осевых якорей (X/Y/Z полюса)
:anchors layer L5     — якоря слоя L5
:anchors domain D1    — якоря домена EXECUTION
:match порядок        — какие якоря сработают для слова
```

**Когнитивный слой:**
```
:traces          — experience traces (top-20 по weight)
:trace <N>       — детали одного trace
:tension         — активные tension traces
:depth           — Cognitive Depth: max_passes, min_coherence
:arbiter         — пороги Arbiter по доменам + Reflector
:guardian        — статистика GUARDIAN
:dream           — DREAM-цикл: кристаллизуемые паттерны
:reflector       — per-domain accuracy REFLECTOR
:impulses        — очередь goal/curiosity/tension impulses
```

**Диагностика:**
```
:frontier        — Causal Frontier по доменам
:connections     — связи между токенами
:events [N]      — последние N COM-событий
:domain <id>     — полные детали одного домена
:multipass       — статистика multi-pass обработки
```

**Управление:**
```
:detail [off|min|mid|max]     — уровень детализации
:verbose [on|off]             — verbose после каждого ввода
:watch <traces|tension|tps>   — следить за полем в реальном времени
:tick [N]                     — прокрутить N тиков вручную
:tickrate                     — адаптивная частота (Sentinel)
```

**Persistence:**
```
:save [path]              — сохранить (bincode, default: ./axiom-data/)
:load [path]              — загрузить
:autosave on 1000         — автосохранение каждые 1000 тиков
:export traces [path]     — экспорт знаний в bincode
:import traces [path]     — импорт с GUARDIAN-валидацией (weight×0.7)
```

**Схемы и прочее:**
```
:schema [axiom|domain|heartbeat|cli]  — JSON-схема конфига
:config          — текущая конфигурация CLI
:schedule        — интервалы периодических задач
:help [command]  — полный список или детали команды
:quit / :q       — выход (с автосохранением если включено)
```

---

## WebSocket-сервер (Phase 1)

Запуск с WebSocket-сервером:

```bash
cargo run --bin axiom-cli --release -- --server
# [ws] WebSocket server on ws://127.0.0.1:8765/ws

cargo run --bin axiom-cli --release -- --server --port 9000
```

Протокол — JSON сообщения:

```json
// Подписка на события
{"type":"subscribe","channels":["ticks","state"]}

// Текстовый ввод
{"type":"inject","text":"порядок структуры"}

// Мета-команда
{"type":"read_command","cmd":":status"}
{"type":"mutate_command","cmd":":save"}

// Запрос деталей домена
{"type":"domain_snapshot","domain_id":100}
```

Ответы сервера (`ServerMessage`):
```json
{"type":"tick","tick_count":100,"traces":5,"tension":1,"last_matched":3}
{"type":"result","command_id":"1","domain_name":"SUTRA","coherence":0.85,...}
{"type":"command_result","command_id":"2","output":"  ══ Engine Status ..."}
{"type":"state","tick_count":100,"snapshot":{...}}
{"type":"domain_detail",...}
{"type":"error","message":"..."}
```

Тесты интеграции: `cargo test -p axiom-agent --test ws_tests`

---

## REST API (Phase 2)

Работает на том же порту что и WebSocket:

```bash
cargo run --bin axiom-cli --release -- --server --port 8765
```

Endpoints:

```bash
# Текстовый ввод
POST http://localhost:8765/inject
Content-Type: application/json
{"text": "порядок структуры"}

# Мета-команды
GET http://localhost:8765/status
GET http://localhost:8765/domains
GET http://localhost:8765/traces

# Детали домена
GET http://localhost:8765/domain-detail/100
```

Ответы — те же JSON-структуры что и у WebSocket (`ServerMessage`).
Timeout ожидания ответа — 5 секунд.

Тесты: `cargo test -p axiom-agent --test rest_tests`

---

## Workstation (V1.0)

Десктопный рабочий стол оператора на iced 0.13. Подключается к движку через выделенный WebSocket-сервер (`axiom-broadcasting`), не через axiom-cli.

> **Статус:** `axiom-broadcasting` ещё не подключён к тик-циклу движка (BRD-TD-07 — откладывается до `axiom-node`). Workstation компилируется и запускается, но без живого сервера будет ждать подключения на Welcome-экране.

```bash
cargo run -p axiom-workstation
```

По умолчанию подключается к `127.0.0.1:9876`. Адрес меняется в Configuration → Connection.

**8 вкладок:**
- **System Map** — мандала ASHTI с пульсацией и анимацией состояния
- **Live Field** — 3D-визуализация токенов, орбитальная камера (drag + scroll)
- **Conversation** — текстовый ввод в Engine с историей и domain selector
- **Patterns** — sparklines активности слоёв L1-L8 + лента событий
- **Dream State** — состояние цикла сна, fatigue, force sleep / wake up
- **Configuration** — schema-driven редактор конфигурации движка
- **Files** — импорт данных через адаптеры (progress + история)
- **Benchmarks** — запуск бенчмарков и история результатов

**Keyboard shortcuts:** `Ctrl+1–8` — переключение вкладок, `Ctrl+S` — применить конфиг, `Ctrl+Z` — сбросить изменения.

---

## egui Dashboard (Phase 3)

Standalone desktop GUI — подключается к запущенному axiom-cli:

```bash
# Сначала запустить сервер
cargo run --bin axiom-cli --release -- --server

# Затем в другом терминале
cargo run -p axiom-dashboard
# или с другим адресом
cargo run -p axiom-dashboard -- ws://127.0.0.1:9000/ws
```

Панели:
- **Status** — tick_count, traces, tension, last_matched, uptime
- **Space View** — scatter-plot токенов по доменам (загружается через `DomainDetail`)
- **Domain List** — список доменов с числом токенов
- **Input** — текстовый ввод и кнопки `:status` / `:domains` / `:traces`

---

## Telegram-адаптер (Phase 4)

Требует feature flag `telegram`. Токен получить у [@BotFather](https://t.me/BotFather).

```bash
cargo run --bin axiom-cli --release --features telegram -- \
  --telegram-token YOUR_BOT_TOKEN

# С ограничением по user_id (можно повторять)
cargo run --bin axiom-cli --release --features telegram -- \
  --telegram-token YOUR_BOT_TOKEN \
  --telegram-allow 123456789 \
  --telegram-allow 987654321
```

Команды в Telegram:
```
/start          — приветствие + статус
/status         — :status
/domains        — :domains
/traces         — :traces
любой текст     — inject в engine
:status         — мета-команда (read)
:save           — мета-команда (mutate)
```

Build-проверка без запуска: `cargo build --features telegram`

---

## OpenSearch-адаптер (Phase 5)

Требует feature flag `opensearch`. Индексирует результаты инферов и тик-пульсы.

```bash
cargo run --bin axiom-cli --release --features opensearch -- \
  --server \
  --opensearch-url http://localhost:9200

# С кастомным индексом и тик-событиями каждые 100 тиков
cargo run --bin axiom-cli --release --features opensearch -- \
  --opensearch-url http://localhost:9200 \
  --opensearch-index my-axiom \
  --opensearch-tick 100
```

Документы в индексе:

```json
// Результат инфера
{
  "@timestamp": "2026-04-19T12:00:00.000Z",
  "type": "result",
  "command_id": "42",
  "domain_name": "SUTRA",
  "coherence": 0.85,
  "traces_matched": 3,
  "position": [1, 2, 3]
}

// Тик-пульс (при --opensearch-tick N)
{
  "@timestamp": "...",
  "type": "tick",
  "tick_count": 100,
  "traces": 15,
  "tension": 2
}
```

Build-проверка: `cargo build --features opensearch`

---

## Все фичи одновременно

```bash
cargo run --bin axiom-cli --release \
  --features telegram,opensearch -- \
  --server \
  --port 8765 \
  --telegram-token YOUR_TOKEN \
  --opensearch-url http://localhost:9200 \
  --opensearch-tick 100 \
  --adaptive \
  --hot-reload
```

---

## Rust API

```rust
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

let mut engine = AxiomEngine::new();
let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
engine.process_command(&cmd);
let events = engine.drain_events();
```

С якорным позиционированием:

```rust
use axiom_config::AnchorSet;
use axiom_agent::perceptors::text::TextPerceptor;
use std::sync::Arc;

let anchors = Arc::new(AnchorSet::load_or_empty(std::path::Path::new("config")));
engine.inject_anchor_tokens(&anchors);
let perceptor = TextPerceptor::with_anchors(anchors);
let cmd = perceptor.perceive("порядок");
engine.process_and_observe(&cmd);
```

С External Adapters (tick_loop):

```rust
use axiom_agent::tick_loop::tick_loop;
use axiom_agent::adapters_config::AdaptersConfig;
use axiom_agent::channels::cli::CliConfig;
use tokio::sync::{broadcast, mpsc};

let (cmd_tx, cmd_rx) = mpsc::channel(256);
let (bcast_tx, _)    = broadcast::channel(1024);
let snapshot = Arc::new(RwLock::new(BroadcastSnapshot::default()));
let config   = AdaptersConfig::from_cli_config(&CliConfig::default());

tokio::spawn(tick_loop(engine, cmd_rx, bcast_tx, snapshot, saver, None, config, None));
```

Подробнее: [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## Benchmarks

```bash
cargo bench -p axiom-bench
```

Результаты: [docs/bench/RESULTS.md](docs/bench/RESULTS.md)

---

## Common Issues

**Build failure:**
```bash
cargo clean && cargo build --release
```

**Сброс состояния:**
```bash
rm -rf axiom-data/
cargo run --bin axiom-cli --release -- --no-load
```

**Якоря не загружаются:**
Убедитесь что директория `config/anchors/` существует и содержит `axes.yaml`.
Система работает без якорей — FNV-1a fallback активен автоматически.

**WebSocket не подключается:**
Проверьте что axiom-cli запущен с флагом `--server`.
Dashboard по умолчанию подключается к `ws://127.0.0.1:8765/ws`.

**Telegram адаптер не компилируется:**
Убедитесь что передан флаг `--features telegram` при сборке.

**Запустить конкретный тест:**
```bash
cargo test -p axiom-runtime test_inject_anchor_tokens_axes
cargo test -p axiom-agent --test ws_tests
cargo test -p axiom-agent --test rest_tests
```
