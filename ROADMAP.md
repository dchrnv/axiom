# Axiom Roadmap

**Версия:** 48.0  
**Дата:** 2026-05-12

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
                                                    ↑
                                               axiom-broadcasting
                                                    ↑
                                               axiom-workstation
```

**1183 тестов, 0 failures.** FrameWeaver V1.3, Workstation V1.0 + UI polish, protocol extensions завершены.

---

## Фазы работы

### Фаза A — «Живая Workstation» 🔑

**Главный приоритет.** Все остальные фазы либо разблокируются после A, либо независимы от него.

### Фаза S — Axiom Sentinel V1.1 ⚡

**Источник:** `docs/architecture/Axiom Sentinel v1.0.md`  
**Цель:** адаптивный когнитивный движок — ≤50 ns/тик, ≤10 ms gravity 1M, ≤5 µs resonance 10K.

Уже реализовано (не делаем): worker pool `available_parallelism`, `resonance_search_parallel`, `AdaptiveTickRate` 60–1000 Hz.

---

#### S0 — Rayon pool: once_cell (пре-реквизит бенчей)

**Где:** `crates/axiom-runtime/src/engine.rs`

`AxiomEngine::try_new` вызывает `rayon::ThreadPoolBuilder::new().build()` — стоит ~1.34 ms, из которых ~567 µs это pool construction. Перенести в `once_cell::sync::Lazy<rayon::ThreadPool>` (глобальный или per-process). Без этого `AxiomEngine::new` будет искажать бенчи при каждом вызове.

**Результат:** `AxiomEngine::new` < 800 µs.

---

#### S1 — Direct Sensor Injection

**Где:** `crates/axiom-runtime/src/engine.rs`

Добавить `pub fn inject_token_direct(&mut self, domain_id: u16, token: Token) -> Result<usize, CapacityExceeded>` — обходит UCL parsing (~10–15 ns десериализации), напрямую вызывает `ashti.inject_token`. Горячий путь для сенсорных данных без Guardian-валидации (документировать явно). Существующий `process_command` остаётся для доверенного ввода.

**Результат:** inject latency 20 ns вместо ~35 ns.

---

#### S2 — Dynamic Memory Distillation

**Где:** `crates/axiom-arbiter/src/experience.rs`, `crates/axiom-runtime/src/engine.rs`

Три подзадачи:

1. `Experience::set_max_traces(n: usize)` — API вместо hardcoded 1000.
2. `traces_seen_total: u64` counter + `should_trigger_export() -> bool` (fires at кратных 5000). Arbiter вызывает `export_skills` при срабатывании.
3. `estimate_memory_bytes() -> usize` в Experience (~traces.len() × size_of::<ExperienceTrace>()). В periodic tick `TickSchedule::memory_pressure_threshold_bytes` (default 1.8 GiB) → если превышен, `run_horizon_gc()` немедленно минуя интервал.

**Результат:** автономная кристаллизация опыта, защита от OOM при длительных сессиях.

---

#### S3 — L2 Cache Chunk Sizing

**Где:** `crates/axiom-space/src/simd.rs`, `crates/axiom-space/src/lib.rs`

Добавить `const L2_CHUNK_TOKENS: usize = 65536` (512 KB / 8 bytes per token) и `apply_gravity_batch_chunked` — итерирует входные срезы окнами по чанку. Заменить вызов `apply_gravity_batch` на chunked-вариант в местах с потенциально большими N. Результат одинаковый, cache-miss минимизирован при > 1M токенов.

**Результат:** gravity batch 1M токенов: 25 ms → ближе к 10 ms.

---

#### S4 — AVX2 SIMD

**Где:** `.cargo/config.toml` (создать), `crates/axiom-bench/`

Шаг 1: создать `.cargo/config.toml` с `[profile.release] rustflags = ["-C", "target-cpu=native"]`. Scalar loop в `simd.rs` уже написан под auto-vectorisation. Запустить бенч 1M токенов.

Шаг 2 (если цель 8–10 ms не достигнута): явные AVX2 intrinsics в `#[cfg(feature = "simd")]` блоке через `std::arch::x86_64`. Scalar fallback остаётся.

**Результат:** `apply_gravity_batch` 1M: цель 8–10 ms.

---

#### S5 — Semantic Layer Hierarchy (бюджет тика)

**Где:** `crates/axiom-runtime/src/engine.rs`, `crates/axiom-runtime/src/orchestrator.rs`, `crates/axiom-arbiter/src/lib.rs`

`TickBudget` — `Instant` в начале тика, `budget_used_fraction() -> f32`. При > 80% бюджета пропускать роли 4–8 (MAP/PROBE/LOGIC/DREAM/ETHICS). Роли 1–3 (EXECUTION/SHADOW/CODEX) — всегда.

Gate за `TickSchedule::enable_layer_priority: bool` (default `false`) — существующие тесты не ломаются.

**Результат:** TickForward при высокой нагрузке стабилизируется в бюджете; L1–L3 рефлексы гарантированы.

---

#### S6 — Speculative Layer (отдельный этап, после S0–S5)

**Где:** `crates/axiom-space/src/lib.rs`, `crates/axiom-domain/src/domain_state.rs`, `crates/axiom-runtime/src/engine.rs`

Пока Arbiter обрабатывает тик N, свободные воркеры предвычисляют 2–3 вероятных состояния `SpatialHashGrid` для тика N+1. Zero-cost switch при совпадении (~9 µs vs ~40 µs полный rebuild).

**Требует:** отделить `SpatialHashGrid` от `DomainState` как самостоятельную speculatable единицу, добавить `SpatialHashGrid::snapshot/restore_from_grid_snapshot`. Высокая сложность, затрагивает ownership ~200+ тестов.

**Когда:** после стабилизации S0–S5 и бенчей.

---

### Фаза E — «Контент и инфраструктура»

#### E1 — Anchor-Fill: якорные YAML-файлы

14 файлов (L1–L8 кроме L5, D2–D8). ~7–10 якорей каждый. Делать вручную — это семантический
контент, не код. Диагностика: `:match "слово"` в CLI. Система работает без них (FNV-1a fallback).

**Когда:** По мере понимания семантики. Без дедлайна.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing. Очень низкий приоритет.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
