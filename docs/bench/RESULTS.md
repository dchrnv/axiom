# Axiom Benchmark Results

**v11 · 2026-05-17** · AMD Ryzen 5 3500U · 4c/8t · 3.46 GHz · Linux x86-64 · criterion 0.5 · `release`

---

## Быстрая справка — ключевые числа

| Операция | Время | Примечание |
|----------|-------|------------|
| `TickForward` (50 tok, 100K тиков) | **~281 ns/тик** | sustained, default schedule |
| `TickForward` (50 tok, hot only) | **~256 ns/тик** | без периодических задач |
| `AxiomEngine::new` | **914 µs** | v11 — с AE/CR/NA в конструкторе (+387 µs vs v10) |
| `snapshot` | **10.1 µs** | 0 токенов (с Phase C данными) |
| `restore_from` | **572 µs** | 100 токенов |
| `FrameWeaver` on_tick (20 patterns) | **3.2 µs** | MAYA с 20 активными паттернами |
| Phase C tick (AE fires, t%5) | **25.2 µs** | пустой engine |
| Phase C tick (CR fires, t%7) | **23.7 µs** | пустой engine |
| `resonance_search` | **O(1) ~16 µs** | 1K трейсов |
| `Token::new` | **32 ns** | |
| `SpatialHashGrid::rebuild` (1K tok) | **9.5 µs** | |
| `apply_gravity_batch` (1K tok) | **29.7 µs** | ~30 ns/токен |

---

## v11 — текущие результаты (2026-05-17)

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

**Ключевые изменения v11 vs v10:**
- `AxiomEngine::new`: 527 µs → **914 µs** (+74%, Phase C init)
- `TickForward` (50 tok): 353 ns → **348 ns** (−1%, без изменений)
- Phase C overhead: 34–44 µs → **23–25 µs** (другой bench-метод, точнее)
- tick_schedule_overhead: 31–45 µs → **25–30 µs** (Sentinel S1-S6 эффект)

**Потолки throughput (стресс-тест v7, не перезамерялся):**

| Компонент | Throughput |
|-----------|-----------|
| `apply_gravity_batch` (<50K токенов, в L3) | ~30M tok/s |
| `apply_gravity_batch` (>1M токенов, в RAM) | ~15M tok/s |
| `SpatialHashGrid::rebuild` (<50K, в L3) | ~120M tok/s |
| `resonance_search` | O(1), ~16 µs до 1K трейсов |

*Полная история v1–v10 с детальными таблицами — в git log.*
