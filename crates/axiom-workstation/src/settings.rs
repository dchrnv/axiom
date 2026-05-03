use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub engine_address: String,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            engine_address: "127.0.0.1:9876".to_string(),
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("axiom-workstation")
        .join("config.toml")
}

pub fn is_first_run() -> bool {
    !config_path().exists()
}

pub fn load_settings() -> UiSettings {
    std::fs::read_to_string(config_path())
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

#[allow(dead_code)]
pub fn save_settings(settings: &UiSettings) {
    let Ok(toml_str) = toml::to_string(settings) else { return };
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, toml_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 3.7.a — config persistence (сериализация/десериализация без I/O)
    #[test]
    fn test_settings_roundtrip() {
        let original = UiSettings {
            engine_address: "192.168.1.100:12345".to_string(),
        };
        let toml_str = toml::to_string(&original).expect("serialize");
        let loaded: UiSettings = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(loaded.engine_address, original.engine_address);
    }

    #[test]
    fn test_settings_default() {
        let s = UiSettings::default();
        assert_eq!(s.engine_address, "127.0.0.1:9876");
    }
}
