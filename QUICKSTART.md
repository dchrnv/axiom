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

cargo test --workspace   # все тесты
cargo build --release    # production build
```

---

## Запуск CLI

```bash
cargo run --bin axiom-cli --release
```

При старте система загружает состояние из `./axiom-data` (если существует).

```bash
# Полезные флаги
cargo run --bin axiom-cli --release -- --verbose    # показывать состояние после каждого ввода
cargo run --bin axiom-cli --release -- --adaptive   # адаптивная частота тиков
cargo run --bin axiom-cli --release -- --no-load    # чистый старт без загрузки
```

---

## Работа в CLI

**Текстовый ввод** — любая строка без `:` в начале:
```
axiom> hello world

  path:     slow-path       # slow-path → reflex после повторных паттернов
  domain:   101 (EXECUTION)
  coherence:0.75            # < 0.6 → создаётся tension trace
  traces:   47 matched
  position: (3113, 6636, 10985)
```

**Служебные команды** (начинаются с `:`):
```
:status          — tick_count и tension
:memory          — полная статистика памяти
:domains         — список доменов с токенами
:verbose [on|off]— подробный вывод
:save [path]     — сохранить состояние
:load [path]     — загрузить состояние
:autosave on N   — автосохранение каждые N тиков
:export traces   — экспорт знаний в JSON
:import traces   — импорт знаний из JSON
:tick [N]        — прокрутить N тиков вручную
:help            — полный список команд
:quit            — выход
```

Подробное описание всех команд, параметров и конфигурации:
**[docs/guides/CLI_Reference.md](docs/guides/CLI_Reference.md)**

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

**Запустить конкретный тест:**
```bash
cargo test -p axiom-runtime test_gateway_process
```
