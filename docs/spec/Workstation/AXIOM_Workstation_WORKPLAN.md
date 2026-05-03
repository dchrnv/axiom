# AXIOM WORKSTATION — WORKPLAN

**Назначение:** Операционный план реализации. Живой документ — обновляется по ходу работы.
**База:** AXIOM_Workstation_04_Implementation_Phases.md + CORRECTIONS_V1_0.md
**Начало:** 2026-05-02
**Исполнитель:** Sonnet

---

## Статусы этапов

| Этап | Название                                        | Статус      | Дата        |
|------|-------------------------------------------------|-------------|-------------|
| 0    | Подготовка проекта                              | ✅ DONE     | 2026-05-02  |
| 1    | axiom-protocol                                  | ✅ DONE     | 2026-05-02  |
| 2    | axiom-broadcasting + Engine integration         | ✅ DONE     | 2026-05-02  |
| 3    | axiom-workstation базовая инфраструктура        | ✅ DONE     | 2026-05-02  |
| 4    | Multi-window, tabs, System Map                  | ✅ DONE     | 2026-05-03  |
| 5    | Configuration tab                               | ✅ DONE     | 2026-05-03  |
| 6    | Conversation tab                                | ✅ DONE     | 2026-05-03  |
| 7    | Patterns + Dream State tabs                     | ✅ DONE     | 2026-05-03  |
| 8    | Files + Benchmarks tabs                         | ✅ DONE     | 2026-05-03  |
| 9    | Welcome + общие компоненты                      | ✅ DONE     | 2026-05-03  |
| 10   | Live Field (3D)                                 | ✅ DONE     | 2026-05-03  |
| 11   | Финальная валидация и release prep              | TODO        | —           |

---

## Этап 0 — подготовка проекта ✅ DONE

**Дата:** 2026-05-02
**Цель:** настроить рабочую среду, ничего не реализуя.

### Что сделано

**Workspace:**
- Добавлен `postcard = "1"` в `[workspace.dependencies]` (отсутствовал)
- Три новых crate в `workspace.members`: axiom-protocol, axiom-broadcasting, axiom-workstation

**Созданы crate-ы:**
- `crates/axiom-protocol/` — общие типы Engine ↔ Workstation
- `crates/axiom-broadcasting/` — WebSocket сервер на стороне Engine
- `crates/axiom-workstation/` — клиентское приложение iced

**Зависимости axiom-workstation:**
- iced взят как per-crate (не workspace) — единственный потребитель, нет смысла в workspace
- tokio, serde, postcard — из workspace

### Критерии готовности

- [x] Три crate-а созданы и компилируются
- [x] Все зависимости разрешаются
- [x] `cargo build --workspace` проходит
- [x] `cargo test --workspace` проходит
- [x] CI зелёный

### Errata этапа 0

_Нет расхождений со спекой._

---

## Этап 1 — axiom-protocol ✅ DONE

**Дата:** 2026-05-02

### Что сделано

**Модули:**
- `messages.rs` — EngineMessage, ClientMessage, ClientKind, ShutdownReason, CommandResultData
- `snapshot.rs` — SystemSnapshot, DomainSnapshot, OverDomainSnapshot, FatigueSnapshot, DreamReport, FrameWeaverStats, GuardianStats, DreamPhaseStats, FrameDetails, DomainConfigSummary
- `events.rs` — EngineEvent (14 вариантов), EngineState, SleepTrigger, AlertLevel
- `commands.rs` — EngineCommand (14 вариантов; GetConfig не реализован — C2)
- `config.rs` — ConfigSchema, ConfigSection, ConfigField, ConfigFieldType, ConfigValue, ConfigCategory (C2)
- `bench.rs` — BenchSpec, BenchOptions, BenchResults, BenchEnvironment
- `adapters.rs` — AdapterInfo, AdapterOption, AdapterProgress, AdapterStatus
- `lib.rs` — event_category битовые флаги, PROTOCOL_VERSION = 0x01_00_00_00
- `tests.rs` — 41 round-trip тест через postcard

**CORRECTIONS применены:**
- C2: GetConfigSchema / GetConfigSection / UpdateConfigField вместо GetConfig
- C2: полная иерархия ConfigSchema в config.rs
- C2: ConfigSchema(ConfigSchema), ConfigSection(ConfigSection), ConfigUpdateApplied, ConfigValidationError в CommandResultData
- ConfigFieldType::Float имеет опциональный `step: Option<f64>`

### Критерии готовности

- [x] Все типы определены
- [x] postcard сериализация работает для всех вариантов
- [x] 41 round-trip тест (требование ≥ 30)
- [x] axiom-protocol компилируется без errors
- [x] PROTOCOL_VERSION определён
- [x] Весь workspace зелёный (ноль регрессий)

### Errata этапа 1

- `BenchOptions` не имел `PartialEq` через `Default` derive — добавлен явно.

---

## Этап 2 — axiom-broadcasting ✅ DONE

**Дата:** 2026-05-02

### Что сделано

**Модули:**
- `config.rs` — BroadcastingConfig (C4): tick_event_interval=100, domain_activity_threshold=5,
  max_event_queue_per_client=1000, heartbeat_interval=30s, pong_timeout=10s, DropOldest
- `server.rs` — BroadcastServer + BroadcastHandle:
  - accept loop, per-client tokio task
  - бинарное postcard handshake: ClientMessage::Hello → EngineMessage::Hello / Bye(VersionMismatch)
  - subscription filter (event_category битовые флаги + tick % tick_event_interval)
  - broadcast fan-out: event_tx → всем подписанным клиентам
  - command channel: клиент → Engine (mpsc unbounded)
  - heartbeat: сервер посылает Ping каждые heartbeat_interval, ждёт Pong до pong_timeout
  - RecvError::Lagged: предупреждение логируется, соединение не разрывается (SCALE-POINT)
- `snapshot.rs` — build_system_snapshot(): BroadcastSnapshot → SystemSnapshot
- `tests.rs` — 6 интеграционных тестов (2.7.a–f)

**Технические решения:**
- tokio-tungstenite зафиксирован на 0.24 (совместимость с axiom-agent, Vec<u8> API)
- build_system_snapshot() в axiom-broadcasting (не в axiom-runtime) — избегаем циклических deps

### Критерии готовности

- [x] BroadcastServer запускается и принимает соединения
- [x] Handshake с version check (major byte)
- [x] Subscription filter работает (tick interval + category bits)
- [x] Outgoing heartbeat: сервер инициирует ping
- [x] 6 интеграционных тестов: 2.7.a–f (все pass)
- [x] Весь workspace зелёный (ноль регрессий)

### Deferred (не блокирует Stage 3)

- **BRD-TD-01** — DomainActivity threshold enforcement (требует Engine API)
- **BRD-TD-03** — Snapshot resync при RecvError::Lagged (SCALE-POINT в коде)
- **BRD-TD-05** — Полнота полей build_system_snapshot() (zero defaults, расширяется с Engine API)
- **BRD-TD-06** — Pong timeout disconnect integration test (tungstenite клиент авто-отвечает на ping)
- Engine integration: broadcasting feature в axiom-runtime + hook в tick loop → **начало Stage 3**

### Errata этапа 2

- tokio-tungstenite в workspace был 0.26+ (Bytes API) — откатили до 0.24 (Vec<u8>)
- BroadcastingConfig: добавлены `heartbeat_interval` и `pong_timeout` поля (не были в исходной спеке)

---

## Этап 3 — axiom-workstation базовая инфраструктура ✅ DONE

**Дата:** 2026-05-02

### Что сделано

**Новые файлы:**
- `settings.rs` — UiSettings (engine_address: String), load_settings() / save_settings(), config path через dirs
- `app.rs` — WorkstationApp: ConnectionState (4 состояния), Message (6 вариантов), update/view/subscription
- `connection.rs` — ws_subscription() → iced::Subscription, run_session() с handshake + основным циклом

**Ключевые решения:**
- iced 0.13: `iced::stream::channel(size, FnOnce) + iced::Subscription::run_with_id(id, stream)` — channel из iced::subscription отсутствует
- Reconnect backoff: `[1, 2, 5, 10, 30]` сек, attempt счётчик сбрасывается при успешном соединении
- ConnectionState::Connected содержит `connected_at: Instant` (uptime в view)
- Handshake: Hello → Hello(engine_version) → WsConnected
- recent_events: VecDeque с capacity 1000

**Тесты:**
- 3.7.a `test_settings_roundtrip` — TOML сериализация / десериализация
- 3.7.b `test_settings_default` — дефолтный адрес 127.0.0.1:9876
- 3.7.c `test_websocket_handshake` — подключение к BroadcastServer, ожидание WsConnected

### Критерии готовности

- [x] `cargo check -p axiom-workstation` — ноль ошибок
- [x] `cargo test -p axiom-workstation` — 3/3 тестов pass
- [x] ConnectionState / Message определены
- [x] WebSocket subscription с reconnect backoff
- [x] Settings persistence (TOML в config dir)
- [x] Базовый view: статус соединения + tick + events count

### Deferred

- BRD-TD-07: Engine tick-loop → BroadcastHandle (cyclic dep axiom-runtime ↔ axiom-broadcasting — откладывается в axiom-node)

### Errata этапа 3

- `iced::subscription::channel` не существует в iced 0.13.1 — правильный путь: `iced::stream::channel + iced::Subscription::run_with_id`

---

## Этап 4 — Multi-window, tabs, System Map ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Архитектура:**
- Переход с `iced::application` на `iced::daemon` (multi-window view с `window::Id`)
- Главное окно открывается в `run_with` через `window::open()`
- `view(&self, id: window::Id)` диспатчится на main vs detached окна

**Новые файлы:**
- `ui/mod.rs` — модуль UI-компонентов
- `ui/header.rs` — заголовок с индикатором подключения
- `ui/tabs.rs` — таб-бар (фильтрует detached вкладки)
- `ui/placeholder.rs` — заглушка для нереализованных вкладок
- `ui/system_map.rs` — System Map через `canvas::Program`

**System Map (canvas):**
- Мандала: 3 концентрических кольца + 8 разделителей ASHTI + ядро SUTRA
- Пульсация через `animation_phase` (sin-функция)
- Цвет состояния: Wake=синий, Dreaming=индиго, FallingAsleep/Waking=переходные
- Домены вокруг мандалы: активные подсвечиваются, линии к центру
- Bottom labels: state, fatigue%, tick, frames, events
- Loading state: вращающаяся дуга

**Новые features iced:**
- `canvas` — iced::widget::canvas
- `tokio` — iced::time::every

### Критерии готовности

- [x] Multi-window: main + detached окна с разным view
- [x] Tabs переключаются (TabSelected message)
- [x] TabKind: 8 вкладок (Map + 7 placeholder)
- [x] Detach: открывает новое окно, убирает таб из main bar
- [x] Window close: main → exit(), detached → close + вернуть таб
- [x] System Map рендерит мандалу с Canvas
- [x] Анимация ~30fps через iced::time::every(33ms)
- [x] 7 тестов: 3.7.a, 3.7.b, 3.7.d (stage 3) + 4 новых (4.6.a + 3 unit)

### Errata этапа 4

- `iced::application` не поддерживает разные view по window::Id — нужен `iced::daemon`
- `Padding` в iced 0.13 не поддерживает `[i32; 4]` — только `[u16; 2]` / `f32` / `u16`
- canvas и time::every требуют явных features в Cargo.toml

---

## Этап 5 — Configuration tab ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Новые файлы:**
- `ui/config.rs` — schema-driven Configuration UI

**Архитектура:**
- Двухпанельный layout: левая панель (дерево секций) + правая панель (поля)
- `ConfigurationState.sections: Vec<ConfigSection>` — секции хранятся в стейте, не вычисляются в view (решение проблемы lifetime iced)
- `rebuild_sections()` — пересобирает список при получении схемы или изменении настроек

**Bidirectional WS (connection.rs обновлён):**
- После handshake создаётся `(cmd_tx, cmd_rx): iced::futures::channel::mpsc::channel(32)`
- `CommandSenderReady(CommandSender)` → app хранит sender, немедленно отправляет `GetConfigSchema`
- Основной loop: `tokio::select!` на `stream.next()` и `cmd_rx.next()`
- `WsCommandResult` обрабатывает `ConfigSchema`, `ConfigUpdateApplied`, `ConfigValidationError`

**UI компоненты (config.rs):**
- `section_panel()` — рекурсивное дерево секций с depth-indent
- `field_panel()` + `field_row()` + `field_control()` — рендер по `ConfigFieldType`
- Контролы: `Bool`→checkbox, `String`→text_input, `Integer/UInt/Float`→text_input+parse, `Enum`→button row, `Duration/Domain`→text_input
- Pending-индикатор: `●` в label поля
- Apply/Discard кнопки (disabled при нет pending)

**Workstation-секция:**
- `build_workstation_section(settings)` — локальная секция "Connection" с полем `engine_address`
- ConfigApply для workstation: обновляет `settings.engine_address` + `save_settings()` + `rebuild_sections()`
- ConfigApply для engine-секций: `UpdateConfigField` команда на каждое изменённое поле

**Новые Message варианты:**
- `CommandSenderReady(CommandSender)`, `SendCommand(EngineCommand)`, `WsCommandResult { command_id, result }`
- `ConfigSectionSelected(String)`, `ConfigFieldChanged { section_id, field_id, value }`, `ConfigApply { section_id }`, `ConfigDiscard`

**Тесты (5.7.a–d):**
- `test_field_change_marks_pending` — изменение поля → pending
- `test_discard_clears_pending` — Discard очищает pending активной секции
- `test_apply_workstation_updates_settings` — Apply workstation → settings.engine_address
- `test_section_navigation` — выбор секции обновляет active_section_id

### Критерии готовности

- [x] Schema-driven UI: все `ConfigFieldType` варианты рендерятся
- [x] Workstation-секция (Connection) с полем engine_address
- [x] Apply/Discard с pending-индикацией
- [x] Bidirectional WS: команды app → engine
- [x] GetConfigSchema при подключении
- [x] `cargo check -p axiom-workstation` — ноль ошибок
- [x] `cargo test -p axiom-workstation` — 11/11 тестов pass

### Deferred

- **WS5-TD-01** — Конфиг вкладка: горячая перезагрузка WS-адреса (требует рестарт subscription)
- **WS5-TD-02** — Конфиг: горячая перезагрузка engine subscription при смене адреса

### Errata этапа 5

- `Padding` в iced 0.13 не поддерживает `[i32; 4]` — используется `Padding { top, right, bottom, left }` struct
- `build_section_list` не может быть локальным в view-функции (lifetime проблема iced) — секции перенесены в `ConfigurationState.sections: Vec<ConfigSection>`, пересобираются через `rebuild_sections()`
- `workstation_section` переименован в `build_workstation_section` (pub для вызова из app.rs)

---

## Этап 6 — Conversation tab ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Новые файлы:**
- `ui/conversation.rs` — UI чата (лента + форма ввода)

**Новые типы в app.rs:**
- `ConversationState` — messages, input_buffer, target_domain, sending, last_submit_at, pending_submit_id
- `ConversationMessage` — User / System enum
- `SystemMessageKind` — Acknowledgment / FrameCreated / FrameReactivated / Error

**UI (conversation.rs):**
- Лента сообщений: scrollable, старые сверху, новые снизу
- `You` / `System` префиксы с цветовым кодированием по kind
- Timestamp в формате HH:MM:SS (UTC)
- Selector целевого домена: 8 кнопок EXEC/SHDW/CODX/MAP/PROB/LOGI/DREM/ETHI
- text_input с on_submit (Enter) + кнопка Submit
- Submit disabled при sending=true или пустом буфере
- "Sending..." лейбл при отправке

**Логика (app.rs):**
- `ConversationSubmit` → push User message, sending=true, запоминает pending_submit_id, отправляет `EngineCommand::SubmitText`
- `WsCommandResult` с pending_submit_id → push Acknowledgment или Error system message, clearing input_buffer
- `WsEvent::FrameCrystallized/FrameReactivated` → если `is_recent_submit()` (< 5 сек) → push FrameCreated/FrameReactivated в ленту

**Тесты (6.7.a–f):**
- `test_conversation_empty_no_submit` — пустой ввод без эффекта
- `test_conversation_submit_adds_message` — Submit добавляет User message, sending=true
- `test_conversation_no_double_submit` — двойной submit пока sending → только одно сообщение
- `test_conversation_domain_selector` — смена домена
- `test_conversation_ack_on_result` — CommandResult(Ok) → Ack, input очищен
- `test_conversation_error_on_result` — CommandResult(Err) → Error message

### Критерии готовности

- [x] Tab Conversation открывается
- [x] Подача текста работает (User message в ленте)
- [x] Системные ответы появляются (Acknowledgment / FrameCreated / FrameReactivated / Error)
- [x] Цвет сообщений: Error — тёплый красный, Frame — зелёный, Reactivated — фиолетовый
- [x] Submit по Enter и по кнопке
- [x] Selector целевого домена (8 вариантов)
- [x] Корреляция с FrameCrystallized / FrameReactivated (< 5 сек)
- [x] 17/17 тестов pass

### Deferred

- **WS6-TD-01** — Multi-line text_editor + Ctrl+Enter (сейчас text_input, одна строка)
- **WS6-TD-02** — Auto-scroll to bottom при новом сообщении (требует `scrollable::Id` + `scroll_to` Task)

### Errata этапа 6

- Timestamp без chrono: `SystemTime::UNIX_EPOCH` + `secs % 86400` → UTC HH:MM:SS
- Корреляция через `Instant::elapsed()` в ConversationState, не через timestamp

---

## Этап 7 — Patterns + Dream State tabs ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Новые файлы:**
- `ui/patterns.rs` — Patterns tab: sparklines L1-L8 + recent frames feed
- `ui/dream_state.rs` — Dream State tab: current state + fatigue + dream history

**Новые типы в app.rs:**
- `FrameEvent` — 4 варианта: Crystallized / Reactivated / Vetoed / Promoted
- `PatternsState` — layer_history ([VecDeque<u8>; 8]) + recent_frames (VecDeque<FrameEvent>)
- `DreamWindowState` — recent_dreams (VecDeque<DreamReport>) + confirm_force_sleep

**Patterns UI:**
- Sparklines L1-L8 через Unicode block chars (▁▂▃▄▅▆▇█) в monospace шрифте
- Цветовое кодирование уровня: silent/low/medium/high/highest
- Recent frames лента: ● кристаллизация, ↻ реактивация, ⊗ veto, ↑ промоция
- format_ago(): "just now" / "Xs ago" / "Xm ago"

**Dream State UI:**
- Текущее состояние крупно (WAKE/DREAMING/...) с цветом по state
- Ticks since last transition
- Force sleep с inline-подтверждением (Cancel + Sleep now)
- Wake up кнопка в DREAMING состоянии
- Fatigue panel: процент, sparkline из history, token_rate, ticks_since_dream
- Recent dreams: cycle_id, fatigue before→after, accepted/rejected/promoted counts

**Интеграция с WsEvent:**
- FrameCrystallized → PatternsState.push_frame_event + conversation correlation
- FrameReactivated → patterns + conversation
- FramePromoted → patterns
- GuardianVeto → patterns
- DomainActivity → push_layer_snapshot (per-domain activations)

**Интеграция с WsSnapshot:**
- over_domain.layer_activations → push_layer_snapshot
- last_dream_report → DreamWindowState.push_dream (если новый cycle_id)

**Новые Message варианты:**
- ForceSleepRequest, ForceSleepConfirm, ForceSleepCancel, ForceWakeRequest

**Тесты (7.6.a–d):**
- `test_patterns_frame_event_from_ws_event` — FrameCrystallized → patterns feed
- `test_patterns_veto_event` — GuardianVeto → patterns feed
- `test_dream_force_sleep_confirm_flow` — Request → confirm_force_sleep=true, Cancel → false
- `test_patterns_layer_history_from_event` — DomainActivity → layer_history

### Критерии готовности

- [x] Patterns: sparklines L1-L8 с level-labels
- [x] Patterns: recent frames с 4 типами событий (●↻⊗↑)
- [x] Dream State: текущее состояние с цветом
- [x] Dream State: fatigue с sparkline
- [x] Dream State: recent dreams лента
- [x] Force-sleep с inline подтверждением
- [x] Wake-up кнопка в DREAMING состоянии
- [x] 21/21 тестов pass

### Deferred

- **WS7-TD-01** — Syntactic S1-S8 sparklines (данные отсутствуют в протоколе — нет `syntactic_layer_activations` в FrameWeaverStats)
- **WS7-TD-02** — Show more / пагинация в Patterns и Dream State лентах

### Errata этапа 7

- `Element` в iced 0.13 не имеет метода `.width()` — нужен `container(elem).width(N)`
- Syntactic layers S1-S8 отсутствуют в протоколе (FrameWeaverStats не содержит per-layer stats)

---

## Этап 8 — Files + Benchmarks tabs ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Новые файлы:**
- `ui/files.rs` — Files tab: выбор адаптера (кнопки), поле пути + Browse placeholder, кнопка Start, прогресс-панель с cancel-confirm, история импортов
- `ui/benchmarks.rs` — Benchmarks tab: поле iterations, кнопка Run (placeholder), прогресс бегущего бенча, история результатов с форматированием ns→µs→ms

**Новые типы в app.rs:**
- `RunningImport` — adapter_id, source, processed, total
- `CompletedImport` — adapter_id, source, tokens_added, errors, timestamp_secs, cancelled
- `FilesState` — available_adapters, adapters_fetched, source_path, selected_adapter_id, running_import, completed_imports, cancel_confirm
- `RunningBench` — bench_id, run_id, completed, total
- `BenchmarksState` — history (VecDeque<BenchResults>), running, iterations_input

**Новые Message варианты:**
- Files: `FilesPathChanged`, `FilesAdapterSelected`, `FilesStartImport`, `FilesCancelRequest`, `FilesConfirmCancel`, `FilesCancelDismiss`
- Benchmarks: `BenchIterationsChanged`, `BenchRun`

**Интеграция с WsEvent:**
- `AdapterStarted` → FilesState.running_import
- `AdapterProgress` → running_import.processed/total
- `AdapterFinished` → running_import → completed_imports
- `BenchStarted` → BenchmarksState.running
- `BenchProgress` → running.completed/total
- `BenchFinished` → running → history

**CommandSenderReady** расширен: при подключении отправляет `ListAdapters` (id2).
**WsCommandResult** → `AdapterList` → files.available_adapters.

**Тесты (8.6.a–f):**
- `test_files_path_changed`, `test_files_start_import_no_adapter_noop`, `test_adapter_started_sets_running`, `test_adapter_finished_moves_to_completed`, `test_bench_lifecycle`, `test_adapter_list_result`

### Критерии готовности

- [x] Files: список адаптеров из движка
- [x] Files: path input + Start/Cancel
- [x] Files: прогресс-бар и cancel-confirm flow
- [x] Files: история импортов (max 50)
- [x] Benchmarks: прогресс бегущего бенча
- [x] Benchmarks: история результатов с ns→µs→ms
- [x] 27/27 тестов pass

### Deferred

- **WS8-TD-01** — Нативный файловый пикер (`rfd`) — зависимость не добавлена
- **WS8-TD-02** — `EngineCommand::RunBench` отсутствует в протоколе

### Errata этапа 8

- `let adapter_row: Element<'a, Message>` — явная аннотация типа нужна при условном `if` с `text()`: компилятор не может вывести тип параметра `Theme` без контекста.

---

## Этап 9 — Welcome + общие компоненты ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**Новые файлы:**
- `ui/welcome.rs` — экран приветствия: "AXIOM" 52pt + "Workstation" 28pt, центровка; 4 состояния по ConnectionState (Connecting, Reconnecting, Disconnected, Connected)

**Изменения app.rs:**
- `AppPhase` enum: `Welcome` / `Main`
- `AlertEntry` struct: message + timestamp_secs
- Новые поля в WorkstationApp: `phase`, `show_connection_details`, `alerts: VecDeque<AlertEntry>`, `subscription_key: u64`
- `push_alert()` метод
- `WsConnected` в Welcome → переход в Main + save_settings (создаёт конфиг-файл, фиксирует "видел")
- `ConfigApply` для workstation.connection инкрементирует `subscription_key` (fix WS5-TD-01)
- `ConversationSubmit` + `WsCommandResult` возвращают `chat_scroll_to_bottom()` через `Task::batch`
- `AnimationTick` — авто-dismiss alerts старше 10s
- `view()` диспатчит Welcome vs Main по `phase`
- `main_window_view` использует `iced::widget::stack!` для overlay алертов
- Module-level функции: `chat_scroll_to_bottom()`, `alert_overlay()`, `keyboard_shortcut()`

**Новые Message варианты:**
- `SkipToMain`, `ToggleConnectionDetails`, `DismissAlert(usize)`, `ConfigApplyActive`

**Изменения ui/header.rs** (переписан):
- Индикатор соединения → кнопка `button::text`, эмитит `ToggleConnectionDetails`
- При `show_details=true` — панель с engine address/version/uptime + кнопка Disconnect

**Изменения connection.rs (WS5-TD-01 fix):**
- `ws_subscription(address: String, key: u64)` → id = `(address, key)` в `Subscription::run_with_id`

**Изменения settings.rs:**
- `is_first_run() -> bool` — `!config_path().exists()`

**Клавиатурные шорткаты (keyboard_shortcut fn):**
- Ctrl+1–8 → переключение табов
- Ctrl+, → Config tab
- Ctrl+S → ConfigApplyActive
- Ctrl+Z → ConfigDiscard

**Тесты (9.5.a–f):**
- `test_skip_to_main`, `test_ws_connected_transitions_welcome_to_main`, `test_ws_connected_main_stays_main`, `test_subscription_key_increments_on_address_change`, `test_toggle_connection_details`, `test_dismiss_alert`

**Закрыто из DEFERRED:** WS5-TD-01, WS6-TD-02

### Критерии готовности

- [x] Welcome screen с 4 состояниями по подключению
- [x] First-run detection через config_path().exists()
- [x] WsConnected в Welcome → переход в Main
- [x] Alert overlay через stack!, auto-dismiss 10s
- [x] Clickable connection indicator + details popup
- [x] Keyboard shortcuts Ctrl+1–8, Ctrl+S, Ctrl+Z
- [x] Hot-reload адреса через subscription_key
- [x] 33/33 тестов pass

### Deferred

- **WS9-TD-01** — MenuBar (iced 0.13 нет нативного widget)
- **WS9-TD-02** — Welcome fade-in анимация
- **WS9-TD-03** — DetachTab UI (зависит от WS9-TD-01)

### Errata этапа 9

- `iced::keyboard::on_key_press` принимает `fn` pointer, не замыкание: сигнатура `fn(Key, Modifiers) -> Option<Message>` — хешируемый указатель функции. Решение: вынести в module-level `fn keyboard_shortcut(...)`.
- WS5-TD-01: передача только `address.clone()` как ID давала одинаковый ID при смене адреса → subscription не перезапускалась. Исправлено через `(address, key)` кортеж.

---

## Этап 10 — Live Field (3D) ✅ DONE

**Дата:** 2026-05-03

### Что сделано

**app.rs:**
- Типы: `OrbitCamera`, `DisplayOptions`, `LiveFieldOption`, `LiveFieldState`
- Новые сообщения: `LiveFieldDomainSelected`, `LiveFieldCameraRotate`, `LiveFieldCameraZoom`, `LiveFieldCameraReset`, `LiveFieldToggleOption`
- Поле `live_field: LiveFieldState` в `WorkstationApp`
- Обработчики в `update()`: вращение/зум/сброс камеры, переключение display-опций
- Tab wiring: `TabKind::LiveField → live_field::live_field_view(...)`

**ui/live_field.rs (новый):**
- Боковая панель: список доменов (кнопки-селекторы), статы выбранного домена, 4 display-toggle, "Reset camera"
- `canvas::Program` (`LiveFieldCanvas<'a>`) с `type State = DragState`
- Орбитальная камера: сферические координаты (azimuth, elevation, distance), простое перспективное проецирование (матрица вращения Ry(-az)·Rx(-el), затем деление на -fz)
- Мышь: drag → `LiveFieldCameraRotate`, scroll → `LiveFieldCameraZoom`, `mouse_interaction: Grab/Grabbing`
- Процедурные точки: детерминированный LCG по (domain_id, index), позиции в сфере радиуса [0.3, 1.0], MAX_POINTS=300 на домен
- Layer color coding: 8 цветов (синий→пурпурный), выбор слоя по `layer_activations` weights
- `show_connections`: fan-линии от точек к anchor-точке (первой точке домена)
- `show_anchors`: октаэдр из 12 рёбер в origin
- `highlight_recent`: +1px к точкам активных доменов
- "No engine data" fallback текст в canvas

**Протокол-gap:** `DomainSnapshot` не содержит индивидуальных позиций токенов → WS10-TD-01 в DEFERRED.

### Тесты

6 новых тестов (10.5.a–f): domain select, rotate, elevation clamp, zoom clamp, reset, toggle.

**Итого:** 39 тестов, 0 ошибок.

---

## Этапы 11 (TODO)

_Детализируется поэтапно по мере продвижения._

---

## Журнал расхождений (Errata)

_Заполняется по ходу реализации. Переносится в `Workstation_V1_0_errata.md` на этапе 11._

| # | Этап | Расхождение | Решение |
|---|------|-------------|---------|
| 1 | 8    | `text()` в условном `if` не выводит параметр `Theme` | Явная аннотация `let adapter_row: Element<'a, Message> = ...` |
| 2 | 9    | `on_key_press` принимает `fn` pointer, не замыкание | Вынести в module-level `fn keyboard_shortcut(...)` |
| 3 | 9    | `run_with_id(address)` не перезапускается при той же строке | ID изменён на кортеж `(address, key)` + поле `subscription_key: u64` |
| 4 | 10   | `DomainSnapshot` не содержит индивидуальных позиций токенов | Процедурная визуализация через детерминированный LCG; реальные данные — WS10-TD-01 |
