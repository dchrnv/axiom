# Axiom — Showcase Report

> Generated: 2026-06-13  
> Engine: V7 (ContextRecognizer V7+DIL-TD-01+OBS-ACC-02, NeuralAdvisor V3, DREAM V1.1, FractalChain)  
> Corpus: `config/obs/corpus_showcase.yaml`

---

## Environment

| | |
|---|---|
| **OS** | Freedesktop SDK 25.08 (Flatpak runtime) |
| **Kernel** | 6.19.9-arch1-1 |
| **CPU** | AMD Ryzen 5 3500U with Radeon Vega Mobile Gfx |
| **Cores** | 8 |
| **RAM** | 6 GiB |
| **Rust** | rustc 1.91.1 (ed61e7d7e 2025-11-07) |

---

## OBS — Live Corpus Run

### Parameters

- Ticks: 200000 · Snapshot every: 2000 · Corpus entries: 19

### Final State

| Metric | Value |
|---|---|
| Frames | 7 |
| Total evaluations | 21 |
| Total conflicts | 0 |
| Profile count | 7 |
| Dominant subsystem | 4 |
| Dominant octant | 6 |
| Depth store entries | 7 |
| Experience traces | 8 |
| Tension traces | 0 |
| Avg shell similarity | 0.734 |
| Dilemmas active | 0 |
| **Dilemmas resolved** | **8** |
| Meta dominant | meta_synthesis |

### Experience Trace Growth (ключевые точки)

| Tick | Exp traces | Tension | Frames | Profiles | ShellSim |
|---|---|---|---|---|---|
| 0 | 1 | 1 | 0 | 0 | 0.000 |
| 2000 | 5 | 0 | 3 | 3 | 0.445 |
| 4000 | 8 | 1 | 5 | 5 | 0.699 |
| 6000 | 8 | 3 | 6 | 6 | 0.734 |
| 10000 | 8 | 9 | 7 | 7 | 0.734 |
| … | … | … | … | … | … |
| 124000 | 8 | 0 | 7 | 7 | 0.734 |
| 200000 | 8 | 0 | 7 | 7 | 0.734 |

*Стабилизация к тику 10000. С тика 124000 tension=0 постоянно — все дилеммы разрешены.*

### Avg Depth per Octant (final)

| Octant | Avg depth |
|---|---|
| O1–O6 | 0 |
| O7 | 33200 ★ |
| O8 | 4310 ★ |

★ = depth ≥ 3000 (emergent candidate threshold)

### Detection Accuracy

| Entry | Expected | Accuracy |
|---|---|---|
| abstract_emergence | abstractions | ✓ 100% |
| abstract_infinity | abstractions | ✓ 100% |
| logic_deductive | logic | ✓ 100% |
| logic_fallacies | logic | ✓ 100% |
| logic_inductive | logic | ✓ 100% |
| math_arithmetic | mathematics | ✓ 100% |
| math_calculus | mathematics | ✓ 100% |
| math_geometry | mathematics | ✓ 100% |
| morality_consequences | morality | ✓ 100% |
| morality_duty | morality | ✓ 100% |
| music_harmony | music | ✓ 100% |
| music_rhythm | music | ✓ 100% |
| time_arrow | time | ✓ 100% |
| time_perception | time | ✓ 100% |
| values_honesty | values | ✓ 100% |
| values_justice | values | ✓ 100% |
| writing_metaphor | writing | ✓ 100% |
| writing_narrative | writing | ✓ 100% |
| writing_style | writing | ✓ 100% |

### Coherence Analysis

- Average coherence: **0.250** · Min: 0.250 · Max: 0.250
- Reflex hits: 7578 / 7600
- **Multi-pass events: 7600 / 7600** *(все инъекции через 3 прохода)*
- Per-text accuracy: **100.0%** (7600 / 7600)

*Coherence 0.250 = 1 из 4 полей совпадает после мембранных трансформов 8 ASHTI-доменов.*
*Это expected при diverse membrane profiles (temperature ±8, mass ±5, valence ±3 — только один уровень может согласоваться).*
*Multi-pass: порог min_coherence=200/255≈0.784 строгий — система всегда делает все 3 прохода.*

### Threshold Assessment

Octants above depth threshold (≥3000): **O7, O8**

⚠ 7 emergent candidate(s) pending — not yet above approval threshold.

Conflict rate: 0.0% (0 / 21 evaluations)

### V6 Activity Dynamics (сокращено)

| Tick | Fill | Persistence | Entropy | Fatigue | Meta | Signature |
|---|---|---|---|---|---|---|
| 0 | 1 | 0.00 | 0.00 | 1 | — | Uncertain |
| 2000 | 16 | 0.98 | 0.00 | 2 | meta_synthesis | Steady |
| 4000 | 16 | 0.70 | 0.23 | 3 | meta_synthesis | Steady, Diverging |
| 6000 | 16 | 0.48 | -0.01 | 3 | meta_synthesis | Steady |
| … (Steady на всём прогоне, fatigue=3, meta_synthesis) … |
| 124000 | 16 | 1.00 | 0.00 | 3 | meta_synthesis | Steady |
| 200000 | 16 | 1.00 | 0.00 | 3 | meta_synthesis | Steady |

*С тика 124000 система полностью стабилизировалась: persistence=1.0, entropy=0.0.*

### Meta-subsystem Activations

Active: 3 · Dominant: **meta_synthesis**

### Dilemmas

- **Active: 0 · Resolved: 8**

✓ DilemmaDetector V2.0 + Resolution Pipeline: 8 дилемм разрешены к тику 10000, активных нет.

---

## Benchmark Results

All measurements: release build, Criterion 0.5, Freedesktop SDK 25.08 (Flatpak runtime), x86_64.

### Hot Path Regression (TickForward / 50 tokens)

```
                        time:   [24.098 µs 24.249 µs 24.411 µs]
                        change: [-15.135% -12.198% -8.9379%] (p = 0.00 < 0.05)
```

### Over-Domain Layer (V7 pipeline)

```
                        time:   [245.42 µs 261.97 µs 280.69 µs]
                        change: [+48.971% +61.000% +73.232%] (p = 0.00 < 0.05)
                        time:   [218.57 µs 234.82 µs 253.07 µs]
                        change: [+19.264% +29.733% +41.613%] (p = 0.00 < 0.05)
                        time:   [227.19 µs 243.62 µs 261.64 µs]
                        change: [+25.972% +35.789% +46.924%] (p = 0.00 < 0.05)
                        time:   [167.95 µs 175.14 µs 183.13 µs]
                        change: [+3.0866% +8.8236% +15.215%] (p = 0.00 < 0.05)
                        time:   [81.916 µs 86.665 µs 92.034 µs]
                        change: [+8.1807% +21.344% +34.248%] (p = 0.00 < 0.05)
                        time:   [74.077 µs 76.326 µs 79.020 µs]
                        change: [+9.2763% +14.667% +19.733%] (p = 0.00 < 0.05)
```

---

## Summary

| Parameter | Value |
|-----------|-------|
| Engine ticks | 200000 |
| Corpus texts | 19 |
| Subsystems covered | mathematics · writing · logic · music · time · values · morality · abstractions |
| Dilemmas resolved | 8 (все к тику 10000) |
| Multi-pass events | 7600 / 7600 (min_coherence=0.784 строгий, 3 прохода всегда) |
| Avg coherence | 0.250 (diverse membrane profiles, expected) |
| **Per-text accuracy** | **100.0% (7600/7600)** |
| Emergent octants | O7 ★ 33200 · O8 ★ 4310 |

Criterion HTML reports: `target/criterion/`  
Raw bench logs: `showcase/bench_out/`  
OBS snapshots: `showcase/obs_out/`  
