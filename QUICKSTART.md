# AXIOM Quick Start

---

## Prerequisites

- **OS:** Linux (Arch Linux recommended)
- **Rust:** 1.75+

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- **Node.js:** 20+ (для Workstation V2)

```bash
# Arch Linux
pacman -S nodejs npm
```

---

## Installation

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom

cargo test --workspace   # 1592 тестов, 0 failures
cargo build --release    # production build (~7 мин первый раз)
```

---

## Интерфейсы

AXIOM имеет несколько независимых интерфейсов:

| Интерфейс | Бинарник / Команда | Описание |
|-----------|-------------------|----------|
| **Workstation V2** | `axiom-node` + `axiom-web` | React SPA — основной оперативный интерфейс |
| **CLI** | `axiom-cli` | Интерактивная командная строка |
| **WebSocket** | `axiom-cli --server` | JSON WebSocket сервер (Phase 1) |
| **REST API** | `axiom-cli --server` | REST поверх того же порта (Phase 2) |
| **egui Dashboard** | `axiom-dashboard` | Desktop GUI через WS (Phase 3) |
| **Telegram** | `axiom-cli --features telegram` | Telegram-бот (Phase 4) |
| **OpenSearch** | `axiom-cli --features opensearch` | Индексация событий (Phase 5) |

---

## Где что живёт

После `just run` всё доступно по этим адресам:

| Адрес | Что |
|-------|-----|
| `http://localhost:8080` | Workstation V2 — основной UI |
| `http://localhost:8080/api/status` | JSON статус движка (tick + dream_phase) |
| `ws://localhost:8080/api/ws` | WebSocket JSON поток (снапшоты + события) |
| `http://localhost:5173` | Workstation V2 dev-режим (только `just dev`) |

---

## Workstation V2 (рекомендуется)

### Быстрый запуск

```bash
just run          # production: axiom-node раздаёт dist/ на :8080
just dev          # dev: axiom-node :8080 + npm run dev :5173 (hot reload)
just run-build    # принудительная пересборка + запуск
```

Или напрямую через `run.sh`:

```bash
./run.sh           # production
./run.sh --dev     # dev
./run.sh --build   # пересборка + запуск
```

### Вручную (два терминала)

#### Терминал 1 — axiom-node

```bash
cargo run -p axiom-node --release
# → "http server on 127.0.0.1:8080"
```

#### Терминал 2 — Workstation (dev-режим)

```bash
cd tools/axiom-web
npm install
npm run dev
# → http://localhost:5173
```

### Production (один терминал)

```bash
cd tools/axiom-web && npm install && npm run build && cd ../..
cargo run -p axiom-node --release
# axiom-node раздаёт dist/ на http://127.0.0.1:8080
```

Полный гайд по UI: [docs/guides/Workstation_V2_Guide.md](docs/guides/Workstation_V2_Guide.md)

---

## axiom-tray (системный трей)

Показывает статус `axiom-node` в трее GNOME/KDE: тик + DREAM-фаза. Позволяет открыть Workstation в браузере, запустить или остановить `axiom-node`.

**Требования (GNOME):** расширение `appindicatorsupport@rgcjonas.gmail.com` должно быть включено.

```bash
# Сборка
cargo build -p axiom-tray --release

# Запуск (из корня проекта — иначе не найдёт target/release/axiom-node)
./target/release/axiom-tray

# Кастомные настройки
AXIOM_PORT=8080 AXIOM_BIN=/path/to/axiom-node ./target/release/axiom-tray
```

Меню трея:
- **Статус** — `● tick 12345 · dreaming ✦` (обновляется каждые 2 с)
- **Open Workstation** — открывает `http://127.0.0.1:8080` в браузере
- **Start / Stop axiom-node** — запуск или остановка процесса
- **Quit** — выход из axiom-tray

---

---

## HTTP API (axiom-node)

```bash
# Текстовый ввод в Engine
POST http://localhost:8080/api/text/submit
Content-Type: application/json
{"text": "порядок структуры"}

# Advisory — подтвердить / отклонить
POST http://localhost:8080/api/advisory/confirm/{id}
POST http://localhost:8080/api/advisory/reject/{id}

# Статус движка (JSON: tick + dream_phase)
GET http://localhost:8080/api/status

# WebSocket (снапшот при подключении + события)
ws://localhost:8080/api/ws
```

---

## CLI

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
{"type":"subscribe","channels":["ticks","state"]}
{"type":"inject","text":"порядок структуры"}
{"type":"read_command","cmd":":status"}
{"type":"mutate_command","cmd":":save"}
{"type":"domain_snapshot","domain_id":100}
```

Ответы сервера:
```json
{"type":"tick","tick_count":100,"traces":5,"tension":1,"last_matched":3}
{"type":"result","command_id":"1","domain_name":"SUTRA","coherence":0.85,...}
{"type":"command_result","command_id":"2","output":"  ══ Engine Status ..."}
{"type":"state","tick_count":100,"snapshot":{...}}
{"type":"error","message":"..."}
```

Тесты интеграции: `cargo test -p axiom-agent --test ws_tests`

---

## REST API (Phase 2)

```bash
cargo run --bin axiom-cli --release -- --server --port 8765
```

Endpoints:

```bash
POST http://localhost:8765/inject          # текстовый ввод
GET  http://localhost:8765/status
GET  http://localhost:8765/domains
GET  http://localhost:8765/traces
GET  http://localhost:8765/domain-detail/100
```

Тесты: `cargo test -p axiom-agent --test rest_tests`

---

## egui Dashboard (Phase 3)

```bash
# Сначала запустить сервер
cargo run --bin axiom-cli --release -- --server

# Затем в другом терминале
cargo run -p axiom-dashboard
# или с другим адресом
cargo run -p axiom-dashboard -- ws://127.0.0.1:9000/ws
```

Панели: Status, Space View, Domain List, Input.

---

## Telegram-адаптер (Phase 4)

Токен получить у [@BotFather](https://t.me/BotFather).

```bash
cargo run --bin axiom-cli --release --features telegram -- \
  --telegram-token YOUR_BOT_TOKEN

# С ограничением по user_id
cargo run --bin axiom-cli --release --features telegram -- \
  --telegram-token YOUR_BOT_TOKEN \
  --telegram-allow 123456789
```

Команды в Telegram:
```
/start, /status, /domains, /traces
любой текст     — inject в engine
:status, :save  — мета-команды
```

Build-проверка: `cargo build --features telegram`

---

## OpenSearch-адаптер (Phase 5)

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

Подробнее: [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

## axiom-neural (Neural Integration)

Малые нейронные сети которые заменяют rule-based советников NeuralAdvisor.
Пилот: `ReactivationDepthModel` — смотрит на ритмы активности подсистем, предсказывает какие октанты нуждаются в реактивации.

**Важно:** нейронка смотрит не на токены, а на **временны́е ряды активности подсистем** (ActivityTrace rings short=16/mid=64/long=256). Это паттерны второго порядка. Отдельный токен ей ничего не скажет.

### Режим работы (genome.yaml)

```yaml
neural_advisor:
  depth:
    mode: rule          # rule = старые правила (безопасный дефолт)
    weights_path: models/reactivation_depth.bin
    arch:
      conv1_channels: 32
      conv2_channels: 64
      conv2_kernel: 5
      fc1_size: 32
```

Сменить на `mode: neural` — подключить нейронку. Без обученного `.bin` нет смысла.

### Обучение модели

Полный гайд: [docs/guides/Neural_Training_Guide.md](docs/guides/Neural_Training_Guide.md)

Кратко:
1. Запустить OBS — он автоматически сохраняет Sensorium-срезы + ответы rule-based советника в `obs_out/training_data.jsonl`
2. Обучить Python/PyTorch модель (зеркальная архитектура)
3. Экспортировать веса в `models/reactivation_depth.bin` через `axiom-neural-convert`
4. Переключить `mode: neural` в genome.yaml

### Охранная цифра

Inference модели работает раз в 11 тиков и занимает << 1ms. Hot path (TickForward = 24.8 µs) не должен вырасти после подключения. Проверка:

```bash
cargo bench -p axiom-bench --bench engine_bench
```

---

## axiom-observe (OBS)

Автоматический прогон корпуса текстов через движок — измеряет точность распознавания подсистем, собирает снапшоты состояния, генерирует отчёт.

### Быстрый запуск

```bash
just showcase-obs   # showcase-корпус: 18 текстов, 9 подсистем, ~3–5 мин
just showcase       # OBS + все бенчи → showcase/SHOWCASE.md
```

Или через скрипт:

```bash
./scripts/showcase.sh --obs-only   # только OBS
./scripts/showcase.sh              # OBS + бенчи
```

### Напрямую

```bash
# Сборка
cargo build --release -p axiom-observe

# Запуск
./target/release/axiom-observe <corpus.yaml> <out_dir> <anchors_dir>

# Примеры
./target/release/axiom-observe \
  config/obs/corpus_showcase.yaml \
  showcase/obs_out \
  config/anchors

# Большой корпус (40 текстов, 1M тиков, 4 шарда)
./target/release/axiom-observe \
  config/obs/corpus_large.yaml \
  showcase/obs_out \
  config/anchors
```

### Корпусы

| Файл | Тексты | Тиков | Шарды | Время |
|------|--------|-------|-------|-------|
| `corpus_showcase.yaml` | 18 | 200K | 4 | ~3–5 мин |
| `corpus_large.yaml` | 40 | 1M | 4 | ~20–40 мин |
| `corpus_profile.yaml` | 4 | 50K | 1 | ~1 мин (flamegraph) |
| `corpus.yaml` | базовый | — | 1 | быстро |

### Вывод

```
showcase/obs_out/
  snapshots.jsonl   — состояние движка каждые snapshot_every тиков
  events.jsonl      — все инжекции с результатами детекции
  report.md         — итоговый Markdown-отчёт
```

Прогресс выводится в stderr каждые ~5%:
```
[observe] 10000/200000 (5%) — 12s elapsed, ~228s left
[observe] done in 240.1s (833 ticks/sec)
```

### Через Lab панель

Если запущен `axiom-node` — можно запустить OBS прямо из браузера через вкладку **Lab** (`http://localhost:8080`): кнопка **OBS** или **Full Showcase**, прогресс-бар в реальном времени.

### Формат corpus.yaml

```yaml
ticks_total: 200000
snapshot_every: 2000
shards: 4              # параллельные потоки (по числу ядер)

texts:
  - id: math_example
    content: "Производная функции — предел отношения приращений."
    expected_subsystem: mathematics
    inject_every: 300   # каждые N тиков
    inject_count: 400   # всего инжекций для этого текста
    start_at_tick: 0
```

---

## Benchmarks

```bash
just bench                          # все бенчмарки
cargo bench -p axiom-bench          # то же
cargo bench --bench hot_path_regression -- --noplot
cargo bench --bench over_domain_bench -- --noplot
cargo bench --bench stress_bench -- --noplot
```

Результаты: [docs/bench/RESULTS.md](docs/bench/RESULTS.md)

---

## Common Issues

**Build failure:**
```bash
cargo clean && cargo build --release
```

**Сброс состояния (axiom-node):**
```bash
rm -rf data/
```

**Сброс состояния (axiom-cli):**
```bash
rm -rf axiom-data/
cargo run --bin axiom-cli --release -- --no-load
```

**Якоря не загружаются:**
Убедитесь что директория `config/anchors/` существует и содержит `axes.yaml`.
Система работает без якорей — FNV-1a fallback активен автоматически.

**WebSocket не подключается (axiom-node):**
Проверьте что axiom-node запущен и `--http-addr` совпадает с адресом в браузере.

**WebSocket не подключается (axiom-cli):**
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