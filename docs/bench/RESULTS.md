# Axiom Benchmark Results

**v14 · 2026-06-13** · AMD Ryzen 5 3500U · 8t · Linux x86-64 · criterion 0.5 · `release`

---

## Быстрая справка — ключевые числа

| Операция | Время | Δ vs v13 |
|----------|-------|----------|
| TickForward / 50 токенов (hot path) | **21.4 µs** | -11.7% |
| InjectToken (engine) | **16.9 µs** | -63% |
| AxiomEngine::new | **438 µs** | -59% |
| resonance_search / 1000 traces | **13.9 µs** | -72% |
| Arbiter::route_token | **6.6 µs** | -66% |
| 100K ticks / пустой движок | **2.16 s** → 46.4K tick/s | +44% thrpt |
| 1M ticks / пустой движок | **23.3 s** → 43.0K tick/s | +28% thrpt |
| Sustained / 50 tok realistic | **21.3 ms/1Kt** → 46.9K tick/s | +57% thrpt |
| SpatialHashGrid::rebuild / 1M | **7.93 ms** | -22% |
| apply_subsystem_gravity / 10K | **75.8 µs** | -11% |

---

## core_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| Token::new | 17.8 ns | -75% ↑ |
| Token copy (Copy trait) | 24.6 ns | -69% ↑ |
| Token::compute_resonance | 7.7 ns | ~ |
| Event::new | 19.3 ns | -72% ↑ |
| Connection::default | 16.5 ns | -76% ↑ |
| struct_sizes/Token 64B | 665 ps | -76% ↑ |
| struct_sizes/Connection 64B | 664 ps | -76% ↑ |
| struct_sizes/Event 64B | 666 ps | -75% ↑ |

---

## domain_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| EventGenerator::check_decay | 99.9 ns | -70% ↑ |
| EventGenerator::generate_collision | 18.3 ns | -78% ↑ |
| Experience::resonance_search / 0 | 194 ns | -72% ↑ |
| Experience::resonance_search / 10 | 395 ns | -69% ↑ |
| Experience::resonance_search / 100 | 1.62 µs | -75% ↑ |
| Experience::resonance_search / 500 | 6.97 µs | -73% ↑ |
| Experience::resonance_search / 1000 | 13.9 µs | -72% ↑ |
| Arbiter::route_token / strict_200_180 | 6.6 µs | -66% ↑ |
| Arbiter::route_token / loose_50_30 | 6.7 µs | -59% ↑ |

---

## engine_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| AxiomEngine::new / full | 438 µs | -59% ↑ |
| AxiomEngine::new / AshtiCore only | 300 µs | -63% ↑ |
| AxiomEngine: InjectToken | 16.9 µs | -63% ↑ |
| AxiomEngine: TickForward / 0 tok | 21.8 µs | -46% ↑ |
| AxiomEngine: TickForward / 10 tok | 21.7 µs | -52% ↑ |
| AxiomEngine: TickForward / 50 tok | 22.4 µs | -52% ↑ |
| AxiomEngine: TickForward / 100 tok | 22.1 µs | -19% ↑ |
| AxiomEngine: snapshot / 0 tok | 7.19 µs | -68% ↑ |
| AxiomEngine: snapshot / 100 tok | 14.5 µs | -49% ↑ |
| AxiomEngine: restore_from / 0 tok | 564 µs | -58% ↑ |
| AxiomEngine: restore_from / 100 tok | 591 µs | -54% ↑ |
| AshtiCore: full pipeline / 0 traces | 50 µs | ~ |
| AshtiCore: full pipeline / 100 traces | 57.3 µs | -33% ↑ |
| AxiomEngine: run_adaptation / 0 | 29 µs | -55% ↑ |
| AxiomEngine: causal_horizon | 38 ns | -62% ↑ |
| AxiomEngine: export_skills | 8.7 ns | -72% ↑ |
| Gateway: process_channel / 50 cmds | 1.09 ms | -46% ↑ |
| domain_detail_snapshot / t10_c50 | 3.8 µs | **+46% ↓** |
| domain_detail_snapshot / t50_c250 | 16.8 µs | -54% ↑ |
| domain_detail_snapshot / t200_c2000 | 55.9 µs | -53% ↑ |

---

## hot_path_regression

| Benchmark | Время | Δ |
|-----------|-------|---|
| TickForward / 50 tok (regression guard) | **21.4 µs** | **-11.7% ↑** |

---

## integration_bench

| Benchmark | Время | Throughput | Δ |
|-----------|-------|-----------|---|
| 100K ticks / empty | 2.16 s | 46.4K tick/s | +44% ↑ |
| 100K ticks / 50 tokens | 2.22 s | 45.1K tick/s | +36% ↑ |
| 100K ticks / 50tok+100tr default | 2.26 s | 44.2K tick/s | +33% ↑ |
| 100K ticks / 50tok max_schedule | 2.84 s | 35.2K tick/s | +27% ↑ |
| 1M ticks / empty | 23.3 s | 43.0K tick/s | +28% ↑ |
| 1M ticks / 50tok hot_only | 24.2 s | 41.2K tick/s | +27% ↑ |
| 1M ticks / 50tok default_schedule | 25.7 s | 38.9K tick/s | +18% ↑ |
| inject_tick_reconcile cycle | 101 µs | ~ | ~ |
| 1000 ticks then snapshot | 24.9 ms | -22% ↑ |

---

## frameweaver_overhead

| Benchmark | Время | Δ |
|-----------|-------|---|
| A: disabled / 0 tok | 21.9 µs | -31% ↑ |
| A: disabled / 50 tok | 22.4 µs | -25% ↑ |
| B: active, MAYA empty / 50 tok | 22.2 µs | -32% ↑ |
| C: active, 5 MAYA patterns / 50 tok | 27.4 µs | -34% ↑ |
| D: active, 20 MAYA patterns / 50 tok | 46.8 µs | -30% ↑ |
| scan_state / 0 patterns | 11.1 ns | -38% ↑ |
| scan_state / 5 patterns | 2.33 µs | -34% ↑ |
| scan_state / 20 patterns | 10.7 µs | -30% ↑ |
| scan_state / 50 patterns | 30.4 µs | -34% ↑ |

---

## over_domain_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| tick_cold / 0 tok | 144 µs | ~ |
| tick_cold / 50 tok | 145 µs | -16% ↑ |
| tick_cold / 200 tok | 136 µs | -12% ↑ |
| tick_warm_100 / 0 tok | 54.5 µs | -35% ↑ |
| tick_warm_100 / 50 tok | 54.8 µs | -22% ↑ |
| tick_warm_100 / 200 tok | 56.6 µs | -29% ↑ |
| tick_loaded_1000 / 50 tok | 73.4 µs | -12% ↑ |
| tick_loaded_1000 / 200 tok | 92.0 µs | -9% ↑ |
| throughput / 1000t / 50 tok | 21.9 ms | -21% ↑ |
| throughput / 1000t / 200 tok | 22.7 ms | -15% ↑ |
| inject_loaded / after_1000t_200tok | 41.3 µs | **+26% ↓** |

---

## stress_bench (sustained)

| Benchmark | Время | Throughput | Δ |
|-----------|-------|-----------|---|
| realistic / 50tok | 21.3 ms/1Kt | **46.9K tick/s** | +57% ↑ |
| heavy / 200tok max_schedule | 36.2 ms/1Kt | **27.6K tick/s** | +44% ↑ |
| baseline hot_only / 50tok | 21.3 ms/1Kt | **47.0K tick/s** | +45% ↑ |
| SpatialHashGrid::rebuild / 10K | 60.4 µs | -23% ↑ |
| SpatialHashGrid::rebuild / 500K | 3.90 ms | -21% ↑ |
| SpatialHashGrid::rebuild / 1M | 7.93 ms | -22% ↑ |
| resonance_search / 1K | 15.8 µs | -14% ↑ |
| resonance_search / 10K | 16.2 µs | -19% ↑ |
| apply_subsystem_gravity / 100 | 7.81 µs | -8% ↑ |
| apply_subsystem_gravity / 5K | 59.0 µs | -24% ↑ |
| apply_subsystem_gravity / 10K | 75.8 µs | -11% ↑ |

---

## shell_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| compute_shell / 0 conn | 10.7 ns | ~ |
| compute_shell / 20 conn | 238 ns | -18% ↑ |
| compute_shell / 100 conn | 1.29 µs | -11% ↑ |
| incremental_update / 50 tok | 1.83 µs | -19% ↑ |
| incremental_update / 100 tok | 3.68 µs | **+5% ↓** |
| reconcile_batch / 10 | 732 ns | **+9% ↓** |
| reconcile_batch / 50 | 1.87 µs | -8% ↑ |

---

## space_bench

| Benchmark | Время | Δ |
|-----------|-------|---|
| SpatialHashGrid::rebuild / 1K | 10.6 µs | -26% ↑ |
| SpatialHashGrid::rebuild / 5K | 31.3 µs | -27% ↑ |
| find_neighbors / 100 | 211 ns | -22% ↑ |
| find_neighbors / 1K | 1.33 µs | ~ |
| distance2 | 4.06 ns | ~ |

---

## Регрессии (требуют наблюдения)

| Benchmark | Δ | Вероятная причина |
|-----------|---|-------------------|
| domain_detail_snapshot/t10_c50 | +46% | аномалия измерения (outliers), реальный t50_c250 улучшился -54% |
| over_domain/inject_loaded/after_1000t_200tok | +26% | нагруженное состояние + новые мембранные трансформы |
| Shell::reconcile_batch/10 | +9% | в пределах noise threshold |
| Shell::incremental_update/100 | +5% | в пределах noise threshold |

---

## История версий

| v | Дата | Ключевые изменения |
|---|------|-------------------|
| v14 | 2026-06-13 | REPAIR-01 (-1229 строк гравитации), dream_interval=0, мембранные профили, OBS-ACC |
| v13 | 2026-06-05 | SEN-TD-01 (BroadcastSnapshot→SensoriumState), DIL-TD-01, compute_confidence ±8 |
