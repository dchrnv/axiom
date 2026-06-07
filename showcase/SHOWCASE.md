# Axiom — Showcase Report

> Generated: 2026-06-07  
> Engine: V7 (ContextRecognizer V7+DIL-TD-01, NeuralAdvisor V3, DREAM V1.1, FractalChain)  
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
| Frames | 8 |
| Total evaluations | 15 |
| Total conflicts | 0 |
| Profile count | 8 |
| Dominant subsystem | 4 |
| Dominant octant | 2 |
| Experience traces | 5 |
| Emergent pending | 6 |
| Dilemmas active | 8 |
| **Dilemmas resolved** | **64** |
| Meta dominant | meta_synthesis |

### Experience Trace Growth (ключевые точки)

| Tick | Exp traces | Tension | Frames | Profiles |
|---|---|---|---|---|
| 0 | 1 | 0 | 0 | 0 |
| 2000 | 2 | 0 | 2 | 2 |
| 4000 | 5 | 0 | 5 | 5 |
| 6000 | 5 | 0 | 6 | 6 |
| 14000 | 5 | 0 | 8 | 8 |
| … | … | … | … | … |
| 200000 | 5 | 0 | 8 | 8 |

*Стабилизация к тику 14000. Frames=8, Exp traces=5 до конца прогона.*

### Avg Depth per Octant (final)

| Octant | Avg depth |
|---|---|
| O1 | 11270 ★ |
| O2 | 0 |
| O3 | 18750 ★ |
| O4–O6 | 0 |
| O7 | 5 |
| O8 | 3 |

★ = depth ≥ 3000 (emergent candidate threshold)

### Detection Accuracy

| Entry | Expected | Accuracy |
|---|---|---|
| logic_deductive | logic | ✓ 100% |
| logic_fallacies | logic | ✓ 100% |
| logic_inductive | logic | ✓ 100% |
| math_arithmetic | mathematics | ✓ 100% |
| math_calculus | mathematics | ✓ 100% |
| math_geometry | mathematics | ~ 59% |
| morality_consequences | morality | ✗ 0% |
| morality_duty | morality | ~ 47% |
| music_harmony | music | ✓ 100% |
| music_rhythm | music | ✓ 100% |
| time_arrow | time | ✓ 100% |
| time_perception | time | ✓ 100% |
| values_honesty | values | ✓ 100% |
| values_justice | values | ✓ 100% |
| writing_metaphor | writing | ✗ 0% |
| writing_narrative | writing | ✓ 100% |
| writing_style | writing | ✗ 0% |
| abstract_emergence | abstractions | ✗ 0% |
| abstract_infinity | abstractions | ✗ 0% |

### Coherence Analysis

- Average coherence: **0.750** · Min: 0.750 · Max: 1.000
- Reflex hits: 7591 / 7600
- **Multi-pass events: 1 / 7600** *(впервые активирован)*
- Per-text accuracy: **68.7%** (5223 / 7600)

✓ High coherence — system has built good resonance patterns.

### Threshold Assessment

Octants above depth threshold (≥3000): **O1, O3**

⚠ 6 emergent candidate(s) pending — not yet above approval threshold.

Conflict rate: 0.0% (0 / 15 evaluations)

### V6 Activity Dynamics (сокращено)

| Tick | Fill | Persistence | Entropy | Fatigue | Meta | Signature |
|---|---|---|---|---|---|---|
| 0 | 1 | 0.00 | 0.00 | 1 | — | Uncertain |
| 2000 | 16 | 0.98 | 0.00 | 2 | meta_synthesis | Steady |
| 4000 | 16 | 0.70 | 0.28 | 4 | meta_synthesis | Steady, Diverging |
| 6000 | 16 | 0.45 | 0.00 | 4 | meta_synthesis | Steady |
| … (Steady на всём прогоне, fatigue=4, meta_synthesis) … |
| 122000 | 16 | 0.47 | -0.03 | 4 | meta_synthesis | Steady |
| 124000 | 16 | 1.00 | 0.00 | 4 | meta_synthesis | Steady |
| 200000 | 16 | 1.00 | 0.00 | 4 | meta_synthesis | Steady |

*С тика 124000 система полностью стабилизировалась: persistence=1.0, entropy=0.0.*

### Meta-subsystem Activations

Active: 3 · Dominant: **meta_synthesis**

### Dilemmas (DIL-TD-01)

- **Active: 8 · Resolved: 64** (ring-buffer MAX_RESOLVED заполнен)

✓ DilemmaDetector V2.0 + Resolution Pipeline работают.

#### Dilemma Timeline (ключевые точки)

| Tick | Active | Resolved |
|---|---|---|
| 4000 | 4 | 0 |
| 6000 | 8 | 0 |
| 14000 | 8 | 8 |
| 20000 | 8 | 9 |
| 22000 | 8 | 16 |
| 30000 | 8 | 24 |
| 38000 | 8 | 32 |
| 46000 | 8 | 40 |
| 54000 | 8 | 48 |
| 62000 | 8 | 56 |
| 72000 | 8 | **64** |
| … | 8 | 64 |
| 200000 | 8 | 64 |

*Система разрешает ~8 дилемм каждые ~8000 тиков, ring-buffer заполнен к тику 72000.*

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
| Dilemmas resolved | 64 (MAX_RESOLVED, ring-buffer заполнен к тику 72000) |
| Multi-pass events | 1 / 7600 (DIL-TD-01, впервые активирован) |
| Avg coherence | 0.750 (было 1.000 до калибровки) |
| Per-text accuracy | 68.7% |
| Emergent octants | O1 ★ 11270 · O3 ★ 18750 |

Criterion HTML reports: `target/criterion/`  
Raw bench logs: `showcase/bench_out/`  
OBS snapshots: `showcase/obs_out/`  
