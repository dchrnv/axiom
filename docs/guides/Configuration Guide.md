# Axiom Configuration Guide

**Версия:** 1.0  
**Последнее обновление:** 2026-03-08  
**Статус:** Configuration System V1.0 реализована и интегрирована

---

## 📋 Содержание

1. [Обзор Configuration System](#обзор-configuration-system)
2. [Структура конфигурационных файлов](#структура-конфигурационных-файлов)
3. [Использование ConfigLoader](#исользование-configloader)
4. [Интеграция с модулями](#интеграция-с-модулями)
5. [Пресеты и валидация](#пресеты-и-валидация)
6. [Примеры использования](#примеры-использования)
7. [Тестирование конфигураций](#тестирование-конфигураций)
8. [Troubleshooting](#troubleshooting)

---

## 🏗️ Обзор Configuration System

### Что такое Configuration System?

Configuration System - это унифицированная система загрузки, валидации и управления конфигурациями Axiom. Она обеспечивает:

- **Централизованное управление:** Все конфигурации в одном месте
- **Валидацию:** Проверка конфигураций по схемам
- **Пресеты:** Готовые конфигурации для разных сценариев
- **Интеграцию:** Бесшовная работа с модулями Token, Connection, Domain

### Архитектура

```
config/
├── axiom.yaml          # Корневая конфигурация
├── runtime/
│   ├── runtime.yaml   # Параметры runtime
│   └── runtime_schema.yaml
└── schema/
    ├── domain.yaml    # Схема доменов
    ├── token.yaml     # Схема токенов
    ├── connection.yaml# Схема соединений
    ├── grid.yaml      # Схема сетки
    └── upo.yaml       # Схема UPO
```

---

## 📁 Структура конфигурационных файлов

### Корневая конфигурация (`config/axiom.yaml`)

```yaml
# Axiom Root Configuration
# Version: 1.0

# Runtime configuration - системные параметры
runtime:
  file: "config/runtime/runtime.yaml"
  schema: "config/runtime/runtime_schema.yaml"

# Schema configuration - семантическая структура
schema:
  domain: "config/schema/domain.yaml"
  token: "config/schema/token.yaml"
  connection: "config/schema/connection.yaml"
  grid: "config/schema/grid.yaml"
  upo: "config/schema/upo.yaml"

# Loader configuration - параметры загрузчика
loader:
  format: "yaml"
  validation: "strict"
  cache_enabled: true
  hot_reload: false
```

### Конфигурация Runtime (`config/runtime/runtime.yaml`)

```yaml
# Runtime Configuration
# System operational parameters

# System parameters
system:
  threads: 4
  max_tokens: 100000
  memory_limit: "2GB"
  
# Performance settings
performance:
  cache_size: 1000
  batch_size: 100
  gc_threshold: 0.8
  
# Logging configuration
logging:
  level: "info"
  file: "logs/axiom.log"
  max_size: "100MB"
```

### Схемы модулей

#### Token Schema (`config/schema/token.yaml`)

```yaml
# Token Schema Configuration
# Semantic structure of Axiom tokens

# Token types definition
token_types:
  - name: "concept"
    description: "Abstract concept token"
    default_momentum: 1.0
    default_resonance: 440.0
    max_velocity: 100.0
    
  - name: "relation"
    description: "Relationship token"
    default_momentum: 0.8
    default_resonance: 220.0
    max_velocity: 80.0
    
  - name: "context"
    description: "Contextual information token"
    default_momentum: 0.5
    default_resonance: 880.0
    max_velocity: 60.0

# Token configuration template
token_template:
  type: object
  required: ["token_id", "token_type", "momentum", "resonance"]
  properties:
    token_id:
      type: integer
      minimum: 1
      maximum: 4294967295
    token_type:
      type: string
      enum: ["concept", "relation", "context", "custom"]
    momentum:
      type: number
      minimum: 0.0
      maximum: 100.0
      default: 1.0
    resonance:
      type: number
      minimum: 20.0
      maximum: 20000.0
      default: 440.0

# Physics constraints
physics_constraints:
  momentum_conservation:
    description: "Total momentum must be conserved"
    rule: "sum(momentum) <= system_max_momentum"
    
  resonance_harmony:
    description: "Resonance frequencies should be harmonically related"
    rule: "resonance % base_frequency == 0 OR resonance / base_frequency is integer"
```

#### Connection Schema (`config/schema/connection.yaml`)

```yaml
# Connection Schema Configuration
# Semantic structure of Axiom connections

# Connection types definition
connection_types:
  - name: "strong"
    description: "Strong structural connection"
    default_strength: 1.0
    max_connections: 1000
    decay_rate: 0.001
    
  - name: "weak"
    description: "Weak associative connection"
    default_strength: 0.3
    max_connections: 5000
    decay_rate: 0.01
    
  - name: "temporal"
    description: "Time-based connection"
    default_strength: 0.5
    max_connections: 2000
    decay_rate: 0.005

# Topological constraints
topology_constraints:
  max_degree:
    description: "Maximum number of connections per token"
    rule: "count(connections) <= max_connections"
    
  no_self_loops:
    description: "Tokens cannot connect to themselves"
    rule: "source_token != target_token"
    
  symmetry:
    description: "Connection symmetry rules"
    rule: "connection_type == 'strong' implies bidirectional == true"
```

#### Domain Schema (`config/schema/domain.yaml`)

```yaml
# Domain Schema Configuration
# Semantic structure of Axiom domains

# Domain types definition
domain_types:
  - name: "logic"
    description: "Logical reasoning domain"
    structural_role: "execution"
    default_field_size: [50.0, 50.0, 50.0]
    default_gravity: 0.5
    default_temperature: 273.15
    
  - name: "dream"
    description: "Dream processing domain"
    structural_role: "shadow"
    default_field_size: [200.0, 200.0, 200.0]
    default_gravity: 0.1
    default_temperature: 310.15
    
  - name: "math"
    description: "Mathematical computation domain"
    structural_role: "logic"
    default_field_size: [100.0, 100.0, 100.0]
    default_gravity: 1.0
    default_temperature: 293.15

# Domain configuration template
domain_template:
  type: object
  required: ["domain_id", "domain_type", "structural_role"]
  properties:
    domain_id:
      type: integer
      minimum: 1
      maximum: 255
    domain_type:
      type: string
      enum: ["logic", "dream", "math", "custom"]
    structural_role:
      type: string
      enum: ["sutra", "execution", "shadow", "codex", "map", "probe", "logic", "dream", "void", "maya"]
    field_size:
      type: array
      items:
        type: number
        minimum: 1.0
        maximum: 1000.0
      minItems: 3
      maxItems: 3
```

---

## ⚙️ Использование ConfigLoader

### Базовая инициализация

```rust
use crate::config::{initialize, ConfigLoader};

// Инициализация конфигурационной системы
let config = initialize()?;
println!("Configuration loaded successfully!");

// Создание загрузчика
let mut loader = ConfigLoader::new();
```

### Загрузка конфигураций

```rust
use std::path::Path;

// Загрузка корневой конфигурации
let root_config = loader.load_root(Path::new("../config/axiom.yaml"))?;

// Загрузка runtime конфигурации
let runtime_config = loader.load_runtime(Path::new("../config/runtime/runtime.yaml"))?;

// Загрузка схем
let domain_schema = loader.load_schema("domain", Path::new("../config/schema/domain.yaml"))?;
let token_schema = loader.load_schema("token", Path::new("../config/schema/token.yaml"))?;
```

### Валидация конфигураций

```rust
// Валидация конфигурации по схеме
loader.validate(&runtime_config, &runtime_schema)?;

// Получение значений конфигурации
use crate::config::get_config_value;

let threads = get_config_value(&config, "runtime.system.threads");
let max_tokens = get_config_value(&config, "runtime.system.max_tokens");
```

---

## 🔗 Интеграция с модулями

### Token.rs интеграция

```rust
use crate::token::Token;

// Создание токена из пресета
let token = Token::from_preset("concept", 1, 1)?;
assert!(token.validate_with_config().is_ok());

// Доступ к свойствам из пресета
assert_eq!(token.resonance, 440); // default_resonance для concept
assert_eq!(token.momentum[0], 1);  // default_momentum для concept
```

### Connection.rs интеграция

```rust
use crate::connection::Connection;

// Создание соединения из пресета
let connection = Connection::from_preset("strong", 1, 2, 1)?;
assert!(connection.validate_with_config().is_ok());

// Доступ к свойствам из пресета
assert_eq!(connection.strength, 1.0);     // default_strength для strong
assert_eq!(connection.elasticity, 0.999); // 1.0 - decay_rate
```

### Domain.rs интеграция

```rust
use crate::domain::DomainConfig;

// Создание домена из пресета
let domain = DomainConfig::from_preset("logic")?;
assert!(domain.validate());

// Доступ к свойствам из пресета
assert_eq!(domain.domain_type, DomainType::Logic);
assert_eq!(domain.structural_role, StructuralRole::Ashti1);
```

---

## 🎯 Пресеты и валидация

### Доступные пресеты

#### Token пресеты
- **concept**: Абстрактные концепции (resonance: 440Hz, momentum: 1.0)
- **relation**: Отношения (resonance: 220Hz, momentum: 0.8)
- **context**: Контекст (resonance: 880Hz, momentum: 0.5)

#### Connection пресеты
- **strong**: Сильные структурные связи (strength: 1.0, decay: 0.001)
- **weak**: Слабые ассоциативные связи (strength: 0.3, decay: 0.01)
- **temporal**: Временные связи (strength: 0.5, decay: 0.005)

#### Domain пресеты
- **logic**: Логические рассуждения (field: [50,50,50], gravity: 0.5)
- **dream**: Обработка снов (field: [200,200,200], gravity: 0.1)
- **math**: Математические вычисления (field: [100,100,100], gravity: 1.0)

### Валидация

```rust
// Базовая валидация
assert!(token.validate());                    // Встроенная проверка
assert!(connection.validate());               // Встроенная проверка
assert!(domain.validate());                  // Встроенная проверка

// Валидация с конфигурацией
assert!(token.validate_with_config().is_ok());        // С schema
assert!(connection.validate_with_config().is_ok());   // С constraints
```

---

## 💡 Примеры использования

### Пример 1: Создание системы из конфигурации

```rust
use crate::config::initialize;
use crate::{DomainConfig, Token, Connection};

// Загрузка конфигурации
let config = initialize()?;

// Создание домена
let domain = DomainConfig::from_preset("logic")?;

// Создание токенов в домене
let token1 = Token::from_preset("concept", 1, domain.domain_id)?;
let token2 = Token::from_preset("relation", 2, domain.domain_id)?;

// Создание связи между токенами
let connection = Connection::from_preset("strong", 
    token1.sutra_id, token2.sutra_id, domain.domain_id)?;

// Валидация всей системы
assert!(domain.validate());
assert!(token1.validate_with_config().is_ok());
assert!(token2.validate_with_config().is_ok());
assert!(connection.validate_with_config().is_ok());
```

### Пример 2: Настройка параметров системы

```rust
use crate::config::{initialize, get_config_value};

// Загрузка конфигурации
let config = initialize()?;

// Получение параметров runtime
let threads = get_config_value(&config, "runtime.system.threads");
let max_tokens = get_config_value(&config, "runtime.system.max_tokens");

// Использование параметров
if let Some(threads) = threads.and_then(|v| v.as_i64()) {
    println!("System threads: {}", threads);
}
```

### Пример 3: Обработка ошибок конфигурации

```rust
use crate::config::{ConfigError, initialize};

match initialize() {
    Ok(config) => {
        println!("Configuration loaded successfully");
        // Работа с конфигурацией
    }
    Err(ConfigError::MissingFile(file)) => {
        eprintln!("Missing configuration file: {}", file);
    }
    Err(ConfigError::ParseError(err)) => {
        eprintln!("Configuration parse error: {}", err);
    }
    Err(ConfigError::ValidationError(msg)) => {
        eprintln!("Configuration validation error: {}", msg);
    }
}
```

---

## 🧪 Тестирование конфигураций

### Запуск тестов

```bash
# Все тесты конфигурации
cargo test config

# Тесты конкретного модуля
cargo test token::tests
cargo test connection::tests
cargo test domain::tests

# Интеграционные тесты
cargo test config_integration
```

### Структура тестов

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_preset() {
        let token = Token::from_preset("concept", 1, 1).unwrap();
        assert_eq!(token.resonance, 440);
        assert!(token.validate_with_config().is_ok());
    }

    #[test]
    fn test_connection_validation() {
        let connection = Connection::from_preset("strong", 1, 2, 1).unwrap();
        assert!(connection.validate_with_config().is_ok());
        
        // Проверка no_self_loops constraint
        let self_loop = Connection::from_preset("strong", 1, 1, 1).unwrap();
        assert!(self_loop.validate_with_config().is_err());
    }
}
```

---

## 🔧 Troubleshooting

### Частые проблемы

#### 1. "Configuration file not found"

**Проблема:** Неверный путь к конфигурационным файлам
**Решение:** Убедитесь, что пути корректны относительно рабочей директории

```rust
// Для запуска из runtime/ директории
let config = initialize()?; // Автоматически определит путь

// Явное указание пути
let mut loader = ConfigLoader::new();
let config = loader.load_root(Path::new("../config/axiom.yaml"))?;
```

#### 2. "Unknown preset" ошибка

**Проблема:** Неверное имя пресета
**Решение:** Проверьте доступные пресеты в schema файлах

```yaml
# config/schema/token.yaml
token_types:
  - name: "concept"    # ✅ Правильно
  - name: "relation"   # ✅ Правильно
  - name: "context"    # ✅ Правильно
```

#### 3. "Validation failed" ошибка

**Проблема:** Конфигурация не соответствует схеме
**Решение:** Проверьте required поля и ограничения

```rust
// Проверка валидации
match token.validate_with_config() {
    Ok(()) => println!("Token is valid"),
    Err(ConfigError::ValidationError(msg)) => {
        println!("Validation error: {}", msg);
    }
}
```

#### 4. Проблемы с параллельным выполнением тестов

**Проблема:** Тесты меняют рабочую директорию
**Решение:** Запускайте тесты последовательно или изолированно

```bash
# Последовательный запуск
cargo test -- --test-threads=1

# Изолированный запуск конкретного теста
cargo test test_token_from_preset
```

### Отладка конфигураций

```rust
// Включение отладочной информации
use std::env;

if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "debug");
}

// Проверка загрузки конфигурации
let config = initialize()?;
println!("Loaded config: {:#?}", config);

// Проверка путей
println!("Domain schema: {}", config.schema.domain);
println!("Token schema: {}", config.schema.token);
```

---

## 📚 Дополнительные ресурсы

- [DEVELOPMENT_GUIDE.md](../DEVELOPMENT_GUIDE.md) - Руководство по разработке
- [ROADMAP.md](../ROADMAP.md) - План развития проекта
- [Core Invariants.md](../Core%20Invariants.md) - Основные инварианты системы
- [Спецификации](../specs/) - Технические спецификации модулей

---

**Последнее обновление:** 2026-03-08  
**Версия Configuration System:** V1.0  
**Статус:** ✅ Реализована и интегрирована с основными модулями
