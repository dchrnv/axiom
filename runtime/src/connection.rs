// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Connection V5.0: docs/spec/Connection V5.0.md

use crate::config::{ConfigLoader, initialize};

/// Флаги Connection
pub const FLAG_ACTIVE: u32 = 1;
pub const FLAG_INHIBITED: u32 = 2;
pub const FLAG_TEMPORARY: u32 = 4;
pub const FLAG_CRITICAL: u32 = 8;

/// Connection — 64 байта, выравнивание 64.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    // --- ТОПОЛОГИЯ (16 Байт) ---
    pub source_id: u32,
    pub target_id: u32,
    pub domain_id: u16,
    pub link_type: u16,
    pub flags: u32,

    // --- ДИНАМИКА (16 Байт) ---
    pub strength: f32,
    pub current_stress: f32,
    pub ideal_dist: f32,
    pub elasticity: f32,

    // --- ШЛЮЗ (16 Байт) ---
    pub density_gate: u8,
    pub thermal_gate: u8,
    pub reserved_gate: [u8; 14],

    // --- МЕТАДАННЫЕ (16 Байт) ---
    pub created_at: u64,
    pub last_event_id: u64,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            source_id: 0,
            target_id: 0,
            domain_id: 0,
            link_type: 0,
            flags: 0,
            strength: 1.0,
            current_stress: 0.0,
            ideal_dist: 0.0,
            elasticity: 1.0,
            density_gate: 0,
            thermal_gate: 0,
            reserved_gate: [0; 14],
            created_at: 0,
            last_event_id: 0,
        }
    }
}

impl Connection {
    pub fn new(source_id: u32, target_id: u32, domain_id: u16) -> Self {
        Self {
            source_id,
            target_id,
            domain_id,
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.flags & FLAG_ACTIVE != 0
    }

    #[inline]
    pub fn is_inhibited(&self) -> bool {
        self.flags & FLAG_INHIBITED != 0
    }

    #[inline]
    pub fn is_temporary(&self) -> bool {
        self.flags & FLAG_TEMPORARY != 0
    }

    #[inline]
    pub fn is_critical(&self) -> bool {
        self.flags & FLAG_CRITICAL != 0
    }

    /// Валидация согласно спецификации Connection V5.0
    pub fn validate(&self) -> bool {
        self.source_id > 0
        && self.target_id > 0
        && self.domain_id > 0
        && self.strength > 0.0
        && self.current_stress >= 0.0
        && self.elasticity > 0.0
        && self.created_at > 0
        && self.last_event_id >= self.created_at
    }

    /// Проверка прохождения через шлюз массы
    pub fn can_pass_mass(&self, mass: u8) -> bool {
        mass >= self.density_gate
    }

    /// Проверка прохождения через шлюз температуры
    pub fn can_pass_temperature(&self, temperature: u8) -> bool {
        temperature <= self.thermal_gate
    }

    /// Обновление стресса через COM событие
    pub fn update_stress(&mut self, new_stress: f32, event_id: u64) {
        self.current_stress = new_stress.max(0.0);
        self.last_event_id = event_id;
        
        // Автоматическая установка критического флага
        if self.current_stress > self.strength * 0.8 {
            self.flags |= FLAG_CRITICAL;
        }
    }

    /// Вычисление расстояния между токенами
    pub fn compute_distance(&self, source_pos: [i16; 3], target_pos: [i16; 3]) -> f32 {
        let dx = (target_pos[0] - source_pos[0]) as f32;
        let dy = (target_pos[1] - source_pos[1]) as f32;
        let dz = (target_pos[2] - source_pos[2]) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Создать соединение из пресета согласно Configuration System
    pub fn from_preset(preset_name: &str, source_id: u32, target_id: u32, domain_id: u16) -> Result<Self, crate::config::ConfigError> {
        let config = initialize()?;
        let mut loader = ConfigLoader::new();
        
        // Загрузить схему соединений
        let schema = loader.load_schema("connection", 
            std::path::Path::new(&config.schema.connection))?;
        
        // Получить пресет из схемы
        if let Some(connection_types) = schema.get("connection_types") {
            if let Some(connection_types_array) = connection_types.as_sequence() {
                for connection_type in connection_types_array {
                    if let Some(name) = connection_type.get("name") {
                        if let Some(name_str) = name.as_str() {
                            if name_str == preset_name {
                                // Получить значения по умолчанию из пресета
                                let default_strength = connection_type.get("default_strength")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(1.0) as f32;
                                
                                let decay_rate = connection_type.get("decay_rate")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.01) as f32;
                                
                                println!("Creating connection from preset: {} (strength: {}, decay_rate: {})", 
                                    preset_name, default_strength, decay_rate);
                                
                                let mut connection = Self::new(source_id, target_id, domain_id);
                                connection.strength = default_strength;
                                connection.elasticity = 1.0 - decay_rate; // Используем decay_rate для elasticity
                                connection.created_at = 1; // Установить валидное время создания
                                connection.last_event_id = 1; // Установить валидный event_id
                                connection.flags = FLAG_ACTIVE; // Активировать соединение
                                
                                return Ok(connection);
                            }
                        }
                    }
                }
            }
        }
        
        Err(crate::config::ConfigError::ValidationError(format!(
            "Unknown connection preset: {}", preset_name
        )))
    }

    /// Валидация соединения с использованием конфигурации
    pub fn validate_with_config(&self) -> Result<(), crate::config::ConfigError> {
        // Базовая валидация
        if !self.validate() {
            return Err(crate::config::ConfigError::ValidationError(
                "Connection failed basic validation".to_string()
            ));
        }

        // Загрузить схему для дополнительной валидации
        let config = initialize()?;
        let mut loader = ConfigLoader::new();
        let schema = loader.load_schema("connection", 
            std::path::Path::new(&config.schema.connection))?;

        // Проверить topology constraints
        if let Some(_constraints) = schema.get("topology_constraints") {
            // В реальной реализации здесь будет проверка ограничений
            // Например: max_degree, no_self_loops, symmetry, path_length
            
            // Проверка на самосвязи (no_self_loops)
            if self.source_id == self.target_id {
                return Err(crate::config::ConfigError::ValidationError(
                    "Self-loops are not allowed".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_from_preset() {
        // Тест создания соединения из пресета
        let result = Connection::from_preset("strong", 1, 2, 1);
        assert!(result.is_ok());
        
        let connection = result.unwrap();
        assert_eq!(connection.source_id, 1);
        assert_eq!(connection.target_id, 2);
        assert_eq!(connection.domain_id, 1);
        assert_eq!(connection.strength, 1.0); // default_strength для strong
        assert!(connection.is_active());
    }

    #[test]
    fn test_connection_from_unknown_preset() {
        // Тест неизвестного пресета
        let result = Connection::from_preset("unknown", 1, 2, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_connection_validate_with_config() {
        // Тест валидации с конфигурацией
        let connection = Connection::from_preset("weak", 1, 2, 1).unwrap();
        assert!(connection.validate_with_config().is_ok());
        
        // Тест невалидного соединения (самосвязь)
        let self_loop = Connection::from_preset("temporal", 1, 1, 1).unwrap();
        assert!(self_loop.validate_with_config().is_err());
    }

    #[test]
    fn test_connection_presets_loading() {
        // Тест загрузки разных пресетов
        let strong = Connection::from_preset("strong", 1, 2, 1).unwrap();
        let weak = Connection::from_preset("weak", 2, 3, 1).unwrap();
        let temporal = Connection::from_preset("temporal", 3, 4, 1).unwrap();
        
        // Проверить разные значения по умолчанию
        assert_eq!(strong.strength, 1.0);
        assert_eq!(weak.strength, 0.3);
        assert_eq!(temporal.strength, 0.5);
        
        // Проверить elasticity (1.0 - decay_rate)
        assert_eq!(strong.elasticity, 0.999); // 1.0 - 0.001
        assert_eq!(weak.elasticity, 0.99);   // 1.0 - 0.01
        assert_eq!(temporal.elasticity, 0.995); // 1.0 - 0.005
    }

    #[test]
    fn test_connection_topology_constraints() {
        // Тест топологических ограничений
        let connection = Connection::from_preset("strong", 1, 2, 1).unwrap();
        
        // Нормальное соединение должно проходить валидацию
        assert!(connection.validate_with_config().is_ok());
        
        // Самосвязь не должна проходить валидацию
        let mut self_loop = connection;
        self_loop.source_id = 1;
        self_loop.target_id = 1;
        assert!(self_loop.validate_with_config().is_err());
    }
}
