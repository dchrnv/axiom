# Axiom Benchmark Results

**v10 · 2026-05-17** · AMD Ryzen 5 3500U · 4c/8t · 3.46 GHz · Linux x86-64 · criterion 0.5 · `release`

---

## Быстрая справка — ключевые числа

| Операция | Время | Примечание |
|----------|-------|------------|
| `TickForward` (50 tok, 1M тиков) | **~100–135 ns/тик** | sustained, default schedule |
| `TickForward` (50 tok, hot only) | **~30 ns/тик** | без периодических задач |
| `AxiomEngine::new` | **527 µs** | v10 — с AE/CR/NA в конструкторе |
| `snapshot` | **9.1 µs** | 50 токенов |
| `restore_from` | **665 µs** | 50 токенов |
| `FrameWeaver` on_tick (20 patterns) | **2.5 µs** | MAYA с 20 активными паттернами |
| Phase C tick (AE fires, t%5) | **34 µs** | пустой engine |
| Phase C tick (CR fires, t%7) | **44 µs** | пустой engine |
| `resonance_search` | **O(1) ~17–29 µs** | 1K–50K трейсов |
| `Token::new` | **17 ns** | |
| `SpatialHashGrid::rebuild` (1K tok) | **9.5 µs** | |
| `apply_gravity_batch` (1K tok) | **23 µs** | ~23 ns/токен |

---

## v10 — текущие результаты (2026-05-17)

### axiom-core

| Операция | Время |
|----------|-------|
| `Token::new` | 17 ns |
| `Token::compute_resonance` | 5.5 ns |
| `Token copy` | 25 ns |
| `Event::new` | 19 ns |
| `Connection::default` | 18 ns |

*Числа стабильны с v7; ядро структур не менялось.*

---

### axiom-space

| Операция | Токенов | Время | ns/tok |
|----------|---------|-------|--------|
| `SpatialHashGrid::rebuild` | 100 | 5.9 µs | 59 ns |
| | 1 000 | 9.5 µs | 9.5 ns |
| | 5 000 | 27.9 µs | 5.6 ns |
| `find_neighbors` | 1 000 | 1.3 µs | — |
| `distance2` | — | 6.7 ns | — |

Граница L3→RAM при ~50K токенов (~3.2 MB): после неё rebuild дорожает до ~20 ns/tok.

---

### axiom-shell

| Операция | Параметры | Время |
|----------|-----------|-------|
| `compute_shell` | 100 связей | 197 ns |
| `incremental_update` | 100 dirty | 2.9 µs (~29 ns/tok) |
| `reconcile_batch` | 50 | 5.8 µs |

---

### axiom-domain + axiom-arbiter

| Операция | Параметры | Время |
|----------|-----------|-------|
| `check_decay` | — | ~265 ns |
| `generate_collision` | — | ~38 ns |
| `resonance_search` | 0 traces | 280 ns |
| `resonance_search` | 1 000 traces | ~12–18 µs (O(1)) |
| `resonance_search` | 50 000 traces | ~29 µs (O(1)) |
| `compare_tokens` fallback | — | 11 ns |
| `compare_tokens` per_domain | — | 26 ns |

---

### axiom-frontier

| Операция | Параметры | Время |
|----------|-----------|-------|
| `push_pop` | 100 событий | 1.4 µs (~14 ns/событие) |
| `begin_end` | — | ~340 ps |
| `batch_pop` | — | 8.5 µs (vs `normal_pop` 12.4 µs, −31%) |

---

### axiom-runtime — AxiomEngine (engine_bench, v10)

| Операция | Параметры | Время |
|----------|-----------|-------|
| `AxiomEngine::new` | full | **527 µs** |
| `AxiomEngine::new` | AshtiCore only | 445 µs |
| `InjectToken` | — | 19.4 µs |
| `TickForward` | 0 токенов | **230 ns** |
| `TickForward` | 10 токенов | 309 ns |
| `TickForward` | 50 токенов | 353 ns |
| `TickForward` | 100 токенов | 427 ns |
| `snapshot` | 0 токенов | 8.6 µs |
| `snapshot` | 50 токенов | **9.1 µs** |
| `snapshot` | 100 токенов | 11.5 µs |
| `restore_from` | 0 токенов | 700 µs |
| `restore_from` | 50 токенов | **665 µs** |
| `run_adaptation` | 200 traces | 29 µs |
| `run_adaptation` | 500 traces | 36 µs |
| `horizon_gc` (isolated) | — | 35 ns |
| `causal_horizon` | — | 31 ns |
| `export_skills` | — | 11 ns |
| `Gateway::process` (TickForward) | — | 24 µs |

`AxiomEngine::new` вернулся к 527 µs после v8 (1.42 ms) — рефакторинг инициализации. Теперь включает AE/CR/NA в конструкторе.

---

### axiom-runtime — FractalChain

| Операция | Параметры | Время |
|----------|-----------|-------|
| `FractalChain::tick` | 2 уровня, пусто | 42 ns |
| `FractalChain::tick` | 2 уровня, 50 токенов | 45 µs |
| `inject_input` | — | 20 ns |
| `exchange_skills` | 2 уровня | 28 ns |
| `apply_gravity_batch` | 1 000 токенов | 23.4 µs (23 ns/tok) |
| `apply_gravity_batch` | 10 000 токенов | 248 µs (25 ns/tok) |

---

### FrameWeaver overhead (v10)

Замер overhead on_tick в зависимости от состояния MAYA:

| Сценарий | Параметры | Время |
|---------|-----------|-------|
| Disabled (drain only) | 0–100 токенов | 400–455 ns |
| Active, MAYA empty | 50 токенов | **507 ns** |
| Active, 5 паттернов | 50 токенов | **812 ns** |
| Active, 20 паттернов | 50 токенов | **2.5 µs** |
| `scan_state` isolated | 0 паттернов | 16 ns |
| `scan_state` isolated | 5 паттернов | 1.7 µs |
| `scan_state` isolated | 20 паттернов | 6.7 µs |
| `scan_state` isolated | 100 паттернов | 17.4 µs |

FrameWeaver overhead при реальной нагрузке (20 активных паттернов) — **2.5 µs/скан**. Отдельный scan_state при 20 паттернах — 6.7 µs, разница объясняется тем что on_tick проверяет интервал и пропускает многие тики.

---

### Phase C coordinator overhead (v10 — новый)

Замер стоимости одного TickForward в момент срабатывания AE/CR/NA (пустой engine, 20 токенов, 20 трейсов):

| Сценарий | tick_count | Время |
|---------|-----------|-------|
| Базовый (нет Phase C) | t%1 | ~35–78 µs ⚠️ |
| AE on_tick (t%5) | t=5 | **34 µs** |
| CR on_tick (t%7) | t=7 | **44 µs** |
| AE + CR (t%35) | t=35 | **38 µs** |
| AE + CR + NA (t%385) | t=385 | **33 µs** |

⚠️ Базовый (t%1) показывает высокую вариацию из-за `iter_batched` с 0 pre-run тиков. При t=5–385 вариация значительно ниже. Все компоненты на пустом engine выполняются за < 50 µs — минимальный overhead. При наличии реальных Frame данные будут выше (AE оценивает каждый Frame, CR сканирует MAYA).

---

### Integration bench

| Операция | Параметры | Время |
|----------|-----------|-------|
| TickForward / tick_schedule | hot_only, 50 tok | **34 µs/тик** |
| TickForward / tick_schedule | default, 50 tok | **31 µs/тик** |
| TickForward / tick_schedule | max_schedule, 50 tok | **45 µs/тик** |
| `compare_tokens` fallback | — | 11 ns |
| `compare_tokens` per_domain | — | 26 ns |

*1M-тиков и stress-тест не перезамерялись в v10 (60+ с каждый). Ориентир: v9 ~100–135 ns/тик при 50 токенах default schedule.*

---

## История версий

| Версия | Дата | Ключевое изменение | `TickForward` (50 tok) |
|--------|------|--------------------|------------------------|
| v1–v3 | 2026-03-27 | baseline: core/space/domain/shell | 31–35 ns |
| v4–v5 | 2026-03-29 | FractalChain, стресс 10K→10M | 32 ns |
| v6 | 2026-04-03 | integration_bench, 1M тиков | 96.5 ns/тик (1M) |
| v7 | 2026-04-11 | D-01/D-02/D-03: u16 domain_id; полный прогон | 96.5 ns/тик (1M) |
| v8 | 2026-04-12 | CLI Extended V1.0 | ~320 ns/тик (high var) |
| v9 | 2026-04-20 | Adapters 0A-5; domain_detail_snapshot bench | ~350 ns/тик |
| v9.1 | 2026-04-27 | FrameWeaver overhead bench добавлен | — |
| **v10** | **2026-05-17** | **Phase C (AE/CR/NA) в Engine; Phase C overhead bench** | **353 ns/тик** |

**Ключевые регрессии:**
- `AxiomEngine::new`: v6 440 µs → v8 1.42 ms (D-01 + расширение) → **v10 527 µs** (рефакторинг вернул)
- `TickForward` на горячем пути (без периодических задач): **стабильно ~30 ns**; с default schedule ~100–350 ns в зависимости от state/тиков
- Phase C добавляет ≤ 15 µs на периодических тиках на пустом engine

**Потолки throughput (стресс-тест v7, не перезамерялся):**

| Компонент | Throughput |
|-----------|-----------|
| `apply_gravity_batch` (<50K токенов, в L3) | ~30M tok/s |
| `apply_gravity_batch` (>1M токенов, в RAM) | ~15M tok/s |
| `SpatialHashGrid::rebuild` (<50K, в L3) | ~120M tok/s |
| `resonance_search` | O(1), 17–29 µs до 50K трейсов |

*Полная история v1–v9 с детальными таблицами — в git log.*
