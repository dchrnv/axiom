//! Domain Configuration
//!
//! DomainConfig — 128 байт конфигурация домена с 5 блоками:
//! 1. Идентификация [16 байт]
//! 2. Физика поля [32 байта]
//! 3. Семантические оси [16 байт]
//! 4. Мембрана и Arbiter [32 байта]
//! 5. Метаданные [32 байта]

use serde::{Deserialize, Serialize};

/// Константы состояния домена
pub const DOMAIN_ACTIVE: u32 = 1;
pub const DOMAIN_LOCKED: u32 = 2;
pub const DOMAIN_TEMPORARY: u32 = 3;

/// Константы состояния обработки
pub const PROCESSING_IDLE: u8 = 1;
pub const PROCESSING_ACTIVE: u8 = 2;
pub const PROCESSING_FROZEN: u8 = 3;

/// Структурные роли доменов в системе Ashti_Core
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralRole {
    Sutra = 0,
    Execution = 1,
    Shadow = 2,
    Codex = 3,
    Map = 4,
    Probe = 5,
    Logic = 6,
    Dream = 7,
    Void = 8,
    Experience = 9, // Ассоциативная память
    Maya = 10,
}

/// Типы доменов
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainType {
    Logic = 1,
    Dream = 2,
    Math = 3,
    Pattern = 4,
    Memory = 5,
    Interface = 6,
}

/// DomainConfig — 128 байт конфигурация домена
///
/// Размер: 128 байт, выравнивание: 128 байт
/// Соответствует спецификации V2.1
#[repr(C, align(128))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DomainConfig {
    // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
    pub reserved_id: u64,       // 8b | Явный резерв для будущих расширений
    pub domain_id: u16,         // 2b | Уникальный ID Домена
    pub parent_domain_id: u16,  // 2b | Родительский Домен
    pub domain_type: u8,        // 1b | Тип (до 255 вариаций)
    pub structural_role: u8,    // 1b | Роль в Ashti_Core (0-10: SUTRA..MAYA)
    pub generation: u8,         // 1b | Поколение (эволюционный индекс)
    pub flags: u8,              // 1b | Битовая маска состояний (Active, Locked)
    // Offset: 16 байт

    // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
    pub field_size: [f32; 3],   // 12b| Размеры поля (X, Y, Z)
    pub gravity_strength: f32,  // 4b | Гравитация (-MAX..+MAX)
    pub temperature: f32,       // 4b | Температура поля в Кельвинах
    pub time_dilation: u16,     // 2b | Замедление времени (х100)
    pub resonance_freq: u16,    // 2b | Базовая частота (Hz)
    pub pressure: u16,          // 2b | Давление (Pa)
    pub rebuild_frequency: u16, // 2b | SPACE V6.0: частота перестройки spatial grid (событий)
    pub friction_coeff: u8,     // 1b | Трение (0..255 -> 0.0..1.0)
    pub viscosity: u8,          // 1b | Вязкость (0..255 -> 0.0..1.0)
    pub elasticity: u8,         // 1b | Упругость (0..255 -> 0.0..1.0)
    pub quantum_noise: u8,      // 1b | Квантовый шум (0..255 -> 0.0..1.0)
    // Offset: 48 байт

    // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
    pub axis_x_ref: u32,        // 4b | Референс концепции оси X
    pub axis_y_ref: u32,        // 4b | Референс концепции оси Y
    pub axis_z_ref: u32,        // 4b | Референс концепции оси Z
    pub axis_config: u32,       // 4b | Конфигурация полюсов (Bit-packed u16x2)
    // Offset: 64 байт

    // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
    pub input_filter: u64,      // 8b | 64-bit Bloom Filter или хэш входа
    pub output_filter: u64,     // 8b | 64-bit Bloom Filter или хэш выхода

    // -- Блок Arbiter [8 Байт] (V2.1: бывший reserved_membrane) --
    pub reflex_threshold: u8,   // 1b | Порог рефлекса (0..255 -> 0.0..1.0)
    pub association_threshold: u8, // 1b | Порог ассоциации (0..255 -> 0.0..1.0)
    pub arbiter_flags: u8,      // 1b | Битовая маска поведения Arbiter
    pub reflex_cooldown: u8,    // 1b | Минимальный интервал между рефлексами (в пульсах)
    pub max_concurrent_hints: u8, // 1b | Макс. кол-во ассоциаций-подсказок одновременно
    pub feedback_weight_delta: u8, // 1b | Шаг изменения weight при обратной связи (0..255)
    pub reserved_arbiter: [u8; 2], // 2b | Резерв блока Arbiter

    pub gate_complexity: u16,   // 2b | Вычислительная сложность шлюзов
    pub threshold_mass: u16,    // 2b | Порог массы для прохождения
    pub threshold_temp: u16,    // 2b | Порог температуры для прохождения
    pub permeability: u8,       // 1b | Проницаемость (0..255 -> 0.0..1.0)
    pub membrane_state: u8,     // 1b | OPEN/CLOSED/SEMI/ADAPTIVE
    // Offset: 96 байт

    // --- 5. МЕТАДАННЫЕ [32 Байт] ---
    pub created_at: u64,        // 8b | COM event_id (Время создания)
    pub last_update: u64,       // 8b | COM event_id (Последнее обновление)
    pub token_capacity: u32,    // 4b | Максимальная емкость токенов
    pub connection_capacity: u32, // 4b | Максимальная емкость связей
    pub error_count: u16,       // 2b | Счетчик когнитивных ошибок
    pub processing_state: u8,   // 1b | IDLE/PROCESSING/FROZEN/CRASHED
    pub complexity_score: u8,   // 1b | Оценка сложности (0..255 -> 0.0..1.0)
    pub performance_score: u8,  // 1b | Производительность (0..255 -> 0.0..1.0)
    pub reserved_meta: [u8; 3], // 3b | Добивка до границы 128 байт
    // Итого: 128 байт. Offset: 128. Без скрытого паддинга.
}

// Compile-time size assertion
const _: () = assert!(std::mem::size_of::<DomainConfig>() == 128);

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,
            domain_id: 1,
            parent_domain_id: 0,
            domain_type: DomainType::Logic as u8,
            structural_role: StructuralRole::Logic as u8,
            generation: 0,
            flags: DOMAIN_ACTIVE as u8,

            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [100.0, 100.0, 100.0],
            gravity_strength: 1.0,
            temperature: 293.15, // 20°C
            time_dilation: 100,  // 1.0x
            resonance_freq: 440,
            pressure: 50000,
            rebuild_frequency: 10,
            friction_coeff: 25,
            viscosity: 3,
            elasticity: 128,
            quantum_noise: 0,

            // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
            axis_x_ref: 0,
            axis_y_ref: 0,
            axis_z_ref: 0,
            axis_config: 0,

            // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
            input_filter: 0,
            output_filter: 0,
            reflex_threshold: 128,
            association_threshold: 64,
            arbiter_flags: 0,
            reflex_cooldown: 5,
            max_concurrent_hints: 10,
            feedback_weight_delta: 10,
            reserved_arbiter: [0; 2],
            gate_complexity: 100,
            threshold_mass: 0,
            threshold_temp: 0,
            permeability: 128,
            membrane_state: 1, // OPEN

            // --- 5. МЕТАДАННЫЕ [32 Байт] ---
            created_at: 0,
            last_update: 0,
            token_capacity: 1000,
            connection_capacity: 5000,
            error_count: 0,
            processing_state: PROCESSING_IDLE,
            complexity_score: 0,
            performance_score: 255,
            reserved_meta: [0; 3],
        }
    }
}

impl DomainConfig {
    /// Создать новый домен с базовыми параметрами
    pub fn new(domain_id: u16, domain_type: DomainType, structural_role: StructuralRole) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.domain_type = domain_type as u8;
        config.structural_role = structural_role as u8;
        config
    }

    /// Базовая фабрика (ВАКУУМ)
    ///
    /// Создает абсолютно нейтральное, пустое пространство (Role::Void = 8).
    /// Используется как основа для всех остальных фабрик.
    pub fn default_void() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,
            domain_id: 0,
            parent_domain_id: 0,
            domain_type: 0,
            structural_role: StructuralRole::Void as u8,
            generation: 0,
            flags: 0,

            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [0.0, 0.0, 0.0],
            gravity_strength: 0.0,
            temperature: 0.0,
            time_dilation: 0,
            resonance_freq: 0,
            pressure: 0,
            rebuild_frequency: 0,
            friction_coeff: 0,
            viscosity: 0,
            elasticity: 0,
            quantum_noise: 0,

            // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
            axis_x_ref: 0,
            axis_y_ref: 0,
            axis_z_ref: 0,
            axis_config: 0,

            // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
            input_filter: 0,
            output_filter: 0,
            reflex_threshold: 0,
            association_threshold: 0,
            arbiter_flags: 0,
            reflex_cooldown: 0,
            max_concurrent_hints: 0,
            feedback_weight_delta: 0,
            reserved_arbiter: [0; 2],
            gate_complexity: 0,
            threshold_mass: 0,
            threshold_temp: 0,
            permeability: 0,
            membrane_state: 0,

            // --- 5. МЕТАДАННЫЕ [32 Байт] ---
            created_at: 0,
            last_update: 0,
            token_capacity: 0,
            connection_capacity: 0,
            error_count: 0,
            processing_state: 0,
            complexity_score: 0,
            performance_score: 0,
            reserved_meta: [0; 3],
        }
    }

    /// SUTRA (0) - Источник Истины
    ///
    /// Физика: Кристаллизация. Абсолютный ноль, колоссальная гравитация.
    /// Сюда ничего не проникает извне, отсюда только рождаются смыслы.
    pub fn factory_sutra(domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.structural_role = StructuralRole::Sutra as u8;

        config.gravity_strength = f32::MAX;
        config.temperature = 0.0;
        config.permeability = 0;
        config.membrane_state = 2; // CLOSED

        config.token_capacity = 100;
        config.connection_capacity = 1000;
        config
    }

    /// Валидация конфигурации
    pub fn validate(&self) -> Result<(), String> {
        // Проверка размера структуры
        if std::mem::size_of::<Self>() != 128 {
            return Err(format!(
                "DomainConfig size is {} bytes, expected 128",
                std::mem::size_of::<Self>()
            ));
        }

        // Проверка field_size
        if self.field_size[0] < 0.0
            || self.field_size[1] < 0.0
            || self.field_size[2] < 0.0
        {
            return Err("field_size components must be non-negative".to_string());
        }

        // Проверка temperature
        if self.temperature < 0.0 {
            return Err("temperature must be non-negative (Kelvin)".to_string());
        }

        // Проверка capacities
        if self.token_capacity == 0 {
            return Err("token_capacity must be > 0".to_string());
        }

        if self.connection_capacity == 0 {
            return Err("connection_capacity must be > 0".to_string());
        }

        Ok(())
    }
}
