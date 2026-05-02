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
| 2    | axiom-broadcasting + Engine integration         | TODO        | —           |
| 3    | axiom-workstation базовая инфраструктура        | TODO        | —           |
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

## Этапы 2–11 (TODO)

_Детализируются поэтапно по мере продвижения._

---

## Журнал расхождений (Errata)

_Заполняется по ходу реализации. Переносится в `Workstation_V1_0_errata.md` на этапе 11._

| # | Этап | Расхождение | Решение |
|---|------|-------------|---------|
| — | —    | —           | —       |
