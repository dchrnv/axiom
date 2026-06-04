# Axiom — Отложенные задачи

**Версия:** 80.0
**Обновлён:** 2026-06-04

---

## Sensorium

### SEN-TD-01 — Полное поглощение TickSnapshot → Sensorium V2.0

**Проблема:** Сейчас два независимых пути "пульса" наружу:
1. `TickSnapshot` (старый) — в `engine.rs`, через `BroadcastHandle`. Используется Workstation, OBS, axiom-tray.
2. `Sensorium.current_state` (новый) — собирается, хранится внутри, наружу не идёт.

Спека §7: "уровень 0 = переоформленный TickSnapshot — не создавать заново." В финале `Sensorium level 0` заменяет TickSnapshot.

**Решение (V2.0):**
- Перенести поля из `BroadcastSnapshot` в `SensoriumState level 0`.
- `SensoriumAdapter → Workstation` переводит `SensoriumState` → WS-формат.
- OBS-runner и axiom-tray переключаются на новый адаптер. TickSnapshot удаляется.

**Почему сейчас рано:** Workstation, OBS, tray плотно привязаны к `BroadcastSnapshot`. Переключение = согласованный обновление React SPA + axiom-node + axiom-observe + axiom-tray.

**Когда:** при добавлении первого Sensorium-адаптера для Workstation.

---

## Семантическая гравитация к якорям (Anchor Gravity)

### GRAVITY-TD-01 — Токены притягиваются к (0,0,0), а должны к позиции семантического якоря

**Проблема:**  
Сейчас `apply_gravity_batch` тянет все токены к `(0, 0, 0)` — константы `ANCHOR_X/Y/Z = 0`
в `crates/axiom-space/src/lib.rs:28-31`. Это "базовая" гравитация домена.

Но в нескольких спеках прямо написано, что ценности и абстракции должны работать как
**гравитационные аттракторы** — то есть токены тянутся не к центру домена, а к позиции
конкретного семантического якоря:

- **Values_V1_0.md §1:** "Ценности — это гравитационные аттракторы оценки. Токен 'помочь
  раненому' не содержит val_beneficial как компонент — он **гравитационно притягивается** к нему."
- **Values_V1_0.md §6:** "Может потребоваться специальный pass в apply_gravity_batch для
  ценностных якорей с бо́льшим radius_of_influence."
- **Abstractions_V1_0.md §2:** "Каждый Frame в SUTRA может гравитировать к одному из
  абстракционных якорей в зависимости от своей глубины."
- **Abstractions_V1_0.md §7:** "При apply_gravity_batch — дополнительный pass для
  абстракционных якорей: Frame с composition_level=C3 притягивается к abstraction_category."

**Текущее состояние:**  
PRIM-TD-03 ✅ реализован (2026-06-04): `crates/axiom-runtime/src/subsystem_gravity.rs`.
val_beneficial pull(0.20) / val_harmful push(0.20) / abstraction_theory+constructor pull(0.08, radius=8000).
Формула: нормализованный вектор × BASE_FORCE(16) × factor_256/256. Interval=500 тиков.

Но за PRIM-TD-03 стоит более широкий принцип, который пока не реализован:
любой STATE_LOCKED якорь с достаточной массой является потенциальным аттрактором.

**Якоря STATE_LOCKED → позиции фиксированы навечно** — кешируются в SubsystemGravityRule при boot.

**Ключевые позиции (сверено с yaml):**
```
val_beneficial       [8000, 12000, 13000]   — притяжение
val_harmful          [3000,  1000, 11000]   — отталкивание
abstraction_theory   [13000, 10000, 14000]  — C5/A5
abstraction_constructor [14000, 12000, 15000] — C5+/A6
```

**Когда:** после накопления реальных данных о том как якоря взаимодействуют.

---

## FrameWeaver

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно.

Нужно для: CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны).

**Когда:** при проектировании CausalWeaver или AnalogyWeaver. Структуру не выбирать заранее — форма данных зависит от первого потребителя.

---

## OBS → Engine feedback loop

### OBS-FEED-01 — Импорт паттернов из OBS в живой движок

**Идея:** замкнуть цикл — трейсы из OBS-прогона импортировать в `axiom-node`.

**Что нужно:** сериализовать Experience traces после OBS → endpoint `:import-obs <path>` → GUARDIAN-валидация (weight×0.7) → Lab-панель: кнопка "Import to Engine".

**Когда:** после стабильной accuracy >70% на showcase-корпусе и накопления реального (не синтетического) корпуса.

---

## Долгосрочный backlog (V8–V9)

### V8 — Axiogenesis through Dilemmas

Аксиогенетические конфликты → новые ценностные якоря в Values. Требует реальной истории дилемм.

**Зависимости:** SubsystemLifecycle ✅, реальная история дилемм (6+ месяцев), Ethics composite ✅.

**Когда:** после накопления живых данных.

---

### V9 — Active NeuralAdvisor (нейронные модели)

Заменить все 5 advisory-слотов обученными моделями (~1M параметров). Тренировка на `AdvisoryHistory` + `DivergenceLog`.

**Зависимости:** G1 ✅, G2 ✅, тысячи resolved advisories.

**Когда:** после накопленной истории.

---

## AxialEvaluator

### AE-TD-06 — NARRATIVE_WINDOW_SIZE: правильный ли размер окна?

**Где:** `crates/axiom-runtime/src/over_domain/axial_evaluator/narrative.rs:13`

`NARRATIVE_WINDOW_SIZE = 8` — не калибровалось. Слишком малое → ложные сдвиги; слишком большое → медленная реакция.

**Когда:** после накопления реального нарратива (тысячи инъекций).

---

## Для проекта Companion

### COMP-01 — Vital Signs окно

Ambient display — постоянное отображение состояния системы на физическом светильнике-банере. Вертикальная композиция, Raspberry Pi + HDMI.

**Когда:** первое окно Companion.

---

## axiom-agent

### AGENT-TD-01 — TextPerceptor: embeddings (Path A реализован)

**Где:** `crates/axiom-agent/src/perceptors/text.rs`

Path A (100% accuracy) ✅. Path B: заменить lookup-таблицы на векторные embeddings (ONNX/candle).

**Когда:** после накопления живых данных на OBS-02+ прогонах.

---

## Shell

### Shell-TD-02 — resonance_search shell bonus

**Где:** `axiom-arbiter` → `resonance_search`

Shell-бонус требует `ShellRegistry` в Experience/Arbiter. Сейчас живёт только в engine.

**Когда:** Shell Metrics V2+.

---

## NeuralAdvisor — Emergent

### EMERGENT-TD-01 — Калибровка порогов под неоднородный корпус

**Где:** `crates/axiom-runtime/src/over_domain/neural_advisor/implementations/emergent.rs`

Пороги откалиброваны по синтетическому корпусу. При реальном тексте потребуется пересмотр.

**Когда:** после production-прогона (10k+ инъекций).

---

## Observability

### OBS-TD-03 — delta-energy per-text нерабочий (метод оставлен)

**Где:** `crates/axiom-runtime/src/engine.rs` → `snapshot_subsystem_energies`

Методы оставлены намеренно — пригодятся при embeddings (AGENT-TD-01), когда позиции будут семантически выровнены.

**Когда:** AGENT-TD-01 (embeddings).

---

### OBS-MON-01 — Мониторинг роста traces в production

Traces плато на синтетическом корпусе — нормально. Отслеживать: traces_count после 10k/50k/200k тиков, reflex_hit%.

**Когда:** первый production-прогон с реальным текстом.

---

### OBS-MON-02 — Мониторинг tension и activity dynamics в production

tension=0 на синтетическом корпусе — нормально. На реальном тексте: tension traces, "Converging"/"Oscillating".

**Когда:** первый production-прогон с реальным текстом.

---

## Cross-Modal Binding

### CMB-TD-01 — Stress-driven revocation

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/cross_modal/`

Когда модальности расходятся — `current_stress` на CROSS_MODAL_BOND должен расти → INHIBITED → предложение к отзыву.

Инфраструктура: `current_stress` на Connection ✅, `cross_modal.allow_revocation` в genome ✅.

**Когда:** после накопления cross-modal bonds в реальной работе (требует Vision pipeline + реальные данные).
