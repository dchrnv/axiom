# AXIOM — AxialEvaluator V3.0

**Статус:** Спецификация  
**Версия:** 3.0  
**Дата:** 2026-05-23  
**Предыдущая версия:** `AxialEvaluator_V2_0.md`  
**Категория:** Over-Domain Mechanism  
**Crate:** `axiom-runtime` / `over_domain/axial_evaluator/`

---

## 1. Что меняется относительно V2

V2 добавил AxialEvaluator голос через Advisory-систему: OctantStability + ConflictPersistence
→ рекомендации в Arbiter. Но этот голос был однонаправленным и глухим к ответу.

**Четыре главных изменения V3:**

1. **`on_feedback` авто-калибровка** — OctantStabilityTracker учится на confirmed/rejected;
   порог стабильности корректируется per-Frame вместо глобальной константы (AE-TD-01).

2. **`AxialStore::override_octant()`** — advisory override флаг; AxialEvaluator уважает его
   и не перетирает подтверждённый советом октант на следующем тике (ARB-TD-03).

3. **NarrativeOctantTracker** — скользящее окно по последним N Frame-оценкам одной сессии;
   вычисляет «нарративный октант» как агрегат и генерирует NarrativeShift advisory (AE-TD-03).

4. **EvaluationLevel из конфига** — таблица SubsystemId→EvaluationLevel переезжает в
   `genome.yaml`; хардкод остаётся как fallback (AE-TD-02).

Всё остальное из V2 остаётся: оси X/Y/Z, 8 октантов, OctantStabilityTracker, ConflictPersistenceTracker, AxialStore, Corpus Callosum.

---

## 2. Feedback-авто-калибровка OctantStabilityTracker

### Проблема (AE-TD-01)

`STABILITY_THRESHOLD = 0.7` — глобальная константа. Для одних Frame 70% — мало (нестабильный
Frame флуктуирует и всё равно проходит порог), для других — избыточно (стабильный Frame долго
не получает рекомендацию).

### Решение: per-Frame адаптивный порог

```rust
pub struct FrameStabilityState {
    history:   VecDeque<Octant>,   // ring, cap STABILITY_HISTORY_DEPTH=10
    threshold: f32,                // начальное STABILITY_THRESHOLD=0.70
    confirmed: u32,
    rejected:  u32,
}
```

`on_feedback(advisory_id, outcome)`:

```
if outcome == Confirmed:
    state.confirmed += 1
    // порог снижается — Frame доверяет этому источнику
    state.threshold = (state.threshold - CALIBRATION_STEP).max(THRESHOLD_MIN)

if outcome == Rejected:
    state.rejected += 1
    // порог растёт — нужна большая уверенность
    state.threshold = (state.threshold + CALIBRATION_STEP).min(THRESHOLD_MAX)
```

| Константа | Значение |
|-----------|----------|
| CALIBRATION_STEP | 0.02 |
| THRESHOLD_MIN | 0.50 |
| THRESHOLD_MAX | 0.90 |

`advisory_id` кодирует `sutra_id` в старших битах — AxialEvaluator извлекает его аналогично
NeuralAdvisor: `sutra_id = (id >> 8) as u32`.

Глобальный `STABILITY_THRESHOLD` остаётся как начальное значение для новых Frame.

---

## 3. AxialStore advisory override (ARB-TD-03)

### Проблема

`OctantCorrection` advisory применён через Arbiter → Arbiter вызывает `execute()` →
но записать октант в AxialStore нечем: AxialEvaluator пересчитает на следующем тике и
перетрёт. `AutoApply` для OctantCorrection заблокирован в Arbiter из-за этого (ARB-TD-03).

### Решение: override флаг в AxialEvaluation

```rust
pub struct AxialEvaluation {
    // ... существующие поля ...
    /// Если Some — октант был установлен advisory, не пересчитывать.
    pub advisory_octant_override: Option<Octant>,
}
```

Новый метод в `EvaluatorStorage`:

```rust
pub fn override_octant(&mut self, sutra_id: u32, octant: Octant) {
    // Устанавливает advisory_octant_override в последней записи для sutra_id.
    // Все записи для sutra_id помечаются флагом.
}
```

В `AxialEvaluator::evaluate_frame()`:

```rust
// Если у Frame есть advisory override — пропустить пересчёт октанта
if let Some(override_octant) = self.storage.get_override(anchor.sutra_id) {
    // Использовать override_octant вместо вычисленного analytic_octant
    // (только октант; X/Y/Z оценки всё равно пересчитываются)
}
```

`advisory_octant_override` сбрасывается при следующей реактивации Frame (по событию
`on_frame_reactivated`). Это предотвращает вечную заморозку октанта.

**Дополнение в `AdvisoryAction`:**

```rust
pub enum AdvisoryAction {
    ApplyDepth { octant: Octant, depth: u16 },
    NotifyWorkstation { label: String },
    OverrideOctant { sutra_id: u32, octant: Octant },  // V3 new
}
```

Arbiter вызывает `axial_evaluator.storage().override_octant(sutra_id, octant)` при
AutoApply OctantCorrection с геном-разрешением `Control`.

---

## 4. NarrativeOctantTracker

### Идея (AE-TD-03)

Система сейчас оценивает Frame независимо. Но смысл возникает в *последовательности*: один Frame
про смерть — фактический; десять подряд — нарративный паттерн. NarrativeOctantTracker отслеживает
скользящее окно последних N оценённых Frame в сессии и вычисляет агрегированный октант.

### Структура

```rust
pub struct NarrativeOctantTracker {
    /// Очередь пар (sutra_id, octant) — последние N оценённых Frame.
    window: VecDeque<(u32, Octant)>,
    /// Последний эмитированный нарративный октант (для детекции изменений).
    last_narrative: Option<Octant>,
}

pub const NARRATIVE_WINDOW_SIZE: usize = 8;
pub const NARRATIVE_SHIFT_MIN_DISTANCE: usize = 3;  // минимум октантов до смены
```

Алгоритм после каждой оценки Frame:

```
window.push_back((sutra_id, octant))
if window.len() > NARRATIVE_WINDOW_SIZE:
    window.pop_front()

if window.len() < NARRATIVE_WINDOW_SIZE / 2:
    skip  // недостаточно данных

narrative_octant = most_common_octant(window)
confidence       = count(narrative_octant) / window.len()

if narrative_octant != last_narrative
   && octant_distance(narrative_octant, last_narrative) >= NARRATIVE_SHIFT_MIN_DISTANCE:
    emit NarrativeShift advisory
    last_narrative = narrative_octant
```

`octant_distance` — расстояние между октантами в пространстве {X,Y,Z} ± знаков
(максимум 3, минимум 0 для одного октанта).

### Новый AdvisoryType

```rust
pub enum AdvisoryType {
    DepthHint,
    OctantCorrection,
    ConflictDiagnosis,
    NarrativeShift,    // V3 new — смена нарративного октанта сессии
}
```

`NarrativeShift` advisory:
- `subject_id` = 0 (глобальный, не привязан к конкретному Frame)
- `confidence` = count(narrative_octant) / window.len()
- `action` = `NotifyWorkstation { label: "narrative → {:?} ({:.2})" }`
- `octant_hint` = `Some(narrative_octant.index())`

**TrustConfig добавить:**

| Source | AdvisoryType | Режим | min_confidence |
|--------|--------------|-------|----------------|
| AxialEvaluator (1) | NarrativeShift | RequireConfirmation | 0.55 |

---

## 5. EvaluationLevel из конфига (AE-TD-02)

### Текущее состояние

Таблица `SubsystemId → EvaluationLevel` захардкожена в `levels.rs`. Для базовых подсистем
(Writing, Math, Music, Logic, Time, Values) это разумно, но сложно тестировать и менять.

### V3: override через Genome

```yaml
# genome.yaml
axial_evaluator:
  subsystem_level_overrides:
    Aesthetic: Conceptual    # переопределить для проекта с аналитической музыкой
    Cognitive: Transcendent  # философский контекст
```

```rust
pub struct AxialEvaluatorConfig {
    pub subsystem_level_overrides: HashMap<SubsystemId, EvaluationLevel>,
}
```

`AxialEvaluator::on_boot()` читает конфиг из генома, строит merge таблицы
(overrides поверх встроенного дефолта). Хардкод остаётся как fallback при отсутствии конфига.

---

## 6. Что в коде

Изменения относительно V2:

```
over_domain/axial_evaluator/
├── mod.rs          — + NarrativeOctantTracker поле,
│                     + on_feedback() авто-калибровка,
│                     + AxialEvaluatorConfig (genome.yaml),
│                     + NarrativeShift advisory emission
├── stability.rs    — FrameStabilityState (per-Frame threshold),
│                     on_feedback handler
├── narrative.rs    — NarrativeOctantTracker (новый файл)
├── storage.rs      — + override_octant(), get_override(), clear_override()
├── levels.rs       — + AxialEvaluatorConfig merge в determine_applicable_levels
└── (conflict.rs, synthesis.rs, metrics.rs, persistence.rs — без изменений)
```

В `arbiter/source.rs`:

```rust
pub enum AdvisoryAction {
    // ... existing ...
    OverrideOctant { sutra_id: u32, octant: Octant },
}
```

В `engine.rs` тик %5 — передать feedback AxialEvaluator из Arbiter:

```rust
// После drain_pending_advisories в Arbiter:
for (id, outcome) in arbiter.drain_feedback_for_source(AXIAL_EVALUATOR_SOURCE_ID) {
    axial_evaluator.on_feedback(id, outcome);
}
```

---

## 7. Известные ограничения V3

- **AE-TD-04** — `NarrativeOctantTracker` сбрасывается при рестарте Engine. Персистентный
  нарратив требует сериализации window в `axiom-persist`. V4.

- **AE-TD-05** — `override_octant` сохраняется только in-memory. После рестарта AxialEvaluator
  пересчитает октант заново. Для долгосрочных override нужна запись в EXPERIENCE. V4.

- **AE-TD-06** — `NARRATIVE_WINDOW_SIZE=8` не откалиброван. Правильное значение зависит от
  частоты текстовых инъекций. Нужны данные OBS-03+.

---

## 8. Что в V4+

- **V4:** NarrativeOctantTracker персистентен через `axiom-persist`; нарративный октант
  виден в PhaseCSnapshot → Workstation.
- **V4:** `override_octant` записывается в EXPERIENCE; переживает рестарт.
- **V4:** интеграция с подсистемой Morality (слой 7, Existential) — аксиальная оценка
  этических измерений.
- **V5:** генерация Frame с целевыми координатами в осях (обратный путь от октанта к тексту).

---

## История

- **V1.0** (2026-05-15): первая спецификация. AxialEvaluator как пятый над-доменный модуль.
  Оценка X/Y/Z, 8 октантов, 8 слоёв, Corpus Callosum. Результаты только в AxialStore.

- **V2.0** (2026-05-19): AxialEvaluator становится `AdvisorySource`. OctantStabilityTracker +
  ConflictPersistenceTracker → рекомендации в OverDomainArbiter. EvaluationLevel из
  ContextRecognizer. Cap на хранилище.

- **V3.0** (2026-05-23): Feedback-калибровка OctantStabilityTracker per-Frame. AxialStore
  advisory override для стабилизации OctantCorrection. NarrativeOctantTracker — скользящее
  окно + NarrativeShift advisory. EvaluationLevel override через genome.yaml.
