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
mod event_generator;
mod causal_frontier;
mod heartbeat;
mod com;
mod clock;
mod upo;
mod config;
mod experience;
mod arbiter;
mod ashti_processor;
mod maya_processor;

pub use domain::{
    Domain, DomainConfig, StructuralRole, DomainType,
    DOMAIN_ACTIVE, DOMAIN_LOCKED, DOMAIN_TEMPORARY,
    PROCESSING_IDLE, PROCESSING_ACTIVE, PROCESSING_FROZEN,
    MEMBRANE_OPEN, MEMBRANE_CLOSED, MEMBRANE_SEMI
};

pub use ucl_command::{
    UclCommand, UclResult, OpCode, CommandStatus,
    UclBuilder, SpawnDomainPayload, ApplyForcePayload,
    InjectTokenPayload, ChangeTemperaturePayload,
    ProcessTokenPayload, FinalizeComparisonPayload
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

pub use event_generator::{
    EventGenerator
};

pub use causal_frontier::{
    CausalFrontier, FrontierState
};

pub use heartbeat::{
    HeartbeatGenerator, HeartbeatConfig, handle_heartbeat
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

pub use arbiter::{
    Arbiter, RoutingResult
};

pub use ashti_processor::{
    AshtiProcessor
};

pub use maya_processor::{
    MayaProcessor
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

    // --- Time Model V1.0 Cross-Spec Validation Tests ---

    #[test]
    fn test_time_model_token_uses_event_id_for_age() {
        // Time Model V1.0: Token использует только event_id для определения возраста
        let mut token = Token::default();
        token.sutra_id = 1;
        token.domain_id = 1;
        token.last_event_id = 10;

        let mut com = COM::new();
        // Продвигаем COM вперёд
        for _ in 0..100 {
            com.next_event_id(1);
        }

        // Причинный возраст вычисляется через event_id разницу
        let causal_age = com.compute_causal_age(token.last_event_id);
        assert_eq!(causal_age, 90, "Causal age should be computed via event_id difference (100 - 10)");
    }

    #[test]
    fn test_time_model_connection_uses_event_id() {
        // Time Model V1.0: Connection использует event_id, не timestamps
        let mut connection = Connection::default();
        connection.source_id = 1;
        connection.target_id = 2;
        connection.domain_id = 1;
        connection.created_at = 50; // event_id
        connection.last_event_id = 100; // event_id

        // Проверяем что все временные поля это event_id
        assert!(connection.created_at > 0);
        assert!(connection.last_event_id >= connection.created_at);

        // Вычисляем причинный возраст через event_id разницу
        let age = connection.last_event_id - connection.created_at;
        assert_eq!(age, 50);
    }

    #[test]
    fn test_time_model_domain_config_event_ids() {
        // Time Model V1.0: DomainConfig использует event_id для метаданных
        let config = DomainConfig::factory_logic(6, 1);

        // created_at и last_update это event_id (могут быть 0 для конфигураций)
        assert_eq!(config.created_at, 0); // Не установлено до создания через COM
        assert_eq!(config.last_update, 0);

        // После инициализации через COM, они будут содержать event_id
        let mut config_with_events = config;
        config_with_events.created_at = 100; // event_id создания
        config_with_events.last_update = 150; // event_id обновления

        assert!(config_with_events.validate());
    }

    #[test]
    fn test_time_model_decay_uses_causal_age() {
        // Time Model V1.0: затухание вычисляется через причинный возраст
        let mut token = Token::default();
        token.sutra_id = 1;
        token.domain_id = 1;
        token.last_event_id = 10;
        token.valence = 10;

        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(10000); // Большой причинный возраст

        // Затухание должно определяться причинным возрастом (event_id разница)
        let decay_event = event_generator.check_decay(&token, 0.01);
        assert!(decay_event.is_some(), "Decay should be triggered by causal age");

        let event = decay_event.unwrap();
        assert_eq!(event.event_type, EventType::TokenDecayed as u16);
    }

    #[test]
    fn test_time_model_heartbeat_is_causal() {
        // Time Model V1.0: Heartbeat - легитимная причинность
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = HeartbeatConfig {
            interval: 5,
            ..HeartbeatConfig::medium()
        };
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Heartbeat генерируется по COUNT событий, не по wall-clock времени
        for _ in 0..4 {
            assert!(domain.on_event().is_none());
        }

        let pulse = domain.on_event();
        assert!(pulse.is_some(), "Heartbeat triggered by event count");
        assert_eq!(pulse.unwrap(), 1);

        // Это легитимная причинность: "прошло N событий" → новое событие
    }

    #[test]
    fn test_time_model_no_wall_clock_in_core_structs() {
        // Time Model V1.0: core структуры не содержат wall-clock времени

        // Token: нет timestamp полей
        let token = Token::default();
        assert_eq!(token.last_event_id, 0); // event_id, не timestamp

        // Connection: нет timestamp полей
        let connection = Connection::default();
        assert_eq!(connection.created_at, 0); // event_id, не timestamp
        assert_eq!(connection.last_event_id, 0);

        // Domain: нет timestamp полей (created_at/last_update это event_id)
        let config = DomainConfig::factory_logic(6, 1);
        assert_eq!(config.created_at, 0); // event_id, не Unix timestamp
        assert_eq!(config.last_update, 0);

        // Event: pulse_id это монотонный счётчик, не timestamp
        let event = Event::with_pulse(
            1, 1, EventType::TokenCreate, EventPriority::Normal,
            0x1234, 1, 0, 0, 42
        );
        assert_eq!(event.pulse_id, 42); // Номер пульса, не timestamp
    }

    #[test]
    fn test_time_model_com_monotonic_causality() {
        // Time Model V1.0: COM обеспечивает монотонность причинного порядка
        let mut com = COM::new();

        let id1 = com.next_event_id(1);
        let id2 = com.next_event_id(1);
        let id3 = com.next_event_id(2);

        // Монотонность
        assert!(id2 > id1);
        assert!(id3 > id2);

        // Все event_id строго упорядочены
        assert_eq!(com.current_event_id(), id3);
    }

    #[test]
    fn test_time_model_gravity_uses_causal_age() {
        // Time Model V1.0: гравитация не зависит от wall-clock времени
        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 10.0;

        let mut heartbeat_config = HeartbeatConfig::medium();
        heartbeat_config.enable_gravity = true;
        heartbeat_config.enable_decay = false;

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        let mut token = Token::default();
        token.sutra_id = 1;
        token.domain_id = 6;

        let tokens = vec![token];
        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        domain.frontier.push_token(0);

        // Гравитация генерируется через причинный порядок, не через время
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        let has_gravity = events.iter().any(|e| e.event_type == EventType::GravityUpdate as u16);
        assert!(has_gravity, "Gravity computed via causal order");
    }

    // --- Phase 5: Integration & Performance Tests ---

    #[test]
    fn test_integration_full_causal_time_system() {
        // Полная интеграция: COM → Event-Driven → Causal Frontier → Heartbeat
        let mut com = COM::new();
        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 1.0;

        let heartbeat_config = HeartbeatConfig {
            interval: 10,
            batch_size: 5,
            enable_decay: true,
            enable_gravity: true,
            enable_connection_maintenance: false,
            ..HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);
        domain.active_tokens = 20;

        // Создаем токены
        let tokens: Vec<Token> = (0..20).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.last_event_id = 0;
            token.valence = 5;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();

        // Симулируем полный lifecycle
        for cycle in 0..30 {
            let event_id = com.next_event_id(6);
            event_generator.set_event_id(event_id);

            if let Some(pulse) = domain.on_event() {
                event_generator.set_pulse_id(pulse);
                domain.handle_heartbeat(pulse);

                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

                // Применяем события через COM
                for event in events {
                    com.apply_event(event);
                }

                domain.frontier.reset_cycle();
            }
        }

        // Проверка: система корректно обработала все циклы
        assert!(com.current_event_id() >= 30);
        assert!(domain.current_pulse() >= 3);
    }

    #[test]
    fn test_performance_o_active_entities() {
        // Causal Frontier V1: O(active_entities) сложность
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Большое количество токенов
        let tokens: Vec<Token> = (0..10000).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем только 100 активных токенов
        for i in 0..100 {
            domain.frontier.push_token(i);
        }

        // Обработка должна затронуть только активные токены (~100)
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Проверка: обработаны только активные, не все 10000
        // Количество событий должно быть пропорционально активным сущностям
        assert!(events.len() <= 200, "Should process only active entities, not all");
    }

    #[test]
    fn test_performance_idle_state_zero_cpu() {
        // Causal Frontier V1, раздел 8: Idle state
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        let tokens = vec![];
        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Пустой frontier → idle state
        assert!(domain.frontier.is_empty());

        // Обработка пустого frontier не должна производить вычислений
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        assert!(events.is_empty());
        assert!(domain.frontier.is_empty());
        // В реальной системе это означает нулевую нагрузку CPU
    }

    #[test]
    fn test_determinism_reproducible_simulation() {
        // Детерминизм: одинаковый input → одинаковый output
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = HeartbeatConfig {
            interval: 5,
            batch_size: 2,
            enable_decay: true,
            enable_gravity: false,
            ..HeartbeatConfig::medium()
        };

        // Запуск 1
        let mut domain1 = Domain::with_heartbeat(config, heartbeat_config);
        domain1.active_tokens = 10;

        let tokens: Vec<Token> = (0..10).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.last_event_id = 10;
            token.valence = 5;
            token
        }).collect();

        let connections = vec![];
        let mut gen1 = EventGenerator::new();
        gen1.set_event_id(1000);

        for _ in 0..5 {
            domain1.on_event();
        }
        domain1.handle_heartbeat(1);
        let events1 = domain1.process_frontier(&tokens, &connections, &mut gen1);

        // Запуск 2 (тот же input)
        let mut domain2 = Domain::with_heartbeat(config, heartbeat_config);
        domain2.active_tokens = 10;

        let mut gen2 = EventGenerator::new();
        gen2.set_event_id(1000);

        for _ in 0..5 {
            domain2.on_event();
        }
        domain2.handle_heartbeat(1);
        let events2 = domain2.process_frontier(&tokens, &connections, &mut gen2);

        // Проверка детерминизма: одинаковое количество событий
        assert_eq!(events1.len(), events2.len());

        // Одинаковые event_type
        for (e1, e2) in events1.iter().zip(events2.iter()) {
            assert_eq!(e1.event_type, e2.event_type);
            assert_eq!(e1.target_id, e2.target_id);
        }
    }
}
