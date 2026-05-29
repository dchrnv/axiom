# Axiom Roadmap

**Версия:** 66.0  
**Дата:** 2026-05-29

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
axiom-corpus                                        ↑
                                               axiom-broadcasting
```

**1631 тестов, 0 failures.**  
Phases E–H завершены. NeuralAdvisor V3, OverDomainArbiter V3, DREAM Phase V1.1, CR V6 — в продакшне.  
Workstation V2, axiom-node, axiom-corpus — в продакшне.  
V7 (A–E) завершён: TransitionMatrix, FatigueStore→experience, directed cascade, CompositeSubsystem, SubsystemVersionStore, SplitMergeDetector, SubsystemDependencies, EmergentSubsystemRules (GUARDIAN), L0VisionPerceptor.

---

## Активные задачи

---

## Sprint: Performance & Tooling

> **Контекст:** OBS 1M-тиковый прогон был прерван — занял 2+ часа без прогресса на экране. Корень: токены накапливаются без eviction (~64K к концу корпуса), каждый тик становится O(n) тяжелее. Sprint устраняет это и добавляет нормальный инструментарий.

---

### PERF-01 — Token Lifecycle (decay / eviction) 🔴

**Почему критично:** без eviction engine накапливает токены без ограничения. При inject_count=1600 × 40 текстов = 64K токенов к тику ~960K; avg ~32K на тик → каждый тик ~7ms → 1M тиков = 2 часа. Это не проблема алгоритма — это отсутствие жизненного цикла токена.

**Что нужно:**

1. **Energy decay** — каждый тик токен теряет `energy × decay_rate` (например 0.001). Когда `energy < eviction_threshold` → удалить из пространства. Настраивается в `corpus.yaml` или `engine_config`.
2. **Age-based TTL** — опциональный параметр `max_age_ticks` в `InjectionConfig`; токен удаляется по достижению возраста.
3. **Max tokens cap** — `engine.max_live_tokens` в конфиге; при превышении вытесняется LRU (по last_active тику).
4. **Eviction hook** — при удалении токена опционально записывать в EXPERIENCE (short trace) если токен участвовал в кристаллизациях.

**Ожидаемый результат:** стабильный пул ≤ N_max токенов → время тика не растёт со временем → 1M тиков за минуты, не часы.

**Где реализовывать:** `axiom-core` (Token age поле), `axiom-space` (eviction в batch-step), `axiom-runtime` (engine eviction hook), `axiom-observe` (конфиг).

---

### PERF-02 — Профилирование горячего пути при большом N

**Что нужно:** запустить OBS с малым корпусом (10K тиков, inject_count=500) и `cargo flamegraph` или `perf record` чтобы точно знать что занимает время при N=10K, 30K, 60K токенов. Построить график `tick_time vs N`.

**Ожидаемый результат:** данные для приоритизации — что именно растёт быстрее всего (gravity, grid rebuild, over_domain pipeline, experience search).

**Когда:** параллельно с PERF-01 или после, как уточнение.

---

### PERF-03 — Параллелизм 🔴

**Контекст:** движок однопоточный. При наличии 8+ ядер всё время тика проходит на одном. Parallelism — следующий порядок после eviction по impact на OBS throughput.

**Три уровня параллелизма:**

#### 3a. Параллельный тик доменов (rayon, внутри одного тика)

Текущее состояние: `AshtiCore::tick()` обходит 11 доменов **последовательно**. Каждый домен вызывает `process_frontier` — независимая операция (домены не пишут друг в друга во время тика, только читают). Уже есть прецедент: `prepare_speculative_grids()` использует rayon для параллельного rebuild.

**Что нужно:**
- Разделить состояние для мутабельного доступа: `domains` и `states` — `split_at_mut` или `Arc<Mutex<>>` на каждый домен
- Запускать `process_frontier` для 11 доменов через `rayon::scope`
- Собирать события в `Vec<Vec<Event>>`, flatten после join
- Ожидаемый прирост: 11 доменов → теоретически 11x, реально 3–5x с учётом sync overhead

**Где:** `crates/axiom-domain/src/ashti_core.rs` → `tick()`

#### 3b. Параллельный OBS: несколько corpus shards на разных потоках ✅

**Реализовано:** `crates/axiom-observe/src/shard.rs` — round-robin split, `std::thread::spawn`, merge (events concat, snapshots от shard 0). `corpus_large.yaml` → `shards: 4`. Ожидаемый прирост: ~4x на 4 ядрах.

#### 3c. SIMD/AVX2 расширение горячих путей

Текущее состояние: `apply_gravity_batch_avx2` реализован (axiom-space), но не все горячие операции покрыты SIMD.

**Что нужно:**
- `resonance_search` в ExperienceModule — SIMD-сравнение `[u8; 8]` shell-профилей
- `scan_region` в ContextRecognizer — параллельная фильтрация MAYA токенов по октанту через bitwise SIMD
- Измерить через `over_domain_bench` до/после

**Когда:** после 3a и 3b (более высокий impact, меньше риск), 3c — точечная оптимизация.

**Ожидаемый суммарный результат:** 1M тиков за 5–15 минут вместо 2+ часов; OBS становится практичным инструментом для регулярных прогонов.

---

### OBS-01 — Progress reporting в axiom-observe 🟡

> (из DEFERRED OBS-TD-01 — перенесён как активная задача)

**Где:** `crates/axiom-observe/src/runner.rs` → `run()`

**Что:** `eprintln!("[observe] {tick}/{total} ({pct:.0}%) elapsed={elapsed:.0}s eta={eta:.0}s")` каждые 50K тиков или 10%. Elapsed + ETA через `std::time::Instant`. Дополнительно: финальная строка `[observe] done in {total_secs:.1}s ({ticks_per_sec:.0} ticks/sec)`.

**Критерий готовности:** запустить 100K-тиковый прогон — на экране видно прогресс каждые ~10 секунд.

---

### OBS-02 — Streaming output (не накапливать в памяти) 🟡

> (из DEFERRED OBS-TD-04)

**Где:** `crates/axiom-observe/src/runner.rs`, `crates/axiom-observe/src/report.rs`

**Что:**
- Снапшоты писать в `obs_out/snapshots.jsonl` по мере накопления (append), не держать `Vec<Snapshot>` в памяти
- Events — аналогично в `obs_out/events.jsonl`
- `report.md` генерировать в конце из файлов (не из RAM)
- Параметр `flush_every: 1000` (снапшотов) в corpus.yaml

**Ожидаемый результат:** RAM стабильна на протяжении прогона (~20MB вместо 110MB+).

---

### OBS-03 — Калибровка корпуса для showcase

**Что нужно:** после PERF-01 пересчитать `corpus_large.yaml` под новые параметры:
- `inject_count` подобрать под `max_age_ticks` / decay так чтобы в engine жило N_max ≤ 5000 токенов
- `ticks_total` — сколько нужно для репрезентативного snaphot-набора (скорее всего 100K–200K достаточно)
- Добавить `corpus_showcase.yaml` как "быстрый прогон для демонстрации" (~5 мин)

---

### DEV-01 — Lab Panel в Workstation V2 UI 🟢

> (из DEFERRED DEV-PANEL-01 — перенесён как активная задача)

Вкладка **"Lab"** в Workstation V2 для запуска инструментов прямо из браузера.

**Серверная часть (axiom-node):**
- Endpoint `POST /api/lab/run` с `{ job: "obs" | "bench_hot" | "bench_od" | "bench_stress" | "test" | "showcase" }` + опциональные параметры (corpus path, bench filter)
- `tokio::process::Command` — спавнит нужный процесс, stdout/stderr → WebSocket канал `/ws/lab/log`
- State machine: `Idle → Running → Done | Failed`; один активный job одновременно
- По завершении — автосохранение артефактов в нужные файлы (`showcase/SHOWCASE.md`, `bench_out/*.txt`)

**UI (React, Workstation V2):**
- **Run-панель:** кнопки OBS / Hot Bench / OverDomain Bench / Stress / Tests / Full Showcase; при выборе OBS — дропдаун корпуса
- **Лог-монитор:** `<pre>` с auto-scroll, цветная подсветка `[observe] …%`, `criterion … time:`, `test … ok/FAILED`; кнопка "Stop"
- **Progress bar:** парсинг строк `[observe] {n}/{total} ({pct}%)` → визуальная полоса с % и ETA
- **Results panel:** после завершения — красивое отображение итогов:
  - OBS → таблица subsystem accuracy + энергетика + граф снапшотов (линейный chart)
  - Bench → таблица time/thrpt с delta к предыдущему прогону
  - Tests → счётчик passed/failed + список failed
- **История прогонов:** последние N результатов в sidebar с timestamp + статусом

**Критерий готовности:** нажать "Full Showcase" в браузере → видеть прогресс в реальном времени → после завершения видеть результаты → `showcase/SHOWCASE.md` обновлён автоматически.

---

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **AE-TD-08** — Full semantic connections at injection time. Приоритет повышается в V7 (связан с L0 bonds). См. DEFERRED.md.
- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами. После стабилизации SubsystemVersioning. См. §2.8.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
