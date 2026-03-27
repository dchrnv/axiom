# Axiom Benchmark Results — v2

**Дата:** 2026-03-27
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
| 0 (холодная память) | 35–50 µs | Высокая вариация |
| 10 | 43–251 µs | ⚠️ экстремальная вариация |
| 100 | 30–42 µs | |

⚠️ **Вариация AshtiCore::process** обусловлена `iter_batched`: создание нового AxiomEngine включает аллокацию Domain/DomainState/Arbiter структур, HashMap инициализацию. Сам pipeline занимает ~30–50 µs — dominируют HashMap lookup и AshtiProcessor × 8 вызовов.

**Вывод:** Один мыслительный акт ≈ 35–50 µs. При 1000 Hz тике бюджет 1 ms → возможно ~20 параллельных обработок в одном тике.

### Прочие операции AxiomEngine

| Операция | Время | Комментарий |
|----------|-------|-------------|
| `AxiomEngine::new` | 151.3 ns → ~25 µs* | *с AshtiCore (11 доменов) |
| `InjectToken` | 2.45 µs | |
| `TickForward` (0 токенов) | 169.9 ns | |
| `TickForward` (100 токенов) | 159.4 ns | |
| `snapshot::capture` (100 токенов) | 1.82 µs | |
| `restore_from` | 29–31 µs | Доминирует пересоздание структур |

---

## Сводная таблица — горячий путь (1000 Hz тик, бюджет 1 ms)

| Операция | Время | % бюджета |
|----------|-------|-----------|
| `TickForward` (100 токенов) | 159 ns | 0.016% |
| `AshtiCore::process` (один акт) | ~40 µs | 4.0% |
| `resonance_search` (1000 traces) | 10.6 µs | 1.06% |
| `SpatialHashGrid::rebuild` (1000 токенов) | 15.1 µs | 1.51% |
| `Shell::incremental_update` (100 dirty) | 3.05 µs | 0.31% |
| `Arbiter::route_token` | 4.2 µs | 0.42% |

**Общая оценка:** Shell V3.0 и EventGenerator — быстрые, не требуют оптимизации. Arbiter маршрутизация ~4 µs, пороги не влияют на скорость. AshtiCore pipeline ~40 µs — основной потребитель при интенсивной обработке.

---

## Замеченные аномалии — требуют наблюдения

1. **AshtiCore/traces/10** — экстремальная вариация (43–251 µs). Вероятно, аллокаторные паузы при создании AxiomEngine в iter_batched. Требует профилирования без iter_batched.
2. **resonance_search/500** — вариация 5.6–7.7 µs. Граничный эффект HashMap.
3. **Arbiter strict vs loose** — разница <3%. Означает, что fast/slow path routing пока не создаёт асимметрии по скорости.

---

---

## История версий

| Версия | Дата | Что добавлено |
|--------|------|--------------|
| v1 | 2026-03-27 | Baseline: axiom-core, axiom-space, EventGenerator, resonance_search, AxiomEngine (add_domain, InjectToken, TickForward, Snapshot) |
| v2 | 2026-03-27 | AshtiCore pipeline, Shell V3.0 (compute/incremental/reconcile), Arbiter thresholds; рефактор engine_bench под новый API |
