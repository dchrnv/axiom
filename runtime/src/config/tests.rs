//! Configuration System Tests

#[cfg(test)]
mod tests {
    use crate::config::{ConfigLoader, AxiomConfig, RuntimeConfig, SchemaConfig, LoaderConfig, initialize, get_config_value};
    use std::path::Path;

    #[test]
    fn test_load_root_config() {
        let mut loader = ConfigLoader::new();
        let config = loader.load_root(Path::new("config/axiom.yaml"));
        
        assert!(config.is_ok());
        let config = config.unwrap();
        
        assert_eq!(config.runtime.file, "config/runtime/runtime.yaml");
        assert_eq!(config.schema.domain, "config/schema/domain.yaml");
        assert_eq!(config.schema.token, "config/schema/token.yaml");
        assert_eq!(config.loader.format, "yaml");
    }

    #[test]
    fn test_load_runtime_config() {
        let mut loader = ConfigLoader::new();
        let config = loader.load_runtime(Path::new("config/runtime/runtime.yaml"));
        
        assert!(config.is_ok());
        let config = config.unwrap();
        
        // Check system configuration
        if let Some(system) = config.get("system") {
            if let Some(threads) = system.get("threads") {
                assert_eq!(threads.as_i64(), Some(4));
            }
        }
    }

    #[test]
    fn test_load_domain_schema() {
        let mut loader = ConfigLoader::new();
        let schema = loader.load_schema("domain", Path::new("config/schema/domain.yaml"));
        
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        
        // Check domain types
        if let Some(domain_types) = schema.get("domain_types") {
            assert!(domain_types.is_sequence());
        }
    }

    #[test]
    fn test_validation() {
        let mut loader = ConfigLoader::new();
        
        // Valid configuration
        let valid_config = serde_yaml::from_str(r#"
        threads: 4
        max_tokens: 100000
        "#).unwrap();
        
        let valid_schema = serde_yaml::from_str(r#"
        type: object
        required: true
        properties:
          threads:
            type: integer
            minimum: 1
            maximum: 64
        "#).unwrap();
        
        assert!(loader.validate(&valid_config, &valid_schema).is_ok());
        
        // Invalid configuration
        let invalid_config = serde_yaml::from_str(r#"
        threads: 0  # Below minimum
        "#).unwrap();
        
        assert!(loader.validate(&invalid_config, &valid_schema).is_err());
    }

    #[test]
    fn test_config_initialization() {
        let result = initialize();
        
        assert!(result.is_ok());
        let config = result.unwrap();
        
        assert_eq!(config.runtime.file, "config/runtime/runtime.yaml");
        assert_eq!(config.schema.domain, "config/schema/domain.yaml");
    }

    #[test]
    fn test_get_config_value() {
        let config = AxiomConfig {
            runtime: RuntimeConfig {
                file: "test.yaml".to_string(),
                schema: "test_schema.yaml".to_string(),
            },
            schema: SchemaConfig {
                domain: "domain.yaml".to_string(),
                token: "token.yaml".to_string(),
                connection: "connection.yaml".to_string(),
                grid: "grid.yaml".to_string(),
                upo: "upo.yaml".to_string(),
            },
            loader: LoaderConfig {
                format: "yaml".to_string(),
                validation: "strict".to_string(),
                cache_enabled: true,
                hot_reload: false,
            },
        };
        
        // Test path resolution
        assert_eq!(
            get_config_value(&config, "runtime.file"),
            Some(serde_yaml::to_value("test.yaml").unwrap())
        );
        
        assert_eq!(
            get_config_value(&config, "schema.domain"),
            Some(serde_yaml::to_value("domain.yaml").unwrap())
        );
    }
}
