# Axiom

> High-performance cognitive architecture engine.
> Pure Rust core.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-590%20passing-brightgreen.svg)]()
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![License: Commercial](https://img.shields.io/badge/License-Commercial_Available-purple.svg)](#licensing)

«Свобода для людей, для корпораций за деньги».

---

### ⚠️ Project Status: Active Development

**Axiom is in active development — core architecture complete, 590 tests passing.**

---

### 🤖 AXIOM: AI Disclaimer

> За исключением этого предупреждения, **весь код в этом репозитории написан искусственным интеллектом**.
>
> Здесь смешаны мои промпты, моя концепция и архитектура, тесты и тонны кода, в котором ИИ пытался угадать, чего я хочу на самом деле. Моя роль заключалась в архитектурном проектировании, отборе лучших идей и бесконечном цикле: *«скормить ошибку теста — получить исправление»*.
>
> **«Код проходит все тесты, кроме теста на здравый смысл».**

**🛠 Окружение**
Код написан на **Arch Linux**. Совместимость с Windows/Mac — **нулевая (или случайная)**.

**⚖️ Отказ от ответственности**
Не рекомендуется для продакшена без понимания того, что внутри.

---

### Core Philosophy

Axiom begins with a deliberately uncomfortable idea:

> **What if intelligence is not something we train — but something that emerges when the structure is right?**

Most modern AI systems optimize behavior. **Axiom experiments with conditions.**

This is not another neural network, and not an attempt to simulate the human brain.
Axiom is an exploration of whether coherent cognition can arise from deterministic rules acting within a structured semantic space.

**Think less "model" — more engine.**

#### Semantic Physics
Instead of weights, gradients, or probabilistic inference, Axiom operates on **semantic physics**.
Concepts exist inside **Domains** — bounded environments with local rules:
- Attraction and repulsion.
- Inertia and resistance.
- Interaction constraints.

Reasoning is not executed step-by-step. **It unfolds as state evolution.**

#### Determinism as a Feature
Axiom is intentionally deterministic.
Every transition has a cause. Every outcome can be traced. Nothing hides behind opaque vectors.

---

### Architecture

```
Gateway ── единая точка входа для внешних систем
  └── AxiomEngine
        ├── AshtiCore ── 11 доменов (SUTRA=100 .. MAYA=110)
        │     ├── Arbiter   ── dual-path routing + Experience + Reflector + SkillSet
        │     ├── 11×Domain ── физика поля + CausalFrontier V2.0
        │     └── 11×DomainState ── токены + связи
        └── Guardian  ── CODEX + GENOME: контроль + адаптация + DREAM

Channel ── in-process очередь команд и событий
```

#### Crates

| Crate | Тесты | Назначение |
|-------|-------|-----------|
| axiom-core | 24 | Token (64B), Connection (64B), Event (64B) |
| axiom-genome | 26 | Конституция системы, GenomeIndex O(1) |
| axiom-frontier | 32 | CausalFrontier V2.0, Storm Control |
| axiom-config | 48 | DomainConfig, ConfigLoader, YAML presets |
| axiom-space | 95 | SpatialHashGrid, физика поля |
| axiom-shell | 48 | Shell V3.0, семантические профили |
| axiom-arbiter | 86 | Arbiter, Experience, REFLECTOR, SKILLSET, GridHash |
| axiom-heartbeat | 11 | Heartbeat V2.0 |
| axiom-upo | 13 | UPO v2.2, DynamicTrace, Screen |
| axiom-ucl | 5 | UCL: UclCommand, UclResult, OpCode |
| axiom-domain | 99 | Domain, AshtiCore, CausalHorizon |
| axiom-runtime | 101 | AxiomEngine, Guardian, Gateway, Channel |
| **Итого** | **590** | |

---

### Quick Start

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom
cargo test --workspace
```

Полная документация: [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

```rust
use axiom_runtime::{Gateway, Channel};
use axiom_ucl::{UclCommand, OpCode};

let mut gw = Gateway::with_default_engine();
let mut ch = Channel::new();

ch.send(UclCommand::new(OpCode::TickForward, 0, 0, 0));
let result = gw.process_channel(&mut ch);
```

---

### Documentation

- [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md) — полное руководство по архитектуре и API
- [STATUS.md](STATUS.md) — текущее состояние, тесты по crates
- [ROADMAP.md](ROADMAP.md) — сводка завершённых этапов
- [DEFERRED.md](DEFERRED.md) — технический долг и будущие планы
- [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) — правила разработки
- [docs/bench/RESULTS.md](docs/bench/RESULTS.md) — результаты бенчмарков

---

### Support

<a href="https://buymeacoffee.com/dreeftwood" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;">
</a>

---

### 📜 Licensing & Commercial Use

**Axiom follows a strict Dual License model.**

#### 1. Free for Humanity (AGPL-3.0)
- **Code:** [AGPLv3](LICENSE). Open for research, hacking, and open-source contributions.
- **Condition:** If you share, you must share alike.

#### 2. Paid for Business (Commercial License)
- **Target:** Proprietary software, Enterprise integration, Closed-source SaaS.
- **Benefit:** Removes copyleft restrictions.
- **Includes:** Legal warranty & Priority support.

📩 **Get a Commercial License:** dreeftwood@gmail.com
