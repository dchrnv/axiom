// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM V1.0: docs/spec/COM V1.0.md

/// Тип события (причинный порядок).
/// Event-Driven V1: семантические события физики и эволюции системы
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    // SPACE V6.0 события движения (0x0010-0x001F)
    TokenMoved = 0x0010,        // Токен переместился (SPACE V6.0)
    TokenCollision = 0x0011,    // Столкновение токенов (SPACE V6.0)
    TokenEnteredCell = 0x0012,  // Токен вошёл в новую ячейку (SPACE V6.0)

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
    Heartbeat = 0x3001,
    GravityUpdate = 0x3002,         // Обновление гравитации (Event-Driven V1)
    CollisionDetected = 0x3003,     // Обнаружено столкновение (Event-Driven V1)
    ResonanceTriggered = 0x3004,    // Триггер резонанса (Event-Driven V1)
    ThermodynamicsUpdate = 0x3005,  // Обновление температуры (Event-Driven V1)

    // Системные события (0xF000-0xFFFF)
    SystemCheckpoint = 0xF001,
    SystemRollback = 0xF002,
    SystemShutdown = 0xF003,
}

impl From<u16> for EventType {
    fn from(v: u16) -> Self {
        match v {
            0x0001 => EventType::TokenCreate,
            0x0002 => EventType::TokenUpdate,
            0x0003 => EventType::TokenDelete,
            0x0004 => EventType::TokenMove,
            0x0005 => EventType::TokenTransform,
            0x0006 => EventType::TokenDecayed,
            0x0007 => EventType::TokenMerged,
            0x0008 => EventType::TokenSplit,
            0x0009 => EventType::TokenActivated,
            0x000A => EventType::TokenDeactivated,
            0x000B => EventType::TokenFrozen,
            0x000C => EventType::TokenThawed,
            0x0010 => EventType::TokenMoved,
            0x0011 => EventType::TokenCollision,
            0x0012 => EventType::TokenEnteredCell,
            0x1001 => EventType::ConnectionCreate,
            0x1002 => EventType::ConnectionUpdate,
            0x1003 => EventType::ConnectionDelete,
            0x1004 => EventType::ConnectionStress,
            0x1005 => EventType::ConnectionWeakened,
            0x1006 => EventType::ConnectionStrengthened,
            0x1007 => EventType::ConnectionBroken,
            0x1008 => EventType::ConnectionFormed,
            0x2001 => EventType::DomainCreate,
            0x2002 => EventType::DomainConfig,
            0x2003 => EventType::DomainReset,
            0x3001 => EventType::Heartbeat,
            0x3002 => EventType::GravityUpdate,
            0x3003 => EventType::CollisionDetected,
            0x3004 => EventType::ResonanceTriggered,
            0x3005 => EventType::ThermodynamicsUpdate,
            0xF001 => EventType::SystemCheckpoint,
            0xF002 => EventType::SystemRollback,
            0xF003 => EventType::SystemShutdown,
            n => panic!("Unknown event type: {}", n),
        }
    }
}

/// Приоритеты событий
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum EventPriority {
    Low = 0,
    Normal = 128,
    High = 200,
    Critical = 255,
}

/// Флаги событий
pub const EVENT_REVERSIBLE: u8 = 1;
pub const EVENT_CRITICAL: u8 = 2;
pub const EVENT_BATCHED: u8 = 4;

/// COM Event — 64 байта, одна кэш-линия
/// Event-Driven V1 + Heartbeat V2.0 + COM V1.1
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Event {
    // --- ПРИЧИННОСТЬ [16 байт] ---
    pub event_id: u64,          // 8b  | Монотонный причинный индекс (COM)
    pub parent_event_id: u64,   // 8b  | Предыдущее событие в цепочке

    // --- СОДЕРЖАНИЕ [16 байт] ---
    pub payload_hash: u64,      // 8b  | Хеш содержимого (валидация/детерминизм)
    pub target_id: u32,         // 4b  | ID целевого объекта (Token/Connection)
    pub source_id: u32,         // 4b  | ID источника

    // --- ИДЕНТИФИКАЦИЯ [8 байт] ---
    pub domain_id: u16,         // 2b  | Домен события
    pub event_type: u16,        // 2b  | Тип события (EventType enum)
    pub payload_size: u16,      // 2b  | Размер payload (было u32 — u16 достаточно)
    pub priority: u8,           // 1b  | Приоритет (0..255)
    pub flags: u8,              // 1b  | Флаги (CRITICAL, REVERSIBLE, etc.)

    // --- HEARTBEAT [8 байт] ---
    pub pulse_id: u64,          // 8b  | Номер пульса (0 = не привязан к пульсу)

    // --- РЕЗЕРВ [16 байт] ---
    pub _reserved: [u8; 16],    // 16b | Резерв для будущих расширений
}

impl Event {
    pub fn new(
        event_id: u64,
        domain_id: u16,
        event_type: EventType,
        priority: EventPriority,
        payload_hash: u64,
        target_id: u32,
        source_id: u32,
        parent_event_id: u64,
    ) -> Self {
        Self {
            event_id,
            parent_event_id,
            payload_hash,
            target_id,
            source_id,
            domain_id,
            event_type: event_type as u16,
            payload_size: 0,
            priority: priority as u8,
            flags: 0,
            pulse_id: 0, // 0 означает "не привязано к пульсу"
            _reserved: [0; 16],
        }
    }

    /// Создает событие с привязкой к пульсу Heartbeat
    pub fn with_pulse(
        event_id: u64,
        domain_id: u16,
        event_type: EventType,
        priority: EventPriority,
        payload_hash: u64,
        target_id: u32,
        source_id: u32,
        parent_event_id: u64,
        pulse_id: u64,
    ) -> Self {
        Self {
            event_id,
            parent_event_id,
            payload_hash,
            target_id,
            source_id,
            domain_id,
            event_type: event_type as u16,
            payload_size: 0,
            priority: priority as u8,
            flags: 0,
            pulse_id,
            _reserved: [0; 16],
        }
    }

    /// Валидация согласно спецификации COM V1.0
    pub fn validate(&self, timeline: &Timeline) -> bool {
        self.event_id <= timeline.current_event_id
        && self.parent_event_id < self.event_id
        && self.payload_hash != 0
        && self.validate_event_type()
    }

    fn validate_event_type(&self) -> bool {
        matches!(EventType::from(self.event_type),
            EventType::TokenCreate | EventType::TokenUpdate | EventType::TokenDelete |
            EventType::TokenMove | EventType::TokenTransform | EventType::TokenDecayed |
            EventType::TokenMerged | EventType::TokenSplit | EventType::TokenActivated |
            EventType::TokenDeactivated | EventType::TokenFrozen | EventType::TokenThawed |
            EventType::ConnectionCreate | EventType::ConnectionUpdate |
            EventType::ConnectionDelete | EventType::ConnectionStress |
            EventType::ConnectionWeakened | EventType::ConnectionStrengthened |
            EventType::ConnectionBroken | EventType::ConnectionFormed |
            EventType::DomainCreate | EventType::DomainConfig | EventType::DomainReset |
            EventType::Heartbeat | EventType::GravityUpdate | EventType::CollisionDetected |
            EventType::ResonanceTriggered | EventType::ThermodynamicsUpdate |
            EventType::SystemCheckpoint | EventType::SystemRollback | EventType::SystemShutdown
        )
    }

    pub fn is_critical(&self) -> bool {
        self.flags & EVENT_CRITICAL != 0
    }

    pub fn is_reversible(&self) -> bool {
        self.flags & EVENT_REVERSIBLE != 0
    }
}

/// Timeline COM - управление событиями
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Timeline {
    pub current_event_id: u64,     // Текущий максимум
    pub domain_offsets: [u64; 256], // Смещения по доменам
    pub checkpoint_id: u64,        // ID последней контрольной точки
    pub total_events: u64,         // Общее количество событий
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            current_event_id: 0,
            domain_offsets: [0; 256],
            checkpoint_id: 0,
            total_events: 0,
        }
    }

    pub fn next_event_id(&mut self, domain_id: u16) -> u64 {
        self.current_event_id += 1;
        self.domain_offsets[domain_id as usize] = self.current_event_id;
        self.total_events += 1;
        self.current_event_id
    }

    pub fn create_checkpoint(&mut self) -> u64 {
        self.checkpoint_id = self.current_event_id;
        self.checkpoint_id
    }
}

/// Снапшот состояния после применения событий до snapshot_id.
/// Time Model V1.0: содержит только причинный порядок, НЕ wall-clock время
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Snapshot {
    pub snapshot_id: u64,  // Причинный порядок (event_id последнего события)
    pub state_hash: u64,   // Хеш состояния для валидации
    pub event_count: u32,  // Количество событий
    pub _reserved: u32,    // Резерв (было timestamp - УДАЛЕНО согласно Time Model V1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_size() {
        // Event — ровно 64 байта данных, выравнивание 64 (одна кэш-линия)
        // COM V1.1: оптимизированная структура без padding
        assert_eq!(std::mem::size_of::<Event>(), 64);
        assert_eq!(std::mem::align_of::<Event>(), 64);
    }

    #[test]
    fn test_semantic_event_types() {
        // Проверка новых семантических типов Event-Driven V1
        assert_eq!(EventType::TokenDecayed as u16, 0x0006);
        assert_eq!(EventType::TokenMerged as u16, 0x0007);
        assert_eq!(EventType::ConnectionWeakened as u16, 0x1005);
        assert_eq!(EventType::ConnectionBroken as u16, 0x1007);
        assert_eq!(EventType::GravityUpdate as u16, 0x3002);
        assert_eq!(EventType::CollisionDetected as u16, 0x3003);
    }

    #[test]
    fn test_event_creation_without_pulse() {
        let event = Event::new(
            1,
            0,
            EventType::TokenDecayed,
            EventPriority::Normal,
            0x1234567890ABCDEF,
            100,
            200,
            0,
        );

        assert_eq!(event.event_id, 1);
        assert_eq!(event.domain_id, 0);
        assert_eq!(event.event_type, EventType::TokenDecayed as u16);
        assert_eq!(event.pulse_id, 0); // Не привязано к пульсу
    }

    #[test]
    fn test_event_creation_with_pulse() {
        let event = Event::with_pulse(
            5,
            1,
            EventType::GravityUpdate,
            EventPriority::Low,
            0xABCDEF1234567890,
            150,
            250,
            4,
            42, // pulse_id
        );

        assert_eq!(event.event_id, 5);
        assert_eq!(event.pulse_id, 42); // Привязано к пульсу 42
        assert_eq!(event.event_type, EventType::GravityUpdate as u16);
    }

    #[test]
    fn test_snapshot_no_timestamp() {
        // Time Model V1.0: Snapshot не содержит wall-clock timestamp
        let snapshot = Snapshot {
            snapshot_id: 1000,
            state_hash: 0x123456789ABCDEF0,
            event_count: 1000,
            _reserved: 0,
        };

        assert_eq!(snapshot.snapshot_id, 1000);
        assert_eq!(snapshot.event_count, 1000);
        // Проверяем что поле timestamp удалено (нет доступа к полю)
    }

    #[test]
    fn test_event_type_conversion() {
        // Проверка From<u16> для новых типов
        assert_eq!(EventType::from(0x0006), EventType::TokenDecayed);
        assert_eq!(EventType::from(0x1005), EventType::ConnectionWeakened);
        assert_eq!(EventType::from(0x3002), EventType::GravityUpdate);
    }

    #[test]
    fn test_timeline_monotonicity() {
        let mut timeline = Timeline::new();

        let id1 = timeline.next_event_id(0);
        let id2 = timeline.next_event_id(0);
        let id3 = timeline.next_event_id(1);

        // Монотонность
        assert!(id2 > id1);
        assert!(id3 > id2);
        assert_eq!(timeline.total_events, 3);
    }

    #[test]
    fn test_event_validation() {
        let mut timeline = Timeline::new();
        let event_id = timeline.next_event_id(0);

        let event = Event::new(
            event_id,
            0,
            EventType::TokenCreate,
            EventPriority::Normal,
            0x1234,
            1,
            0,
            0,
        );

        assert!(event.validate(&timeline));
    }

    #[test]
    fn test_heartbeat_event_type() {
        // Heartbeat V2.0: тип события уже определен
        assert_eq!(EventType::Heartbeat as u16, 0x3001);

        let heartbeat = Event::with_pulse(
            10,
            0,
            EventType::Heartbeat,
            EventPriority::Low,
            0x1111,
            0,
            0,
            9,
            5, // pulse_number = 5
        );

        assert_eq!(heartbeat.event_type, EventType::Heartbeat as u16);
        assert_eq!(heartbeat.pulse_id, 5);
    }
}
