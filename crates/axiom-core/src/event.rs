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
    /// Создание токена
    TokenCreate = 0x0001,
    /// Обновление токена
    TokenUpdate = 0x0002,
    /// Удаление токена
    TokenDelete = 0x0003,
    /// Перемещение токена
    TokenMove = 0x0004,
    /// Трансформация токена
    TokenTransform = 0x0005,
    /// Затухание токена
    TokenDecayed = 0x0006,
    /// Слияние токенов
    TokenMerged = 0x0007,
    /// Разделение токена
    TokenSplit = 0x0008,
    /// Активация токена
    TokenActivated = 0x0009,
    /// Деактивация токена
    TokenDeactivated = 0x000A,
    /// Заморозка токена
    TokenFrozen = 0x000B,
    /// Разморозка токена
    TokenThawed = 0x000C,

    // SPACE события движения (0x0010-0x001F)
    /// Токен переместился
    TokenMoved = 0x0010,
    /// Столкновение токенов
    TokenCollision = 0x0011,
    /// Токен вошёл в новую ячейку
    TokenEnteredCell = 0x0012,

    // Connection события (0x1000-0x1FFF)
    /// Создание связи
    ConnectionCreate = 0x1001,
    /// Обновление связи
    ConnectionUpdate = 0x1002,
    /// Удаление связи
    ConnectionDelete = 0x1003,
    /// Изменение стресса связи
    ConnectionStress = 0x1004,
    /// Ослабление связи
    ConnectionWeakened = 0x1005,
    /// Усиление связи
    ConnectionStrengthened = 0x1006,
    /// Разрыв связи
    ConnectionBroken = 0x1007,
    /// Формирование новой связи
    ConnectionFormed = 0x1008,

    // Domain события (0x2000-0x2FFF)
    /// Создание домена
    DomainCreate = 0x2001,
    /// Конфигурация домена
    DomainConfig = 0x2002,
    /// Сброс домена
    DomainReset = 0x2003,

    // Physics события (0x3000-0x3FFF)
    /// Периодический пульс системы
    Heartbeat = 0x3001,
    /// Обновление гравитации
    GravityUpdate = 0x3002,
    /// Обнаружено столкновение
    CollisionDetected = 0x3003,
    /// Триггер резонанса
    ResonanceTriggered = 0x3004,
    /// Обновление температуры
    ThermodynamicsUpdate = 0x3005,

    // Внешние агентские события (0xE000-0xEFFF)
    /// Выполнение shell-команды (ShellEffector)
    ShellExec = 0xE001,
    /// Ответ MAYA — внешний вывод системы
    MayaOutput = 0xE002,

    // Системные события (0xF000-0xFFFE)
    /// Создание контрольной точки системы
    SystemCheckpoint = 0xF001,
    /// Откат системы к контрольной точке
    SystemRollback = 0xF002,
    /// Остановка системы
    SystemShutdown = 0xF003,

    /// Неизвестный тип — безопасная замена panic при десериализации
    Unknown = 0xFFFF,
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
            0xE001 => EventType::ShellExec,
            0xE002 => EventType::MayaOutput,
            0xF001 => EventType::SystemCheckpoint,
            0xF002 => EventType::SystemRollback,
            0xF003 => EventType::SystemShutdown,
            _ => EventType::Unknown,
        }
    }
}

/// Приоритеты событий
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum EventPriority {
    /// Низкий приоритет
    Low = 0,
    /// Обычный приоритет
    Normal = 128,
    /// Высокий приоритет
    High = 200,
    /// Критический приоритет
    Critical = 255,
}

/// Флаги событий
pub const EVENT_REVERSIBLE: u8 = 1;
/// Флаг критического события
pub const EVENT_CRITICAL: u8 = 2;
/// Флаг пакетного события
pub const EVENT_BATCHED: u8 = 4;
/// Флаг внутреннего события (от Internal Drive — Cognitive Depth)
pub const EVENT_INTERNAL: u8 = 8;

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

    // --- РАСШИРЕНИЕ [16 байт] (бывший _reserved) ---
    /// Домен-источник события (для GUARDIAN enforce_protocol).
    /// По умолчанию = domain_id. Отличается когда PROBE(5) инициирует событие в EXECUTION(1).
    pub source_domain: u16,

    /// Второй уровень классификации события.
    /// Уточняет причину внутри одного event_type.
    /// По умолчанию = 0 (SUBTYPE_NONE). Не влияет на сигнатуру конструкторов.
    pub event_subtype: u16,

    /// ID снапшота на момент создания события.
    /// Causal Horizon: события с snapshot_event_id < текущего снапшота безопасны для архивации.
    pub snapshot_event_id: u32,

    /// Inline payload (8 байт структурированных данных).
    /// Интерпретация зависит от event_type:
    /// - ShellExec:       [command_index: u16 LE, _: 6]
    /// - InternalImpulse: [impulse_type: u8, intensity: u8, source_trace: u32 LE, _: 2]
    /// - TokenMove:       [dx: i16 LE, dy: i16 LE, dz: i16 LE, _: 2]
    /// - Остальные:       [0u8; 8]
    pub payload: [u8; 8],
}

// Проверка размера на этапе компиляции
const _: () = assert!(std::mem::size_of::<Event>() == 64);

// === Event subtypes ===
// Не указан (обратная совместимость — все существующие события)
pub const SUBTYPE_NONE: u16 = 0;

// Подтипы для TokenMove (0x0004) и TokenMoved (0x0010)
/// Движение от гравитации
pub const SUBTYPE_GRAVITY: u16 = 1;
/// Ручное перемещение (ApplyForce / пользователь)
pub const SUBTYPE_MANUAL: u16 = 2;
/// Отскок от столкновения
pub const SUBTYPE_COLLISION: u16 = 3;
/// Внутренний импульс (Cognitive Depth)
pub const SUBTYPE_IMPULSE: u16 = 4;
/// Инерционное движение
pub const SUBTYPE_INERTIA: u16 = 5;
/// Движение к target (аттрактор)
pub const SUBTYPE_ATTRACTOR: u16 = 6;

// Подтипы для InternalImpulse (0x4001, будущий тип)
/// Tension trace
pub const SUBTYPE_TENSION: u16 = 1;
/// DREAM curiosity
pub const SUBTYPE_CURIOSITY: u16 = 2;
/// Goal persistence
pub const SUBTYPE_GOAL: u16 = 3;
/// Incompletion trace
pub const SUBTYPE_INCOMPLETION: u16 = 4;

// Подтипы для ConnectionCreate (0x1001)
/// Связь создана резонансом
pub const SUBTYPE_RESONANCE: u16 = 1;
/// Связь создана столкновением
pub const SUBTYPE_COLLISION_LINK: u16 = 2;
/// Связь импортирована (persistence / exchange)
pub const SUBTYPE_IMPORTED: u16 = 3;

// Подтипы для SystemCheckpoint (0xF001)
/// Ручное сохранение (:save)
pub const SUBTYPE_MANUAL_SAVE: u16 = 1;
/// Автосохранение (кристаллизация)
pub const SUBTYPE_AUTO_SAVE: u16 = 2;
/// Сохранение при :quit
pub const SUBTYPE_SHUTDOWN_SAVE: u16 = 3;

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
    #[allow(clippy::too_many_arguments)]
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
            pulse_id: 0,
            source_domain: domain_id,
            event_subtype: SUBTYPE_NONE,
            snapshot_event_id: 0,
            payload: [0; 8],
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
    #[allow(clippy::too_many_arguments)]
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
            source_domain: domain_id,
            event_subtype: SUBTYPE_NONE,
            snapshot_event_id: 0,
            payload: [0; 8],
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
        // Unknown (0xFFFF) намеренно исключён — события с Unknown-типом не проходят validate()
        matches!(
            self.event_type,
            0x0001..=0x000C | 0x0010..=0x0012 | 0x1001..=0x1008 |
            0x2001..=0x2003 | 0x3001..=0x3005 |
            0xE001..=0xE002 | 0xF001..=0xF003
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

    /// Проверяет, является ли событие внутренним (от Internal Drive)
    #[inline]
    pub fn is_internal(&self) -> bool {
        (self.flags & EVENT_INTERNAL) != 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_event_type_does_not_panic() {
        // Любой неизвестный код → Unknown, не паника
        assert_eq!(EventType::from(0xBEEF), EventType::Unknown);
        assert_eq!(EventType::from(0x0000), EventType::Unknown);
        assert_eq!(EventType::from(0x9999), EventType::Unknown);
    }

    #[test]
    fn known_event_types_parse_correctly() {
        assert_eq!(EventType::from(0x0001), EventType::TokenCreate);
        assert_eq!(EventType::from(0xE001), EventType::ShellExec);
        assert_eq!(EventType::from(0xF001), EventType::SystemCheckpoint);
        assert_eq!(EventType::from(0xFFFF), EventType::Unknown);
    }

    #[test]
    fn event_size_unchanged() {
        assert_eq!(std::mem::size_of::<Event>(), 64);
    }

    #[test]
    fn event_subtype_default_is_zero() {
        let e = Event::new(1, 1, EventType::TokenMove, EventPriority::Normal, 1, 0, 0, 0);
        assert_eq!(e.event_subtype, SUBTYPE_NONE);
        assert_eq!(e.event_subtype, 0);
    }

    #[test]
    fn event_subtype_roundtrip() {
        let mut e = Event::new(1, 1, EventType::TokenMove, EventPriority::Normal, 1, 0, 0, 0);
        e.event_subtype = SUBTYPE_GRAVITY;
        assert_eq!(e.event_subtype, SUBTYPE_GRAVITY);
        assert_eq!(e.event_subtype, 1);
    }

    #[test]
    fn event_with_pulse_subtype_default_is_zero() {
        let e = Event::with_pulse(1, 1, EventType::Heartbeat, EventPriority::Normal, 1, 0, 0, 0, 42);
        assert_eq!(e.event_subtype, SUBTYPE_NONE);
    }
}
