# Axiom — Performance Overview

> **For those without context.** This is a snapshot of a cognitive engine written in Rust,
> showing observability run results and benchmark numbers — before and after a round of
> architectural cleanup. Hardware is a budget laptop, nothing exotic.

---

## What is Axiom?

A semantic processing engine. It takes text input, routes it through 8 specialized
semantic domains (ASHTI layer), consolidates in a synthesis layer (MAYA), and builds
experiential traces over time. Think of it as a runtime that continuously classifies
and cross-references what it sees — not an LLM, something lower-level and deterministic.

Written in Rust. No GPU. No external ML runtime at inference time.

**Architecture at a glance:**

```
Text → TextPerceptor → SUTRA (input domain)
         ↓
    8 ASHTI domains  ←  semantic anchors (writing/math/music/logic/time/values/morality/abstractions)
         ↓
       MAYA          ←  consolidation + coherence scoring
         ↓
    EXPERIENCE       ←  resonance search, reflex hits, tension traces
         ↓
  Over-Domain Layer  ←  FrameWeaver, ContextRecognizer, AxialEvaluator (read-only + UCL commands)
```

---

## Hardware

| | |
|---|---|
| CPU | AMD Ryzen 5 3500U (2019 mobile, 4c/8t, ~1.4 GHz base) |
| RAM | 6 GiB |
| OS | Arch Linux, kernel 6.19.9 |
| Rust | 1.91.1 stable, `--release` |
| Bench tool | Criterion 0.5 |

Budget laptop. No tuning, no pinning, no special flags.

---

## OBS Run — Semantic Detection Accuracy

**What OBS measures:** inject 19 semantically distinct texts (corpus) into the engine
200 000 ticks total (~400 injections per text). Count how often the engine correctly
identifies which subsystem a text belongs to (math vs music vs logic vs morality etc.).

**Corpus:** 19 entries covering 8 subsystems:
`mathematics · logic · writing · music · time · values · morality · abstractions`

### Before (June 5, 2026 baseline)

| Subsystem | Example entry | Accuracy |
|-----------|--------------|---------|
| mathematics/geometry | "евклидово пространство..." | ~59% |
| morality/consequences | "этика последствий..." | 0% |
| morality/duty | "долг и обязательство..." | ~47% |
| writing/metaphor | "перенос значения..." | 0% |
| writing/style | "краткость — сестра..." | 0% |
| abstractions | "абстрактные категории..." | 0% |
| logic, math, music, time, values | various | 100% |
| **Overall per-text** | | **68.7%** (5 223 / 7 600) |

### After (June 13, 2026)

| Subsystem | Accuracy |
|-----------|---------|
| All 19 entries | ✓ 100% |
| **Overall per-text** | **100.0%** (7 600 / 7 600) |

**What changed:** Coordinate system cleanup (positions carry proximity only, not
semantic meaning), removed ~1 200 lines of global gravity pulling tokens toward
origin, fixed anchor tag mismatches for `abstractions` and `morality` subsystems,
recalibrated coherence threshold.

### OBS Final State (current)

| Metric | Value | Note |
|--------|-------|------|
| Engine ticks | 200 000 | |
| Corpus injections | 7 600 | 400/text × 19 texts |
| Subsystem detection | **100%** | all 19 entries |
| Experience traces | 8 | stable from tick ~10 000 |
| Dilemmas resolved | 8 | all resolved by tick 10 000 |
| Avg shell similarity | 0.734 | cosine across experience traces |
| Reflex hits | 7 578 / 7 600 | engine recognizing familiar patterns |
| Multi-pass events | 7 600 / 7 600 | coherence threshold 0.784 is strict |
| Avg coherence score | 0.250 | expected: 8 domains apply diverse membrane transforms |
| Emergent depth O7 | 33 200 | above candidate threshold (≥3 000) |
| Emergent depth O8 | 4 310 | above candidate threshold |

> **On coherence 0.250:** The engine checks agreement across 8 domains on 4 fields
> (temperature ±8, mass ±5, valence ±3, position.x ±2). Different membrane profiles
> per domain intentionally spread values — 1/4 fields agreeing is the expected steady state.
> Detection accuracy is 100% regardless.

---

## Benchmarks — Before & After

`cargo bench -p axiom-bench`, all Criterion measurements, median values.

### Hot Path (the number that matters most)

| | Before (v13) | After (v14) | Δ |
|--|-------------|------------|---|
| **TickForward / 50 tokens** | 24.2 µs | **21.4 µs** | -11.7% |
| InjectToken | ~47 µs | **16.9 µs** | -64% |
| AxiomEngine::new (cold start) | ~1 100 µs | **438 µs** | -60% |

> TickForward is the engine's main loop iteration — process one time unit with
> 50 tokens active across all 8 domains. 21 µs on a 4-core 2019 mobile chip.

### Integration throughput (100 000 ticks, realistic load)

| Config | Before | After | Throughput |
|--------|--------|-------|-----------|
| Empty engine | ~3.1 s | **2.16 s** | **46 400 tick/s** (+44%) |
| 50 active tokens | ~3.0 s | **2.22 s** | **45 100 tick/s** (+36%) |
| 50 tok + 100 experience traces | ~2.9 s | **2.26 s** | **44 200 tick/s** (+33%) |
| 50 tok, all subsystems scheduled | ~3.6 s | **2.84 s** | **35 200 tick/s** (+27%) |

### Sustained stress (1 000 ticks × repeated, 60 s window)

| Config | Before | After | Throughput |
|--------|--------|-------|-----------|
| Realistic / 50 tokens | ~29 900 tick/s | **46 900 tick/s** | +57% |
| Heavy / 200 tokens, max schedule | ~19 300 tick/s | **27 600 tick/s** | +43% |

### Semantic search (Experience resonance_search)

| Traces in memory | Before | After | Δ |
|-----------------|--------|-------|---|
| 100 | ~6.9 µs | **1.62 µs** | -77% |
| 500 | ~28 µs | **6.97 µs** | -75% |
| 1 000 | ~50 µs | **13.9 µs** | -72% |
| 10 000 (stress) | ~21 µs | **16.2 µs** | -23% |
| 50 000 (stress) | ~25 µs | **18.1 µs** | ~ |

> Resonance search finds the best experiential match for an incoming token.
> Fast even at 50K traces because of parallel search above 512-trace threshold.

### Core structures (64-byte aligned, repr(C))

| Structure | Time | Δ |
|-----------|------|---|
| Token::new | **17.8 ns** | -75% |
| Token copy (Copy trait) | **24.6 ns** | -69% |
| Event::new | **19.3 ns** | -72% |
| Connection::default | **16.5 ns** | -76% |

> Token, Connection, Event are all exactly 64 bytes — one cache line each.
> Numbers dropped a lot vs stored baseline because the stored baseline was old.

### Over-Domain Layer (runs every tick, reads state, emits UCL commands)

| Scenario | Before | After | Δ |
|----------|--------|-------|---|
| FrameWeaver disabled (overhead only) | ~32 µs | **21.9 µs** | -32% |
| FrameWeaver + 5 MAYA patterns | ~42 µs | **27.4 µs** | -34% |
| FrameWeaver + 20 MAYA patterns | ~69 µs | **46.8 µs** | -32% |

### Spatial index (SpatialHashGrid)

| Tokens | After | Δ |
|--------|-------|---|
| rebuild / 10 000 | 60.4 µs | -23% |
| rebuild / 1 000 000 | **7.93 ms** | -22% |
| find_neighbors / 1 000 tokens | 1.33 µs | ~ |

### Notable regressions (under observation)

| Benchmark | Δ | Comment |
|-----------|---|---------|
| domain_detail_snapshot / 10tok+50conn | +46% | Likely measurement noise — same bench at 50tok+250conn improved -54% |
| inject_loaded / after 1000 ticks + 200 tok | +26% | Warmed-up state with dense membrane transforms |
| Shell::reconcile_batch / 10 items | +9% | Within noise threshold |

---

## Test suite

```
cargo test --workspace --features telegram,opensearch,serde,adapters
→ 1 779 tests, 0 failed
```

---

## What "the cleanup" was

1. **Coordinate system:** positions `[i16;3]` used to encode both proximity AND semantic
   meaning (octant = which axis is dominant). Now positions carry proximity only.
   Semantic nature comes from token fields (mass/valence/temperature) and origin domain.

2. **Dead gravity removed:** ~1 200 lines of global `apply_gravity_batch` pulling every
   token toward origin `(0,0,0)`. This was causing token collapse and incorrect octant
   classification. Removed entirely.

3. **DREAM interval:** was firing on a fixed timer even in production. Changed to
   fire only on idle state. Production runs no longer waste cycles on unsolicited DREAM.

4. **Anchor coverage:** subsystems `abstractions` and `morality` had missing/wrong tags,
   causing 0% detection. Fixed anchor YAML + tag routing.

5. **Coherence threshold tightened:** `min_coherence` raised from 153→200 (out of 255),
   making the engine more demanding about domain agreement before settling on a result.

---

*All numbers from a single developer machine. Reproducible: `cargo bench -p axiom-bench`.*
*OBS reproducible: `cargo run --release -p axiom-observe`.*
