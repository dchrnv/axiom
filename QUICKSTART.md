# Axiom Quick Start

---

## Prerequisites

- **OS:** Linux (Arch Linux recommended)
- **Rust:** 1.75+

```bash
# Установить Rust (если не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Installation

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom

# Собрать и прогнать все тесты
cargo test --workspace

# Release build
cargo build --release
```

---

## Basic Usage

### Minimal example

```rust
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

let mut engine = AxiomEngine::new();
let cmd = UclCommand::new(OpCode::TickForward, 0, 0, 0);
engine.process_command(&cmd);
let events = engine.drain_events();
```

### Через Gateway (рекомендуется для внешних систем)

```rust
use axiom_runtime::{Gateway, Channel};
use axiom_ucl::{UclCommand, OpCode};

let mut gw = Gateway::with_default_engine();
let mut ch = Channel::new();

ch.send(UclCommand::new(OpCode::TickForward, 0, 0, 0));
let result = gw.process_channel(&mut ch);
assert!(result.all_ok());
```

### Инъекция токена

```rust
use axiom_runtime::AxiomEngine;
use axiom_ucl::{UclCommand, OpCode};

let mut engine = AxiomEngine::new();

// LOGIC domain = level_id(1)*100 + role(6) = 106
let mut cmd = UclCommand::new(OpCode::InjectToken, 106, 100, 0);
cmd.payload[0] = 106u8;  // domain_id lo
cmd.payload[1] = 0u8;    // domain_id hi
cmd.payload[4..8].copy_from_slice(&100.0f32.to_le_bytes()); // mass
engine.process_command(&cmd);
```

---

## Benchmarks

```bash
cargo bench -p axiom-bench
```

Результаты: [docs/bench/RESULTS.md](docs/bench/RESULTS.md)

---

## Documentation

- [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md) — полное руководство
- [STATUS.md](STATUS.md) — текущее состояние
- [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) — правила разработки

---

## Common Issues

**Rust not found**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Build failures**
```bash
cargo clean && cargo build --release
```

**Запустить конкретный тест**
```bash
cargo test -p axiom-runtime test_gateway_process
```
