use axiom_config::HeartbeatConfig;

#[test]
fn test_heartbeat_config_presets() {
    let weak = HeartbeatConfig::weak();
    assert_eq!(weak.interval, 10000);
    assert_eq!(weak.batch_size, 1);
    assert!(!weak.enable_gravity);
    assert!(!weak.enable_shell_reconciliation);

    let medium = HeartbeatConfig::medium();
    assert_eq!(medium.interval, 1024);
    assert_eq!(medium.batch_size, 10);
    assert!(medium.enable_gravity);
    assert!(medium.enable_shell_reconciliation);

    let powerful = HeartbeatConfig::powerful();
    assert_eq!(powerful.interval, 256);
    assert_eq!(powerful.batch_size, 50);
    assert!(powerful.enable_thermodynamics);
    assert!(powerful.enable_shell_reconciliation);

    let disabled = HeartbeatConfig::disabled();
    assert_eq!(disabled.interval, u32::MAX);
    assert_eq!(disabled.batch_size, 0);
    assert!(!disabled.enable_shell_reconciliation);
}

#[test]
fn test_heartbeat_config_default() {
    let config = HeartbeatConfig::default();
    assert_eq!(config.interval, 1024);
    assert_eq!(config.batch_size, 10);
}

#[test]
fn test_heartbeat_config_validation() {
    let config = HeartbeatConfig::medium();
    assert!(config.validate().is_ok());

    let mut invalid = HeartbeatConfig::medium();
    invalid.interval = 0;
    assert!(invalid.validate().is_err());
}
