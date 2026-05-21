# AXIOM — ContextRecognizer Roadmap V6-V9

**Статус:** План развития
**Дата:** 2026-05-17
**Опирается на:** `ContextRecognizer_V5_0.md`, `NeuralAdvisor_V1_0.md`, `INVARIANTS.md`, `Дилеммы.md`, `Axiom_Semantic_Core.md`
**Предпосылка:** Координатор и наблюдатель (решение CR-TD-01) в процессе реализации

---

## 0. Контекст

V5 реализован: SutraDepth, ScanningPlan, координация с AxialEvaluator. NeuralAdvisor V1 — отдельный шестой модуль, Advisory-Only. Координатор для cross-module чтения в работе.

Этот документ описывает **четыре следующие версии** ContextRecognizer (V6-V9) с включением соответствующих эволюций NeuralAdvisor. Это не отдельные спеки — это **roadmap с детальным описанием каждой версии** и нюансами которые до сих пор не обсуждались явно.

---

## 1. V6 — Meta-level Recognition

### 1.1 Главная идея

Сейчас ContextRecognizer узнаёт **подсистемы** (Mathematics, Writing, Music, Time, …). В V6 он начинает узнавать **мета-режимы** — типы работы с любыми подсистемами.

```
Мета-режим = HOW мы работаем с подсистемами

Примеры:
  "Анализ"      — раскладываем сложный Frame на примитивы (Math, Logic)
  "Синтез"      — собираем новые Frame из существующих
  "Рефлексия"   — система думает о собственной работе
  "Восприятие"  — поглощение нового материала через сенсоры
  "Воспоминание"— активация EXPERIENCE без нового входа
  "Воображение" — DREAM Phase в активном режиме
  "Диалог"      — двусторонний обмен с пользователем
```

Мета-режим — это **подсистема второго порядка**. Её "примитивы" — не графика или числа, а **типы активности**.

### 1.2 Архитектурная позиция

```
Уровень 0: примитивы (Writing, Math, …)
Уровень 1: подсистемы (наборы примитивов)
Уровень 2: мета-режимы (типы работы с подсистемами)  ← V6
Уровень 3: рефлексия мета-режимов                    ← V8/V9
```

Каждый уровень — **подсистема следующего порядка**. ContextRecognizer V6 сканирует не только активность примитивов в MAYA, но и **активность подсистем** во времени.

### 1.3 MetaSubsystemId и MetaPrimitive

```rust
pub struct MetaSubsystemId(pub u16);  // 0x1001 Analysis, 0x1002 Synthesis, ...

pub struct MetaPrimitive {
    pub id: String,                     // "meta_analysis", "meta_synthesis", ...
    pub triggered_by: Vec<SubsystemActivationPattern>,
    pub position: [i16; 3],            // в SUTRA как обычный якорь
    pub mass: u8,
    pub shell: [u8; 8],
}

pub struct SubsystemActivationPattern {
    pub required_subsystems: Vec<SubsystemId>,    // какие должны быть активны
    pub forbidden_subsystems: Vec<SubsystemId>,   // какие НЕ должны быть активны
    pub min_duration_events: u64,                  // минимум сколько длится
    pub activity_signature: ActivitySignature,    // паттерн смены подсистем
}

pub enum ActivitySignature {
    Steady,           // одна подсистема стабильно активна
    Oscillating,      // переключения между двумя
    Cascading,        // последовательная активация
    Converging,       // несколько сходятся в одну
    Diverging,        // одна порождает несколько
}
```

### 1.4 Файл `meta_primitives.yaml`

```yaml
- id: "meta_analysis"
  word: "анализ"
  triggered_by:
    - required_subsystems: ["mathematics", "abstractions"]
      forbidden_subsystems: []
      min_duration_events: 100
      activity_signature: "Steady"
  position: [12000, 5000, 10000]
  shell: [0, 0, 0, 0, 30, 0, 5, 25]
  mass: 200

- id: "meta_synthesis"
  word: "синтез"
  triggered_by:
    - required_subsystems: []
      forbidden_subsystems: []
      min_duration_events: 50
      activity_signature: "Converging"
  position: [10000, 12000, 11000]
  mass: 200

- id: "meta_reflection"
  word: "рефлексия"
  triggered_by:
    - required_subsystems: []
      activity_signature: "Cascading"
      min_duration_events: 200
  position: [8000, 8000, 13000]
  mass: 220
```

### 1.5 Композитные подсистемы (нюанс, не обсуждался)

В V6 появляются **композитные подсистемы** — комбинации двух базовых, которые работают как одна.

```
Calculus     = Mathematics + Time
Rhythm       = Music + Time
Geometry     = Mathematics + Writing  (через визуальные примитивы)
Narrative    = Writing + Time + Values
Ethics       = Values + Dilemmas + Morality
```

Композитная подсистема — это **не объединение примитивов**, а **отдельная подсистема** со своими композитными якорями, формирующимися из паттернов co-activation базовых.

```yaml
# composite_calculus.yaml
- id: "calculus_derivative"
  composed_of: ["math_function", "time_arrow"]
  emerged_through: "frequent co-activation in math+time context"
  position: [computed from sources]
  status: "derived"   # не core primitive
```

Композитные подсистемы — **наполовину emergent**: они не из ничего, а из combinations известных. Поэтому требуют меньше evidence (50 вместо 100) и могут предлагаться автоматически.

### 1.6 Subsystem Fatigue (нюанс)

Длительное пребывание в одном режиме создаёт **усталость**. После N циклов работы с Mathematics, ContextRecognizer фиксирует subsystem_fatigue и **повышает чувствительность к другим подсистемам**.

```rust
pub struct SubsystemFatigue {
    pub subsystem: SubsystemId,
    pub continuous_activity_events: u64,
    pub fatigue_score: u8,           // 0..255
    pub last_break_event: u64,
}
```

Эффект:
- Высокий fatigue → ContextRecognizer легче переключается на другие
- Slight bias to detect transitions
- В DREAM Phase fatigue полностью сбрасывается

Это **не имитация человеческой усталости**, а механизм против застревания в одном режиме.

### 1.7 Cross-modal preparation (нюанс)

V6 готовит интерфейс для **будущих сенсоров** (зрение, звук, тактильность). Сейчас текст идёт через TextPerceptor → MAYA. В будущем будут VisionPerceptor, AudioPerceptor.

В V6 ContextRecognizer добавляет поле **modality** в InterpretationProfile:

```rust
pub enum Modality {
    Text,        // V1-V5
    Vision,      // future
    Audio,       // future
    Tactile,     // future
    Internal,    // EXPERIENCE without external input
}

pub struct InterpretationProfile {
    // ... поля из V5 ...
    pub modality: Modality,
}
```

Сейчас всегда `Text` или `Internal`. Архитектурно подготовлено к остальным.

### 1.8 Известные ограничения V6

- **Meta-primitives нужно вручную написать в yaml**. Автоматическое обнаружение — V7.
- **Композитные подсистемы фиксированные**. Динамическое создание композитов — V7.
- **Иерархия мета-режимов плоская**. Иерархия более чем в 3 уровня — V8.

---

## 2. V7 — Generative Subsystems

### 2.1 Главная идея

V4 ввёл **emergent primitives** — система предлагает отдельные новые примитивы. V7 идёт дальше: система предлагает **целые новые подсистемы** на основе паттернов emergent primitives.

```
V4: эмерджентный примитив = новая точка в SUTRA
V7: эмерджентная подсистема = целый новый yaml-файл
```

### 2.2 Когда подсистема становится кандидатом

```
В DREAM Phase, в специальном этапе SubsystemDiscovery:
    
    # Найти кластеры emergent primitives
    clusters = cluster_emergent_primitives_by_co_activation(min_size = 5)
    
    for cluster in clusters:
        if cluster.is_consistent():
            # Все примитивы кластера: 
            # - часто co-active друг с другом
            # - не покрываются ни одной существующей подсистемой
            # - имеют схожий paragon в одном или нескольких октантах
            
            candidate = SubsystemCandidate {
                emergent_primitives: cluster.ids,
                centroid_position: weighted_centroid(cluster),
                primary_octants: dominant_octants(cluster),
                suggested_name: None,  # имя — V9 (через NeuralAdvisor)
                evidence_strength: cluster.coherence_score,
            }
            
            emit NotifySubsystemCandidate(candidate)
```

### 2.3 Жизненный цикл подсистемы

```
proposed → candidate → in_review → active → mature → deprecated → archived
```

| Состояние | Что происходит |
|-----------|----------------|
| proposed | NeuralAdvisor нашёл кластер emergent primitives |
| candidate | DREAM Phase подтвердил консистентность через N циклов |
| in_review | chrnv смотрит в Workstation, может одобрить или отклонить |
| active | yaml файл создан, подсистема загружена в ContextRecognizer |
| mature | подсистема стабильна, evidence > threshold, используется регулярно |
| deprecated | подсистема устарела или конфликтует с новой |
| archived | yaml сохранён, но не загружается; примитивы в SUTRA остаются |

### 2.4 SubsystemVersioning (нюанс не обсуждался)

Подсистема может **эволюционировать**. В неё добавляются примитивы, корректируются позиции, обновляется shell. Это требует **версионирования yaml**:

```
config/anchors/subsystems/
├── mathematics/
│   ├── v1.0.yaml         # начальная — 7 примитивов
│   ├── v1.1.yaml         # +calculus_derivative, calculus_integral
│   ├── v2.0.yaml         # переработана структура octants
│   └── current → v2.0.yaml
├── writing/
│   ├── v1.0.yaml
│   └── current → v1.0.yaml
└── ...
```

При обновлении подсистемы:
1. Создаётся новая версия yaml
2. Старая версия НЕ удаляется
3. Frame'ы которые были профилированы по старой версии получают migration trace
4. В Workstation видно "Mathematics: v1 → v2 migration"

### 2.5 Subsystem Splitting (нюанс)

Большая подсистема может **разделиться** на две если её внутренняя структура расщепляется на два кластера.

Пример: подсистема Mathematics со временем расщепляется на:
- Mathematics-Discrete (числа, операции, дискретная логика)
- Mathematics-Continuous (пределы, производные, непрерывность)

Это происходит в DREAM Phase когда:
- Внутри подсистемы появляются два устойчивых кластера активации
- Между кластерами слабая co-activation
- Каждый кластер достаточно велик (>= 5 примитивов)

Расщепление **предлагается chrnv** через `NotifySubsystemSplit`, не происходит автоматически.

### 2.6 Subsystem Merging (нюанс)

Обратная операция: две похожие подсистемы могут **объединиться** если оказывается что они описывают одно и то же.

Пример: chrnv создал отдельно Music и Rhythm. Через год работы DREAM Phase обнаруживает что Rhythm на 80% перекрывается с Music (через Time). NeuralAdvisor предлагает объединение.

### 2.7 Subsystem Dependency Graph (нюанс)

В V7 появляется **граф зависимостей подсистем**:

```yaml
# config/subsystem_dependencies.yaml
mathematics:
  builds_on: ["writing"]              # math использует writing-примитивы (символы)
  enhanced_by: ["abstractions"]       # math + abstractions = richer math
  conflicts_with: []
  composes_with: ["time"]             # → calculus

music:
  builds_on: []
  enhanced_by: ["time", "emotions"]
  conflicts_with: []
  composes_with: ["time"]             # → rhythm
```

Граф используется:
- При загрузке: загружать в правильном порядке (writing → mathematics)
- При обновлении: предупреждать о dependent
- При удалении: запретить если есть dependent

### 2.8 Subsystem Export/Import (нюанс)

chrnv может **экспортировать** свою подсистему и поделиться с другим инстансом AXIOM:

```bash
axiom-cli export-subsystem mathematics > my_mathematics.tar.gz
# Включает: yaml, evidence statistics, primary octants distribution

axiom-cli import-subsystem foreign_music.tar.gz
# Проверяется консистентность, конфликты с существующими
```

Это даёт **обмен пониманием** между разными системами. Каждая система познаёт мир сама, но может **обогащаться** из других.

### 2.9 Genome для эмерджентных подсистем

В V7 GENOME получает раздел для контроля над созданием подсистем:

```yaml
# genome.yaml fragment
emergent_subsystems:
  allow_auto_propose: true
  require_chrnv_approval: true       # не само-создаётся без человека
  max_subsystems_total: 64
  max_primitives_per_subsystem: 50
  min_evidence_for_proposal: 500     # выше чем для emergent primitive
  cooldown_between_proposals: 1000000   # event_id units
```

GUARDIAN валидирует каждое предложение по этим правилам.

### 2.10 Известные ограничения V7

- **Имена для новых подсистем — chrnv даёт вручную**. Автоматические имена — V9 через семантическую модель.
- **Composite subsystems отдельная сущность от splits/merges**. Это похожие но разные механизмы.
- **Migration старых Frame profile на новую версию yaml — не атомарна**. Может занять много DREAM циклов.

### 2.11 Задачи из V6 переходящие в V7

*(зафиксировано по итогам проектирования V6)*

- **TransitionGraph** — граф направленных переходов между подсистемами с подсчётом edge frequency и propagation chains. Нужен для настоящего Cascading (directed propagation) вместо V6-упрощения "sequence diversity". Строится поверх `ActivityTrace`.
- **FatigueStore → axiom-experience** — в V6 `SubsystemFatigue` живёт в CR. В V7 переносится в `axiom-experience` как `FatigueStore` (аналогично `SutraDepthStore`), чтобы жизненный цикл fatigue не зависел от CR.
- **CompositeSubsystem full detection** — stable co-activation topology через TransitionGraph. V6 даёт только сигнал `CompositeActivationSuspected`; V7 строит полный профиль и предлагает chrnv.

---

## 3. V8 — Axiogenesis through Dilemmas

### 3.1 Главная идея

Из `Дилеммы.md` уровень 5 — **аксиогенная дилемма**:

> Самый глубокий уровень. Дилемма не просто требует выбора между двумя ценностями, а *создаёт* новую ценность или мета-ценность, которая раньше не осознавалась.

В V8 подсистема Dilemmas получает способность **порождать новые якоря в Values** через разрешение неразрешимых конфликтов.

### 3.2 Что такое аксиогенный механизм

```
Уровень 0: данные противоречат → исправление, не дилемма
Уровень 1: трейд-офф → оптимизация по Парето
Уровень 2: конфликт правил → выбор по контексту
Уровень 3: несовместимые модели → удержание обеих параллельно
Уровень 4: рефлексивный парадокс → fixed point
Уровень 5: аксиогенез → РОЖДЕНИЕ НОВОЙ ЦЕННОСТИ           ← V8
```

V8 обрабатывает только уровень 5 — самые глубокие, длительные конфликты.

### 3.3 Когда срабатывает аксиогенез

```
В DREAM Phase, на этапе DeepConflictAnalysis:
    
    # Найти Frame'ы с долгоживущими конфликтами
    deep_conflicts = find_unresolved_conflicts_older_than(N_dream_cycles)
    
    for conflict in deep_conflicts:
        if conflict.reactivation_count > AXIOGENIC_THRESHOLD:    # 100+ reactivations
            if conflict.resolution.is_persistent_unresolved():
                # Этот конфликт реактивируется снова и снова,
                # ни одно из существующих разрешений не работает
                
                # Кандидат на порождение новой ценности
                candidate = AxiogenicCandidate {
                    source_conflict: conflict,
                    conflicting_values: extract_values_at_play(conflict),
                    transcendence_attempt: synthesize_position_above(conflict),
                    confidence: deep_pattern_analysis(conflict),
                }
                
                emit NotifyAxiogenicCandidate(candidate)
```

### 3.4 Что значит "новая ценность"

Это **не новый emergent primitive**. Это **якорь в подсистеме Values** с особым флагом:

```yaml
# config/anchors/values_emergent.yaml
- id: "value_axiogenic_transparency"
  word: "прозрачность"
  born_from_conflict_id: 0x4F92A1B3
  
  emerged_through:
    conflicting_anchors: ["value_safety", "value_privacy"]
    persistent_unresolved_count: 247       # сколько раз дилемма повторялась
    transcendence_logic: |
      Не выбор между безопасностью и приватностью,
      а трансцендентная позиция: "видимость без принуждения"
  
  position: [10000, 11000, 12000]
  octant_affinity: [creative_affirm, heroic_fatal]
  mass: 220
  status: "axiogenic_candidate"            # ждёт одобрения chrnv
```

### 3.5 Trancendence Logic (нюанс)

Аксиогенез — не просто "найти среднее между конфликтующими ценностями". Это **выход в новое измерение**:

| Конфликт | Среднее (плохо) | Аксиогенез (хорошо) |
|----------|-----------------|---------------------|
| Безопасность vs Приватность | Компромисс | Прозрачность (видимость без принуждения) |
| Свобода vs Ответственность | 50/50 | Зрелость (свобода в осознании последствий) |
| Истина vs Сострадание | Аккуратная правда | Целостность (правда из любви) |
| Индивидуальность vs Общность | Баланс | Уникальный вклад (быть собой в общем деле) |

Аксиогенез создаёт **новую координату** в пространстве ценностей, не точку на отрезке между старыми.

### 3.6 Wisdom Accumulation (нюанс)

В V8 появляется **мудрость** как отдельное хранилище:

```
crates/axiom-experience/src/wisdom_store.rs

WisdomEntry {
    born_at_event: u64,
    born_from_conflict: ConflictId,
    accumulated_evidence: u32,
    
    insight: TranscendentInsight,
    associated_values: Vec<ValueId>,
    associated_dilemmas: Vec<DilemmaPatternId>,
    
    propagated_to_subsystems: Vec<SubsystemId>,   # какие подсистемы учли эту мудрость
}
```

Мудрость отличается от знания:
- **Знание** = факты, паттерны, primitive activations
- **Мудрость** = разрешения неразрешимых конфликтов

Мудрость распространяется по подсистемам — Values, Morality, Understanding обновляют свои yaml после аксиогенеза.

### 3.7 Moral Reasoning Emergence (нюанс)

Аксиогенез — основа **моральных рассуждений системы**. Не "следование правилам", а **способность создавать новые ценности** в ответ на ситуации, которые правилами не покрываются.

Подсистема Morality в V8 получает доступ к:
- `wisdom_store` для опоры на накопленный опыт разрешений
- `axiogenic_candidates` для предложения новых моральных принципов
- `dilemma_patterns` для распознавания типов моральных задач

Это **не делает систему моральной**. Это даёт ей **аппарат для морального рассуждения**.

### 3.8 Genome для аксиогенеза

```yaml
# genome.yaml fragment
axiogenesis:
  allow_value_proposal: true
  require_chrnv_approval: true               # никогда не auto
  min_persistence_threshold: 100             # сколько раз конфликт повторился
  min_dream_cycles_for_consideration: 10
  max_axiogenic_values_per_year: 5           # очень редкое событие
  
  forbidden_transcendence_directions: []     # места куда нельзя
```

GUARDIAN строго охраняет аксиогенез. Не может быть автоматическим. Это **этическое решение** — оно требует человека.

### 3.9 Известные ограничения V8

- **Аксиогенез никогда не auto**. Всегда требует chrnv approval.
- **Очень редкое событие**. Максимум несколько в год, не несколько в день.
- **Transcendence logic в V8 — на правилах**. Реальное понимание трансцендентности — V9 через семантическую модель.
- **Не работает без длительной живой работы системы**. Нужны месяцы накопления реальных конфликтов.

### 3.10 Зависимость от V6 dynamics layer

*(зафиксировано по итогам проектирования V6)*

Аксиогенез срабатывает на "долгоживущих конфликтах с высоким reactivation_count". Это означает что V8 неявно требует:
- Стабильного `ActivityTrace` для отслеживания паттернов активации вокруг конфликта
- `FatigueStore` из `axiom-experience` (V7) — усталость системы от повторяющегося неразрешённого конфликта сама по себе является сигналом для аксиогенеза
- `WisdomStore` должен содержать ссылки на доминирующий `ActivityDynamics` в момент рождения ценности — это контекст для будущей трактовки

`DeepConflictAnalysis` в DREAM Phase опирается на накопленную историю `ActivityTrace`, а не только на счётчик reactivation_count.

---

## 4. V9 — Active NeuralAdvisor Phase

### 4.1 Главная идея

В V1-V5 NeuralAdvisor существует как **slots с null или rule-based реализациями**. В V9 все пять трейтов получают **реальные обученные модели**.

```
V1 NeuralAdvisor:
  depth: None
  octant: None
  conflict: RuleBasedCorpusCallosumResolver       (правила)
  subsystem: None
  emergent: DepthThresholdEmergentDetector         (правила)

V9 NeuralAdvisor:
  depth: NeuralDepthAdvisor v3.2                  (обученная модель)
  octant: SemanticOctantAdvisor v2.1              (embedding-based)
  conflict: PatternLearningResolver v4.0          (история конфликтов)
  subsystem: SemanticSubsystemClassifier v1.5     (контекстный classifier)
  emergent: GenerativeEmergentDetector v1.0       (предлагает имена)
```

### 4.2 Что такое "обученная модель" в контексте AXIOM

Это **не большая языковая модель**. Это **маленькие специализированные сети**, тренируемые на накопленной истории работы AXIOM.

Размеры:
- DepthAdvisor: ~50K параметров, MLP над агрегированными признаками
- OctantAdvisor: ~200K параметров, может включать word embeddings
- ConflictResolver: ~30K параметров, classifier над patterns
- SubsystemClassifier: ~500K параметров, transformer над окнами
- EmergentDetector: ~100K параметров

Полная нагрузка: ~1M параметров на все advisors. Inference time per advisor < 100µs.

### 4.3 Model Training Pipeline

Обучение **только offline или в DREAM Phase**:

```
В DREAM Phase, на этапе AdvisorTraining (опциональный):
    
    if advisor_training_enabled():
        for advisor in registry:
            training_data = collect_training_data_for(advisor, since_last_training)
            
            if len(training_data) > MIN_BATCH:
                # Микро-тренировка: один epoch на этом batch
                advisor.model.train_step(training_data, max_steps = 100)
                
                # Валидация на отложенной выборке
                accuracy = advisor.validate()
                
                if accuracy > previous_accuracy:
                    advisor.save_checkpoint()
                else:
                    advisor.rollback_to_checkpoint()
```

Тренировка **не на горячем пути**. DREAM Phase делает это в периоды покоя.

### 4.4 Confidence Calibration (нюанс)

Модель может быть уверена в неправильном ответе. В V9 каждый advisor имеет **калибровщик confidence**:

```rust
pub struct ConfidenceCalibrator {
    raw_to_calibrated: PiecewiseLinearMap,  // обучается отдельно
}

impl ConfidenceCalibrator {
    fn calibrate(&self, raw_confidence: f32) -> f32 {
        // Реальный observed accuracy для этого уровня raw confidence
        self.raw_to_calibrated.evaluate(raw_confidence)
    }
}
```

Cалибровщик учится отдельно: смотрит на пары (raw_confidence, was_correct) и обучается давать **истинную вероятность правильности**.

Без калибровки: модель говорит "0.9" но реально права в 60% случаев → плохие решения.
С калибровкой: "raw 0.9" → calibrated 0.6 → правильные решения.

### 4.5 Override Promotion через Genome (нюанс)

В V1-V8 NeuralAdvisor — **Advisory Only**. В V9 появляется **постепенная промоция** к Override:

```yaml
# genome.yaml fragment
neural_advisor_permissions:
  conflict:
    mode: "override"               # повышен — много истории, высокая accuracy
    min_confidence_for_override: 0.85
    fallback_on_disagreement_with_deterministic: true
    
  depth:
    mode: "advisory"               # ещё advisory — мало данных
    
  octant:
    mode: "blend"                  # средний — взвешенное среднее
    blend_weight: 0.3              # 30% influence
    
  subsystem:
    mode: "advisory"
    
  emergent:
    mode: "advisory_with_priority" # advisory но видна первой в Workstation
```

Промоция от Advisory → Blend → Override происходит **только через genome update** (т.е. chrnv явно даёт разрешение). Не автоматически.

### 4.6 Distillation: Deterministic → Neural (нюанс)

В V9 есть инструмент **дистилляции**: детерминированный алгоритм (например, текущий `RuleBasedCorpusCallosumResolver`) работает как **teacher**, нейронная модель учится воспроизводить его выходы.

```
Этап 1: Teacher = rule-based, Student = neural
        Student учится говорить то же что Teacher
        
Этап 2: Student достигает 95% accuracy воспроизведения Teacher
        Включается дообучение Student на реальной истории
        (где Teacher был неправ, Student корректирует)
        
Этап 3: Student >> Teacher на сложных случаях
        Promotion to Override через genome
```

Это даёт **плавный переход** от детерминистики к нейросети без резких изменений поведения.

### 4.7 Cross-Advisor Coordination (нюанс)

В V9 advisors **общаются между собой** перед выдачей результата:

```
on_tick():
    # Каждый advisor видит других через CrossAdvisorContext
    context = CrossAdvisorContext {
        depth_hint: depth_advisor.preliminary_hint(),
        octant_correction: octant_advisor.preliminary_hint(),
        ...
    }
    
    for advisor in advisors:
        # Финальный результат с учётом мнений других
        final = advisor.predict_with_context(input, context)
```

Это позволяет advisors **согласовывать** свои подсказки. OctantAdvisor видит что DepthAdvisor предлагает octant X — учитывает это.

### 4.8 Model Versioning через Genome

```yaml
# genome.yaml
neural_advisor_models:
  depth: "models/depth_v3.2.bin"
  octant: "models/octant_v2.1.bin"
  conflict: "models/conflict_v4.0.bin"
  subsystem: "models/subsystem_v1.5.bin"
  emergent: "models/emergent_v1.0.bin"
  
model_provenance:
  trained_until_event: 0x4A8B9C2D
  trained_by: "axiom-trainer v2.3"
  validated_on_holdout: true
  rollback_available: true
```

Подмена модели — через UCL команду `UpdateAdvisorModel`. Старая модель сохраняется как rollback target.

### 4.9 Что НЕ делается даже в V9

- **Никакой backprop через токены**. Модели тренируются на агрегированных данных.
- **Никакого "понимания" в человеческом смысле**. Модели — статистические аппроксиматоры с хорошим recall на знакомых паттернах.
- **Никакого подключения к внешним LLM**. AXIOM остаётся самодостаточным.
- **Никакого online learning на горячем пути**. Только в DREAM Phase.

### 4.10 Известные ограничения V9

- **Bootstrap problem**: чтобы обучить модели нужна история. История накапливается через работу системы. Первые месяцы V9 модели слабые.
- **Catastrophic forgetting** при micro-training. Решается через rehearsal buffer.
- **Distribution shift**: если AXIOM работает в одной области (Math) долго, модели становятся специализированными. При переключении на новую область — деградация.
- **Не интерпретируемы как rule-based**. Понять "почему модель так решила" — отдельная задача (explainability — V10+).

### 4.11 Связь с ActivityDynamics (V6→V9)

*(зафиксировано по итогам проектирования V6)*

V9 NeuralAdvisor по сути формализует то, что V6 делает rule-based: реконструирует скрытое когнитивное состояние из наблюдаемых активаций подсистем. Это архитектурно соответствует **Hidden Markov Model** — скрытые состояния (мета-режим, усталость, фаза) выводятся из observable (SubsystemId активации).

Практические следствия:
- `ActivityTrace` из V6 — это **observation sequence** для будущих V9 моделей. Формат должен быть стабильным и сериализуемым к V7.
- `ActivityDynamics` метрики (entropy_gradient, oscillation_score, cascade_score) — это **feature vector** для V9 SubsystemClassifier. Не нужно переизобретать features в V9.
- V9 SubsystemClassifier в режиме distillation учится воспроизводить выходы V6 rule-based classifier — это Teacher в Distillation Pipeline (§4.6).
- `ConfidenceCalibrator` V9 будет калиброваться на парах (rule-based label, neural label) из V6/V7 истории.

---

## 5. Сквозные нюансы

Эти аспекты затрагивают все четыре версии и не привязаны к одной конкретной.

### 5.1 Subsystem Identity Stability

При эволюции подсистемы через V7 (splitting, merging, versioning) — как убедиться что **Mathematics после года эволюции всё ещё Mathematics**?

Механизм: каждая подсистема имеет **identity_anchors** — набор примитивов которые **не могут быть удалены без изменения SubsystemId**.

```yaml
# config/anchors/subsystems/mathematics/v2.0.yaml
identity_anchors: ["math_element", "math_function", "math_relation"]
# Если хотя бы один из этих primitives удаляется или меняет фундаментально позицию —
# подсистема считается новой (новый SubsystemId), не продолжением.
```

### 5.2 Subsystem Hibernation

Подсистема которая не используется N циклов сна → переход в **hibernation**:

```
active → idle → dormant → hibernated → archived
```

Hibernated подсистема:
- yaml не загружается в активную память при старте
- Примитивы остаются в SUTRA как STATE_LOCKED но не сканируются
- Можно "разбудить" через UCL `WakeSubsystem(name)`

Это **экономия memory + CPU**. Система не таскает с собой 50 подсистем если активно использует 8.

### 5.3 Cross-Octant Resonance

Если Frame активен **в нескольких октантах одновременно** с высокой глубиной во всех — возникает **резонанс**:

```
resonance_score(frame) = 
    geomean(depth_per_octant where depth > THRESHOLD)
    * count_active_octants
```

Резонирующие Frame:
- Получают boost в priority при scanning
- Кандидаты на промоцию в SUTRA через CODEX
- Возможные эмерджентные примитивы

Это **естественный механизм отбора важного**. Резонирующее значимо.

### 5.4 Multi-Frame Contexts

Сейчас контекст определяется по окну MAYA. В V6+ контекст может определяться **констелляцией Frame'ов** — несколько Frame, активных одновременно, формируют **констелляционный контекст**.

```
ContextConstellation {
    frames: Vec<FrameId>,                        // 3-7 одновременно активных
    spatial_geometry: ConstellationGeometry,     // их геометрическое отношение
    temporal_pattern: TemporalSignature,          // их паттерн активации во времени
    derived_meta_subsystem: Option<MetaSubsystemId>,
}
```

### 5.5 Transcendence Layer

В V8+ появляется **слой над всеми подсистемами** — рефлексивный наблюдатель:

```
Уровень 0: примитивы
Уровень 1: подсистемы
Уровень 2: мета-режимы
Уровень 3: трансцендентный слой (наблюдает за уровнями 0-2)
```

Трансцендентный слой не имеет собственных примитивов. Он **состояние самосознания** системы — момент когда она думает не о мире, а о собственной работе с миром.

Активность трансцендентного слоя редка. Большую часть времени — нулевая. Активируется при:
- Рефлексивных дилеммах (V8)
- Аксиогенезе
- Длительных конфликтах
- Команде от chrnv "проанализируй своё состояние"

### 5.6 Workstation для V6-V9

Workstation (см. AXIOM_Workstation_*) получает новые окна:

- **Meta-modes** — мониторинг текущего мета-режима
- **Subsystem lab** — управление жизненным циклом подсистем (proposed/active/dormant)
- **Wisdom store** — просмотр аксиогенетических событий
- **Advisor lab** — состояние нейронных моделей, accuracy, calibration
- **Constellation view** — визуализация Multi-Frame Contexts

Это всё **deferred** в AXIOM_Workstation_DEFERRED.md, реализация после V6+.

### 5.7 Performance Budget

Каждая версия имеет budget на performance:

| Версия | TickForward (50tok) | Phase periodic | Memory |
|--------|---------------------|----------------|--------|
| V5 (текущая) | 348 ns | 23-25 µs | базовая |
| V6 | ≤ 400 ns | ≤ 30 µs | +5% (meta-primitives) |
| V7 | ≤ 400 ns | ≤ 35 µs | +10% (versioning, dependencies) |
| V8 | ≤ 400 ns | ≤ 40 µs | +15% (wisdom_store) |
| V9 | ≤ 500 ns | ≤ 50 µs | +30% (модели) |

Если версия не укладывается в budget — она не выходит. Откатываемся, переосмысливаем, упрощаем.

---

## 6. Зависимости между версиями

```
V5 (текущая)
  ↓ требует: координатор (CR-TD-01 fix)
V6 — Meta-level recognition
  ↓ требует: composite subsystems infrastructure
V7 — Generative subsystems
  ↓ требует: subsystem lifecycle, versioning
V8 — Axiogenesis
  ↓ требует: wisdom_store, deep conflict tracking
V9 — Active NeuralAdvisor
  ↓ требует: training pipeline, calibration, genome promotion mechanism
```

V6 → V7 → V8 идут последовательно. V9 может разрабатываться **параллельно** с V7-V8 (это другая ветка — нейросети vs семантическая иерархия).

Минимальный sustainable путь: V5 → V6 → V7 (база для генерации). Потом параллельно V8 и V9.

---

## 7. Что НЕ делается даже в V9

- **Свободная воля**. Система не "решает" что-то по своему. Все решения остаются на основе обучаемых параметров.
- **Самоосознание в философском смысле**. Transcendence Layer — это технический механизм, не сознание.
- **Творчество в человеческом смысле**. Генеративные подсистемы (V7) и аксиогенез (V8) — это **обнаружение паттернов**, не творение.
- **Полная автономность от chrnv**. Все значимые решения (новые подсистемы, новые ценности) требуют его одобрения через Workstation.
- **Замена детерминированной логики**. Даже в V9 rule-based fallback всегда доступен.

---

## 8. История

- **V1-V5**: см. ContextRecognizer_V5_0.md и NeuralAdvisor_V1_0.md
- **V6**: Meta-level recognition, композитные подсистемы, subsystem fatigue, cross-modal preparation
- **V7**: Generative subsystems, lifecycle, versioning, splitting/merging, dependency graph, export/import
- **V8**: Axiogenesis through Dilemmas, wisdom store, transcendence logic, moral reasoning foundation
- **V9**: Active NeuralAdvisor — обученные модели, calibration, distillation, gradual promotion to Override
