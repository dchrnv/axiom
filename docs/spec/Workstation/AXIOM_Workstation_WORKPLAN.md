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
| 1    | axiom-protocol                                  | TODO        | —           |
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

## Этап 1 — axiom-protocol (TODO)

_Детализируется перед началом этапа._

**CORRECTIONS к учёту:**
- C2: добавить ConfigSchema, ConfigSection, ConfigField, ConfigFieldType, ConfigValue
- C2: GetConfig { section } → **не реализовывать**; заменяется GetConfigSchema / GetConfigSection / UpdateConfigField

---

## Этапы 2–11 (TODO)

_Детализируются поэтапно по мере продвижения._

---

## Журнал расхождений (Errata)

_Заполняется по ходу реализации. Переносится в `Workstation_V1_0_errata.md` на этапе 11._

| # | Этап | Расхождение | Решение |
|---|------|-------------|---------|
| — | —    | —           | —       |
