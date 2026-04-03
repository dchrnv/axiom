# Axiom

> High-performance cognitive architecture engine.
> Pure Rust core.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-797%20passing-brightgreen.svg)]()
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Weights License: CC BY-NC-SA 4.0](https://img.shields.io/badge/Weights_License-CC_BY--NC--SA_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![License: Commercial](https://img.shields.io/badge/License-Commercial_Available-purple.svg)](#licensing)

«Свобода для людей, для корпораций за деньги».

---

### ⚠️ Project Status: Active Development

**Axiom is in active development — core architecture complete, 797 tests passing.**

---

### 🤖 AXIOM: AI Disclaimer

> За исключением этого предупреждения, **весь код в этом репозитории написан искусственным интеллектом**.
>
> Здесь смешаны мои промпты, моя концепция и архитектура, тесты и тонны кода, в котором ИИ пытался угадать, чего я хочу на самом деле. Моя роль заключалась в архитектурном проектировании, отборе лучших идей и бесконечном цикле: *«скормить ошибку теста — получить исправление»*.
>
> Я честно старался вычитывать результат, но ни одна строчка не прошла проверку на «человеческую логику». Мы с ИИ уже разошлись во взглядах на качество кода.
> **«Код проходит все тесты, кроме теста на здравый смысл».**

**🛠 Окружение**
Код написан на **Arch Linux**. Совместимость с Windows/Mac — **нулевая (или случайная)** — ни я, ни ИИ не пытались это проверить.

**⚖️ Отказ от ответственности**
Не рекомендуется для продакшена без понимания того, что внутри. Если он сожжёт ваш процессор, вызовет экзистенциальный кризис или обидит вашу кошку — мы с нейросетью не виноваты.
*Врачи могут ошибаться и делать ложные заявления — искусственный интеллект делает это ещё увереннее. Используйте на свой страх и риск.*

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

**This is not a limitation — it is the experiment.**

---

### Architecture

```
                    ┌─────────────────────────────────────────────┐
  External World    │  Gateway — единая точка входа (UCL protocol) │
  CLI / Telegram /  │  Channel — in-process command/event queue    │
  Shell / gRPC      └──────────────────┬──────────────────────────┘
                                       │ UclCommand (64B)
                    ┌──────────────────▼──────────────────────────┐
                    │               AxiomEngine                    │
                    │                                              │
                    │  ┌─────────────────────────────────────┐    │
                    │  │            AshtiCore                 │    │
                    │  │                                      │    │
                    │  │  SUTRA (100) ── точка входа потока   │    │
                    │  │  ASHTI 1–8   ── когнитивные домены   │    │
                    │  │    ├─ Domain: физика поля            │    │
                    │  │    │   CausalFrontier V2.0           │    │
                    │  │    └─ DomainState: токены + связи    │    │
                    │  │  EXPERIENCE (109) ── ассоц. память   │    │
                    │  │  MAYA (110)  ── консолидация         │    │
                    │  │                                      │    │
                    │  │  Arbiter ── dual-path routing:       │    │
                    │  │    fast path: рефлекс (Experience)   │    │
                    │  │    slow path: ASHTI 1→8→MAYA         │    │
                    │  │    + Reflector, SkillSet             │    │
                    │  │    + Cognitive Depth (TensionTrace,  │    │
                    │  │      InternalImpulse, GoalPersist,   │    │
                    │  │      Curiosity)                      │    │
                    │  └─────────────────────────────────────┘    │
                    │                                              │
                    │  Guardian ── CODEX + GENOME enforcement:     │
                    │    enforce_access, validate_reflex           │
                    │    adapt_thresholds, dream_propose           │
                    │                                              │
                    │  COM ── монотонный event_id, TickSchedule    │
                    └──────────────────────────────────────────────┘
                                       │
                    ┌──────────────────▼──────────────────────────┐
                    │  FractalChain — N уровней AshtiCore          │
                    │  MAYA[n] → SUTRA[n+1], skill exchange        │
                    └─────────────────────────────────────────────┘
```

Каждый токен (64B, `repr(C, align(64))`) — единица смысла в семантическом пространстве.
Связи (64B) — взаимодействие между токенами. Оба обрабатываются детерминированной физикой поля.
Время в ядре — только причинный порядок (`event_id: u64`), никакого wall-clock.

---

### Quick Start

```bash
git clone https://github.com/dchrnv/axiom.git
cd axiom
cargo test --workspace
```

```rust
use axiom_runtime::{Gateway, Channel};
use axiom_ucl::{UclCommand, OpCode};

let mut gw = Gateway::with_default_engine();
let mut ch = Channel::new();

ch.send(UclCommand::new(OpCode::TickForward, 0, 0, 0));
let result = gw.process_channel(&mut ch);
```

Полная документация: [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)

---

### Documentation

- [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md) — полное руководство по архитектуре и API
- [docs/guides/ML_ENGINE_GUIDE.md](docs/guides/ML_ENGINE_GUIDE.md) — MLEngine, VisionPerceptor, AudioPerceptor
- [docs/guides/FRACTAL_SIMD_GUIDE.md](docs/guides/FRACTAL_SIMD_GUIDE.md) — FractalChain, batch-физика
- [STATUS.md](STATUS.md) — текущее состояние, тесты по crates
- [ROADMAP.md](ROADMAP.md) — активные планы
- [DEFERRED.md](DEFERRED.md) — технический долг
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

#### 1. Free for Humanity (AGPL-3.0 + CC BY-NC-SA)
- **Code:** [AGPLv3](LICENSE). Open for research, hacking, and open-source contributions.
- **Weights:** [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Open for academic study and non-commercial experiments.
- **Condition:** If you share, you must share alike. No closed doors.

#### 2. Paid for Business (Commercial License)
- **Target:** Proprietary software, Enterprise integration, Closed-source SaaS.
- **Benefit:** Removes copyleft and non-commercial restrictions.
- **Includes:** Legal warranty & Priority support.

📩 **Get a Commercial License:** dreeftwood@gmail.com
