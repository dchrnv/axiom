# NeuralAdvisor V2 — План реализации

**Статус:** план  
**Дата:** 2026-05-23  
**Scope:** AdvisoryHistory + DepthHistoryBiasAdvisor + AnchorVotingAdvisor + CognitiveProfile

---

## Архитектурные решения

### Где хранить `AdvisoryHistory`

**Решение: внутри `NeuralAdvisor`, в `crates/axiom-runtime`.**

`AdvisoryHistory` — это оперативная память о качестве собственных советов (что принято/отклонено). Это данные о модуле, не о Frame. Размещение в `axiom-experience` создаст циклическую зависимость: `axiom-runtime` → `axiom-experience` → `axiom-runtime`. Cap = **32 записи на sutra_id** (32×11 тиков ≈ 350 тиков истории; при 1000 Frame ≈ 2.5 МБ).

### Где хранить `CognitiveProfile`

**Решение: `crates/axiom-runtime/src/over_domain/arbiter/profile.rs`.**

`CognitiveProfile` — конфигурация поведения Arbiter, а не семантические данные. Arbiter уже содержит `TrustConfig` — тот же паттерн. `TrustConfig` отвечает «применять или нет», `CognitiveProfile` — «с каким весом масштабировать confidence по октанту». Это разные ортогональные уровни.

### CognitiveProfile в Arbiter vs в NeuralAdvisor

**Arbiter — правильное место.** NA генерирует advisory с «сырым» confidence, честно отражая паттерны. Arbiter решает, применять ли его. CognitiveProfile влияет на критерий принятия, а не на качество генерации. Если CognitiveProfile в NA — NA начинает учитывать «вкусы» системы, что нарушит advisory-only гарантию.

---

## Граф зависимостей фаз

```
Фаза 1 (AdvisoryHistory)
    ↓
Фаза 2 (DepthHistoryBiasAdvisor)  ←─ параллельно с Фазой 3
Фаза 3 (AnchorVotingAdvisor)      ←─ независимо от Фазы 2
    ↓ оба
Фаза 4 (CognitiveProfile)
    ↓
Фаза 5 (wiring в engine.rs)
```

---

## Фаза 1 — `AdvisoryHistory`

**Новый файл:** `crates/axiom-runtime/src/over_domain/neural_advisor/history.rs`

```rust
pub struct AdvisoryHistoryEntry {
    pub computed_at_event: u64,
    pub octant_suggestion: Option<Octant>,
    pub octant_confidence: f32,
    pub subsystem_suggestion: Option<SubsystemId>,
    pub subsystem_confidence: f32,
    pub outcome: AdvisoryHistoryOutcome,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AdvisoryHistoryOutcome {
    Pending,
    Applied,
    Confirmed,
    Rejected,
    Skipped,
}

pub struct AdvisoryRingBuffer {
    entries: VecDeque<AdvisoryHistoryEntry>,
    cap: usize,
}

impl AdvisoryRingBuffer {
    pub fn new(cap: usize) -> Self
    pub fn push(&mut self, entry: AdvisoryHistoryEntry)
    pub fn iter(&self) -> impl Iterator<Item = &AdvisoryHistoryEntry>
    pub fn acceptance_rate_octant(&self) -> f32
    pub fn dominant_accepted_octant(&self) -> Option<Octant>
}

pub struct AdvisoryHistory {
    per_sutra: HashMap<u32, AdvisoryRingBuffer>,
    cap_per_sutra: usize,
}

impl AdvisoryHistory {
    pub const DEFAULT_CAP: usize = 32;
    pub fn new() -> Self
    pub fn with_cap(cap: usize) -> Self
    pub fn record(&mut self, sutra_id: u32, entry: AdvisoryHistoryEntry)
    pub fn update_outcome(&mut self, sutra_id: u32, event_id: u64, outcome: AdvisoryHistoryOutcome)
    // update_outcome обновляет последнюю запись со статусом Pending для данного sutra_id
    pub fn get(&self, sutra_id: u32) -> Option<&AdvisoryRingBuffer>
}
```

**Изменения в существующих файлах:**

`neural_advisor/mod.rs`:
- Добавить поле `history: AdvisoryHistory`
- В `on_feedback` → `history.update_outcome(sutra_id, event, outcome)`; sutra_id декодируется из advisory_id: `sutra_id = (id >> 8) as u32`
- После `self.result_store.insert(result)` → `self.history.record(sutra_id, entry_from_result(&result))`

**Тесты в `history.rs`:**
- `test_ring_buffer_cap` — при push сверх cap старые вытесняются
- `test_update_outcome_finds_last_pending`
- `test_acceptance_rate_empty_returns_zero`
- `test_dominant_accepted_octant_votes_correctly`

---

## Фаза 2 — `DepthHistoryBiasAdvisor`

**Новый файл:** `crates/axiom-runtime/src/over_domain/neural_advisor/implementations/octant.rs`

**Расширение `traits.rs`** — добавить в `OctantAdvisorInput`:
```rust
pub struct OctantAdvisorInput {
    // ... существующие поля ...
    pub depth_per_octant: [u16; 8],   // НОВОЕ
    pub reactivation_count: u32,       // НОВОЕ
}
```

Данные уже вычисляются в `process_frame` для других советников — просто пробросить.

**Логика `DepthHistoryBiasAdvisor`:**

```rust
pub struct DepthHistoryBiasAdvisor {
    min_depth_threshold: u16,   // 800 — минимальная глубина для предложения октанта
    min_depth_advantage: u16,   // 300 — победитель должен быть на 300+ глубже текущего
}
impl Default ...
```

Алгоритм `suggest_octant`:
1. `best_oct = argmax(depth_per_octant)`. Если `depth_per_octant[best_oct] < min_depth_threshold` → `None`.
2. Если `best_oct == analytic_octant.index()` и нет конфликта (`analytic == synthetic`) → `None`.
3. Если `depth_per_octant[best_oct] - depth_per_octant[analytic_octant.index()] < min_depth_advantage` → `None`.
4. `confidence = (depth_per_octant[best_oct] as f32 / 3000.0).min(0.85)`
5. Если `reactivation_count < 10` → `confidence *= reactivation_count as f32 / 10.0`
6. Вернуть `OctantSuggestion { octant: Octant::from_index(best_oct), reason: DepthHistoryBias, confidence }`

**Тесты:**
- `test_no_suggestion_when_all_depths_zero`
- `test_no_suggestion_when_analytic_already_dominant`
- `test_suggests_historical_octant_when_dominant`
- `test_confidence_scales_with_depth`
- `test_no_suggestion_when_advantage_insufficient`
- `test_low_reactivation_reduces_confidence`

---

## Фаза 3 — `AnchorVotingAdvisor`

**Новый файл:** `crates/axiom-runtime/src/over_domain/neural_advisor/implementations/subsystem.rs`

**Чем отличается от `AnchorSet::dominant_subsystem_of()`:**
- `dominant_subsystem_of()` — работает с `&[AnchorMatch]` (результаты текстового матчинга), top-1 по score тегов. Вызывается в TextPerceptor.
- `AnchorVotingAdvisor` — работает с `energy_weights: Vec<(SubsystemId, u8)>` из `InterpretationProfile` (накопленные за всё время жизни Frame), усиливает сигнал на глубину в аффинном октанте, поддерживает dual-subsystem вывод.

```rust
pub struct AnchorVotingAdvisor {
    min_energy_weight: u8,       // 20 — порог участия в голосовании
    dominance_threshold: f32,    // 0.50 — если winner < 50% суммы → снизить confidence
    dual_threshold: f32,         // 0.15 — если разрыв 1-й/2-й < 15% → вернуть secondary
}
impl Default ...
```

Алгоритм `suggest_subsystem`:
1. Фильтровать `energy_weights`: только `weight >= min_energy_weight`.
2. Для каждой подсистемы: `score(s) = weight as f32 * depth_bonus(s)`, где  
   `depth_bonus(s) = (1.0 + depth_per_octant[SUBSYSTEM_AFFINITY[s]] as f32 / 2000.0).min(2.0)`
3. `primary = argmax(scores)`. Если пуст → `None`.
4. `total = sum(scores)`. Если `scores[primary] < dominance_threshold * total` → `confidence *= 0.7`.
5. Если `|scores[primary] - scores[second]| / total < dual_threshold` → `secondary = Some(second)`.
6. Если `reactivation_count < 5` → `confidence *= 0.6`.
7. Вернуть `SubsystemSuggestion { primary, secondary, confidence }`.

**Тесты:**
- `test_empty_weights_returns_none`
- `test_single_subsystem_wins`
- `test_low_weight_filtered_out`
- `test_dual_subsystem_detected_when_close`
- `test_depth_bonus_boosts_affine_subsystem`
- `test_confidence_reduced_on_low_reactivation`

---

## Фаза 4 — `CognitiveProfile`

**Новый файл:** `crates/axiom-runtime/src/over_domain/arbiter/profile.rs`

```rust
#[derive(Debug, Clone)]
pub struct CognitiveProfile {
    pub octant_weights: [f32; 8],   // мультипликаторы confidence, init 1.0
}

impl Default for CognitiveProfile {
    fn default() -> Self { Self { octant_weights: [1.0; 8] } }
}

impl CognitiveProfile {
    pub const WEIGHT_MIN: f32 = 0.5;
    pub const WEIGHT_MAX: f32 = 2.0;
    pub const LEARNING_RATE: f32 = 0.05;

    pub fn scale_confidence(&self, octant_idx: usize, raw: f32) -> f32 {
        (raw * self.octant_weights[octant_idx]).min(1.0)
    }

    pub fn update(&mut self, octant_idx: usize, accepted: bool) {
        let delta = if accepted { Self::LEARNING_RATE } else { -Self::LEARNING_RATE };
        self.octant_weights[octant_idx] =
            (self.octant_weights[octant_idx] + delta).clamp(Self::WEIGHT_MIN, Self::WEIGHT_MAX);
    }
}
```

**Расширение `Advisory`** (`arbiter/source.rs`):
```rust
pub struct Advisory {
    // ... существующие поля ...
    pub octant_hint: Option<usize>,   // НОВОЕ — для CognitiveProfile scaling
}
```

При генерации OctantCorrection advisory в `NeuralAdvisor::poll_advisories()`:
```rust
advisory.octant_hint = Some(suggestion.octant.index());
```

**Изменения в `arbiter/mod.rs`:**
1. Добавить поле `cognitive_profile: CognitiveProfile`
2. В `tick_with_stores`, перед проверкой confidence:
```rust
let effective_confidence = match advisory.octant_hint {
    Some(idx) if advisory.advisory_type == AdvisoryType::OctantCorrection =>
        self.cognitive_profile.scale_confidence(idx, advisory.confidence),
    _ => advisory.confidence,
};
```
3. В `confirm_pending` / `on_feedback(Applied/Confirmed)`:
```rust
if let Some(idx) = pending.advisory.octant_hint {
    self.cognitive_profile.update(idx, true);
}
```
4. В `reject_pending`:
```rust
if let Some(idx) = pending.advisory.octant_hint {
    self.cognitive_profile.update(idx, false);
}
```

**Тесты в `profile.rs`:**
- `test_default_profile_unity_weights`
- `test_scale_confidence_multiplied`
- `test_scale_clamped_to_one`
- `test_update_accepted_increases_weight`
- `test_update_rejected_decreases_weight`
- `test_weight_clamped_to_bounds`

**Новый тест в `arbiter/mod.rs`:**
- `test_octant_correction_scaled_by_profile` — профиль с weight=2.0 для октанта → совет с confidence=0.50 проходит порог 0.60 (0.50×2.0=1.0≥0.60)

---

## Фаза 5 — Wiring

**`crates/axiom-runtime/src/over_domain/neural_advisor/registry.rs`:**
```rust
pub fn default_v2() -> Self {
    Self {
        depth: Some(Arc::new(ReactivationDepthAdvisor)),
        octant: Some(Arc::new(DepthHistoryBiasAdvisor::default())),
        conflict: Some(Arc::new(RuleBasedCorpusCallosumResolver)),
        subsystem: Some(Arc::new(AnchorVotingAdvisor::default())),
        emergent: Some(Arc::new(DepthThresholdEmergentDetector)),
    }
}
```

**`crates/axiom-runtime/src/engine.rs`:**
```rust
neural_advisor: NeuralAdvisor::with_default_v2(),
```

Добавить аксессоры для тестов:
```rust
pub fn neural_advisor_history(&self) -> &AdvisoryHistory { ... }
pub fn cognitive_profile(&self) -> &CognitiveProfile { ... }
```

**Интеграционные тесты:**
- `test_octant_advisor_slot_filled_in_v2` — `registry.octant.is_some()`
- `test_subsystem_advisor_slot_filled_in_v2` — `registry.subsystem.is_some()`
- `test_advisory_history_records_result` — после on_tick, `neural_advisor_history().get(sutra_id).is_some()`

---

## Полный список файлов

### Новые файлы
| Файл | Содержимое |
|------|------------|
| `neural_advisor/history.rs` | `AdvisoryHistory`, `AdvisoryRingBuffer`, `AdvisoryHistoryOutcome` |
| `neural_advisor/implementations/octant.rs` | `DepthHistoryBiasAdvisor` |
| `neural_advisor/implementations/subsystem.rs` | `AnchorVotingAdvisor` |
| `arbiter/profile.rs` | `CognitiveProfile` |

### Изменяемые файлы
| Файл | Изменение |
|------|-----------|
| `neural_advisor/traits.rs` | `+depth_per_octant`, `+reactivation_count` в `OctantAdvisorInput` |
| `neural_advisor/mod.rs` | `+history: AdvisoryHistory`, `with_default_v2()`, `on_feedback` impl, запись в историю |
| `neural_advisor/registry.rs` | `+default_v2()` |
| `neural_advisor/implementations/mod.rs` | `pub mod octant; pub mod subsystem;` |
| `arbiter/source.rs` | `+octant_hint: Option<usize>` в `Advisory` |
| `arbiter/mod.rs` | `+cognitive_profile`, scaling в `tick_with_stores`, update в confirm/reject |
| `over_domain/mod.rs` | re-export `CognitiveProfile`, `AdvisoryHistory`, новых советников |
| `engine.rs` | переключить на `with_default_v2()` |

---

## Потенциальные сложности

1. **`on_feedback` sutra mapping** — advisory_id через битовый сдвиг: `sutra_id = (id >> 8) as u32`. Формат должен быть стабилен — убедиться что `advisory_id()` в mod.rs не менялся.

2. **Тест `test_default_registry_has_conflict_and_emergent`** — проверяет `registry.octant.is_none()`. В V2 становится `is_some()`. Обновить тест.

3. **`SUBSYSTEM_AFFINITY` таблица** — для `AnchorVotingAdvisor` нужна таблица `SubsystemId → аффинный октант`. Уже используется в `depth.rs` (`SubsystemAffinityDepthAdvisor`). Вынести в `implementations/mod.rs` как `pub const SUBSYSTEM_AFFINITY_OCTANT: [usize; 6]` или переиспользовать из `depth.rs`.

4. **Thread safety** — `DepthHistoryBiasAdvisor` и `AnchorVotingAdvisor` используют только `&self` без внутренней мутабельности → `Send + Sync` выполняется автоматически.

---

## Ожидаемые результаты после V2

- Все 5 слотов NeuralAdvisoryRegistry заполнены
- `AdvisoryHistory`: observable trend по каждому Frame (acceptance_rate, dominant_accepted_octant)
- `DepthHistoryBiasAdvisor`: OctantCorrection advisory когда история глубин противоречит аналитике
- `AnchorVotingAdvisor`: SubsystemAttribution advisory с возможным dual-subsystem
- `CognitiveProfile`: Arbiter самообучается по октантам через feedback (online learning, rate=0.05)
- Тестов: ~20 новых unit + ~3 интеграционных
