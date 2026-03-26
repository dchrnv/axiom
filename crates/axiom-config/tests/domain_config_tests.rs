use axiom_config::{
    DomainConfig, DomainType, StructuralRole,
    PROCESSING_IDLE, PROCESSING_ACTIVE, PROCESSING_FROZEN,
    MEMBRANE_OPEN, MEMBRANE_SEMI, MEMBRANE_CLOSED, MEMBRANE_ADAPTIVE,
};

#[test]
fn test_domain_config_size() {
    assert_eq!(std::mem::size_of::<DomainConfig>(), 128);
    assert_eq!(std::mem::align_of::<DomainConfig>(), 128);
}

#[test]
fn test_domain_config_default() {
    let config = DomainConfig::default();
    assert_eq!(config.domain_id, 1);
    assert_eq!(config.structural_role, StructuralRole::Logic as u8);
    assert_eq!(config.token_capacity, 1000);
    assert_eq!(config.connection_capacity, 5000);
}

#[test]
fn test_domain_config_void() {
    let config = DomainConfig::default_void();
    assert_eq!(config.domain_id, 0);
    assert_eq!(config.structural_role, StructuralRole::Void as u8);
    assert_eq!(config.token_capacity, 0);
    assert_eq!(config.gravity_strength, 0.0);
}

#[test]
fn test_domain_config_new() {
    let config = DomainConfig::new(5, DomainType::Memory, StructuralRole::Experience);
    assert_eq!(config.domain_id, 5);
    assert_eq!(config.domain_type, DomainType::Memory as u8);
    assert_eq!(config.structural_role, StructuralRole::Experience as u8);
}

#[test]
fn test_domain_config_sutra() {
    let config = DomainConfig::factory_sutra(1);
    assert_eq!(config.domain_id, 1);
    assert_eq!(config.structural_role, StructuralRole::Sutra as u8);
    assert_eq!(config.gravity_strength, f32::MAX);
    assert_eq!(config.temperature, 0.0);
    assert_eq!(config.permeability, 0);
    assert_eq!(config.membrane_state, 2); // CLOSED
}

#[test]
fn test_domain_config_validation() {
    let config = DomainConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_domain_config_validation_negative_field_size() {
    let mut config = DomainConfig::default();
    config.field_size[0] = -1.0;
    assert!(config.validate().is_err());
}

#[test]
fn test_domain_config_validation_zero_capacity() {
    let mut config = DomainConfig::default();
    config.token_capacity = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_structural_role_enum() {
    assert_eq!(StructuralRole::Sutra as u8, 0);
    assert_eq!(StructuralRole::Logic as u8, 6);
    assert_eq!(StructuralRole::Maya as u8, 10);
}

#[test]
fn test_domain_type_enum() {
    assert_eq!(DomainType::Logic as u16, 1);
    assert_eq!(DomainType::Memory as u16, 5);
}

#[test]
fn test_processing_state_constants() {
    assert_eq!(PROCESSING_IDLE, 1);
    assert_eq!(PROCESSING_ACTIVE, 2);
    assert_eq!(PROCESSING_FROZEN, 3);
}

#[test]
fn test_membrane_constants() {
    assert_eq!(MEMBRANE_OPEN, 0);
    assert_eq!(MEMBRANE_SEMI, 1);
    assert_eq!(MEMBRANE_CLOSED, 2);
    assert_eq!(MEMBRANE_ADAPTIVE, 3);
}

#[test]
fn test_all_factory_methods_valid() {
    let configs = vec![
        DomainConfig::factory_sutra(1),
        DomainConfig::factory_execution(1, 0),
        DomainConfig::factory_shadow(2, 0),
        DomainConfig::factory_codex(3, 1),
        DomainConfig::factory_map(4, 0),
        DomainConfig::factory_probe(5, 0),
        DomainConfig::factory_logic(6, 1),
        DomainConfig::factory_dream(7, 1),
        DomainConfig::factory_void(8, 0),
        DomainConfig::factory_experience(9, 1),
        DomainConfig::factory_maya(10, 1),
    ];
    for config in &configs {
        assert!(
            config.validate().is_ok(),
            "factory for role {} produced invalid config: {:?}",
            config.structural_role,
            config.validate()
        );
    }
}

#[test]
fn test_factory_execution() {
    let config = DomainConfig::factory_execution(1, 0);
    assert_eq!(config.structural_role, StructuralRole::Execution as u8);
    assert_eq!(config.membrane_state, MEMBRANE_SEMI);
    assert_eq!(config.token_capacity, 5000);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_shadow() {
    let config = DomainConfig::factory_shadow(2, 0);
    assert_eq!(config.structural_role, StructuralRole::Shadow as u8);
    assert_eq!(config.membrane_state, MEMBRANE_CLOSED);
    assert_eq!(config.temperature, 250.0);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_codex() {
    let config = DomainConfig::factory_codex(3, 1);
    assert_eq!(config.structural_role, StructuralRole::Codex as u8);
    assert_eq!(config.membrane_state, MEMBRANE_CLOSED);
    assert_eq!(config.temperature, 10.0);
    assert_eq!(config.arbiter_flags, 0b00000000);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_map() {
    let config = DomainConfig::factory_map(4, 0);
    assert_eq!(config.structural_role, StructuralRole::Map as u8);
    assert_eq!(config.membrane_state, MEMBRANE_CLOSED);
    assert_eq!(config.token_capacity, 10000);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_probe() {
    let config = DomainConfig::factory_probe(5, 0);
    assert_eq!(config.structural_role, StructuralRole::Probe as u8);
    assert_eq!(config.membrane_state, MEMBRANE_OPEN);
    assert_eq!(config.temperature, 350.0);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_logic() {
    let config = DomainConfig::factory_logic(6, 1);
    assert_eq!(config.structural_role, StructuralRole::Logic as u8);
    assert_eq!(config.membrane_state, MEMBRANE_ADAPTIVE);
    assert_eq!(config.temperature, 273.0);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_dream() {
    let config = DomainConfig::factory_dream(7, 1);
    assert_eq!(config.structural_role, StructuralRole::Dream as u8);
    assert_eq!(config.membrane_state, MEMBRANE_OPEN);
    assert_eq!(config.gravity_strength, 0.0);
    assert_eq!(config.quantum_noise, 200);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_void() {
    let config = DomainConfig::factory_void(8, 0);
    assert_eq!(config.structural_role, StructuralRole::Void as u8);
    assert_eq!(config.membrane_state, MEMBRANE_OPEN);
    assert_eq!(config.temperature, 1000.0);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_experience() {
    let config = DomainConfig::factory_experience(9, 1);
    assert_eq!(config.structural_role, StructuralRole::Experience as u8);
    assert_eq!(config.membrane_state, MEMBRANE_SEMI);
    assert_eq!(config.field_size, [5000.0, 5000.0, 5000.0]);
    assert_eq!(config.resonance_freq, 1000);
    assert_eq!(config.token_capacity, 100000);
    assert_eq!(config.connection_capacity, 50000);
    assert!(config.validate().is_ok());
}

#[test]
fn test_factory_maya() {
    let config = DomainConfig::factory_maya(10, 1);
    assert_eq!(config.structural_role, StructuralRole::Maya as u8);
    assert_eq!(config.membrane_state, MEMBRANE_OPEN);
    assert_eq!(config.permeability, 255);
    assert!(config.validate().is_ok());
}

#[test]
fn test_is_active_locked_temporary() {
    let mut config = DomainConfig::default();
    config.flags = 0;
    assert!(!config.is_active());

    config.flags = 1; // DOMAIN_ACTIVE
    assert!(config.is_active());
    assert!(!config.is_locked());
}

#[test]
fn test_calculate_complexity() {
    let config = DomainConfig::factory_logic(6, 1);
    let complexity = config.calculate_complexity();
    assert!(complexity > 0.0);
}

#[test]
fn test_can_enter() {
    let config = DomainConfig::factory_logic(6, 1);
    // threshold_mass = 0, membrane = ADAPTIVE → всё входит
    assert!(config.can_enter(0, 0));

    let sutra = DomainConfig::factory_sutra(1);
    // CLOSED → никто не входит
    assert!(!sutra.can_enter(100, 100));
}
