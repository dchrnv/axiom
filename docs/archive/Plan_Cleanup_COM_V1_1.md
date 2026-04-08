# План реализации: COM V1.1 + Cleanup + Tick Scheduling

**Версия:** 1.0  
**Дата:** 2026-04-02  
**Назначение:** Подготовка ядра AXIOM к живому использованию  
**Для:** Claude Sonnet (исполнитель)  
**Контекст:** Все этапы роадмапа V2.1 (1-8, 12A/12B) реализованы. Cognitive Depth V1.0 реализован. Система имеет 590+ тестов, бенчмарки v5 подтверждают стабильность. Этот план закрывает технический долг из DEFERRED.md и готовит ядро к CLI Channel.

---

## Порядок выполнения

| Фаза | Название | Что делаем | Риск |
|------|---------|-----------|------|
| 1 | Unsafe Unwrap Cleanup | Устранение паник на горячем пути | Низкий — точечные замены |
| 2 | EventType::Unknown | Защита от неизвестных типов событий | Низкий — добавление варианта |
| 3 | COM V1.1 — Event 64B | Расширение Event до 64 байт | **Средний** — структурное изменение |
| 4 | com_next_id в Snapshot | Корректное сохранение/восстановление COM-счётчика | Низкий |
| 5 | Magic Numbers → Config | Вынос захардкоженных порогов | Низкий |
| 6 | Tick Scheduling | Частотные делители для подсистем | Низкий — расширение |

**Принцип:** Каждая фаза — отдельный коммит. `cargo test --workspace` зелёный после каждой фазы.

---

## Фаза 1: Unsafe Unwrap Cleanup

**Цель:** Устранить все потенциальные паники из горячего пути.

### 1.1 experience.rs: `partial_cmp().unwrap()` → `f32::total_cmp`

**Где:** `crates/axiom-arbiter/src/experience.rs:173, 316`

**Текущий код:**
```rust
.min_by(|(_, a), (_, b)| a.weight.partial_cmp(&b.weight).unwrap())
.max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
```

**Замена:**
```rust
.min_by(|(_, a), (_, b)| a.weight.total_cmp(&b.weight))
.max_by(|a, b| a.weight.total_cmp(&b.weight))
```

`f32::total_cmp` стабилен с Rust 1.62. Определяет полный порядок: NaN > все значения. Никогда не паникует.

### 1.2 arbiter/lib.rs: `resonance.trace.unwrap()` на горячем пути

**Где:** `crates/axiom-arbiter/src/lib.rs:278, 386`

**Текущий код:**
```rust
let reflex_token = resonance.trace.as_ref().unwrap().pattern; // :278
Some(resonance.trace.as_ref().unwrap().pattern)               // :386
```

**Замена:**
```rust
// Вариант A: ранний выход (если контекст — функция возвращающая Option)
let reflex_token = resonance.trace.as_ref()?.pattern;

// Вариант B: если контекст не позволяет ? — if let
if let Some(trace) = resonance.trace.as_ref() {
    let reflex_token = trace.pattern;
    // ... продолжение логики
}
```

Выбрать вариант исходя из контекста вызова. Цель — **никогда не паниковать** если `trace` оказался `None`.

### 1.3 loader.rs: `key.as_str().unwrap()` при парсинге YAML

**Где:** `crates/axiom-config/src/loader.rs:289, 300`

**Текущий код:**
```rust
key.as_str().unwrap()                            // :289
self.validate_field(key.as_str().unwrap(), ...)  // :300
```

**Замена:**
```rust
// Вариант: пропустить невалидный ключ
let Some(key_str) = key.as_str() else { continue; };
self.validate_field(key_str, ...)
```

Или: вернуть ошибку конфигурации с описанием проблемы.

### Тесты:
- Добавить тест: `resonance_search` с `trace = None` — не паникует.
- Добавить тест: сортировка по weight с NaN в наборе — не паникует.
- Добавить тест: загрузка YAML с нестроковым ключом — ошибка, не паника.

---

## Фаза 2: EventType::Unknown

**Цель:** Неизвестный тип события не обрушивает процесс.

**Где:** `crates/axiom-core/src/event.rs:152`

**Текущий код:**
```rust
_ => panic!("Unknown event type: {:#06x}", v),
```

**Замена:**
```rust
#[repr(u16)]
pub enum EventType {
    // Token события (0x0000-0x0FFF)
    TokenCreate     = 0x0001,
    TokenUpdate     = 0x0002,
    TokenDelete     = 0x0003,
    TokenMove       = 0x0004,
    TokenTransform  = 0x0005,

    // Connection события (0x1000-0x1FFF)
    ConnectionCreate = 0x1001,
    ConnectionUpdate = 0x1002,
    ConnectionDelete = 0x1003,
    ConnectionStress = 0x1004,

    // Domain события (0x2000-0x2FFF)
    DomainCreate = 0x2001,
    DomainConfig = 0x2002,
    DomainReset  = 0x2003,

    // Guardian события (0x3000-0x3FFF)
    ReflexApproved   = 0x3001,
    ReflexVetoed     = 0x3002,
    PatternInhibited = 0x3003,
    CodexRuleUpdated = 0x3004,

    // Cognitive события (0x4000-0x4FFF) — для Cognitive Depth
    InternalImpulse  = 0x4001,
    TensionCreated   = 0x4002,
    CoherenceCheck   = 0x4003,
    GoalPersist      = 0x4004,

    // Gateway события (0x5000-0x5FFF) — зарезервированы для CLI Channel
    ShellExec        = 0x5001,
    MessageReceived  = 0x5002,
    MessageSent      = 0x5003,

    // Системные события (0xF000-0xFFFF)
    SystemCheckpoint = 0xF001,
    SystemRollback   = 0xF002,
    SystemShutdown   = 0xF003,

    // Неизвестный тип — безопасная обработка
    Unknown          = 0xFFFF,
}

impl From<u16> for EventType {
    fn from(v: u16) -> Self {
        match v {
            0x0001 => Self::TokenCreate,
            0x0002 => Self::TokenUpdate,
            // ... все известные типы ...
            _ => Self::Unknown,  // НЕ паника
        }
    }
}
```

**Важно:** Диапазоны типов (0x3000, 0x4000, 0x5000) — это **резервация**. Если какие-то из этих типов уже определены в коде под другими значениями, сохранить существующие значения. Если нет — использовать указанные диапазоны. Главное — `Unknown` вместо `panic!`.

### Обработка Unknown в pipeline:

Событие с типом `Unknown` не должно обрабатываться — оно логируется и пропускается:

```rust
EventType::Unknown => {
    // Логирование для диагностики (если есть механизм логов)
    // Пропустить — не применять к состоянию
    continue;
}
```

### Тесты:
- `EventType::from(0xBEEF)` → `Unknown` (не паника).
- Событие с типом `Unknown` в pipeline — пропускается, не ломает обработку.

---

## Фаза 3: COM V1.1 — Event 64B

**Цель:** Расширить Event до 64 байт (одна кэш-линия). Решает проблему ShellEffector payload и даёт место для причинных цепочек.

**Это СТРУКТУРНОЕ изменение.** Затрагивает все модули которые создают, читают или обрабатывают Event. Действовать аккуратно.

### 3.1 Новая структура Event

```rust
#[repr(C, align(64))]
pub struct Event {
    // === ИДЕНТИФИКАЦИЯ (16B) ===
    pub event_id: u64,            // 8B  — монотонный причинный индекс
    pub parent_event_id: u64,     // 8B  — предыдущее событие в цепочке (НОВОЕ в коде)

    // === КЛАССИФИКАЦИЯ (8B) ===
    pub domain_id: u16,           // 2B  — целевой домен
    pub event_type: u16,          // 2B  — тип события (EventType enum)
    pub priority: u8,             // 1B  — приоритет (0..255)
    pub flags: u8,                // 1B  — CRITICAL, REVERSIBLE, INTERNAL и т.д.
    pub source_domain: u16,       // 2B  — НОВОЕ: домен-источник (для GUARDIAN enforce_protocol)

    // === ЦЕЛИ (8B) ===
    pub target_id: u32,           // 4B  — ID целевого объекта (Token/Connection)
    pub source_id: u32,           // 4B  — ID источника

    // === СОДЕРЖАНИЕ (16B) ===
    pub payload_hash: u64,        // 8B  — хеш содержимого (валидация/детерминизм)
    pub payload: [u8; 8],         // 8B  — НОВОЕ: inline payload (structured данные)

    // === МЕТАДАННЫЕ (16B) ===
    pub payload_size: u32,        // 4B  — размер внешних данных (если payload — индекс в буфер)
    pub snapshot_event_id: u32,   // 4B  — ID снапшота (для отслеживания причинного горизонта)
    pub _reserved: [u8; 8],       // 8B  — резерв на будущее
}
// Итого: 16 + 8 + 8 + 16 + 16 = 64 байта
```

### 3.2 Порядок полей — для repr(C, align(64))

Поля размещены от крупных к мелким внутри логических блоков. Проверить что `size_of::<Event>() == 64` и `align_of::<Event>() == 64` — добавить `static_assert` (compile-time проверку):

```rust
const _: () = assert!(std::mem::size_of::<Event>() == 64);
const _: () = assert!(std::mem::align_of::<Event>() == 64);
```

### 3.3 Новые поля — семантика

**`parent_event_id: u64`**  
Причинная цепочка. Инвариант: `parent_event_id < event_id` (или 0 для корневых событий). Позволяет восстанавливать причинно-следственные цепочки при debug и rollback.

**`source_domain: u16`**  
Домен, породивший событие. Пример: событие `TokenMove` в домене `EXECUTION(1)` может быть инициировано из `PROBE(5)`. `domain_id = 1` (где произошло), `source_domain = 5` (кто инициировал). GUARDIAN использует пару `(source_domain, domain_id)` для `enforce_protocol()`.

Для событий без явного источника: `source_domain = domain_id`.

**`payload: [u8; 8]`**  
Inline structured данные. Интерпретация зависит от `event_type`:

| event_type | payload содержит |
|---|---|
| TokenMove | `[dx: i16, dy: i16, dz: i16, _pad: u16]` — дельта перемещения |
| ShellExec | `[command_index: u16, arg1: u16, arg2: u16, _pad: u16]` — индекс команды в таблице |
| InternalImpulse | `[impulse_type: u8, intensity: u8, source_trace: u32, _pad: u16]` |
| Другие | Зависит от контекста; по умолчанию `[0u8; 8]` |

**Для ShellEffector:** `command_index` — индекс в таблице команд (side-channel буфер в Gateway). Event не несёт строку — только индекс. Gateway хранит маппинг `command_index → String`. Это сохраняет принцип "ядро не знает про строки".

**`payload_size: u32`**  
Размер данных за пределами inline payload. Если `payload_size > 0`, `payload` содержит offset/index в внешний буфер. Если `payload_size == 0`, всё в inline payload.

**`snapshot_event_id: u32`**  
ID последнего снапшота, известного на момент создания события. Используется при Causal Horizon: события с `snapshot_event_id < current_snapshot` безопасны для архивации.

**`_reserved: [u8; 8]`**  
Резерв. Заполняется нулями. Не использовать до следующей ревизии спецификации.

### 3.4 Обновление Event::new()

Минимальный конструктор должен инициализировать новые поля нулями:

```rust
impl Event {
    pub fn new(event_id: u64, domain_id: u16, event_type: EventType) -> Self {
        Self {
            event_id,
            parent_event_id: 0,
            domain_id,
            event_type: event_type as u16,
            priority: 0,
            flags: 0,
            source_domain: domain_id,  // по умолчанию = целевой домен
            target_id: 0,
            source_id: 0,
            payload_hash: 0,
            payload: [0u8; 8],
            payload_size: 0,
            snapshot_event_id: 0,
            _reserved: [0u8; 8],
        }
    }
}
```

### 3.5 Новый флаг в flags

Добавить бит для маркировки внутренних импульсов (Cognitive Depth):

```rust
pub const FLAG_CRITICAL:   u8 = 0b0000_0001;
pub const FLAG_REVERSIBLE: u8 = 0b0000_0010;
pub const FLAG_INTERNAL:   u8 = 0b0000_0100;  // НОВОЕ: событие от Internal Drive
pub const FLAG_BATCH:      u8 = 0b0000_1000;  // НОВОЕ: батч-событие (Storm Control)
// биты 4-7 — резерв
```

### 3.6 Обновление всех мест создания Event

**Это основная работа.** Найти все вызовы `Event::new(...)` или конструкции `Event { ... }` во всех crates и обновить:

1. `axiom-core` — определение структуры.
2. `axiom-domain` — EventGenerator (`check_decay`, `generate_gravity_update`, `generate_collision`).
3. `axiom-arbiter` — события маршрутизации.
4. `axiom-runtime` — Guardian события (`ReflexApproved`, `ReflexVetoed`, и т.д.).
5. `axiom-frontier` — обработка событий в pipeline.
6. `axiom-heartbeat` — генерация Heartbeat-событий.
7. `axiom-agent` (если есть) — Gateway события.

**Стратегия:** Поскольку `Event::new()` уже инициализирует все поля нулями, большинство мест где вызывается `Event::new(event_id, domain_id, event_type)` — не нужно менять. Нужно только обновить те места, которые обращаются к `_reserved` или к полям, которые переименованы/перемещены.

Провести `grep -rn "Event {" crates/` и `grep -rn "Event::new" crates/` для полного списка.

### 3.7 Обновление бенчмарков

`Event::new` бенчмарк должен отражать новый размер. Ожидаемое время: ~24-30 ns (как текущее — Event уже 64B в бенчмарке, или рядом).

### 3.8 Обновление ShellEffector

Заменить заглушку `extract_command()`:

```rust
fn extract_command(event: &Event) -> Option<u16> {
    if event.event_type == EventType::ShellExec as u16 {
        // Первые 2 байта payload — command_index
        let command_index = u16::from_le_bytes([event.payload[0], event.payload[1]]);
        if command_index > 0 {
            Some(command_index)
        } else {
            None
        }
    } else {
        None
    }
}
```

Полноценный маппинг `command_index → команда` живёт в Gateway (side-channel буфер). Ядро возвращает `command_index`, Gateway транслирует в строку и исполняет.

### Тесты:
- `size_of::<Event>() == 64` (compile-time assert).
- `Event::new()` с последующим чтением всех полей.
- `parent_event_id` инвариант: `parent < event_id` (тест валидации).
- `source_domain` корректно устанавливается.
- `payload` корректно записывается и читается для разных event_type.
- `extract_command()` возвращает `Some(index)` для ShellExec и `None` для остальных.
- Все существующие тесты проходят без изменений (обратная совместимость через конструктор).

### Бенчмарки:
- `Event::new` — целевое время ≤ 30 ns.
- AshtiCore pipeline — проверить что overhead от расширения Event минимален.

---

## Фаза 4: com_next_id в Snapshot

**Цель:** Корректное сохранение и восстановление COM-счётчика при snapshot/restore.

### 4.1 Добавить com_next_id в AxiomEngine

**Где:** `crates/axiom-runtime/src/engine.rs`

```rust
pub struct AxiomEngine {
    // ... существующие поля ...
    pub(crate) com_next_id: u64,  // НОВОЕ: текущий COM-счётчик
}
```

Все места генерации `event_id` должны брать значение из `self.com_next_id` и инкрементировать:

```rust
fn next_event_id(&mut self) -> u64 {
    let id = self.com_next_id;
    self.com_next_id += 1;
    id
}
```

Если `event_id` сейчас генерируется через `AtomicU64` или другой механизм — заменить на поле в Engine (ядро однопоточное, атомики не нужны).

### 4.2 Обновить EngineSnapshot

**Где:** `crates/axiom-runtime/src/engine.rs:133`

```rust
pub struct EngineSnapshot {
    pub domains: Vec<DomainState>,
    pub com_next_id: u64,   // ИСПРАВЛЕНО: был 0
    pub created_at: u64,     // event_id на момент снапшота
}
```

При создании снапшота:
```rust
EngineSnapshot {
    domains,
    com_next_id: self.com_next_id,  // Сохраняем текущий счётчик
    created_at: self.com_next_id,    // или horizon, как сейчас
}
```

При восстановлении:
```rust
fn restore_from(&mut self, snapshot: EngineSnapshot) {
    // ... восстановление доменов ...
    self.com_next_id = snapshot.com_next_id;  // Восстанавливаем счётчик
}
```

### Тесты:
- Создать Engine → inject tokens → snapshot → inject ещё → restore → проверить что `com_next_id` восстановлен корректно.
- После restore: новые события имеют `event_id > snapshot.com_next_id` (монотонность).
- Специально проверить: `event_id` не коллидируют после restore.

---

## Фаза 5: Magic Numbers → Config

**Цель:** Вынести захардкоженные пороги сравнения токенов в конфигурацию.

### 5.1 compare_tokens пороги

**Где:** `crates/axiom-arbiter/src/lib.rs:536-538`

**Текущий код:**
```rust
let temp_match    = (...).abs() < 10;
let mass_match    = (...).abs() < 5;
let valence_match = (...).abs() < 2;
```

**Решение:** Добавить поля в DomainConfig (или в конфиг Arbiter):

```rust
pub struct TokenCompareConfig {
    pub temp_tolerance: i16,     // default: 10
    pub mass_tolerance: i16,     // default: 5
    pub valence_tolerance: i16,  // default: 2
}
```

Добавить в существующий конфиг (DomainConfig или ArbiterConfig — выбрать по контексту). Если в DomainConfig — это позволяет разным доменам иметь разную чувствительность сравнения.

Если добавлять в DomainConfig — внимание на размер структуры (128B, repr(C, align(64))). Если места нет — использовать глобальный ArbiterConfig.

### 5.2 UclBuilder::spawn_domain structural_role

**Где:** `crates/axiom-ucl/src/lib.rs:258`

Если `structural_role` семантически совпадает с `factory_preset` — удалить дублирование и использовать одно поле. Если не совпадает — определить таблицу маппинга.

Это low-priority, но если делать cleanup — сделать и это.

### Тесты:
- `compare_tokens` с default tolerance — результат совпадает с текущим поведением.
- `compare_tokens` с нулевой tolerance — только точное совпадение.

---

## Фаза 6: Tick Scheduling (Частотные делители)

**Цель:** Разные подсистемы работают на разных частотах. Горячий путь — каждый тик. Тёплый и холодный — реже.

### 6.1 TickSchedule в RuntimeConfig

Добавить структуру конфигурации частот:

```rust
pub struct TickSchedule {
    /// Адаптация порогов Arbiter (run_adaptation). Default: 50
    pub adaptation_interval: u32,

    /// Causal Horizon GC (run_horizon_gc). Default: 500
    pub horizon_gc_interval: u32,

    /// Snapshot + prune. Default: 5000
    pub snapshot_interval: u32,

    /// DREAM(7) фоновый анализ. Default: 100
    pub dream_interval: u32,

    /// Проверка tension traces (Cognitive Depth). Default: 10
    pub tension_check_interval: u32,

    /// Проверка активных целей (Cognitive Depth). Default: 10
    pub goal_check_interval: u32,

    /// Полная Shell reconciliation. Default: 200
    pub reconcile_interval: u32,
}

impl Default for TickSchedule {
    fn default() -> Self {
        Self {
            adaptation_interval: 50,
            horizon_gc_interval: 500,
            snapshot_interval: 5000,
            dream_interval: 100,
            tension_check_interval: 10,
            goal_check_interval: 10,
            reconcile_interval: 200,
        }
    }
}
```

### 6.2 Счётчик тиков в AxiomEngine

```rust
pub struct AxiomEngine {
    // ... существующие поля ...
    pub(crate) tick_count: u64,
    pub(crate) tick_schedule: TickSchedule,
}
```

### 6.3 Интеграция в главный цикл

В `TickForward` (или где сейчас происходит основной цикл обработки):

```rust
fn tick_forward(&mut self) {
    self.tick_count += 1;
    let t = self.tick_count;
    let s = &self.tick_schedule;

    // === ГОРЯЧИЙ ПУТЬ (каждый тик) ===
    self.process_frontier();          // CausalFrontier: pop + evaluate
    self.process_pending_commands();  // UCL commands
    self.shell_incremental_update();  // Shell dirty-токены

    // === ТЁПЛЫЙ ПУТЬ (периодически) ===
    if t % s.tension_check_interval as u64 == 0 {
        self.check_tension_traces();  // Cognitive Depth
    }
    if t % s.goal_check_interval as u64 == 0 {
        self.check_active_goals();    // Cognitive Depth
    }
    if t % s.dream_interval as u64 == 0 {
        self.run_dream_analysis();    // DREAM(7) фоновый
    }
    if t % s.adaptation_interval as u64 == 0 {
        self.run_adaptation();        // Адаптивные пороги
    }

    // === ХОЛОДНЫЙ ПУТЬ (редко) ===
    if t % s.reconcile_interval as u64 == 0 {
        self.shell_reconcile();       // Полный пересчёт Shell
    }
    if t % s.horizon_gc_interval as u64 == 0 {
        self.run_horizon_gc();        // Causal Horizon GC
    }
    if t % s.snapshot_interval as u64 == 0 {
        self.snapshot_and_prune();    // Snapshot + pruning
    }
}
```

**Важно:** Если какие-то из этих вызовов сейчас делаются в других местах (например в Heartbeat callback, или в отдельных методах) — не дублировать. Перенести логику в единый планировщик или оставить в Heartbeat но с учётом `tick_count`.

**Альтернатива:** Если текущая архитектура уже использует Heartbeat для планирования периодических задач — можно интегрировать `TickSchedule` в Heartbeat вместо AxiomEngine. Главное — единая точка конфигурации частот.

### 6.4 Конфигурация через YAML

Добавить в `config/runtime.yaml` (или в существующий конфиг):

```yaml
tick_schedule:
  adaptation_interval: 50
  horizon_gc_interval: 500
  snapshot_interval: 5000
  dream_interval: 100
  tension_check_interval: 10
  goal_check_interval: 10
  reconcile_interval: 200
```

Три пресета для разного железа:

```yaml
# Weak hardware (Ryzen 5 3500U, 2GB RAM) — текущий таргет
tick_schedule:
  adaptation_interval: 100    # реже
  horizon_gc_interval: 1000   # реже
  snapshot_interval: 10000    # реже
  dream_interval: 200         # реже
  tension_check_interval: 20
  goal_check_interval: 20
  reconcile_interval: 500

# Medium hardware
tick_schedule:
  adaptation_interval: 50
  horizon_gc_interval: 500
  snapshot_interval: 5000
  dream_interval: 100
  tension_check_interval: 10
  goal_check_interval: 10
  reconcile_interval: 200

# Strong hardware
tick_schedule:
  adaptation_interval: 20
  horizon_gc_interval: 200
  snapshot_interval: 2000
  dream_interval: 50
  tension_check_interval: 5
  goal_check_interval: 5
  reconcile_interval: 100
```

### 6.5 Влияние на бюджет тика

Текущая ситуация (если всё каждый тик):
```
AshtiCore::process   40 µs
run_adaptation       30 µs
run_horizon_gc       30 µs
snapshot_and_prune   40 µs
Shell reconcile      ~3 µs
                     -------
Итого:              ~143 µs (14.3% бюджета при 1000 Hz)
```

С Tick Scheduling (weak preset):
```
Типичный тик:
  process_frontier    ~50 µs  (только если есть активные события)
  Shell incremental   ~3 µs
                      -------
Итого:               ~53 µs  (5.3% бюджета)

Каждый 20-й тик: + tension/goal check  ~200 ns
Каждый 100-й тик: + adaptation          ~30 µs  (= 83 µs тик)
Каждый 200-й тик: + DREAM               ~depends
Каждый 500-й тик: + reconcile           ~3 µs
Каждый 1000-й тик: + horizon_gc         ~30 µs
Каждый 10000-й тик: + snapshot          ~40 µs
```

Типичный тик экономит ~90 µs по сравнению с "всё каждый раз". На слабом железе это критично.

### Тесты:
- `tick_count` корректно инкрементируется.
- `adaptation_interval: 50` → `run_adaptation` вызывается на тиках 50, 100, 150...
- `adaptation_interval: 1` → `run_adaptation` каждый тик (обратная совместимость).
- Все периодические задачи корректно срабатывают.
- Пресет `Default` совпадает с medium hardware.

### Бенчмарки:
- `TickForward` с TickSchedule (weak preset) — измерить типичный тик.
- Сравнить с текущим TickForward (без scheduling).

---

## Справка: MLEngine input_size/output_size = 0

**Статус:** Отложен на неопределённый срок. НЕ реализовывать сейчас.

**Контекст для будущего:**

Проблема в `crates/axiom-agent/src/ml/engine.rs:120-123`. При загрузке ONNX-модели через `tract` размеры тензоров не извлекаются из model facts — оба размера остаются 0.

```rust
Ok(MLEngine::Real {
    model: Box::new(model),
    input_size: 0,  // Должно быть: model.input_fact(0).shape
    output_size: 0, // Должно быть: model.output_fact(0).shape
})
```

Последствие: проверка `if *input_size > 0` никогда не срабатывает, что скрывает ShapeMismatch-ошибки. При реальном использовании ML-моделей это приведёт к молчаливым ошибкам.

**Когда исправлять:** Когда появится первая реальная ONNX-модель для инференса.

**Как исправлять:**
```rust
let input_fact = model.input_fact(0)
    .map_err(|e| MLError::ModelLoad(format!("No input fact: {}", e)))?;
let input_size = input_fact.shape.as_concrete()
    .map(|s| s.iter().product::<usize>())
    .unwrap_or(0);

// Аналогично для output_fact(0)
```

---

## Итоговый чеклист

После выполнения всех фаз:

- [ ] `cargo test --workspace` зелёный (все существующие + новые тесты)
- [ ] `size_of::<Event>() == 64` (compile-time assert)
- [ ] Никаких `unwrap()` на горячем пути в arbiter и experience
- [ ] `EventType::Unknown` вместо `panic!`
- [ ] `com_next_id` сохраняется в snapshot и восстанавливается
- [ ] `ShellEffector::extract_command()` возвращает `Option<u16>` (не всегда None)
- [ ] Magic numbers вынесены в конфиг
- [ ] `TickSchedule` интегрирован, периодические задачи не выполняются каждый тик
- [ ] Бенчмарки обновлены для Event 64B и TickSchedule
- [ ] DEFERRED.md обновлён: закрытые пункты помечены ✅ с датой

---

## История изменений

- **V1.0**: Первая версия. 6 фаз: unwrap cleanup, EventType::Unknown, COM V1.1, com_next_id, magic numbers, tick scheduling.
