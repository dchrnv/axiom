// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Token V5.2: docs/spec/Token V5.2.md

use crate::config::initialize;

/// Состояние Token: Active, Sleeping, Locked...
pub const STATE_ACTIVE: u8 = 1;
pub const STATE_SLEEPING: u8 = 2;
pub const STATE_LOCKED: u8 = 3;

/// Token — 64 байта, выравнивание 64.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Token {
    // --- ИДЕНТИФИКАЦИЯ (8 Байт) ---
    pub sutra_id: u32,
    pub domain_id: u16,
    pub type_flags: u16,

    // --- ЛОКАЛЬНАЯ ФИЗИКА ПОЛЯ (16 Байт) ---
    pub position: [i16; 3],
    pub velocity: [i16; 3],
    pub target: [i16; 3],
    pub reserved_phys: u16,

    // --- ТЕРМОДИНАМИКА (4 Байта) ---
    pub valence: i8,
    pub mass: u8,
    pub temperature: u8,
    pub state: u8,

    // --- ФРАКТАЛЬНАЯ НАВИГАЦИЯ (36 Байт) ---
    pub lineage_hash: u64,
    pub momentum: [i32; 3],
    pub resonance: u32,
    pub last_event_id: u64,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            sutra_id: 0,
            domain_id: 0,
            type_flags: 0,
            position: [0; 3],
            velocity: [0; 3],
            target: [0; 3],
            reserved_phys: 0,
            valence: 0,
            mass: 1,
            temperature: 0,
            state: STATE_ACTIVE,
            lineage_hash: 0,
            momentum: [0; 3],
            resonance: 0,
            last_event_id: 0,
        }
    }
}

impl Token {
    pub fn new(sutra_id: u32, domain_id: u16) -> Self {
        Self {
            sutra_id,
            domain_id,
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.state == STATE_ACTIVE
    }

    #[inline]
    pub fn is_sleeping(&self) -> bool {
        self.state == STATE_SLEEPING
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.state == STATE_LOCKED
    }

    /// Валидация согласно спецификации Token V5.1
    pub fn validate(&self) -> bool {
        self.sutra_id > 0
        && self.domain_id > 0
        && self.mass > 0
        && self.last_event_id > 0
        && self.position.iter().all(|&p| p >= i16::MIN && p <= i16::MAX)
    }

    /// Обновление momentum через COM событие
    pub fn update_momentum(&mut self, force: [i32; 3], event_id: u64) {
        self.momentum[0] += force[0];
        self.momentum[1] += force[1];
        self.momentum[2] += force[2];
        self.last_event_id = event_id;
    }

    pub fn compute_resonance(&self) -> u32 {
        (self.momentum[0].pow(2) + self.momentum[1].pow(2) + self.momentum[2].pow(2)) as u32
    }

    /// Вычисление резонанса с другим токеном
    pub fn compute_resonance_with(&self, other: &Token) -> u32 {
        let freq_diff = (self.resonance as i32 - other.resonance as i32).abs();
        (1000 - freq_diff.min(999)) as u32
    }

    /// Создать токен из пресета согласно Configuration System
    pub fn from_preset(preset_name: &str, sutra_id: u32, domain_id: u16) -> Result<Self, crate::config::ConfigError> {
        let config = initialize()?;
        let mut loader = crate::config::ConfigLoader::new();
        
        // Загрузить схему токенов
        let schema = loader.load_schema("token", 
            std::path::Path::new(&config.schema.token))?;
        
        // Получить пресет из схемы
        if let Some(token_types) = schema.get("token_types") {
            if let Some(token_types_array) = token_types.as_sequence() {
                for token_type in token_types_array {
                    if let Some(name) = token_type.get("name") {
                        if let Some(name_str) = name.as_str() {
                            if name_str == preset_name {
                                // Получить значения по умолчанию из пресета
                                let default_momentum = token_type.get("default_momentum")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(1.0) as i32;
                                
                                let default_resonance = token_type.get("default_resonance")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(440.0) as u32;
                                
                                println!("Creating token from preset: {} (momentum: {}, resonance: {})", 
                                    preset_name, default_momentum, default_resonance);
                                
                                let mut token = Self::new(sutra_id, domain_id);
                                token.momentum = [default_momentum, 0, 0];
                                token.resonance = default_resonance;
                                token.last_event_id = 1; // Установить валидный event_id
                                
                                return Ok(token);
                            }
                        }
                    }
                }
            }
        }
        
        Err(crate::config::ConfigError::ValidationError(format!(
            "Unknown token preset: {}", preset_name
        )))
    }

    /// Валидация токена с использованием конфигурации
    pub fn validate_with_config(&self) -> Result<(), crate::config::ConfigError> {
        // Базовая валидация
        if !self.validate() {
            return Err(crate::config::ConfigError::ValidationError(
                "Token failed basic validation".to_string()
            ));
        }

        // Загрузить схему для дополнительной валидации
        let config = initialize()?;
        let mut loader = crate::config::ConfigLoader::new();
        let schema = loader.load_schema("token", 
            std::path::Path::new(&config.schema.token))?;

        // Проверить physics constraints
        if let Some(_constraints) = schema.get("physics_constraints") {
            // В реальной реализации здесь будет проверка ограничений
            // Например: conservation of momentum, resonance harmony, velocity limits
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_preset() {
        // Тест создания токена из пресета
        let result = Token::from_preset("concept", 1, 1);
        assert!(result.is_ok());
        
        let token = result.unwrap();
        assert_eq!(token.sutra_id, 1);
        assert_eq!(token.domain_id, 1);
        assert_eq!(token.momentum[0], 1); // default_momentum для concept
        assert_eq!(token.resonance, 440); // default_resonance для concept
    }

    #[test]
    fn test_token_from_unknown_preset() {
        // Тест неизвестного пресета
        let result = Token::from_preset("unknown", 1, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_validate_with_config() {
        // Тест валидации с конфигурацией
        let token = Token::from_preset("relation", 1, 1).unwrap();
        assert!(token.validate_with_config().is_ok());
        
        // Тест невалидного токена
        let mut invalid_token = token;
        invalid_token.sutra_id = 0; // Невалидный ID
        assert!(invalid_token.validate_with_config().is_err());
    }

    #[test]
    fn test_token_presets_loading() {
        // Тест загрузки разных пресетов
        let concept = Token::from_preset("concept", 1, 1).unwrap();
        let relation = Token::from_preset("relation", 2, 1).unwrap();
        let context = Token::from_preset("context", 3, 1).unwrap();
        
        // Проверить разные значения по умолчанию
        assert_eq!(concept.resonance, 440);
        assert_eq!(relation.resonance, 220);
        assert_eq!(context.resonance, 880);
        
        assert_eq!(concept.momentum[0], 1);
        assert_eq!(relation.momentum[0], 0); // 0.8 округлится до 0
        assert_eq!(context.momentum[0], 0); // 0.5 округлится до 0
    }
}
