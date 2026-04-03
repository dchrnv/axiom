// Copyright (C) 2024-2026 Chernov Denys
//
// UCL V2.0 - Unified Command Language (Zero-Allocation FFI Frame)
// 64 байта, repr(C, align(64))

/// Статус выполнения команды
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandStatus {
    Success = 0,
    PhysicsViolation = 1,
    TargetNotFound = 2,
    InvalidPayload = 3,
    AccessDenied = 4,
    SystemError = 5,
    Timeout = 6,
    UnknownOpcode = 7,
}

/// Коды операций (OpCodes)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // --- Генезис и Пространство (1000+) ---
    SpawnDomain = 1000,      // Рождение домена
    CollapseDomain = 1001,   // Уничтожение домена
    LockMembrane = 1002,     // Изменение мембраны
    ReshapeDomain = 1003,    // Изменение формы домена
    
    // --- Токены и Кинематика (2000+) ---
    InjectToken = 2000,      // Вброс нового смысла
    ApplyForce = 2001,       // Векторный толчок токена
    AnnihilateToken = 2002,  // Уничтожение токена
    BondTokens = 2003,       // Связать токены
    SplitToken = 2004,        // Разделить токен
    
    // --- Хронодинамика и Система (3000+) ---
    TickForward = 3000,      // Шаг симуляции
    ChangeTemperature = 3001,// Изменение термодинамики
    ApplyGravity = 3002,     // Изменение гравитации
    PhaseTransition = 3003,  // Фазовый переход

    // --- Dual-Path Processing (4000+) ---
    ProcessTokenDualPath = 4000, // Обработка токена через Arbiter (reflex + ASHTI)
    FinalizeComparison = 4001,   // Финализация сравнения и обучение

    // --- Администрирование (9000+) ---
    CoreShutdown = 9000,     // Остановка реактора
    CoreReset = 9001,        // Сброс состояния
    BackupState = 9002,      // Резервное копирование
    RestoreState = 9003,     // Восстановление состояния
}

/// Флаги команд
pub mod flags {
    pub const SYNC: u8 = 0x01;           // Синхронное выполнение
    pub const FORCE: u8 = 0x02;           // Принудительное выполнение
    pub const BYPASS_MEMBRANE: u8 = 0x04;  // Обойти мембрану
    pub const NO_EVENTS: u8 = 0x08;       // Не генерировать события
    pub const CRITICAL: u8 = 0x10;        // Критический приоритет
}

/// Основная структура команды - 64 байта
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct UclCommand {
    // --- ПОЛЕЗНАЯ НАГРУЗКА (PAYLOAD) [48 байт] ---
    pub payload: [u8; 48],      // 48b | Raw данные для разных команд

    // --- ЗАГОЛОВОК [16 байт] ---
    pub command_id: u64,        // 8b | Уникальный ID транзакции (COM)
    pub target_id: u32,         // 4b | Цель (Domain ID или Token ID)
    pub opcode: u16,            // 2b | Тип команды (OpCode)
    pub priority: u8,           // 1b | 0 (Low) - 255 (Critical)
    pub flags: u8,              // 1b | Битовая маска (Sync, Force, Bypass_Membrane)
}

/// Ответ ядра - 32 байта
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy)]
pub struct UclResult {
    pub command_id: u64,        // 8b | Ссылка на исходную команду
    pub execution_time_us: u32, // 4b | Время выполнения в микросекундах
    pub consumed_energy: f32,   // 4b | Затраченная энергия Домена на операцию
    pub error_code: u16,        // 2b | Детальный код аномалии
    pub events_generated: u16,  // 2b | Кол-во порожденных событий в шине COM
    pub status: u8,             // 1b | Статус выполнения
    pub reserved: [u8; 7],      // 7b | Добивка до 32 байт
}

/// Payload для SpawnDomain
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpawnDomainPayload {
    pub parent_domain_id: u16,  // 2b | Родительский домен
    pub factory_preset: u8,     // 1b | 0=Void, 1=Sutra, 6=Logic, 7=Dream, 10=Maya
    pub structural_role: u8,    // 1b | Для валидации
    pub initial_energy: f32,    // 4b | Начальная энергия
    pub seed: u32,              // 4b | Сид генерации для детерминизма
    pub reserved: [u8; 36],     // 36b | Резерв
}

/// Payload для ApplyForce
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ApplyForcePayload {
    pub force_vector: [f32; 3], // 12b | Направление X, Y, Z
    pub magnitude: f32,         // 4b  | Сила импульса
    pub duration_ticks: u32,    // 4b  | Как долго действует сила
    pub target_token_id: u32,   // 4b  | ID целевого токена
    pub force_type: u8,         // 1b  | Тип силы (гравитация, электромагнитная)
    pub reserved: [u8; 23],     // 23b | Резерв
}

/// Payload для InjectToken
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InjectTokenPayload {
    pub target_domain_id: u16, // 2b | Целевой домен
    pub token_type: u8,         // 1b | Тип токена
    pub mass: f32,              // 4b | Масса токена
    pub position: [f32; 3],     // 12b | Позиция X, Y, Z
    pub velocity: [f32; 3],     // 12b | Скорость X, Y, Z
    pub semantic_weight: f32,   // 4b | Семантический вес
    pub temperature: f32,       // 4b | Температура токена
    pub reserved: [u8; 6],      // 6b | Резерв
}

/// Payload для ChangeTemperature
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChangeTemperaturePayload {
    pub target_domain_id: u16, // 2b | Целевой домен
    pub delta_temperature: f32, // 4b | Изменение температуры
    pub transfer_rate: f32,     // 4b | Скорость передачи тепла
    pub source_point: [f32; 3],// 12b | Точка источника тепла
    pub radius: f32,           // 4b | Радиус воздействия
    pub duration_ticks: u32,    // 4b | Длительность воздействия
    pub reserved: [u8; 14],     // 14b | Резерв
}

/// Payload для ProcessTokenDualPath (4000)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcessTokenPayload {
    pub token_id: u32,          // 4b | ID токена для обработки
    pub source_domain: u8,      // 1b | Домен-источник
    pub enable_learning: u8,    // 1b | 1=включить обучение, 0=только inference
    pub reserved: [u8; 42],     // 42b | Резерв
}

/// Payload для FinalizeComparison (4001)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FinalizeComparisonPayload {
    pub event_id: u64,          // 8b | ID события для финализации
    pub reserved: [u8; 40],     // 40b | Резерв
}

impl UclCommand {
    /// Создать новую команду с автоматическим ID
    pub fn new(opcode: OpCode, target_id: u32, priority: u8, flags: u8) -> Self {
        Self {
            command_id: generate_command_id(),
            opcode: opcode as u16,
            target_id,
            priority,
            flags,
            payload: [0; 48],
        }
    }
    
    /// Установить payload для команды
    pub fn with_payload<T>(mut self, payload: &T) -> Self {
        unsafe {
            let payload_ptr = payload as *const T as *const u8;
            let payload_slice = std::slice::from_raw_parts(payload_ptr, std::mem::size_of::<T>());
            self.payload[..payload_slice.len()].copy_from_slice(payload_slice);
        }
        self
    }
    
    /// Получить payload как конкретный тип
    pub fn get_payload<T>(&self) -> T 
    where 
        T: Copy + Clone,
    {
        unsafe {
            std::ptr::read_unaligned(self.payload.as_ptr() as *const T)
        }
    }
    
    /// Проверить валидность команды
    pub fn is_valid(&self) -> bool {
        match self.opcode {
            1000 => true,                   // SpawnDomain - создает новый домен, target_id может быть 0
            1001 => self.target_id != 0,    // CollapseDomain требует target_id
            2001 => self.target_id != 0,    // ApplyForce требует target_id
            2000 => self.target_id != 0,    // InjectToken требует target_id
            3001 => self.target_id != 0,    // ChangeTemperature требует target_id
            _ => true,                       // Остальные команды валидны
        }
    }
}

impl UclResult {
    /// Создать успешный результат
    pub fn success(command_id: u64) -> Self {
        Self {
            command_id,
            status: CommandStatus::Success as u8,
            error_code: 0,
            consumed_energy: 0.0,
            events_generated: 0,
            execution_time_us: 0,
            reserved: [0; 7],
        }
    }
    
    /// Создать результат с ошибкой
    pub fn error(command_id: u64, status: CommandStatus, error_code: u16) -> Self {
        Self {
            command_id,
            status: status as u8,
            error_code,
            consumed_energy: 0.0,
            events_generated: 0,
            execution_time_us: 0,
            reserved: [0; 7],
        }
    }
    
    /// Проверить успешность выполнения
    pub fn is_success(&self) -> bool {
        self.status == CommandStatus::Success as u8
    }
}

/// Генератор ID команд
///
/// Time Model V1.0: использует атомарный счётчик вместо wall-clock времени.
/// В реальной системе command_id должны выдаваться через COM.
fn generate_command_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COMMAND_COUNTER: AtomicU64 = AtomicU64::new(1);
    COMMAND_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Билдер для удобного создания команд
pub struct UclBuilder;

/// Преобразовать UCL factory_preset в числовое значение StructuralRole.
///
/// UCL и StructuralRole enum используют разные числовые коды для одних ролей:
/// | Роль  | UCL preset | StructuralRole |
/// |-------|-----------|----------------|
/// | Void  | 0         | 8              |
/// | Sutra | 1         | 0              |
/// | Остальные | n    | n              |
pub fn ucl_preset_to_structural_role(preset: u8) -> u8 {
    match preset {
        0 => 8, // Void
        1 => 0, // Sutra
        n => n, // Остальные совпадают
    }
}

impl UclBuilder {
    /// Создать команду SpawnDomain
    pub fn spawn_domain(target_id: u32, preset: u8) -> UclCommand {
        let payload = SpawnDomainPayload {
            parent_domain_id: 0,
            factory_preset: preset,
            structural_role: ucl_preset_to_structural_role(preset),
            initial_energy: 100.0,
            seed: target_id,
            reserved: [0; 36],
        };
        
        UclCommand::new(OpCode::SpawnDomain, target_id, 100, 0)
            .with_payload(&payload)
    }
    
    /// Создать команду ApplyForce
    pub fn apply_force(target_id: u32, force: [f32; 3], magnitude: f32) -> UclCommand {
        let payload = ApplyForcePayload {
            force_vector: force,
            magnitude,
            duration_ticks: 1,
            target_token_id: target_id,
            force_type: 1, // Гравитационная
            reserved: [0; 23],
        };
        
        UclCommand::new(OpCode::ApplyForce, target_id, 100, 0)
            .with_payload(&payload)
    }
    
    /// Создать команду InjectToken
    pub fn inject_token(target_id: u32, token_type: u8, mass: f32, position: [f32; 3]) -> UclCommand {
        let payload = InjectTokenPayload {
            target_domain_id: target_id as u16,
            token_type,
            mass,
            position,
            velocity: [0.0, 0.0, 0.0],
            semantic_weight: 1.0,
            temperature: 273.0,
            reserved: [0; 6],
        };
        
        UclCommand::new(OpCode::InjectToken, target_id, 100, 0)
            .with_payload(&payload)
    }
}

