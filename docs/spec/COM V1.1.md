# AXIOM MODULE SPECIFICATION: COM V1.1

**Статус:** Актуальная спецификация (core)
**Версия:** 1.1.0
**Дата:** 2026-03-20
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
    pub event_id: u64,          // Монотонный причинный индекс (COM)
    pub parent_event_id: u64,   // Предыдущее событие в цепочке

    // --- СОДЕРЖАНИЕ [16 байт] ---
    pub payload_hash: u64,      // Хеш содержимого (валидация/детерминизм)
    pub target_id: u32,         // ID целевого объекта (Token/Connection)
    pub source_id: u32,         // ID источника

    // --- ИДЕНТИФИКАЦИЯ [8 байт] ---
    pub domain_id: u16,         // Домен события
    pub event_type: u16,        // Тип события (EventType enum)
    pub payload_size: u16,      // Размер payload в байтах (0..65535)
    pub priority: u8,           // Приоритет (0..255)
    pub flags: u8,              // Флаги (CRITICAL, REVERSIBLE, etc.)

    // --- HEARTBEAT [8 байт] ---
    pub pulse_id: u64,          // Номер пульса Heartbeat (0 = не привязан к пульсу)

    // --- РЕЗЕРВ [16 байт] ---
    pub _reserved: [u8; 16],    // Резерв для будущих расширений
}
// Итого: 16 + 16 + 8 + 8 + 16 = 64 байта
// Padding: 0 (поля упорядочены по убыванию размера внутри блоков)
```

### Изменения относительно V1.0:

1. **Размер**: 32 → 64 байта (полная кэш-линия для оптимизации доступа)
2. **Alignment**: 32 → 64 (совмещение с размером кэш-линии)
3. **Добавлено поле**: `pulse_id: u64` (интеграция с Heartbeat V2.0)
4. **Оптимизировано**: `payload_size: u32 → u16` (65535 байт достаточно для любого payload)
5. **Резерв**: `_reserved: [u8; 4] → [u8; 16]` (место для будущих расширений)
6. **Упорядочивание**: Поля переупорядочены по убыванию размера внутри логических блоков для исключения padding

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

- **V1.1**: Event расширен до 64 байт. Добавлен `pulse_id` (Heartbeat V2.0). `payload_size` сокращён до u16. Резерв 16 байт. Добавлены семантические типы событий Event-Driven V1.
- **V1.0**: Каноническая спецификация с полной структурой Event (32 байта)
- **V0.x**: Концептуальные описания без детальной реализации
