# Axiom — Отложенные задачи

**Версия:** 78.0
**Обновлён:** 2026-06-03

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

## FrameWeaver

### FW-TD-01 — RequestFrameDetails не реализован ⚡ ГОТОВО РЕАЛИЗОВАТЬ

**Где:** UCL OpCode `RequestFrameDetails` (opcode 4002?), `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

UCL-команда существует в протоколе, обработчик не написан. Нужна для детального просмотра участников Frame в Workstation.

**Что нужно:** handler в engine.rs → берёт anchor_id из payload → возвращает список участников из FrameWeaver.candidates или EXPERIENCE-state.

**Когда:** ✅ Условие выполнено (Workstation V2.0 готов). Можно реализовать.

---

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно.

Нужно для: CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны).

**Когда:** при проектировании CausalWeaver или AnalogyWeaver. Структуру не выбирать заранее — форма данных зависит от первого потребителя.

---

## Generative Subsystems — примитивы и детекторы

### PRIM-TD-03 — ValueGravity и AbstractionGravity ⚡ ГОТОВО РЕАЛИЗОВАТЬ

**Где:** `crates/axiom-space/` или `crates/axiom-runtime/`

Values_V1_0.md §6: `val_beneficial/val_harmful` создают притяжение/отталкивание в Values-домене. Abstractions_V1_0.md §6: более абстрактные якоря (A5 theory, A6 constructor) притягивают токены из нижних слоёв.

**Что нужно:** расширить `apply_gravity_batch` subsystem-specific правилами притяжения (hook или стратегия).

**Зависимости:** V7-A1 (Composition bonds) ✅, TransitionMatrix ✅. Все закрыты.

**Когда:** ✅ V7-E2 готов. Можно реализовать. Требует правки физики — оцени объём перед стартом.

---

### PRIM-TD-04 — TemporalPerceptor ⚡ ГОТОВО РЕАЛИЗОВАТЬ

**Где:** `crates/axiom-agent/src/perceptors/` (новый файл, аналог `vision_l0.rs`)

Time_V1_0.md §5: выделяет темпоральные маркеры из текста (до/после/одновременно/периодичность), маппит на `time_*`-якоря. Инжектирует в Time-домен с правильной позицией.

**Что нужно:** `TemporalPerceptor` аналогично `L0VisionPerceptor`:
- regex/keyword detector для темпоральных маркеров
- маппинг на time_before / time_after / time_simultaneous / time_periodic / time_duration / time_horizon
- `vision_anchor_stable_id` паттерн → `temporal_anchor_stable_id` (новый диапазон, бит 28)
- InjectToken в SUTRA с stable_id

**Зависимости:** `time/primitives.yaml` ✅ (7 якорей), V7-A2 (L0/L1 структура) ✅, паттерн `L0VisionPerceptor` ✅.

**Когда:** ✅ Все зависимости закрыты. Можно реализовать сейчас.

---

### PRIM-TD-05 — L0 уровень для абстракций ⚡ ГОТОВО РЕАЛИЗОВАТЬ

**Где:** `config/anchors/abstractions/primitives.yaml` + `crates/axiom-config/src/anchor.rs`

Abstractions_V1_0.md §7.2: A0 `abstraction_raw` — L0/L1 граница. Сейчас A0 захардкожен как L1.

**Что нужно:** добавить `layer: L0` в AnchorFile schema (если ещё нет) и пересмотреть A0 как L0.

**Зависимости:** V7-A1 (Composition bonds) ✅, V7-A2 (L0/L1 структура) ✅.

**Когда:** ✅ Все зависимости закрыты. Небольшое изменение конфига.

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
