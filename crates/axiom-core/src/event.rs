//! Event — события в системе причинного порядка (COM)
//!
//! Event представляет атомарное событие в системе. Размер строго 64 байта.
//! Использует выравнивание на 64 байта для оптимизации кеширования.
//!
//! # COM (Causal Order Model)
//! События используют причинный порядок вместо wall-clock времени.
//! Каждое событие имеет монотонно возрастающий `event_id` и ссылку на родительское событие.
//!
//! # Инварианты
//! - `event_id > 0` — каждое событие имеет уникальный идентификатор
//! - `parent_event_id < event_id` — родитель всегда предшествует потомку
//! - `payload_hash != 0` — содержимое события всегда имеет хеш
//! - `domain_id` определяет контекст события
//! - Размер структуры строго 64 байта

use std::fmt;

/// Тип события (причинный порядок)
///
/// Семантические события физики и эволюции системы.
/// Разбиты по категориям с зарезервированными диапазонами.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EventType {
    // Token события (0x0000-0x0FFF)
    TokenCreate = 0x0001,
    TokenUpdate = 0x0002,
    TokenDelete = 0x0003,
    TokenMove = 0x0004,
    TokenTransform = 0x0005,
    TokenDecayed = 0x0006,      // Затухание токена
    TokenMerged = 0x0007,       // Слияние токенов
    TokenSplit = 0x0008,        // Разделение токена
    TokenActivated = 0x0009,    // Активация токена
    TokenDeactivated = 0x000A,  // Деактивация токена
    TokenFrozen = 0x000B,       // Заморозка токена
    TokenThawed = 0x000C,       // Разморозка токена

    // SPACE события движения (0x0010-0x001F)
    TokenMoved = 0x0010,        // Токен переместился
    TokenCollision = 0x0011,    // Столкновение токенов
    TokenEnteredCell = 0x0012,  // Токен вошёл в новую ячейку

    // Connection события (0x1000-0x1FFF)
    ConnectionCreate = 0x1001,
    ConnectionUpdate = 0x1002,
    ConnectionDelete = 0x1003,
    ConnectionStress = 0x1004,
    ConnectionWeakened = 0x1005,    // Ослабление связи
    ConnectionStrengthened = 0x1006, // Усиление связи
    ConnectionBroken = 0x1007,      // Разрыв связи
    ConnectionFormed = 0x1008,      // Формирование новой связи

    // Domain события (0x2000-0x2FFF)
    DomainCreate = 0x2001,
    DomainConfig = 0x2002,
    DomainReset = 0x2003,

    // Physics события (0x3000-0x3FFF)
    Heartbeat = 0x3001,
    GravityUpdate = 0x3002,         // Обновление гравитации
    CollisionDetected = 0x3003,     // Обнаружено столкновение
    ResonanceTriggered = 0x3004,    // Триггер резонанса
    ThermodynamicsUpdate = 0x3005,  // Обновление температуры

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
            _ => panic!("Unknown event type: {:#06x}", v),
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

/// Event — событие в причинном порядке
///
/// Структура имеет фиксированный размер 64 байта и выравнивание на 64 байта.
/// Содержит информацию о причинности, содержании, идентификации и привязке к Heartbeat.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Event {
    // --- ПРИЧИННОСТЬ [16 байт] ---
    /// Монотонный причинный индекс (COM)
    pub event_id: u64,

    /// Предыдущее событие в цепочке
    pub parent_event_id: u64,

    // --- СОДЕРЖАНИЕ [16 байт] ---
    /// Хеш содержимого (валидация/детерминизм)
    pub payload_hash: u64,

    /// ID целевого объекта (Token/Connection)
    pub target_id: u32,

    /// ID источника
    pub source_id: u32,

    // --- ИДЕНТИФИКАЦИЯ [8 байт] ---
    /// Домен события
    pub domain_id: u16,

    /// Тип события (EventType enum)
    pub event_type: u16,

    /// Размер payload
    pub payload_size: u16,

    /// Приоритет (0..255)
    pub priority: u8,

    /// Флаги (CRITICAL, REVERSIBLE, etc.)
    pub flags: u8,

    // --- HEARTBEAT [8 байт] ---
    /// Номер пульса (0 = не привязан к пульсу)
    pub pulse_id: u64,

    // --- РЕЗЕРВ [16 байт] ---
    /// Резерв для будущих расширений
    pub _reserved: [u8; 16],
}

// Проверка размера на этапе компиляции
const _: () = assert!(std::mem::size_of::<Event>() == 64);

impl Event {
    /// Создает новое событие
    ///
    /// # Arguments
    /// * `event_id` - Уникальный монотонный идентификатор события
    /// * `domain_id` - ID домена события
    /// * `event_type` - Тип события
    /// * `priority` - Приоритет события
    /// * `payload_hash` - Хеш содержимого события
    /// * `target_id` - ID целевого объекта
    /// * `source_id` - ID источника события
    /// * `parent_event_id` - ID родительского события
    ///
    /// # Returns
    /// Новый экземпляр Event без привязки к пульсу
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
    ///
    /// # Arguments
    /// Те же, что и у `new()`, плюс:
    /// * `pulse_id` - Номер пульса Heartbeat
    ///
    /// # Returns
    /// Новый экземпляр Event с привязкой к пульсу
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

    /// Валидирует базовые инварианты события
    ///
    /// # Returns
    /// `Ok(())` если все инварианты соблюдены, `Err(String)` с описанием ошибки иначе
    pub fn validate(&self) -> Result<(), String> {
        if self.event_id == 0 {
            return Err("Event.event_id must be > 0".to_string());
        }
        if self.parent_event_id >= self.event_id {
            return Err("Event.parent_event_id must be < event_id".to_string());
        }
        if self.payload_hash == 0 {
            return Err("Event.payload_hash must be != 0".to_string());
        }
        if !self.is_valid_event_type() {
            return Err(format!("Invalid event type: {:#06x}", self.event_type));
        }
        Ok(())
    }

    /// Проверяет валидность типа события
    fn is_valid_event_type(&self) -> bool {
        // Проверяем, что можем конвертировать u16 → EventType без паники
        matches!(
            self.event_type,
            0x0001..=0x000C | 0x0010..=0x0012 | 0x1001..=0x1008 |
            0x2001..=0x2003 | 0x3001..=0x3005 | 0xF001..=0xF003
        )
    }

    /// Проверяет, является ли событие критическим
    #[inline]
    pub fn is_critical(&self) -> bool {
        (self.flags & EVENT_CRITICAL) != 0
    }

    /// Проверяет, является ли событие обратимым
    #[inline]
    pub fn is_reversible(&self) -> bool {
        (self.flags & EVENT_REVERSIBLE) != 0
    }

    /// Проверяет, является ли событие частью батча
    #[inline]
    pub fn is_batched(&self) -> bool {
        (self.flags & EVENT_BATCHED) != 0
    }

    /// Получает тип события как enum
    pub fn get_event_type(&self) -> EventType {
        EventType::from(self.event_type)
    }

    /// Получает приоритет события как enum
    pub fn get_priority(&self) -> EventPriority {
        match self.priority {
            0..=63 => EventPriority::Low,
            64..=191 => EventPriority::Normal,
            192..=254 => EventPriority::High,
            255 => EventPriority::Critical,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event[id={}, type={:?}, domain={}, target={}, priority={}]",
            self.event_id,
            EventType::from(self.event_type),
            self.domain_id,
            self.target_id,
            self.priority
        )
    }
}

/// Снапшот состояния после применения событий
///
/// Содержит только причинный порядок, НЕ wall-clock время.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    /// Причинный порядок (event_id последнего события)
    pub snapshot_id: u64,

    /// Хеш состояния для валидации
    pub state_hash: u64,

    /// Количество событий
    pub event_count: u32,

    /// Резерв для будущих расширений
    pub _reserved: u32,
}

impl Snapshot {
    /// Создает новый снапшот
    pub fn new(snapshot_id: u64, state_hash: u64, event_count: u32) -> Self {
        Self {
            snapshot_id,
            state_hash,
            event_count,
            _reserved: 0,
        }
    }
}
