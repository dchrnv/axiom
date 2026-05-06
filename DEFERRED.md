# Axiom — Отложенные задачи

**Версия:** 39.0
**Обновлён:** 2026-05-06

---

## Workstation V1.0 — отложено из Stage 2

### ~~BRD-TD-01 — DomainActivity throttle enforcement~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D6. `should_send()` проверяет `domain_activity_threshold`: события с `recent_activity < threshold` отфильтровываются.

---

### ~~BRD-TD-03 — Snapshot resync при RecvError::Lagged~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D6. При `RecvError::Lagged` сервер читает `snapshot_cache` и отправляет клиенту полный снапшот для resync.

---

### BRD-TD-05 — Полнота полей build_system_snapshot()

**Где:** `crates/axiom-broadcasting/src/snapshot.rs`

Многие поля `SystemSnapshot` заполнены нулями/пустыми значениями: `capacity`, `temperature_decay`, `temperature_avg`, `recent_activity`, `layer_activations`, `guardian_stats`, `last_dream_report`, `adapter_progress`. Расширяется по мере добавления публичного API в axiom-runtime.

**Когда:** Постепенно, по мере интеграции Engine (axiom-node).

---

### BRD-TD-07 — Engine tick-loop → BroadcastHandle интеграция

**Где:** axiom-runtime (Engine), axiom-broadcasting (BroadcastHandle)

Спека предписывала `broadcasting` feature в axiom-runtime, который зависит от axiom-broadcasting. Невозможно: axiom-broadcasting уже зависит от axiom-runtime → цикл зависимостей. Правильный подход: интеграция делается на уровне бинарного crate (будущий `axiom-node` или демо-бинарник) который зависит на оба crate и вызывает `handle.publish(...)` из хука тик-цикла. Engine к этому готов: `snapshot_for_broadcast()` уже экспортируется через `BroadcastSnapshot`.

**Когда:** При добавлении `axiom-node` бинарника, который зависит на оба crate.

---

### BRD-TD-06 — Интеграционный тест pong timeout

**Где:** `crates/axiom-broadcasting/src/tests.rs`

Тест 2.7.e проверяет только что сервер отправляет Ping. Проверку разрыва соединения при отсутствии Pong сделать нельзя: `tokio-tungstenite` клиент автоматически отвечает на Ping без участия пользовательского кода.

**Когда:** При необходимости — через raw TCP клиент без WebSocket framing.

---

## Workstation V1.0 — отложено из Stage 4

### ~~WS4-TD-01 — DetachTab: нет UI-триггера~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B6. Пункт "View → Detach current tab" добавлен в MenuBar.

---

### ~~WS4-TD-02 — System Map canvas без Cache~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B4. Статический слой (домены, кольца, разделители) через `canvas::Cache`, динамика мандалы — отдельный `Frame` при AnimationTick.

---

### WS4-TD-03 — System Map: неполные визуальные фичи спеки

**Где:** `crates/axiom-workstation/src/ui/system_map.rs`

Не реализованы три элемента из спеки (Документ 3A, раздел 2.4):
- **ASHTI sector fill** — активные домены должны заливать соответствующий сектор среднего кольца мандалы (цвет состояния). Сейчас только линии-разделители.
- **Flow lines** — линии между доменами должны подсвечиваться при `EngineEvent::DomainActivity` за последние ~500ms. Сейчас статические линии к центру.
- **Alert ring** — при `guardian_stats.vetoes_since_wake > 0` снаружи мандалы появляется тонкое красное кольцо. Не реализовано.

**Когда:** Stage 10 (Live Field) — тогда будут живые данные и реальная нагрузка на canvas для проверки.

---

### WS4-TD-04 — SystemSnapshot: поля bottom-panel из спеки отсутствуют в протоколе

**Где:** `crates/axiom-protocol/src/snapshot.rs`, `crates/axiom-workstation/src/ui/system_map.rs`

Спека (Документ 3A, раздел 4.5) описывает в bottom-panel поля `last_hot_path_ns` (время горячего пути) и `promotions_today` / `last_dream_ago`. В `SystemSnapshot` их нет. Bottom-panel сейчас показывает state, fatigue%, tick, frames, events — всё что есть в протоколе.

**Что нужно:** Добавить в `SystemSnapshot`:
- `hot_path_ns: u64` — измеряется в tick-loop
- `promotions_today: u32` — счётчик в FrameWeaverStats
- `dream_phase_stats.last_dream_ended_at_tick` — для вычисления ago

**Когда:** При интеграции живого Engine (axiom-node) — когда snapshot будет заполняться реальными данными.

---

## Workstation V1.0 — отложено из Stage 10

### ~~WS10-TD-01 — Live Field: индивидуальные позиции токенов (протокол-gap)~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в C3. `TokenFieldPoint { position: [f32; 3], layer: u8, temperature: u8, anchor_membership: Option<u32> }` добавлен в протокол и `DomainSnapshot`; sampling max 300 токенов; Live Field теперь использует реальные данные с warmth-color + anchor highlight.

---

## Workstation V1.0 — отложено из Stage 9

### ~~WS9-TD-01 — Главное меню~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B6. Кастомный dropdown MenuBar через `stack` + условный overlay; меню `File / Engine / View / Configuration`.

---

### ~~WS9-TD-02 — Welcome screen анимация (fade-in)~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B5. `welcome_opacity: f32` в `WorkstationApp`, накапливается в `AnimationTick`.

---

### ~~WS9-TD-03 — DetachTab UI~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B6 вместе с WS9-TD-01. "View → Detach current tab" добавлен в MenuBar.

---

## Workstation V1.0 — отложено из Stage 8

### ~~WS8-TD-01 — Файловый пикер (rfd)~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B1. `rfd = { version = "0.14", features = ["tokio"] }`, кнопка Browse через `rfd::AsyncFileDialog::new().pick_file()` + `Task::future`.

---

### ~~WS8-TD-02 — RunBench отсутствует в протоколе~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в C2. `EngineCommand::RunBench { spec: BenchSpec }` добавлен; tick_loop выполняет бенч и публикует `BenchStarted/Progress/Finished`; Workstation `Message::BenchRun` подключён.

---

## Workstation V1.0 — отложено из Stage 7

### ~~WS7-TD-01 — Syntactic S1-S8 sparklines~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в C1. `syntactic_layer_activations: [u8; 8]` добавлен в `FrameWeaverStats`; Patterns tab получает реальные данные из FrameWeaver.

---

### ~~WS7-TD-02 — Show more / пагинация~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B3. Кнопка `[ Show more... ]` в Patterns (max 20, store 100) и Dream State (max 5, store 20) с `show_all: bool` флагом.

---

## Workstation V1.0 — отложено из Stage 6

### ~~WS6-TD-01 — Multi-line text editor + Ctrl+Enter~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в B2. `iced::widget::text_editor` с `text_editor::Content`; Enter = новая строка, Ctrl+Enter = отправка через `on_key_press`.

---

### ~~WS6-TD-02 — Auto-scroll to bottom в ленте~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-03 в Stage 9.

`scrollable` в `message_feed()` теперь имеет `Id::new("chat_feed")`. В `update()` при `ConversationSubmit` и `WsCommandResult` (submit) возвращается `scrollable::scroll_to(Id::new("chat_feed"), AbsoluteOffset { x: 0.0, y: f32::MAX })` через `chat_scroll_to_bottom()`.

---

## Workstation V1.0 — отложено из Stage 5

### ~~WS5-TD-01 — Горячая перезагрузка WS-адреса~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-03 в Stage 9.

Добавлено поле `subscription_key: u64` в `WorkstationApp`. В `ConfigApply` для `workstation.connection` `subscription_key` инкрементируется → iced видит новый id для `Subscription::run_with_id` → пересоздаёт subscription с новым адресом. `ws_subscription` принимает `key: u64` как второй параметр.

---

## Ждут конкретного триггера

### Anchor-Fill — Наполнение якорных YAML-файлов (Фаза 4)

**Где:** `config/anchors/`

Сейчас загружены только:

- `axes.yaml` — 6 осевых якорей (X/Y/Z полюса)
- `layers/L5_cognitive.yaml` — 10 якорей когнитивного слоя
- `domains/D1_execution.yaml` — 6 якорей домена EXECUTION

Для полного семантического покрытия нужно заполнить:

| Файл                     | Слой / Домен | Рекомендуемых якорей |
| ---------------------------- | --------------------- | --------------------------------------- |
| `layers/L1_physical.yaml`  | L1 Physical           | 7+                                      |
| `layers/L2_sensory.yaml`   | L2 Sensory            | 10+                                     |
| `layers/L3_motor.yaml`     | L3 Motor              | 7+                                      |
| `layers/L4_emotional.yaml` | L4 Emotional          | 7+                                      |
| `layers/L6_social.yaml`    | L6 Social             | 7+                                      |
| `layers/L7_temporal.yaml`  | L7 Temporal           | 7+                                      |
| `layers/L8_abstract.yaml`  | L8 Abstract           | 7+                                      |
| `domains/D2_shadow.yaml`   | SHADOW                | 5+                                      |
| `domains/D3_codex.yaml`    | CODEX                 | 5+                                      |
| `domains/D4_map.yaml`      | MAP                   | 5+                                      |
| `domains/D5_probe.yaml`    | PROBE                 | 5+                                      |
| `domains/D6_logic.yaml`    | LOGIC                 | 5+                                      |
| `domains/D7_dream.yaml`    | DREAM                 | 6 (пример в спеке)          |
| `domains/D8_ethics.yaml`   | ETHICS                | 5+                                      |

Формат: [docs/spec/Anchor_Tokens_V1_0.md](docs/spec/Anchor_Tokens_V1_0.md), раздел 7.
Диагностика через CLI: `:match "текст"` — показывает совпадения и вычисленную позицию.

**Когда:** По мере накопления понимания семантики системы (chrnv). Система работает без них — FNV-1a fallback.

---

### ~~D-06 — MLEngine: input_size/output_size = 0 при загрузке ONNX~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в E2. `process_image()` возвращает `MLError::ShapeMismatch { expected: 0, got: 0 }` при `input_size == 0` вместо молчаливого fallback `else { 224 }`.

---

### ~~FW-TD-02 — FrameWeaver: min_participant_anchors не проверяется~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D2. `count_participant_anchors()` выполняет cross-domain lookup через EXPERIENCE-connections с `link_type >> 8 == 0x08`; `qualifies_for_promotion` использует результат.

---

### ~~FW-TD-03 — Weaver::check_promotion без доступа к current_tick~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D1. `tick: u64` добавлен в сигнатуру `Weaver::check_promotion`; `dream_propose` передаёт `engine.tick_count`; `min_age_ticks` теперь проверяется корректно.

---

### ~~FW-TD-04 — on_boot не проверяет GENOME-права~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D4. `on_boot` вызывает `genome.index().check_access(ModuleId::FrameWeaver, ...)` для трёх ресурсов (MAYA/Read, EXPERIENCE/ReadWrite, SUTRA/Control); возвращает `Err(GenomeDenied)` при нарушении.

---

### ~~FW-TD-05 — propose_to_dream возвращает пустые команды (DREAM не реализован)~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-04-29 в DREAM Phase V1.0 + FrameWeaver V1.2.

`propose_to_dream()` заменён на `dream_propose()` — вызывается однократно при входе в `FallingAsleep`. FrameWeaver сканирует кандидатов и передаёт `DreamProposal::Promotion` в `DreamCycle`. Интеграция через `DreamCycle` (Stabilization→Processing→Consolidation).

Оставшийся вопрос о заполнении `commands` снят: промоция идёт через `DreamCycle`, а не через `process_command` напрямую.

---

### ~~FW-TD-06 — промоция EXPERIENCE→SUTRA без участников~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D2. Участники восстанавливаются из EXPERIENCE-графа через `count_participant_anchors`; `check_promotion` передаёт `ashti: &AshtiCore` для cross-domain lookup.

---

### ~~FW-TD-07 — три нереализованных RuleTrigger ветки~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D3. `DreamCycle`: флаг `dream_cycle_completed` в FrameWeaver, устанавливается через `on_dream_wake()`; `HighConfidence(f32)`: `confidence` в `FrameCandidate` = avg `connection.strength`; `RepeatedAssembly`: `stability_count * scan_interval_ticks >= window_ticks`.

---

### ~~EA-TD-07 — Применение domain config при hot-reload~~ ✅ ЗАКРЫТО

**Закрыто:** 2026-05-06 в D5. `apply_domain_config(&mut self, domain_id: u16, cfg: &DomainConfig)` добавлен в `AxiomEngine`; `tick_loop` итерирует `new_cfg.domains` при `watcher.poll()` и применяет каждый домен с `domain_id != 0`.

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

V1.0: проверяется только `major`-байт `PROTOCOL_VERSION`. Если Engine обновлён на minor — Workstation подключается, но поля протокола могут расходиться (новые варианты enum игнорируются, старые enum в новом движке вызывают decode error).

**Что нужно:** Матрица совместимости major.minor: перечень что сломает minor-bump, graceful degradation для смежных minor-версий (игнорировать unknown enum variants), UI-индикатор "version mismatch, limited mode".

**Когда:** V2.0 / перед публичным релизом.

---

### WS-V2-05 — Сетевой режим (remote Engine)

**Где:** `crates/axiom-workstation/src/settings.rs`, `crates/axiom-workstation/src/connection.rs`

V1.0 рассчитан на локальный Engine (`127.0.0.1:9876`). Точки расширения уже помечены в архитектуре V1.0 — address конфигурируем, transport абстрагирован через WebSocket.

**Что нужно:** TLS поверх WS (`wss://`), аутентификация (token в Hello), обработка network timeouts, reconnect при network partition (сейчас backoff предполагает локальный сервис).

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

## Внешние адаптеры

**Спецификация:** [docs/spec/External_Adapters_V3_0.md](docs/spec/External_Adapters_V3_0.md)
**Гайд:** [docs/guides/External_Adapters_Guide_V1_0.md](docs/guides/External_Adapters_Guide_V1_0.md)

| Адаптер         | Requires          | Фаза   | Статус |
|-----------------|-------------------|--------|--------|
| Рефактор CLI    | —                 | 0A/0B/0C | ✅   |
| WebSocket       | axum              | 1      | ✅     |
| REST API        | axum              | 2      | ✅     |
| egui Dashboard  | eframe            | 3      | ✅     |
| Telegram        | reqwest (feature) | 4      | ✅     |
| OpenSearch      | reqwest (feature) | 5      | ✅     |
| gRPC            | tonic + protobuf  | —      | не сейчас |
| Python bindings | pyo3              | —      | не сейчас |
