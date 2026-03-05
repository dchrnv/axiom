//! Axiom Configuration System
//! Unified configuration loading and validation

pub mod loader;

#[cfg(test)]
mod tests;

pub use loader::{
    ConfigLoader, AxiomConfig, RuntimeConfig, SchemaConfig, LoaderConfig, ConfigError
};

/// Configuration system initialization
pub fn initialize() -> Result<AxiomConfig, ConfigError> {
    let mut loader = ConfigLoader::new();
    
    // Load root configuration
    let root_config = loader.load_root(std::path::Path::new("config/axiom.yaml"))?;
    
    // Validate runtime configuration
    let _runtime_config = loader.load_runtime(std::path::Path::new(&root_config.runtime.file))?;
    let _runtime_schema = loader.load_schema("runtime", std::path::Path::new(&root_config.runtime.schema))?;
    loader.validate(&_runtime_config, &_runtime_schema)?;
    
    // Validate schema configurations
    let _domain_config = loader.load_schema("domain", std::path::Path::new(&root_config.schema.domain))?;
    let _token_config = loader.load_schema("token", std::path::Path::new(&root_config.schema.token))?;
    let _connection_config = loader.load_schema("connection", std::path::Path::new(&root_config.schema.connection))?;
    
    // In real implementation, these would be loaded into their respective structs
    println!("Configuration loaded successfully:");
    println!("  Runtime: {}", root_config.runtime.file);
    println!("  Schema: domain={}, token={}, connection={}", 
        root_config.schema.domain, 
        root_config.schema.token, 
        root_config.schema.connection
    );
    
    Ok(root_config)
}

/// Get configuration value by path
pub fn get_config_value(config: &AxiomConfig, path: &str) -> Option<serde_yaml::Value> {
    // Simple path resolution - extend with nested access
    let parts: Vec<&str> = path.split('.').collect();
    
    match parts.as_slice() {
        ["runtime", "file"] => Some(serde_yaml::to_value(&config.runtime.file).unwrap()),
        ["runtime", "schema"] => Some(serde_yaml::to_value(&config.runtime.schema).unwrap()),
        ["schema", "domain"] => Some(serde_yaml::to_value(&config.schema.domain).unwrap()),
        ["schema", "token"] => Some(serde_yaml::to_value(&config.schema.token).unwrap()),
        ["schema", "connection"] => Some(serde_yaml::to_value(&config.schema.connection).unwrap()),
        ["schema", "grid"] => Some(serde_yaml::to_value(&config.schema.grid).unwrap()),
        ["schema", "upo"] => Some(serde_yaml::to_value(&config.schema.upo).unwrap()),
        _ => None,
    }
}
