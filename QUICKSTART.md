# AXIOM Quick Start

**v11 · 2026-05-17**

---

## Prerequisites

- **OS:** Linux (Arch Linux рекомендуется)
- **Rust:** 1.75+

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Installation

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom

cargo test --workspace   # 1344 тестов, 0 failures
cargo build --release    # production build (~7 мин первый раз)
```

---

## Запуск

### Один скрипт (рекомендуется)

```bash
./run.sh          # node + workstation, нода останавливается при закрытии окна
./run.sh --build  # пересобрать перед запуском
```

### Вручную (два терминала)

```bash
# Терминал 1 — движок
cargo run -p axiom-node --release

# Терминал 2 — интерфейс (после того как node залогировал "listening on 127.0.0.1:9876")
cargo run -p axiom-workstation --release
```

Workstation по умолчанию подключается к `127.0.0.1:9876`.

---

## Архитектура

```
axiom-node          — когнитивный движок (tick loop 60 Hz, WebSocket-сервер)
axiom-workstation   — десктопный интерфейс оператора (iced 0.13)
axiom-broadcasting  — WebSocket-сервер между node и workstation
```

При старте `axiom-node`:
1. Восстанавливает состояние из `./data/` (если есть)
2. Загружает якоря из `config/anchors/` и инжектирует их в движок
3. Инициализирует Phase C компоненты (AxialEvaluator, ContextRecognizer, NeuralAdvisor)
4. Запускает tick loop на 60 Hz

---

## Workstation

**8 вкладок:**

| Вкладка | Что показывает |
|---------|---------------|
| **System Map** | Мандала ASHTI-доменов с пульсацией активности |
| **Live Field** | 3D-визуализация токенов, орбитальная камера (drag + scroll) |
| **Conversation** | Текстовый ввод → TextPerceptor → Engine |
| **Patterns** | Sparklines слоёв L1–L8, Phase C (октант/подсистема), лента Frame-событий |
| **Dream State** | Fatigue, DREAM-цикл, force sleep / wake |
| **Configuration** | Редактор конфигурации движка (требует реализации в node) |
| **Files** | Импорт через адаптеры (в разработке) |
| **Benchmarks** | Запуск встроенных бенчмарков (в разработке) |

**Keyboard shortcuts:** `Ctrl+1–8` — переключение вкладок.

### Подача текста

Вкладка **Conversation** → ввести текст → Enter.

Токен позиционируется через якоря (`config/anchors/`) если слово совпадает,
иначе — через FNV-1a хэш. Температура нового токена: 150–255.

### Phase C (Patterns)

Показывает данные как только FrameWeaver кристаллизовал хотя бы один Frame:
- **Octant** — доминирующий октант (AxialEvaluator, каждый t%5)
- **Subsystem** — доминирующая подсистема (ContextRecognizer, каждый t%7)
- **Emergent candidates** — кандидаты от NeuralAdvisor с кнопкой Approve

---

## Конфигурация

```
config/
  axiom.yaml          — основная конфигурация движка
  genome.yaml         — геном (доступ модулей, параметры)
  anchors/
    axes.yaml         — 6 осевых якорей (X/Y/Z полюса ±30000)
    octants.yaml      — 8 архетипов октантов
    semantic_centers.yaml
    layers/           — якоря Shell-слоёв L1–L8 (только L5 заполнен)
    domains/          — якоря ASHTI-доменов D1–D8
    writing/          — 7 графических примитивов письма
    mathematics/      — 7 структурных примитивов математики
```

---

## Benchmarks

```bash
# Полный прогон (без stress и 1M-тиков, ~15 мин)
cargo bench -p axiom-bench -- "100k|integrated_cycle|periodic|tick_schedule|engine_bench|fractalchain|frameweaver|phase_c"

# Конкретная группа
cargo bench -p axiom-bench --bench engine_bench
cargo bench -p axiom-bench --bench integration_bench -- "100k"
```

Результаты: [docs/bench/RESULTS.md](docs/bench/RESULTS.md)

---

## Rust API

```rust
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

let mut engine = AxiomEngine::new();
let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
engine.process_command(&cmd);
```

С якорным позиционированием и TextPerceptor:

```rust
use axiom_config::AnchorSet;
use axiom_agent::perceptors::text::TextPerceptor;
use std::sync::Arc;

let anchors = Arc::new(AnchorSet::load_dir(std::path::Path::new("config/anchors")));
engine.inject_anchor_tokens(&anchors);
engine.apply_anchor_set(&anchors);
let perceptor = TextPerceptor::with_anchors(anchors);
let cmd = perceptor.perceive("порядок");
engine.process_and_observe(&cmd);
```

---

## Common Issues

**Якоря не загружаются (loaded 0 anchors):**
Запускай `axiom-node` из корня репозитория — путь `config/anchors` резолвится относительно CWD.
`./run.sh` делает это автоматически.

**Workstation зависла на Welcome-экране:**
Убедись что `axiom-node` запущен и залогировал `"listening on 127.0.0.1:9876"`.

**Conversation не принимает новый ввод:**
Если первый SubmitText завис — авторелиз через 5 секунд.

**Сброс состояния:**
```bash
rm -rf data/
```

**Пересборка с нуля:**
```bash
cargo clean && cargo build --release
```

**Запустить конкретный тест:**
```bash
cargo test -p axiom-runtime -- engine
cargo test -p axiom-config -- anchor --include-ignored
cargo test --workspace
```
