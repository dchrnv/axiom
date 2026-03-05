// Copyright (C) 2024-2026 Chernov Denys
//
// Domain V1.3: docs/spec/Domain V1.3.md

use crate::config::{ConfigLoader, initialize};

#[cfg(test)]
mod tests;

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
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub domain_id: u16,
    pub domain_type: DomainType,
    pub structural_role: StructuralRole,
    pub generation: u8,
    pub parent_domain_id: u16,
    pub flags: u32,
    pub reserved_id: [u8; 8],
    
    // --- ФИЗИЧЕСКИЕ ПАРАМЕТРЫ (64 Байта) ---
    pub field_size: [f32; 3],
    pub gravity_strength: f32,
    pub friction_coeff: f32,
    pub resonance_freq: f32,
    pub temperature: f32,
    pub pressure: f32,
    pub viscosity: f32,
    pub elasticity: f32,
    pub quantum_noise: f32,
    pub time_dilation: f32,
    pub reserved_physics: [u8; 3],
    
    // --- ФИЛЬТРЫ (16 Байт) ---
    pub input_filter: [u8; 16],
    pub output_filter: [u8; 16],
    
    // --- МЕМБРАНА (8 Байт) ---
    pub permeability: f32,
    pub threshold_mass: u8,
    pub threshold_temp: u8,
    pub gate_complexity: u8,
    pub membrane_state: u8,
    pub reserved_membrane: [u8; 5],
    
    // --- СИСТЕМНЫЕ (32 Байта) ---
    pub created_at: u64,
    pub last_update: u64,
    pub token_capacity: u32,
    pub connection_capacity: u32,
    pub energy_budget: f32,
    pub complexity_score: f32,
    pub processing_state: u8,
    pub error_count: u16,
    pub performance_score: f32,
    pub reserved_meta: [u8; 12],
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            domain_id: 1,
            domain_type: DomainType::Logic,
            structural_role: StructuralRole::Ashti1,
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
    /// Создать новый домен с параметрами по умолчанию
    pub fn new(domain_id: u16, domain_type: DomainType, role: StructuralRole) -> Self {
        Self {
            domain_id,
            domain_type,
            structural_role: role,
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

    /// Создать домен из пресета согласно Configuration System
    pub fn from_preset(preset_name: &str) -> Result<Self, crate::config::ConfigError> {
        let config = initialize()?;
        let mut loader = crate::config::ConfigLoader::new();
        
        // Загрузить схему доменов
        let schema = loader.load_schema("domain", 
            std::path::Path::new(&config.schema.domain))?;
        
        // Получить пресет из схемы
        if let Some(presets) = schema.get("domain_types") {
            if let Some(_preset) = presets.get(preset_name) {
                // В реальной реализации здесь будет десериализация
                // из YAML в DomainConfig структуру
                println!("Loading domain from preset: {}", preset_name);
                return Ok(Self::default()); // Временно
            }
        }
        
        Err(crate::config::ConfigError::ValidationError(format!(
            "Unknown preset: {}", preset_name
        )))
    }

    /// Валидация согласно спецификации Domain V1.3
    pub fn validate(&self) -> bool {
        // Базовая валидация
        if self.domain_id == 0 {
            return false;
        }
        
        if self.field_size.iter().any(|&x| x <= 0.0 || x > 1000.0) {
            return false;
        }
        
        if self.temperature < 0.0 || self.temperature > 1000.0 {
            return false;
        }
        
        if self.permeability < 0.0 || self.permeability > 10.0 {
            return false;
        }
        
        true
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
