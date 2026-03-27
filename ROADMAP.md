# Axiom Roadmap

**Версия:** 9.5
**Дата:** 2026-03-27

---

## 🔄 Следующая задача: Physics Events → AxiomEngine

**Спека:** `drain_events()` должен возвращать реальные физические события

### Проблема

`AshtiCore::tick()` вызывает `on_event()` + `handle_heartbeat()`, но не вызывает
`process_frontier()`. Поэтому `Domain::generated_events` никогда не попадают
в `AxiomEngine::pending_events` — `drain_events()` всегда пустой.

### Что нужно сделать

**Шаг 1** — `AshtiCore::tick()` возвращает накопленные события

`crates/axiom-domain/src/ashti_core.rs`

```rust
pub fn tick(&mut self) -> Vec<Event> {
    self.pulse += 1;
    let mut all_events = Vec::new();
    for (i, domain) in self.domains.iter_mut().enumerate() {
        if let Some(pulse) = domain.on_event() {
            domain.handle_heartbeat(pulse);
            let tokens  = self.states[i].tokens();
            let conns   = self.states[i].connections();
            let events  = domain.process_frontier(tokens, conns, &mut EventGenerator::new());
            all_events.extend(events);
        }
    }
    all_events
}
```

**Шаг 2** — `AxiomEngine::handle_tick_forward()` пробрасывает события

`crates/axiom-runtime/src/engine.rs`

```rust
fn handle_tick_forward(&mut self, cmd: &UclCommand) -> UclResult {
    let events = self.ashti.tick();
    let count = events.len() as u16;
    self.pending_events.extend(events);
    make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, count)
}
```

**Шаг 3** — Тесты

- `tick()` без токенов → `drain_events()` пустой
- `tick()` с токенами и decay → события в `drain_events()`
- `UclResult::events_generated` отражает реальное количество

---

## 🔜 Малая задача: axiom-upo тесты

**Файл:** `crates/axiom-upo/src/lib.rs`
**Объём:** ~8 тестов, один файл `tests/upo_tests.rs`

- `DynamicTrace::new()` / `update()` / `is_active()`
- `UPOEngine::record_token_change()` / `record_connection_change()` / `generate_patch()`
- `TraceSourceType` enum значения (Token=1, Connection=2, Field=3)
- Size assertion: `DynamicTrace` = 128 байт, align 32

---

## 🔮 Долгосрочные цели

### axiom-upo тесты
UPO v2.2 мигрирован без тестов. Покрыть: `DynamicTrace`, `UPOEngine::record_*`, `generate_patch`. Низкий приоритет.

### Configuration System
YAML-загрузка пространственных параметров и semantic_contributions. Требует согласования с DomainConfig 128-byte constraint.

### Адаптеры
Python bindings, REST API, gRPC — нужны для внешней интеграции.

### Производительность
SIMD (AVX-512), incremental spatial hash rebuild — после стабилизации архитектуры.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-27
