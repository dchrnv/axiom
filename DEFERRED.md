# Axiom — Отложенные задачи

**Версия:** 69.0
**Обновлён:** 2026-05-28

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

### PRIM-TD-01 — MoralSignalDetector

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/` (новый файл)

Morality_V1_0.md §4 описывает детектор моральных сигналов: активация moral-якорей → расчёт moral_intensity (сумма весов) и dominant_foundation (якорь с максимальным весом). Подаёт сигнал в DilemmaDetector при конфликте оснований.

**Что нужно:** `MoralSignalDetector::detect(matches: &[AnchorMatch]) → Option<MoralSignal>`; `MoralSignal { intensity: f32, dominant: AnchorId, secondary: Option<AnchorId> }`.

**Зависимости:** DilemmaDetector (будущий).

**Когда:** V7 или при реализации DilemmaDetector.

---

### PRIM-TD-02 — DilemmaDetector V1.0

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/` (новый файл)

Dilemmas_V1_0.md §3 описывает детектор: сравнивает энергии конфликтующих подсистем, проверяет `is_natural_tension()` из SubsystemDependencies, классифицирует тип дилеммы (I–V) и создаёт DilemmaRecord для DilemmaStore.

**Что нужно:** `DilemmaDetector::on_tick(energies: &HashMap<SubsystemId, u8>, deps: &SubsystemDependencies) → Vec<DilemmaRecord>`.

**Зависимости:** DilemmaStore V1.1 (✅), SubsystemDependencies (✅), MoralSignalDetector (PRIM-TD-01).

**Когда:** V7-C или позже (требует TransitionMatrix для правильного определения «конфликта»).

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

### CR-TD-04 — ActivityTrace не сериализуется

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/activity_trace.rs`

`ActivityTrace` (`VecDeque` буферы) не имеет serde derive. V9 NeuralAdvisor требует observation sequence как тренировочные данные. При рестарте история активности теряется.

**Что нужно:** `#[derive(serde::Serialize, serde::Deserialize)]` на `ActivityTrace`/`RingBuf`; интеграция с axiom-persist.

**Когда:** V7, одновременно с переносом FatigueStore.

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

### Shell-TD-01 — ShellProximity + crystallization_rules архитектура

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs` → `evaluate_crystallization_rules`

`ShellProximity(threshold)` — opt-in правило. `crystallization_rules: vec![]` по умолчанию намеренно. Проблема: при добавлении любого правила в список `evaluate_crystallization_rules` перестаёт фолбэчить на `stability_threshold` → все кандидаты получают `Defer` → Frames=0.

**Что нужно:** добавлять `ShellProximity` в паре с явным `StabilityReached`-правилом, **или** рефакторить `evaluate_crystallization_rules` чтобы stability_threshold работал как minimum-baseline независимо от списка.

**Когда:** при следующей работе с кристаллизацией.

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

### OBS-TD-01 — Нет прогресса во время прогона

**Где:** `crates/axiom-observe/src/runner.rs` → `run()`

ObsRunner не печатает прогресс — только старт и финиш. При длинных прогонах (1M тиков = ~90+ минут) невозможно понять на каком этапе процесс и не завис ли он.

**Что нужно:** периодический `eprintln!("[observe] tick {tick}/{total} ({pct:.0}%)")` каждые N тиков (например каждые 50K или каждые 10% от ticks_total). Опционально: elapsed time и оценка оставшегося.

**Когда:** **первым делом** — до следующего большого прогона. Текущий 1M-тиковый прогон занял ~2+ часа вслепую.

---

### OBS-TD-04 — Оптимизация длинных прогонов

**Где:** `crates/axiom-observe/src/runner.rs`, `crates/axiom-observe/src/report.rs`

При 1M тиков прогон занял ~90 минут, RAM вырос до ~108MB — все snapshots и events накапливаются в памяти и пишутся в конце одним блоком.

**Что нужно:**
- Стриминг событий/снапшотов в файл по мере накопления вместо Vec в памяти (JSONL или bincode append)
- Параметр `max_injection_count` в corpus.yaml — ограничитель на суммарное число инъекций
- Возможно: `--sample-rate` флаг чтобы записывать каждый N-й снапшот

**Когда:** перед следующим большим прогоном.

---

### OBS-TD-02 — avg_shell_similarity всегда 0

**Где:** `crates/axiom-observe/src/runner.rs` → `capture_snapshot`

Кандидаты FrameWeaver кристаллизуются за ~60 тиков (stability=3 × scan_interval=20). При `snapshot_every=500` к моменту снапшота активных кандидатов нет → `avg_candidate_shell_similarity()` = 0.

**Варианты:** per-crystallization event capture; rolling avg за последние N кристаллизаций; уменьшить snapshot_every для shell-наблюдений.

**Когда:** при следующей работе с shell metrics.

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

### DEV-PANEL-01 — Dev/Lab панель в Workstation V2

**Где:** `tools/axiom-web/src/` (новая вкладка в Workstation V2 UI)

Вкладка "Lab" / "Dev" для запуска инструментов разработки прямо из браузера:

- **Кнопки запуска:** OBS (с выбором корпуса), бенчи (hot_path / over_domain / stress), cargo test
- **Монитор прогресса:** реальный stdout процесса через WebSocket → `<pre>` в браузере; терминация по кнопке
- **Автосохранение:** по завершении прогона axiom-node сохраняет артефакты в нужные файлы (`showcase/SHOWCASE.md`, `showcase/bench_out/*.txt`, `showcase/obs_out/report.md`) и обновляет их

**Архитектура (сервер):**
- `axiom-node` добавляет endpoint `/api/dev/run` (POST с `{ cmd: "obs" | "bench_hot" | "bench_od" | "test" }`)
- `tokio::process::Command` спавнит нужный бинарь/cargo-команду
- stdout/stderr стримятся через WebSocket (новый `/ws/dev/log` channel)
- результаты пишутся в файлы напрямую из процесса (нынешний showcase.sh approach)

**Когда:** после OBS-TD-01 (прогресс), как отдельная фаза Workstation V2.

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

### AE-TD-08 — Полноценное подключение якорей при инъекции текста

**Где:** `crates/axiom-agent/src/perceptors/text.rs`, `crates/axiom-runtime/src/over_domain/axial_evaluator/mod.rs`

Текущий фикс AE-TD-07 (позиционный fallback когда `participants < 2`) — прагматически верен, но архитектурно неполон. Правильное решение: при инъекции текста TextPerceptor должен не только вычислять позицию, но и создавать рёбра связей (ConnectToken команды) от нового токена к matched anchor-токенам. Тогда participants будут непустыми, и стандартный расчёт метрик (entropy/density/will) заработает по-настоящему.

**Почему сейчас fallback корректен:** позиция токена вычислена из семантических якорей (TextPerceptor) и несёт смысловой контекст. Октантное распределение в OBS-03c подтвердило корректность.

**Почему нужен полный фикс:** с реальными связями появится ненулевой entropy/density/will и более точные ConflictDiagnosis advisories.

**Когда:** после базовой стабилизации; приоритет средний — fallback работает достаточно хорошо.

---

### Anchor-id — Domain/Layer якоря без id

**Где:** `config/anchors/domains/D*.yaml`, `config/anchors/layers/L*.yaml`

Domain и Layer якоря загружаются через `parse_domain` / `parse_layer` и матчатся через `match_text()`, но поле `id:` пустое (`#[serde(default)]`). `AnchorMatchTable` ищет по id → не видит их.

**Что нужно:** добавить `id:` с осмысленным префиксом (`exec_*`, `L1_*` и т.п.); расширить `subsystem_from_anchor_id()` или добавить отдельный маппинг для domain/layer контекстов.

**Когда:** при расширении AnchorMatchTable coverage.
