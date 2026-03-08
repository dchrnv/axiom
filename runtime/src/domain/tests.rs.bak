//! Domain Configuration System Tests

#[cfg(test)]
mod tests {
    use crate::{DomainConfig, DomainType, StructuralRole};
    use crate::config;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_domain_config_size() {
        use std::mem;
        let size = mem::size_of::<DomainConfig>();
        println!("DomainConfig size: {} bytes", size);
        assert_eq!(size, 128, "DomainConfig should be exactly 128 bytes according to V2.0 specification");
    }

    #[test]
    fn test_domain_from_preset() {
        // Установить рабочую директорию в корень проекта
        env::set_current_dir("/home/chrnv/Axiom").ok();
        
        // Тест загрузки домена из пресета
        let result = DomainConfig::from_preset("logic");
        assert!(result.is_ok());
        
        let domain = result.unwrap();
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.domain_type, DomainType::Logic as u8);
        assert!(domain.validate());
    }

    #[test]
    fn test_domain_from_unknown_preset() {
        env::set_current_dir("/home/chrnv/Axiom").ok();
        
        // Тест неизвестного пресета
        let result = DomainConfig::from_preset("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_domain_validation() {
        // Тест валидации домена
        let domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti6);
        assert!(domain.validate());
        
        // Тест невалидного домена - создаем вручную
        let mut invalid_domain = DomainConfig::default();
        invalid_domain.domain_id = 0;  // Невалидный ID
        assert!(!invalid_domain.validate());
        
        // Тест невалидных размеров поля
        let mut invalid_domain2 = DomainConfig::default();
        invalid_domain2.domain_id = 1;
        invalid_domain2.field_size = [0.0, 100.0, 100.0];
        assert!(!invalid_domain2.validate());
    }

    #[test]
    fn test_domain_config_integration() {
        env::set_current_dir("/home/chrnv/Axiom").ok();
        
        // Тест интеграции с Configuration System
        let config = config::initialize();
        assert!(config.is_ok());
        
        let system_config = config.unwrap();
        assert_eq!(system_config.schema.domain, "config/schema/domain.yaml");
    }

    #[test]
    fn test_domain_preset_loading() {
        env::set_current_dir("/home/chrnv/Axiom").ok();
        
        // Тест загрузки пресетов из схемы
        let result = DomainConfig::from_preset("logic");
        assert!(result.is_ok());
        
        let domain = result.unwrap();
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.domain_type, DomainType::Logic as u8);
        assert_eq!(domain.structural_role, StructuralRole::Ashti6 as u8);
    }
}
