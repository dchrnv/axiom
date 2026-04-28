# DREAM Phase V1.0 — Errata

**Дата создания:** 2026-04-27  
**Назначение:** фиксация неточностей и пробелов спецификации V1.0, обнаруженных в процессе реализации. Все правки войдут в V1.1 после стабилизации.

---

## E1. Naming discrepancy: GatewayCommand → AdapterCommand

Спека и план используют термин `GatewayCommand` для входящих внешних команд. В реальной кодовой базе этот тип называется `AdapterCommand` и живёт в `axiom-agent/src/adapter_command.rs`.

**Решение в реализации:**
- `GatewayPriority` определён в `axiom-runtime/src/over_domain/dream_phase/state.rs`
- `AdapterCommand` расширен полем `priority: GatewayPriority` (default: `Normal`)
- В `AxiomEngine` буфер `dream_priority_buffer: VecDeque<UclCommand>` хранит `UclCommand` (а не `AdapterCommand`) — поскольку `AxiomEngine` в `axiom-runtime` не может зависеть от `axiom-agent`

**Статус:** зафиксировано, реализация консистентна. В V1.1 спеку можно синхронизировать с реальными именами.

---

## Resolution Summary

*(заполняется по мере закрытия этапов стабилизации)*

| Errata | Статус | Этап |
|--------|--------|------|
