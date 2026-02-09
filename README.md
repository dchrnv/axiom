# Axiom

> High-performance cognitive architecture.
> **Rust Core + Python Interface.**

[![Python](https://img.shields.io/badge/python-3.10+-green.svg)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/rust-core-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)

---

### ⚠️ Project Status: Concept & R&D

**Axiom is currently in the early stages of architectural design.**
We are building the foundation using [Neurograph](https://github.com/dchrnv/neurograph) as the primary storage engine.
_Current focus: Core implementation and Python bindings._

---

### 🤖 AXIOM: AI Disclaimer

> За исключением этого предупреждения, **весь код в этом репозитории написан искусственным интеллектом**.
>
> Здесь смешаны мои промпты, мои тесты и тонны кода, в котором ИИ пытался угадать, чего я хочу на самом деле. Моя роль заключалась в архитектурном проектировании, отборе лучших идей и бесконечном цикле: *«скормить ошибку теста — получить исправление»*.
>
> Я честно старался вычитывать результат, но ни одна строчка не прошла проверку на «человеческую логику». Мы с ИИ уже разошлись во взглядах на качество кода.
> **«Код проходит все тесты, кроме теста на здравый смысл».**

**🛠 Окружение**
Код написан на **Arch Linux**. Совместимость с Windows/Mac — **нулевая (или случайная)** — ни я, ни ИИ не пытались это проверить.

**⚖️ Отказ от ответственности**
Я не рекомендую использовать этот код в продакшене. Если он сожжет ваш процессор, вызовет экзистенциальный кризис или обидит вашу кошку — мы с нейросетью не виноваты.
*Врачи могут ошибаться и делать ложные заявления — искусственный интеллект делает это еще увереннее. Используйте на свой страх и риск.*

---

### Core Philosophy & Origin

Axiom begins with a deliberately uncomfortable idea:

> **What if intelligence is not something we train — but something that emerges when the structure is right?**

Most modern AI systems optimize behavior. **Axiom experiments with conditions.**

This is not another neural network, and not an attempt to simulate the human brain.
Axiom is an exploration of whether coherent cognition can arise from deterministic rules acting within a structured semantic space.

**Think less “model” — more engine.**

#### 1. From Infrastructure to Dynamics
Axiom grows directly out of **[Neurograph](https://github.com/dchrnv/neurograph)**.

* **Neurograph** solved the static problem: how to store and traverse large semantic structures efficiently.
* **Axiom** addresses the dynamic one: what happens when meaning is allowed to move, interact, stabilize, and collapse inside such a structure?

If Neurograph is the medium, Axiom defines the motion.

#### 2. Semantic Physics
Instead of weights, gradients, or probabilistic inference, Axiom operates on **semantic physics**.
Concepts exist inside **Domains** — bounded environments with local rules:
* Attraction and repulsion.
* Inertia and resistance.
* Interaction constraints.

Reasoning is not executed step-by-step. **It unfolds as state evolution.**
We do not calculate answers. We define laws — and observe what becomes inevitable.

#### 3. Determinism as a Feature
Axiom is intentionally deterministic.
Every transition has a cause. Every outcome can be traced. Nothing hides behind opaque vectors.

**This is not a limitation — it is the experiment.**

The system is designed for cases where:
* Interpretability matters more than scale.
* Structure matters more than approximation.
* Understanding the *process* matters as much as the *result*.

#### 4. A Controlled Madness
Axiom does not claim consciousness. It does not claim understanding. It does not attempt to imitate humans.

It asks a narrower, sharper question:
**If meaning is treated as a physical system, what kinds of intelligence become possible?**

The answer is unknown.
That is the point.

#### 5. The Stack
* **Core (Rust):** High-frequency state transitions, domain mechanics, and tight integration with Neurograph’s memory model.
* **Interface (Python):** Domain definition, experimentation, inspection, and visualization of semantic dynamics.

---

**Axiom is an executable hypothesis.**
Not a product.
Not a promise.
A machine built to find the edge of what structured cognition can be.

---

### Support

<a href="https://buymeacoffee.com/dreeftwood" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;">
</a>

---

### Installation (Dev)

_Note: This is a pre-alpha build. Expect breaking changes._

```bash
git clone [https://github.com/dchrnv/axiom.git](https://github.com/dchrnv/axiom.git)
cd axiom
pip install -e ".[dev]"
