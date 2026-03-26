use axiom_config::{ConfigError, ConfigLoader};

#[test]
fn test_config_loader_creation() {
    let loader = ConfigLoader::new();
    assert_eq!(loader.cache.len(), 0);
}

#[test]
fn test_config_loader_default() {
    let loader = ConfigLoader::default();
    assert_eq!(loader.cache.len(), 0);
}

#[test]
fn test_config_error_display() {
    let error = ConfigError::ValidationError("test error".to_string());
    assert_eq!(format!("{}", error), "Validation error: test error");

    let error = ConfigError::MissingFile("config.yaml".to_string());
    assert_eq!(format!("{}", error), "Missing file: config.yaml");
}
