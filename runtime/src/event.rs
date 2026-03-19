// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM V1.0: docs/spec/COM V1.0.md

/// Тип события (причинный порядок).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    // Time события (0x3000-0x3FFF)
    Heartbeat = 0x3001,

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
            0x1001 => EventType::ConnectionCreate,
            0x1002 => EventType::ConnectionUpdate,
            0x1003 => EventType::ConnectionDelete,
            0x1004 => EventType::ConnectionStress,
            0x2001 => EventType::DomainCreate,
            0x2002 => EventType::DomainConfig,
            0x2003 => EventType::DomainReset,
            0x3001 => EventType::Heartbeat,
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

/// Событие — 32 байта, выравнивание 32.
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug)]
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
            domain_id,
            event_type: event_type as u16,
            priority: priority as u8,
            flags: 0,
            _reserved: [0; 4],
            payload_hash,
            target_id,
            source_id,
            payload_size: 0,
            parent_event_id,
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
            EventType::TokenMove | EventType::TokenTransform |
            EventType::ConnectionCreate | EventType::ConnectionUpdate |
            EventType::ConnectionDelete | EventType::ConnectionStress |
            EventType::DomainCreate | EventType::DomainConfig | EventType::DomainReset |
            EventType::Heartbeat |
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
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub snapshot_id: u64,
    pub state_hash: u64,
    pub event_count: u32,
    pub timestamp: u64,  // Wall-clock время для отладки
}
