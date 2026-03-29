// Этап 9C — Gateway::set_config_watcher / check_config_reload
use axiom_runtime::Gateway;
use axiom_config::ConfigWatcher;
use std::time::Duration;

const AXIOM_YAML_V1: &str = r#"
runtime:
  file: "x"
  schema: "y"
schema:
  domain: "a"
  token: "b"
  connection: "c"
  grid: "d"
  upo: "e"
loader:
  format: yaml
  validation: strict
  cache_enabled: false
  hot_reload: false
"#;

const AXIOM_YAML_V2: &str = r#"
runtime:
  file: "x2"
  schema: "y2"
schema:
  domain: "a"
  token: "b"
  connection: "c"
  grid: "d"
  upo: "e"
loader:
  format: yaml
  validation: relaxed
  cache_enabled: true
  hot_reload: true
"#;

fn temp_dir(suffix: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("axiom_gw_watcher_{suffix}"))
}

#[test]
fn test_gateway_no_watcher_returns_none() {
    let gw = Gateway::with_default_engine();
    assert!(gw.check_config_reload().is_none());
}

#[test]
fn test_gateway_set_watcher_no_change_returns_none() {
    let dir = temp_dir("nochange");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_V1).unwrap();

    let mut gw = Gateway::with_default_engine();
    gw.set_config_watcher(ConfigWatcher::new(&path).unwrap());

    assert!(gw.check_config_reload().is_none());
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_gateway_detects_config_change() {
    let dir = temp_dir("detect");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_V1).unwrap();

    let mut gw = Gateway::with_default_engine();
    gw.set_config_watcher(ConfigWatcher::new(&path).unwrap());

    std::thread::sleep(Duration::from_millis(50));
    let _ = gw.check_config_reload(); // drain startup events

    std::fs::write(&path, AXIOM_YAML_V2).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let result = gw.check_config_reload();
    assert!(result.is_some(), "check_config_reload должен вернуть новую конфигурацию");
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_gateway_reload_config_values() {
    let dir = temp_dir("values");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_V1).unwrap();

    let mut gw = Gateway::with_default_engine();
    gw.set_config_watcher(ConfigWatcher::new(&path).unwrap());

    std::thread::sleep(Duration::from_millis(50));
    let _ = gw.check_config_reload();

    std::fs::write(&path, AXIOM_YAML_V2).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let cfg = gw.check_config_reload().expect("expected config");
    assert!(cfg.root.loader.hot_reload);
    assert_eq!(cfg.root.loader.validation, "relaxed");
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_gateway_genome_not_in_reloaded_config() {
    // GENOME-инвариант: LoadedAxiomConfig не содержит genome
    let dir = temp_dir("genome");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_V1).unwrap();

    let mut gw = Gateway::with_default_engine();
    gw.set_config_watcher(ConfigWatcher::new(&path).unwrap());

    std::thread::sleep(Duration::from_millis(50));
    let _ = gw.check_config_reload();

    std::fs::write(&path, AXIOM_YAML_V2).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let cfg = gw.check_config_reload().expect("expected config");
    // Статически: LoadedAxiomConfig не имеет поля genome — GENOME вне reload
    assert!(cfg.domains.is_empty()); // нет presets.domains_dir в temp yaml
    assert!(cfg.heartbeat.is_none());
    std::fs::remove_dir_all(dir).ok();
}
