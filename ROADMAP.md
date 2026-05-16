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

### Фаза C — Knowledge Subsystems

Цель: семантическая инфраструктура — оценка смысла, подсистемы знания,
философские оси как активный оценщик.

**Зависимости:** C1 → C3 → C4 (axiom-experience нужен до AxialEvaluator,
AxialEvaluator нужен до ContextRecognizer). C2 независим.

---

#### C1 — axiom-experience: хранилища семантических данных

Новый крейт. Чистые data-структуры без сложной логики. Нужен как C3, так и C4.

**Зависимости:** только axiom-core (sutra_id: u32)

**Файлы:**

`crates/axiom-experience/src/`
- `lib.rs`
- `sutra_depth_store.rs` — `SutraDepthEntry { sutra_id, depth_per_octant: [u16; 8], last_settle_event: u64, reactivation_count: u32 }` + `SutraDepthStore (HashMap<u32, SutraDepthEntry>)`
- `axial_store.rs` — `AxialEvaluation`, `AxialScore`, `AxialDominant`, `Octant`, `AxialConflict`, `ConflictResolution`, `EvaluationLevel`, `AxialStore (HashMap<u32, Vec<AxialEvaluation>>)`
- `interpretation_profile_store.rs` — `InterpretationProfile`, `FrameComposition (C0..C5_Plus)`, `InterpretationProfileStore`
- `emergent_primitive_store.rs` — `EmergentPrimitive`, `EmergentPrimitiveStore`

**Тесты:** unit-тесты для каждого store (insert/get/update/remove)

---

#### C2 — AnchorSet: архитектура подсистем

Расширить `axiom-config::AnchorSet` для поддержки нескольких знаниевых подсистем.
Параллельно с C1 — не зависит от него.

**Изменения в коде:**

`crates/axiom-config/src/anchor.rs`:
- Добавить `subsystems: HashMap<String, Vec<Anchor>>` в `AnchorSet`
- `load_subsystems(anchors_dir)` — сканирует `anchors/{name}/` директории,
  загружает все `*.yaml` с ключом `anchors:`
- `all_anchors()` и `total_count()` включают subsystem-якоря
- `get_subsystem(name) -> &[Anchor]` — доступ по имени подсистемы

**Новые YAML-файлы:**

`config/anchors/writing/primitives.yaml` — 7 графических примитивов
(из `Writing_V1_0.md`): dot, vline, hline, dslash, bslash, hook, arc.
Позиции [0..32767]³, Вариант Б.

`config/anchors/mathematics/primitives.yaml` — 7 математических примитивов
(из `Mathematics_V1_0.md`): element, function, relation, operation, limit, group, fractal.

`config/anchors/octants.yaml` — 8 архетипов октантов (Величие, Мудрость,
Власть, Равновесие, Экстаз, Мечта, Ярость, Потенциал). Живут в SUTRA.

`config/anchors/semantic_centers.yaml` — ~10 якорей: Истина [15000,15000,15000],
Ложь [500,500,500], Жизнь, Смерть, Бытие, Небытие и т.д.

**Тесты:** загрузка subsystem-директории, total_count включает subsystems,
match_text находит primitve-якоря.

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

#### C4 — ContextRecognizer

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
- `advisors/` — `NeuralAdvisor` trait (stub в V1)

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
