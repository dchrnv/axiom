# Axiom Roadmap

**Версия:** 53.0  
**Дата:** 2026-05-17

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

**1332 тестов, 0 failures.**
FrameWeaver V1.3, DREAM Phase V1.0, Workstation V1.0, axiom-node, Axiom Sentinel V1.1, Phase C (C1..C5) завершены.

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

### Фаза I — Integration Sprint («подтянуть хвосты»)

Цель: замкнуть петлю. Phase C построила компоненты — Phase I заставляет их работать вместе.
Без этой фазы AE/CR/NA существуют в vacuum: не запускаются, не видят друг друга, не дают данных.

**Зависимости:** Phase C полностью завершена.

---

#### I1 — Engine: подключить Phase C компоненты

**Проблема:** `over_domain_components: Vec::new()` — AE, CR, NA нигде не инстанцированы и не вызываются.

**Что сделать:**

Добавить конкретные поля в `AxiomEngine` (аналогично `frame_weaver`):

```rust
axial_evaluator: AxialEvaluator,
context_recognizer: ContextRecognizer,
neural_advisor: NeuralAdvisor,
```

В tick loop после соответствующих интервалов — вызывать `on_tick` и **синхронизировать снапшоты**:

```
tick % 5 == 0 → ae.on_tick() → cr.sync_axial_store(ae.axial_store())
                                na.sync_axial_store(ae.axial_store())
tick % 7 == 0 → cr.on_tick() → na.sync_profile_store(cr.profile_store())
                                na.sync_depth_store(cr.depth_store())
tick % 11 == 0 → na.on_tick() → process ucl_commands
```

Закрывает: CR-TD-01, NA-TD-01.

**Файлы:** `engine.rs`, `engine/new()`.

---

#### I2 — ContextRecognizer: from_anchor_set конструктор

**Проблема:** `ContextRecognizer::new(HashMap::new())` — субсистем нет, энергии не считаются, все профили `SubsystemId::Unknown`.

**Что сделать:**

```rust
impl ContextRecognizer {
    pub fn from_anchor_set(anchors: &AnchorSet) -> Self {
        let subsystem_refs = build_subsystem_refs(anchors);
        Self::new(subsystem_refs)
    }
}
```

`build_subsystem_refs` группирует якоря из AnchorSet по тегам (subsystem_id: writing/mathematics/…)
и извлекает их позиции как опорные точки.

Движок использует этот конструктор при старте: `ContextRecognizer::from_anchor_set(&anchor_set)`.

Закрывает: CR-TD-03.

**Файлы:** `context_recognizer/mod.rs`, `engine.rs`.

---

#### I3 — Якорный контент: примитивы подсистем

**Проблема:** без семантических примитивов Writing/Mathematics — ContextRecognizer не распознаёт подсистемы.

**Что сделать:**

Создать YAML-файлы якорных примитивов для двух подсистем:
- `config/anchors/subsystems/writing_primitives.yaml` — существительное, глагол, метафора, нарратив, образ, ритм, смысл...
- `config/anchors/subsystems/mathematics_primitives.yaml` — число, операция, доказательство, множество, функция, граф, аксиома...

Каждый примитив: id, position ([i16;3] в семантическом пространстве), subsystem_id, описание.

Параллельно: заполнить оставшиеся слоевые якоря (L1–L4, L6–L8) из DEFERRED Anchor-Fill — они нужны TextPerceptor для осмысленного позиционирования токенов.

**Файлы:** `config/anchors/subsystems/`, `config/anchors/layers/`.

---

#### I4 — Engine: ApproveEmergentCandidate handler

**Проблема:** UCL 5201 определён, но Engine его игнорирует — оператор не может одобрить кандидата.

**Что сделать:**

```rust
OpCode::ApproveEmergentCandidate => {
    let payload = ApproveEmergentCandidatePayload::from_bytes(&cmd.payload);
    self.neural_advisor.approve_emergent(payload.sutra_id);
}
```

Закрывает: NA-TD-03.

**Файлы:** `engine.rs`.

---

#### I5 — OBS-01: живое наблюдение

**Проблема:** система никогда не запускалась с Phase C активной на живых данных.

**Что сделать:**

После I1–I4 — запустить `axiom-node` + Workstation. Подавать тексты через TextPerceptor.
Зафиксировать (из DEFERRED OBS-01):

1. Какие Frame кристаллизуются? На каких текстах?
2. Какие SubsystemId определяет ContextRecognizer? Правильно ли?
3. Есть ли конфликты octant analytic vs synthetic в AxialEvaluator?
4. Появляются ли emergent-кандидаты в NeuralAdvisor?
5. Первый `NotifyEmergentCandidate` в UCL — при каких условиях?
6. Корректно ли работают пороги DepthThresholdEmergentDetector (8000/30/100)?

Результат: список наблюдений → tuning порогов → возможные errata по компонентам.

Закрывает: OBS-01 из DEFERRED.

---

#### I6 — Workstation: Phase C visibility

**Проблема:** оператор не видит что происходит в AE/CR/NA — только Frame в EXPERIENCE.

**Что сделать:**

Расширить существующие вкладки Workstation минимально:

- **Patterns tab**: добавить текущий октант Frame (из AxialStore) и dominant SubsystemId (из InterpretationProfileStore)
- **Dream State tab** или отдельная панель: список emergent-кандидатов из EmergentPrimitiveStore с кнопкой Approve (→ UCL 5201)
- **System Map**: цвет домена EXPERIENCE отражает доминирующую подсистему

**Файлы:** `axiom-workstation/src/ui/patterns.rs`, `axiom-broadcasting`, `axiom-protocol`.

Зависимость: I1 (данные должны течь), I5 (OBS-01 покажет что важно отображать).

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
