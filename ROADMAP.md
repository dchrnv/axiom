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

### PERF-01 — Token Lifecycle (decay / eviction) ✅

По дизайну токены **не удаляются** — они угасают до инертного состояния (`STATE_SLEEPING`).

1. **Decay через TokenDecayed события** — `EventGenerator::check_decay()` уже генерирует `TokenDecayed` когда причинный возраст токена превышает порог (1/decay_rate событий). Теперь engine применяет эти события: `apply_token_decay_events()` в `tick_wake()` — переводит токен в `STATE_SLEEPING`, `valence=0`.
2. **Scan filtering** — `scan_region()` в ContextRecognizer уже фильтрует `t.state == STATE_ACTIVE`. Спящие токены автоматически невидимы для всех сканеров.
3. **Capacity management** — `DomainState::add_token()` при переполнении вызывает `evict_sleeping(1)` — освобождает слот от спящего токена для нового активного.
4. **Eviction hook** — при переходе в сон: если токен был connection-referenced → добавить trace в Experience (weight=0.4).

---

### PERF-02 — Профилирование горячего пути при большом N ✅

**Инфраструктура создана:** `config/obs/corpus_profile.yaml` — 4 текста, 50K тиков, без decay/cap для наблюдения деградации. Инструкции внутри файла: `cargo flamegraph --bin axiom-observe` или `perf record`. Запускать после каждого значимого изменения движка для верификации прироста.

---

### PERF-03 — Параллелизм 🔴

**Контекст:** движок однопоточный. При наличии 8+ ядер всё время тика проходит на одном. Parallelism — следующий порядок после eviction по impact на OBS throughput.

**Три уровня параллелизма:**

#### 3a. Параллельный тик доменов (rayon, внутри одного тика) ✅

**Реализовано:** `AshtiCore::tick()` разбит на 2 прохода: (1) sequential — `on_event` + `handle_heartbeat` для 11 доменов; (2) parallel — `process_frontier` через `par_iter_mut().zip().zip()`. Домены независимы при обработке frontier → безопасный параллелизм без mutex.

#### 3b. Параллельный OBS: несколько corpus shards на разных потоках ✅

**Реализовано:** `crates/axiom-observe/src/shard.rs` — round-robin split, `std::thread::spawn`, merge (events concat, snapshots от shard 0). `corpus_large.yaml` → `shards: 4`. Ожидаемый прирост: ~4x на 4 ядрах.

#### 3c. SIMD/AVX2 расширение горячих путей ✅

**Статус:** `resonance_search_parallel()` уже реализован в ExperienceModule с rayon (fold/reduce, parallel threshold). `scan_region` автоматически пропускает STATE_SLEEPING токены (фильтр `STATE_ACTIVE`), что даёт основной прирост. `apply_gravity_batch_avx2` реализован в axiom-space. Дополнительная SIMD-оптимизация `pattern_similarity` — точечная и откладывается до данных профилировщика (PERF-02).

**Ожидаемый суммарный результат PERF-03:** 1M тиков за 5–15 минут благодаря 3a + 3b + STATE_SLEEPING фильтрации.

---

### OBS-01 — Progress reporting в axiom-observe 🟡

> (из DEFERRED OBS-TD-01 — перенесён как активная задача)

**Где:** `crates/axiom-observe/src/runner.rs` → `run()`

**Что:** `eprintln!("[observe] {tick}/{total} ({pct:.0}%) elapsed={elapsed:.0}s eta={eta:.0}s")` каждые 50K тиков или 10%. Elapsed + ETA через `std::time::Instant`. Дополнительно: финальная строка `[observe] done in {total_secs:.1}s ({ticks_per_sec:.0} ticks/sec)`.

**Критерий готовности:** запустить 100K-тиковый прогон — на экране видно прогресс каждые ~10 секунд.

---

### OBS-02 — Streaming output ✅

`run_streaming()` пишет снапшоты в `obs_out/snapshots.jsonl` и события в `obs_out/events.jsonl` через `BufWriter` по мере накопления. Vec в RAM не растёт. Report генерируется из файлов через `load_snapshots_jsonl()` / `load_events_jsonl()`. Metrics + Serde Serialize/Deserialize.

---

### OBS-03 — Калибровка корпуса для showcase ✅

`config/obs/corpus_showcase.yaml` — 18 текстов, 9 подсистем, 200K тиков, shards=4. При ~3-5 минутах прогона даёт репрезентативный snapshot-набор. `showcase.sh` использует его по умолчанию (`AXIOM_CORPUS=...` для переопределения). Для полного прогона: `corpus_large.yaml`.

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
