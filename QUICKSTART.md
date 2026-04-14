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

cargo test --workspace   # 932 тестов, 0 failures
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

  path:     slow-path
  domain:   101 (EXECUTION)
  coherence:0.75
  traces:   3 matched
  position: (22000, 1500, 500)   ← якорное позиционирование
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

**Запустить конкретный тест:**
```bash
cargo test -p axiom-runtime test_inject_anchor_tokens_axes
```
