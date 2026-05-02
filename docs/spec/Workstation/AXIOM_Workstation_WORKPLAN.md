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
| 4    | Multi-window, tabs, System Map                  | TODO        | —           |
| 5    | Configuration tab                               | TODO        | —           |
| 6    | Conversation tab                                | TODO        | —           |
| 7    | Patterns + Dream State tabs                     | TODO        | —           |
| 8    | Files + Benchmarks tabs                         | TODO        | —           |
| 9    | Welcome + общие компоненты                      | TODO        | —           |
| 10   | Live Field (3D)                                 | TODO        | —           |
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

## Этапы 4–11 (TODO)

_Детализируются поэтапно по мере продвижения._

---

## Журнал расхождений (Errata)

_Заполняется по ходу реализации. Переносится в `Workstation_V1_0_errata.md` на этапе 11._

| # | Этап | Расхождение | Решение |
|---|------|-------------|---------|
| — | —    | —           | —       |
