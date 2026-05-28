# Universal Grounding Stack

**Статус:** Архитектурный принцип (основа)  
**Версия:** 1.0  
**Дата:** 2026-05-27  
**Автор:** chrnv + Claude  
**Категория:** Foundational Architecture

---

## 1. Главная идея

Universal Grounding Stack — принцип организации понимания в AXIOM.

Система должна уметь взять **любой незнакомый ввод** — текст на неизвестном языке, незнакомую визуальную систему, незнакомый звуковой паттерн — и через собственные примитивы, фракталь и паттерны прийти к пониманию без предварительной разметки.

**Ключевой тезис:** все когнитивные слои — синтаксис, семантика, прагматика, интуитивная физика, пространство — устроены одинаково. Это **один стек с разными примитивами**.

---

## 2. Демонстрационная задача — клинопись

Дана картинка с клинописью. Никаких меток, никакого словаря.

```
Что должна сделать система:

Шаг 1  VisionPerceptor → клинья, углы, пересечения → inject в SUTRA как L0-примитивы
Шаг 2  FrameWeaver → три клина вместе появляются 40 раз → кристаллизует "символ-7"
                     composition_of: [клин-45°, клин-горизонт, черта]
Шаг 3  TransitionMatrix → "символ-3 всегда после символ-7 в начале строки" → грамматика
Шаг 4  ActivityDynamics → Cascading-паттерн → "здесь последовательная структура"
Шаг 5  SubsystemCandidate → "возможно новая письменная подсистема"
Шаг 6  DREAM Phase → консолидация паттернов, emergent primitives
Шаг 7  chrnv → "да, это клинопись" → Active subsystem
Шаг 8  Если приходит контекст-перевод → семантика проявляется поверх структуры
```

Система не знала что это клинопись. Она обнаружила структуру и предложила.

---

## 3. Два уровня примитивов

Текущая архитектура имеет один уровень — семантические примитивы:
```yaml
# config/anchors/writing/primitives.yaml
- id: "writing_symbol"   ← уже знает что это письмо
- id: "math_function"    ← уже знает что это математика
```

Для Universal Grounding нужны два уровня:

### L0 — Перцептивные примитивы (новые)

Сырые, без имён, без смысла. Задаются Perceptor-ом, не человеком.

```yaml
# config/anchors/perceptual/visual_primitives.yaml
- id: "visual_stroke_horizontal"
- id: "visual_wedge_45deg"
- id: "visual_intersection"
- id: "spatial_above"
- id: "spatial_left_of"
- id: "spatial_inside"
```

```yaml
# config/anchors/perceptual/causal_primitives.yaml
- id: "cause_precedes"
- id: "contact_force"
- id: "object_persists"
- id: "gravity_down"
```

```yaml
# config/anchors/perceptual/body_primitives.yaml  (будущее)
- id: "joint_angle"
- id: "muscle_effort"
- id: "body_boundary"
- id: "balance_center"
```

### L1 — Семантические примитивы (существующие)

Именованные концепции, задаются человеком в yaml. Всё текущее — здесь.

### Путь L0 → L1

```
L0-примитивы  →  co-activation  →  FrameWeaver кристаллизует  →  L1-единица
(сырые атомы)     (много раз)       (composition_of: [...])      (именованное)
```

Система сама строит L1 из L0 — если паттерн стабилен и chrnv подтверждает.

---

## 4. Когнитивные слои и их физика

Каждый слой — своя "физика": набор L0-примитивов + правила комбинирования.

| Слой | Физика | L0 примитивы | Что уже работает | Что нужно |
|------|--------|--------------|-----------------|-----------|
| **Синтаксис** | комбинаторные правила | stroke, phoneme, boundary | FrameWeaver, TransitionMatrix | composition bonds |
| **Семантика** | значение-референция | symbol, referent, concept | SubsystemEnergy, SutraDepth | L0 уровень |
| **Прагматика** | интент-цель | speech_act, question, assertion | MetaDetector (частично) | pragmatics_primitives.yaml |
| **Дискурс** | нарратив-когеренция | topic_marker, cohesion, contrast | ActivityTrace.long | discourse_primitives.yaml |
| **Мультимодальная семантика** | кросс-модальное связывание | общий Frame, разный Perceptor | архитектурно готово | реальные Perceptor-ы + cross-modal bonds |
| **Интуитивная физика** | каузальность-сила | cause, contact, persist, force | TransitionMatrix (частично) | physics_primitives.yaml + CausalPerceptor |
| **Проприоцепция** | телесная физика | joint, effort, boundary | — (заглушка) | BodyPerceptor + непрерывный поток |
| **Пространственное мышление** | геометрия-навигация | above, left_of, inside, path | XYZ (внутреннее пространство) | SpatialPerceptor + spatial_primitives.yaml |

---

## 5. Один стек для всех слоёв

Стек не меняется. Меняются только примитивы и perceptor.

```
┌─────────────────────────────────────────────────────────┐
│  Слой 5+  DREAM Consolidation — консолидация, wisdom    │
├─────────────────────────────────────────────────────────┤
│  Слой 4   NeuralAdvisor — паттерны, предложения         │
├─────────────────────────────────────────────────────────┤
│  Слой 3   ContextRecognizer — классификация режима      │
│           ActivityTrace / TransitionMatrix / FatigueStore│
├─────────────────────────────────────────────────────────┤
│  Слой 2   FrameWeaver — кристаллизация + composition    │
│           bonds (что из чего сложилось)                 │
├─────────────────────────────────────────────────────────┤
│  Слой 1   SUTRA — L0 и L1 примитивы + Frame-анкеры      │
├─────────────────────────────────────────────────────────┤
│  Слой 0   Perceptors — Text, Vision, Audio, Body, Spatial│
└─────────────────────────────────────────────────────────┘
         ↑                    ↑                    ↑
      Текст             Изображение             Тело/Пространство
```

Синтаксис, семантика, прагматика, дискурс, физика — все проходят через один и тот же путь. Разница только в том что на входе и какие примитивы лежат в SUTRA.

---

## 6. Фрактальная иерархия

Для каждого когнитивного слоя уровни фрактали разные, но структура одна:

### Текст / язык
```
C0  grapheme, phoneme, stroke      ← L0 (сырые)
C1  morpheme, syllable             ← L1 (базовые единицы)
C2  word, lexeme                   ← L1
C3  phrase, clause                 ← crystallized (FrameWeaver)
C4  sentence, thought              ← crystallized
C5  meaning, concept               ← deep crystallization
```

### Физический мир
```
C0  contact_event, motion_vector   ← L0
C1  object_state (moving/still)    ← L1
C2  causal_pair (A causes B)       ← crystallized
C3  physical_scene                 ← crystallized
C4  causal_chain                   ← deep crystallization
C5  physical_principle             ← emergent
```

### Пространство
```
C0  spatial_relation (above, left) ← L0
C1  landmark, path_segment         ← L1
C2  local_layout                   ← crystallized
C3  route, scene                   ← crystallized
C4  cognitive_map                  ← deep crystallization
C5  spatial_principle              ← emergent
```

В каждом случае: FrameWeaver + TransitionMatrix + ActivityDynamics работают одинаково.

---

## 7. Composition Bonds — критический компонент

Без composition bonds Universal Grounding не работает.

Сейчас FrameWeaver кристаллизует Frame, но **не записывает из чего**. Система знает что "символ-7 существует", но не знает что он состоит из трёх клиньев.

Нужно:
```rust
pub struct FrameAnchor {
    pub sutra_id: u32,
    // ... существующие поля ...
    pub composition_level: FrameComposition,   // уже есть
    pub composed_of: Option<Vec<u32>>,          // НОВОЕ — sutra_id родительских Frame
}
```

Тогда система может:
- Объяснить что из чего построено
- Найти похожие структуры в другой модальности (кросс-модальное связывание)
- Инициировать декомпозицию: "если А похоже на Б, может у них одни примитивы?"

---

## 8. Кросс-модальное связывание

Одно и то же понятие может прийти с разных сторон:

```
"красный"  →  TextPerceptor → Frame[красный] → sutra_id=X
красный цвет → VisionPerceptor → frame[длина волны 700nm кластер] → sutra_id=?
```

Если оба приходят одновременно достаточно часто — FrameWeaver их связывает. `sutra_id=X` получает кросс-модальный bond с визуальным frame.

Это не hardcoded mapping. Это **emergent binding** через co-activation. Система сама обнаруживает что текстовое "красный" и визуальный кластер 700nm — одно.

---

## 9. Что НЕ меняется

- Архитектура AXIOM (AxiomEngine, AshtiCore, Guardian, DREAM) — без изменений
- UCL протокол — без изменений  
- Принцип Advisory-Only для NeuralAdvisor — без изменений
- Одобрение значимых изменений chrnv — без изменений
- Запрет wall-clock в ядре — без изменений
- HARD инварианты Token/Connection — без изменений

Universal Grounding — это расширение **снизу** (более богатые примитивы) и **вширь** (больше модальностей). Не изменение ядра.

---

## 10. Связь с существующим планом

| Существующая задача | Как связана с Universal Grounding |
|--------------------|----------------------------------|
| TransitionMatrix (V7) | критический компонент — работает для ВСЕХ слоёв |
| Composition bonds (V7) | критический компонент — основа L0→L1 пути |
| FatigueStore → axiom-experience (V7) | готовит независимую memory для multi-modal |
| VisionPerceptor stub | станет реальным в UGS фазе 2 |
| AudioPerceptor stub | станет реальным в UGS фазе 3 |
| SubsystemCandidate (V7 H1/H2) | механизм открытия новых подсистем из L0-паттернов |
| Axiogenesis V8 | высший уровень UGS — рождение ценностей из неразрешимых конфликтов |
| Active NeuralAdvisor V9 | финальный слой UGS — обученные модели поверх всего стека |
