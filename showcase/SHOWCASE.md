# Axiom — Showcase Report

> Generated: 2026-05-29 20:43  
> Engine: V7 + Performance Sprint (parallel ticks, STATE_SLEEPING lifecycle, parallel OBS shards)  
> Corpus: `config/obs/corpus_showcase.yaml` (18 texts, 9 subsystems, 200K ticks, 4 shards)

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

| Tick | Exp traces | Tension | Frames | Profiles | ShellSim |
|---|---|---|---|---|---|
| 0 | 1 | 0 | 0 | 0 | 0.000 |
| 2000 | 2 | 0 | 9 | 9 | 0.000 |
| 4000 | 7 | 0 | 29 | 29 | 0.000 |
| 6000 | 7 | 0 | 61 | 61 | 0.000 |
| 8000 | 7 | 0 | 95 | 95 | 0.000 |
| 10000 | 7 | 0 | 129 | 129 | 0.000 |
| 12000 | 7 | 0 | 161 | 161 | 0.000 |
| 14000 | 7 | 0 | 195 | 195 | 0.000 |
| 16000 | 7 | 0 | 229 | 229 | 0.000 |
| 18000 | 7 | 0 | 261 | 261 | 0.000 |
| 20000 | 7 | 0 | 295 | 295 | 0.000 |
| 22000 | 7 | 0 | 329 | 329 | 0.000 |
| 24000 | 7 | 0 | 361 | 361 | 0.000 |
| 26000 | 7 | 0 | 395 | 395 | 0.000 |
| 28000 | 7 | 0 | 429 | 429 | 0.000 |
| 30000 | 7 | 0 | 461 | 461 | 0.000 |
| 32000 | 7 | 0 | 495 | 495 | 0.000 |
| 34000 | 7 | 0 | 529 | 529 | 0.000 |
| 36000 | 7 | 0 | 561 | 561 | 0.000 |
| 38000 | 7 | 0 | 595 | 595 | 0.000 |
| 40000 | 7 | 0 | 629 | 629 | 0.000 |
| 42000 | 7 | 0 | 661 | 661 | 0.000 |
| 44000 | 7 | 0 | 695 | 695 | 0.000 |
| 46000 | 7 | 0 | 729 | 729 | 0.000 |
| 48000 | 7 | 0 | 761 | 761 | 0.000 |
| 50000 | 7 | 0 | 795 | 795 | 0.000 |
| 52000 | 7 | 0 | 829 | 829 | 0.000 |
| 54000 | 7 | 0 | 861 | 861 | 0.000 |
| 56000 | 7 | 0 | 895 | 895 | 0.000 |
| 58000 | 7 | 0 | 929 | 929 | 0.000 |
| 60000 | 7 | 0 | 961 | 961 | 0.000 |
| 62000 | 7 | 0 | 995 | 995 | 0.000 |
| 64000 | 7 | 0 | 1029 | 1029 | 0.000 |
| 66000 | 7 | 0 | 1061 | 1061 | 0.000 |
| 68000 | 7 | 0 | 1095 | 1095 | 0.000 |
| 70000 | 7 | 0 | 1129 | 1129 | 0.000 |
| 72000 | 7 | 0 | 1161 | 1161 | 0.000 |
| 74000 | 7 | 0 | 1195 | 1195 | 0.000 |
| 76000 | 7 | 0 | 1229 | 1229 | 0.000 |
| 78000 | 7 | 0 | 1261 | 1261 | 0.000 |
| 80000 | 7 | 0 | 1295 | 1295 | 0.000 |
| 82000 | 7 | 0 | 1329 | 1329 | 0.000 |
| 84000 | 7 | 0 | 1361 | 1361 | 0.000 |
| 86000 | 7 | 0 | 1395 | 1395 | 0.000 |
| 88000 | 7 | 0 | 1429 | 1429 | 0.000 |
| 90000 | 7 | 0 | 1461 | 1461 | 0.000 |
| 92000 | 7 | 0 | 1495 | 1495 | 0.000 |
| 94000 | 7 | 0 | 1529 | 1529 | 0.000 |
| 96000 | 7 | 0 | 1561 | 1561 | 0.000 |
| 98000 | 7 | 0 | 1595 | 1595 | 0.000 |
| 100000 | 7 | 0 | 1629 | 1629 | 0.000 |
| 102000 | 7 | 0 | 1661 | 1661 | 0.000 |
| 104000 | 7 | 0 | 1695 | 1695 | 0.000 |
| 106000 | 7 | 0 | 1729 | 1729 | 0.000 |
| 108000 | 7 | 0 | 1761 | 1761 | 0.000 |
| 110000 | 7 | 0 | 1795 | 1795 | 0.000 |
| 112000 | 7 | 0 | 1829 | 1829 | 0.000 |
| 114000 | 7 | 0 | 1861 | 1861 | 0.000 |
| 116000 | 7 | 0 | 1895 | 1895 | 0.000 |
| 118000 | 7 | 0 | 1929 | 1929 | 0.000 |
| 120000 | 7 | 0 | 1961 | 1961 | 0.000 |
| 122000 | 7 | 0 | 1986 | 1986 | 0.000 |
| 124000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 126000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 128000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 130000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 132000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 134000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 136000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 138000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 140000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 142000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 144000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 146000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 148000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 150000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 152000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 154000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 156000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 158000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 160000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 162000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 164000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 166000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 168000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 170000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 172000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 174000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 176000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 178000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 180000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 182000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 184000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 186000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 188000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 190000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 192000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 194000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 196000 | 7 | 0 | 2000 | 2000 | 0.000 |
| 198000 | 7 | 0 | 2000 | 2000 | 0.000 |

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

## Injection Events

| Tick | Entry | Expected | Per-text | Detected | Coherence | Reflex | Passes | Exp traces |
|---|---|---|---|---|---|---|---|---|
| 0 | math_arithmetic | mathematics | ✓ mathematics | — | 1.00 | — | 1 | 1 |
| 0 | abstract_infinity | abstractions | mathematics | — | 1.00 | — | 1 | 1 |
| 0 | logic_deductive | logic | ✓ logic | — | 1.00 | — | 1 | 1 |
| 0 | writing_narrative | writing | ✓ writing | — | 1.00 | — | 1 | 1 |
| 300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 1 |
| 300 | abstract_infinity | abstractions | mathematics | 0 | 1.00 | ✓ | 1 | 1 |
| 300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 1 |
| 500 | music_harmony | music | ✓ music | 0 | 1.00 | — | 1 | 2 |
| 500 | morality_duty | morality | logic | 0 | 1.00 | ✓ | 1 | 3 |
| 600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 1 |
| 600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 3 |
| 600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 1 |
| 800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 3 |
| 800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 3 |
| 900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 1 |
| 900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 3 |
| 900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 1 |
| 1000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 2 |
| 1100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 1100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 1200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 1 |
| 1200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 1200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 2 |
| 1300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 2 |
| 1400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 1400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 1500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 1 |
| 1500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 1500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 1500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 2 |
| 1500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 3 |
| 1600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 3 |
| 1700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 1700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 1800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 2 |
| 1800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 1800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 1 |
| 1800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 3 |
| 1800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 3 |
| 1900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 1900 | values_honesty | values | ✓ values | 3 | 1.00 | ✓ | 1 | 3 |
| 2000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 2000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 2 |
| 2000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 3 |
| 2000 | logic_inductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 4 |
| 2100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 2 |
| 2100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 2100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 3 |
| 2100 | writing_narrative | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 4 |
| 2100 | time_perception | time | ✓ time | 3 | 1.00 | ✓ | 1 | 4 |
| 2200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 2200 | values_honesty | values | ✓ values | 3 | 1.00 | ✓ | 1 | 4 |
| 2300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 2300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 3 |
| 2300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 3 |
| 2300 | logic_inductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 4 |
| 2400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 2 |
| 2400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 2400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 3 |
| 2400 | writing_narrative | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 4 |
| 2400 | time_perception | time | ✓ time | 3 | 1.00 | ✓ | 1 | 4 |
| 2500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 2500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 4 |
| 2500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 2500 | values_honesty | values | ✓ values | 3 | 1.00 | ✓ | 1 | 4 |
| 2600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 2600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 2600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 2600 | logic_inductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 4 |
| 2700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 2 |
| 2700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 2700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 2700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 2700 | writing_narrative | writing | ✓ writing | 3 | 1.00 | ✓ | 1 | 4 |
| 2700 | time_perception | time | ✓ time | 3 | 1.00 | ✓ | 1 | 4 |
| 2800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 4 |
| 2800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 2800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 2800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 2900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 4 |
| 2900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 4 |
| 2900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 2900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 2900 | logic_inductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 4 |
| 3000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 2 |
| 3000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 2 |
| 3000 | time_arrow | time | ✓ time | 1 | 0.75 | — | 1 | 3 |
| 3000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 3000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 3000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 3100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 3100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 3100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 3200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 3200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 3200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 3300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 3 |
| 3300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 3 |
| 3300 | time_arrow | time | ✓ time | 1 | 1.00 | — | 1 | 4 |
| 3300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 3300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 3300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 3400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 3400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 3400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 3500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 5 |
| 3500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 3500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 3500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 3600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 5 |
| 3600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 5 |
| 3600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 6 |
| 3600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 3600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 3600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 3700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 3700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 3700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 3800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 6 |
| 3800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 3800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 3800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 3800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 3900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 6 |
| 3900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 6 |
| 3900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 6 |
| 3900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 3900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 3900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 3900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 3900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 4000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 4000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 4000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 4100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 4100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 4100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 4100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 4100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 4200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 4200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 4200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 4200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 4200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 4200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 4200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 4300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 4300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 4300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 4400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 4400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 4400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 4400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 4400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 4500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 4500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 4500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 4500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 4500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 4500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 4600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 4600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 4600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 4700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 4700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 4700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 4700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 4700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 4800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 4800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 4800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 4800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 4800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 4800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 4900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 4900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 4900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 4900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 4900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 5000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 5000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 5000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 5000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 5100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 5100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 5100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 5100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 5100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 5100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 5200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 5200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 5200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 5200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 5300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 5300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 5300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 5300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 5400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 5400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 5400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 5400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 5400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 5400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 5400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 5500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 5500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 5500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 5500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 5600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 5600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 5600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 5600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 5700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 5700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 5700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 5700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 5700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 5700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 5800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 5800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 5800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 5800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 5900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 5900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 5900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 5900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 5900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 5900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 6000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 6000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 6000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 6000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 6000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 6000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 6100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 6100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 6100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 6200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 6200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 6200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 6200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 6200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 6300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 6300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 6300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 6300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 6300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 6300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 6400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 6400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 6400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 6500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 6500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 6500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 6500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 6500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 6600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 6600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 6600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 6600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 6600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 6600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 6700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 6700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 6700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 6800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 6800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 6800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 6800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 6800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 6900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 6900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 6900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 6900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 6900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 6900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 6900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 6900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 7000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 7000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 7000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 7100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 7100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 7100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 7100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 7100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 7200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 7200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 7200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 7200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 7200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 7200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 7300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 7300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 7300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 7400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 7400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 7400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 7400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 7400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 7500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 7500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 7500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 7500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 7500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 7500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 7500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 7600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 7600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 7600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 7700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 7700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 7700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 7700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 7700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 7800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 7800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 7800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 7800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 7800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 7800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 7900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 7900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 7900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 7900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 7900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 8000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 8000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 8000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 8000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 8000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 8100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 8100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 8100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 8100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 8100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 8100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 8200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 8200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 8200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 8300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 8300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 8300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 8300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 8300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 8400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 8400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 8400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 8400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 8400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 8400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 8500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 8500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 8500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 8600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 8600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 8600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 8600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 8600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 8700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 8700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 8700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 8700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 8700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 8700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 8800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 8800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 8800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 8800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 8900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 8900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 8900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 8900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 8900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 8900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 9000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 9000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 9000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 9000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 9000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 9000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 9100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 9100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 9100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 9200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 9200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 9200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 9200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 9200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 9300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 9300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 9300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 9300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 9300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 9300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 9400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 9400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 9400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 9500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 9500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 9500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 9500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 9500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 9600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 9600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 9600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 9600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 9600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 9600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 9700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 9700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 9700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 9700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 9800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 9800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 9800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 9800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 9800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 9900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 9900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 9900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 9900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 9900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 9900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 9900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 9900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 10000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 10000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 10000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 10000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 10100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 10100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 10100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 10100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 10100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 10200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 10200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 10200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 10200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 10200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 10200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 10200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 10300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 10300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 10300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 10300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 10400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 10400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 10400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 10400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 10400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 10500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 10500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 10500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 10500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 10500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 10500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 10500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 10600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 10600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 10600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 10600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 10700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 10700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 10700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 10700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 10700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 10800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 10800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 10800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 10800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 10800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 10800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 10800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 10900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 10900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 10900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 10900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 10900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 11000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 11000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 11000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 11000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 11000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 11100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 11100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 11100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 11100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 11100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 11100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 11100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 11200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 11200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 11200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 11200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 11300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 11300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 11300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 11300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 11300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 11400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 11400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 11400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 11400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 11400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 11400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 11400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 11500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 11500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 11500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 11500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 11600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 11600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 11600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 11600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 11600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 11700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 11700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 11700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 11700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 11700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 11700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 11700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 11800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 11800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 11800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 11800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 11900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 11900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 11900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 11900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 11900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 11900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 12000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 12000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 12000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 12000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 12000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 12000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 12100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 12100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 12100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 12200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 12200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 12200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 12200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 12200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 12300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 12300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 12300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 12300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 12300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 12300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 12300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 12400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 12400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 12400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 12500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 12500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 12500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 12500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 12500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 12600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 12600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 12600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 12600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 12600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 12600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 12700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 12700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 12700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 12800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 12800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 12800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 12800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 12800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 12900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 12900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 12900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 12900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 12900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 12900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 12900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 12900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 13000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 13000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 13000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 13000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 13100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 13100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 13100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 13100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 13100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 13200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 13200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 13200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 13200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 13200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 13200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 13200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 13300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 13300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 13300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 13300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 13400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 13400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 13400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 13400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 13400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 13500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 13500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 13500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 13500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 13500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 13500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 13500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 13600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 13600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 13600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 13600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 13700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 13700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 13700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 13700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 13700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 13800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 13800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 13800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 13800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 13800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 13800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 13800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 13900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 13900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 13900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 13900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 13900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 14000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 14000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 14000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 14000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 14000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 14100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 14100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 14100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 14100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 14100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 14100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 14100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 14200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 14200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 14200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 14200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 14300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 14300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 14300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 14300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 14300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 14400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 14400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 14400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 14400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 14400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 14400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 14400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 14500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 14500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 14500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 14500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 14600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 14600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 14600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 14600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 14600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 14700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 14700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 14700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 14700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 14700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 14700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 14700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 14800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 14800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 14800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 14800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 14900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 14900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 14900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 14900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 14900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 14900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 15000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 15000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 15000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 15000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 15000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 15000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 15000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 15100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 15100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 15100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 15100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 15200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 15200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 15200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 15200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 15200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 15300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 15300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 15300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 15300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 15300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 15300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 15300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 15400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 15400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 15400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 15400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 15500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 15500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 15500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 15500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 15500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 15600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 15600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 15600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 15600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 15600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 15600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 15600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 15700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 15700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 15700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 15700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 15800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 15800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 15800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 15800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 15800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 15900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 15900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 15900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 15900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 15900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 15900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 15900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 15900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 16000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 16000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 16000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 16000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 16100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 16100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 16100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 16100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 16100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 16200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 16200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 16200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 16200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 16200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 16200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 16200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 16300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 16300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 16300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 16300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 16400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 16400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 16400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 16400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 16400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 16500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 16500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 16500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 16500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 16500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 16500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 16500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 16600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 16600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 16600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 16600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 16700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 16700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 16700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 16700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 16700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 16800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 16800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 16800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 16800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 16800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 16800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 16800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 16900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 16900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 16900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 16900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 16900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 17000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 17000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 17000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 17000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 17000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 17100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 17100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 17100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 17100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 17100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 17100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 17200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 17200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 17200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 17300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 17300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 17300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 17300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 17300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 17400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 17400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 17400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 17400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 17400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 17400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 17500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 17500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 17500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 17600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 17600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 17600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 17600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 17600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 17700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 17700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 17700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 17700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 17700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 17700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 17800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 17800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 17800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 17800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 17900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 17900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 17900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 17900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 17900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 17900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 18000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 18000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 18000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 18000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 18000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 18000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 18100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 18100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 18100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 18200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 18200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 18200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 18200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 18200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 18300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 18300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 18300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 18300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 18300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 18300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 18400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 18400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 18400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 18500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 18500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 18500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 18500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 18500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 18600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 18600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 18600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 18600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 18600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 18600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 18600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 18700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 18700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 18700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 18800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 18800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 18800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 18800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 18800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 18900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 18900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 18900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 18900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 18900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 18900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 18900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 18900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 19000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 19000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 19000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 19000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 19100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 19100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 19100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 19100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 19200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 19200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 19200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 19200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 19200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 19200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 19300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 19300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 19300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 19300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 19400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 19400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 19400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 19400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 19500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 19500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 19500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 19500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 19500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 19500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 19600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 19600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 19600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 19600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 19700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 19700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 19700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 19700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 19800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 19800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 19800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 19800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 19800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 19800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 19800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 19900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 19900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 19900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 19900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 19900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 20000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 20000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 20000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 20000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 20000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 20100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 20100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 20100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 20100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 20100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 20100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 20200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 20200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 20200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 20300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 20300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 20300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 20300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 20300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 20400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 20400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 20400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 20400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 20400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 20400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 20500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 20500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 20500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 20600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 20600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 20600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 20600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 20600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 20700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 20700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 20700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 20700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 20700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 20700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 20800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 20800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 20800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 20800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 20900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 20900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 20900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 20900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 20900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 20900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 21000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 21000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 21000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 21000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 21000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 21000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 21000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 21100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 21100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 21100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 21100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 21200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 21200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 21200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 21200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 21200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 21300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 21300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 21300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 21300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 21300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 21300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 21300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 21400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 21400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 21400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 21400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 21500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 21500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 21500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 21500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 21500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 21600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 21600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 21600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 21600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 21600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 21600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 21600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 21700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 21700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 21700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 21700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 21800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 21800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 21800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 21800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 21800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 21900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 21900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 21900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 21900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 21900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 21900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 21900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 21900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 22000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 22000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 22000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 22100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 22100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 22100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 22100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 22100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 22200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 22200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 22200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 22200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 22200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 22200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 22300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 22300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 22300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 22400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 22400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 22400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 22400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 22400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 22500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 22500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 22500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 22500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 22500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 22500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 22600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 22600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 22600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 22700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 22700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 22700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 22700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 22700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 22800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 22800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 22800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 22800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 22800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 22800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 22900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 22900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 22900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 22900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 22900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 23000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 23000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 23000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 23000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 23100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 23100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 23100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 23100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 23100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 23100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 23200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 23200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 23200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 23200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 23300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 23300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 23300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 23300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 23400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 23400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 23400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 23400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 23400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 23400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 23400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 23500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 23500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 23500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 23500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 23600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 23600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 23600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 23600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 23700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 23700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 23700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 23700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 23700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 23700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 23800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 23800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 23800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 23800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 23900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 23900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 23900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 23900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 23900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 23900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 24000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 24000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 24000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 24000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 24000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 24000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 24100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 24100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 24100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 24200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 24200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 24200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 24200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 24200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 24300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 24300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 24300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 24300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 24300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 24300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 24400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 24400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 24400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 24500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 24500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 24500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 24500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 24500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 24600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 24600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 24600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 24600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 24600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 24600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 24700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 24700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 24700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 24800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 24800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 24800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 24800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 24800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 24900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 24900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 24900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 24900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 24900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 24900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 24900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 24900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 25000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 25000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 25000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 25100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 25100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 25100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 25100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 25100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 25200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 25200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 25200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 25200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 25200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 25200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 25300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 25300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 25300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 25400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 25400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 25400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 25400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 25400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 25500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 25500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 25500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 25500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 25500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 25500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 25600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 25600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 25600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 25700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 25700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 25700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 25700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 25700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 25800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 25800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 25800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 25800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 25800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 25800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 25900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 25900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 25900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 25900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 25900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 26000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 26000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 26000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 26000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 26000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 26100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 26100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 26100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 26100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 26100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 26100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 26100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 26200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 26200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 26200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 26200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 26300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 26300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 26300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 26300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 26300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 26400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 26400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 26400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 26400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 26400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 26400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 26400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 26500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 26500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 26500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 26500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 26600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 26600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 26600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 26600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 26600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 26700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 26700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 26700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 26700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 26700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 26700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 26700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 26800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 26800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 26800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 26800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 26900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 26900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 26900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 26900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 26900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 26900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 27000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 27000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 27000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 27000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 27000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 27000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 27100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 27100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 27100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 27100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 27200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 27200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 27200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 27200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 27300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 27300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 27300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 27300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 27300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 27300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 27400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 27400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 27400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 27400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 27500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 27500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 27500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 27500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 27600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 27600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 27600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 27600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 27600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 27600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 27600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 27700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 27700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 27700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 27700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 27800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 27800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 27800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 27800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 27900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 27900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 27900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 27900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 27900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 27900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 27900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 27900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 28000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 28000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 28000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 28000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 28100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 28100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 28100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 28100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 28100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 28200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 28200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 28200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 28200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 28200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 28200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 28200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 28300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 28300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 28300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 28300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 28400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 28400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 28400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 28400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 28400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 28500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 28500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 28500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 28500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 28500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 28500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 28500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 28600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 28600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 28600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 28600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 28700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 28700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 28700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 28700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 28700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 28800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 28800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 28800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 28800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 28800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 28800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 28800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 28900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 28900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 28900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 28900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 28900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 29000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 29000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 29000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 29000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 29000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 29100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 29100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 29100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 29100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 29100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 29100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 29100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 29200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 29200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 29200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 29200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 29300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 29300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 29300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 29300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 29300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 29400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 29400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 29400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 29400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 29400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 29400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 29400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 29500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 29500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 29500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 29500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 29600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 29600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 29600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 29600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 29600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 29700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 29700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 29700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 29700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 29700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 29700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 29700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 29800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 29800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 29800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 29800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 29900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 29900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 29900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 29900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 29900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 29900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 30000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 30000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 30000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 30000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 30000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 30000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 30100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 30100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 30100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 30100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 30200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 30200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 30200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 30200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 30300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 30300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 30300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 30300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 30300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 30300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 30300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 30400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 30400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 30400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 30400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 30500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 30500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 30500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 30500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 30600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 30600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 30600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 30600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 30600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 30600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 30700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 30700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 30700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 30700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 30800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 30800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 30800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 30800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 30900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 30900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 30900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 30900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 30900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 30900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 30900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 30900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 31000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 31000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 31000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 31100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 31100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 31100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 31100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 31100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 31200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 31200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 31200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 31200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 31200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 31200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 31300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 31300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 31300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 31400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 31400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 31400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 31400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 31400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 31500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 31500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 31500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 31500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 31500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 31500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 31500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 31600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 31600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 31600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 31700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 31700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 31700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 31700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 31700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 31800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 31800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 31800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 31800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 31800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 31800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 31900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 31900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 31900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 31900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 31900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 32000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 32000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 32000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 32000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 32100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 32100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 32100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 32100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 32100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 32100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 32200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 32200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 32200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 32200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 32300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 32300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 32300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 32300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 32400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 32400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 32400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 32400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 32400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 32400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 32500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 32500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 32500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 32500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 32600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 32600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 32600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 32600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 32700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 32700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 32700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 32700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 32700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 32700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 32700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 32800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 32800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 32800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 32800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 32900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 32900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 32900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 32900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 32900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 32900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 33000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 33000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 33000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 33000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 33000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 33000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 33100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 33100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 33100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 33200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 33200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 33200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 33200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 33200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 33300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 33300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 33300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 33300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 33300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 33300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 33400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 33400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 33400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 33500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 33500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 33500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 33500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 33500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 33600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 33600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 33600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 33600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 33600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 33600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 33700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 33700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 33700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 33800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 33800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 33800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 33800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 33800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 33900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 33900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 33900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 33900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 33900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 33900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 33900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 33900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 34000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 34000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 34000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 34000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 34100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 34100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 34100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 34100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 34100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 34200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 34200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 34200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 34200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 34200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 34200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 34200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 34300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 34300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 34300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 34300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 34400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 34400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 34400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 34400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 34400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 34500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 34500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 34500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 34500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 34500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 34500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 34500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 34600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 34600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 34600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 34600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 34700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 34700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 34700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 34700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 34700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 34800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 34800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 34800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 34800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 34800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 34800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 34800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 34900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 34900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 34900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 34900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 34900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 35000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 35000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 35000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 35000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 35000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 35100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 35100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 35100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 35100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 35100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 35100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 35200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 35200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 35200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 35300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 35300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 35300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 35300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 35300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 35400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 35400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 35400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 35400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 35400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 35400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 35500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 35500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 35500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 35600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 35600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 35600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 35600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 35600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 35700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 35700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 35700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 35700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 35700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 35700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 35800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 35800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 35800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 35800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 35900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 35900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 35900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 35900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 35900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 35900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 36000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 36000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 36000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 36000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 36000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 36000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 36100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 36100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 36100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 36200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 36200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 36200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 36200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 36200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 36300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 36300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 36300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 36300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 36300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 36300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 36400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 36400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 36400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 36500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 36500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 36500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 36500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 36500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 36600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 36600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 36600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 36600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 36600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 36600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 36600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 36700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 36700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 36700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 36800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 36800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 36800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 36800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 36800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 36900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 36900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 36900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 36900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 36900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 36900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 36900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 36900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 37000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 37000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 37000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 37100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 37100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 37100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 37100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 37100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 37200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 37200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 37200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 37200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 37200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 37200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 37300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 37300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 37300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 37400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 37400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 37400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 37400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 37400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 37500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 37500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 37500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 37500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 37500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 37500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 37600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 37600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 37600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 37700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 37700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 37700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 37700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 37700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 37800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 37800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 37800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 37800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 37800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 37800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 37800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 37900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 37900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 37900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 37900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 37900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 38000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 38000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 38000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 38000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 38000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 38100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 38100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 38100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 38100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 38100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 38100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 38200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 38200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 38200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 38300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 38300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 38300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 38300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 38300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 38400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 38400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 38400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 38400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 38400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 38400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 38500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 38500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 38500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 38600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 38600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 38600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 38600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 38600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 38700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 38700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 38700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 38700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 38700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 38700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 38800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 38800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 38800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 38800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 38900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 38900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 38900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 38900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 38900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 38900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 39000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 39000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 39000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 39000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 39000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 39000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 39000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 39100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 39100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 39100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 39100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 39200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 39200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 39200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 39200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 39200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 39300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 39300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 39300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 39300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 39300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 39300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 39300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 39400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 39400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 39400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 39400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 39500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 39500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 39500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 39500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 39500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 39600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 39600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 39600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 39600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 39600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 39600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 39600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 39700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 39700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 39700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 39700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 39800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 39800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 39800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 39800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 39800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 39900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 39900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 39900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 39900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 39900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 39900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 39900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 39900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 40000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 40000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 40000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 40000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 40100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 40100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 40100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 40100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 40100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 40200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 40200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 40200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 40200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 40200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 40200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 40200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 40300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 40300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 40300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 40300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 40400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 40400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 40400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 40400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 40400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 40500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 40500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 40500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 40500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 40500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 40500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 40500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 40600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 40600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 40600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 40600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 40700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 40700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 40700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 40700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 40700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 40800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 40800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 40800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 40800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 40800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 40800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 40800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 40900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 40900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 40900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 40900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 40900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 41000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 41000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 41000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 41000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 41000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 41100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 41100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 41100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 41100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 41100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 41100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 41100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 41200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 41200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 41200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 41200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 41300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 41300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 41300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 41300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 41300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 41400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 41400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 41400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 41400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 41400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 41400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 41400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 41500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 41500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 41500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 41500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 41600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 41600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 41600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 41600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 41600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 41700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 41700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 41700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 41700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 41700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 41700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 41700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 41800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 41800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 41800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 41800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 41900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 41900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 41900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 41900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 41900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 41900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 42000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 42000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 42000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 42000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 42000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 42000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 42000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 42100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 42100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 42100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 42200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 42200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 42200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 42200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 42200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 42300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 42300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 42300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 42300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 42300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 42300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 42400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 42400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 42400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 42500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 42500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 42500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 42500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 42500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 42600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 42600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 42600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 42600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 42600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 42600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 42700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 42700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 42700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 42800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 42800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 42800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 42800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 42800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 42900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 42900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 42900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 42900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 42900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 42900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 42900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 42900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 43000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 43000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 43000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 43100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 43100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 43100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 43100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 43100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 43200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 43200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 43200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 43200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 43200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 43200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 43200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 43300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 43300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 43300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 43400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 43400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 43400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 43400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 43400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 43500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 43500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 43500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 43500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 43500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 43500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 43600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 43600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 43600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 43700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 43700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 43700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 43700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 43700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 43800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 43800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 43800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 43800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 43800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 43800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 43900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 43900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 43900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 43900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 43900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 44000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 44000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 44000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 44000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 44100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 44100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 44100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 44100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 44100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44100 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 44100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 44200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 44200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 44200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 44200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 44300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 44300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 44300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 44300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 44400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 44400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 44400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 44400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 44400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 44400 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 44400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 44500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 44500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 44500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 44500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 44600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 44600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 44600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 44600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 44700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 44700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 44700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 44700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 44700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44700 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 44700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 44800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 44800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 44800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 44800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 44900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 44900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 44900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 44900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 44900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 44900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 45000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 45000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 45000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 45000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 45000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45000 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 45000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 45100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 45100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 45100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 45100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 45200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 45200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 45200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 45200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 45300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 45300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 45300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 45300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 45300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45300 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 45300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 45400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 45400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 45400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 45400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 45500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 45500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 45500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 45500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 45600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 45600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 45600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 45600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 45600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45600 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 45600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 45700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 45700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 45700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 45700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 45800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 45800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 45800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 45800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 45800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 45900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 45900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 45900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 45900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 45900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 45900 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 45900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 45900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 46000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 46000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 46000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 46100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 46100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 46100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 46100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 46100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 46200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 46200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 46200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 46200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 46200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46200 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 46200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 46300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 46300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 46300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 46400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 46400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 46400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 46400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 46400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 46500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 46500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 46500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 46500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 46500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46500 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 46500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 46600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 46600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 46600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 46700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 46700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 46700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 46700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 46700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 46800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 46800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 46800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 46800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 46800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 46800 | logic_deductive | logic | ✓ logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 46800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 46900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 46900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 46900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 46900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 46900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 47000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 47000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 47000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 47000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 47000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 47100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 47100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 47100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 47100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 47100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 47100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 47100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 47200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 47200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 47200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 47300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 47300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 47300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 47300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 47300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 47400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 47400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 47400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 47400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 47400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 47400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 47400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 47500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 47500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 47500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 47600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 47600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 47600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 47600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 47600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 47700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 47700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 47700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 47700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 47700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 47700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 47700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 47800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 47800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 47800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 47800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 47900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 47900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 47900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 47900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 47900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 47900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 48000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 48000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 48000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 48000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 48000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 48000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 48000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 48100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 48100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 48100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 48200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 48200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 48200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 48200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 48200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 48300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 48300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 48300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 48300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 48300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 48300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 48300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 48400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 48400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 48400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 48500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 48500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 48500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 48500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 48500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 48600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 48600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 48600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 48600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 48600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 48600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 48600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 48700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 48700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 48700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 48800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 48800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 48800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 48800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 48800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 48800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 48900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 48900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 48900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 48900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 48900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 48900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 48900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 48900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 49000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 49000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 49000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 49100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 49100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 49100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 49100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 49100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 49200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 49200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 49200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 49200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 49200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 49200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 49200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 49300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 49300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 49300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 49400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 49400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 49400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 49400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 49400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 49500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 49500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 49500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 49500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 49500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 49500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 49500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 49600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 49600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 49600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 49700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 49700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 49700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 49700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 49700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 49800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 49800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 49800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 49800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 49800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 49800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 49800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 49800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 49900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 49900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 49900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 49900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 49900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 50000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 50000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 50000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 50000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 50000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 50100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 50100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 50100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 50100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 50100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 50100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 50100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 50200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 50200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 50200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 50300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 50300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 50300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 50300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 50300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 50400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 50400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 50400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 50400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 50400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 50400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 50400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 50500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 50500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 50500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 50600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 50600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 50600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 50600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 50600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 50700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 50700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 50700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 50700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 50700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 50700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 50700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 50800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 50800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 50800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 50800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 50900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 50900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 50900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 50900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 50900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 50900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 51000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 51000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 51000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 51000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 51000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 51000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 51000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 51000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 51100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 51100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 51100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 51100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 51200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 51200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 51200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 51200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 51200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 51300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 51300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 51300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 51300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 51300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 51300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 51300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 51300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 51400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 51400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 51400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 51400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 51500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 51500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 51500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 51500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 51500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 51600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 51600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 51600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 51600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 51600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 51600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 51600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 51600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 51700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 51700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 51700 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 51700 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 51800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 51800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 51800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 51800 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 51800 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 51800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 51900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 51900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 51900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 51900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 51900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 51900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 51900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 51900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 52000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 52000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 52000 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 52000 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 52100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 52100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 52100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 52100 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52100 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 52100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 52200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 52200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 52200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 52200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 52200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 52200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 52200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 52200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 52300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 52300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 52300 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 52300 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 52400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 52400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 52400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 52400 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52400 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 52400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 52500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 52500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 52500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 52500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 52500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 52500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 52500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 52500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 52600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 52600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 52600 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 52600 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 52700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 52700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 52700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 52700 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52700 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 52700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 52800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 52800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 52800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 52800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 52800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 52800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 52800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 52800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 52900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 52900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 52900 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 52900 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 52900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 53000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 53000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 53000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53000 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53000 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 53000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 53100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 53100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 53100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 53100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 53100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 53100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 53100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 53200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 53200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 53200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 53200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 53300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 53300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 53300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53300 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53300 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 53300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 53400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 53400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 53400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 53400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 53400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 53400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 53400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 53500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 53500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 53500 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 53500 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 53600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 53600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 53600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53600 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53600 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 53600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 53700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 53700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 53700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 53700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 53700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 53700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 53700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 53700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 53800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 53800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 53800 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 53800 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 53900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 53900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 53900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 53900 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 53900 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 53900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 54000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 54000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 54000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 54000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 54000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 54000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 54100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 54100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 54100 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 54100 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 54100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 54200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 54200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 54200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54200 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 54200 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 54200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 54300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 54300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 54300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 54300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 54300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 54300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 54300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 54400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 54400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 54400 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 54400 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 54400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 54500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 54500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 54500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54500 | math_geometry | mathematics | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 54500 | morality_consequences | morality | values | 4 | 1.00 | ✓ | 1 | 5 |
| 54500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 54600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 54600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 54600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 54600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 54600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 54600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 54700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 54700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 54700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 54700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 54800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 54800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 54800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 54800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 54900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 54900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 54900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 54900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 54900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 54900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 54900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 54900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 55000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 55000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 55000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 55000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 55100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 55100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 55100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 55100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 55100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 55200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 55200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 55200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 55200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 55200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 55200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 55200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 55300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 55300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 55300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 55300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 55400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 55400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 55400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 55400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 55400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 55500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 55500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 55500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 55500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 55500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 55500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 55500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 55600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 55600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 55600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 55600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 55700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 55700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 55700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 55700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 55700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 55800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 55800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 55800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 55800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 55800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 55800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 55800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 55900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 55900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 55900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 55900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 55900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 56000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 56000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 56000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 56000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 56000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 56100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 56100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 56100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 56100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 56100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 56100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 56100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 56200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 56200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 56200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 56200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 56300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 56300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 56300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 56300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 56300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 56400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 56400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 56400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 56400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 56400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 56400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 56400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 56500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 56500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 56500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 56500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 56600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 56600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 56600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 56600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 56600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 56700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 56700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 56700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 56700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 56700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 56700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 56700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 56800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 56800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 56800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 56800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 56900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 56900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 56900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 56900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 56900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 56900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 57000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 57000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 57000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 57000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 57000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 57000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 57000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 57100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 57100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 57100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 57200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 57200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 57200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 57200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 57200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 57300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 57300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 57300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 57300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 57300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 57300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 57400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 57400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 57400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 57500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 57500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 57500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 57500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 57500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 57600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 57600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 57600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 57600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 57600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 57600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 57700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 57700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 57700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 57800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 57800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 57800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 57800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 57800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 57900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 57900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 57900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 57900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 57900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 57900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 57900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 57900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 58000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 58000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 58000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 58100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 58100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 58100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 58100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 58100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 58200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 58200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 58200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 58200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 58200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 58200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 58300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 58300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 58300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 58400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 58400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 58400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 58400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 58400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 58500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 58500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 58500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 58500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 58500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 58500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 58600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 58600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 58600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 58700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 58700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 58700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 58700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 58700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 58800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 58800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 58800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 58800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 58800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 58800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 58900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 58900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 58900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 58900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 58900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 59000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 59000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 59000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 59000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 59000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 59100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 59100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 59100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 59100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 59100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 59100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 59100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 59200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 59200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 59200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 59200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 59300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 59300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 59300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 59300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 59300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 59400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 59400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 59400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 59400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 59400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 59400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 59400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 59500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 59500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 59500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 59500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 59600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 59600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 59600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 59600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 59600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 59700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 59700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 59700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 59700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 59700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 59700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 59700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 59800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 59800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 59800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 59800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 59900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 59900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 59900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 59900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 59900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 59900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 60000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 60000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 60000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 60000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 60000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 60000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 60000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 60100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 60100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 60100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 60200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 60200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 60200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 60200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 60200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 60300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 60300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 60300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 60300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 60300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 60300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 60400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 60400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 60400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 60500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 60500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 60500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 60500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 60500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 60600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 60600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 60600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 60600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 60600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 60600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 60700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 60700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 60700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 60800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 60800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 60800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 60800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 60800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 60900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 60900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 60900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 60900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 60900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 60900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 60900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 60900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 61000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 61000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 61000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 61100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 61100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 61100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 61100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 61100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 61200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 61200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 61200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 61200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 61200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 61200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 61300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 61300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 61300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 61400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 61400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 61400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 61400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 61400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 61500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 61500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 61500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 61500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 61500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 61500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 61500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 61600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 61600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 61600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 61700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 61700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 61700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 61700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 61700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 61800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 61800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 61800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 61800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 61800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 61800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 61900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 61900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 61900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 61900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 61900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 62000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 62000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 62000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 62000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 62100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 62100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 62100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 62100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 62100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 62100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 62200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 62200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 62200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 62200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 62300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 62300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 62300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 62300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 62400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 62400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 62400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 62400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 62400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 62400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 62500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 62500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 62500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 62500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 62600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 62600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 62600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 62600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 62700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 62700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 62700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 62700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 62700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 62700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 62800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 62800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 62800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 62800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 62900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 62900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 62900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 62900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 62900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 62900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 63000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 63000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 63000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 63000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 63000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 63000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 63100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 63100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 63100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 63200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 63200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 63200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 63200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 63200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 63300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 63300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 63300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 63300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 63300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 63300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 63400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 63400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 63400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 63500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 63500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 63500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 63500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 63500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 63600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 63600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 63600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 63600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 63600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 63600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 63700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 63700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 63700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 63800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 63800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 63800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 63800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 63800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 63900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 63900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 63900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 63900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 63900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 63900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 63900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 63900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 64000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 64000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 64000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 64000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 64100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 64100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 64100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 64100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 64100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 64200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 64200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 64200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 64200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 64200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 64200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 64200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 64300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 64300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 64300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 64300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 64400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 64400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 64400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 64400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 64400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 64500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 64500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 64500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 64500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 64500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 64500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 64500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 64600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 64600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 64600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 64600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 64700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 64700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 64700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 64700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 64700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 64800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 64800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 64800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 64800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 64800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 64800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 64800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 64900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 64900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 64900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 64900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 64900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 65000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 65000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 65000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 65000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 65000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 65100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 65100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 65100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 65100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 65100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 65100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 65200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 65200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 65200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 65300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 65300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 65300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 65300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 65300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 65400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 65400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 65400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 65400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 65400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 65400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 65500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 65500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 65500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 65600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 65600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 65600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 65600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 65600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 65700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 65700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 65700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 65700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 65700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 65700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 65800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 65800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 65800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 65800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 65900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 65900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 65900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 65900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 65900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 65900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 66000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 66000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 66000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 66000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 66000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 66000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 66100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 66100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 66100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 66200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 66200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 66200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 66200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 66200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 66300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 66300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 66300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 66300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 66300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 66300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 66400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 66400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 66400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 66500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 66500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 66500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 66500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 66500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 66600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 66600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 66600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 66600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 66600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 66600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 66700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 66700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 66700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 66800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 66800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 66800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 66800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 66800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 66900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 66900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 66900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 66900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 66900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 66900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 66900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 66900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 67000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 67000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 67000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 67100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 67100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 67100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 67100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 67100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 67200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 67200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 67200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 67200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 67200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 67200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 67300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 67300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 67300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 67400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 67400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 67400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 67400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 67400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 67500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 67500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 67500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 67500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 67500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 67500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 67600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 67600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 67600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 67700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 67700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 67700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 67700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 67700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 67800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 67800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 67800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 67800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 67800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 67800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 67900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 67900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 67900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 67900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 67900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 68000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 68000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 68000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 68000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 68100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 68100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 68100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 68100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 68100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 68100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 68200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 68200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 68200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 68200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 68300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 68300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 68300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 68300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 68400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 68400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 68400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 68400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 68400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 68400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 68400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 68500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 68500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 68500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 68500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 68600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 68600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 68600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 68600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 68700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 68700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 68700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 68700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 68700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 68700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 68800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 68800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 68800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 68800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 68900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 68900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 68900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 68900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 68900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 68900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 69000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 69000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 69000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 69000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 69000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 69000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 69000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 69100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 69100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 69100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 69100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 69200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 69200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 69200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 69200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 69200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 69300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 69300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 69300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 69300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 69300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 69300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 69300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 69400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 69400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 69400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 69400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 69500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 69500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 69500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 69500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 69500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 69600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 69600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 69600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 69600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 69600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 69600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 69600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 69700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 69700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 69700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 69700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 69800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 69800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 69800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 69800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 69800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 69900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 69900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 69900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 69900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 69900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 69900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 69900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 69900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 70000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 70000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 70000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 70100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 70100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 70100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 70100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 70100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 70200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 70200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 70200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 70200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 70200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 70200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 70300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 70300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 70300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 70400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 70400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 70400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 70400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 70400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 70500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 70500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 70500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 70500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 70500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 70500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 70600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 70600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 70600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 70700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 70700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 70700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 70700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 70700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 70800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 70800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 70800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 70800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 70800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 70800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 70900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 70900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 70900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 70900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 70900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 71000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 71000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 71000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 71000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 71100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 71100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 71100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 71100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 71100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 71100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 71100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 71200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 71200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 71200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 71200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 71300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 71300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 71300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 71300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 71400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 71400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 71400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 71400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 71400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 71400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 71500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 71500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 71500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 71500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 71600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 71600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 71600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 71600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 71700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 71700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 71700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 71700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 71700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 71700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 71800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 71800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 71800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 71800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 71900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 71900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 71900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 71900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 71900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 71900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 72000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 72000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 72000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 72000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 72000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 72000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 72100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 72100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 72100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 72200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 72200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 72200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 72200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 72200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 72300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 72300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 72300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 72300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 72300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 72300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 72400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 72400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 72400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 72500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 72500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 72500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 72500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 72500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 72600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 72600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 72600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 72600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 72600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 72600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 72700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 72700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 72700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 72800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 72800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 72800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 72800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 72800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 72900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 72900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 72900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 72900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 72900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 72900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 72900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 72900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 73000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 73000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 73000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 73100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 73100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 73100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 73100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 73100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 73200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 73200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 73200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 73200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 73200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 73200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 73200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 73300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 73300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 73300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 73400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 73400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 73400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 73400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 73400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 73500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 73500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 73500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 73500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 73500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 73500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 73600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 73600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 73600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 73700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 73700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 73700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 73700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 73700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 73800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 73800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 73800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 73800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 73800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 73800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 73900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 73900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 73900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 73900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 73900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 74000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 74000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 74000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 74000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 74000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 74100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 74100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 74100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 74100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 74100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 74100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 74100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 74200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 74200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 74200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 74200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 74300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 74300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 74300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 74300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 74300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 74400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 74400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 74400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 74400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 74400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 74400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 74400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 74500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 74500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 74500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 74500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 74600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 74600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 74600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 74600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 74600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 74700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 74700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 74700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 74700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 74700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 74700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 74700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 74800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 74800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 74800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 74800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 74900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 74900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 74900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 74900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 74900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 74900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 75000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 75000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 75000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 75000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 75000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 75000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 75100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 75100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 75100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 75100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 75200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 75200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 75200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 75200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 75300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 75300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 75300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 75300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 75300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 75300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 75300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 75400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 75400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 75400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 75400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 75500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 75500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 75500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 75500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 75600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 75600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 75600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 75600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 75600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 75600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 75700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 75700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 75700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 75700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 75800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 75800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 75800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 75800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 75900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 75900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 75900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 75900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 75900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 75900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 75900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 75900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 76000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 76000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 76000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 76100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 76100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 76100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 76100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 76100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 76200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 76200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 76200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 76200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 76200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 76200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 76200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 76300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 76300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 76300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 76400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 76400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 76400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 76400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 76400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 76500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 76500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 76500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 76500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 76500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 76500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 76600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 76600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 76600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 76700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 76700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 76700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 76700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 76700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 76800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 76800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 76800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 76800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 76800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 76800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 76900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 76900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 76900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 76900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 76900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 77000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 77000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 77000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 77000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 77000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 77100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 77100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 77100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 77100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 77100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 77100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 77100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 77200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 77200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 77200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 77200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 77300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 77300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 77300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 77300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 77300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 77400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 77400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 77400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 77400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 77400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 77400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 77400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 77500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 77500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 77500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 77500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 77600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 77600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 77600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 77600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 77600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 77700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 77700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 77700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 77700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 77700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 77700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 77700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 77800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 77800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 77800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 77800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 77900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 77900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 77900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 77900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 77900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 77900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 78000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 78000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 78000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 78000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 78000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 78000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 78100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 78100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 78100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 78100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 78200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 78200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 78200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 78200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 78300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 78300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 78300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 78300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 78300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 78300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 78300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 78400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 78400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 78400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 78400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 78500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 78500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 78500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 78500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 78600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 78600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 78600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 78600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 78600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 78600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 78700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 78700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 78700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 78700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 78800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 78800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 78800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 78800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 78900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 78900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 78900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 78900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 78900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 78900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 78900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 78900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 79000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 79000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 79000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 79100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 79100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 79100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 79100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 79100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 79200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 79200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 79200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 79200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 79200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 79200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 79200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 79300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 79300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 79300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 79400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 79400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 79400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 79400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 79400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 79500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 79500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 79500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 79500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 79500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 79500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 79600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 79600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 79600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 79700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 79700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 79700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 79700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 79700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 79800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 79800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 79800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 79800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 79800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 79800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 79900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 79900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 79900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 79900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 79900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 80000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 80000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 80000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 80000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 80000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 80100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 80100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 80100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 80100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 80100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 80100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 80200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 80200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 80200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 80300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 80300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 80300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 80300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 80300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 80400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 80400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 80400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 80400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 80400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 80400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 80500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 80500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 80500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 80600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 80600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 80600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 80600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 80600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 80700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 80700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 80700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 80700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 80700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 80700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 80800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 80800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 80800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 80800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 80900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 80900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 80900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 80900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 80900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 80900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 81000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 81000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 81000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 81000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 81000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 81000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 81000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 81100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 81100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 81100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 81100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 81200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 81200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 81200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 81200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 81300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 81300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 81300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 81300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 81300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 81300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 81400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 81400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 81400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 81400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 81500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 81500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 81500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 81500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 81600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 81600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 81600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 81600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 81600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 81600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 81700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 81700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 81700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 81700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 81800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 81800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 81800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 81800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 81900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 81900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 81900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 81900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 81900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 81900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 81900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 81900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 82000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 82000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 82000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 82100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 82100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 82100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 82100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 82100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 82200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 82200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 82200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 82200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 82200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 82200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 82200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 82300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 82300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 82300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 82400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 82400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 82400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 82400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 82400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 82500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 82500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 82500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 82500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 82500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 82500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 82600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 82600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 82600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 82700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 82700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 82700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 82700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 82700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 82800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 82800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 82800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 82800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 82800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 82800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 82900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 82900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 82900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 82900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 82900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 83000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 83000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 83000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 83000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 83100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 83100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 83100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 83100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 83100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 83100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 83200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 83200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 83200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 83200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 83300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 83300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 83300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 83300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 83400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 83400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 83400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 83400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 83400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 83400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 83500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 83500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 83500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 83500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 83600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 83600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 83600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 83600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 83700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 83700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 83700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 83700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 83700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 83700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 83700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 83800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 83800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 83800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 83800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 83900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 83900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 83900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 83900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 83900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 83900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 84000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 84000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 84000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 84000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 84000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 84000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 84000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 84100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 84100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 84100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 84100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 84200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 84200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 84200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 84200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 84200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 84300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 84300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 84300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 84300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 84300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 84300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 84300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 84400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 84400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 84400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 84400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 84500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 84500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 84500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 84500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 84500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 84600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 84600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 84600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 84600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 84600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 84600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 84600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 84700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 84700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 84700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 84700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 84800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 84800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 84800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 84800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 84800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 84900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 84900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 84900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 84900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 84900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 84900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 84900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 84900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 85000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 85000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 85000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 85100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 85100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 85100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 85100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 85100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 85200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 85200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 85200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 85200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 85200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 85200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 85300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 85300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 85300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 85400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 85400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 85400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 85400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 85400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 85500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 85500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 85500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 85500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 85500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 85500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 85500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 85600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 85600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 85600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 85700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 85700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 85700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 85700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 85700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 85800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 85800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 85800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 85800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 85800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 85800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 85900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 85900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 85900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 85900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 85900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 86000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 86000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 86000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 86000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 86100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 86100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 86100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 86100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 86100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 86100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 86100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 86200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 86200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 86200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 86200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 86300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 86300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 86300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 86300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 86400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 86400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 86400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 86400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 86400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 86400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 86500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 86500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 86500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 86500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 86600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 86600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 86600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 86600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 86700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 86700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 86700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 86700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 86700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 86700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 86800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 86800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 86800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 86800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 86900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 86900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 86900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 86900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 86900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 86900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 87000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 87000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 87000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 87000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 87000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 87000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 87000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 87100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 87100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 87100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 87100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 87200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 87200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 87200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 87200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 87300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 87300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 87300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 87300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 87300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 87300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 87400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 87400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 87400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 87400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 87500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 87500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 87500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 87500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 87600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 87600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 87600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 87600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 87600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 87600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 87700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 87700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 87700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 87700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 87800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 87800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 87800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 87800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 87900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 87900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 87900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 87900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 87900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 87900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 87900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 87900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 88000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 88000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 88000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 88000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 88100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 88100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 88100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 88100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 88200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 88200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 88200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 88200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 88200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 88200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 88300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 88300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 88300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 88300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 88400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 88400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 88400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 88400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 88500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 88500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 88500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 88500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 88500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 88500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 88600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 88600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 88600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 88600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 88700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 88700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 88700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 88700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 88800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 88800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 88800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 88800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 88800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 88800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 88800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 88900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 88900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 88900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 88900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 88900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 89000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 89000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 89000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 89000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 89000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 89100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 89100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 89100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 89100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 89100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 89100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 89100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 89200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 89200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 89200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 89200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 89300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 89300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 89300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 89300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 89300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 89400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 89400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 89400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 89400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 89400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 89400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 89400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 89500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 89500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 89500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 89500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 89600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 89600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 89600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 89600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 89600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 89700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 89700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 89700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 89700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 89700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 89700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 89700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 89800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 89800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 89800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 89800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 89900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 89900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 89900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 89900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 89900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 89900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 90000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 90000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 90000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 90000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 90000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 90000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 90000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 90100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 90100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 90100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 90200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 90200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 90200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 90200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 90200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 90300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 90300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 90300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 90300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 90300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 90300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 90400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 90400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 90400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 90500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 90500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 90500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 90500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 90500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 90600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 90600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 90600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 90600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 90600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 90600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 90700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 90700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 90700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 90800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 90800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 90800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 90800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 90800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 90900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 90900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 90900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 90900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 90900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 90900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 90900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 90900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 91000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 91000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 91000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 91000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 91100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 91100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 91100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 91100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 91100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 91200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 91200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 91200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 91200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 91200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 91200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 91200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 91300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 91300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 91300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 91300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 91400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 91400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 91400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 91400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 91400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 91500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 91500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 91500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 91500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 91500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 91500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 91500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 91600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 91600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 91600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 91600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 91700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 91700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 91700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 91700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 91700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 91800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 91800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 91800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 91800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 91800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 91800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 91800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 91900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 91900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 91900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 91900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 91900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 92000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 92000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 92000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 92000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 92000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 92100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 92100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 92100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 92100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 92100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 92100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 92100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 92200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 92200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 92200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 92200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 92300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 92300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 92300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 92300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 92300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 92400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 92400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 92400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 92400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 92400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 92400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 92400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 92500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 92500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 92500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 92500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 92600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 92600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 92600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 92600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 92600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 92700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 92700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 92700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 92700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 92700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 92700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 92700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 92800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 92800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 92800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 92800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 92900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 92900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 92900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 92900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 92900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 92900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 93000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 93000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 93000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 93000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 93000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 93000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 93100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 93100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 93100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 93200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 93200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 93200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 93200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 93200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 93300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 93300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 93300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 93300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 93300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 93300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 93400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 93400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 93400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 93500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 93500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 93500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 93500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 93500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 93600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 93600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 93600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 93600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 93600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 93600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 93700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 93700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 93700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 93800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 93800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 93800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 93800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 93800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 93900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 93900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 93900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 93900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 93900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 93900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 93900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 93900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 94000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 94000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 94000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 94100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 94100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 94100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 94100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 94100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 94200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 94200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 94200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 94200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 94200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 94200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 94300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 94300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 94300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 94400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 94400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 94400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 94400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 94400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 94500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 94500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 94500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 94500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 94500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 94500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 94600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 94600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 94600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 94700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 94700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 94700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 94700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 94700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 94800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 94800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 94800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 94800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 94800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 94800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 94900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 94900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 94900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 94900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 94900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 95000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 95000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 95000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 95000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 95100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 95100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 95100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 95100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 95100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 95100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 95100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 95200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 95200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 95200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 95200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 95300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 95300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 95300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 95300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 95400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 95400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 95400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 95400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 95400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 95400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 95500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 95500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 95500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 95500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 95600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 95600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 95600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 95600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 95700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 95700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 95700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 95700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 95700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 95700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 95800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 95800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 95800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 95800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 95900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 95900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 95900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 95900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 95900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 95900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 96000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 96000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 96000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 96000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 96000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 96000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 96000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 96100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 96100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 96100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 96100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 96200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 96200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 96200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 96200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 96200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 96300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 96300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 96300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 96300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 96300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 96300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 96300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 96400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 96400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 96400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 96400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 96500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 96500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 96500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 96500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 96500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 96600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 96600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 96600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 96600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 96600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 96600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 96600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 96700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 96700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 96700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 96700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 96800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 96800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 96800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 96800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 96800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 96900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 96900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 96900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 96900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 96900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 96900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 96900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 96900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 97000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 97000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 97000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 97100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 97100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 97100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 97100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 97100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 97200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 97200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 97200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 97200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 97200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 97200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 97300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 97300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 97300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 97400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 97400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 97400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 97400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 97400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 97500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 97500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 97500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 97500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 97500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 97500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 97600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 97600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 97600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 97700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 97700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 97700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 97700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 97700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 97800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 97800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 97800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 97800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 97800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 97800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 97800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 97900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 97900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 97900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 97900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 97900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 98000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 98000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 98000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 98000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 98000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 98100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 98100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 98100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 98100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 98100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 98100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 98100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 98200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 98200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 98200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 98200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 98300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 98300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 98300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 98300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 98300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 98400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 98400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 98400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 98400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 98400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 98400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 98400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 98500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 98500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 98500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 98500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 98600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 98600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 98600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 98600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 98600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 98700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 98700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 98700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 98700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 98700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 98700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 98700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 98800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 98800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 98800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 98800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 98900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 98900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 98900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 98900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 98900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 98900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 99000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 99000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 99000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 99000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 99000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 99000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 99100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 99100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 99100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 99200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 99200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 99200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 99200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 99200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 99300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 99300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 99300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 99300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 99300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 99300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 99400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 99400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 99400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 99500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 99500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 99500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 99500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 99500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 99600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 99600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 99600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 99600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 99600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 99600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 99600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 99700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 99700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 99700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 99800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 99800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 99800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 99800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 99800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 99900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 99900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 99900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 99900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 99900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 99900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 99900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 99900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 100000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 100000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 100000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 100100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 100100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 100100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 100100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 100100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 100200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 100200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 100200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 100200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 100200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 100200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 100300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 100300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 100300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 100400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 100400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 100400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 100400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 100400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 100500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 100500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 100500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 100500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 100500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 100500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 100600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 100600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 100600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 100700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 100700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 100700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 100700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 100700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 100800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 100800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 100800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 100800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 100800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 100800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 100900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 100900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 100900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 100900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 100900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 101000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 101000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 101000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 101000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 101100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 101100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 101100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 101100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 101100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 101100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 101100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 101200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 101200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 101200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 101200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 101300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 101300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 101300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 101300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 101400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 101400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 101400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 101400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 101400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 101400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 101500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 101500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 101500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 101500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 101600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 101600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 101600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 101600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 101700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 101700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 101700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 101700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 101700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 101700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 101800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 101800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 101800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 101800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 101900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 101900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 101900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 101900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 101900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 101900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 102000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 102000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 102000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 102000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 102000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 102000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 102000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 102100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 102100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 102100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 102100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 102200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 102200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 102200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 102200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 102300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 102300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 102300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 102300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 102300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 102300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 102400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 102400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 102400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 102400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 102500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 102500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 102500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 102500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 102600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 102600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 102600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 102600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 102600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 102600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 102700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 102700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 102700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 102700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 102800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 102800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 102800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 102800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 102900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 102900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 102900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 102900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 102900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 102900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 102900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 102900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 103000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 103000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 103000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 103000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 103100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 103100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 103100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 103100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 103200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 103200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 103200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 103200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 103200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 103200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 103300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 103300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 103300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 103300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 103400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 103400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 103400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 103400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 103500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 103500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 103500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 103500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 103500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 103500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 103600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 103600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 103600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 103600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 103700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 103700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 103700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 103700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 103800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 103800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 103800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 103800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 103800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 103800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 103800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 103900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 103900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 103900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 103900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 103900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 104000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 104000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 104000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 104000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 104100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 104100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 104100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 104100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 104100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 104100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 104200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 104200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 104200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 104200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 104300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 104300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 104300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 104300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 104400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 104400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 104400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 104400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 104400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 104400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 104500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 104500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 104500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 104500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 104600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 104600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 104600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 104600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 104700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 104700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 104700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 104700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 104700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 104700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 104800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 104800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 104800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 104800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 104900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 104900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 104900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 104900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 104900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 104900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 105000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 105000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 105000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 105000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 105000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 105000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 105000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 105100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 105100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 105100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 105100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 105200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 105200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 105200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 105200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 105200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 105300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 105300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 105300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 105300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 105300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 105300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 105300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 105400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 105400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 105400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 105400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 105500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 105500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 105500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 105500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 105500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 105600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 105600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 105600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 105600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 105600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 105600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 105600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 105700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 105700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 105700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 105700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 105800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 105800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 105800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 105800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 105800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 105900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 105900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 105900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 105900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 105900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 105900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 105900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 105900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 106000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 106000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 106000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 106100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 106100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 106100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 106100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 106100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 106200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 106200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 106200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 106200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 106200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 106200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 106300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 106300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 106300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 106400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 106400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 106400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 106400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 106400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 106500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 106500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 106500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 106500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 106500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 106500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 106500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 106600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 106600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 106600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 106700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 106700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 106700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 106700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 106700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 106800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 106800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 106800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 106800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 106800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 106800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 106900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 106900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 106900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 106900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 106900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 107000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 107000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 107000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 107000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 107100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 107100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 107100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 107100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 107100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 107100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 107200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 107200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 107200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 107200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 107300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 107300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 107300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 107300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 107400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 107400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 107400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 107400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 107400 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 107400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 107500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 107500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 107500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 107500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 107600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 107600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 107600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 107600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 107700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 107700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 107700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 107700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 107700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 107700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 107700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 107800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 107800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 107800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 107800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 107900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 107900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 107900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 107900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 107900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 107900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 108000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 108000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 108000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 108000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 108000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 108000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 108000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 108100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 108100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 108100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 108100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 108200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 108200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 108200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 108200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 108200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 108300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 108300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 108300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 108300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 108300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 108300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 108300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 108400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 108400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 108400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 108400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 108500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 108500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 108500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 108500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 108500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 108600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 108600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 108600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 108600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 108600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 108600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 108600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 108700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 108700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 108700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 108700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 108800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 108800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 108800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 108800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 108800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 108900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 108900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 108900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 108900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 108900 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 108900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 108900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 108900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 109000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 109000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 109000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 109000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 109100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 109100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 109100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 109100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 109100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 109200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 109200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 109200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 109200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 109200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 109200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 109200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 109300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 109300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 109300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 109300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 109400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 109400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 109400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 109400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 109400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 109500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 109500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 109500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 109500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 109500 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 109500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 109500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 109600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 109600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 109600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 109600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 109700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 109700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 109700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 109700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 109700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 109800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 109800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 109800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 109800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 109800 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 109800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 109800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 109900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 109900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 109900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 109900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 109900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 110000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 110000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 110000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 110000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 110000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 110100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 110100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 110100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 110100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 110100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 110100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 110100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 110200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 110200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 110200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 110200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 110300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 110300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 110300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 110300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 110300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 110400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 110400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 110400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 110400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 110400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 110400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 110400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 110500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 110500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 110500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 110500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 110600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 110600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 110600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 110600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 110600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 110700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 110700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 110700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 110700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 110700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 110700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 110700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 110800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 110800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 110800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 110800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 110900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 110900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 110900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 110900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 110900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 110900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 111000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 111000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 111000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 111000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 111000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 111000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 111000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 111100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 111100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 111100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 111100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 111200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 111200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 111200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 111200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 111200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 111300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 111300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 111300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 111300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 111300 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 111300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 111300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 111400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 111400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 111400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 111400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 111500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 111500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 111500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 111500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 111500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 111600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 111600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 111600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 111600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 111600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 111600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 111600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 111700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 111700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 111700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 111700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 111800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 111800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 111800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 111800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 111800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 111900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 111900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 111900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 111900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 111900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 111900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 111900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 111900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 112000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 112000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 112000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 112100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 112100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 112100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 112100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 112100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 112200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 112200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 112200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 112200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 112200 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 112200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 112200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 112300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 112300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 112300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 112400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 112400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 112400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 112400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 112400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 112500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 112500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 112500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 112500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 112500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 112500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 112600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 112600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 112600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 112700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 112700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 112700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 112700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 112700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 112800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 112800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 112800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 112800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 112800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 112800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 112900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 112900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 112900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 112900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 112900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 113000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 113000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 113000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 113000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 113100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 113100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 113100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 113100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 113100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 113100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 113200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 113200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 113200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 113200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 113300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 113300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 113300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 113300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 113400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 113400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 113400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 113400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 113400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 113400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 113400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 113500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 113500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 113500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 113500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 113600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 113600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 113600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 113600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 113700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 113700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 113700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 113700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 113700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 113700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 113800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 113800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 113800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 113800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 113900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 113900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 113900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 113900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 113900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 113900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 114000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 114000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 114000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 114000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 114000 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 114000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 114000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 114100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 114100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 114100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 114200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 114200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 114200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 114200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 114200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 114300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 114300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 114300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 114300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 114300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 114300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 114400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 114400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 114400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 114500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 114500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 114500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 114500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 114500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 114600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 114600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 114600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 114600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 114600 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 114600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 114700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 114700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 114700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 114800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 114800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 114800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 114800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 114800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 114900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 114900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 114900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 114900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 114900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 114900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 114900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 114900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 115000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 115000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 115000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 115100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 115100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 115100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 115100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 115100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 115200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 115200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 115200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 115200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 115200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 115200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 115300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 115300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 115300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 115400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 115400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 115400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 115400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 115400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 115500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 115500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 115500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 115500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 115500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 115500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 115600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 115600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 115600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 115700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 115700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 115700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 115700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 115700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 115800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 115800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 115800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 115800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 115800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 115800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 115900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 115900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 115900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 115900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 115900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 116000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 116000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 116000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 116000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 116000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 116100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 116100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 116100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 116100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 116100 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 116100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 116200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 116200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 116200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 116300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 116300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 116300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 116300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 116300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 116400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 116400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 116400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 116400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 116400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 116400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 116500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 116500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 116500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 116600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 116600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 116600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 116600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 116600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 116700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 116700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 116700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 116700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 116700 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 116700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 116800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 116800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 116800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 116800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 116900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 116900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 116900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 116900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 116900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 116900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 117000 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 117000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 117000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 117000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 117000 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117000 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117000 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 117000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 117100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 117100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 117100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 117200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 117200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 117200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 117200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 117200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 117300 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 117300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 117300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 117300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 117300 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117300 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117300 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 117300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 117400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 117400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 117400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 117500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 117500 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 117500 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 117500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 117500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 117600 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 117600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 117600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 117600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 117600 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 117600 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117600 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 117600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 117700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 117700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 117700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 117800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 117800 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 117800 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 117800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 117800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 117900 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 117900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 117900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 117900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 117900 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 117900 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 117900 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 117900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 118000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 118000 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 118000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118000 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 118100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 118100 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 118100 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 118100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 118100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 118200 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 118200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 118200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 118200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 118200 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118200 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118200 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 118200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 118300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 118300 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 118300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118300 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 118400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 118400 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 118400 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 118400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 118400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 118500 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 118500 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 118500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 118500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 118500 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118500 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118500 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 118500 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 118600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 118600 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 118600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118600 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 118700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 118700 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 118700 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 118700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 118700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 118800 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 118800 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 118800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 118800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 118800 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118800 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118800 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 118800 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 118900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 118900 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 118900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 118900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 118900 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 119000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 119000 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 119000 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119000 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119000 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 119000 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 119100 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 119100 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 119100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 119100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 119100 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119100 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119100 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 119100 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 119200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 119200 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 119200 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 119200 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119200 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 119300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 119300 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 119300 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119300 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119300 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 119300 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 119400 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 119400 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 119400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 119400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 119400 | abstract_infinity | abstractions | mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 119400 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119400 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 119400 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 119500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 119500 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 119500 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 119500 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119500 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 119600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 119600 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 119600 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119600 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119600 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 119600 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 119700 | math_arithmetic | mathematics | ✓ mathematics | 1 | 1.00 | ✓ | 1 | 7 |
| 119700 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 119700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 119700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 119700 | abstract_infinity | abstractions | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119700 | logic_deductive | logic | ✓ logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119700 | writing_narrative | writing | ✓ writing | 0 | 1.00 | ✓ | 1 | 4 |
| 119700 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 119800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 119800 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 119800 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 119800 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119800 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 119900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 119900 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 119900 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 119900 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 119900 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 119900 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 120000 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 120000 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 120000 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 120000 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 120100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 120100 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 120100 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 120100 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120100 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 120200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 120200 | music_harmony | music | ✓ music | 5 | 1.00 | ✓ | 1 | 5 |
| 120200 | morality_duty | morality | logic | 5 | 1.00 | ✓ | 1 | 5 |
| 120200 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120200 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 120200 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 120300 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 120300 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 120300 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 120300 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 120400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 120400 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 120400 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 120400 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120400 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 120500 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 120500 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120500 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 120500 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 120600 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 120600 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 120600 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 120600 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 120700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 120700 | math_calculus | mathematics | ✓ mathematics | 5 | 1.00 | ✓ | 1 | 5 |
| 120700 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 120700 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120700 | values_honesty | values | ✓ values | 0 | 1.00 | ✓ | 1 | 4 |
| 120800 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 120800 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 120800 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 120800 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 120900 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 120900 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 120900 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 120900 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 121000 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 121000 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 121000 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121100 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 121100 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121100 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 121100 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 121200 | writing_style | writing | music | 1 | 1.00 | ✓ | 1 | 7 |
| 121200 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 121200 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 121200 | time_perception | time | ✓ time | 0 | 1.00 | ✓ | 1 | 4 |
| 121300 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 121300 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 121300 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121400 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 121400 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121400 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 121400 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 121500 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 121500 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 121600 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 121600 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 121600 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121700 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 121700 | math_geometry | mathematics | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 121700 | morality_consequences | morality | values | 3 | 1.00 | ✓ | 1 | 5 |
| 121700 | logic_inductive | logic | ✓ logic | 0 | 1.00 | ✓ | 1 | 4 |
| 121800 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 121800 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 121900 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 121900 | music_rhythm | music | ✓ music | 3 | 1.00 | ✓ | 1 | 5 |
| 121900 | abstract_emergence | abstractions | logic | 3 | 1.00 | ✓ | 1 | 5 |
| 122000 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 122100 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 122100 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 122200 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 122200 | music_rhythm | music | ✓ music | 4 | 1.00 | ✓ | 1 | 5 |
| 122200 | abstract_emergence | abstractions | logic | 4 | 1.00 | ✓ | 1 | 5 |
| 122300 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 122400 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 122400 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 122500 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 122600 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 122700 | time_arrow | time | ✓ time | 1 | 1.00 | ✓ | 1 | 7 |
| 122700 | writing_metaphor | writing | time | 5 | 1.00 | ✓ | 1 | 5 |
| 122800 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 122900 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 123100 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 123200 | values_justice | values | ✓ values | 1 | 1.00 | ✓ | 1 | 7 |
| 123400 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |
| 123700 | logic_fallacies | logic | ✓ logic | 1 | 1.00 | ✓ | 1 | 7 |

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

| Tick | Fill | Persistence | Entropy | Oscillation | Cascade | Fatigue | Meta | Signatures |
|---|---|---|---|---|---|---|---|---|
| 0 | 0 | 0.00 | 0.00 | 0.00 | 0.00 | 0 | — | Uncertain |
| 2000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 4000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 6000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 8000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 10000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 12000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 14000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 16000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 18000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 20000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 22000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 24000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 26000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 28000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 30000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 32000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 34000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 36000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 38000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 40000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 42000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 44000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 46000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 48000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 50000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 52000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 54000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 56000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 58000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 60000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 62000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 64000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 66000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 68000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 70000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 72000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 74000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 76000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 78000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 80000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 82000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 84000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 86000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 88000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 90000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 92000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 94000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 96000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 98000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 100000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 102000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 104000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 106000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 108000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 110000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 112000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 114000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 116000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 118000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 120000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 122000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 124000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 126000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 128000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 130000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 132000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 134000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 136000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 138000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 140000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 142000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 144000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 146000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 148000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 150000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 152000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 154000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 156000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 158000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 160000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 162000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 164000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 166000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 168000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 170000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 172000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 174000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 176000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 178000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 180000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 182000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 184000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 186000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 188000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 190000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 192000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 194000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 196000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |
| 198000 | 16 | 1.00 | 0.00 | 0.00 | 0.00 | 1 | — | Steady |

## Composite Co-activation Suspects (final)

None detected.

## Meta-subsystem Activations (final)

Active: 0  |  Dominant: none


---

## Benchmark Results

All measurements: release build, Criterion 0.5.

### Hot Path Regression (TickForward / 50 tokens)

```
                        time:   [24.098 µs 24.249 µs 24.411 µs]
                        change: [-15.135% -12.198% -8.9379%] (p = 0.00 < 0.05)
```

### Over-Domain Layer (V7 + parallel ticks)

```
```

---

## Summary

| Parameter | Value |
|-----------|-------|
| Corpus texts | 18 |
| Subsystems | 9 (mathematics · writing · logic · music · time · values · morality · abstractions · time) |
| Engine ticks | 200 000 × 4 shards |
| Hot path (50 tokens) | ~26 µs/tick |
| Warm tick (after 100 ticks) | ~65–70 µs/tick |
| Loaded tick (after 1000 ticks) | ~80–90 µs/tick |
| OBS runtime | ~5–6 min (4 parallel shards) |

Criterion HTML reports: `target/criterion/`  
Bench logs: `showcase/bench_out/`  
OBS report: `showcase/obs_out/report.md`  
