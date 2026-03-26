// Copyright (C) 2024-2026 Chernov Denys
//
// PhysicsProcessor - обработчик UCL команд
// Реализует физическую семантику AXIOM

use crate::ucl_command::{UclCommand, UclResult, CommandStatus};
use crate::domain::{DomainConfig, StructuralRole};
use crate::com::COM;
use crate::arbiter::Arbiter;
use crate::token::Token;
use std::collections::HashMap;
// Time Model V1.0: Instant используется только для профилирования (метрика адаптера),
// не влияет на причинную логику ядра
use std::time::Instant;

/// Ошибки физического процессора
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum PhysicsError {
    UnknownOpcode = 1000,
    InvalidTarget = 1001,
    PhysicsViolation = 1002,
    InsufficientEnergy = 1003,
    MembraneBlocked = 1004,
    DomainNotFound = 1005,
    TokenNotFound = 1006,
    InvalidPayload = 1007,
}

/// Состояние физического процессора
pub struct PhysicsProcessor {
    pub domains: HashMap<u32, DomainConfig>,
    pub next_domain_id: u32,
    pub com: COM,
    /// Arbiter для dual-path маршрутизации (опционально)
    pub arbiter: Option<Arbiter>,
    /// Хранилище токенов (token_id -> Token)
    pub tokens: HashMap<u32, Token>,
    /// Счетчик для генерации token_id
    pub next_token_id: u32,
}

impl PhysicsProcessor {
    /// Создать новый процессор
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
            next_domain_id: 1000,
            com: COM::new(),
            arbiter: None,
            tokens: HashMap::new(),
            next_token_id: 1,
        }
    }

    /// Включить dual-path маршрутизацию через Arbiter
    ///
    /// Это активирует full Ashti_Core v2.0 архитектуру:
    /// SUTRA → EXPERIENCE → [ Arbiter ] → ASHTI(1-8) / MAYA(10)
    pub fn enable_routing(&mut self) -> Result<(), &'static str> {
        if self.arbiter.is_some() {
            return Err("Arbiter already enabled");
        }

        // Создаем Arbiter с копией domains и новым COM
        self.arbiter = Some(Arbiter::new(self.domains.clone(), COM::new()));

        Ok(())
    }

    /// Проверить, активен ли Arbiter
    pub fn is_routing_enabled(&self) -> bool {
        self.arbiter.is_some()
    }
    
    /// Главная точка входа - обработка команды
    pub fn execute(&mut self, command: &UclCommand) -> UclResult {
        let start_time = Instant::now();
        
        // Валидация команды
        if !command.is_valid() {
            return UclResult::error(
                command.command_id,
                CommandStatus::InvalidPayload,
                PhysicsError::InvalidPayload as u16,
            );
        }
        
        // Обработка команды
        let result = match command.opcode {
            1000 => self.spawn_domain(command),
            1001 => self.collapse_domain(command),
            1002 => self.lock_membrane(command),
            2000 => self.inject_token(command),
            2001 => self.apply_force(command),
            2002 => self.annihilate_token(command),
            3000 => self.tick_forward(command),
            3001 => self.change_temperature(command),
            4000 => self.process_token_dual_path(command),
            4001 => self.finalize_comparison(command),
            9000 => self.core_shutdown(command),
            _ => UclResult::error(
                command.command_id,
                CommandStatus::UnknownOpcode,
                PhysicsError::UnknownOpcode as u16,
            ),
        };
        
        // Обновляем время выполнения (метрика профилирования, не влияет на причинную логику)
        // Time Model V1.0: wall-clock метрики допустимы в адаптерах
        let mut final_result = result;
        final_result.execution_time_us = start_time.elapsed().as_micros() as u32;

        final_result
    }
    
    /// Рождение домена (SpawnDomain)
    fn spawn_domain(&mut self, command: &UclCommand) -> UclResult {
        let payload = command.get_payload::<crate::ucl_command::SpawnDomainPayload>();
        
        // Проверяем физические законы
        // factory_preset = 0 допустим только для SUTRA (structural_role = 0)
        println!("DEBUG: factory_preset={}, structural_role={}", payload.factory_preset, payload.structural_role);
        if payload.factory_preset == 0 && payload.structural_role != 0 {
            println!("ERROR: factory_preset=0 not allowed for structural_role={}", payload.structural_role);
            return UclResult::error(
                command.command_id,
                CommandStatus::InvalidPayload,
                PhysicsError::InvalidPayload as u16,
            );
        }
        
        // Создаем домен через factory метод
        let domain = match payload.structural_role {
            0 => DomainConfig::factory_sutra(self.next_domain_id as u16),
            3 => DomainConfig::factory_codex(self.next_domain_id as u16, payload.parent_domain_id),
            6 => DomainConfig::factory_logic(self.next_domain_id as u16, payload.parent_domain_id),
            7 => DomainConfig::factory_dream(self.next_domain_id as u16, payload.parent_domain_id),
            9 => DomainConfig::factory_experience(self.next_domain_id as u16, payload.parent_domain_id),
            10 => DomainConfig::factory_maya(self.next_domain_id as u16, payload.parent_domain_id),
            _ => DomainConfig::factory_sutra(self.next_domain_id as u16), // По умолчанию
        };
        
        // Проверяем валидность созданного домена
        if !domain.validate() {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::PhysicsViolation as u16,
            );
        }
        
        // Сохраняем домен
        self.domains.insert(self.next_domain_id, domain);
        
        let result = UclResult::success(command.command_id);
        self.next_domain_id += 1;
        
        result
    }
    
    /// Уничтожение домена (CollapseDomain)
    fn collapse_domain(&mut self, command: &UclCommand) -> UclResult {
        if !self.domains.contains_key(&command.target_id) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Проверяем, можно ли уничтожить домен
        let domain = self.domains.get(&command.target_id).unwrap();
        
        // SUTRA домен нельзя уничтожить (физический закон)
        if domain.structural_role == StructuralRole::Sutra as u8 {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::PhysicsViolation as u16,
            );
        }
        
        // Удаляем домен
        self.domains.remove(&command.target_id);
        
        UclResult::success(command.command_id)
    }
    
    /// Изменение мембраны (LockMembrane)
    fn lock_membrane(&mut self, command: &UclCommand) -> UclResult {
        if !self.domains.contains_key(&command.target_id) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Временно - просто возвращаем успех
        // TODO: Реализовать изменение мембраны
        UclResult::success(command.command_id)
    }
    
    /// Вброс токена (InjectToken)
    fn inject_token(&mut self, command: &UclCommand) -> UclResult {
        let payload = command.get_payload::<crate::ucl_command::InjectTokenPayload>();
        
        if !self.domains.contains_key(&(payload.target_domain_id as u32)) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Проверяем проницаемость мембраны
        let domain = self.domains.get(&(payload.target_domain_id as u32)).unwrap();
        
        if domain.permeability == 0 && (command.flags & 0x04) == 0 {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::MembraneBlocked as u16,
            );
        }
        
        // Проверяем температуру (токен не может быть горячее домена)
        if payload.temperature > domain.temperature + 100.0 {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::PhysicsViolation as u16,
            );
        }

        // Создаем токен из payload
        let token = Token {
            sutra_id: 0, // Будет установлен SUTRA
            domain_id: payload.target_domain_id,
            type_flags: payload.token_type as u16,
            position: [
                payload.position[0] as i16,
                payload.position[1] as i16,
                payload.position[2] as i16,
            ],
            velocity: [
                payload.velocity[0] as i16,
                payload.velocity[1] as i16,
                payload.velocity[2] as i16,
            ],
            target: [0; 3],
            reserved_phys: 0,
            valence: 0,
            mass: (payload.mass as u8).max(1),
            temperature: payload.temperature as u8,
            state: crate::token::STATE_ACTIVE,
            lineage_hash: 0,
            momentum: [0; 3],
            resonance: (payload.semantic_weight as u32),
            last_event_id: self.com.current_event_id(),
        };

        // Сохраняем токен в хранилище
        let token_id = self.next_token_id;
        self.tokens.insert(token_id, token);
        self.next_token_id += 1;

        UclResult::success(command.command_id)
    }
    
    /// Применение силы (ApplyForce)
    fn apply_force(&mut self, command: &UclCommand) -> UclResult {
        let payload = command.get_payload::<crate::ucl_command::ApplyForcePayload>();
        
        if !self.domains.contains_key(&command.target_id) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Проверяем физические законы
        let domain = self.domains.get(&command.target_id).unwrap();
        
        // В домене с нулевой гравитацией сила не работает
        if domain.gravity_strength == 0.0 && payload.force_type == 1 {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::PhysicsViolation as u16,
            );
        }
        
        // Проверяем энергию
        let required_energy = payload.magnitude * payload.duration_ticks as f32;
        if required_energy > 1000.0 { // Временно - максимальная энергия
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::InsufficientEnergy as u16,
            );
        }
        
        // Временно - просто возвращаем успех
        // TODO: Реализовать применение силы к токену
        let mut result = UclResult::success(command.command_id);
        result.consumed_energy = required_energy;
        result
    }
    
    /// Уничтожение токена (AnnihilateToken)
    fn annihilate_token(&mut self, command: &UclCommand) -> UclResult {
        if !self.domains.contains_key(&command.target_id) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Временно - просто возвращаем успех
        // TODO: Реализовать уничтожение токена
        UclResult::success(command.command_id)
    }
    
    /// Шаг симуляции (TickForward)
    fn tick_forward(&mut self, command: &UclCommand) -> UclResult {
        // Генерируем event_id через COM для пометки шага симуляции
        let _event_id = self.com.next_event_id(0);

        // Временно - просто возвращаем успех
        // TODO: Реализовать шаг симуляции с генерацией COM Event
        let mut result = UclResult::success(command.command_id);
        result.events_generated = 1; // Генерируем событие о шаге
        result
    }
    
    /// Изменение температуры (ChangeTemperature)
    fn change_temperature(&mut self, command: &UclCommand) -> UclResult {
        let payload = command.get_payload::<crate::ucl_command::ChangeTemperaturePayload>();
        
        if !self.domains.contains_key(&(payload.target_domain_id as u32)) {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::DomainNotFound as u16,
            );
        }
        
        // Проверяем физические законы
        let domain = self.domains.get_mut(&(payload.target_domain_id as u32)).unwrap();
        
        // Нельзя изменить температуру SUTRA домена (абсолютный ноль)
        if domain.structural_role == StructuralRole::Sutra as u8 && payload.delta_temperature != 0.0 {
            return UclResult::error(
                command.command_id,
                CommandStatus::PhysicsViolation,
                PhysicsError::PhysicsViolation as u16,
            );
        }
        
        // Применяем изменение температуры
        domain.temperature += payload.delta_temperature;
        
        // Температура не может быть отрицательной
        if domain.temperature < 0.0 {
            domain.temperature = 0.0;
        }
        
        // Временно - просто возвращаем успех
        let mut result = UclResult::success(command.command_id);
        result.consumed_energy = payload.delta_temperature.abs() * payload.transfer_rate;
        result
    }
    
    /// Остановка реактора (CoreShutdown)
    fn core_shutdown(&mut self, command: &UclCommand) -> UclResult {
        // Очищаем все домены
        self.domains.clear();

        UclResult::success(command.command_id)
    }

    /// Обработка токена через Arbiter dual-path (ProcessTokenDualPath)
    fn process_token_dual_path(&mut self, command: &UclCommand) -> UclResult {
        use crate::ucl_command::ProcessTokenPayload;

        // Проверяем, включен ли Arbiter
        if self.arbiter.is_none() {
            return UclResult::error(
                command.command_id,
                CommandStatus::SystemError,
                PhysicsError::PhysicsViolation as u16,
            );
        }

        let payload = command.get_payload::<ProcessTokenPayload>();

        // 1. Получаем токен из хранилища
        let token = match self.tokens.get(&payload.token_id) {
            Some(t) => t.clone(),
            None => {
                return UclResult::error(
                    command.command_id,
                    CommandStatus::TargetNotFound,
                    PhysicsError::TokenNotFound as u16,
                );
            }
        };

        // 2. Маршрутизируем через Arbiter
        let routing_result = if let Some(ref mut arbiter) = self.arbiter {
            arbiter.route_token(token, payload.source_domain)
        } else {
            return UclResult::error(
                command.command_id,
                CommandStatus::SystemError,
                PhysicsError::PhysicsViolation as u16,
            );
        };

        // 3. Генерируем результат
        let mut result = UclResult::success(command.command_id);
        result.events_generated = routing_result.routed_events.len() as u16;

        // Сохраняем consolidated результат обратно в хранилище (если есть)
        if let Some(consolidated) = routing_result.consolidated {
            self.tokens.insert(payload.token_id, consolidated);
        }

        result
    }

    /// Финализация сравнения reflex vs ASHTI и обучение (FinalizeComparison)
    fn finalize_comparison(&mut self, command: &UclCommand) -> UclResult {
        use crate::ucl_command::FinalizeComparisonPayload;

        // Проверяем, включен ли Arbiter
        if self.arbiter.is_none() {
            return UclResult::error(
                command.command_id,
                CommandStatus::SystemError,
                PhysicsError::PhysicsViolation as u16,
            );
        }

        let payload = command.get_payload::<FinalizeComparisonPayload>();

        // Вызываем finalize_comparison на Arbiter
        let finalize_result = if let Some(ref mut arbiter) = self.arbiter {
            arbiter.finalize_comparison(payload.event_id)
        } else {
            return UclResult::error(
                command.command_id,
                CommandStatus::SystemError,
                PhysicsError::PhysicsViolation as u16,
            );
        };

        // Проверяем результат
        if let Err(_err_msg) = finalize_result {
            return UclResult::error(
                command.command_id,
                CommandStatus::TargetNotFound,
                PhysicsError::TokenNotFound as u16,
            );
        }

        let mut result = UclResult::success(command.command_id);
        result.events_generated = 1; // Событие обучения

        // Генерируем событие обучения через COM (domain 9 = EXPERIENCE)
        let _learning_event_id = self.com.next_event_id(9);

        result
    }

    /// Получить домен по ID
    pub fn get_domain(&self, domain_id: u32) -> Option<&DomainConfig> {
        self.domains.get(&domain_id)
    }
    
    /// Получить список всех доменов
    pub fn list_domains(&self) -> Vec<(u32, &DomainConfig)> {
        self.domains.iter().map(|(&id, domain)| (id, domain)).collect()
    }
    
    /// Получить статистику
    pub fn get_stats(&self) -> PhysicsStats {
        PhysicsStats {
            total_domains: self.domains.len(),
            current_event_id: self.com.current_event_id(),
            next_domain_id: self.next_domain_id,
        }
    }
}

/// Статистика физического процессора
#[derive(Debug, Clone)]
pub struct PhysicsStats {
    pub total_domains: usize,
    pub current_event_id: u64,
    pub next_domain_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ucl_command::{UclBuilder, OpCode};
    
    #[test]
    fn test_physics_processor_creation() {
        let processor = PhysicsProcessor::new();
        assert_eq!(processor.domains.len(), 0);
        assert_eq!(processor.next_domain_id, 1000);
        assert_eq!(processor.com.current_event_id(), 0);
    }
    
    #[test]
    fn test_spawn_domain() {
        let mut processor = PhysicsProcessor::new();
        let command = UclBuilder::spawn_domain(0, 1); // SUTRA
        
        let result = processor.execute(&command);
        assert!(result.is_success());
        assert_eq!(processor.domains.len(), 1);
        assert_eq!(processor.next_domain_id, 1001);
    }
    
    #[test]
    fn test_spawn_different_domains() {
        let mut processor = PhysicsProcessor::new();
        
        // SUTRA
        let sutra = UclBuilder::spawn_domain(0, 0);
        let result = processor.execute(&sutra);
        assert!(result.is_success());
        
        // LOGIC
        let logic = UclBuilder::spawn_domain(0, 6);
        let result = processor.execute(&logic);
        assert!(result.is_success());
        
        // DREAM
        let dream = UclBuilder::spawn_domain(0, 7);
        let result = processor.execute(&dream);
        assert!(result.is_success());
        
        assert_eq!(processor.domains.len(), 3);
    }
    
    #[test]
    fn test_collapse_domain() {
        let mut processor = PhysicsProcessor::new();
        
        // Создаем домен
        let create = UclBuilder::spawn_domain(0, 6); // LOGIC
        let result = processor.execute(&create);
        assert!(result.is_success());
        
        // Уничтожаем домен
        let collapse = UclCommand::new(OpCode::CollapseDomain, 1000, 100, 0);
        let result = processor.execute(&collapse);
        assert!(result.is_success());
        assert_eq!(processor.domains.len(), 0);
    }
    
    #[test]
    fn test_collapse_sutra_forbidden() {
        let mut processor = PhysicsProcessor::new();
        
        // Создаем SUTRA
        let create = UclBuilder::spawn_domain(0, 0);
        let result = processor.execute(&create);
        assert!(result.is_success());
        
        // Пытаемся уничтожить SUTRA
        let collapse = UclCommand::new(OpCode::CollapseDomain, 1000, 100, 0);
        let result = processor.execute(&collapse);
        assert!(!result.is_success());
        assert_eq!(result.status, CommandStatus::PhysicsViolation as u8);
    }
    
    #[test]
    fn test_change_temperature() {
        let mut processor = PhysicsProcessor::new();
        
        // Создаем LOGIC домен
        let create = UclBuilder::spawn_domain(0, 6);
        let result = processor.execute(&create);
        assert!(result.is_success());
        
        // Изменяем температуру
        let payload = crate::ucl_command::ChangeTemperaturePayload {
            target_domain_id: 1000,
            delta_temperature: 10.0,
            transfer_rate: 1.0,
            source_point: [0.0, 0.0, 0.0],
            radius: 100.0,
            duration_ticks: 1,
            reserved: [0; 14],
        };
        
        let command = UclCommand::new(OpCode::ChangeTemperature, 1000, 100, 0)
            .with_payload(&payload);
        
        let result = processor.execute(&command);
        assert!(result.is_success());
        
        let domain = processor.get_domain(1000).unwrap();
        assert_eq!(domain.temperature, 283.0); // 273.0 + 10.0
    }
    
    #[test]
    fn test_unknown_opcode() {
        let mut processor = PhysicsProcessor::new();
        let command = UclCommand::new(OpCode::CoreReset, 0, 100, 0);

        let result = processor.execute(&command);
        assert!(!result.is_success());
        assert_eq!(result.status, CommandStatus::UnknownOpcode as u8);
    }

    #[test]
    fn test_enable_routing() {
        let mut processor = PhysicsProcessor::new();

        // Изначально routing выключен
        assert!(!processor.is_routing_enabled());

        // Включаем routing
        let result = processor.enable_routing();
        assert!(result.is_ok());
        assert!(processor.is_routing_enabled());

        // Повторное включение должно вернуть ошибку
        let result = processor.enable_routing();
        assert!(result.is_err());
    }

    #[test]
    fn test_inject_and_store_token() {
        let mut processor = PhysicsProcessor::new();

        // Создаем домен
        let create = UclBuilder::spawn_domain(0, 6); // LOGIC
        let result = processor.execute(&create);
        assert!(result.is_success());

        // Инжектируем токен
        let payload = crate::ucl_command::InjectTokenPayload {
            target_domain_id: 1000,
            token_type: 1,
            mass: 10.0,
            position: [1.0, 2.0, 3.0],
            velocity: [0.5, 0.5, 0.5],
            semantic_weight: 50.0,
            temperature: 20.0,
            reserved: [0; 6],
        };

        let command = UclCommand::new(OpCode::InjectToken, 1000, 100, 0)
            .with_payload(&payload);

        let result = processor.execute(&command);
        assert!(result.is_success());

        // Проверяем, что токен сохранен
        assert_eq!(processor.tokens.len(), 1);
        assert!(processor.tokens.contains_key(&1));
    }

    #[test]
    fn test_dual_path_processing() {
        use crate::ucl_command::ProcessTokenPayload;

        let mut processor = PhysicsProcessor::new();

        // 1. Создаем EXPERIENCE домен
        let create = UclBuilder::spawn_domain(0, 9); // EXPERIENCE
        let result = processor.execute(&create);
        assert!(result.is_success());

        // 2. Включаем routing
        let result = processor.enable_routing();
        assert!(result.is_ok());

        // 3. Инжектируем токен
        let inject_payload = crate::ucl_command::InjectTokenPayload {
            target_domain_id: 1000,
            token_type: 1,
            mass: 10.0,
            position: [1.0, 2.0, 3.0],
            velocity: [0.5, 0.5, 0.5],
            semantic_weight: 50.0,
            temperature: 20.0,
            reserved: [0; 6],
        };

        let command = UclCommand::new(OpCode::InjectToken, 1000, 100, 0)
            .with_payload(&inject_payload);

        let result = processor.execute(&command);
        assert!(result.is_success());

        // 4. Обрабатываем через dual-path
        let process_payload = ProcessTokenPayload {
            token_id: 1,
            source_domain: 9, // EXPERIENCE
            enable_learning: 1,
            reserved: [0; 42],
        };

        let command = UclCommand::new(OpCode::ProcessTokenDualPath, 0, 100, 0)
            .with_payload(&process_payload);

        let result = processor.execute(&command);
        // Arbiter вернет ошибку если не все домены зарегистрированы,
        // но opcode должен выполниться без системных ошибок
        assert!(result.is_success());
    }

    #[test]
    fn test_finalize_comparison_opcode() {
        use crate::ucl_command::FinalizeComparisonPayload;

        let mut processor = PhysicsProcessor::new();

        // Включаем routing
        let result = processor.enable_routing();
        assert!(result.is_ok());

        // Пытаемся финализировать несуществующее сравнение
        let payload = FinalizeComparisonPayload {
            event_id: 12345,
            reserved: [0; 40],
        };

        let command = UclCommand::new(OpCode::FinalizeComparison, 0, 100, 0)
            .with_payload(&payload);

        let result = processor.execute(&command);
        // Должно вернуть ошибку, так как event_id не существует
        assert!(!result.is_success());
    }

    #[test]
    fn test_token_storage() {
        let mut processor = PhysicsProcessor::new();

        assert_eq!(processor.tokens.len(), 0);
        assert_eq!(processor.next_token_id, 1);

        // Создаем домен и инжектируем несколько токенов
        let create = UclBuilder::spawn_domain(0, 6);
        processor.execute(&create);

        for i in 0..5 {
            let payload = crate::ucl_command::InjectTokenPayload {
                target_domain_id: 1000,
                token_type: i,
                mass: 10.0 + i as f32,
                position: [i as f32, 0.0, 0.0],
                velocity: [0.0; 3],
                semantic_weight: 50.0,
                temperature: 20.0,
                reserved: [0; 6],
            };

            let command = UclCommand::new(OpCode::InjectToken, 1000, 100, 0)
                .with_payload(&payload);

            processor.execute(&command);
        }

        assert_eq!(processor.tokens.len(), 5);
        assert_eq!(processor.next_token_id, 6);
    }
}
