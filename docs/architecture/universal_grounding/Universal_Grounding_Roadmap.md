# Universal Grounding Stack — Roadmap

**Статус:** Активное планирование  
**Версия:** 1.0  
**Дата:** 2026-05-27  
**Опирается на:** `Universal_Grounding_Stack.md`, `ContextRecognizer_Roadmap_V6_V9.md`, `DEFERRED.md`

---

## Текущее состояние

```
Синтаксис          ████████░░   ~80%  FrameWeaver, TextPerceptor, TransitionMatrix (plan)
Семантика          ████████░░   ~80%  SubsystemEnergy, SutraDepth, AnchorSet
Прагматика         ████░░░░░░   ~40%  MetaDetector даёт частичный сигнал
Дискурс            ███░░░░░░░   ~30%  ActivityTrace.long
Мультимодальная    ██░░░░░░░░   ~20%  architecture ready, no real Perceptors
Интуитивная физика ░░░░░░░░░░   ~0%   нет примитивов, нет perceptor
Проприоцепция      ░░░░░░░░░░   ~0%   BodyPerceptor stub не начат
Пространство       █████░░░░░   ~50%  XYZ-пространство системы, spatial_primitives нет
```

Всё что ниже 50% зависит от двух **критических компонентов** которых ещё нет:
- **Composition bonds** в FrameWeaver
- **L0 примитивный слой** в anchors

---

## Фазы реализации

---

### Фаза 0 — Фундамент (до V7)

Без этого всё остальное невозможно.

#### 0.1 Composition bonds в FrameWeaver

```rust
// В FrameAnchor или отдельном FrameCompositionStore:
pub composed_of: Option<Vec<u32>>   // sutra_id родительских Frame
pub composition_level: FrameComposition  // уже есть
```

Когда FrameWeaver кристаллизует новый Frame из co-activation — записывает из чего.

**Зависит от:** FrameWeaver V1.3 (есть)  
**Блокирует:** L0→L1 путь, кросс-модальное связывание, весь UGS  
**Объём:** средний (FrameWeaver + FrameAnchor структуры + тесты)

#### 0.2 Формализовать L0/L1 разделение в anchors

Создать структуру:
```
config/anchors/
├── perceptual/          ← L0 (новая директория)
│   ├── visual_primitives.yaml
│   ├── spatial_primitives.yaml
│   ├── causal_primitives.yaml
│   └── (body_primitives.yaml — позже)
├── writing/             ← L1 (существующее)
├── mathematics/         ← L1 (существующее)
└── ...
```

Добавить флаг `layer: L0 | L1` в AnchorSet / AnchorFile.

**Объём:** малый (yaml + AnchorFile schema extension)

#### 0.3 TransitionMatrix в ContextRecognizer

Уже решено и зафиксировано в ContextRecognizer_Roadmap §2.11.

`[[f32; 16]; 16]`, decaying, обновляется в ActivityTrace.push(). Нужен для Cascading upgrade и CompositeSubsystem V7.

**Объём:** малый-средний

---

### Фаза 1 — Языковые слои (V7 параллельно)

Достраивает синтаксис и семантику до 100%, добавляет прагматику и дискурс.

#### 1.1 Синтаксис → 100%

- TransitionMatrix (фаза 0.3) + upgrade Cascading detection
- Composition bonds (фаза 0.1) → FrameWeaver видит иерархию
- Тесты: клинопись или другой неизвестный текст распознаётся без словаря

#### 1.2 Прагматика

Новый yaml + расширение MetaDetector:

```yaml
# config/anchors/perceptual/pragmatic_primitives.yaml
- id: "speech_act_question"    # rising intonation / "?" marker
- id: "speech_act_assertion"   # declarative structure
- id: "speech_act_command"     # imperative markers
- id: "speech_act_promise"     # first-person future commitment
- id: "speech_act_challenge"   # adversarial framing
```

MetaDetector V2 (расширение §C в CR) — к существующим мета-режимам добавить прагматические.

**Зависит от:** MetaDetector (есть), L0 anchors (фаза 0.2)

#### 1.3 Дискурс

ActivityTrace.long (256 записей) уже отслеживает последовательности. Нужно:

```yaml
# config/anchors/perceptual/discourse_primitives.yaml
- id: "discourse_topic_intro"  # "во-первых", "рассмотрим"
- id: "discourse_contrast"     # "однако", "но", "despite"
- id: "discourse_consequence"  # "поэтому", "thus", "so"
- id: "discourse_reference"    # анафора, местоимения
- id: "discourse_conclusion"   # "итак", "в итоге"
```

TransitionMatrix тогда отслеживает дискурсивные переходы: topic_intro → assertion → contrast → conclusion.

---

### Фаза 2 — Визуальный Perceptor (параллельно V7/V8)

Разблокирует мультимодальную семантику и пространственное мышление.

#### 2.1 VisionPerceptor — реальная реализация

Текущий stub (`MLEngine::VisionPerceptor`) → реальный:

```
Вход: изображение (байты)
Выход: Vec<UclCommand::InjectToken> с L0 visual примитивами

Pipeline:
  1. Edge detection → stroke primitives
  2. Shape clustering → form primitives  
  3. Spatial relation extraction → spatial primitives
  4. inject_token_direct() в SUTRA
```

Возможные стратегии: ONNX-модель для feature extraction (не LLM, маленькая CNN).

#### 2.2 Spatial primitives yaml

```yaml
# config/anchors/perceptual/spatial_primitives.yaml
- id: "spatial_above"
- id: "spatial_below"
- id: "spatial_left_of"
- id: "spatial_right_of"
- id: "spatial_inside"
- id: "spatial_near"
- id: "spatial_path_connects"
- id: "spatial_boundary"
```

#### 2.3 Кросс-модальное связывание

Когда TextPerceptor и VisionPerceptor оба активируют Frame с одинаковой позицией в SUTRA — FrameWeaver связывает их.

Нужен механизм: `cross_modal_bond: Option<Vec<u32>>` в FrameAnchor — sutra_id того же Frame в другой модальности.

---

### Фаза 3 — Интуитивная физика (V8 параллельно)

#### 3.1 Causal primitives yaml

```yaml
# config/anchors/perceptual/causal_primitives.yaml
- id: "causal_precedes"       # А происходит до Б
- id: "causal_contact"        # физический контакт
- id: "causal_force_applied"  # сила приложена
- id: "causal_state_change"   # состояние изменилось
- id: "object_persists"       # объект продолжает существовать
- id: "object_disappears"     # объект исчез (нарушение persistence)
- id: "gravity_aligned"       # движение вниз
- id: "support_required"      # объект требует опоры
```

#### 3.2 CausalPerceptor

Новый perceptor: принимает последовательность событий/состояний, инжектирует causal primitives.

TransitionMatrix здесь становится буквально "каузальной статистикой": если cause_precedes всегда предшествует state_change — система понимает каузальность без объяснений.

---

### Фаза 4 — Проприоцепция (после V8)

Специфична тем что требует **непрерывного потока**, не дискретных токенов.

#### 4.1 BodyPerceptor

```
Вход: непрерывный поток сенсорных данных (Joint angles, IMU, pressure)
Адаптер: aggregate_window(N) → discretize → InjectToken

Pipeline:
  1. Sliding window агрегация (например 100мс)
  2. Change detection → inject только при значимом изменении
  3. body_primitives.yaml → позиционные/усилие примитивы
```

#### 4.2 Body primitives yaml

```yaml
# config/anchors/perceptual/body_primitives.yaml
- id: "joint_flexion"
- id: "joint_extension"
- id: "muscle_effort_high"
- id: "muscle_effort_low"
- id: "balance_stable"
- id: "balance_unstable"
- id: "contact_surface"
- id: "motion_forward"
- id: "motion_stop"
```

---

### Фаза 5 — Интеграция и обучение (V9)

К этому моменту все L0 слои работают. NeuralAdvisor V9 обучается на истории cross-modal активаций.

#### 5.1 Universal Frame

Один Frame-анкер активируется из нескольких модальностей:

```
"яблоко"     ← TextPerceptor (слово)
[визуал]     ← VisionPerceptor (красный круг)
[звук]       ← AudioPerceptor (хруст)
```

Все три → один crystallized Frame "яблоко" с `cross_modal_bonds: [text_id, visual_id, audio_id]`.

#### 5.2 NeuralAdvisor на кросс-модальных данных

`SubsystemAttributionAdvisor` обучается распознавать: "это активация соответствует чему в семантическом пространстве" — используя историю cross-modal co-activations.

---

## Критический путь

```
Фаза 0.1  Composition bonds        ← блокирует всё
  └─ Фаза 0.2  L0/L1 anchors structure
       └─ Фаза 0.3  TransitionMatrix
            └─ Фаза 1    Языковые слои (прагматика, дискурс)
                 └─ Фаза 2    VisionPerceptor + spatial
                      └─ Фаза 3    Интуитивная физика
                           └─ Фаза 4    Проприоцепция
                                └─ Фаза 5    Интеграция + NeuralAdvisor V9
```

Фазы 2–4 могут идти параллельно между собой после фазы 1.

---

## Влияние на DEFERRED

Эти задачи из DEFERRED.md приобретают новый приоритет в свете UGS:

| DEFERRED ID | Задача | Роль в UGS |
|-------------|--------|------------|
| AE-TD-08 | Full semantic connections at injection | нужен для L0 семантических bonds |
| OBS-MON-01/02 | Мониторинг activity dynamics | нужен чтобы видеть L0 паттерны |


---

## Что НЕ входит в UGS

- Изменение Token/Connection структур (HARD инварианты)
- Изменение UCL протокола
- Изменение Guardian/Genome механизмов
- Изменение DREAM Phase логики
- Любое "понимание в человеческом смысле" — UGS даёт структуру, не сознание

---

## Метрика готовности

Система считается прошедшей UGS-тест если может:

1. **Синтаксис:** взять неизвестный текст и выделить повторяющиеся единицы без словаря
2. **Семантика:** связать одинаковые концепции из разных контекстов
3. **Прагматика:** различить вопрос от утверждения без синтаксических маркеров
4. **Физика:** предсказать "объект упадёт" из истории gravity_aligned паттернов
5. **Кросс-модальность:** связать текстовое "красный" с визуальным кластером

Ни один из этих тестов не требует предварительной разметки.
