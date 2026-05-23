# axiom-observe: OBS-01 Report

## Parameters

- Ticks: 3000
- Snapshot every: 100
- Corpus entries: 8

## Final State

| Metric | Value |
|---|---|
| Frames | 12 |
| Total evaluations | 26 |
| Total conflicts | 26 |
| Profile count | 12 |
| Dominant subsystem | 1 |
| Dominant octant | 6 |
| Depth store entries | 12 |
| Emergent pending | 0 |
| Emergent approved | 0 |
| Experience traces | 14 |
| Tension traces | 0 |
| Activity fill | 16 |
| Dominant persistence | 1.000 |
| Entropy gradient | 0.000 |
| Oscillation score | 0.000 |
| Cascade score | 0.000 |
| Fatigue entries | 2 |
| Meta dominant | meta_analysis |

## Experience Trace Growth

| Tick | Exp traces | Tension | Frames | Profiles |
|---|---|---|---|---|
| 0 | 2 | 0 | 0 | 0 |
| 100 | 5 | 0 | 3 | 2 |
| 200 | 8 | 0 | 6 | 6 |
| 300 | 9 | 0 | 7 | 7 |
| 400 | 11 | 0 | 8 | 8 |
| 500 | 11 | 0 | 8 | 8 |
| 600 | 13 | 0 | 9 | 9 |
| 700 | 14 | 0 | 12 | 12 |
| 800 | 14 | 0 | 12 | 12 |
| 900 | 14 | 0 | 12 | 12 |
| 1000 | 14 | 0 | 12 | 12 |
| 1100 | 14 | 0 | 12 | 12 |
| 1200 | 14 | 0 | 12 | 12 |
| 1300 | 14 | 0 | 12 | 12 |
| 1400 | 14 | 0 | 12 | 12 |
| 1500 | 14 | 0 | 12 | 12 |
| 1600 | 14 | 0 | 12 | 12 |
| 1700 | 14 | 0 | 12 | 12 |
| 1800 | 14 | 0 | 12 | 12 |
| 1900 | 14 | 0 | 12 | 12 |
| 2000 | 14 | 0 | 12 | 12 |
| 2100 | 14 | 0 | 12 | 12 |
| 2200 | 14 | 0 | 12 | 12 |
| 2300 | 14 | 0 | 12 | 12 |
| 2400 | 14 | 0 | 12 | 12 |
| 2500 | 14 | 0 | 12 | 12 |
| 2600 | 14 | 0 | 12 | 12 |
| 2700 | 14 | 0 | 12 | 12 |
| 2800 | 14 | 0 | 12 | 12 |
| 2900 | 14 | 0 | 12 | 12 |

## Avg Depth per Octant (final)

| Octant | Avg depth |
|---|---|
| O1 | 0 |
| O2 | 0 |
| O3 | 0 |
| O4 | 0 |
| O5 | 0 |
| O6 | 0 |
| O7 | 200 |
| O8 | 0 |

★ = depth ≥ 8000 (potential emergent threshold)

## Injection Events

| Tick | Entry | Expected | Detected | Coherence | Reflex | Passes | Exp traces |
|---|---|---|---|---|---|---|---|
| 0 | math_basic | mathematics | — | 1.00 | — | 1 | 1 |
| 0 | logic_reasoning | logic | — | 0.75 | — | 1 | 2 |
| 50 | math_advanced | mathematics | — | 0.75 | — | 1 | 3 |
| 80 | time_history | time | 0 | 1.00 | — | 1 | 4 |
| 100 | writing_narrative | writing | 1 | 0.75 | — | 1 | 5 |
| 120 | values_ethics | values | 1 | 0.75 | — | 1 | 6 |
| 150 | writing_poetry | writing | 1 | 0.75 | — | 1 | 7 |
| 200 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 200 | music_theory | music | 1 | 1.00 | ✓ | 1 | 8 |
| 300 | logic_reasoning | logic | 1 | 1.00 | ✓ | 1 | 9 |
| 350 | math_advanced | mathematics | 1 | 1.00 | ✓ | 1 | 10 |
| 350 | writing_narrative | writing | 1 | 1.00 | ✓ | 1 | 11 |
| 400 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 11 |
| 530 | time_history | time | 1 | 1.00 | ✓ | 1 | 12 |
| 550 | writing_poetry | writing | 1 | 1.00 | ✓ | 1 | 13 |
| 550 | music_theory | music | 1 | 1.00 | ✓ | 1 | 13 |
| 600 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 13 |
| 600 | writing_narrative | writing | 1 | 1.00 | ✓ | 1 | 13 |
| 600 | logic_reasoning | logic | 1 | 1.00 | ✓ | 1 | 13 |
| 620 | values_ethics | values | 1 | 1.00 | ✓ | 1 | 14 |
| 650 | math_advanced | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 800 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 850 | writing_narrative | writing | 1 | 1.00 | ✓ | 1 | 14 |
| 900 | music_theory | music | 1 | 1.00 | ✓ | 1 | 14 |
| 900 | logic_reasoning | logic | 1 | 1.00 | ✓ | 1 | 14 |
| 950 | math_advanced | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 950 | writing_poetry | writing | 1 | 1.00 | ✓ | 1 | 14 |
| 980 | time_history | time | 1 | 1.00 | ✓ | 1 | 14 |
| 1000 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 1100 | writing_narrative | writing | 1 | 1.00 | ✓ | 1 | 14 |
| 1120 | values_ethics | values | 1 | 1.00 | ✓ | 1 | 14 |
| 1200 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 1200 | logic_reasoning | logic | 1 | 1.00 | ✓ | 1 | 14 |
| 1250 | math_advanced | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 1250 | music_theory | music | 1 | 1.00 | ✓ | 1 | 14 |
| 1350 | writing_narrative | writing | 1 | 1.00 | ✓ | 1 | 14 |
| 1350 | writing_poetry | writing | 1 | 1.00 | ✓ | 1 | 14 |
| 1400 | math_basic | mathematics | 1 | 1.00 | ✓ | 1 | 14 |
| 1430 | time_history | time | 1 | 1.00 | ✓ | 1 | 14 |
| 1500 | logic_reasoning | logic | 1 | 1.00 | ✓ | 1 | 14 |
| 1600 | music_theory | music | 1 | 1.00 | ✓ | 1 | 14 |

## Coherence Analysis

- Average coherence: 0.970
- Min coherence: 0.750
- Max coherence: 1.000
- Reflex hits: 34 / 41
- Multi-pass events: 0 / 41

✓ High coherence — system has built good resonance patterns.

## Threshold Assessment

No octants reached depth ≥ 8000. Consider more injections or longer run.

No emergent candidates detected yet.

Conflict rate: 100.0% (26 / 26 evaluations)

## V6 Activity Dynamics

| Tick | Fill | Persistence | Entropy | Oscillation | Cascade | Fatigue | Meta | Signatures |
|---|---|---|---|---|---|---|---|---|
| 0 | 0 | 0.00 | 0.00 | 0.00 | 0.00 | 0 | — | Uncertain |
| 100 | 14 | 0.00 | 0.00 | 0.00 | 0.00 | 2 | — | Uncertain |
| 200 | 16 | 0.61 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 300 | 16 | 0.74 | -0.25 | 0.00 | 0.00 | 2 | meta_analysis | Steady, Converging |
| 400 | 16 | 0.81 | -0.33 | 0.00 | 0.00 | 2 | meta_analysis | Steady, Converging |
| 500 | 16 | 0.94 | -0.23 | 0.00 | 0.00 | 2 | meta_analysis | Steady, Converging |
| 600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1100 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1200 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1300 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1400 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1500 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 1900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2100 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2200 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2300 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2400 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2500 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |
| 2900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 2 | meta_analysis | Steady |

## Composite Co-activation Suspects (final)

None detected.

## Meta-subsystem Activations (final)

Active: 2  |  Dominant: meta_analysis

