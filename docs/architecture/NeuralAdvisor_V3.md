# NeuralAdvisor V3.0

**Статус:** Реализовано (V3)  
**Дата:** 2026-05-27  
**Crate:** `axiom-runtime` / `over_domain/neural_advisor/`  
**Опирается на:** `NeuralAdvisor_V2.md`, `ContextRecognizer_V6_0.md`, `AxialEvaluator_V3_0.md`

---

## 1. Назначение

NeuralAdvisor — шестой над-доменный модуль. Даёт *второй голос* внутри семантического слоя.

Детерминированные компоненты (AxialEvaluator, ContextRecognizer) вычисляют из метрик: энтропии, плотности графа, близости к примитивам. NeuralAdvisor накапливает историю, обнаруживает паттерны и формулирует advisory-рекомендации которые Arbiter может принять или отклонить.

`ModuleId = 19`, `tick_interval = 11`.

**Чем НЕ является:**
- Не заменяет детерминированные компоненты
- Не перезаписывает их результаты напрямую (Advisory-Only через Arbiter)
- Не требует ML для базовой работы — V3 реализован на правилах, статистике и истории

---

## 2. Позиция в архитектуре

```
TextPerceptor → MAYA tokens → ContextRecognizer → InterpretationProfile
                                                         ↓
                EXPERIENCE anchors → AxialEvaluator → AxialEvaluation
                                                         ↓
                                              NeuralAdvisor (читает оба)
                                                         ↓
                                              AdvisoryResult (per Frame)
                                                         ↓
                                    UCL: NotifyEmergentCandidate → chrnv
```

NeuralAdvisor читает из:
- `AxialStore` (снапшот от AxialEvaluator)
- `InterpretationProfileStore` (снапшот от ContextRecognizer)
- `SutraDepthStore` (снапшот от ContextRecognizer)
- EXPERIENCE domain через AshtiCore (активные Frame-анкеры)

Пишет в:
- `AdvisoryResultStore` (собственное in-memory хранилище)
- `EmergentPrimitiveStore` (кандидаты на новые примитивы)
- `DivergenceLog` (расхождения octant advisory vs analytic, V3 G1)
- UCL (уведомления)

Также реализует `AdvisorySource` → `poll_advisories() -> Vec<Advisory>` с `octant_hint`.

---

## 3. Архитектурный выбор: Advisory-Only

NeuralAdvisor **никогда не перезаписывает** детерминированные результаты напрямую. Его рекомендации живут рядом как отдельное поле.

**Почему:**
1. Качество советника на правилах неизвестно — давать Override значит деградировать систему молча
2. Детерминированная система проверена и воспроизводима
3. На этапе калибровки нужно видеть расхождение — Override его скрывает

**Промоция через геном (V9+):** для каждого типа советника геном может разрешить `Override`. Переход от Advisory к Override происходит явно после наблюдения.

*V2+ частично решает это через `CognitiveProfile` в Arbiter: confirm/reject обновляют `octant_weights[8]` онлайн (learning_rate=0.05), создавая мягкий feedback-контур без Override.*

---

## 4. Пять трейтов

### 4.1 DepthPredictionAdvisor

Предсказывает куда Frame должен осесть в SutraDepth.

```
Input:  sutra_id, subsystem, current_depth[8], reactivation_count,
        frame_age_ticks, primary_octant, event_id
Output: Option<DepthHint { target_octant, suggested_depth, confidence }>
```

**V1–V3:** `ReactivationDepthAdvisor` — `depth = base_depth + reactivation_count × STEP`.  
**V9:** нейронная модель с embedding Frame как входом.

### 4.2 OctantCorrectionAdvisor

Предлагает семантически правильный октант когда метрики AE дают неточный результат.

```
Input:  sutra_id, analytic_octant, synthetic_octant, x/y/z AxialScore,
        evaluation_level, primary_subsystem, primary_octant_from_profile, event_id,
        depth_per_octant: [u16; 8], reactivation_count: u32
Output: Option<OctantSuggestion { octant, confidence, reason }>
Reason: SemanticContent | DepthHistoryBias | SubsystemAffinity | BoundaryResolution
```

**V1:** `None`.  
**V2–V3:** `DepthHistoryBiasAdvisor` (см. §8.2).  
Advisory несёт `octant_hint: Some(octant.index())` → Arbiter → CognitiveProfile.  
**V9:** embedding-модель понимает семантический контент.

### 4.3 CorpusCallosumResolver

Диагностирует конфликт analytic vs synthetic октанта.

```
Input:  sutra_id, analytic_octant, synthetic_octant, conflict_strength (0..255),
        frame_age_ticks, reactivation_count, primary_subsystem, event_id
Output: ConflictResolutionHint { diagnosis: ConflictDiagnosis, confidence }

ConflictDiagnosis:
  Unresolved | BoundaryFrame | TransitionState | DualNature | DominantOctant(Octant)
```

**V1–V2:** `RuleBasedCorpusCallosumResolver` (см. §8.1).  
**V3:** `PatternLearningResolver` — если накоплено ≥ 5 решённых записей истории по Frame → выводит `DominantOctant`; иначе fallback на правила (см. §9.2).  
**V9:** semantic model — понимает семантический контент конфликта.

### 4.4 SubsystemAttributionAdvisor

Дополняет энергетическую детекцию ContextRecognizer.

```
Input:  sutra_id, energy_weights: HashMap<SubsystemId, u8>, primary_octant,
        depth_per_octant[8], reactivation_count, event_id
Output: Option<SubsystemSuggestion { primary, secondary: Option<SubsystemId>, confidence }>
```

**V1:** `None`.  
**V2–V3:** `AnchorVotingAdvisor` (см. §8.3).  
**V9:** embedding-классификатор независимый от AnchorSet proximity.

### 4.5 EmergentPatternAdvisor

Обнаруживает кандидатов в новые примитивы.

```
Input:  sutra_id, octant, depth_per_octant[8], reactivation_count,
        frame_age_ticks, known_primitive_ids: &[u32], event_id
Output: EmergentDetectionResult { is_candidate: bool, confidence: f32 }
```

Если `is_candidate = true` → добавляет в `EmergentPrimitiveStore` + `NotifyEmergentCandidate` UCL.

**V1–V3:** `DepthThresholdEmergentDetector` (см. §8.4).  
**V9:** генеративная модель — предлагает имя и начальные координаты.

---

## 5. NeuralAdvisorRegistry

```rust
pub struct NeuralAdvisorRegistry {
    pub depth:     Option<Arc<dyn DepthPredictionAdvisor>>,
    pub octant:    Option<Arc<dyn OctantCorrectionAdvisor>>,
    pub conflict:  Option<Arc<dyn CorpusCallosumResolver>>,
    pub subsystem: Option<Arc<dyn SubsystemAttributionAdvisor>>,
    pub emergent:  Option<Arc<dyn EmergentPatternAdvisor>>,
}
```

`None` = слот незаполнен (не тратит ресурсы, не виден в логах).

**default_v3() — V3 конфигурация (все 5 слотов):**
- depth:    `ReactivationDepthAdvisor`
- octant:   `DepthHistoryBiasAdvisor`
- conflict: `PatternLearningResolver` (с fallback на RuleBased)
- subsystem:`AnchorVotingAdvisor`
- emergent: `DepthThresholdEmergentDetector`

Engine использует `NeuralAdvisor::with_default_v2()` (V2 конфиг); переход на `default_v3()` при явном обновлении.

---

## 6. AdvisoryResult + AdvisoryHistory

### AdvisoryResult

```rust
pub struct AdvisoryResult {
    pub sutra_id:               u32,
    pub computed_at_event:      u64,
    pub octant_suggestion:      Option<OctantSuggestion>,
    pub conflict_diagnosis:     Option<ConflictResolutionHint>,
    pub subsystem_suggestion:   Option<SubsystemSuggestion>,
    pub depth_hint:             Option<DepthHint>,
}
```

`AdvisoryResultStore: HashMap<u32, AdvisoryResult>` — in-memory, перезаписывается каждый тик.

### AdvisoryHistory

```rust
pub struct AdvisoryHistory {
    per_sutra: HashMap<u32, AdvisoryRingBuffer>,
    cap_per_sutra: usize,  // = 32
}

pub struct AdvisoryHistoryEntry {
    pub computed_at_event:    u64,
    pub octant_suggestion:    Option<Octant>,
    pub octant_confidence:    f32,
    pub subsystem_suggestion: Option<SubsystemId>,
    pub subsystem_confidence: f32,
    pub outcome:              AdvisoryHistoryOutcome,
}

pub enum AdvisoryHistoryOutcome { Pending, Applied, Confirmed, Rejected, Skipped }
```

Методы: `record()`, `update_outcome(sutra_id, event_id, outcome)`, `acceptance_rate_octant()`, `dominant_accepted_octant()`.

---

## 7. NeuralAdvisor как OverDomainComponent

**on_boot:** проверяет `ExperienceMemory/Read` + `AshtiField/Read` в геноме.

**on_tick:**
1. Получить список активных Frame-анкеров из EXPERIENCE
2. Для каждого Frame: собрать входные данные из снапшотов
3. Вызвать каждый советник из registry
4. Сохранить в AdvisoryResultStore
5. Для октантного расхождения (Hamming ≥ 2) → записать в DivergenceLog (G1)
6. Для emergent кандидатов → EmergentPrimitiveStore + UCL emit

---

## 8. V1/V2 реализации (история слотов)

### 8.1 RuleBasedCorpusCallosumResolver

| conflict_strength | frame_age_ticks | reactivation_count | Диагноз | confidence |
|-------------------|-----------------|--------------------|---------|------------|
| 85 (1 ось) | любой | любой | BoundaryFrame | 0.80 |
| 170 (2 оси) | < 20 | любой | TransitionState | 0.65 |
| 170 (2 оси) | ≥ 20 | ≥ 10 | DualNature | 0.70 |
| прочее | — | — | Unresolved | 0.50 |

### 8.2 DepthHistoryBiasAdvisor (V2, octant slot)

| Константа | Значение |
|-----------|----------|
| `DHB_MIN_DEPTH_THRESHOLD` | 800 |
| `DHB_MIN_DEPTH_ADVANTAGE` | 300 |
| `DHB_CONFIDENCE_DEPTH_NORM` | 3000.0 |
| `DHB_MIN_FULL_TRUST_REACTIVATIONS` | 10 |

Алгоритм: `best_idx = argmax(depth_per_octant)`. Возвращает `None` если best_depth < threshold, или если best_idx == analytic_idx и нет конфликта, или advantage недостаточно. Confidence = `(best_depth / 3000).min(0.85) × reactivation_penalty`.

### 8.3 AnchorVotingAdvisor (V2, subsystem slot)

| Константа | Значение |
|-----------|----------|
| `AV_MIN_ENERGY_WEIGHT` | 20 |
| `AV_DOMINANCE_THRESHOLD` | 0.50 |
| `AV_DUAL_THRESHOLD` | 0.15 |
| `AV_MIN_FULL_TRUST_REACTIVATIONS` | 5 |

`score(s) = weight × depth_bonus(s)` где `depth_bonus = (1.0 + depth[SUBSYSTEM_AFFINITY[s]] / 2000.0).min(2.0)`.
Dual-subsystem вывод если gap между primary и secondary < AV_DUAL_THRESHOLD.

### 8.4 DepthThresholdEmergentDetector (V1, emergent slot)

Кандидат если Frame:
- Не в `known_primitive_ids`
- `depth[octant] ≥ EMERGENT_CANDIDATE_MIN_DEPTH` (**1000**, снижено с 8000 по OBS-02)
- `reactivation_count ≥ EMERGENT_CANDIDATE_MIN_REACTIVATIONS` (**5**, снижено с 30)
- Старше `EMERGENT_CANDIDATE_MIN_AGE_TICKS` = 100

Confidence = 0.60. Сигнал для chrnv, не решение.

---

## 9. V3 добавления (G1 / G2 / G3)

### 9.1 G1: DivergenceLog (divergence.rs)

```rust
pub struct DivergenceLog {
    entries: VecDeque<DivergenceEntry>,
    total_recorded: u64,
}
// cap = 256

pub struct DivergenceEntry {
    pub event_id:           u64,
    pub sutra_id:           u32,
    pub analytic_octant:    Octant,
    pub advisory_octant:    Octant,
    pub distance:           usize,  // Hamming 0..3
    pub advisor_confidence: f32,
}

pub fn octant_hamming_distance(a: Octant, b: Octant) -> usize
```

Записывается когда `advisory_octant` отличается от `analytic_octant` на Hamming distance ≥ 2.  
Кольцевой буфер 256 записей. `total_recorded()` не обнуляется при вытеснении.

Назначение: калибровочный инструмент — где советник постоянно расходится с детерминированной системой, там либо советник ошибается, либо метрики AE неадекватны для этих Frame.

### 9.2 G2: PatternLearningResolver (implementations/conflict.rs)

```rust
pub struct PatternLearningResolver {
    fallback: RuleBasedCorpusCallosumResolver,
}
pub const MIN_SAMPLES: usize = 5;
```

Логика на каждом вызове `resolve(input)`:
1. Если в `input.history` накоплено < MIN_SAMPLES решённых записей → fallback на RuleBased
2. Иначе: найти октант с наибольшим числом Confirmed-записей в истории
3. Если acceptance_rate для этого октанта > MIN_ACCEPTANCE_RATE (0.5) → возвращает `DominantOctant(octant)` с confidence пропорциональной acceptance_rate
4. Иначе → fallback

Учится per-Frame: история AdvisoryHistory per sutra_id подаётся как `input.history`. Не требует глобального состояния.

### 9.3 G3: NeuralAdvisorConfig (config.rs)

```rust
pub struct NeuralAdvisorConfig {
    pub depth_enabled:     bool,
    pub octant_enabled:    bool,
    pub conflict_enabled:  bool,
    pub subsystem_enabled: bool,
    pub emergent_enabled:  bool,
}
impl Default → все true
```

Загружается из секции `neural_advisor` в `genome.yaml`:

```yaml
neural_advisor:
  depth:
    enabled: true
  octant:
    enabled: true
  conflict:
    enabled: false  # отключить PatternLearningResolver в prod
  subsystem:
    enabled: true
  emergent:
    enabled: true
```

`NeuralAdvisorConfig::apply_to_registry(&mut registry)` — `enabled: false` → `registry.slot = None`.

При отсутствии секции или ошибке парсинга → `Default` (все включены).

---

## 10. UCL команды

```
NotifyEmergentCandidate = 5200
  Payload: { sutra_id: u32, octant: u8, depth: u16, confidence_scaled: u8 }
  Направление: NeuralAdvisor → Workstation/chrnv

ApproveEmergentCandidate = 5201
  Payload: { sutra_id: u32 }
  Направление: chrnv → Engine
  Эффект: emergent_store.approve(sutra_id) + SutraDepthStore::register_primitive(sutra_id)
```

---

## 11. Genome-интеграция

**on_boot:** проверяет `ExperienceMemory/Read` + `AshtiField/Read`.

**G3 (V3):** `NeuralAdvisorConfig::from_genome_yaml()` вызывается при инициализации. `apply_to_registry()` применяется к `default_v3()` registry. Конфигурация статическая (не hot-reload).

**Advisory.octant_hint:** `Option<usize>` — `Some(idx)` для OctantCorrection; `None` для остальных. Используется Arbiter для направления feedback в CognitiveProfile.

**CognitiveProfile (V2, в OverDomainArbiter):**

```
CognitiveProfile { octant_weights: [f32; 8], init 1.0 }
  WEIGHT_MIN = 0.5, WEIGHT_MAX = 2.0, LEARNING_RATE = 0.05

scale_confidence(octant_idx, raw) → raw × weight, cap снизу 1.0
update(octant_idx, accepted):
  accepted  → weight += 0.05
  rejected  → weight -= 0.05
  clamp [WEIGHT_MIN, WEIGHT_MAX]
```

---

## 12. Связь с другими компонентами

| Компонент | Отношение |
|-----------|-----------|
| AxialEvaluator | Читает AxialStore снапшот |
| ContextRecognizer | Читает InterpretationProfileStore + SutraDepthStore |
| OverDomainArbiter | Источник Advisory (AdvisorySource трейт), получает feedback |
| DreamPhase | Не связан напрямую в V3 |
| FrameWeaver | Не связан напрямую в V3 |

---

## 13. Что в V9

- Все пять трейтов заменяются обученными моделями (~1M параметров суммарно, inference < 100µs)
- `NeuralAdvisorRegistry` заполняется из genome.yaml (model endpoints)
- Обучение только offline / в DREAM Phase (micro-training, один epoch на batch)
- `ConfidenceCalibrator` — калибровка raw confidence через исторические пары (raw, was_correct)
- Промоция: Advisory → Blend → Override через явный genome update
- Distillation: rule-based = Teacher, neural = Student до 95% accuracy воспроизведения
- Cross-advisor coordination: предварительные hints перед финальными результатами

---

## 14. История

- **V1.0** (2026-05-17): Advisory-only архитектура. Пять трейтов. Слоты depth/conflict/emergent заполнены (ReactivationDepthAdvisor, RuleBasedCorpusCallosumResolver, DepthThresholdEmergentDetector). Слоты octant/subsystem — None.
- **V1.0 patch** (2026-05): Калибровка порогов DepthThresholdEmergentDetector по OBS-02 (MIN_DEPTH 8000→1000, MIN_REACTIVATIONS 30→5). Исправление reactivation_count в SutraDepthStore.
- **V2.0** (2026-05-23): Все 5 слотов заполнены. DepthHistoryBiasAdvisor (octant), AnchorVotingAdvisor (subsystem). AdvisoryHistory ring-32. OctantAdvisorInput расширен (depth_per_octant[8] + reactivation_count). Advisory.octant_hint. CognitiveProfile в OverDomainArbiter с online learning. Engine переведён на `with_default_v2()`.
- **V3.0** (2026-05-26): G1: DivergenceLog (ring 256, octant_hamming_distance). G2: PatternLearningResolver в conflict slot (online learning на AdvisoryHistory per-Frame, MIN_SAMPLES=5). G3: NeuralAdvisorConfig из genome.yaml секции `neural_advisor` → per-advisor enable/disable, apply_to_registry(). AdvisorySource трейт → poll_advisories() с octant_hint.
