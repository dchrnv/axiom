# AXIOM MODULE SPECIFICATION: COM V1.0

**Статус:** Актуальная спецификация (core)  
**Версия:** 1.0.0  
**Дата:** 2026-03-04  
**Название:** Causal Order Model (Причинно-следственная модель порядка)  
**Формат:** 32 байта на событие, `repr(C, align(32))`  
**Связанные спеки:** Token V5.1, Connection V5.0, Domain V1.2, UPO v2.1

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

## 3. Структура Event (32 байта)

```rust
#[repr(C, align(32))]
pub struct Event {
    // --- ИДЕНТИФИКАТОР (8 Байт) ---
    pub event_id: u64,        // Монотонный причинный индекс
    pub domain_id: u16,       // Домен события
    pub event_type: u16,      // Тип события
    pub priority: u8,         // Приоритет (0..255)
    pub flags: u8,            // Флаги (CRITICAL, REVERSIBLE, etc.)
    pub _reserved: [u8; 4],   // Резерв

    // --- СОДЕРЖАНИЕ (16 Байт) ---
    pub payload_hash: u64,    // Хеш содержимого (валидация/детерминизм)
    pub target_id: u32,       // ID целевого объекта (Token/Connection)
    pub source_id: u32,       // ID источника (если применимо)
    pub payload_size: u32,    // Размер данных в байтах

    // --- МЕТАДАННЫЕ (8 Байт) ---
    pub parent_event_id: u64,  // Предыдущее событие в цепочке
}
```

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

    // Connection события (0x1000-0x1FFF)
    ConnectionCreate = 0x1001,
    ConnectionUpdate = 0x1002,
    ConnectionDelete = 0x1003,
    ConnectionStress = 0x1004,

    // Domain события (0x2000-0x2FFF)
    DomainCreate = 0x2001,
    DomainConfig = 0x2002,
    DomainReset = 0x2003,

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
- Token.last_event_id синхронизируется с COM
- Все изменения Token генерируют COM события
- Token.momentum обновляется через последовательность событий

### 8.2 С Connection
- Connection.created_at хранит COM event_id
- Изменения stress/strength генерируют события
- Gates фильтруют события на основе свойств

### 8.3 С Domain
- Domain имеет собственный event_id offset
- Domain.config изменения генерируют события
- Domain.reset создает новую точку в timeline

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
    event.event_id > timeline.current_event_id
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

---

## 12. История изменений

- **V1.0**: Каноническая спецификация с полной структурой Event
- **V0.x**: Концептуальные описания без детальной реализации
