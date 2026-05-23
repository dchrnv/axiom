# axiom-observe: OBS-01 Report

## Parameters

- Ticks: 3000
- Snapshot every: 100
- Corpus entries: 8

## Final State

| Metric | Value |
|---|---|
| Frames | 31 |
| Total evaluations | 45 |
| Total conflicts | 0 |
| Profile count | 31 |
| Dominant subsystem | 3 |
| Dominant octant | 6 |
| Depth store entries | 31 |
| Emergent pending | 0 |
| Emergent approved | 0 |
| Experience traces | 15 |
| Tension traces | 0 |
| Activity fill | 16 |
| Dominant persistence | 1.000 |
| Entropy gradient | 0.000 |
| Oscillation score | 0.000 |
| Cascade score | 0.000 |
| Fatigue entries | 1 |
| Avg shell similarity | 0.000 |
| Meta dominant | meta_perception |

## Experience Trace Growth

| Tick | Exp traces | Tension | Frames | Profiles | ShellSim |
|---|---|---|---|---|---|
| 0 | 2 | 0 | 0 | 0 | 0.000 |
| 100 | 5 | 0 | 3 | 2 | 0.000 |
| 200 | 8 | 0 | 6 | 6 | 0.000 |
| 300 | 9 | 0 | 8 | 8 | 0.000 |
| 400 | 11 | 0 | 9 | 9 | 0.000 |
| 500 | 11 | 0 | 10 | 10 | 0.000 |
| 600 | 13 | 0 | 11 | 11 | 0.000 |
| 700 | 14 | 0 | 16 | 16 | 0.000 |
| 800 | 14 | 0 | 16 | 16 | 0.000 |
| 900 | 14 | 0 | 18 | 17 | 0.000 |
| 1000 | 14 | 0 | 20 | 20 | 0.000 |
| 1100 | 14 | 0 | 21 | 21 | 0.000 |
| 1200 | 15 | 0 | 23 | 23 | 0.000 |
| 1300 | 15 | 0 | 27 | 25 | 0.000 |
| 1400 | 15 | 0 | 27 | 27 | 0.000 |
| 1500 | 15 | 0 | 29 | 29 | 0.000 |
| 1600 | 15 | 0 | 30 | 30 | 0.000 |
| 1700 | 15 | 0 | 31 | 31 | 0.000 |
| 1800 | 15 | 0 | 31 | 31 | 0.000 |
| 1900 | 15 | 0 | 31 | 31 | 0.000 |
| 2000 | 15 | 0 | 31 | 31 | 0.000 |
| 2100 | 15 | 0 | 31 | 31 | 0.000 |
| 2200 | 15 | 0 | 31 | 31 | 0.000 |
| 2300 | 15 | 0 | 31 | 31 | 0.000 |
| 2400 | 15 | 0 | 31 | 31 | 0.000 |
| 2500 | 15 | 0 | 31 | 31 | 0.000 |
| 2600 | 15 | 0 | 31 | 31 | 0.000 |
| 2700 | 15 | 0 | 31 | 31 | 0.000 |
| 2800 | 15 | 0 | 31 | 31 | 0.000 |
| 2900 | 15 | 0 | 31 | 31 | 0.000 |

## Avg Depth per Octant (final)

| Octant | Avg depth |
|---|---|
| O1 | 0 |
| O2 | 0 |
| O3 | 0 |
| O4 | 0 |
| O5 | 0 |
| O6 | 0 |
| O7 | 188 |
| O8 | 0 |

★ = depth ≥ 8000 (potential emergent threshold)

## Injection Events

| Tick | Entry | Expected | Per-text | Detected | Coherence | Reflex | Passes | Exp traces |
|---|---|---|---|---|---|---|---|---|
| 0 | math_basic | mathematics | ✓ mathematics | — | 1.00 | — | 1 | 1 |
| 0 | logic_reasoning | logic | ✓ logic | — | 0.75 | — | 1 | 2 |
| 50 | math_advanced | mathematics | ✓ mathematics | — | 0.75 | — | 1 | 3 |
| 80 | time_history | time | ✓ time | 0 | 0.75 | — | 1 | 4 |
| 100 | writing_narrative | writing | logic | 3 | 0.75 | — | 1 | 5 |
| 120 | values_ethics | values | ✓ values | 3 | 0.75 | — | 1 | 6 |
| 150 | writing_poetry | writing | ✓ writing | 3 | 0.75 | — | 1 | 7 |
| 200 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 7 |
| 200 | music_theory | music | ✓ music | 3 | 1.00 | ✓ | 1 | 8 |
| 300 | logic_reasoning | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 9 |
| 350 | math_advanced | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 10 |
| 350 | writing_narrative | writing | logic | 3 | 1.00 | ✓ | 1 | 11 |
| 400 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 11 |
| 530 | time_history | time | ✓ time | 3 | 1.00 | ✓ | 1 | 12 |
| 550 | writing_poetry | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 13 |
| 550 | music_theory | music | ✓ music | 3 | 1.00 | ✓ | 1 | 13 |
| 600 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 13 |
| 600 | writing_narrative | writing | logic | 3 | 1.00 | ✓ | 1 | 13 |
| 600 | logic_reasoning | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 13 |
| 620 | values_ethics | values | ✓ values | 3 | 1.00 | — | 1 | 14 |
| 650 | math_advanced | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 14 |
| 800 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 14 |
| 850 | writing_narrative | writing | logic | 3 | 1.00 | ✓ | 1 | 14 |
| 900 | music_theory | music | ✓ music | 3 | 1.00 | ✓ | 1 | 14 |
| 900 | logic_reasoning | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 14 |
| 950 | math_advanced | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 14 |
| 950 | writing_poetry | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 14 |
| 980 | time_history | time | ✓ time | 3 | 1.00 | ✓ | 1 | 14 |
| 1000 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 14 |
| 1100 | writing_narrative | writing | logic | 3 | 1.00 | ✓ | 1 | 14 |
| 1120 | values_ethics | values | ✓ values | 3 | 1.00 | ✓ | 1 | 15 |
| 1200 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 15 |
| 1200 | logic_reasoning | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 15 |
| 1250 | math_advanced | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 15 |
| 1250 | music_theory | music | ✓ music | 3 | 1.00 | ✓ | 1 | 15 |
| 1350 | writing_narrative | writing | logic | 3 | 1.00 | ✓ | 1 | 15 |
| 1350 | writing_poetry | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 15 |
| 1400 | math_basic | mathematics | ✓ mathematics | 3 | 1.00 | ✓ | 1 | 15 |
| 1430 | time_history | time | ✓ time | 3 | 1.00 | ✓ | 1 | 15 |
| 1500 | logic_reasoning | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 15 |
| 1600 | music_theory | music | ✓ music | 3 | 1.00 | ✓ | 1 | 15 |

## Coherence Analysis

- Average coherence: 0.963
- Min coherence: 0.750
- Max coherence: 1.000
- Reflex hits: 33 / 41
- Multi-pass events: 0 / 41
- Per-text accuracy: 35 / 41 (85.4%)

✓ High coherence — system has built good resonance patterns.

## Threshold Assessment

No octants reached depth ≥ 8000. Consider more injections or longer run.

No emergent candidates detected yet.

Conflict rate: 0.0% (0 / 45 evaluations)

## V6 Activity Dynamics

| Tick | Fill | Persistence | Entropy | Oscillation | Cascade | Fatigue | Meta | Signatures |
|---|---|---|---|---|---|---|---|---|
| 0 | 0 | 0.00 | 0.00 | 0.00 | 0.00 | 0 | — | Uncertain |
| 100 | 14 | 0.00 | 0.00 | 0.00 | 0.00 | 1 | — | Uncertain |
| 200 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 300 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 400 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 500 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1100 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1200 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1300 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1400 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1500 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 1900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2100 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2200 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2300 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2400 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2500 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2600 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2700 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2800 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |
| 2900 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | meta_perception | Steady |

## Composite Co-activation Suspects (final)

None detected.

## Meta-subsystem Activations (final)

Active: 1  |  Dominant: meta_perception

