// Copyright (C) 2024-2026 Chernov Denys
//
// Axiom Core Runtime Library

mod domain;
mod ucl_command;
mod physics_processor;
mod ffi;
mod token;
mod connection;
mod event;
mod com;
mod clock;
mod upo;
mod config;
mod experience;

pub use domain::{
    DomainConfig, StructuralRole, DomainType,
    DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY,
    PROCESSING_IDLE, PROCESSING_ACTIVE, PROCESSING_FROZEN,
    MEMBRANE_OPEN, MEMBRANE_CLOSED, MEMBRANE_SEMI
};

pub use ucl_command::{
    UclCommand, UclResult, OpCode, CommandStatus,
    UclBuilder, SpawnDomainPayload, ApplyForcePayload,
    InjectTokenPayload, ChangeTemperaturePayload
};

pub use physics_processor::{
    PhysicsProcessor, PhysicsStats, PhysicsError
};

pub use token::{
    Token, STATE_ACTIVE, STATE_SLEEPING, STATE_LOCKED
};

pub use connection::{
    Connection, FLAG_ACTIVE, FLAG_INHIBITED, FLAG_TEMPORARY, FLAG_CRITICAL
};

pub use event::{
    Event, EventType, EventPriority, Timeline
};

pub use com::{
    COM
};

pub use clock::{
    CausalClock
};

pub use upo::{
    DynamicTrace, TraceSourceType, Screen, TRACE_ACTIVE, TRACE_FADING, TRACE_LOCKED, TRACE_ETERNAL
};

pub use experience::{
    Experience, ExperienceTrace, Skill, ResonanceLevel, ResonanceResult
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ucl_system() {
        // Тестируем UCL V2.0 систему
        use crate::ucl_command::*;
        use crate::physics_processor::*;
        
        let mut processor = PhysicsProcessor::new();
        
        // Создаем SUTRA домен
        let sutra = UclBuilder::spawn_domain(0, 0);
        let result = processor.execute(&sutra);
        println!("SUTRA result: status={}, error_code={}", result.status, result.error_code);
        assert!(result.is_success());
        
        // Создаем LOGIC домен
        let logic = UclBuilder::spawn_domain(0, 6);
        let result = processor.execute(&logic);
        println!("LOGIC result: status={}, error_code={}", result.status, result.error_code);
        assert!(result.is_success());
        
        // Получаем статистику
        let stats = processor.get_stats();
        assert_eq!(stats.total_domains, 2);
        
        // Применяем силу к LOGIC домену
        let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 10.0);
        let result = processor.execute(&force);
        println!("Force result: status={}, error_code={}", result.status, result.error_code);
        assert!(result.is_success());
        assert!(result.consumed_energy > 0.0);
    }

    #[test]
    fn test_ffi_interface() {
        // Тестируем FFI интерфейс
        let mut command_buffer = [0u8; 64];
        let mut result_buffer = [0u8; 32];
        
        // Создаем команду через FFI
        let result = unsafe {
            crate::ffi::ucl_spawn_domain(
                command_buffer.as_mut_ptr(),
                123,
                6, // LOGIC
                0,
            )
        };
        
        assert_eq!(result, 0);
        
        // Выполняем команду через FFI
        let result = unsafe {
            crate::ffi::ucl_execute(
                command_buffer.as_ptr(),
                result_buffer.as_mut_ptr(),
            )
        };
        
        assert_eq!(result, 0);
        
        // Проверяем результат
        let ucl_result = unsafe {
            std::ptr::read_unaligned(result_buffer.as_ptr() as *const UclResult)
        };
        
        assert!(ucl_result.is_success());
    }
}
