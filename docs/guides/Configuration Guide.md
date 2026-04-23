# Axiom Configuration Guide

**Версия:** 2.0  
**Последнее обновление:** 2026-04-12  
**Статус:** Config V1.1 — AdaptiveTickRate задокументирован. GuardianConfig — запланирован (hardcoded в guardian.rs).

---

## Содержание

1. [Обзор Configuration System](#обзор-configuration-system)
2. [Структура конфигурационных файлов](#структура-конфигурационных-файлов)
3. [Использование ConfigLoader](#использование-configloader)
4. [AdaptiveTickRate](#adaptiveticktrate)
5. [GuardianConfig (планируется)](#guardianconfig-планируется)
6. [Интеграция с модулями](#интеграция-с-модулями)
7. [Пресеты и валидация](#пресеты-и-валидация)
8. [Горячая перезагрузка (ConfigWatcher)](#горячая-перезагрузка-configwatcher)
9. [Troubleshooting](#troubleshooting)

---

## Обзор Configuration System

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

## Структура конфигурационных файлов

### Корневая конфигурация (`config/axiom.yaml`)

```yaml
# Axiom Root Configuration
# Version: 2.0

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

# Presets — готовые конфигурации компонентов
presets:
  domains_dir: "presets/domains"          # директория YAML-пресетов доменов
  tokens_dir: "presets/tokens"            # директория YAML-пресетов токенов
  connections_dir: "presets/connection"   # директория YAML-пресетов связей
  spatial: "presets/spatial/medium.yaml"  # конфигурация SpatialHashGrid
  semantic_contributions: "schema/semantic_contributions.yaml"
  # heartbeat_file: "presets/heartbeat.yaml"  # опционально — HeartbeatConfig
```

Все поля в секции `presets` опциональны. Отсутствие `heartbeat_file` → `LoadedAxiomConfig.heartbeat = None`.  
Несуществующий файл в `heartbeat_file` → `None` без ошибки (graceful degradation).

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

## Использование ConfigLoader

### Базовая инициализация

```rust
use axiom_config::{ConfigLoader, ConfigError};
use std::path::Path;

let mut loader = ConfigLoader::new();
```

### Полная загрузка через `load_all`

`load_all` — основной метод. Читает `axiom.yaml`, загружает все указанные компоненты:

```rust
let loaded = loader.load_all(Path::new("config/axiom.yaml"))?;

// loaded.root       — AxiomConfig (корневая конфигурация)
// loaded.domains    — HashMap<String, DomainConfig> (11 доменов)
// loaded.heartbeat  — Option<HeartbeatConfig> (если heartbeat_file задан)
```

Что загружается автоматически:
- все `*.yaml` из `presets.domains_dir` → `loaded.domains`
- `presets.spatial` → кэш (для последующей загрузки через `SpatialConfig`)
- `presets.semantic_contributions` → кэш
- `presets.heartbeat_file` → `loaded.heartbeat` (если файл существует)

### LoadedAxiomConfig

```rust
pub struct LoadedAxiomConfig {
    pub root:      AxiomConfig,                   // корневая конфигурация
    pub domains:   HashMap<String, DomainConfig>, // name → config (e.g. "logic", "maya")
    pub heartbeat: Option<HeartbeatConfig>,       // None если heartbeat_file не задан
}
```

Пример использования `heartbeat`:

```rust
let loaded = loader.load_all(Path::new("config/axiom.yaml"))?;
if let Some(hb) = loaded.heartbeat {
    println!("heartbeat interval: {}", hb.interval);
    println!("gravity enabled: {}", hb.enable_gravity);
}
```

### Загрузка отдельных компонентов

```rust
// DomainConfig из файла
let domain = loader.load_domain_config(Path::new("config/presets/domains/logic.yaml"))?;

// HeartbeatConfig из файла
let hb = loader.load_heartbeat_config(Path::new("config/presets/heartbeat.yaml"))?;

// Пресеты токенов из директории
let tokens = loader.load_token_presets(Path::new("config/presets/tokens"))?;

// Пресеты связей из директории
let conns = loader.load_connection_presets(Path::new("config/presets/connection"))?;
```

### Пути к spatial и semantic_contributions

```rust
let base = Path::new("config");
let spatial_path = loader.spatial_config_path(&loaded, base);
let sem_path     = loader.semantic_contributions_path(&loaded, base);
// → Option<PathBuf> для последующей загрузки через соответствующие крейты
```

### Валидация конфигурации по схеме

```rust
// Ручная валидация YAML-значения против схемы
loader.validate(&config_value, &schema_value)?;
// Проверяет: required поля, типы, minimum/maximum для числовых полей
```

---

---

## AdaptiveTickRate

`AdaptiveTickRate` управляет переменной частотой тиков CLI-канала. Структура определена в коде, параметры выставляются через секцию `tick_schedule.adaptive_tick` в `axiom-cli.yaml`.

**Триггеры повышения частоты:** tension traces, внешний ввод, multipass-обработка.  
**Снижение:** после `cooldown` idle-тиков частота уменьшается на `step_down`.

```yaml
tick_schedule:
  # ... другие интервалы ...

  adaptive_tick:
    min_hz: 60       # минимум в режиме ожидания
    max_hz: 1000     # максимум под нагрузкой
    step_up: 200     # прирост частоты при триггере
    step_down: 20    # снижение после cooldown idle-тиков
    cooldown: 50     # сколько idle-тиков до снижения
```

Адаптивный режим активируется при `adaptive_tick_rate: true` в `axiom-cli.yaml`. Без этого флага `tick_schedule.adaptive_tick` загружается но не применяется — `tick_hz` используется как фиксированная частота.

---

## GuardianConfig (планируется)

**Статус:** не реализован. Параметры захардкожены в `axiom-runtime/src/guardian.rs`.

`GuardianConfig` — это **ручки скорости обучения** модели. Guardian использует их в `adapt_thresholds` и `adapt_domain_physics` чтобы решать насколько агрессивно менять пороги Arbiter и физику домена в ответ на feedback.

Планируемые поля (текущие захардкоженные значения):

| Поле | Значение | Назначение |
|------|----------|------------|
| `high_success_threshold` | 0.8 | success_rate выше → пороги ужесточаются |
| `low_success_threshold` | 0.3 | success_rate ниже → пороги ослабляются |
| `physics_high_threshold` | 0.7 | success_rate выше → temperature снижается |
| `threshold_step` | 5 | шаг изменения reflex_threshold за цикл |
| `temp_step` | 5.0 | шаг изменения temperature (Кельвин) |
| `temp_min` | 0.1 | нижний предел temperature после адаптации |
| `temp_max` | 500.0 | верхний предел temperature после адаптации |
| `resonance_step` | 10 | шаг изменения resonance_freq |
| `confidence_ceiling` | 0.99 | верхняя граница валидного ML confidence |

Когда будет реализован — загружается аналогично `HeartbeatConfig`: опциональный путь в `axiom-cli.yaml`, при отсутствии файла — используются defaults из кода.

```yaml
# axiom-cli.yaml (планируется)
guardian_config_file: "presets/guardian.yaml"
```

```yaml
# presets/guardian.yaml (планируется)
high_success_threshold: 0.8
low_success_threshold: 0.3
physics_high_threshold: 0.7
threshold_step: 5
temp_step: 5.0
temp_min: 0.1
temp_max: 500.0
resonance_step: 10
confidence_ceiling: 0.99
```

---

## Интеграция с модулями

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

## Пресеты и валидация

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

## Примеры использования

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

## Тестирование конфигураций

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

## Горячая перезагрузка (ConfigWatcher)

`ConfigWatcher` следит за `axiom.yaml` через inotify (Linux) / FSEvents (macOS) и перезагружает конфигурацию при изменении файла.

### Как включить в CLI

Три равнозначных способа:

```bash
# 1. CLI-флаг
cargo run --bin axiom-cli -- --hot-reload

# 2. axiom-cli.yaml
hot_reload: true

# 3. Программно
```

### Что перезагружается

| Параметр | Перезагружается | Примечание |
|----------|-----------------|------------|
| `tick_schedule.*` | ✅ | Применяется к живому Engine без остановки |
| `tick_hz` | ❌ | Требует перезапуска |
| `verbose` | ❌ | Требует перезапуска |
| `detail_level` | ❌ | Требует перезапуска |

Механизм: при изменении `axiom.yaml` тик-петля CLI перечитывает `axiom-cli.yaml` и применяет из него новый `tick_schedule` к `engine.tick_schedule`.

### Программное использование

```rust
use axiom_config::{ConfigWatcher, ConfigLoader};
use std::time::Duration;

let watcher = ConfigWatcher::new("config/axiom.yaml")?;

loop {
    if let Some(new_cfg) = watcher.poll() {
        // new_cfg — LoadedAxiomConfig с обновлёнными данными
        println!("Config reloaded: {} domains", new_cfg.domains.len());
        if let Some(hb) = new_cfg.heartbeat {
            println!("New heartbeat interval: {}", hb.interval);
        }
    }
    std::thread::sleep(Duration::from_millis(500));
}
```

`poll()` неблокирующий. Несколько изменений файла между вызовами `poll()` сворачиваются в одну перезагрузку.

### HeartbeatConfig — пример файла

`config/presets/heartbeat.yaml`:

```yaml
interval: 1024
batch_size: 10
connection_batch_size: 5
enable_decay: true
enable_gravity: true
enable_spatial_collision: true
enable_connection_maintenance: true
enable_thermodynamics: true
attach_pulse_id: true
enable_shell_reconciliation: true
```

Пресеты из кода: `HeartbeatConfig::weak()`, `HeartbeatConfig::medium()`, `HeartbeatConfig::powerful()`, `HeartbeatConfig::disabled()`.

---

## Troubleshooting

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

**Последнее обновление:** 2026-04-21  
**Версия Configuration System:** V1.1  
**Статус:** ✅ Heartbeat, ConfigWatcher, AdaptiveTickRate задокументированы. GuardianConfig — запланирован.
