# Axiom Benchmark Results — v4

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
