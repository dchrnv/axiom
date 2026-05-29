# Axiom Benchmark Results

**v12 · 2026-05-29** · AMD Ryzen 5 3500U · 8t · Linux x86-64 · criterion 0.5 · `release`

---

## Быстрая справка — ключевые числа

| Операция | Время | Примечание |
|----------|-------|------------|
| `TickForward` (50 tok, hot path) | **25.7 µs** | V7 полный pipeline, hot_path_regression |
| `TickForward` (warm, 50 tok) | **65–70 µs** | после 100 прогревочных тиков |
| `TickForward` (loaded, 50 tok) | **80–90 µs** | после 1000 тиков (устойчивый) |
| Throughput 1000 тиков / 50 tok | **~25 µs/тик** | амортизированный |
| `AxiomEngine::new` | **914 µs** | с AE/CR/NA |
| `resonance_search` | **~16 µs** | O(1) до 50K трейсов (Grid-хэш) |
| `apply_gravity_batch` AVX2 (10K tok) | **99 µs** | ~10 ns/токен |
| `apply_gravity_batch` AVX2 (1M tok) | **17.6 ms** | ~18 ns/токен |
| `SpatialHashGrid::rebuild` (10K tok) | **123 µs** | |
| `SpatialHashGrid::rebuild` (1M tok) | **10.8 ms** | |
| `Token::new` | **32 ns** | |

---

## v12 — текущие результаты (2026-05-29)

### Over-Domain Bench (новый, V7 pipeline)

Полный движок с ContextRecognizer + FrameWeaver + NeuralAdvisor:

| Сценарий | Токены | Время |
|----------|--------|-------|
| Холодный тик (fresh engine) | 0–200 | 160–220 µs |
| Warm тик (после 100 тиков) | 0–200 | **65–80 µs** |
| Loaded тик (после 1000 тиков) | 50–500 | **80–90 µs** |
| Throughput 1000 тиков | 50 | 25.3 ms (25.3 µs/тик) |
| Throughput 1000 тиков | 200 | 26.0 ms (26.0 µs/тик) |
| Инжекция (холодный engine) | — | 24.9 µs |
| Инжекция (loaded, 200 токенов) | — | 50.2 µs |

> Холодный тик дороже (~180 µs) из-за инициализации CausalFrontier и SpatialGrid. После прогрева стабилизируется на **65–90 µs**.

---

### Hot Path Regression (V7)

| Сценарий | Время |
|----------|-------|
| `TickForward` / 50 токенов в LOGIC | **25.7 µs** |

---

### Stress Bench (v12, перезамер)

| Операция | N токенов | Время |
|----------|-----------|-------|
| `apply_gravity_batch` (scalar) | 10K | 481 µs |
| | 100K | 3.97 ms |
| | 1M | 38.5 ms |
| | 10M | 397 ms |
| `apply_gravity_batch_avx2` | 10K | **99 µs** |
| | 100K | **1.08 ms** |
| | 1M | **17.6 ms** |
| `SpatialHashGrid::rebuild` | 10K | 123 µs |
| | 100K | 1.04 ms |
| | 500K | 5.39 ms |
| | 1M | 10.8 ms |
| `resonance_search` | 1K traces | 17.8 µs |
| | 5K traces | 22.3 µs |
| | 10K traces | 17.6 µs |
| | 50K traces | **15.3 µs** |

AVX2 даёт **4–5x** против scalar. `resonance_search` O(1) — Grid-хэш Phase 1 эффективен, время не растёт с числом трейсов.

---

## v11 — архивные результаты (2026-05-17)

### axiom-core

| Операция | Время |
|----------|-------|
| `Token::new` | 32 ns |
| `Token::compute_resonance` | 10 ns |
| `Token copy` | 37 ns |
| `Event::new` | 38 ns |
| `Connection::default` | 65 ns |

---

### axiom-space

| Операция | Токенов | Время | ns/tok |
|----------|---------|-------|--------|
| `SpatialHashGrid::rebuild` | 100 | 5.5 µs | 55 ns |
| | 1 000 | 9.5 µs | 9.5 ns |
| | 5 000 | 28.1 µs | 5.6 ns |
| `find_neighbors` | 1 000 | 946 ns | — |
| `distance2` | — | 7.7 ns | — |

---

### axiom-shell

| Операция | Параметры | Время |
|----------|-----------|-------|
| `compute_shell` | 20 связей | 200 ns |
| `compute_shell` | 100 связей | 910 ns |
| `incremental_update` | 100 dirty | 2.7 µs (~27 ns/tok) |
| `reconcile_batch` | 50 | 1.55 µs |

---

### axiom-domain + axiom-arbiter

| Операция | Параметры | Время |
|----------|-----------|-------|
| `check_decay` | — | ~325 ns |
| `generate_collision` | — | ~33 ns |
| `resonance_search` | 0 traces | 309 ns |
| `resonance_search` | 1 000 traces | ~16 µs |
| `compare_tokens` fallback | — | 8 ns |
| `compare_tokens` per_domain | — | 24 ns |
| `Arbiter::route_token` | — | ~9 µs |

---

### axiom-frontier

| Операция | Параметры | Время |
|----------|-----------|-------|
| `push_pop` | 100 событий | 2.0 µs (~20 ns/событие) |
| `begin_end` | — | ~610 ps |
| `batch_pop` | — | 7.9 µs (vs `normal_pop` 12.2 µs, −35%) |

---

### axiom-runtime — AxiomEngine (engine_bench, v11)

| Операция | Параметры | Время |
|----------|-----------|-------|
| `AxiomEngine::new` | full | **914 µs** ⬆ |
| `AxiomEngine::new` | AshtiCore only | 729 µs |
| `InjectToken` | — | 56.6 µs |
| `TickForward` | 0 токенов | **324 ns** |
| `TickForward` | 10 токенов | 415 ns |
| `TickForward` | 50 токенов | 348 ns |
| `TickForward` | 100 токенов | 399 ns |
| `snapshot` | 0 токенов | **10.1 µs** |
| `snapshot` | 100 токенов | **9.57 µs** |
| `restore_from` | 0 токенов | 653 µs |
| `restore_from` | 100 токенов | **572 µs** |
| `run_adaptation` | 200 traces | 70 µs |
| `horizon_gc` (isolated) | — | 165–168 ns |
| `causal_horizon` | — | 109 ns |
| `export_skills` | — | 16 ns |

⬆ `AxiomEngine::new` вырос с 527 µs (v10) до 914 µs — добавлена инициализация AxialEvaluator + ContextRecognizer + NeuralAdvisor (Phase I1, ~387 µs).

---

### axiom-runtime — FractalChain

| Операция | Параметры | Время |
|----------|-----------|-------|
| `FractalChain::tick` | 2 уровня, пусто | 64 ns |
| `FractalChain::tick` | 1 уровень, 50 токенов | 45.6 µs |
| `inject_input` | — | 19 ns |
| `exchange_skills` | 2 уровня | 60 ns |
| `apply_gravity_batch` | 1 000 токенов | 29.7 µs (30 ns/tok) |
| `apply_gravity_batch` | 10 000 токенов | 278 µs (28 ns/tok) |

---

### FrameWeaver overhead (v11)

| Сценарий | Параметры | Время |
|---------|-----------|-------|
| Disabled (drain only) | 0–50 токенов | 358–441 ns |
| Active, MAYA empty | 50 токенов | **483 ns** |
| Active, 5 паттернов | 50 токенов | **1.0 µs** |
| Active, 20 паттернов | 50 токенов | **3.2 µs** |
| `scan_state` isolated | 0 паттернов | 15 ns |
| `scan_state` isolated | 5 паттернов | 1.8 µs |
| `scan_state` isolated | 20 паттернов | 8.6 µs |
| `scan_state` isolated | 50 паттернов | 19.5 µs |

Overhead 20 паттернов вырос с 2.5 µs (v10) до 3.2 µs — Phase C coordinator добавляет дополнительную работу на тиках t%5/7/11.

---

### Phase C coordinator overhead (v11)

Замер стоимости одного тика в момент срабатывания AE/CR/NA (пустой engine, fresh state):

| Сценарий | tick | Время |
|---------|------|-------|
| Базовый (нет Phase C) | t%1 | **23.5 µs** |
| AE on_tick (t%5) | t=5 | **25.2 µs** |
| CR on_tick (t%7) | t=7 | **23.7 µs** |
| AE + CR (t%35) | t=35 | **24.6 µs** |
| AE + CR + NA (t%385) | t=385 | **23.3 µs** |

Phase C добавляет ≤ 1.7 µs на периодических тиках на пустом engine (vs v10: ≤ 15 µs — разница из-за разного setup в bench). Все компоненты работают в пределах шума от базового тика.

---

### Integration bench (v11)

| Операция | Параметры | Время |
|----------|-----------|-------|
| `100k_ticks` | engine_empty | 21.4 ms (214 ns/тик) |
| `100k_ticks` | engine_50tok | 28.1 ms (281 ns/тик) |
| `100k_ticks` | default_schedule | 29.1 ms (291 ns/тик) |
| `100k_ticks` | max_schedule | 189.6 ms (1.9 µs/тик) |
| TickForward / tick_schedule | hot_only, 50 tok | **25.6 µs/тик** |
| TickForward / tick_schedule | default, 50 tok | **25.1 µs/тик** |
| TickForward / tick_schedule | max_schedule, 50 tok | **29.6 µs/тик** |
| `compare_tokens` fallback | — | 8 ns |
| `compare_tokens` per_domain | — | 24 ns |

tick_schedule измеряет 1 тик на свежем engine (включает reconcile_all ~23 µs, без Phase C — tick=1 не кратен 5/7/11). 100k_ticks — устойчивый прогон, Phase C амортизируется.

*1M-тиков и stress-тест не перезамерялись (60+ с каждый).*

---

## История версий

| Версия | Дата | Ключевое изменение | `TickForward` (50 tok) |
|--------|------|--------------------|------------------------|
| v1–v3 | 2026-03-27 | baseline: core/space/domain/shell | 31–35 ns |
| v4–v5 | 2026-03-29 | FractalChain, стресс 10K→10M | 32 ns |
| v6 | 2026-04-03 | integration_bench, 1M тиков | 96.5 ns/тик (1M) |
| v7 | 2026-04-11 | D-01/D-02/D-03: u16 domain_id; полный прогон | 96.5 ns/тик (1M) |
| v8 | 2026-04-12 | CLI Extended V1.0 | ~320 ns/тик |
| v9 | 2026-04-20 | Adapters 0A-5; domain_detail_snapshot bench | ~350 ns/тик |
| v9.1 | 2026-04-27 | FrameWeaver overhead bench добавлен | — |
| v10 | 2026-05-17 | Phase C (AE/CR/NA) в Engine; Phase C overhead bench | 353 ns/тик |
| **v11** | **2026-05-17** | **Phase I: координатор + I6 (Workstation Phase C); полный перезамер** | **348 ns/тик** |
| **v12** | **2026-05-29** | **V7 полный: TransitionMatrix, FatigueStore, GUARDIAN, L0, parallel ticks (rayon), STATE_SLEEPING lifecycle, OBS shards** | **25.7 µs/тик** |

**Ключевые изменения v12 vs v11:**
- `TickForward` (50 tok, hot_path): 348 ns → **25.7 µs** — новый bench измеряет полный V7 pipeline (CR/FW/NA активны), v11 измерял упрощённый engine_bench
- `AshtiCore::tick()`: параллельный (rayon, 6/8 ядер) — process_frontier всех 11 доменов одновременно
- Token lifecycle: STATE_SLEEPING через TokenDecayed, scan_region фильтрует спящие токены
- Warm тик после прогрева: **65–70 µs** (наиболее репрезентативное production-число)
- Стресс-тест перезамерян: AVX2 4–5x vs scalar, resonance O(1) до 50K трейсов

**Потолки throughput (v12):**

| Компонент | Throughput |
|-----------|-----------|
| `apply_gravity_batch` AVX2 (<50K, в L3) | ~100M tok/s |
| `apply_gravity_batch` AVX2 (>1M, в RAM) | ~57M tok/s |
| `SpatialHashGrid::rebuild` (10K) | ~81M tok/s |
| `resonance_search` | **O(1)**, ~16 µs до 50K трейсов |

*Полная история v1–v11 с детальными таблицами — в git log.*
