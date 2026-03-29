use axiom_config::{ConfigError, ConfigLoader};

fn axiom_yaml_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/axiom.yaml")
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
fn test_load_all_missing_root() {
    let mut loader = ConfigLoader::new();
    let result = loader.load_all(std::path::Path::new("/nonexistent/axiom.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_axiom_yaml_backward_compat() {
    // axiom.yaml без секции presets должен загружаться нормально
    let tmp = std::env::temp_dir().join("axiom_test_compat.yaml");
    std::fs::write(&tmp, b"runtime:\n  file: x\n  schema: y\nschema:\n  domain: a\n  token: b\n  connection: c\n  grid: d\n  upo: e\nloader:\n  format: yaml\n  validation: strict\n  cache_enabled: false\n  hot_reload: false\n").unwrap();
    let mut loader = ConfigLoader::new();
    let loaded = loader.load_all(&tmp).unwrap();
    assert!(loaded.root.presets.domains_dir.is_none());
    assert!(loaded.root.presets.spatial.is_none());
    std::fs::remove_file(tmp).ok();
}
