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

---

## E2. FrameWeaver: промоция в on_tick — неверный путь

Спека V1.1 описывала шаги 4–5 `on_tick` как место формирования `PromotionProposal` для SUTRA. Реализация показала, что это создаёт противоречие: GUARDIAN вето выдаёт при записи `FRAME_ANCHOR` в SUTRA вне состояния DREAMING. Если `on_tick` работает в WAKE — любые промоционные команды будут отклонены.

**Решение в реализации:**
- Шаги 4–5 удалены из `on_tick`
- Добавлен метод `dream_propose(ashti) -> Vec<DreamProposal>` в `FrameWeaver`
- Вызывается DreamCycle ровно один раз при `tick_falling_asleep`
- Предложения обрабатываются в `Processing` стадии, когда `dream_phase_state == Dreaming`
- Спека обновлена: FrameWeaver V1.2

**Статус:** закрыто в FrameWeaver V1.2.

---

## E3. DreamSchedulerStats: имя поля

Спека и начальный код использовали `total_sleep_decisions`, в финальной реализации поле называется `sleep_decisions`. Расхождение обнаружено в smoke-тесте.

**Решение:** поле `sleep_decisions` — итоговое имя.

**Статус:** закрыто, код консистентен.

---

## E4. Признак total_sleeps vs completed_cycles — off-by-one

`total_sleeps` инкрементируется при входе в `FallingAsleep` (начало цикла), `completed_cycles` — при завершении `Consolidation`. Если система завершает тест в середине цикла, `completed_cycles == total_sleeps - 1`. Это корректное поведение, не баг.

**Решение:** smoke-тест использует `completed_cycles >= total_sleeps.saturating_sub(1)`.

**Статус:** задокументировано, не требует исправлений.

---

## Resolution Summary

| Errata | Статус | Этап |
|--------|--------|------|
| E1 — GatewayCommand → AdapterCommand | Зафиксировано | Этап 0 |
| E2 — Промоция в on_tick (FrameWeaver) | Закрыто в V1.2 | Этап 4 |
| E3 — DreamSchedulerStats.sleep_decisions | Закрыто | Этап 7 |
| E4 — total_sleeps vs completed_cycles off-by-one | Задокументировано | Этап 7 |
