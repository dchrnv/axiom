# NeuralAdvisor V1.0

**Статус:** реализован (V1)  
**Дата:** 2026-05-17  
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
- Не перезаписывает их результаты (до V2, см. §3)
- Не требует ML для базовой работы — V1 реализован на правилах и порогах

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

### На что обратить внимание в V2+

1. **Workstation должна показывать расхождение.** Рядом с аналитическим октантом —
   `advisory_octant` если есть и confidence > 0.5. Это главный инструмент калибровки.

2. **Промоция советника через геном.** Для каждого типа советника геном может
   разрешить `Override` (аналог `Permission::Control`). Переход от Advisory к Override
   происходит явно, по конкретному типу, после наблюдения.

3. **Логировать случаи значительного расхождения.** Если `advisory_octant` отличается
   от `analytic_octant` на 2+ оси — это сигнал для анализа. Либо советник ошибается,
   либо метрики AxialEvaluator неадекватны для этого фрейма.

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

**V1:** NullDepthAdvisor — всегда `None`. Система использует organic depth growth.  
**V2:** обучен на SutraDepthStore истории; предсказывает settling depth для новых Frame  
        на основе их InterpretationProfile и контекста создания.  
**V9:** нейронная модель с embedding нового Frame как входом.

### 4.2 OctantCorrectionAdvisor

Предлагает семантически правильный октант когда метрики AxialEvaluator дают неточный
результат. Пример: текст про смерть в красивых словах — EntropyScore низкий (Apollonian)
но семантически Thanatos (Y-ось negative).

```
Input:  sutra_id, analytic_octant, synthetic_octant, x/y/z AxialScore,
        evaluation_level, primary_subsystem, primary_octant_from_profile, event_id
Output: Option<OctantSuggestion { octant, confidence, reason }>
Reason: SemanticContent | DepthHistoryBias | SubsystemAffinity | BoundaryResolution
```

**V1:** NullOctantAdvisor — всегда `None`.  
**V2:** DepthHistoryBiasAdvisor — если Frame исторически активен в октанте X,
        предлагает X при низкой уверенности детерминированного результата.  
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

**V1:** NullSubsystemAdvisor — всегда `None`. ContextRecognizer достаточен.  
**V2:** AnchorVotingAdvisor — вместо ближайшего примитива, голосование по всем
        примитивам подсистемы с весами; cross-octant subsystem signals.  
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

В V1 дефолтная конфигурация:
- depth: `None`
- octant: `None`  
- conflict: `Some(Arc<RuleBasedCorpusCallosumResolver>)`
- subsystem: `None`
- emergent: `Some(Arc<DepthThresholdEmergentDetector>)`

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
Overwrite каждый тик — не накапливается история. История советника: V2+.

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
- Имеет глубину в заданном октанте ≥ `EMERGENT_CANDIDATE_MIN_DEPTH` (8000)
- Имеет `reactivation_count` ≥ `EMERGENT_CANDIDATE_MIN_REACTIVATIONS` (30)
- Старше `EMERGENT_CANDIDATE_MIN_AGE_TICKS` (100 тиков)

Confidence = 0.60. Не претендует на высокую точность — сигнал для chrnv, не решение.

Все пороги — именованные константы, будут откалиброваны по OBS-01.

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
(вне genome-контроля в V1). Genome-per-advisor control (например, запретить
EmergentDetector в production конфиге) — V2+.

В `config/genome.yaml` добавляется секция для `NeuralAdvisor` с теми же ресурсами.

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

### V2
- DepthHistoryBiasAdvisor: OctantCorrection на основе SutraDepth истории
- ClusteringEmergentDetector: обнаружение семантических кластеров без примитива
- Логирование расхождений advisor vs deterministic
- Genome-per-advisor control (Override permission per advisor type)
- AdvisoryHistory: хранить последние N результатов для trend analysis

### V3
- PatternLearningResolver: обучаемый ConflictResolver на истории конфликтов
- AnchorVotingAdvisor: SubsystemAttribution через голосование примитивов
- Workstation: отображение AdvisoryResult рядом с детерминированными результатами

### V9
- Все пять трейтов удовлетворены обученными моделями
- NeuralAdvisorRegistry заполняется из конфига genome.yaml (model endpoints)
- EmergentPatternAdvisor предлагает имя и координаты нового примитива
- OctantCorrectionAdvisor понимает семантический контент через embeddings
- Механизм промоции: Advisory → Override через геном после наблюдения

---

## История

- **V1.0** (2026-05-17): Advisory-only архитектура. Пять трейтов. RuleBasedCorpusCallosumResolver,
  DepthThresholdEmergentDetector. Null-реализации для остальных.
  Заменяет `context_recognizer/advisors/mod.rs` (стаб заглушки).
