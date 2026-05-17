# Axiom — Отложенные задачи

**Версия:** 57.0
**Обновлён:** 2026-05-17

---

## FrameWeaver

### FW-TD-01 — RequestFrameDetails не реализован

**Где:** `crates/axiom-protocol/src/commands.rs`, `crates/axiom-workstation/`

`EngineCommand::RequestFrameDetails { anchor_id }` и `EngineEvent::FrameDetails(FrameDetails)` определены в axiom-protocol, но handler нигде не реализован — ни в axiom-workstation, ни в axiom-broadcasting.

`FrameDetails` содержит `last_reactivated_at_tick: Option<u64>`, которое требует дополнительного per-anchor хранилища в FrameWeaver: `reactivation_counts` хранит счётчик, но не тик последней реактивации. `crystallized_at_tick` можно взять из `token.last_event_id` (он не сбрасывается при `ReinforceFrame`).

**Что нужно:** добавить `last_reactivated_at: HashMap<u32, u64>` в FrameWeaver + реализовать handler в axiom-workstation (Benchmarks или отдельная вкладка).

**Когда:** при добавлении детальной инспекции Frame в Workstation V2.0.

---

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

## ContextRecognizer

### CR-TD-02 — Первые тики без данных AxialEvaluator

**Где:** `axial_bridge::current_active_octants_for`, `mod.rs::on_tick`

Пока AxialEvaluator не накопил оценки, `ScanningPlan` всегда использует поверхностный
регион `CreativeAffirmation`. Профили Frame создаются с `primary_octant = CreativeAffirmation`
вне зависимости от реального состояния.

**V2:** тёплый старт через `all_octants_in_store` вместо фильтрации по временному окну
на первых тиках.

**Спека:** ContextRecognizer_V5_0.md §12 CR-TD-02.

---

## NeuralAdvisor

### NA-TD-02 — AdvisoryResultStore не потребляется

**Где:** `crates/axiom-runtime/src/over_domain/neural_advisor/results.rs`

NeuralAdvisor заполняет `AdvisoryResultStore` после каждого тика, но никто не читает
результаты. Нет ни координатора, ни Workstation-отображения, ни UCL-команды
«применить рекомендацию».

**V2:** добавить в Workstation вкладку или панель с AdvisoryResult рядом с детерминированными
результатами AxialEvaluator. Coordinator может использовать октантную подсказку при
пересмотре профиля.

**V3:** PatternLearningResolver обучается на расхождениях advisor vs deterministic.

**Спека:** NeuralAdvisor_V1_0.md §12.

---

---

## axiom-agent

### AGENT-TD-01 — TextPerceptor: замена FNV-1a на embeddings

**Где:** `crates/axiom-agent/src/perceptors/text.rs`

Сейчас позиция токена в пространстве вычисляется через FNV-1a хеш от текста + якорное позиционирование (если есть совпадение в AnchorSet). Следующий шаг — заменить FNV-1a на настоящие text embeddings: тогда семантически близкие тексты будут попадать в соседние точки пространства без якорей.

Якоря из `config/anchors/` становятся обучающей выборкой для калибровки embedding-модели.

**Что нужно:** выбрать embedding backend (ONNX runtime, candle, или внешний API), интегрировать в TextPerceptor, обеспечить fallback на FNV-1a при недоступности модели.

**Когда:** после стабилизации поведения системы на живых данных (после OBS-01).

> Не делать до OBS-01 — FNV-1a baseline нужен для сравнения. Embeddings изменят геометрию пространства кардинально; без живых данных непонятно как это ляжет на физику поля.
