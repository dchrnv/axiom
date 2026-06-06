# Axiom Benchmark Results

**v13 · 2026-06-05** · AMD Ryzen 5 3500U · 8t · Linux x86-64 · criterion 0.5 · `release`

---

## Быстрая справка — ключевые числа

| Операция | Время | Δ vs v12 |
|----------|-------|----------|
| `TickForward` (50 tok, hot path) | **31.3 µs** | +5.6 µs (Sensorium V2.0) |
| `TickForward` (warm, 50 tok) | **71–83 µs** | ~+5 µs |
| `TickForward` (loaded, 50 tok) | **90–98 µs** | ~+5 µs |
| Throughput 1000 тиков / 50 tok | **~31 µs/тик** | ~+6 µs |
| `AxiomEngine::new` | **1.60 ms** | +700 µs (Sensorium V2.0 + SubsystemGravity) |
| `resonance_search` | **~20 µs** | ≈ (shell_registry path: нейтраль без данных) |
| `apply_subsystem_gravity` 500 tok | **35 µs** | NEW (PRIM-TD-03) |
| `apply_gravity_batch` AVX2 (1M tok) | **11.0 ms** | ≈ |
| `SpatialHashGrid::rebuild` (1M tok) | **10.8 ms** | ≈ |
| `Token::new` | **69 ns** | ≈ |

> **Sensorium V2.0 overhead:** SensoriumState.collect() каждый тик, domain_summaries (11 доменов),
> pre-compute всех полей BroadcastSnapshot → +5–7 µs на тик. Цена убрана из BroadcastSnapshot.
> AxiomEngine::new вырос из-за SubsystemGravityRule boot-time init и расширенного SensoriumView.

---

## v13 — текущие результаты (2026-06-05)

### Over-Domain Bench (V7 pipeline + Sensorium V2.0)

| Сценарий | Токены | Время |
|----------|--------|-------|
| Холодный тик | 0 | 156 µs |
| Холодный тик | 10–50 | 144–155 µs |
| Холодный тик | 200 | 173 µs |
| Warm тик (100 тиков) | 0 | 81 µs |
| Warm тик (100 тиков) | 10–50 | **71–79 µs** |
| Warm тик (100 тиков) | 200 | 78 µs |
| Loaded тик (1000 тиков) | 50 | **98 µs** |
| Loaded тик (1000 тиков) | 200 | 93 µs |
| Loaded тик (1000 тиков) | 500 | 118 µs |
| Throughput 1000 тиков | 50 | 31.0 ms (31 µs/тик) |
| Throughput 1000 тиков | 200 | 31.2 ms (31 µs/тик) |
| Инжекция (loaded, 200 токенов) | — | 48 µs |

Прогрев → **71–80 µs/тик** (production-число).

---

### Hot Path Regression

| Сценарий | Время |
|----------|-------|
| `TickForward` / 50 токенов в LOGIC | **31.3 µs** |

*+5.6 µs vs v12 (25.7 µs) — Sensorium V2.0 collect() на каждом тике.*

---

### Stress Bench (v13)

#### apply_gravity_batch (scalar)

| N токенов | Время | ns/tok |
|-----------|-------|--------|
| 10K | 342 µs | 34 ns |
| 100K | 3.59 ms | 36 ns |
| 1M | **33.9 ms** | 34 ns |
| 10M | 439 ms | 44 ns |

#### apply_gravity_batch (AVX2)

| N токенов | Время | ns/tok |
|-----------|-------|--------|
| 10K | ~140 µs | 14 ns |
| 100K | **865 µs** | 8.7 ns |
| 1M | **11.0 ms** | 11 ns |

AVX2 **3–4x** против scalar при 100K–1M. Разрыв снизился с v12 (4–5x) из-за измерений на другом состоянии системы.

#### apply_subsystem_gravity (NEW — PRIM-TD-03)

4 правила: val_beneficial pull, val_harmful push, abstraction_theory/constructor pull(radius=8000).

| N токенов | Время | ns/tok |
|-----------|-------|--------|
| 100 | 8.6 µs | 86 ns |
| 500 | **35 µs** | 70 ns |
| 1K | **81 µs** | 81 ns |

> Бенч с N=5K/10K даёт ~80 µs — столько же, что и 1K, т.к. `DomainConfig::default()` ограничивает ёмкость домена 1000 токенов. Реальный hot-path: N ≤ 1000 ток → ~80 µs/проход, раз в 500 тиков (не каждый тик). Амортизированный overhead: **< 0.2 µs/тик**.

#### SpatialHashGrid::rebuild

| N токенов | Время | ns/tok |
|-----------|-------|--------|
| 10K | 77 µs | 7.7 ns |
| 50K | 478 µs | 9.6 ns |
| 100K | 809 µs | 8.1 ns |
| 500K | 5.2 ms | 10 ns |
| 1M | **10.8 ms** | 10.8 ns |

#### resonance_search (с shell_registry)

Shell-TD-02: shell_cosine() добавляет 15%-модификатор к score. Без данных в registry → нейтральный путь (≈ baseline).

| N трейсов | Время |
|-----------|-------|
| 1K | **20 µs** |
| 5K | **21.6 µs** |
| 10K | **20.6 µs** |
| 50K | ~30 µs (high variance) |

O(1)-поведение сохранилось — Grid-хэш Phase 1 эффективен.

---

### AxiomEngine::new (v13)

| Сценарий | Время |
|----------|-------|
| `AxiomEngine::new` (full) | **1.60 ms** |
| `AshtiCore::new` only | **1.01 ms** |

+700 µs vs v12 (914 µs). Причины:
- SubsystemGravityRule init (boot-time) — minor
- Sensorium V2.0 инициализация + расширенный SensoriumView pre-alloc
- AshtiCore::new вырос: rayon thread pool + speculative grids + Shell-TD-02 registry setup

---

### AxiomEngine: TickForward detail

| N токенов | Время |
|-----------|-------|
| 0 | 44.5 µs |
| 10 | 48.5 µs |
| 50 | **48.2 µs** |
| 100 | 51.0 µs |

Разница между 0 и 50 токенов — ~4 µs. Доминирует Sensorium collect + OD-компоненты.

---

### FrameWeaver overhead (v13)

| Сценарий | Время |
|---------|-------|
| Disabled (drain only) | **31.4 µs** |
| Active, MAYA empty | **31.4 µs** |
| Active, 5 паттернов | **41.4 µs** |
| Active, 20 паттернов | **63.9 µs** |
| `scan_state` isolated / 0 паттернов | 18 ns |
| `scan_state` isolated / 5 паттернов | 3.5 µs |
| `scan_state` isolated / 20 паттернов | 16.5 µs |
| `scan_state` isolated / 50 паттернов | 41.8 µs |

Disabled overhead вырос с 441 ns (v11) до ~31 µs — теперь baseline включает Sensorium V2.0 collect().

---

### Phase C coordinator overhead (v13)

| Сценарий | Время |
|---------|-------|
| Базовый (t%1) | **41 µs** |
| AE on_tick (t%5) | **141 µs** |
| CR on_tick (t%7) | **103 µs** |
| AE + CR (t%35) | **118 µs** |
| AE + CR + NA (t%385) | **99 µs** |

Phase C добавляет 60–100 µs на периодических тиках. Базовый тик вырос с 23.5 µs (v11) до 41 µs — Sensorium V2.0.

---

### Integration bench (v13)

| Операция | N | Время | µs/тик |
|----------|---|-------|--------|
| `1M_ticks` / hot only | — | 29.7 s | **29.7 µs** |
| `1M_ticks` / default schedule | 50 tok | 30.8 s | **30.8 µs** |
| `100k_ticks` / default schedule | 50 tok | 3.02 s | **30.2 µs** |
| `100k_ticks` / max schedule | 50 tok | 3.60 s | **36.0 µs** |
| `reconcile_all` | 50 tok | 36 µs | — |
| `reconcile_all` | 200 tok, 500 conn | 43 µs | — |

---

### Frontier bench (v13)

| Операция | Время |
|----------|-------|
| `push_pop` 100 событий | 1.96 µs (~20 ns/событие) |
| `begin_end` overhead | 485 ps |
| `batch_pop` 1000 storm | **12.0 µs** |
| `normal_pop` 1000 | **16.9 µs** |

batch_pop на 29% быстрее normal_pop (v12: 35%).

---

### axiom-core (v13 / stable)

| Операция | Время |
|----------|-------|
| `Token::new` | **69 ns** |
| `Token::compute_resonance` | 20 ns |
| `Token copy` | 90 ns |
| `Event::new` | 79 ns |
| `Connection::default` | 61 ns |

---

## История версий

| Версия | Дата | Ключевое изменение | `TickForward` (50 tok) |
|--------|------|--------------------|------------------------|
| v1–v3 | 2026-03-27 | baseline: core/space/domain/shell | 31–35 ns |
| v4–v5 | 2026-03-29 | FractalChain, стресс 10K→10M | 32 ns |
| v6 | 2026-04-03 | integration_bench, 1M тиков | 96.5 ns/тик (1M) |
| v7 | 2026-04-11 | D-01/D-02/D-03: u16 domain_id | 96.5 ns/тик (1M) |
| v8 | 2026-04-12 | CLI Extended V1.0 | ~320 ns/тик |
| v9 | 2026-04-20 | Adapters 0A-5 | ~350 ns/тик |
| v9.1 | 2026-04-27 | FrameWeaver overhead bench | — |
| v10 | 2026-05-17 | Phase C (AE/CR/NA) в Engine | 353 ns/тик |
| v11 | 2026-05-17 | Phase I координатор + полный перезамер | 348 ns/тик |
| v12 | 2026-05-29 | V7 полный: TransitionMatrix, FatigueStore, L0, rayon, STATE_SLEEPING | **25.7 µs/тик** |
| **v13** | **2026-06-05** | **Shell-TD-02 (shell_cosine в resonance), PRIM-TD-03 (SubsystemGravity, NEW bench), SEN-TD-01 V2.0 (Sensorium collect каждый тик)** | **31.3 µs/тик** |

**Ключевые изменения v13 vs v12:**
- `TickForward` (50 tok): 25.7 µs → **31.3 µs** (+5.6 µs) — Sensorium V2.0 collect() на каждом тике
- `AxiomEngine::new`: 914 µs → **1.60 ms** — расширенный boot (SubsystemGravityRule + SensoriumView alloc)
- `resonance_search`: аналогично v12 — shell_cosine добавляет нейтральный overhead без данных в registry
- `apply_subsystem_gravity` **NEW**: 35 µs @ 500 tokens, раз в 500 тиков → < 0.2 µs/тик амортизировано
- `apply_gravity_batch` AVX2: ≈ v12 (11 ms @ 1M)
- `SpatialHashGrid::rebuild`: ≈ v12 (10.8 ms @ 1M)

**Потолки throughput (v13):**

| Компонент | Throughput |
|-----------|-----------|
| `apply_gravity_batch` AVX2 (100K, L3) | ~115M tok/s |
| `apply_gravity_batch` AVX2 (1M, RAM) | ~91M tok/s |
| `SpatialHashGrid::rebuild` (10K) | ~130M tok/s |
| `resonance_search` | **O(1)**, ~20 µs до 50K трейсов |
| `apply_subsystem_gravity` (1K tok, reconcile) | ~12M tok/s |

*Полная история v1–v12 с детальными таблицами — в git log.*
