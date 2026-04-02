# Axiom Roadmap

**Версия:** 18.0
**Дата:** 2026-04-02

---

## Cleanup + COM V1.1 + Tick Scheduling

**Цель:** Закрыть технический долг, подготовить ядро к живому использованию.
**Принцип:** Каждая фаза — отдельный коммит. `cargo test --workspace` зелёный после каждой фазы.

---

### Фаза 1 — Unsafe Unwrap Cleanup

**Цель:** Убрать все потенциальные паники с горячего пути.

**1A. experience.rs — `partial_cmp().unwrap()` → `total_cmp`**

Файл: `crates/axiom-arbiter/src/experience.rs:173, 316`

```rust
// было
.min_by(|(_, a), (_, b)| a.weight.partial_cmp(&b.weight).unwrap())
.max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())

// стало
.min_by(|(_, a), (_, b)| a.weight.total_cmp(&b.weight))
.max_by(|a, b| a.weight.total_cmp(&b.weight))
```

`f32::total_cmp` стабилен с Rust 1.62. Определяет полный порядок включая NaN. Не паникует никогда.

**1B. arbiter/lib.rs — `resonance.trace.unwrap()` → `if let`**

Файл: `crates/axiom-arbiter/src/lib.rs:278, 386`

```rust
// было
let reflex_token = resonance.trace.as_ref().unwrap().pattern;

// стало (выбрать вариант по контексту)
let Some(trace) = resonance.trace.as_ref() else { return ...; };
let reflex_token = trace.pattern;
```

**1C. loader.rs — `key.as_str().unwrap()` → safe fallback**

Файл: `crates/axiom-config/src/loader.rs:289, 300`

```rust
// было
key.as_str().unwrap()

// стало
let Some(key_str) = key.as_str() else { continue; };
```

**Тесты к Фазе 1:**
- `resonance_search` с `trace = None` → не паникует, возвращает корректный результат
- Сортировка по weight c NaN в наборе → не паникует
- Загрузка YAML с нестроковым ключом → возвращает ошибку, не паникует

---

### Фаза 2 — EventType::Unknown

**Цель:** Неизвестный тип события не обрушивает процесс.

Файл: `crates/axiom-core/src/event.rs:152`

```rust
// было
_ => panic!("Unknown event type: {:#06x}", v),

// стало — добавить вариант в enum
Unknown = 0xFFFF,

// в From<u16>:
_ => Self::Unknown,
```

В pipeline обработка `Unknown` — пропускать событие молча (не применять к состоянию).

**Тесты к Фазе 2:**
- `EventType::from(0xBEEF)` → `Unknown`, не паника
- Событие с типом `Unknown` в pipeline → пропускается, обработка продолжается

---

### Фаза 3 — Event: source_domain + payload[8] + snapshot_event_id

**Цель:** Заполнить `_reserved: [u8; 16]` структурированными полями. Решает ShellEffector payload.

**Контекст:** Event уже 64B (`_reserved: [u8; 16]` занимает последние 16 байт). Это не расширение структуры — только переименование зарезервированного места.

Файл: `crates/axiom-core/src/event.rs`

**3A. Заменить `_reserved: [u8; 16]` на три поля:**

```rust
// было
pub _reserved: [u8; 16],

// стало
pub source_domain: u16,        // 2B — домен-источник (для GUARDIAN enforce_protocol)
pub snapshot_event_id: u32,    // 4B — ID снапшота (для Causal Horizon архивации)
pub payload: [u8; 8],          // 8B — inline payload (structured данные)
pub _pad: [u8; 2],             // 2B — выравнивание до 16B
```

Итого: 2 + 4 + 8 + 2 = 16 байт. Размер Event не меняется. compile-time assert остаётся зелёным.

**Семантика `payload: [u8; 8]` по event_type:**

| event_type | payload содержит |
|---|---|
| ShellExec | `[command_index: u16 LE, _pad: 6]` — индекс команды в side-channel таблице Gateway |
| InternalImpulse | `[impulse_type: u8, intensity: u8, source_trace: u32 LE, _pad: 2]` |
| TokenMove | `[dx: i16 LE, dy: i16 LE, dz: i16 LE, _pad: 2]` |
| Остальные | `[0u8; 8]` |

**Семантика `source_domain`:** домен, породивший событие. По умолчанию = `domain_id`. GUARDIAN использует пару `(source_domain, domain_id)` для `enforce_protocol`.

**Семантика `snapshot_event_id`:** ID последнего снапшота на момент создания события. Causal Horizon: события с `snapshot_event_id < текущий` безопасны для архивации.

**3B. Обновить `Event::new()`:** новые поля инициализировать нулями, `source_domain = domain_id`.

**3C. Добавить новые флаги в `flags: u8`:**

```rust
pub const FLAG_CRITICAL:  u8 = 0b0000_0001; // уже есть
pub const FLAG_REVERSIBLE:u8 = 0b0000_0010; // уже есть
pub const FLAG_INTERNAL:  u8 = 0b0000_0100; // НОВЫЙ: от Internal Drive
pub const FLAG_BATCH:     u8 = 0b0000_1000; // НОВЫЙ: батч-событие
```

**3D. Исправить ShellEffector:**

Файл: `crates/axiom-agent/src/channels/shell.rs:83`

```rust
// было: всегда None
fn extract_command(event: &Event) -> Option<String> { None }

// стало: читаем command_index из payload
fn extract_command_index(event: &Event) -> Option<u16> {
    if event.event_type != SHELL_EXEC_EVENT_TYPE { return None; }
    let idx = u16::from_le_bytes([event.payload[0], event.payload[1]]);
    if idx > 0 { Some(idx) } else { None }
}
```

Gateway хранит маппинг `command_index → String`. Ядро возвращает только индекс — строки в ядро не входят.

**Тесты к Фазе 3:**
- `size_of::<Event>() == 64` — compile-time assert остаётся
- `Event::new()` → `source_domain == domain_id`, `payload == [0u8; 8]`
- `extract_command_index` → `Some(idx)` для ShellExec с `payload[0..2] = idx.to_le_bytes()`
- `extract_command_index` → `None` для других event_type
- Все существующие тесты проходят без изменений

---

### Фаза 4 — com_next_id в Snapshot

**Цель:** COM-счётчик не сбрасывается при restore, монотонность event_id гарантирована.

Файл: `crates/axiom-runtime/src/engine.rs`

**4A. Добавить поле в AxiomEngine:**

```rust
pub struct AxiomEngine {
    // ... существующие поля ...
    pub(crate) com_next_id: u64,
}
```

Все места генерации `event_id` берут из `self.com_next_id` и инкрементируют:

```rust
fn next_event_id(&mut self) -> u64 {
    let id = self.com_next_id;
    self.com_next_id += 1;
    id
}
```

**4B. Сохранять в snapshot (строка 133):**

```rust
// было
com_next_id: 0,

// стало
com_next_id: self.com_next_id,
```

**4C. Восстанавливать при restore:**

```rust
fn restore_from(&mut self, snapshot: EngineSnapshot) {
    // ... восстановление доменов ...
    self.com_next_id = snapshot.com_next_id;
}
```

**Тесты к Фазе 4:**
- inject tokens → snapshot → inject ещё → restore → `com_next_id` восстановлен
- После restore: все новые события имеют `event_id > snapshot.com_next_id` (монотонность)
- event_id не коллидируют после restore

---

### Фаза 5 — Magic Numbers → Config

**Цель:** Захардкоженные пороги становятся конфигурируемыми.

**5A. compare_tokens пороги в DomainConfig**

Файл: `crates/axiom-arbiter/src/lib.rs:536-538`

Добавить в `DomainConfig` (или `ArbiterConfig` — по контексту, смотреть на размер 128B):

```rust
// В конфиге:
pub token_compare_temp_tolerance:    i16,  // default: 10
pub token_compare_mass_tolerance:    i16,  // default: 5
pub token_compare_valence_tolerance: i16,  // default: 2
```

```rust
// В compare_tokens():
let temp_match    = diff_temp.abs()    < config.token_compare_temp_tolerance;
let mass_match    = diff_mass.abs()    < config.token_compare_mass_tolerance;
let valence_match = diff_valence.abs() < config.token_compare_valence_tolerance;
```

**5B. UclBuilder::spawn_domain — structural_role**

Файл: `crates/axiom-ucl/src/lib.rs:258`

Проверить: если `structural_role` и `factory_preset` семантически одно и то же — убрать дублирование и оставить одно поле. Если разное — определить явный маппинг и убрать комментарий "Временно".

**Тесты к Фазе 5:**
- `compare_tokens` с default tolerance → результат совпадает с текущим поведением (regression)
- `compare_tokens` с tolerance = 0 → только точное совпадение

---

### Фаза 6 — Tick Scheduling

**Цель:** Периодические задачи работают на нужных частотах, не каждый тик.

**6A. Структура TickSchedule**

Файл: новый или в `crates/axiom-runtime/src/engine.rs`

```rust
pub struct TickSchedule {
    pub adaptation_interval:    u32,  // default: 50
    pub horizon_gc_interval:    u32,  // default: 500
    pub snapshot_interval:      u32,  // default: 5000
    pub dream_interval:         u32,  // default: 100
    pub tension_check_interval: u32,  // default: 10
    pub goal_check_interval:    u32,  // default: 10
    pub reconcile_interval:     u32,  // default: 200
}

impl Default for TickSchedule {
    fn default() -> Self {
        Self {
            adaptation_interval:    50,
            horizon_gc_interval:    500,
            snapshot_interval:      5000,
            dream_interval:         100,
            tension_check_interval: 10,
            goal_check_interval:    10,
            reconcile_interval:     200,
        }
    }
}
```

Три пресета (в YAML или константах): `weak` (×2 от default), `medium` (default), `strong` (÷2 от default).

**6B. tick_count в AxiomEngine**

```rust
pub struct AxiomEngine {
    // ...
    pub(crate) tick_count: u64,
    pub(crate) tick_schedule: TickSchedule,
}
```

**6C. Интеграция в handle_tick_forward**

```rust
fn handle_tick_forward(&mut self, cmd: &UclCommand) -> UclResult {
    self.tick_count += 1;
    let t = self.tick_count;
    let s = &self.tick_schedule;

    // Горячий путь — каждый тик
    let events = self.ashti.tick();
    self.pending_events.extend(events);

    // Тёплый путь
    if t % s.tension_check_interval as u64 == 0 {
        self.check_tension_traces();
    }
    if t % s.goal_check_interval as u64 == 0 {
        self.check_active_goals();
    }
    if t % s.dream_interval as u64 == 0 {
        self.run_dream_propose();
    }
    if t % s.adaptation_interval as u64 == 0 {
        self.run_adaptation();
    }

    // Холодный путь
    if t % s.reconcile_interval as u64 == 0 {
        self.ashti.reconcile_all();
    }
    if t % s.horizon_gc_interval as u64 == 0 {
        self.run_horizon_gc();
    }
    if t % s.snapshot_interval as u64 == 0 {
        self.snapshot_and_prune();
    }

    let count = self.pending_events.len() as u16;
    make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, count)
}
```

**Важно:** Перед интеграцией проверить что `check_tension_traces`, `check_active_goals`, `run_dream_propose` уже существуют как методы или их нужно создать как обёртки над существующим кодом.

**Тесты к Фазе 6:**
- `tick_count` инкрементируется корректно
- `adaptation_interval: 50` → `run_adaptation` вызывается на тиках 50, 100, 150
- `adaptation_interval: 1` → каждый тик (обратная совместимость)
- Пресет `Default` совпадает с medium

**Бенчмарк к Фазе 6:**
- `TickForward` с TickSchedule default — сравнить с текущим baseline

---

## Сводка

| Фаза | Что | Риск | Файлы |
|------|-----|------|-------|
| 1 | Unwrap cleanup | Низкий | experience.rs, lib.rs, loader.rs |
| 2 | EventType::Unknown | Низкий | event.rs |
| 3 | Event payload + source_domain | Низкий (размер не меняется) | event.rs, shell.rs |
| 4 | com_next_id в Snapshot | Низкий | engine.rs |
| 5 | Magic numbers → Config | Низкий | lib.rs, domain_config.rs, ucl/lib.rs |
| 6 | Tick Scheduling | Средний | engine.rs, (новый файл) |

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
