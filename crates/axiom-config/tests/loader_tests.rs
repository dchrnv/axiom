use axiom_config::schema;
use axiom_config::{ConfigError, ConfigLoader};

fn axiom_yaml_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/axiom.yaml")
}

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

// ─── ConfigLoader::load_all ──────────────────────────────────────────────────

#[test]
fn test_load_all_returns_root_config() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    assert_eq!(loaded.root.loader.format, "yaml");
    assert!(loaded.root.loader.cache_enabled);
}

#[test]
fn test_load_all_presets_section_present() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    assert!(loaded.root.presets.domains_dir.is_some());
    assert!(loaded.root.presets.spatial.is_some());
    assert!(loaded.root.presets.semantic_contributions.is_some());
}

#[test]
fn test_load_all_domains_loaded() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    // Должны быть загружены все 11 доменов
    assert_eq!(loaded.domains.len(), 11);
    assert!(loaded.domains.contains_key("sutra"));
    assert!(loaded.domains.contains_key("logic"));
    assert!(loaded.domains.contains_key("maya"));
    assert!(loaded.domains.contains_key("experience"));
}

#[test]
fn test_load_all_domains_valid() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    for (name, domain) in &loaded.domains {
        assert!(
            domain.validate().is_ok(),
            "domain '{}' failed validation",
            name
        );
    }
}

#[test]
fn test_load_all_caches_files() {
    let mut loader = ConfigLoader::new();
    loader.load_all(&axiom_yaml_path()).unwrap();
    // После load_all кэш должен содержать как минимум axiom.yaml + spatial + semantic
    assert!(loader.cache.len() >= 3);
}

#[test]
fn test_load_all_spatial_path() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    let base = axiom_yaml_path().parent().unwrap().to_path_buf();
    let spatial_path = loader.spatial_config_path(&loaded, &base);
    assert!(spatial_path.is_some());
    assert!(spatial_path.unwrap().exists());
}

#[test]
fn test_load_all_semantic_path() {
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    let base = axiom_yaml_path().parent().unwrap().to_path_buf();
    let sem_path = loader.semantic_contributions_path(&loaded, &base);
    assert!(sem_path.is_some());
    assert!(sem_path.unwrap().exists());
}

#[test]
fn test_load_all_heartbeat_none_by_default() {
    // axiom.yaml без heartbeat_file → loaded.heartbeat == None
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_yaml_path()).unwrap();
    assert!(
        loaded.heartbeat.is_none(),
        "heartbeat должен быть None если heartbeat_file не задан"
    );
}

#[test]
fn test_load_all_heartbeat_loaded_when_file_present() {
    use std::io::Write;
    let dir = std::env::temp_dir().join("axiom_hb_test");
    std::fs::create_dir_all(&dir).unwrap();

    // Создаём heartbeat.yaml
    let hb_path = dir.join("heartbeat.yaml");
    let hb_yaml = b"\
interval: 512\n\
batch_size: 5\n\
connection_batch_size: 3\n\
enable_decay: true\n\
enable_gravity: true\n\
enable_spatial_collision: false\n\
enable_connection_maintenance: true\n\
enable_thermodynamics: false\n\
attach_pulse_id: true\n\
enable_shell_reconciliation: false\n";
    std::fs::File::create(&hb_path)
        .unwrap()
        .write_all(hb_yaml)
        .unwrap();

    // Создаём axiom.yaml с heartbeat_file
    let axiom_path = dir.join("axiom.yaml");
    let axiom_yaml = "runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\
         \n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\
         \n  validation: strict\n  cache_enabled: false\
         \npresets:\n  heartbeat_file: \"heartbeat.yaml\"\n";
    std::fs::write(&axiom_path, axiom_yaml).unwrap();

    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_path).unwrap();
    assert!(loaded.heartbeat.is_some(), "heartbeat должен загрузиться");
    let hb = loaded.heartbeat.unwrap();
    assert_eq!(hb.interval, 512);
    assert_eq!(hb.batch_size, 5);
    assert!(hb.enable_gravity);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_all_heartbeat_missing_file_is_none() {
    // heartbeat_file задан, но файл не существует → None (без ошибки)
    let dir = std::env::temp_dir().join("axiom_hb_missing");
    std::fs::create_dir_all(&dir).unwrap();

    let axiom_path = dir.join("axiom.yaml");
    let axiom_yaml = "runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\
         \n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\
         \n  validation: strict\n  cache_enabled: false\
         \npresets:\n  heartbeat_file: \"nonexistent_heartbeat.yaml\"\n";
    std::fs::write(&axiom_path, axiom_yaml).unwrap();

    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&axiom_path).unwrap();
    assert!(
        loaded.heartbeat.is_none(),
        "несуществующий файл → None без ошибки"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_all_missing_root() {
    let mut loader = ConfigLoader::new();
    let result = loader.load_all(std::path::Path::new("/nonexistent/axiom.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_axiom_yaml_backward_compat() {
    // axiom.yaml без секции presets должен загружаться нормально
    let tmp = std::env::temp_dir().join("axiom_test_compat.yaml");
    std::fs::write(&tmp, b"runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\n  validation: strict\n  cache_enabled: false\n").unwrap();
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&tmp).unwrap();
    assert!(loaded.root.presets.domains_dir.is_none());
    assert!(loaded.root.presets.spatial.is_none());
    std::fs::remove_file(tmp).ok();
}

// ─── Schema / D-07 ──────────────────────────────────────────────────────────────

#[test]
fn test_schema_json_is_valid_json() {
    let s = schema::axiom_schema_json();
    let v: serde_json::Value = serde_json::from_str(&s).expect("axiom schema must be valid JSON");
    assert!(v.is_object());
}

#[test]
fn test_domain_schema_json_is_valid_json() {
    let s = schema::domain_schema_json();
    let v: serde_json::Value = serde_json::from_str(&s).expect("domain schema must be valid JSON");
    assert!(v.is_object());
}

#[test]
fn test_heartbeat_schema_json_is_valid_json() {
    let s = schema::heartbeat_schema_json();
    let v: serde_json::Value =
        serde_json::from_str(&s).expect("heartbeat schema must be valid JSON");
    assert!(v.is_object());
}

#[test]
fn test_validate_yaml_valid_axiom_config() {
    use axiom_config::loader::AxiomConfig;
    let yaml = "runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\
        \n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\
        \n  validation: strict\n  cache_enabled: false\n";
    let result = schema::validate_yaml::<AxiomConfig>(yaml);
    assert!(
        result.is_ok(),
        "valid axiom config should pass schema validation"
    );
}

#[test]
fn test_validate_yaml_invalid_type_returns_error() {
    use axiom_config::loader::AxiomConfig;
    // cache_enabled должен быть bool, передаём строку
    let yaml = "runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\
        \n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\
        \n  validation: strict\n  cache_enabled: not_a_bool\n";
    let result = schema::validate_yaml::<AxiomConfig>(yaml);
    // serde может сам отклонить невалидный bool как ParseError
    assert!(result.is_err(), "invalid type should return an error");
}

#[test]
fn test_schema_contains_expected_fields() {
    let s = schema::axiom_schema_json();
    // Схема должна содержать ключевые поля
    assert!(
        s.contains("\"runtime\"") || s.contains("runtime"),
        "schema should reference runtime field"
    );
    assert!(
        s.contains("\"loader\"") || s.contains("loader"),
        "schema should reference loader field"
    );
}
