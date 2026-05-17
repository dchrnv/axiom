# Axiom Roadmap

**Версия:** 52.0  
**Дата:** 2026-05-16

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

**1202 тестов, 0 failures.**
FrameWeaver V1.3, DREAM Phase V1.0, Workstation V1.0, axiom-node, Axiom Sentinel V1.1 завершены.

---

## Фазы работы

### Фаза A — axiom-node: Живая Workstation ✅
### Фаза S — Axiom Sentinel V1.1 ✅

---

### Фаза C — Knowledge Subsystems ✅ C1+C2+C3+C4+C5

Цель: семантическая инфраструктура — оценка смысла, подсистемы знания,
философские оси как активный оценщик.

**Зависимости:** C1 → C3 → C4 (axiom-experience нужен до AxialEvaluator,
AxialEvaluator нужен до ContextRecognizer). C2 независим.

#### C1 — axiom-experience ✅
#### C2 — AnchorSet: архитектура подсистем ✅
#### C3 — AxialEvaluator ✅
#### C4 — ContextRecognizer ✅
#### C5 — NeuralAdvisor ✅

---

#### C3 — AxialEvaluator

Пятый над-доменный модуль. Превращает оси X/Y/Z из статических координат
в активный оценщик. Спека: `docs/architecture/AxialEvaluator_V1_0.md`.

**Зависимости:** C1 (axiom-experience::axial_store)

**Подготовка:**
- Добавить `AxialEvaluator` в `ModuleId` в `axiom-genome/src/types.rs`
- Access rules в `Genome::default_ashti_core()` и `config/genome.yaml`

**Файлы:**

`crates/axiom-runtime/src/over_domain/axial_evaluator/`
- `mod.rs` — `AxialEvaluator` struct, `OverDomainComponent` impl, `on_tick`
- `metrics.rs` — `entropy_score`, `graph_density`, `will_score` (по участникам Frame)
- `synthesis.rs` — `synthesize_octant` (целостное распознавание архетипа по Frame)
- `conflict.rs` — `AxialConflict` детектор: analytic vs synthetic octant
- `levels.rs` — `determine_applicable_levels(frame, participants)` → `Vec<EvaluationLevel>`
- `storage.rs` — запись `AxialEvaluation` в `axiom-experience::AxialStore` через UCL

**UCL:**
- `ProposeAxialAdjustment { sutra_id: u32, suggested_position: [i16; 3], reason: u8 }` — новый OpCode

**Интеграция в AxiomEngine:** регистрация через `over_domain` список, вызов `on_tick`.

**Тесты:** entropy_score детерминирован, octant derivation из трёх score,
конфликт когда analytic ≠ synthetic, EvaluationLevel::Conceptual срабатывает для L5-Frame.

---

#### C4 — ContextRecognizer ✅

Четвёртый над-доменный модуль. Отвечает на вопрос "что мы понимаем и в каком режиме".
Спека: `docs/architecture/ContextRecognizer_V5_0.md`.

**Зависимости:** C1 (все stores) + C3 (AxialEvaluator через `axial_bridge`)

**Подготовка:**
- `ContextRecognizer` в `ModuleId`, access rules в genome

**Файлы:**

`crates/axiom-runtime/src/over_domain/context_recognizer/`
- `mod.rs` — `ContextRecognizer`, `OverDomainComponent` impl
- `scanning_plan.rs` — `ScanningPlan`, `ActiveRegion`, `DepthRange`
- `scanner.rs` — сканирование MAYA по `ScanningPlan` (octant × depth_range × FractalLevel)
- `energy.rs` — подсчёт энергий активных регионов
- `profile.rs` — `InterpretationProfile` + запись в store
- `transitions.rs` — детектор переключений между подсистемами
- `conflicts.rs` — разрешение конфликтов подсистем
- `learning.rs` — обновление позиций + обновление `SutraDepth` в DREAMING
- `hot_reload.rs` — UCL-команда `RefreshPrimitiveScan`
- `axial_bridge.rs` — `current_active_octants()` через AxialEvaluator
- `depth_bridge.rs` — мост к `SutraDepthStore`
- `snapshot.rs` — `ContextSnapshot` для записи в EXPERIENCE
- `emergent/` — детектор эмерджентных примитивов (stub в V1)

**UCL:**
- `RefreshPrimitiveScan`
- `QueryDepthDistribution { octant: u8 }` — для Workstation
- `ResetDepthForFrame { sutra_id: u32 }` — debug через GUARDIAN

**Тесты:** ScanningPlan формируется корректно, scan_region фильтрует по depth_range,
SutraDepth обновляется только в DREAMING, конфликт двух активных подсистем.

---

### Фаза E — «Контент и инфраструктура»

#### E1 — Anchor-Fill: якорные YAML-файлы

14 файлов (L1–L8 кроме L5, D2–D8). ~7–10 якорей каждый.
Делать вручную — это семантический контент, не код.

**Когда:** по мере понимания семантики. Без дедлайна.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **WS-V2-***, **COMP-01** — V2.0 идеи и Companion. См. DEFERRED.md.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
