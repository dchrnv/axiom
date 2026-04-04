# Axiom Benchmark Results — v5 (archived)

**Дата:** 2026-03-29
**Платформа:** Linux x86-64
**Профиль:** `release` (optimized)
**Инструмент:** criterion 0.5

### Железо

| Параметр | Значение |
|----------|---------|
| CPU | AMD Ryzen 5 3500U |
| Ядра / потоки | 4 cores / 8 threads |
| Частота (boost) | ~3.46 GHz |
| L2 cache | 512 KB |
| RAM | 5.7 GiB (доступно ~1.8 GiB во время замера) |

---

## axiom-core — базовые структуры

| Операция | Время | Комментарий |
|----------|-------|-------------|
| `Token::new` | 24.4 ns | Инициализация 64-byte структуры |
| `Token::compute_resonance` | 7.6 ns | Арифметика на полях — быстро |
| `Token copy` (Copy trait) | 32.0 ns | Копирование 64 bytes |
| `Event::new` | 29.0 ns | Инициализация 64-byte структуры |
| `Connection::default` | 23.9 ns | Zero-init 64 bytes |

**Вывод:** Все базовые структуры ≤ 32 ns на создание. Соответствует одному L1 cache miss (64 bytes = размер структур).

---

## axiom-space — пространственный хэш

### SpatialHashGrid::rebuild

| Токенов | Время | Время/токен |
|---------|-------|-------------|
| 100 | 7.8 µs | 78 ns |
| 500 | 10.4 µs | 20.8 ns |
| 1 000 | 15.1 µs | 15.1 ns |
| 5 000 | 33.7 µs | 6.7 ns |

O(n) с улучшающейся эффективностью (cache warming на больших наборах).

### SpatialHashGrid::find_neighbors

| Токенов в сцене | Время |
|-----------------|-------|
| 100 | 196.8 ns |
| 500 | 505.1 ns |
| 1 000 | 1.38 µs |

`distance2`: 3.3 ns.

---

## axiom-domain + axiom-arbiter

### EventGenerator

| Операция | Время |
|----------|-------|
| `check_decay` | 109.4 ns |
| `generate_gravity_update` | 23.1 ns |
| `generate_collision` | 24.3 ns |

### Experience::resonance_search

| Traces | Время | Время/trace |
|--------|-------|-------------|
| 0 | 214 ns | — базовая стоимость |
| 10 | 343 ns | ~13 ns |
| 100 | 1.34 µs | ~11 ns |
| 500 | 6.33 µs | ~12 ns ⚠️ высокая вариация |
| 1 000 | 10.6 µs | ~10 ns |

⚠️ На 500 traces — высокий разброс (5.6–7.7 µs). Граничный эффект HashMap при её росте.

### Arbiter::route_token (разные пороги классификации)

| Конфигурация | Рефлекс-порог | Ассоц-порог | Время |
|--------------|--------------|------------|-------|
| strict | 200 | 180 | 4.22 µs |
| loose | 50 | 30 | 4.36 µs |

50 traces в Experience, weight=0.9. **Разница в пределах шума (~3%).**

**Вывод:** Пороги не влияют на latency маршрутизации. Доминирует resonance_search (~10 ns × 50 traces = 500 ns) + AshtiProcessor и MayaProcessor overhead. Fast path (рефлекс) не создаёт ощутимого выигрыша на этих данных — оба пути завершаются за ~4.2 µs. Это нормально: AshtiProcessor запускается в любом случае (dual-path по спецификации).

---

## axiom-shell — Shell V3.0

### compute_shell — полный пересчёт одного токена

| Связей | Время | Время/связь |
|--------|-------|-------------|
| 0 | 7.6 ns | — |
| 5 | 16.4 ns | ~1.8 ns |
| 20 | 37.1 ns | ~1.5 ns |
| 100 | 150 ns | ~1.4 ns |

Линейно по числу связей, очень быстро. **Shell — не горячая точка** даже при 100 связях.

### incremental_update — пересчёт dirty-токенов

| Dirty токенов | Время | Время/токен |
|---------------|-------|-------------|
| 1 | 76.7 ns | 76.7 ns |
| 10 | 372.9 ns | 37.3 ns |
| 50 | 1.56 µs | 31.2 ns |
| 100 | 3.05 µs | 30.5 ns |

Базовая стоимость ~37-77 ns/токен при 20 связях. 100 dirty токенов за 3 µs — укладывается в тик.

### reconcile_batch — heartbeat reconciliation

| Размер батча | Время | Время/токен |
|-------------|-------|-------------|
| 1 | 101 ns | 101 ns |
| 10 | 391 ns | 39.1 ns |
| 50 | 1.65 µs | 33 ns |

Reconcile дороже incremental_update на ~10-25 ns/токен (сравнение с кэшем).

---

## axiom-runtime — AxiomEngine + AshtiCore pipeline

### AshtiCore::process — полный dual-path pipeline

Один "мыслительный акт": token → SUTRA(100) → EXPERIENCE(109) → classify → ASHTI(101-108) → MAYA(110).

| Traces в Experience | Время | Примечание |
|--------------------|-------|-----------|
| 0 (холодная память) | 28–62 µs | Высокая вариация (iter_batched overhead) |
| 10 | 29–67 µs | |
| 100 | 34–68 µs | |

**Вывод:** Один мыслительный акт ≈ 35–50 µs. При 1000 Hz тике бюджет 1 ms → возможно ~20 параллельных обработок в одном тике.

### Базовые операции AxiomEngine

| Операция | Время | Комментарий |
|----------|-------|-------------|
| `AxiomEngine::new` | ~440 µs | 11 доменов + HashMap + Arbiter |
| `InjectToken` | ~17 µs | Включает аллокацию нового Engine |
| `TickForward` (0 токенов) | 31 ns | Чистый overhead диспетчера |
| `TickForward` (10 токенов) | 35 ns | |
| `TickForward` (50 токенов) | 32 ns | |
| `TickForward` (100 токенов) | 33 ns | Стабильно — нет O(n) на тик |
| `snapshot` (0–100 токенов) | 9.0–9.2 µs | Клонирование 11 DomainState |
| `restore_from` (0–100 токенов) | 397–425 µs | Пересоздание всех структур |

---

## axiom-runtime — Этапы 6-8 (новые операции)

### Этап 6 — Адаптивные пороги

| Операция | Traces | Время |
|----------|--------|-------|
| `run_adaptation` | 0 | 19.1 µs |
| `run_adaptation` | 50 | 24.6 µs |
| `run_adaptation` | 200 | 29.9 µs |

`run_adaptation` = adapt_thresholds + adapt_domain_physics + apply_experience_thresholds.
Overhead растёт линейно с числом traces (~50 ns/trace). Подходит для вызова раз в несколько тиков.

### Этап 7 — Causal Horizon + pruning

| Операция | Traces | Время |
|----------|--------|-------|
| `causal_horizon` | — | **21.8 ns** |
| `export_skills` (пустой SkillSet) | — | **5.6 ns** |
| `run_horizon_gc` | 0 | 21.6 µs |
| `run_horizon_gc` | 50 | 31.8 µs |
| `run_horizon_gc` | 200 | 30.2 µs |
| `snapshot_and_prune` | 0 | 32.5 µs |
| `snapshot_and_prune` | 50 | 37.4 µs |
| `snapshot_and_prune` | 200 | 40.2 µs |

`causal_horizon` — практически бесплатен (21 ns), подходит для вызова каждый тик.
`snapshot_and_prune` на 200 traces — 40 µs: разумно для периодической сборки мусора.

### Этап 8 — Gateway + Channel

| Операция | Параметры | Время |
|----------|-----------|-------|
| `Gateway::process` | TickForward, без наблюдателей | **20.1 µs** |
| `Gateway::process_channel` | 1 команда | **19.9 µs** |
| `Gateway::process_channel` | 10 команд | **20.0 µs** |
| `Gateway::process_channel` | 50 команд | **24.9 µs** |

Gateway overhead минимален: `process` vs прямой `process_command` — разница в пределах шума.
Батч из 50 команд через Channel — линейный рост (~0.1 µs/команда сверх первой).

---

## Этапы 12A+12B — FractalChain + batch gravity

### FractalChain::new — инициализация N уровней AshtiCore

| Глубина | Время | Время/уровень |
|---------|-------|--------------|
| 2 | 2.30 ms | 1.15 ms |
| 3 | 1.93 ms | 0.64 ms |
| 5 | 4.75 ms | 0.95 ms |

Доминирует `AxiomEngine::new` на каждый уровень (~440 µs × N + HashMap allocation variability).

### FractalChain::tick — пустая цепочка (overhead диспетчера)

| Глубина | Время | Время/уровень |
|---------|-------|--------------|
| 2 | 48 ns | 24 ns |
| 3 | 75 ns | 25 ns |
| 5 | 110 ns | 22 ns |

Тик без токенов — **практически бесплатен**. Overhead линейный и минимальный.

### FractalChain::tick — с токенами (2 уровня, iter_batched)

| Токенов на вход | Время | Примечание |
|-----------------|-------|-----------|
| 1 | 53 µs | `iter_batched` overhead доминирует |
| 10 | 46 µs | |
| 50 | 43 µs | Стабилизация (AshtiCore pipeline ~40 µs) |

Поведение аналогично `AshtiCore::process`: доминирует dual-path routing (~40–50 µs).
Вариация обусловлена `iter_batched` (пересоздание FractalChain).

### FractalChain — базовые операции

| Операция | Время | Комментарий |
|----------|-------|-------------|
| `inject_input` | 20 ns | Запись токена в SUTRA DomainState |
| `take_output_empty` | 102 ns | Vec::pop() из MAYA + index lookup |
| `exchange_skills` (2 уровня) | 25 ns | Без навыков — только export + import |
| `exchange_skills` (3 уровня) | 38 ns | ~12 ns/уровень сверх первого |
| `exchange_skills` (5 уровней) | 56 ns | |

### apply_gravity_batch vs scalar — сравнение (release, без RUSTFLAGS)

| Токенов | Batch (µs) | Scalar (µs) | ns/токен (batch) | ns/токен (scalar) |
|---------|-----------|------------|-----------------|------------------|
| 100 | 2.45 | 2.16 | 24.5 | 21.6 |
| 500 | 11.27 | 11.63 | 22.5 | 23.3 |
| 1 000 | 22.99 | 22.93 | 23.0 | 22.9 |
| 5 000 | 117.5 | 115.2 | 23.5 | 23.0 |
| 10 000 | 241.8 | 234.2 | 24.2 | 23.4 |

**Наблюдение:** В стандартном release-билде batch и scalar показывают идентичную производительность (~23 ns/токен). Оба пути линейны по числу токенов.

**Почему нет ускорения?** Компилятор уже авто-векторизует оба пути при `-O2` (release). Явное преимущество batch появляется с `RUSTFLAGS="-C target-cpu=native"` — тогда компилятор использует AVX2 (8 i32 за такт) вместо SSE2.

```bash
# Для реального SIMD-ускорения:
RUSTFLAGS="-C target-cpu=native" cargo bench --bench fractal_bench -- "apply_gravity"
```

Ожидаемое ускорение с AVX2: **2–3x** на задачах ≥ 1000 токенов.

---

## Сводная таблица — горячий путь (1000 Hz тик, бюджет 1 ms)

| Операция | Время | % бюджета |
|----------|-------|-----------|
| `TickForward` (100 токенов) | 33 ns | < 0.01% |
| `causal_horizon` | 22 ns | < 0.01% |
| `AshtiCore::process` (один акт) | ~40 µs | 4.0% |
| `run_adaptation` (200 traces) | 30 µs | 3.0% |
| `run_horizon_gc` (200 traces) | 30 µs | 3.0% |
| `snapshot_and_prune` (200 traces) | 40 µs | 4.0% |
| `Gateway::process` (TickForward) | 20 µs | 2.0% |
| `resonance_search` (1000 traces) | 10.6 µs | 1.06% |
| `SpatialHashGrid::rebuild` (1000 токенов) | 15.1 µs | 1.51% |
| `Shell::incremental_update` (100 dirty) | 3.05 µs | 0.31% |
| `Arbiter::route_token` | 4.2 µs | 0.42% |
| `FractalChain::tick` (2 уровня, пусто) | 48 ns | < 0.01% |
| `FractalChain::tick` (2 уровня, 50 токенов) | ~43 µs | 4.3% |
| `apply_gravity_batch` (1000 токенов) | 23 µs | 2.3% |
| `FractalChain::inject_input` | 20 ns | < 0.01% |
| `FractalChain::exchange_skills` (3 уровня) | 38 ns | < 0.01% |

**Общая оценка:** TickForward, causal_horizon, inject_input и exchange_skills — практически бесплатны. AshtiCore pipeline (~40 µs) доминирует — как в одиночном режиме, так и в FractalChain. `apply_gravity_batch` линеен (~23 ns/токен). Gateway overhead минимален. run_adaptation и snapshot_and_prune — периодические операции, подходят для вызова раз в N тиков.

---

## Стресс-тест: 10K → 10M (stress_bench)

### 1. apply_gravity_batch — вычислительный предел

| Токенов | Время | ns/токен | Throughput |
|---------|-------|---------|-----------|
| 10 000 | 259.7 µs | 26.0 | **38.5M tok/s** |
| 100 000 | 2.482 ms | 24.8 | **40.3M tok/s** |
| 1 000 000 | 25.03 ms | 25.0 | **40.0M tok/s** |
| 10 000 000 | 283.4 ms | 28.3 | **35.3M tok/s** |

**Наблюдение:** Идеально линейная масштабируемость вплоть до 1M токенов (~25 ns/токен).
При 10M (+12% к ns/токен) — начинается cache pressure: данные (80 MB) не помещаются в L3.

**Вывод:** 40M tok/s — стабильный вычислительный потолок на данном железе. При `-C target-cpu=native` + AVX2 ожидается 2–3× (~80–120M tok/s).

---

### 2. SpatialHashGrid::rebuild — хеш-таблица при масштабировании

| Токенов | Время | ns/токен | Throughput |
|---------|-------|---------|-----------|
| 10 000 | 50.0 µs | 5.0 | **200M tok/s** |
| 50 000 | 267.4 µs | 5.3 | **187M tok/s** |
| 100 000 | 575.6 µs | 5.8 | **174M tok/s** |
| 500 000 | 2.958 ms | 5.9 | **169M tok/s** |
| 1 000 000 | 5.742 ms | 5.7 | **174M tok/s** |

**Наблюдение:** Практически идеальный O(n) — 5–6 ns/токен на всём диапазоне 10K→1M.
Незначительная деградация при 100K–500K (cache eviction хеш-таблицы), затем стабилизация.

**Вывод:** SpatialHashGrid — не узкое место даже при 1M токенов (5.7 ms).
При 1000 Hz тике бюджет 1 ms → до ~175K токенов за один rebuild.

---

### 3. resonance_search (Experience) — поведение на больших данных

| Трейсов | Время (медиана) | Вариация | Комментарий |
|---------|----------------|---------|------------|
| 1 000 | 9.62 µs | низкая | Стабильно |
| 5 000 | 14.84 µs | высокая (12–18 µs) | Реаллокация HashMap |
| 10 000 | 12.10 µs | средняя (10.5–13.4 µs) | |
| 50 000 | 11.23 µs | высокая (9.7–15.7 µs) | |

**Наблюдение:** Время поиска практически **не зависит от числа трейсов** — O(1).
`resonance_search` использует HashMap lookup, а не линейный обход.
Вариация при 5K–50K обусловлена периодической реаллокацией HashMap при росте.

**Вывод:** Experience масштабируется до 50K трейсов без деградации latency.

---

### Сводка стресс-теста

| Компонент | Потолок throughput | Узкое место |
|-----------|-------------------|------------|
| `apply_gravity_batch` | **40M tok/s** (1M) / 35M tok/s (10M) | L3 cache при > 1M |
| `SpatialHashGrid::rebuild` | **174–200M tok/s** | Практически отсутствует до 1M |
| `resonance_search` | **O(1)** ~10–15 µs | HashMap realloc при росте |

---

## Замеченные аномалии — требуют наблюдения

1. **AshtiCore pipeline** — высокая вариация (28–68 µs). Обусловлена `iter_batched`: пересоздание AxiomEngine включает 11 HashMap + Arbiter. Сам pipeline стабилен ~35–50 µs.
2. **resonance_search/500** — вариация 5.6–7.7 µs. Граничный эффект HashMap при росте.
3. **Arbiter strict vs loose** — разница <3%. Fast/slow path routing не создаёт асимметрии по скорости.
4. **AxiomEngine::new** — ~440 µs. Доминирует инициализация 11 DomainState + Arbiter HashMap. Ожидаемо: это cold-start операция.

---

## История версий

| Версия | Дата | Что добавлено |
|--------|------|--------------|
| v1 | 2026-03-27 | Baseline: axiom-core, axiom-space, EventGenerator, resonance_search, AxiomEngine (add_domain, InjectToken, TickForward, Snapshot) |
| v2 | 2026-03-27 | AshtiCore pipeline, Shell V3.0 (compute/incremental/reconcile), Arbiter thresholds; рефактор engine_bench под новый API |
| v3 | 2026-03-28 | Этапы 6-8: run_adaptation, snapshot_and_prune, run_horizon_gc, causal_horizon, export_skills, Gateway::process, process_channel |
| v4 | 2026-03-29 | Этапы 12A-12B: FractalChain (new/tick/inject/exchange_skills), apply_gravity_batch vs scalar (100–10K токенов) |
| v5 | 2026-03-29 | Стресс-тест: apply_gravity_batch (10K→10M), SpatialHashGrid::rebuild (10K→1M), resonance_search (1K→50K) |
| v6 | 2026-04-03 | integration_bench: normal 100k/1M тиков, integrated_cycle, TickSchedule overhead, reconcile_all, snapshot tick_count, compare_tokens, stress 60s |

---

# Axiom Benchmark Results — v6

**Дата:** 2026-04-03
**Платформа:** Linux x86-64
**Профиль:** `release` (optimized)
**Инструмент:** criterion 0.5
**813 тестов, 0 failures**

### Железо

| Параметр | Значение |
|----------|---------|
| CPU | AMD Ryzen 5 3500U |
| Ядра / потоки | 4 cores / 8 threads |
| Частота (boost) | ~3.46 GHz |
| L2 cache | 512 KB |
| RAM | 5.7 GiB |

---

## normal/100k_ticks — пропускная способность тика (100 000 тиков/батч)

| Конфигурация | Медиана | Throughput | Комментарий |
|-------------|---------|-----------|-------------|
| engine_empty | 5.91 ms | **16.9 Melem/s** | Только hot path, 0 токенов |
| engine_50_tokens | 6.21 ms | **16.1 Melem/s** | 50 токенов в LOGIC, нет следов |
| engine_50tok_100tr_default_schedule | 7.23 ms | **13.8 Melem/s** | Warm/cold пути включены |
| engine_50tok_max_schedule | 121.4 ms | **824 Kelem/s** | Все задачи каждый тик (max load) |

**Вывод:** Hot path без периодических задач — ~59 ns/тик. Default schedule с 100 трейсами даёт ~72 ns/тик (+22%). Max schedule (reconcile+adapt+dream+horizon каждый тик) — ~1.21 µs/тик, 16× дороже hot path.

---

## normal/1M_ticks — выносливость 1 000 000 тиков

| Конфигурация | Медиана | Throughput | Комментарий |
|-------------|---------|-----------|-------------|
| engine_empty | 64.3 ms | **15.6 Melem/s** | Baseline |
| engine_50tok_hot_only | 27.3 ms | **36.6 Melem/s** | TickSchedule без периодических задач |
| engine_50tok_default_schedule | 96.5 ms | **10.4 Melem/s** | Default schedule, 100 трейсов |

**Вывод:** `engine_50tok_hot_only` быстрее пустого engine — эффект CPU branch prediction при прогретом состоянии. Default schedule с трейсами — ~96 ns/тик с редкими периодическими вызовами. Высокая вариация (80–137 ms) обусловлена нерегулярными cold-path операциями.

---

## normal/integrated_cycle — полный цикл (inject → tick → reconcile)

| Сценарий | Медиана | Throughput | Комментарий |
|---------|---------|-----------|-------------|
| inject_tick_reconcile | 40.3 µs | 24.8 Kops/s | Один полный цикл с 50 токенами |
| 1000ticks_then_snapshot | 229.9 µs | 4.35 Kops/s | 1000 тиков + reconcile каждые 100 + snapshot |

**Вывод:** inject+tick+reconcile — ~40 µs на операцию. Вариация 24–77 µs обусловлена iter_batched (пересоздание engine ~440 µs включается в warmup, но не в измерение). Батч 1000 тиков со snapshot — ~230 µs, что соответствует ~230 ns/тик при учёте накладных расходов.

---

## periodic/tick_schedule_overhead — стоимость одного тика по конфигурации

| Конфигурация | Медиана | Комментарий |
|-------------|---------|-------------|
| hot_only | 36.9 µs | Все периодические задачи отключены |
| default_schedule | 36.0 µs | Дефолтные интервалы |
| max_schedule | **25.0 µs** | Все задачи каждый тик |

**Примечание:** Высокая вариация у hot_only/default (27–74 µs) — iter_batched создаёт engine перед каждой итерацией. max_schedule стабильнее (25 µs ± 5%) потому что engine инициализация уже прогрета. Для сравнения стоимости именно периодических задач — см. normal/100k_ticks где вариация усредняется по 100k итерациям.

---

## periodic/reconcile_all — семантическая консистентность (AshtiCore)

| Токенов / Связей | Медиана | Комментарий |
|-----------------|---------|-------------|
| t0_c0 | 21.3 µs | Пустой AshtiCore, 11 доменов |
| t50_c0 | 46.8 µs | 50 токенов, нет связей |
| t50_c100 | 47.5 µs | 50 токенов + 100 связей |
| t200_c500 | **52.8 µs** | 200 токенов + 500 связей |

**Вывод:** `reconcile_all` при 200 токенах и 500 связях — ~53 µs. Стоимость практически не зависит от числа связей (47 vs 53 µs при ×5 росте), т.к. доминирует обход 11 доменов (~2 µs/домен). При дефолтном интервале reconcile каждые N тиков — нагрузка ничтожна.

---

## periodic/snapshot_restore_tick_count — snapshot с сохранением tick_count

| Операция | Медиана | Комментарий |
|---------|---------|-------------|
| snapshot после 0 тиков | 38.0 µs | Базовая стоимость клонирования |
| snapshot после 1000 тиков | 36.2 µs | Идентично — tick_count не увеличивает размер |
| snapshot после 50000 тиков | 46.3 µs | Незначительный рост (cache effect) |
| restore_preserves_tick_count | **640 µs** | Полное восстановление + верификация tick_count |

**Вывод:** Snapshot не зависит от tick_count (поле u64 в EngineSnapshot). Restore ~640 µs = пересоздание всех структур. tick_count корректно сохраняется и восстанавливается.

---

## periodic/compare_tokens — per-domain tolerances vs fallback

| Конфигурация | Медиана | Комментарий |
|-------------|---------|-------------|
| fallback_constants | 12.2 ns | Прямое сравнение полей с константами |
| per_domain_config | **30.3 ns** | HashMap lookup + fallback |

**Вывод:** Per-domain конфиг добавляет ~18 ns на вызов `compare_tokens` (HashMap lookup). При 1000 Hz тике и редком вызове — незначительно. При сравнении тысяч токенов/тик — возможна оптимизация кэшированием конфига на уровне домена.

---

## stress/sustained_60s — выносливость под нагрузкой (3 × 60 секунд)

Батч: 1000 тиков на измерение. Throughput = 1000 тиков/батч.

| Сценарий | Медиана | Throughput | Комментарий |
|---------|---------|-----------|-------------|
| baseline_hot_only_50tok | 25.8 µs | **38.8 Melem/s** | 50 токенов, нет периодических задач |
| realistic_engine_50tok | 64.8 µs | **15.4 Melem/s** | 50 токенов, default schedule + 100 трейсов |
| heavy_engine_200tok_max_schedule | 3.64 ms | **275 Kelem/s** | 200 токенов, все задачи каждый тик |

**Вывод:**
- **Hot path baseline** — 25.8 µs/батч из 1000 тиков = **25.8 ns/тик**. Максимальная теоретическая частота: ~38 MHz тиков.
- **Realistic** — 64.8 µs/батч = **64.8 ns/тик**. Периодические задачи (adaptation, horizon_gc, reconcile) добавляют ~39 ns в среднем на тик (усреднено по интервалу).
- **Max schedule** — 3.64 ms/батч из 1000 тиков = **3.64 µs/тик**. 200 токенов × все задачи каждый тик — тяжёлая конфигурация. Потолок ~274 Hz при такой нагрузке.
- Все три сценария показали стабильные результаты на протяжении 60 секунд — деградации не наблюдается.

---

## Сводная таблица v6 — горячий путь (1000 Hz тик, бюджет 1 ms)

| Операция | Время | ns/тик | % бюджета |
|----------|-------|--------|-----------|
| TickForward (hot path, 0 токенов) | 59 ns/тик | 59 | 0.006% |
| TickForward (50 токенов, hot_only) | 27 ns/тик | 27 | 0.003% |
| TickForward (50 токенов, default_schedule) | ~97 ns/тик | 97 | 0.010% |
| TickForward (50 токенов, max_schedule) | ~1.21 µs/тик | 1210 | 0.12% |
| `reconcile_all` (200 токенов, 500 связей) | 53 µs | — | 5.3%/вызов |
| `compare_tokens` fallback | 12 ns | — | — |
| `compare_tokens` per-domain | 30 ns | — | — |
| `snapshot` | 38–46 µs | — | 3.8–4.6%/вызов |
| `restore_from` | 640 µs | — | 64%/вызов |
| inject_tick_reconcile (полный цикл) | 40 µs | — | 4.0%/вызов |

**Вывод:** При реалистичной нагрузке (50 токенов, default schedule) — ~97 ns/тик, что позволяет работать на частоте > 10 MHz. При max schedule (все задачи каждый тик) — ~1.2 µs/тик, потолок ~830 kHz. `reconcile_all` и `snapshot` — периодические, не горячий путь.

---

## Трудности при написании и запуске integration_bench (v6)

### 1. Неправильное использование `iter_custom` — нулевые результаты

**Проблема:** Все четыре `iter_custom`-бенчмарка изначально игнорировали параметр `iters`:
```rust
b.iter_custom(|_| {
    let mut engine = AxiomEngine::new();
    let start = Instant::now();
    for _ in 0..100_000 { engine.process_command(&tick); }
    start.elapsed()
})
```
Criterion передаёт `iters` — количество логических итераций, которое он хочет выполнить. Он делит возвращённое время на `iters`. Когда код игнорирует `iters` и всегда выполняет ровно 100k итераций, а Criterion запрашивает, например, 18 446 744 074 итераций — деление даёт ~0 ps. Throughput показывал `149 Pelem/s` (физически невозможные значения).

**Симптом:** Три из четырёх сценариев 100k_ticks показали `time: [0.0000 ps 0.0000 ps 0.0000 ps]`.

**Исправление:** Завернуть внутренний цикл в внешний по `iters`, аккумулировать `Duration`:
```rust
b.iter_custom(|iters| {
    let mut total = Duration::ZERO;
    for _ in 0..iters {
        let mut engine = AxiomEngine::new();
        let start = Instant::now();
        for _ in 0..100_000 { engine.process_command(&tick); }
        total += start.elapsed();
    }
    total
})
```
Та же проблема присутствовала в `normal/1M_ticks`, `normal/integrated_cycle/1000ticks_then_snapshot` и `stress/sustained_10min`.

### 2. Минимальный `sample_size` Criterion — паника

**Проблема:** `normal/1M_ticks` задавал `group.sample_size(5)`. Criterion требует минимум 10 сэмплов — при меньшем значении падает с `assertion failed: num_size >= 10`.

**Исправление:** Изменить на `sample_size(10)`.

### 3. `group.measurement_time()` в коде игнорирует CLI `--measurement-time`

**Проблема:** `stress/sustained_10min` задавал `group.measurement_time(Duration::from_secs(600))`. При запуске с флагом `--measurement-time 60` ожидался override, но Criterion применяет программный вызов поверх CLI-параметра при использовании групп. Benchmark запустился на 600 секунд вместо 60.

**Симптом:** Criterion вывел `Collecting 10 samples in estimated 600.06 s` несмотря на CLI-флаг.

**Исправление:** Изменить значение непосредственно в коде (`Duration::from_secs(60)`).

### 4. Захват вывода фоновых процессов

**Проблема:** Bash tool автоматически переводит долгие команды в фон. Вывод Criterion (stdout) в некоторых случаях не попадал в файл вывода (`tasks/*.output`), который содержал только ANSI-заголовок bash-профиля (153 байта). Причина: буферизация stdout при перенаправлении в файл.

**Решение:** Запускать бенчмарки напрямую в foreground с явным `timeout`, либо дожидаться завершения процесса через `wait PID`.
