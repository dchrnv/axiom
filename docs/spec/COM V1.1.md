# AXIOM MODULE SPECIFICATION: COM V1.2

**Статус:** Актуальная спецификация (core)
**Версия:** 1.2.0
**Дата:** 2026-04-10
**Название:** Causal Order Model (Причинно-следственная модель порядка)
**Формат:** 64 байта на событие, `repr(C, align(64))`
**Связанные спеки:** Token V5.2, Connection V5.0, Domain V1.2, UPO v2.3, Heartbeat V2.0

---

## 1. Назначение

**COM (Causal Order Model)** — модель времени как упорядоченности изменений, заменяющая глобальную временную ось на причинный порядок событий:

- обеспечивает детерминированную последовательность изменений,
- гарантирует монотонность и воспроизводимость,
- позволяет реконструировать любое состояние из начального,
- обеспечивает синхронизацию между модулями без wall-clock времени.

COM **не использует wall-clock время**. Время определяется как **порядок применения событий**.

---

## 2. Основной принцип

Вместо глобальной временной оси вводится **монотонный причинный порядок изменений**:

```
stateₙ₊₁ = F(stateₙ, eventₙ)
```

где `n` — порядковый номер события, `event_id` — монотонно возрастающий идентификатор.

---

## 3. Структура Event (64 байта)

Event увеличен до 64 байт (одна кэш-линия) для размещения `pulse_id` из Heartbeat V2.0 и резерва для будущих расширений.

```rust
/// COM Event — 64 байта, одна кэш-линия
#[repr(C, align(64))]
pub struct Event {
    // --- ПРИЧИННОСТЬ [16 байт] ---
    pub event_id: u64,           // Монотонный причинный индекс (COM)
    pub parent_event_id: u64,    // Предыдущее событие в цепочке

    // --- СОДЕРЖАНИЕ [16 байт] ---
    pub payload_hash: u64,       // Хеш содержимого (валидация/детерминизм)
    pub target_id: u32,          // ID целевого объекта (Token/Connection)
    pub source_id: u32,          // ID источника

    // --- ИДЕНТИФИКАЦИЯ [8 байт] ---
    pub domain_id: u16,          // Домен-цель события
    pub event_type: u16,         // Тип события (EventType enum)
    pub payload_size: u16,       // Размер payload в байтах (0..65535)
    pub priority: u8,            // Приоритет (0..255)
    pub flags: u8,               // Флаги (CRITICAL, REVERSIBLE, etc.)

    // --- HEARTBEAT [8 байт] ---
    pub pulse_id: u64,           // Номер пульса Heartbeat (0 = не привязан к пульсу)

    // --- РАСШИРЕНИЕ [16 байт] ---
    pub source_domain: u16,      // Домен-инициатор (≠ domain_id только для PROBE → EXECUTION)
    pub event_subtype: u16,      // Второй уровень классификации (0 = SUBTYPE_NONE)
    pub snapshot_event_id: u32,  // event_id снапшота на момент создания (Causal Horizon)
    pub payload: [u8; 8],        // Inline payload (интерпретация зависит от event_type)
}
// Итого: 16 + 16 + 8 + 8 + 16 = 64 байта
```

### Поле `event_subtype`

Второй уровень классификации внутри одного `event_type`. Устанавливается вручную после создания события (не входит в `Event::new()`). По умолчанию `0` (`SUBTYPE_NONE`) — обратная совместимость со всем существующим кодом.

#### Подтипы для TokenMove / TokenMoved

| Константа | Значение | Семантика |
|---|---|---|
| `SUBTYPE_NONE` | 0 | Не указан |
| `SUBTYPE_GRAVITY` | 1 | Движение от гравитации |
| `SUBTYPE_MANUAL` | 2 | Ручное перемещение (ApplyForce) |
| `SUBTYPE_COLLISION` | 3 | Отскок от столкновения |
| `SUBTYPE_IMPULSE` | 4 | Внутренний импульс (Cognitive Depth) |
| `SUBTYPE_INERTIA` | 5 | Инерционное движение |
| `SUBTYPE_ATTRACTOR` | 6 | Движение к target |

#### Подтипы для ConnectionCreate

| Константа | Значение | Семантика |
|---|---|---|
| `SUBTYPE_RESONANCE` | 1 | Связь создана резонансом |
| `SUBTYPE_COLLISION_LINK` | 2 | Связь создана столкновением |
| `SUBTYPE_IMPORTED` | 3 | Связь импортирована (persistence) |

#### Подтипы для SystemCheckpoint

| Константа | Значение | Семантика |
|---|---|---|
| `SUBTYPE_MANUAL_SAVE` | 1 | Ручное сохранение (`:save`) |
| `SUBTYPE_AUTO_SAVE` | 2 | Автосохранение |
| `SUBTYPE_SHUTDOWN_SAVE` | 3 | Сохранение при `:quit` |

### Поле `payload`

Inline payload — 8 байт, структурированных данных. Интерпретация по `event_type`:

| event_type | Структура payload |
|---|---|
| `ShellExec` | `[command_index: u16 LE, _: 6]` |
| `InternalImpulse` | `[impulse_type: u8, intensity: u8, source_trace: u32 LE, _: 2]` |
| `TokenMove` | `[dx: i16 LE, dy: i16 LE, dz: i16 LE, _: 2]` |
| Остальные | `[0u8; 8]` |

### Изменения относительно V1.0:

1. **Размер**: 32 → 64 байта (полная кэш-линия)
2. **Alignment**: 32 → 64
3. **Добавлен** `pulse_id: u64` (Heartbeat V2.0)
4. **Оптимизирован** `payload_size: u32 → u16`
5. **Резерв заменён** расширением: `source_domain`, `event_subtype`, `snapshot_event_id`, `payload`

---

## 4. Типы событий

```rust
#[repr(u16)]
pub enum EventType {
    // Token события (0x0000-0x0FFF)
    TokenCreate = 0x0001,
    TokenUpdate = 0x0002,
    TokenDelete = 0x0003,
    TokenMove = 0x0004,
    TokenTransform = 0x0005,
    TokenDecayed = 0x0006,      // Затухание токена (Event-Driven V1)
    TokenMerged = 0x0007,       // Слияние токенов (Event-Driven V1)
    TokenSplit = 0x0008,        // Разделение токена (Event-Driven V1)
    TokenActivated = 0x0009,    // Активация токена (Event-Driven V1)
    TokenDeactivated = 0x000A,  // Деактивация токена (Event-Driven V1)
    TokenFrozen = 0x000B,       // Заморозка токена (Event-Driven V1)
    TokenThawed = 0x000C,       // Разморозка токена (Event-Driven V1)

    // Connection события (0x1000-0x1FFF)
    ConnectionCreate = 0x1001,
    ConnectionUpdate = 0x1002,
    ConnectionDelete = 0x1003,
    ConnectionStress = 0x1004,
    ConnectionWeakened = 0x1005,    // Ослабление связи (Event-Driven V1)
    ConnectionStrengthened = 0x1006, // Усиление связи (Event-Driven V1)
    ConnectionBroken = 0x1007,      // Разрыв связи (Event-Driven V1)
    ConnectionFormed = 0x1008,      // Формирование новой связи (Event-Driven V1)

    // Domain события (0x2000-0x2FFF)
    DomainCreate = 0x2001,
    DomainConfig = 0x2002,
    DomainReset = 0x2003,

    // Physics события (0x3000-0x3FFF)
    Heartbeat = 0x3001,             // Пульс Heartbeat (Heartbeat V2.0)
    GravityUpdate = 0x3002,         // Обновление гравитации (Event-Driven V1)
    CollisionDetected = 0x3003,     // Обнаружено столкновение (Event-Driven V1)
    ResonanceTriggered = 0x3004,    // Триггер резонанса (Event-Driven V1)
    ThermodynamicsUpdate = 0x3005,  // Обновление температуры (Event-Driven V1)

    // Системные события (0xF000-0xFFFF)
    SystemCheckpoint = 0xF001,
    SystemRollback = 0xF002,
    SystemShutdown = 0xF003,
}
```

---

## 5. Инварианты

1. **Монотонность**: `event_id` строго возрастает
2. **Детерминизм**: Одинаковые `payload_hash` → одинаковые изменения
3. **Целостность**: `parent_event_id` всегда < `event_id`
4. **Доменная изоляция**: События разных доменов независимы
5. **Приоритет**: Higher priority события обрабатываются первыми
6. **Heartbeat синхронизация**: `pulse_id = 0` означает отсутствие привязки к пульсу

---

## 6. Жизненный цикл события

1. **Генерация**: Модуль создает событие с уникальным `event_id`
2. **Валидация**: Проверяется `payload_hash` и целостность
3. **Применение**: Событие применяется к состоянию
4. **Фиксация**: Событие записывается в лог COM
5. **Распространение**: Зависимые модули уведомляются

---

## 7. COM Timeline

```rust
#[repr(C)]
pub struct Timeline {
    pub current_event_id: u64,     // Текущий максимум
    pub domain_offsets: [u64; 256], // Смещения по доменам
    pub checkpoint_id: u64,        // ID последней контрольной точки
    pub total_events: u64,         // Общее количество событий
}
```

---

## 8. Взаимодействия

### 8.1 С Token
- `Token.last_event_id` синхронизируется с COM
- Все изменения Token генерируют COM события
- `Token.momentum` обновляется через последовательность событий

### 8.2 С Connection
- `Connection.created_at` хранит COM event_id
- Изменения stress/strength генерируют события
- Gates фильтруют события на основе свойств

### 8.3 С Domain
- Domain имеет собственный event_id offset
- `Domain.config` изменения генерируют события
- `Domain.reset` создает новую точку в timeline

### 8.4 С Heartbeat
- Heartbeat генерирует события типа `EventType::Heartbeat`
- `pulse_id` связывает события с конкретным пульсом
- События без привязки к пульсу имеют `pulse_id = 0`

---

## 9. Восстановление состояния

```rust
fn restore_state(initial_state: State, events: &[Event]) -> State {
    let mut state = initial_state;
    for event in events {
        if validate_event(event) {
            state = apply_event(state, event);
        }
    }
    state
}
```

---

## 10. Валидация

```rust
fn validate_event(event: &Event, timeline: &Timeline) -> bool {
    event.event_id <= timeline.current_event_id
    && event.parent_event_id < event.event_id
    && event.payload_hash != 0
    && validate_event_type(event.event_type)
}
```

---

## 11. Оптимизации

1. **Batch processing**: Группировка событий по домену
2. **Compression**: Сжатие последовательных payload
3. **Indexing**: Быстрый доступ по target_id и временным диапазонам
4. **Checkpointing**: Периодическое сохранение состояния
5. **Cache-line alignment**: 64-байтовое выравнивание для оптимизации CPU cache

---

## 12. История изменений

- **V1.2**: Расширение `_reserved[16]` заменено конкретными полями: `source_domain`, `event_subtype`, `snapshot_event_id`, `payload`. Добавлен раздел `event_subtype` с таблицами констант (D-02).
- **V1.1**: Event расширен до 64 байт. Добавлен `pulse_id` (Heartbeat V2.0). `payload_size` сокращён до u16. Резерв 16 байт. Добавлены семантические типы событий Event-Driven V1.
- **V1.0**: Каноническая спецификация с полной структурой Event (32 байта)
- **V0.x**: Концептуальные описания без детальной реализации
