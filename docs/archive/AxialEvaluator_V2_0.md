# AXIOM — AxialEvaluator V2.0

**Статус:** Спецификация  
**Версия:** 2.0  
**Дата:** 2026-05-19  
**Предыдущая версия:** `AxialEvaluator_V1_0.md`  
**Категория:** Over-Domain Mechanism  
**Crate:** `axiom-runtime` / `over_domain/axial_evaluator/`

---

## 1. Что меняется относительно V1

V1 оценивал Frame корректно, но был «глухим»: результаты лежали в AxialStore и никуда не шли.
V2 даёт AxialEvaluator голос — через OverDomainArbiter.

**Три главных изменения:**

1. **AxialEvaluator реализует `AdvisorySource`** — генерирует `OctantCorrection` и `ConflictDiagnosis` рекомендации.
2. **OctantStabilityTracker** — следит за тем, насколько стабильно Frame держится в одном октанте; только стабильные оценки становятся рекомендациями.
3. **EvaluationLevel из ContextRecognizer** — вместо прямого маппинга Shell→Level уровень определяется по доминирующей подсистеме из ContextRecognizer.

Всё остальное из V1 остаётся: оси X/Y/Z, 8 октантов, Corpus Callosum, AxialStore.

---

## 2. OctantStabilityTracker

Главная новая структура данных.

```rust
pub struct OctantStabilityTracker {
    /// Последние K октантов для каждого Frame (ring buffer).
    history: HashMap<u32, VecDeque<Octant>>,
}

/// K — глубина истории на Frame.
pub const STABILITY_HISTORY_DEPTH: usize = 10;
/// Порог стабильности: доля одного октанта в истории.
pub const STABILITY_THRESHOLD: f32 = 0.7;
/// Минимальная история перед выдачей рекомендации.
pub const STABILITY_MIN_HISTORY: usize = 5;
```

Алгоритм:

```
after each evaluation of Frame F:
    tracker.push(F.sutra_id, eval.octant)

    if history.len() < STABILITY_MIN_HISTORY:
        skip

    dominant_octant, dominant_count = most_common(history)
    stability_score = dominant_count / history.len()

    if stability_score >= STABILITY_THRESHOLD:
        emit OctantCorrection advisory (confidence = stability_score)
```

Рекомендация не повторяется: после генерации `OctantCorrection` для Frame история
сбрасывается (`history.clear(frame_id)`) — ждём следующей серии оценок.

---

## 3. ConflictPersistenceTracker

Аналогично для конфликтов.

```rust
pub struct ConflictPersistenceTracker {
    /// Количество подряд идущих тиков с конфликтом для каждого Frame.
    streak: HashMap<u32, u32>,
}

/// После скольких тиков подряд с конфликтом генерируем рекомендацию.
pub const CONFLICT_PERSISTENCE_THRESHOLD: u32 = 5;
```

Алгоритм:

```
after each evaluation:
    if eval.has_conflict():
        streak[F] += 1
        if streak[F] >= CONFLICT_PERSISTENCE_THRESHOLD:
            emit ConflictDiagnosis advisory
            streak[F] = 0   // сброс чтобы не спамить
    else:
        streak[F] = 0
```

---

## 4. AxialEvaluator как AdvisorySource

AxialEvaluator реализует трейт `AdvisorySource` из `over_domain::arbiter::source`.

```rust
impl AdvisorySource for AxialEvaluator {
    fn source_id(&self) -> SourceId {
        AXIAL_EVALUATOR_SOURCE_ID  // = 1
    }

    fn poll_advisories(&self) -> Vec<Advisory> {
        self.pending_advisories.drain_all()
    }

    fn on_feedback(&mut self, id: AdvisoryId, outcome: AdvisoryOutcome) {
        // V2: no-op (будущий автотюнинг порогов в V3)
    }
}
```

`pending_advisories` — внутренний `Vec<Advisory>`, который заполняется в `on_tick`
через OctantStabilityTracker и ConflictPersistenceTracker.

**SourceId:**

| Source | SourceId |
|--------|----------|
| NeuralAdvisor | 0 |
| AxialEvaluator | **1** |

**TrustConfig V2 default** (добавить в `TrustConfig::default_v1`):

| Source | AdvisoryType | Режим | min_confidence |
|--------|--------------|-------|----------------|
| AxialEvaluator (1) | OctantCorrection | RequireConfirmation | 0.70 |
| AxialEvaluator (1) | ConflictDiagnosis | RequireConfirmation | 0.60 |

---

## 5. EvaluationLevel из ContextRecognizer

**V1:** уровень маппируется из Shell-профиля Frame (Shell L1 → Sensory, L5 → Conceptual и т.д.).

**V2:** уровень определяется по доминирующей подсистеме ContextRecognizer плюс Shell как тай-брейкер.

Таблица маппинга `SubsystemId → EvaluationLevel`:

| SubsystemId | Подсистема | EvaluationLevel V2 |
|-------------|------------|-------------------|
| 0 | Neutral | из Shell (fallback V1) |
| 1 | Cognitive | Conceptual (4) |
| 2 | Affective | Motivational (5) |
| 3 | Conative | Action (2) |
| 4 | Somatic | Sensory (1) |
| 5 | Social | Social (6) |
| 6 | Aesthetic | Imaginal (3) |
| 7 | Transcendent | Transcendent (8) |
| 8–15 | прочие | из Shell (fallback) |

Это делает уровень оценки **контекстно-зависимым**: один и тот же Frame в музыкальном
контексте оценивается на Imaginal, в математическом — на Conceptual.

**Что меняется в коде:** `levels.rs` получает опциональный `primary_subsystem: SubsystemId`
из снапшота ContextRecognizer; `determine_applicable_levels` использует таблицу выше.

---

## 6. Ограничения хранилища

V1 хранил все оценки без ограничений. V2 вводит cap:

```rust
pub const MAX_EVALUATIONS_PER_FRAME: usize = 20;
```

В `EvaluatorStorage::record()`: если Frame имеет ≥ 20 записей, удаляется самая старая
(по `computed_at_event`).

---

## 7. Advisory ID схема (AxialEvaluator)

Та же схема что у NeuralAdvisor: `(sutra_id as u64) << 8 | type_index`.

Чтобы не было коллизий между источниками, Arbiter различает их по `source` полю,
а не по `id` — ID уникален в рамках источника.

---

## 8. Что в коде

Изменения относительно V1:

```
over_domain/axial_evaluator/
├── mod.rs          — + OctantStabilityTracker, ConflictPersistenceTracker,
│                     + pending_advisories: Vec<Advisory>,
│                     + impl AdvisorySource
├── levels.rs       — determine_applicable_levels(shell, Option<SubsystemId>)
├── storage.rs      — record() с cap MAX_EVALUATIONS_PER_FRAME
├── stability.rs    — OctantStabilityTracker (новый файл)
├── persistence.rs  — ConflictPersistenceTracker (новый файл)
└── (остальное без изменений)
```

В `engine.rs`:

```rust
// Регистрируем AxialEvaluator как источник advisories
self.over_domain_arbiter.register_source(
    Box::new(AxialEvaluatorAdvisoryProxy::new(&self.axial_evaluator))
);
```

> **Проблема borrow:** AxialEvaluator не может быть одновременно `AdvisorySource` (требует
> `&mut self`) и полем AxiomEngine. Решение — `AxialEvaluatorAdvisoryProxy` собирает
> `pending_advisories` через `drain_pending()`, как NeuralAdvisor с `poll_advisories()`.

В `engine.rs` tick %5:

```rust
if t % 5 == 0 {
    // Передаём primary_subsystem из ContextRecognizer для определения уровня
    let primary_subsystem = self.context_recognizer.profile_store().dominant_primary();
    self.axial_evaluator.tick_with_context(t, ashti, primary_subsystem);
}
```

---

## 9. Что НЕ меняется

- Метрики X/Y/Z (entropy, graph_density, valence, will) — без изменений
- AxialStore — структура без изменений, только cap в record()
- Corpus Callosum conflict detection — без изменений
- Восемь октантов, их семантика — HARD
- Все INVARIANTS из V1 §11 остаются в силе

---

## 10. Известные ограничения V2

- **AE-TD-01** — `on_feedback` no-op. V3: автокалибровка `STABILITY_THRESHOLD` по
  соотношению confirmed/rejected.

- **AE-TD-02** — SubsystemId→EvaluationLevel таблица захардкожена. V3: вынести в конфиг.

- **AE-TD-03** — Нет оценки *последовательностей* Frame. V3: скользящее окно Frame
  создаёт «нарративную оценку» — октант нарратива как агрегат серии.

---

## 11. Что в V3+

- **V3:** `OctantCorrectionAdvisor` в NeuralAdvisor использует оценки AxialEvaluator
  как вход — два источника согласуются перед отправкой в Arbiter.
- **V3:** оценка последовательностей Frame → «нарративный октант».
- **V4:** интеграция с подсистемой Morality (слой 7, Existential).
- **V5:** генерация Frame с целевыми координатами в осях.

---

## История

- **V1.0** (2026-05-15): первая спецификация. AxialEvaluator как пятый над-доменный модуль.
  Оценка X/Y/Z, 8 октантов, 8 слоёв, Corpus Callosum. Результаты только в AxialStore.

- **V2.0** (2026-05-19): AxialEvaluator становится `AdvisorySource`. OctantStabilityTracker +
  ConflictPersistenceTracker → рекомендации в OverDomainArbiter. EvaluationLevel из
  ContextRecognizer. Cap на хранилище.
