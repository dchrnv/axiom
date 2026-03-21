use axiom_config::{
    DomainConfig, DomainType, StructuralRole, PROCESSING_IDLE, PROCESSING_ACTIVE,
    PROCESSING_FROZEN,
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
