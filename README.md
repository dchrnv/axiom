# Axiom

> Движок когнитивной архитектуры на чистом Rust.
> Не нейросеть. Эксперимент с тем, что бывает, если сделать всё иначе.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-991%20passing-brightgreen.svg)]()
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Weights License: CC BY-NC-SA 4.0](https://img.shields.io/badge/Weights_License-CC_BY--NC--SA_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![License: Commercial](https://img.shields.io/badge/License-Commercial_Available-purple.svg)](#licensing)

«Свобода для людей, для корпораций за деньги».

---

### ⚠️ Project Status: Active Development

**Axiom is in active development — core architecture complete, 991 tests passing.**

---

### 🤖 Как это сделано

> Весь код в этом репозитории — за исключением этих слов — написан ИИ (и они тоже).
>
> Я занимался концепцией, архитектурой, техническими решениями и обратной связью. ИИ занимался реализацией. Это новый способ строить вещи, и я не вижу смысла делать вид, что всё иначе.
>
> Я читал, отклонял, спорил и переделывал — но не каждую строчку и не с той скрупулёзностью, которой она могла бы заслуживать. Так и работает этот процесс.
>
> **«Идеи мои. Архитектура моя. Баги — честно пополам.»**

**🛠 Окружение:** Написано на **Arch Linux** — потому что жизнь слишком коротка для систем, которые принимают решения за тебя. Совместимость с Windows и Mac не тестировалась и не была целью. Если вдруг заработало — считайте это счастливым случаем.

**🐱 Предупреждение:** Не для продакшена без понимания что внутри. Если сожжёт CPU — это физика. Если вызовет вопросы о природе мышления — это и был план. Если обидит кошку —  это уже серьёзно.

---

### Что такое Axiom

Большинство AI-систем обучаются. Axiom — работает по-другому.

Идея простая и неудобная одновременно: **а что если интеллект — это не то, чему обучают, а то, что возникает само, когда структура правильная?**

Вместо весов, градиентов и вероятностного вывода — **семантическая физика**. Концепции существуют в виде **токенов** — объектов с позицией, массой, температурой и валентностью в трёхмерном семантическом пространстве. Токены живут внутри **доменов** — изолированных сред с локальными физическими правилами: притяжение и отталкивание, инерция, мембранная фильтрация, давление.

Это не нейросеть. Это не симуляция мозга. Это эксперимент с вопросом: **может ли связное поведение возникнуть из детерминированных правил?**

Рассуждение здесь — не последовательность шагов. Оно разворачивается как **эволюция состояния**.

#### Что внутри

- **AshtiCore** — 11 доменов с трёхчастной онтологией: **SUTRA** (первичные истины, вечные сущности) → домены ASHTI 1–8 → **EXPERIENCE** (накопленный опыт) → **MAYA** (живое состояние «сейчас»). Каждый домен — физическое поле со своей конфигурацией.
- **Arbiter** — двойная маршрутизация: быстрый рефлекс через Experience или медленный проход по всем доменам. Как System 1 / System 2 — только без нейронов.
- **Guardian** — CODEX-проверки и GENOME-ограничения. Системные правила, которые нельзя обойти.
- **Over-Domain Layer** — слой компонентов над доменами. **FrameWeaver** сканирует синтаксические узоры в MAYA, кристаллизует стабильные структуры в EXPERIENCE и предлагает промоцию фундаментальных паттернов в SUTRA через CODEX.
- **FractalChain** — несколько уровней AshtiCore, где выход одного становится входом следующего. Масштабирование глубины.
- **Cognitive Depth** — TensionTrace, InternalImpulse, GoalPersistence, Curiosity. Внутренние состояния, влияющие на обработку без внешнего сигнала.
- **CausalFrontier** — очередь событий с причинным порядком. Время в ядре — только `event_id: u64`. Никакого wall-clock, никакой неопределённости.

#### Детерминизм — это не ограничение

Каждый переход имеет причину. Каждый результат можно отследить. Ничто не скрывается за непрозрачными векторами.

Это сознательный выбор. **Это и есть эксперимент.**

---

### Architecture

```
                    ┌─────────────────────────────────────────────┐
  External World    │  External Adapters                           │
  WebSocket /       │    CLI  ── stdin/stdout, axiom-cli.yaml      │
  REST API /        │    WS   ── axum 0.8, ws://host/ws            │
  egui Dashboard /  │    REST ── axum Router, 5 endpoints          │
  Telegram /        │    GUI  ── egui/eframe dashboard             │
  OpenSearch        │    TG   ── Telegram long-poll (feature)      │
                    │    OS   ── OpenSearch indexer  (feature)     │
                    │  tick_loop — единственный writer AxiomEngine │
                    │  Gateway — UCL protocol (in-process)         │
                    └──────────────────┬──────────────────────────┘
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
                    │  Over-Domain Layer ──────────────────────    │
                    │    FrameWeaver: MAYA→scan→EXPERIENCE         │
                    │      синтаксические узоры (0x08 Syntactic)  │
                    │      кристаллизация / ReinforceFrame /       │
                    │      промоция в SUTRA через CODEX            │
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
- [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md) — WebSocket, REST, Dashboard, Telegram, OpenSearch
- [QUICKSTART.md](QUICKSTART.md) — быстрый старт: CLI, WebSocket, REST, адаптеры
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
