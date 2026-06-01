# Axiom — Отложенные задачи

**Версия:** 75.0
**Обновлён:** 2026-05-31

---

## ContextRecognizer (CR) — frontier issue для MAYA-токенов

---

## OBS → Engine feedback loop

### OBS-FEED-01 — Импорт паттернов из OBS в живой движок

**Идея:** сейчас OBS измеряет точность детекции но результаты никуда не идут — движок не учится от прогонов. Замкнуть цикл: паттерны и трейсы накопленные за OBS-прогон импортировать в `axiom-node`.

**Что нужно:**
- После завершения OBS-прогона: сериализовать накопленные Experience traces в файл (`obs_out/traces.bincode`)
- В axiom-node: endpoint или CLI-команда `:import-obs <path>` — загружает трейсы через существующий механизм `import traces` с GUARDIAN-валидацией (weight×0.7)
- В Lab панели: кнопка "Import to Engine" после успешного OBS-прогона

**Почему сейчас рано:** для осмысленного импорта нужны качественные данные — трейсы из синтетического корпуса с правильными `expected_subsystem`. Нужно сначала убедиться что detection accuracy достаточно высока, иначе импортируем шум.

**Когда:** после того как OBS показывает стабильную accuracy >70% на showcase-корпусе и накоплен реальный (не синтетический) корпус текстов.

---

## Долгосрочный backlog (V8–V9)

### V8 — Axiogenesis through Dilemmas

Аксиогенетические конфликты как источник новых ценностных якорей в подсистеме Values. Система, сталкиваясь с дилеммами (конфликтующие подсистемы с равным весом + высокая fatigue), генерирует новый anchor в Values-пространстве как «разрешение» конфликта.

**Зависимости:** SubsystemLifecycle (H2 ✅), реальная история дилемм (6+ месяцев работы), CR-TD-03 (Ethics composite).

**Когда:** после накопления живых данных о конфликтах между подсистемами.

---

### V9 — Active NeuralAdvisor (нейронные модели)

Заменить все 5 advisory-слотов (`depth`, `octant`, `conflict`, `subsystem`, `emergent`) обученными моделями (~1M параметров суммарно). Тренировка на накопленной `AdvisoryHistory` + `DivergenceLog`.

**Зависимости:** G1 (DivergenceLog ✅), G2 (PatternLearningResolver ✅) как baseline для сравнения, накопленная история (тысячи resolved advisories).

**Когда:** после накопленной истории + V8 или независимо по достижении достаточного объёма данных.

---

## AxialEvaluator

### AE-TD-06 — NARRATIVE_WINDOW_SIZE: правильный ли размер окна?

**Где:** `crates/axiom-runtime/src/over_domain/axial_evaluator/narrative.rs:13`

`NARRATIVE_WINDOW_SIZE = 8` — захардкожено, не калибровалось. Вопрос: оптимально ли 8 для детектирования нарративных сдвигов на реальном тексте? Слишком малое окно → ложные сдвиги; слишком большое → медленная реакция.

**Что проверить:** при production-прогоне — частота `NarrativeShift` advisory vs реальная смена темы; сравнить 8 с 4 и 12 на накопленной истории.

**Когда:** после накопления реального нарратива (тысячи инъекций).

---

## FrameWeaver

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик реактиваций на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно (в одном скане или в соседних).

Нужно для: будущего CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны), рефлекторных сокращений опыта в EXPERIENCE. Конкретный вид структуры зависит от первого потребителя.

**Когда:** при проектировании CausalWeaver или AnalogyWeaver.

> Структуру не выбирать заранее — форма данных (пары счётчиков, скользящее окно тиков, матрица вероятностей) зависит от первого потребителя. Реализовать сейчас = угадать API и переделывать.

---

## Для проекта Companion

### COMP-01 — Vital Signs окно

Окно, спроектированное для постоянного отображения на физическом светильнике-банере рядом с рабочим столом. Считываемость с расстояния: доминирующий цвет состояния заполняет фон или ключевой элемент, базовые индикаторы активности, минимум текста.

**Ключевые требования:**

- Вертикальная композиция (форм-фактор светильника-банера)
- Гибкая адаптация под пропорции экрана (горизонтальный, квадратный, вертикальный)
- Может работать на отдельном hardware-устройстве (Raspberry Pi + HDMI дисплей)
- Физическое присутствие AXIOM в комнате — не рабочий инструмент, а ambient display

**Контекст:** Обсуждалось при проектировании Workstation как "светильник-банер на столе". Намеренно вынесено в Companion — Workstation покрывает потребность через System Map.

**Когда:** Первое окно Companion. Открыть этот раздел как стартовую точку при начале проектирования Companion.

---

## Generative Subsystems — примитивы и детекторы

---

### PRIM-TD-03 — ValueGravity и AbstractionGravity

**Где:** `crates/axiom-space/` или `crates/axiom-runtime/`

Values_V1_0.md §6 описывает ValueGravity: val_beneficial/val_harmful создают притяжение/отталкивание токенов в Values-домене. Abstractions_V1_0.md §6 описывает аналогичную AbstractionGravity: более абстрактные якоря (A5 theory, A6 constructor) притягивают токены из нижних слоёв.

**Что нужно:** Расширить `apply_gravity_batch` поддержкой subsystem-specific правил притяжения (hook или стратегия).

**Зависимости:** V7-A1 (Composition bonds), TransitionMatrix (V7-B1).

**Когда:** V7-E или позже.

---

### PRIM-TD-04 — TemporalPerceptor (time-сигналы)

**Где:** `crates/axiom-agent/src/perceptors/` (новый файл)

Time_V1_0.md §5 описывает TemporalPerceptor: выделяет темпоральные маркеры из текста (до/после/одновременно/периодичность), маппит на time_*-якоря. Инжектирует в Time-домен с правильной позицией.

**Что нужно:** `TemporalPerceptor` аналогично `TextPerceptor` но специфичен для временного языка.

**Зависимости:** time/primitives.yaml (✅), V7-A2 (L0/L1 структура).

**Когда:** V7-E2 (VisionPerceptor) даст паттерн для мультимодальных перцепторов.

---

### PRIM-TD-05 — L0 уровень для абстракций (V7-A1 prerequisite)

**Где:** `config/anchors/abstractions/` и `crates/axiom-config/src/anchor.rs`

Abstractions_V1_0.md §7.2 (NOTE): A0 `abstraction_raw` — L0/L1 граница, требует V7-A1 для auto-level computation из composition bonds. Сейчас A0 захардкожен как L1.

**Что нужно:** После V7-A1 — добавить `layer: L0` флаг в AnchorFile schema и пересмотреть A0 как L0.

**Зависимости:** V7-A1 (Composition bonds в FrameWeaver), V7-A2 (L0/L1 структура).

**Когда:** V7-A2.

---

## ContextRecognizer V6 → V7

### CR-TD-03 — Ethics composite неполный

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/composite.rs` → `COMPOSITE_DEFS`

`Ethics` def содержит только `[Logic]` — Values, Dilemmas, Morality как `SubsystemId` не реализованы.

**Что нужно:** добавить `SubsystemId::Values`, `SubsystemId::Dilemmas`, `SubsystemId::Morality` ***в axiom-experience***; обновить def и YAML-якоря.

**Когда:** V7, после добавления соответствующих якорных конфигов.

---

### CR-TD-04 — ActivityTrace: интеграция с axiom-persist

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/activity_trace.rs`

Serde derives добавлены ✅ (ActivityTrace + RingBuf + SubsystemId serde feature). Остаётся подключить к AutoSaver из axiom-persist чтобы история пережила рестарт.

**Что нужно:** сохранять ActivityTrace в AutoSaver snapshot (bincode) при каждом DREAM-цикле или по таймеру; загружать при старте AxiomEngine.

**Когда:** При первой необходимости восстанавливать историю между запусками (V9 подготовка).

---

## axiom-agent

### AGENT-TD-01 — TextPerceptor: embeddings (Path A реализован)

**Где:** `crates/axiom-agent/src/perceptors/text.rs`

**Path A (done, 2026-05-23):** 2-path anchor-matching через `AnchorMatchTable` + `decomposition_table` — `detect_subsystem()` даёт 100% per-text accuracy на 8-текстовом корпусе (OBS-02). Позиция токена = взвешенный центроид совпавших якорей; FNV-1a остаётся как fallback для неизвестных слов.

**Path B (this TD):** заменить lookup-таблицы (`word_signals`/`char_signals`) на векторные embeddings. Семантически близкие тексты без якорей → соседние точки пространства.

**Что нужно:** выбрать embedding backend (ONNX runtime, candle, или внешний API); `perceive(text)` вычисляет вектор → проецирует в пространство якорей. Механизм `compute_position_from_anchors` (взвешенный центроид) остаётся. Fallback на FNV-1a при недоступности модели.

**Когда:** после накопления живых данных на OBS-02+ прогонах.

---

## FrameWeaver — Shell

### FW-TD-01 — RequestFrameDetails не реализован

**Где:** UCL OpCode `RequestFrameDetails`, `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

UCL-команда существует в протоколе, но обработчик не написан. Нужна для Workstation V2.0 (детальный просмотр участников Frame).

**Когда:** Workstation V2.0.

---

---

### Shell-TD-02 — resonance_search shell bonus

**Где:** `axiom-arbiter` → resonance_search

Shell-бонус при поиске резонансных токенов требует доступа к shell-профилям в `axiom-arbiter`. Сейчас `ShellRegistry` живёт в engine, не пробрасывается в Experience/Arbiter.

**Что нужно:** пробросить `shell_registry` в Experience или добавить метод в Arbiter для shell-proximity lookup.

**Когда:** Shell Metrics V2+.

---

## NeuralAdvisor — Emergent

### EMERGENT-TD-01 — Калибровка порогов под неоднородный корпус

**Где:** `crates/axiom-runtime/src/over_domain/neural_advisor/implementations/emergent.rs`

Пороги откалиброваны по OBS-03 (MIN_DEPTH=3000, MIN_REACTIVATIONS=15) на неоднородном корпусе. При накоплении реального текста потребуется повторная калибровка — синтетический корпус не отражает реальное распределение глубин.

**Когда:** после production-прогона с реальным текстом (10k+ инъекций).

---

### EMERGENT-TD-02 — reactivation_count: гранулярность

**Где:** `crates/axiom-experience/src/sutra_depth_store.rs` → `apply_evidence`

Сейчас `reactivation_count` инкрементируется в `apply_evidence` → считает DREAM-циклы с activity (~10-15 за 30k тиков). Слишком грубо.

Вариант: инкрементировать в `dream_activation_acc` (каждый Wake-тик где Frame активен) — более быстрорастущий сигнал, отражает реальную частоту реактивации.

**Когда:** при EMERGENT-TD-01.

---

## Observability


---

### OBS-TD-03 — delta-energy per-text нерабочий (метод оставлен)

**Где:** `crates/axiom-runtime/src/engine.rs` → `snapshot_subsystem_energies`, `context_recognizer/mod.rs` → `compute_raw_energies`

delta-energy подход для per-text subsystem detection не работает: позиции текстовых токенов (centroid якорей) и subsystem refs разнесены, sq_dist в миллионах → energy вклад ≈ 0. Методы намеренно оставлены в коде — пригодятся при embeddings, когда позиции будут семантически выровнены.

**Когда:** AGENT-TD-01 (embeddings).

---

### OBS-MON-01 — Мониторинг роста traces в production

**Наблюдение (OBS-03c):** traces плато на 32-33 из-за фиксированного корпуса (19 записей → 33 уникальных паттерна → рефлекс). Не баг — с реальным разнообразным текстом traces должны расти непрерывно.

**Что отслеживать при production-запуске:**

- traces_count после 10k, 50k, 200k тиков
- reflex_hit % — если > 95% и traces не растут → текст слишком однообразен
- опасный сигнал: traces_count стабилен неделю при активной инъекции → проверить разнообразие входящего текста

**Когда:** при первом production-прогоне с реальным текстом.

---


### OBS-MON-02 — Мониторинг tension и activity dynamics в production

**Наблюдение (OBS-03c):** tension=0 (coherence 0.998 >> порог 0.6), activity="Steady" (монотонный корпус). Оба показателя корректны для синтетического корпуса.

**Что ожидать на реальном тексте:**

- tension traces должны появляться при неоднозначных/противоречивых инъекциях
- activity signature должна показывать "Converging"/"Oscillating" при смене доминирующей темы
- если через 100k тиков tension=0 на разнообразном тексте → исследовать порог `min_coherence_f` (сейчас ~0.6, возможно завышен)
- если "Steady" постоянна при реальном тексте → исследовать разнообразие anchor matches

**Когда:** при первом production-прогоне с реальным текстом.

---

## Конфигурация якорей

---

## Cross-Modal Binding — незакрытые части V1.0

### CMB-TD-01 — Stress-driven revocation (§6 спеки)

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/cross_modal/`

Когда модальности расходятся (Text активен, Vision нет, хотя bond предполагает синхронность) — `current_stress` на CROSS_MODAL_BOND должен расти. При превышении порога за N DREAM-циклов bond помечается INHIBITED и предлагается к отзыву (chrnv подтверждает). Инфраструктура: `current_stress` на Connection уже есть (V5.0), BondTokens с флагом INHIBITED обрабатывается engine. Genome-флаг: `cross_modal.allow_revocation` (уже есть, дефолт `true`).

**Что нужно:** сканировать активные CROSS_MODAL_BOND в EXPERIENCE, проверять синхронность модальностей, инкрементировать stress, drain через DREAM. `CrossModalDetector` получает список existing bonds и делает stress-check в `drain_pending_bond_commands`.

**Когда:** после того как cross-modal bonds начнут накапливаться в реальной работе и появятся ложные связи (требует Vision pipeline).

---

### CMB-TD-03 — Workstation уведомление о новом bond

**Где:** `tools/axiom-web/src/` + `crates/axiom-node/`

Спека §4: "chrnv видит в Workstation, может отозвать". CrossModalDetector уже отправляет BondTokens через DREAM, но Workstation об этом не знает. Нужно: эмитировать WS-событие `cross_modal_bond_proposed { frame_a, frame_b, modality_a, modality_b, strength }` при drain; Lab-панель отображает список кандидатов с кнопкой Revoke.

**Что нужно:** axiom-node: hook на BondTokens с CROSS_MODAL_BOND link_type → SSE-событие. Workstation: новый раздел в Internals или Lab.

**Когда:** после стабилизации Vision pipeline (иначе нечего одобрять/отзывать).

---

### Anchor-id — word_signals() для domain/layer якорей

**Где:** `crates/axiom-agent/src/perceptors/decomposition_table.rs`

`id:` добавлены ко всем domain (`exec_*`, `shadow_*`, ...) и layer (`L1_*`, ...) якорям ✅.
`AnchorMatchTable::build()` теперь включает domain/layer позиции ✅.

Остаётся: добавить domain/layer слова в `word_signals()` в decomposition_table.rs
чтобы Path 2 (fallback) тоже матчил эти слова. Сейчас они доступны через Path 1 (AnchorSet.match_text()).

**Когда:** при необходимости расширить fallback coverage на domain/layer контекст.
