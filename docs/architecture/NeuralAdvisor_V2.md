# NeuralAdvisor V2.0

**Статус:** реализован (V2)  
**Дата:** 2026-05-23  
**Автор:** Chernov Denys

---

## 1. Назначение

NeuralAdvisor — шестой над-доменный модуль. Он не управляет доменами и не кристаллизует
паттерны. Его задача — дать *второй голос* внутри семантического слоя.

Детерминированные компоненты (AxialEvaluator, ContextRecognizer) вычисляют из метрик:
энтропии, плотности графа, близости к примитивам. Они правильны в рамках своей модели,
но не понимают смысл. NeuralAdvisor — слой который может понять, когда вырастет до V9.
В V1 он понимает столько, сколько позволяют правила и статистика накопленной истории.

**Чем НЕ является:**
- Не заменяет детерминированные компоненты
- Не перезаписывает их результаты напрямую (см. §3 — Advisory-Only через Arbiter)
- Не требует ML для базовой работы — V2 реализован на правилах, статистике и истории

---

## 2. Позиция в архитектуре

```
TextPerceptor → MAYA tokens → ContextRecognizer → InterpretationProfile
                                                         ↓
                EXPERIENCE anchors → AxialEvaluator → AxialEvaluation
                                                         ↓
                                              NeuralAdvisor (reads both)
                                                         ↓
                                              AdvisoryResult (per Frame)
                                                         ↓
                                    UCL: NotifyEmergentCandidate → chrnv
```

NeuralAdvisor читает из:
- `AxialStore` (снапшот от AxialEvaluator)
- `InterpretationProfileStore` (снапшот от ContextRecognizer)
- `SutraDepthStore` (снапшот от ContextRecognizer)
- EXPERIENCE domain через AshtiCore (список активных Frame-анкеров)

Пишет в:
- `AdvisoryResultStore` (собственное хранилище, in-memory)
- `EmergentPrimitiveStore` (собственное, кандидаты на новые примитивы)
- UCL (уведомления)

---

## 3. Архитектурный выбор: Advisory-Only

### Три варианта

**Вариант A — Override:** советник с высоким confidence замещает детерминированный результат.  
**Вариант B — Blend:** детерминированный результат и советник взвешиваются по confidence.  
**Вариант C — Advisory-Only:** советник никогда не перезаписывает. Его рекомендации
живут рядом с детерминированным результатом как отдельное поле.

### Почему выбран C

1. **Качество советника неизвестно.** В V1 советник основан на правилах, не на
   обученной модели. Даже в V9 первые итерации модели будут ошибаться. Давать
   ненадёжному советнику Override — значит деградировать систему молча.

2. **Детерминированная система проверена.** AxialEvaluator и ContextRecognizer дают
   воспроизводимые результаты. Advisory-only сохраняет их как source-of-truth.

3. **Наблюдаемость важнее влияния.** На этапе калибровки нужно видеть расхождение
   между советником и детерминированной системой — где они соглашаются, где нет.
   Override скрывает это расхождение.

### Известные минусы варианта C

- Советник не влияет на поведение системы без внешнего вмешательства координатора.
  Хорошая рекомендация по октанту не меняет реальный октант Frame.
- Трудно измерить качество советника: нет обратной связи был ли совет использован.
- При накоплении истории советник может знать лучше детерминированной системы — но
  продолжает молчать, если нет механизма повышения доверия.

### На что обратить внимание в V3+

1. **Workstation должна показывать расхождение.** Рядом с аналитическим октантом —
   `advisory_octant` если есть и confidence > 0.5. Это главный инструмент калибровки.

2. **Промоция советника через геном.** Для каждого типа советника геном может
   разрешить `Override` (аналог `Permission::Control`). Переход от Advisory к Override
   происходит явно, по конкретному типу, после наблюдения.

3. **Логировать случаи значительного расхождения.** Если `advisory_octant` отличается
   от `analytic_octant` на 2+ оси — это сигнал для анализа. Либо советник ошибается,
   либо метрики AxialEvaluator неадекватны для этого фрейма.

*V2 частично решает п.2 через CognitiveProfile в Arbiter: confirm/reject обновляют
octant_weights[8] онлайн (learning_rate=0.05), что создаёт мягкий feedback-контур
без Override на уровне хранилища.*

---

## 4. Пять трейтов

### 4.1 DepthPredictionAdvisor

Предсказывает где Frame должен осесть в SutraDepth для заданной подсистемы.
Вызывается когда ContextRecognizer встречает новый Frame (reactivation_count == 0).

```
Input:  sutra_id, subsystem, current_depth[8], reactivation_count, frame_age_ticks,
        primary_octant, event_id
Output: Option<DepthHint { target_octant, suggested_depth, confidence }>
```

**V1:** `ReactivationDepthAdvisor` — depth = base_depth + reactivation_count × STEP.  
**V2:** то же (без изменений в этом слоте).  
**V9:** нейронная модель с embedding нового Frame как входом; предсказывает settling depth.

### 4.2 OctantCorrectionAdvisor

Предлагает семантически правильный октант когда метрики AxialEvaluator дают неточный
результат. Пример: текст про смерть в красивых словах — EntropyScore низкий (Apollonian)
но семантически Thanatos (Y-ось negative).

```
Input:  sutra_id, analytic_octant, synthetic_octant, x/y/z AxialScore,
        evaluation_level, primary_subsystem, primary_octant_from_profile, event_id,
        depth_per_octant: [u16; 8],   ← V2
        reactivation_count: u32        ← V2
Output: Option<OctantSuggestion { octant, confidence, reason }>
Reason: SemanticContent | DepthHistoryBias | SubsystemAffinity | BoundaryResolution
```

**V1:** `None` (слот не заполнен).  
**V2:** `DepthHistoryBiasAdvisor` (реализован, см. §8.3) — предлагает октант с
        наибольшей исторической глубиной если он превышает analytic_octant на
        ≥ DHB_MIN_DEPTH_ADVANTAGE=300 и имеет глубину ≥ DHB_MIN_DEPTH_THRESHOLD=800.
        Advisory несёт `octant_hint: Some(octant.index())` → CognitiveProfile в Arbiter.  
**V9:** embedding-модель понимает содержание; SubsystemAffinity (математика тяготеет
        к FormalDenying/HeroicFatal, поэзия — к CreativeAffirmation/PassiveSentimental).

### 4.3 CorpusCallosumResolver

Диагностирует конфликт между аналитическим и синтетическим октантом. Текущая система
фиксирует конфликт и оставляет `ConflictResolution::Unresolved`. Резольвер объясняет
*почему* конфликт существует.

```
Input:  sutra_id, analytic_octant, synthetic_octant, conflict_strength (0..255),
        frame_age_ticks, reactivation_count, primary_subsystem, event_id
Output: ConflictResolutionHint { diagnosis: ConflictDiagnosis, confidence }

ConflictDiagnosis:
  Unresolved            — не удалось диагностировать
  BoundaryFrame         — Frame живёт на границе двух октантов, это структурно нормально
  TransitionState       — Frame между состояниями, временно (молодой Frame)
  DualNature            — Frame стабильно принадлежит обоим октантам (зрелый, часто реактивируется)
  DominantOctant(Octant)— один октант явно побеждает по истории активности
```

**V1:** RuleBasedCorpusCallosumResolver (реализован, см. §8.1).  
**V2:** PatternLearningResolver — учится на истории конфликтов; какие типы Frame
        обычно дают BoundaryFrame, какие — TransitionState.  
**V9:** semantic model — понимает что "граница Apollo/Dionysus в контексте математики"
        означает BoundaryFrame, а не TransitionState.

### 4.4 SubsystemAttributionAdvisor

Дополняет энергетическую детекцию ContextRecognizer. ContextRecognizer работает по
близости к опорным примитивам — это точно но ограниченно. Советник видит паттерн
который энергетически неочевиден.

```
Input:  sutra_id, energy_weights: HashMap<SubsystemId, u8>, primary_octant,
        depth_per_octant[8], reactivation_count, event_id
Output: Option<SubsystemSuggestion { primary, secondary: Option<SubsystemId>, confidence }>
```

**V1:** `None` (слот не заполнен).  
**V2:** `AnchorVotingAdvisor` (реализован, см. §8.4) — голосует по energy_weights из
        InterpretationProfile с depth-бонусом per subsystem через SUBSYSTEM_AFFINITY.
        Поддерживает dual-subsystem вывод если gap < AV_DUAL_THRESHOLD=0.15.  
**V9:** embedding-классификатор не зависит от AnchorSet proximity вообще.

### 4.5 EmergentPatternAdvisor

Обнаруживает кандидатов в новые примитивы. Текущий `try_detect_emergent` в
ContextRecognizer всегда возвращает false. Это заменяет его реальной логикой.

```
Input:  sutra_id, octant, depth_per_octant[8], reactivation_count,
        frame_age_ticks, known_primitive_ids: &[u32], event_id
Output: EmergentDetectionResult { is_candidate: bool, confidence: f32 }
```

Нет `suggested_name` в V1 — имя требует семантического понимания.  
Если `is_candidate = true` → NeuralAdvisor добавляет в EmergentPrimitiveStore и
посылает `NotifyEmergentCandidate` UCL-команду. Далее chrnv одобряет вручную.

**V1:** DepthThresholdEmergentDetector (реализован, см. §8.2).  
**V2:** ClusteringEmergentDetector — обнаруживает кластеры в семантическом пространстве
        которым нет соответствующего примитива в AnchorSet.  
**V9:** генеративная модель — предлагает имя и начальные координаты для нового примитива.

---

## 5. NeuralAdvisorRegistry

Центральный объект. Создаётся один раз, передаётся в `NeuralAdvisor::new()`.

Каждый слот — `Option<Arc<dyn Advisor>>`. `None` означает "слот отсутствует". Отличие
от `NullAdvisor`: null-реализация занимает слот явно (виден в логах, геном его видит),
`None` — слот незаполнен.

V1 дефолтная конфигурация (`default_v1()`):
- depth:    `Some(Arc<ReactivationDepthAdvisor>)`
- octant:   `None`
- conflict: `Some(Arc<RuleBasedCorpusCallosumResolver>)`
- subsystem:`None`
- emergent: `Some(Arc<DepthThresholdEmergentDetector>)`

V2 дефолтная конфигурация (`default_v2()`) — все 5 слотов:
- depth:    `Some(Arc<ReactivationDepthAdvisor>)`
- octant:   `Some(Arc<DepthHistoryBiasAdvisor>)`
- conflict: `Some(Arc<RuleBasedCorpusCallosumResolver>)`
- subsystem:`Some(Arc<AnchorVotingAdvisor>)`
- emergent: `Some(Arc<DepthThresholdEmergentDetector>)`

Engine использует `NeuralAdvisor::with_default_v2()` начиная с 2026-05-23.

---

## 6. AdvisoryResult

Один объект на Frame, обновляется каждый тик NeuralAdvisor.

```
AdvisoryResult {
    sutra_id:           u32,
    computed_at_event:  u64,
    octant_suggestion:  Option<OctantSuggestion>,
    conflict_diagnosis: Option<ConflictResolutionHint>,
    subsystem_suggestion: Option<SubsystemSuggestion>,
    depth_hint:         Option<DepthHint>,
    // emergent candidates tracked separately in EmergentPrimitiveStore
}
```

Хранится в `AdvisoryResultStore: HashMap<u32, AdvisoryResult>`. In-memory.
Overwrite каждый тик — текущий снапшот.

**AdvisoryHistory (V2)** — хранит историю советов per sutra_id:

```
AdvisoryHistory { per_sutra: HashMap<u32, AdvisoryRingBuffer>, cap_per_sutra: usize=32 }

AdvisoryHistoryEntry {
    computed_at_event:    u64,
    octant_suggestion:    Option<Octant>,
    octant_confidence:    f32,
    subsystem_suggestion: Option<SubsystemId>,
    subsystem_confidence: f32,
    outcome:              AdvisoryHistoryOutcome,
}

AdvisoryHistoryOutcome: Pending | Applied | Confirmed | Rejected | Skipped
```

Методы `AdvisoryHistory`: `record()`, `update_outcome(sutra_id, event_id, outcome)`,
`acceptance_rate_octant()`, `dominant_accepted_octant()`.
Кольцевой буфер вытесняет самые старые записи при превышении cap=32.

---

## 7. NeuralAdvisor как OverDomainComponent

```
ModuleId::NeuralAdvisor = 19
Tick interval: 11 (нечётное, не совпадает с AxialEvaluator=5, ContextRecognizer=7)
```

**on_boot:** проверяет ExperienceMemory/Read + AshtiField/Read в геноме.

**on_tick:**
1. Получить список активных Frame-анкеров из EXPERIENCE (AshtiCore)
2. Для каждого Frame: собрать входные данные из снапшотов
3. Вызвать каждый советник из registry
4. Сохранить в AdvisoryResultStore
5. Для emergent кандидатов: добавить в EmergentPrimitiveStore, emit UCL

**Синхронизация снапшотов** — аналог CR-TD-01. Нужен внешний координатор для вызова
`sync_axial_store`, `sync_profile_store`, `sync_depth_store` после каждого тика
соответствующих компонентов. Это архитектурный долг, общий для всех над-доменных
компонентов которые читают чужие сторы. Задокументирован в DEFERRED.md → CR-TD-01.

---

## 8. V1 реализации

### 8.1 RuleBasedCorpusCallosumResolver

Правила основаны на `conflict_strength` (1 бит = 85, 2 бита = 170, 3 бита = 255),
возрасте Frame и числе реактиваций.

| conflict_strength | frame_age_ticks       | reactivation_count   | Диагноз         | confidence |
|-------------------|-----------------------|----------------------|-----------------|------------|
| 85 (1 ось)        | любой                 | любой                | BoundaryFrame   | 0.80       |
| 170 (2 оси)       | < TRANSITION_AGE (20) | любой                | TransitionState | 0.65       |
| 170 (2 оси)       | ≥ TRANSITION_AGE      | ≥ STABLE_REACT (10)  | DualNature      | 0.70       |
| 255 (3 оси)       | любой                 | любой                | Unresolved      | 0.50       |
| любой             | —                     | —                    | Unresolved      | 0.50       |

Константы подбираются по OBS-01 (первые живые данные).

### 8.2 DepthThresholdEmergentDetector

Кандидат обнаруживается если Frame:
- Не является уже известным примитивом (не в `known_primitive_ids`)
- Имеет глубину в заданном октанте ≥ `EMERGENT_CANDIDATE_MIN_DEPTH` (~~8000~~ → **1000**)
- Имеет `reactivation_count` ≥ `EMERGENT_CANDIDATE_MIN_REACTIVATIONS` (~~30~~ → **5**)
- Старше `EMERGENT_CANDIDATE_MIN_AGE_TICKS` (100 тиков)

Confidence = 0.60. Не претендует на высокую точность — сигнал для chrnv, не решение.

**Калибровка порогов (OBS-02, 2026-05):** По данным OBS-02 средняя глубина активных Frame (`O7 avg_depth`) составила ~1198. Исходные пороги (MIN_DEPTH=8000, MIN_REACTIVATIONS=30) никогда не выполнялись — ни один Frame не становился emergent-кандидатом. Дополнительная причина нулевых реактиваций: `reactivation_count` в `SutraDepthEntry` не инкрементировался при поступлении `evidence` (мёртвое поле), исправлено — см. ниже.

Пороги снижены до реалистичных значений: за 30k тиков происходит ~10–15 DREAM-циклов, что даёт ~10–15 реактиваций при стабильном Frame.

### 8.3 DepthHistoryBiasAdvisor (V2, octant slot)

Советник по исторической глубине октантов. Источник: SutraDepthStore.depth_per_octant[8].

| Константа | Значение | Смысл |
|-----------|----------|-------|
| DHB_MIN_DEPTH_THRESHOLD | 800 | минимум чтобы рассматривать октант |
| DHB_MIN_DEPTH_ADVANTAGE | 300 | преимущество лидера над analytic_octant |
| DHB_CONFIDENCE_DEPTH_NORM | 3000.0 | нормировщик: depth/norm, cap 0.85 |
| DHB_MIN_FULL_TRUST_REACTIVATIONS | 10 | ниже — confidence штрафуется пропорционально |

Алгоритм:
1. `best_idx` = argmax(depth_per_octant)
2. Если `best_depth < DHB_MIN_DEPTH_THRESHOLD` → None
3. Если `best_idx == analytic_idx` && нет конфликта (analytic == synthetic) → None
4. Если `best_idx != analytic_idx` && `best_depth − analytic_depth < DHB_MIN_DEPTH_ADVANTAGE` → None
5. `confidence = (best_depth / 3000).min(0.85) × reactivation_penalty`
   где penalty = reactivation_count / DHB_MIN_FULL_TRUST_REACTIVATIONS при count < 10, иначе 1.0
6. Если `confidence <= 0.0` → None
7. Возвращает `OctantSuggestion { octant, confidence, reason: DepthHistoryBias }`

Advisory несёт `octant_hint: Some(octant.index())` → Arbiter передаёт в CognitiveProfile.

### 8.4 AnchorVotingAdvisor (V2, subsystem slot)

Советник по атрибуции подсистемы через взвешенное голосование примитивов.

| Константа | Значение | Смысл |
|-----------|----------|-------|
| AV_MIN_ENERGY_WEIGHT | 20 | минимальный вес для участия в голосовании |
| AV_DOMINANCE_THRESHOLD | 0.50 | доля для уверенного вывода; ниже — confidence × 0.7 |
| AV_DUAL_THRESHOLD | 0.15 | gap между первым и вторым для dual-subsystem вывода |
| AV_MIN_FULL_TRUST_REACTIVATIONS | 5 | ниже — штраф на confidence |

Алгоритм:
1. Фильтр `energy_weights` по `AV_MIN_ENERGY_WEIGHT`
2. `score(s) = weight × depth_bonus(s)`
   где `depth_bonus = (1.0 + depth[SUBSYSTEM_AFFINITY[s]] / 2000.0).min(2.0)`
   `SUBSYSTEM_AFFINITY: [u8; 16]` — таблица SubsystemId → affine octant index (из depth.rs)
3. Нормировать scores → доли. `primary = argmax`
4. Если `primary_share < AV_DOMINANCE_THRESHOLD` → `confidence *= 0.7`
5. Если `gap(primary, second) < AV_DUAL_THRESHOLD` → добавить `secondary: Some(second_subsystem)`
6. Reactivation penalty аналогично DHB: count < 5 → штраф

---

## 9. UCL команды

```
NotifyEmergentCandidate = 5200
  Payload: { sutra_id: u32, octant: u8, depth: u16, confidence_scaled: u8 }
  Направление: NeuralAdvisor → Workstation/chrnv

ApproveEmergentCandidate = 5201
  Payload: { sutra_id: u32 }
  Направление: chrnv → NeuralAdvisor
  Эффект: emergent_store.approve(sutra_id) + SutraDepthStore::register_primitive(sutra_id)
```

---

## 10. Genome-интеграция

NeuralAdvisor в `on_boot` проверяет:
- `ExperienceMemory / Read`
- `AshtiField / Read`

Какие советники загружены в registry — решается при конструировании `NeuralAdvisor`
(вне genome-контроля в V1/V2). Genome-per-advisor control (например, запретить
EmergentDetector в production конфиге) — V3+.

В `config/genome.yaml` добавляется секция для `NeuralAdvisor` с теми же ресурсами.

**Advisory.octant_hint (V2):** поле `Option<usize>` в Advisory. `Some(idx)` для
OctantCorrection advisory; `None` для всех остальных типов. Используется Arbiter для
направления feedback в CognitiveProfile.

**CognitiveProfile (V2, в OverDomainArbiter):**

```
CognitiveProfile { octant_weights: [f32; 8], init 1.0 }
  WEIGHT_MIN = 0.5, WEIGHT_MAX = 2.0, LEARNING_RATE = 0.05

scale_confidence(octant_idx, raw) → (raw × weight).max(1.0) [cap снизу чтобы не отсекать]
update(octant_idx, accepted):
  accepted  → weight += 0.05
  rejected  → weight -= 0.05
  clamp [WEIGHT_MIN, WEIGHT_MAX]
```

Ортогонален `TrustConfig`: TrustConfig = кому доверять (источник × тип), CognitiveProfile =
куда смотреть (per-octant bias). Вместе дают двумерный контроль над потоком советов.

---

## 11. Связь с другими компонентами

| Компонент          | Отношение                                          |
|--------------------|----------------------------------------------------|
| AxialEvaluator     | Читает AxialStore снапшот (sync_axial_store)       |
| ContextRecognizer  | Читает InterpretationProfileStore + SutraDepthStore|
| DreamPhase         | Не связан напрямую в V1                            |
| FrameWeaver        | Не связан напрямую в V1                            |
| context_recognizer/advisors/ | Устарел, удалён в V1 NeuralAdvisor       |

---

## 12. Что в V2+

### V2 ✅ (реализовано 2026-05-23)
- DepthHistoryBiasAdvisor: OctantCorrection на основе SutraDepth истории (§8.3)
- AnchorVotingAdvisor: SubsystemAttribution через взвешенное голосование (§8.4)
- AdvisoryHistory: кольцевой буфер 32 записей per sutra_id с outcome
- Advisory.octant_hint: Option<usize> для feedback-контура CognitiveProfile
- CognitiveProfile в Arbiter: per-octant online learning rate=0.05
- OctantAdvisorInput расширен: depth_per_octant[8] + reactivation_count

### V3
- ClusteringEmergentDetector: обнаружение семантических кластеров без примитива
- PatternLearningResolver: обучаемый ConflictResolver на истории конфликтов
- Workstation: отображение AdvisoryResult рядом с детерминированными результатами
- Логирование расхождений advisor vs deterministic (сигнал при gap 2+ оси)
- Genome-per-advisor control (Override permission per advisor type)

### V9
- Все пять трейтов удовлетворены обученными моделями
- NeuralAdvisorRegistry заполняется из конфига genome.yaml (model endpoints)
- EmergentPatternAdvisor предлагает имя и координаты нового примитива
- OctantCorrectionAdvisor понимает семантический контент через embeddings
- Механизм промоции: Advisory → Override через геном после наблюдения

---

## 13. Исправление: SutraDepthStore::reactivation_count (2026-05)

В `SutraDepthEntry.reactivation_count` обнаружена ошибка: поле инициализировалось в 0 и никогда не обновлялось — `apply_evidence` не инкрементировал его. В результате `qualifies_for_promotion` (и `DepthThresholdEmergentDetector`) всегда видели `reactivation_count == 0`.

**Исправление** в `SutraDepthStore::apply_evidence`:

```rust
if evidence > 0 {
    entry.reactivation_count = entry.reactivation_count.saturating_add(1);
}
```

Поле теперь отражает реальное число DREAM-циклов, в которых Frame имел evidence > 0.

---

## История

- **V1.0** (2026-05-17): Advisory-only архитектура. Пять трейтов. RuleBasedCorpusCallosumResolver,
  DepthThresholdEmergentDetector, ReactivationDepthAdvisor. Слоты octant/subsystem — None.
  Заменяет `context_recognizer/advisors/mod.rs` (стаб заглушки).
- **V1.0 patch** (2026-05): Калибровка порогов DepthThresholdEmergentDetector по OBS-02.
  Исправление `reactivation_count` в SutraDepthStore. Всё в рамках V1.0 без структурных изменений.
- **V2.0** (2026-05-23): Все 5 слотов заполнены. DepthHistoryBiasAdvisor (octant),
  AnchorVotingAdvisor (subsystem). AdvisoryHistory ring-32. OctantAdvisorInput расширен.
  Advisory.octant_hint. CognitiveProfile в OverDomainArbiter с online learning.
  Engine переведён на `with_default_v2()`. 1452 тестов.
