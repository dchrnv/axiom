// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: docs/spec/Domain V1.3.md

/// Состояние Domain
pub const DOMAIN_ACTIVE: u32 = 1;
pub const DOMAIN_LOCKED: u32 = 2;
pub const DOMAIN_TEMPORARY: u32 = 3;

/// Состояние обработки
pub const PROCESSING_IDLE: u8 = 1;
pub const PROCESSING_ACTIVE: u8 = 2;
pub const PROCESSING_FROZEN: u8 = 3;

/// Состояние мембраны
pub const MEMBRANE_OPEN: u8 = 1;
pub const MEMBRANE_CLOSED: u8 = 2;
pub const MEMBRANE_SEMI: u8 = 3;

/// Структурные роли в Ashti_Core
#[repr(u8)]
pub enum StructuralRole {
    Sutra = 0,
    Ashti1 = 1,
    Ashti2 = 2,
    Ashti3 = 3,
    Ashti4 = 4,
    Ashti5 = 5,
    Ashti6 = 6,
    Ashti7 = 7,
    Ashti8 = 8,
    Maya = 10,
}

/// Типы доменов
#[repr(u16)]
pub enum DomainType {
    Logic = 1,
    Dream = 2,
    Math = 3,
    Pattern = 4,
    Memory = 5,
    Interface = 6,
}

/// DomainConfig — 128 байт конфигурация домена
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DomainConfig {
    // --- ИДЕНТИФИКАЦИЯ (16 Байт) ---
    pub domain_id: u16,         // Уникальный ID Домена
    pub domain_type: u16,       // Тип Домена (Logic, Dream, Math...)
    pub structural_role: u8,    // Роль в Ashti_Core (0, 1-8, 10)
    pub generation: u8,        // Поколение (для эволюции)
    pub parent_domain_id: u16,  // Родительский Домен
    pub flags: u32,             // ACTIVE/LOCKED/TEMPORARY/SYSTEM
    pub reserved_id: [u8; 8],   // Резерв для будущих полей

    // --- ФИЗИКА ПОЛЯ (32 Байт) ---
    pub field_size: [f32; 3],   // Размеры поля (X, Y, Z)
    pub gravity_strength: f32,  // Сила гравитации (0.0..MAX)
    pub friction_coeff: f32,    // Коэффициент трения (0.0..1.0)
    pub resonance_freq: f32,    // Базовая частота резонанса (Hz)
    pub temperature: f32,       // Базовая температура поля (K)
    pub pressure: f32,          // Давление в поле (Pa)
    pub viscosity: f32,         // Вязкость среды (0.0..1.0)
    pub elasticity: f32,        // Упругость поля (0.0..1.0)
    pub quantum_noise: f32,     // Квантовый шум (0.0..1.0)
    pub time_dilation: f32,     // Замедление времени (0.0..10.0)
    pub reserved_physics: [u32; 3], // Резерв

    // --- МЕМБРАНА (32 Байт) ---
    pub input_filter: [u8; 16], // Хеши разрешенных входных паттернов
    pub output_filter: [u8; 16], // Хеши разрешенных выходных паттернов
    pub permeability: f32,      // Проницаемость мембраны (0.0..1.0)
    pub threshold_mass: u8,     // Порог массы для входа (1..255)
    pub threshold_temp: u8,     // Порог температуры для входа (0..255)
    pub gate_complexity: u16,   // Сложность шлюзов (0..1000)
    pub membrane_state: u8,      // OPEN/CLOSED/SEMI/ADAPTIVE
    pub reserved_membrane: [u8; 5], // Резерв

    // --- МЕТАДАННЫЕ (48 Байт) ---
    pub created_at: u64,        // COM event_id создания
    pub last_update: u64,       // COM event_id последнего обновления
    pub token_capacity: u32,     // Максимальное количество Token
    pub connection_capacity: u32, // Максимальное количество Connection
    pub energy_budget: f32,      // Бюджет энергии Домена
    pub complexity_score: f32,   // Оценка сложности (0.0..1.0)
    pub processing_state: u8,    // IDLE/PROCESSING/FROZEN/CRASHED
    pub error_count: u16,        // Счетчик ошибок
    pub performance_score: f32,  // Оценка производительности
    pub reserved_meta: [u8; 12], // Резерв
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            domain_id: 0,
            domain_type: DomainType::Logic as u16,
            structural_role: StructuralRole::Ashti1 as u8,
            generation: 0,
            parent_domain_id: 0,
            flags: DOMAIN_ACTIVE,
            reserved_id: [0; 8],
            field_size: [100.0, 100.0, 100.0],
            gravity_strength: 1.0,
            friction_coeff: 0.1,
            resonance_freq: 440.0,
            temperature: 293.15, // 20°C в Кельвинах
            pressure: 101325.0, // 1 атм в Паскалях
            viscosity: 0.01,
            elasticity: 0.5,
            quantum_noise: 0.001,
            time_dilation: 1.0,
            reserved_physics: [0; 3],
            input_filter: [255; 16], // Все разрешено
            output_filter: [255; 16], // Все разрешено
            permeability: 1.0,
            threshold_mass: 1,
            threshold_temp: 200, // ~-73°C в Кельвинах (в пределах u8)
            gate_complexity: 1,
            membrane_state: MEMBRANE_OPEN,
            reserved_membrane: [0; 5],
            created_at: 0,
            last_update: 0,
            token_capacity: 1000,
            connection_capacity: 5000,
            energy_budget: 100000.0,
            complexity_score: 0.0,
            processing_state: PROCESSING_IDLE,
            error_count: 0,
            performance_score: 1.0,
            reserved_meta: [0; 12],
        }
    }
}

impl DomainConfig {
    pub fn new(domain_id: u16, domain_type: DomainType, role: StructuralRole) -> Self {
        Self {
            domain_id,
            domain_type: domain_type as u16,
            structural_role: role as u8,
            ..Default::default()
        }
    }

    /// Валидация согласно спецификации Domain V1.3
    pub fn validate(&self) -> bool {
        self.domain_id > 0
        && self.field_size.iter().all(|&size| size > 0.0)
        && self.gravity_strength >= 0.0
        && self.friction_coeff >= 0.0
        && self.temperature >= 0.0
        && self.permeability >= 0.0 && self.permeability <= 1.0
        && self.created_at >= 0
        && self.last_update >= self.created_at
        && self.token_capacity > 0
        && self.connection_capacity > 0
        && self.energy_budget > 0.0
    }

    /// Проверка фильтров мембраны
    pub fn can_enter(&self, mass: u8, temperature: u8) -> bool {
        mass >= self.threshold_mass
        && temperature >= self.threshold_temp
        && self.membrane_state != MEMBRANE_CLOSED
    }

    /// Обновление метаданных при изменении
    pub fn update_metadata(&mut self, event_id: u64) {
        self.last_update = event_id;
        self.error_count = 0; // Сброс счетчика при успешном обновлении
    }

    /// Расчет сложности домена
    pub fn calculate_complexity(&self) -> f32 {
        let token_factor = self.token_capacity as f32 * 0.1;
        let connection_factor = self.connection_capacity as f32 * 0.05;
        let physics_factor = (self.gravity_strength + self.friction_coeff) * 10.0;
        
        token_factor + connection_factor + physics_factor
    }

    /// Проверка состояния домена
    pub fn is_active(&self) -> bool {
        self.flags & DOMAIN_ACTIVE != 0
    }

    pub fn is_locked(&self) -> bool {
        self.flags & DOMAIN_LOCKED != 0
    }

    pub fn is_temporary(&self) -> bool {
        self.flags & DOMAIN_TEMPORARY != 0
    }
}
