# Axiom — Showcase Report

> Generated: 2026-05-29 20:35  
> Engine: V7 (ContextRecognizer V7, NeuralAdvisor V3, DREAM V1.1, FractalChain)  
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

# axiom-observe: OBS-01 Report

## Parameters

- Ticks: 200000
- Snapshot every: 2000
- Corpus entries: 19

## Final State

| Metric | Value |
|---|---|
| Frames | 2000 |
| Total evaluations | 2007 |
| Total conflicts | 0 |
| Profile count | 2000 |
| Dominant subsystem | 3 |
| Dominant octant | 0 |
| Depth store entries | 2000 |
| Emergent pending | 1000 |
| Emergent approved | 0 |
| Experience traces | 7 |
| Tension traces | 0 |
| Activity fill | 16 |
| Dominant persistence | 1.000 |
| Entropy gradient | 0.000 |
| Oscillation score | 0.000 |
| Cascade score | 0.000 |
| Fatigue entries | 1 |
| Avg shell similarity | 0.000 |
| Meta dominant | none |

## Experience Trace Growth

| Метрика | Начало | Конец |
|---|---|---|
| Exp traces | 1 | **7** (plateau — малый корпус) |
| Frames | 0 | **2000** (линейный рост) |
| Tension | 0 | 0 |
| Shell similarity | 0.000 | 0.000 |


## Avg Depth per Octant (final)

| Octant | Avg depth |
|---|---|
| O1 | 7500 ★ |
| O2 | 0 |
| O3 | 0 |
| O4 | 0 |
| O5 | 0 |
| O6 | 0 |
| O7 | 0 |
| O8 | 144 |

★ = depth ≥ 3000 (emergent candidate threshold)

## Detection Accuracy

| Entry | Expected | Correct | Total | Accuracy |
|---|---|---|---|---|
| abstract_emergence | abstractions | 0 | 400 | ✗ 0% |
| abstract_infinity | abstractions | 0 | 400 | ✗ 0% |
| logic_deductive | logic | 400 | 400 | ✓ 100% |
| logic_fallacies | logic | 400 | 400 | ✓ 100% |
| logic_inductive | logic | 400 | 400 | ✓ 100% |
| math_arithmetic | mathematics | 400 | 400 | ✓ 100% |
| math_calculus | mathematics | 400 | 400 | ✓ 100% |
| math_geometry | mathematics | 0 | 400 | ✗ 0% |
| morality_consequences | morality | 0 | 400 | ✗ 0% |
| morality_duty | morality | 0 | 400 | ✗ 0% |
| music_harmony | music | 400 | 400 | ✓ 100% |
| music_rhythm | music | 400 | 400 | ✓ 100% |
| time_arrow | time | 400 | 400 | ✓ 100% |
| time_perception | time | 400 | 400 | ✓ 100% |
| values_honesty | values | 400 | 400 | ✓ 100% |
| values_justice | values | 400 | 400 | ✓ 100% |
| writing_metaphor | writing | 0 | 400 | ✗ 0% |
| writing_narrative | writing | 400 | 400 | ✓ 100% |
| writing_style | writing | 0 | 400 | ✗ 0% |



## Coherence Analysis

- Average coherence: 1.000
- Min coherence: 0.750
- Max coherence: 1.000
- Reflex hits: 7593 / 7600
- Multi-pass events: 0 / 7600
- Per-text accuracy: 4800 / 7600 (63.2%)

✓ High coherence — system has built good resonance patterns.

## Threshold Assessment

Octants above depth threshold (≥3000): O1

⚠ 1000 emergent candidate(s) pending — not yet above approval threshold.

Conflict rate: 0.0% (0 / 2007 evaluations)

## V6 Activity Dynamics

Signature stable throughout: **Steady, Uncertain**  
Fill=16 · Persistence=1.00 · Entropy=0.00 · Oscillation=0.00 · Cascade=0.00  
Fatigue=1 · Meta=—


## Composite Co-activation Suspects (final)

None detected.

## Meta-subsystem Activations (final)

Active: 0  |  Dominant: none


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
                        time:   [74.586 µs 77.441 µs 81.186 µs]
                        change: [+6.5262% +12.585% +19.660%] (p = 0.00 < 0.05)
                        time:   [66.008 µs 67.734 µs 69.851 µs]
                        change: [-3.4850% -0.1242% +3.1325%] (p = 0.95 > 0.05)
                        time:   [92.358 µs 103.12 µs 117.15 µs]
                        change: [+20.081% +34.876% +53.245%] (p = 0.00 < 0.05)
                        time:   [142.26 µs 212.41 µs 301.69 µs]
                        change: [+81.856% +168.29% +274.54%] (p = 0.00 < 0.05)
                        time:   [91.258 µs 131.18 µs 200.60 µs]
                        change: [+2.8007% +47.609% +140.55%] (p = 0.14 > 0.05)
                        time:   [58.140 ms 74.707 ms 90.006 ms]
                        change: [+115.59% +184.28% +263.39%] (p = 0.00 < 0.05)
                        time:   [111.11 ms 137.43 ms 163.10 ms]
                        change: [+335.33% +436.61% +537.48%] (p = 0.00 < 0.05)
                        time:   [69.417 µs 81.129 µs 94.950 µs]
                        change: [+105.93% +137.79% +185.81%] (p = 0.00 < 0.05)
                        time:   [66.377 µs 68.507 µs 71.526 µs]
                        change: [+30.128% +35.089% +41.393%] (p = 0.00 < 0.05)
```

---

## Summary

| Parameter | Value |
|-----------|-------|
| Engine ticks | 200000 |
| Corpus texts | 19 |
| Subsystems covered | mathematics · writing · logic · music · time · values · morality · abstractions · dilemmas |

Criterion HTML reports: `target/criterion/`  
Raw bench logs: `showcase/bench_out/`  
OBS snapshots: `showcase/obs_out/`  

---

## Stress Benchmarks

`axiom-space` под нагрузкой (10K → 10M токенов).

### apply_gravity_batch

| N токенов | Scalar | AVX2 | Speedup |
|-----------|--------|------|---------|
| 10K | 481 µs | 99 µs | ~5x |
| 100K | 3.97 ms | 1.08 ms | ~4x |
| 1M | 38.5 ms | 17.6 ms | ~2x |
| 10M | 397 ms | — | — |

### SpatialHashGrid::rebuild

| N токенов | Время |
|-----------|-------|
| 10K | 123 µs |
| 100K | 1.04 ms |
| 500K | 5.39 ms |
| 1M | 10.76 ms |

### resonance_search (ExperienceModule)

| Traces | Время |
|--------|-------|
| 1K | 17.8 µs |
| 5K | 22.3 µs |
| 10K | 17.6 µs |
| 50K | 15.3 µs |

Grid-хэш Phase 1 эффективен — время практически не растёт с числом трейсов.

> При `max_tokens_per_domain: 2000` все операции работают в диапазоне < 1 µs на тик.
