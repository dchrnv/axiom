// Этап 9B — Token/Connection preset loading
use axiom_config::ConfigLoader;
use std::path::Path;

fn tokens_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/presets/tokens")
}

fn connections_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/presets/connection")
}

// ─── TokenPreset ─────────────────────────────────────────────────────────────

#[test]
fn test_load_token_presets_returns_vec() {
    let mut loader = ConfigLoader::new();
    let presets = loader.load_token_presets(&tokens_dir()).unwrap();
    assert_eq!(presets.len(), 3);
}

#[test]
fn test_token_preset_names_from_filenames() {
    let mut loader = ConfigLoader::new();
    let mut presets = loader.load_token_presets(&tokens_dir()).unwrap();
    presets.sort_by(|a, b| a.name.cmp(&b.name));
    let names: Vec<&str> = presets.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["anchor", "impulse", "input"]);
}

#[test]
fn test_token_preset_fields_loaded() {
    let mut loader = ConfigLoader::new();
    let mut presets = loader.load_token_presets(&tokens_dir()).unwrap();
    presets.sort_by(|a, b| a.name.cmp(&b.name));

    // anchor
    let anchor = &presets[0];
    assert_eq!(anchor.domain_id, 108);
    assert_eq!(anchor.mass, 255);
    assert_eq!(anchor.temperature, 32);
    assert_eq!(anchor.resonance, 220);
    assert_eq!(anchor.valence, -1);

    // impulse
    let impulse = &presets[1];
    assert_eq!(impulse.domain_id, 106);
    assert_eq!(impulse.temperature, 200);
    assert_eq!(impulse.velocity[0], 10);

    // input
    let input = &presets[2];
    assert_eq!(input.domain_id, 100);
    assert_eq!(input.state, 1);
    assert_eq!(input.resonance, 440);
}

#[test]
fn test_token_preset_description_loaded() {
    let mut loader = ConfigLoader::new();
    let presets = loader.load_token_presets(&tokens_dir()).unwrap();
    for preset in &presets {
        assert!(!preset.description.is_empty(), "preset '{}' has empty description", preset.name);
    }
}

#[test]
fn test_token_presets_missing_dir_returns_empty() {
    let mut loader = ConfigLoader::new();
    let result = loader.load_token_presets(Path::new("/nonexistent/tokens")).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_token_preset_roundtrip_yaml() {
    let yaml = r#"
domain_id: 104
type_flags: 5
position: [1, 2, 3]
velocity: [-1, 0, 1]
valence: 2
mass: 100
temperature: 128
state: 3
resonance: 660
description: "Test preset"
"#;
    let tmp = std::env::temp_dir().join("axiom_preset_test");
    std::fs::create_dir_all(&tmp).unwrap();
    std::fs::write(tmp.join("custom.yaml"), yaml).unwrap();

    let mut loader = ConfigLoader::new();
    let presets = loader.load_token_presets(&tmp).unwrap();
    assert_eq!(presets.len(), 1);
    let p = &presets[0];
    assert_eq!(p.name, "custom");
    assert_eq!(p.domain_id, 104);
    assert_eq!(p.type_flags, 5);
    assert_eq!(p.position, [1, 2, 3]);
    assert_eq!(p.velocity, [-1, 0, 1]);
    assert_eq!(p.valence, 2);
    assert_eq!(p.mass, 100);
    assert_eq!(p.temperature, 128);
    assert_eq!(p.state, 3);
    assert_eq!(p.resonance, 660);

    std::fs::remove_file(tmp.join("custom.yaml")).ok();
}

// ─── ConnectionPreset ────────────────────────────────────────────────────────

#[test]
fn test_load_connection_presets_returns_vec() {
    let mut loader = ConfigLoader::new();
    let presets = loader.load_connection_presets(&connections_dir()).unwrap();
    assert_eq!(presets.len(), 2);
}

#[test]
fn test_connection_preset_names_from_filenames() {
    let mut loader = ConfigLoader::new();
    let mut presets = loader.load_connection_presets(&connections_dir()).unwrap();
    presets.sort_by(|a, b| a.name.cmp(&b.name));
    let names: Vec<&str> = presets.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["strong", "temporal"]);
}

#[test]
fn test_connection_preset_fields_loaded() {
    let mut loader = ConfigLoader::new();
    let mut presets = loader.load_connection_presets(&connections_dir()).unwrap();
    presets.sort_by(|a, b| a.name.cmp(&b.name));

    let strong = &presets[0];
    assert_eq!(strong.connection_type, "strong");
    assert!((strong.strength - 1.0).abs() < 1e-6);
    assert!((strong.decay_rate - 0.001).abs() < 1e-6);
    assert_eq!(strong.gate_complexity, 1);
    assert_eq!(strong.flags, 1);

    let temporal = &presets[1];
    assert_eq!(temporal.connection_type, "temporal");
    assert!((temporal.strength - 0.5).abs() < 1e-6);
}

#[test]
fn test_connection_preset_description_loaded() {
    let mut loader = ConfigLoader::new();
    let presets = loader.load_connection_presets(&connections_dir()).unwrap();
    for preset in &presets {
        assert!(!preset.description.is_empty(), "preset '{}' has empty description", preset.name);
    }
}

#[test]
fn test_connection_presets_missing_dir_returns_empty() {
    let mut loader = ConfigLoader::new();
    let result = loader.load_connection_presets(Path::new("/nonexistent/connections")).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_connection_preset_roundtrip_yaml() {
    let yaml = r#"
type: weak
strength: 0.2
decay_rate: 0.05
gate_complexity: 3
flags: 2
description: "Weak ephemeral connection"
"#;
    let tmp = std::env::temp_dir().join("axiom_conn_preset_test");
    std::fs::create_dir_all(&tmp).unwrap();
    std::fs::write(tmp.join("weak.yaml"), yaml).unwrap();

    let mut loader = ConfigLoader::new();
    let presets = loader.load_connection_presets(&tmp).unwrap();
    assert_eq!(presets.len(), 1);
    let p = &presets[0];
    assert_eq!(p.name, "weak");
    assert_eq!(p.connection_type, "weak");
    assert!((p.strength - 0.2).abs() < 1e-6);
    assert_eq!(p.gate_complexity, 3);
    assert_eq!(p.flags, 2);

    std::fs::remove_file(tmp.join("weak.yaml")).ok();
}

// ─── axiom.yaml integration ───────────────────────────────────────────────────

#[test]
fn test_axiom_yaml_has_tokens_and_connections_dirs() {
    let mut loader = ConfigLoader::new();
    let axiom_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/axiom.yaml");
    let loaded = loader.load_all(&axiom_path).unwrap();
    assert!(loaded.root.presets.tokens_dir.is_some());
    assert!(loaded.root.presets.connections_dir.is_some());
}
