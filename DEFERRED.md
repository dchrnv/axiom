# Axiom — Отложенные задачи

**Версия:** 62.0
**Обновлён:** 2026-05-23

---

## CognitiveProfile

### PROFILE-01 — Иерархия важности октантов при принятии решений ✅

**Идея:** ввести `CognitiveProfile` — вектор `octant_weights: [f32; 8]`, загружаемый из YAML, который описывает относительную «ценность» каждого октанта для конкретного агента. Профиль влияет на то, как OverDomainArbiter расставляет приоритеты при принятии решений.

**Где применять:**
- `OverDomainArbiter` — масштабировать `confidence` advisory-а на `octant_weights[advisory.octant]` перед сравнением с `min_confidence` в TrustConfig
- Промоция emergent-кандидатов — приоритизировать кандидатов из «важных» для профиля октантов
- Опционально: TrustConfig переопределяется per-profile (аналитический агент повышает AutoApply для AnalyticGrounding)

**Примеры профилей:**
- `analytic` — высокий вес ApolloGrounding (X+, Y-, Z+), LogicAnalytic
- `creative` — высокий вес DionysusSynthetic (X-, Y+, Z+), CreativeAffirmation
- `balanced` — равномерный вес (текущее поведение по умолчанию)

**Архитектурная заметка:** TrustConfig уже является системой весов по (Source, AdvisoryType). CognitiveProfile добавляет второй ортогональный слой — «куда смотреть» vs «кому доверять». Важно сохранить разделение: TrustConfig = политика доверия источникам, Profile = когнитивная ориентация агента.

**Что нужно:**
1. `CognitiveProfile` struct в `axiom-experience` или `axiom-config`
2. `config/profiles/*.yaml` — предустановленные профили
3. `OverDomainArbiter::set_profile(profile)` — применение при boot или hot-reload
4. Модификация `evaluate_advisory()`: `effective_confidence = confidence * profile.octant_weights[octant]`

**Когда:** после OBS-03 (накопление данных о реальном распределении октантов), V2 OverDomainArbiter.

---

## OverDomainArbiter

### ARB-TD-01 — TrustConfig задаётся в коде, не в конфиге ✅

**Где:** `crates/axiom-runtime/src/over_domain/arbiter/trust.rs` → `TrustConfig::default_v1()`

Пороги и режимы (AutoApply/RequireConfirmation) захардкожены. Нет возможности менять без перекомпиляции.

**Что нужно:** вынести в `config/genome.yaml` секцию `[arbiter.trust]`; `min_confidence` калибруется автоматически по `ArbiterLog` (confirmed / confirmed+rejected).

**Когда:** V2, после накопления данных от OBS-01.

---

### ARB-TD-02 — PendingQueue не протухает ✅

**Где:** `crates/axiom-runtime/src/over_domain/arbiter/mod.rs` → `pending: VecDeque<PendingAdvisory>`

Непринятые рекомендации копятся бесконечно. При долгом бездействии оператора очередь может вырасти неограниченно.

**Что нужно:** добавить `expires_at_event: Option<u64>` в `PendingAdvisory`; TTL ~1000 event_id; при истечении → `ArbiterOutcome::Expired` + `on_feedback(Expired)`.

**Когда:** V2, после наблюдения реального поведения очереди в OBS-01.

---

### ARB-TD-03 — AutoApply только для DepthHint ✅

**Где:** `crates/axiom-runtime/src/over_domain/arbiter/mod.rs` → `execute()`

`OctantCorrection` не может применяться автономно — требует записи в `AxialStore`, что сейчас невозможно без пересчёта позиции (AxialEvaluator пересчитает на следующем тике и перетрёт).

**Что нужно:** `AxialStore::override_octant(sutra_id, octant)` с пометкой "advisory override"; AxialEvaluator уважает флаг.

**Когда:** AxialEvaluator V3 — спроектировано в `AxialEvaluator_V3_0.md` §3.

---

## FrameWeaver

### FW-TD-02 — Per-pair co-activation не отслеживается

**Где:** `crates/axiom-runtime/src/over_domain/weavers/frame.rs`

Текущая структура: `reactivation_counts: HashMap<u32, u32>` — глобальный счётчик реактиваций на Frame-анкер. Нет информации о том, какие Frame-ы активировались совместно (в одном скане или в соседних).

Нужно для: будущего CausalWeaver (причинные связи между Frame), AnalogyWeaver (похожие паттерны), рефлекторных сокращений опыта в EXPERIENCE. Конкретный вид структуры зависит от первого потребителя.

**Когда:** при проектировании CausalWeaver или AnalogyWeaver.

> Структуру не выбирать заранее — форма данных (пары счётчиков, скользящее окно тиков, матрица вероятностей) зависит от первого потребителя. Реализовать сейчас = угадать API и переделывать.

---

## Workstation — расширения для V2.0

_Идеи, не реализованные в V1.0 по объёму или зависимостям. Не блокируют V1.0._

### WS-V2-01 — Long-term история Conversation

**Где:** `crates/axiom-workstation/src/app.rs` → `ConversationState.messages`

При рестарте Workstation лента чата пуста — история нигде не хранится. V1.0 этого не требует, но оператор теряет контекст предыдущих сессий.

**Что нужно:** Хранить историю в EXPERIENCE как часть нарратива Engine: каждый отправленный текст записывается в отдельный лог (файл или Engine API), загружается при старте.

**Когда:** V2.0 или при появлении narrative-log API в Engine.

---

### WS-V2-02 — Pause / Resume импорта

**Где:** `crates/axiom-workstation/src/ui/files.rs`, `crates/axiom-protocol/src/commands.rs`

Реализован Cancel (через `EngineCommand::CancelAdapter`). Pause/Resume нет — требует поддержки в адаптерах и соответствующих команд в протоколе.

**Что нужно:** `EngineCommand::PauseAdapter { run_id: String }` / `ResumeAdapter`, статус `AdapterStatus::Paused` в протоколе, кнопка Pause рядом с Cancel в `files.rs`.

**Когда:** При необходимости паузируемого импорта больших файлов.

---

### WS-V2-03 — Конструктор кастомных бенчмарков

**Где:** `crates/axiom-workstation/src/ui/benchmarks.rs`

V1.0 показывает историю предустановленных бенчмарков (6 вариантов из `BenchSpec`). Нет возможности собрать кастомный сценарий: нагрузка, длительность, выбор метрик.

**Что нужно:** Форма в Benchmarks tab — `BenchSpec` builder: тип нагрузки (dropdown), iterations, duration, domain selection, сохранение preset-ов локально.

**Когда:** V2.0 или при активном использовании бенчмарков.

---

### WS-V2-04 — Полный compatibility matrix Engine ↔ Workstation

**Где:** `crates/axiom-workstation/src/connection.rs`, `crates/axiom-protocol/src/lib.rs`

V1.0: проверяется только `major`-байт `PROTOCOL_VERSION`. Если Engine обновлён на minor — Workstation подключается, но поля протокола могут расходиться.

**Что нужно:** Матрица совместимости major.minor: graceful degradation для смежных minor-версий (игнорировать unknown enum variants), UI-индикатор "version mismatch, limited mode".

**Когда:** V2.0 / перед публичным релизом.

---

### WS-V2-05 — Сетевой режим (remote Engine)

**Где:** `crates/axiom-workstation/src/settings.rs`, `crates/axiom-workstation/src/connection.rs`

V1.0 рассчитан на локальный Engine (`127.0.0.1:9876`). Точки расширения уже помечены в архитектуре V1.0 — address конфигурируем, transport абстрагирован через WebSocket.

**Что нужно:** TLS поверх WS (`wss://`), аутентификация (token в Hello), обработка network timeouts, reconnect при network partition.

**Смотри:** `AXIOM_Workstation_02_Architecture.md` раздел 9 (Network Mode).

**Когда:** V2.0 / `axiom-node` (Engine на отдельном железе).

---

### WS-V2-06 — Sync между Workstation и Companion

Когда оба клиента (Workstation + Companion) подключены к одному Engine одновременно, нужна координация: не дублировать force-sleep запросы, видеть что другой клиент уже подключён. Синхронизация только через Engine, не напрямую.

**Когда:** Когда Companion будет реализован.

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

## ContextRecognizer V6 → V7

### CR-TD-01 — FatigueStore → axiom-experience

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/subsystem_fatigue.rs`

`FatigueStore` живёт в ContextRecognizer (V6). Жизненный цикл усталости не зависит от CR и должен быть в `axiom-experience`, аналогично `SutraDepthStore`. Это позволит V9 NeuralAdvisor читать fatigue напрямую.

**Что нужно:** перенести `SubsystemFatigue`, `FatigueStore` в `axiom-experience`; обновить re-exports в `axiom-runtime`.

**Когда:** V7.

---

### CR-TD-02 — TransitionGraph для directed Cascading

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/activity_trace.rs` → `compute_cascade_score`

V6 `cascade_score` = sequence diversity (unique subsystems per run / n). Не отличает directed propagation от случайного чередования. `Cascading` может ошибочно срабатывать на случайные последовательности.

**Что нужно:** `TransitionGraph { edges: HashMap<(SubsystemId, SubsystemId), u32> }` поверх `ActivityTrace`; edge frequency → directed chain detection; заменить `cascade_score` на граф-метрику.

**Когда:** V7.

---

### CR-TD-03 — Ethics composite неполный

**Где:** `crates/axiom-runtime/src/over_domain/context_recognizer/composite.rs` → `COMPOSITE_DEFS`

`Ethics` def содержит только `[Logic]` — Values, Dilemmas, Morality как `SubsystemId` не реализованы.

**Что нужно:** добавить `SubsystemId::Values`, `SubsystemId::Dilemmas`, `SubsystemId::Morality` в axiom-experience; обновить def и YAML-якоря.

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

Текущие пороги (MIN_DEPTH=1000, MIN_REACTIVATIONS=5) откалиброваны по OBS-02 однородного корпуса → 312/312 frames стали кандидатами (все проходят). Discriminative detection невозможна при однородном опыте.

**Что нужно:** неоднородный корпус (часть текстов 2-3 инжекции, часть 100+), после чего повторная калибровка так чтобы только "глубоко обработанные" Frame проходили порог.

**Когда:** при следующем OBS-прогоне с неоднородным корпусом.

---

### EMERGENT-TD-02 — reactivation_count: гранулярность

**Где:** `crates/axiom-experience/src/sutra_depth_store.rs` → `apply_evidence`

Сейчас `reactivation_count` инкрементируется в `apply_evidence` → считает DREAM-циклы с activity (~10-15 за 30k тиков). Слишком грубо.

Вариант: инкрементировать в `dream_activation_acc` (каждый Wake-тик где Frame активен) — более быстрорастущий сигнал, отражает реальную частоту реактивации.

**Когда:** при EMERGENT-TD-01.

---

## Observability

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

## Конфигурация якорей

### Anchor-id — Domain/Layer якоря без id

**Где:** `config/anchors/domains/D*.yaml`, `config/anchors/layers/L*.yaml`

Domain и Layer якоря загружаются через `parse_domain` / `parse_layer` и матчатся через `match_text()`, но поле `id:` пустое (`#[serde(default)]`). `AnchorMatchTable` ищет по id → не видит их.

**Что нужно:** добавить `id:` с осмысленным префиксом (`exec_*`, `L1_*` и т.п.); расширить `subsystem_from_anchor_id()` или добавить отдельный маппинг для domain/layer контекстов.

**Когда:** при расширении AnchorMatchTable coverage.
