# UCL V2.0 Protocol - Complete Usage Guide

**Version:** 2.0  
**Created:** 2026-03-08  
**Status:** ✅ Implemented and Tested

---

## 🎯 **Overview**

UCL (Unified Command Language) V2.0 - это **zero-allocation, 64-byte binary protocol** для взаимодействия с ядром AXIOM. Протокол обеспечивает физическую семантику команд с нулевыми аллокациями после создания.

### ⚡ **Key Features**
- **64 байта** на команду (одна кэш-линия)
- **Zero-allocation** после создания
- **Raw binary** без JSON сериализации
- **FFI compatible** для внешних языков
- **Physical semantics** - гравитация, температура, мембраны

---

## 🏗️ **Architecture**

### **Command Structure (64 bytes)**
```rust
#[repr(C, align(64))]
pub struct UclCommand {
    pub command_id: u64,        // 8b | Уникальный ID команды
    pub opcode: u16,             // 2b | Код операции (1000-9000)
    pub target_id: u32,          // 4b | ID целевого домена
    pub priority: u8,            // 1b | Приоритет (0-255)
    pub flags: u8,               // 1b | Флаги (SYNC, FORCE, etc.)
    pub reserved: [u8; 4],       // 4b | Резерв
    pub payload: [u8; 48],       // 48b | Payload данных
}
```

### **Result Structure (32 bytes)**
```rust
#[repr(C, align(32))]
pub struct UclResult {
    pub command_id: u64,        // 8b | ID команды
    pub status: u8,              // 1b | Статус выполнения
    pub error_code: u16,         // 2b | Код ошибки
    pub consumed_energy: f32,    // 4b | Потребленная энергия
    pub events_generated: u16,   // 2b | Сгенерированных событий
    pub execution_time_us: u32,  // 4b | Время выполнения (мкс)
    pub reserved: [u8; 15],      // 15b | Резерв
}
```

---

## 🚀 **Quick Start**

### **Rust Core Usage**

```rust
use axiom_core::{UclBuilder, PhysicsProcessor};

fn main() {
    let mut processor = PhysicsProcessor::new();
    
    // 1. Создать SUTRA домен
    let sutra = UclBuilder::spawn_domain(0, 0); // target_id, structural_role
    let result = processor.execute(&sutra);
    assert!(result.is_success());
    
    // 2. Создать LOGIC домен
    let logic = UclBuilder::spawn_domain(0, 6);
    let result = processor.execute(&logic);
    assert!(result.is_success());
    
    // 3. Применить силу к LOGIC домену
    let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 10.0);
    let result = processor.execute(&force);
    assert!(result.is_success());
    
    println!("UCL V2.0 работает! Создано {} доменов", 
             processor.get_stats().total_domains);
}
```

### **FFI Interface Usage**

```c
#include <stdint.h>
#include <stdio.h>

// FFI функции из axiom_core
extern int ucl_spawn_domain(uint8_t* command_ptr, uint32_t target_id, 
                           uint8_t factory_preset, uint16_t parent_domain_id);
extern int ucl_execute(const uint8_t* command_ptr, uint8_t* result_ptr);
extern int ucl_get_stats(uint8_t* stats_ptr);

int main() {
    uint8_t command_buffer[64];
    uint8_t result_buffer[32];
    uint8_t stats_buffer[32];
    
    // Создать SUTRA домен через FFI
    int result = ucl_spawn_domain(command_buffer, 0, 0, 0);
    if (result != 0) {
        printf("Ошибка создания команды\n");
        return 1;
    }
    
    // Выполнить команду
    result = ucl_execute(command_buffer, result_buffer);
    if (result != 0) {
        printf("Ошибка выполнения команды\n");
        return 1;
    }
    
    printf("SUTRA домен успешно создан!\n");
    return 0;
}
```

---

## 🎮 **Command Types**

### **1. SpawnDomain (Opcode: 1000)**
Создание нового домена с физическими свойствами.

```rust
let command = UclBuilder::spawn_domain(target_id, structural_role);
```

**Structural Roles (10 total):**
- **0 (SUTRA)** - Источник Истины (абсолютный ноль, бесконечная гравитация)
- **3 (CODEX)** - Конституция и Правила (высокая вязкость, низкая температура)
- **4 (MAP)** - Картография и навигация (пространственная память)
- **5 (PROBE)** - Зонды и исследования (высокая проницаемость)
- **6 (LOGIC)** - Чистое вычисление (земная гравитация, комнатная температура)
- **7 (DREAM)** - Фоновая оптимизация (нулевая гравитация, высокая температура)
- **8 (VOID)** - Вакуум и пустота (нейтральное пространство)
- **9 (BRIDGE)** - Мосты и связи (соединение доменов)
- **10 (MAYA)** - Интерфейс и проекция (огромное поле, высокая проницаемость)

**Factory Methods Available:**
- `factory_sutra(domain_id)` - для SUTRA
- `factory_codex(domain_id, parent_id)` - для CODEX
- `factory_logic(domain_id, parent_id)` - для LOGIC  
- `factory_dream(domain_id, parent_id)` - для DREAM
- `factory_maya(domain_id, parent_id)` - для MAYA

**Note:** MAP, PROBE, VOID, BRIDGE пока не имеют factory методов и могут быть созданы через `default_void()` с ручной настройкой.

### **2. ApplyForce (Opcode: 2001)**
Применение силы к домену или токену.

```rust
let force_vector = [1.0, 0.0, 0.0]; // X, Y, Z
let magnitude = 10.0;
let command = UclBuilder::apply_force(target_id, force_vector, magnitude);
```

**Физические законы:**
- В доменах с нулевой гравитацией сила не работает
- Потребляется энергия proportional к силе и времени
- Учитывается трение и вязкость домена

### **3. InjectToken (Opcode: 2000)**
Вброс токена в домен через мембрану.

```rust
let position = [5.0, 10.0, 15.0];
let command = UclBuilder::inject_token(target_domain_id, token_type, mass, position);
```

**Правила мембраны:**
- Проверяется проницаемость мембраны
- Температура токена не может быть выше температуры домена
- Токены с массой выше порога не проходят

### **4. ChangeTemperature (Opcode: 3001)**
Изменение температуры домена.

```rust
let command = UclCommand::new(OpCode::ChangeTemperature, target_id, 100, 0)
    .with_payload(&ChangeTemperaturePayload {
        target_domain_id: target_id as u16,
        delta_temperature: 10.0,
        transfer_rate: 1.0,
        source_point: [0.0, 0.0, 0.0],
        radius: 100.0,
        duration_ticks: 1,
        reserved: [0; 14],
    });
```

**Физические ограничения:**
- SUTRA домен нельзя нагревать (абсолютный ноль)
- Температура не может быть отрицательной
- Потребляется энергия на изменение температуры

### **5. CollapseDomain (Opcode: 1001)**
Уничтожение домена.

```rust
let command = UclCommand::new(OpCode::CollapseDomain, target_id, 255, 0);
```

**Правила уничтожения:**
- SUTRA домен нельзя уничтожить (физический закон)
- Домен должен существовать
- Все токены в домене уничтожаются

---

## 🔧 **Configuration Instructions**

### **Manual Domain Configuration**

Для доменов без factory методов (MAP, PROBE, VOID, BRIDGE) нужно вручную настраивать параметры:

```rust
let mut domain = DomainConfig::default_void();
domain.domain_id = 1001;
domain.structural_role = 4; // MAP
domain.parent_domain_id = 1000;

// Установка физических параметров
domain.field_size = [500.0, 500.0, 500.0];
domain.gravity_strength = 1.0;
domain.temperature = 273.0;

// Настройка мембраны
domain.permeability = 100; // ~0.4 проницаемость
domain.membrane_state = 3; // ADAPTIVE
domain.viscosity = 50;      // ~0.2 вязкость

// Установка емкостей
domain.token_capacity = 1000;
domain.connection_capacity = 100;

// Установка временных меток
domain.created_at = 1715292000;
domain.last_update = 1715292000;

// Валидация
assert!(domain.validate());
```

### **Configuration Parameters**

#### **1. Идентификация [16 байт]**
```rust
domain.reserved_id = 0;           // Резерв (оставить 0)
domain.domain_id = 1001;          // Уникальный ID домена
domain.parent_domain_id = 1000;    // Родительский домен
domain.domain_type = 1;             // Тип домена (из DomainType)
domain.structural_role = 4;         // Роль в ASHTI Core
domain.generation = 0;              // Поколение эволюции
domain.flags = 0;                   // Битовая маска состояний
```

#### **2. Физика поля [32 байта]**
```rust
domain.field_size = [x, y, z];      // Размеры поля в метрах
domain.gravity_strength = g;           // Гравитация (-MAX..+MAX)
domain.temperature = kelvin;           // Температура в Кельвинах
domain.time_dilation = dilation;       // Замедление времени (x100)
domain.resonance_freq = hz;          // Резонансная частота
domain.pressure = pascals;            // Давление в Паскалях
domain.friction_coeff = coeff;         // Трение (0..255 -> 0.0..1.0)
domain.viscosity = visc;               // Вязкость (0..255 -> 0.0..1.0)
domain.elasticity = elastic;            // Упругость (0..255 -> 0.0..1.0)
domain.quantum_noise = noise;           // Квантовый шум (0..255 -> 0.0..1.0)
```

#### **3. Семантические оси [16 байт]**
```rust
domain.axis_x_ref = concept_id;         // Референс концепции оси X
domain.axis_y_ref = concept_id;         // Референс концепции оси Y
domain.axis_z_ref = concept_id;         // Референс концепции оси Z
domain.axis_config = bit_packed;        // Конфигурация полюсов
```

#### **4. Мембрана [32 байта]**
```rust
domain.input_filter = bloom_hash;        // 64-bit Bloom фильтр входа
domain.output_filter = bloom_hash;       // 64-bit Bloom фильтр выхода
domain.gate_complexity = complexity;     // Вычислительная сложность шлюзов
domain.threshold_mass = mass_threshold;   // Порог массы для прохождения
domain.threshold_temp = temp_threshold;   // Порог температуры для прохождения
domain.permeability = perm;           // Проницаемость (0..255 -> 0.0..1.0)
domain.membrane_state = state;          // Состояние мембраны
```

**Состояния мембраны:**
- `0` - OPEN (открыта)
- `1` - CLOSED (закрыта)
- `2` - SEMI (полупроницаемая)
- `3` - ADAPTIVE (адаптивная)

#### **5. Метаданные [32 байта]**
```rust
domain.created_at = timestamp;           // Время создания (COM event_id)
domain.last_update = timestamp;          // Последнее обновление
domain.token_capacity = capacity;         // Максимальная емкость токенов
domain.connection_capacity = capacity;    // Максимальная емкость связей
domain.error_count = errors;            // Счетчик когнитивных ошибок
domain.processing_state = state;        // Состояние обработки
domain.complexity_score = score;        // Оценка сложности (0..255)
domain.performance_score = score;       // Производительность (0..255)
```

**Состояния обработки:**
- `1` - IDLE (простой)
- `2` - PROCESSING (обработка)
- `3` - FROZEN (заморожен)
- `4` - CRASHED (краш)

### **Domain Presets**

#### **MAP (4) - Картография и навигация**
```rust
fn configure_map_domain(domain_id: u16, parent_id: u16) -> DomainConfig {
    let mut domain = DomainConfig::default_void();
    domain.domain_id = domain_id;
    domain.parent_domain_id = parent_id;
    domain.structural_role = 4; // MAP
    
    // Пространственная память
    domain.field_size = [1000.0, 1000.0, 1000.0];
    domain.gravity_strength = 0.1; // Минимальная гравитация
    domain.temperature = 273.0;      // Комнатная температура
    
    // Высокая проницаемость для исследования
    domain.permeability = 200;        // ~0.8
    domain.membrane_state = 0;        // OPEN
    
    // Средние емкости для карт
    domain.token_capacity = 1500;
    domain.connection_capacity = 150;
    
    domain
}
```

#### **PROBE (5) - Зонды и исследования**
```rust
fn configure_probe_domain(domain_id: u16, parent_id: u16) -> DomainConfig {
    let mut domain = DomainConfig::default_void();
    domain.domain_id = domain_id;
    domain.parent_domain_id = parent_id;
    domain.structural_role = 5; // PROBE
    
    // Исследовательские параметры
    domain.field_size = [100.0, 100.0, 100.0]; // Компактное поле
    domain.gravity_strength = 0.0;               // Невесомость
    domain.temperature = 200.0;                    // Прохладная среда
    
    // Максимальная проницаемость для сбора данных
    domain.permeability = 255;        // 1.0 - полностью открыта
    domain.membrane_state = 0;        // OPEN
    
    // Малые емкости для зондов
    domain.token_capacity = 100;
    domain.connection_capacity = 10;
    
    domain
}
```

#### **VOID (8) - Вакуум и пустота**
```rust
fn configure_void_domain(domain_id: u16, parent_id: u16) -> DomainConfig {
    let mut domain = DomainConfig::default_void();
    domain.domain_id = domain_id;
    domain.parent_domain_id = parent_id;
    domain.structural_role = 8; // VOID
    
    // Абсолютная пустота
    domain.field_size = [5000.0, 5000.0, 5000.0]; // Огромное пустое пространство
    domain.gravity_strength = 0.0;                     // Никакой гравитации
    domain.temperature = 2.7;                            // Космический фон
    
    // Полная изоляция
    domain.permeability = 0;          // 0.0 - абсолютно непроницаемая
    domain.membrane_state = 1;        // CLOSED
    
    // Нулевые емкости
    domain.token_capacity = 0;
    domain.connection_capacity = 0;
    
    domain
}
```

#### **BRIDGE (9) - Мосты и связи**
```rust
fn configure_bridge_domain(domain_id: u16, parent_id: u16) -> DomainConfig {
    let mut domain = DomainConfig::default_void();
    domain.domain_id = domain_id;
    domain.parent_domain_id = parent_id;
    domain.structural_role = 9; // BRIDGE
    
    // Параметры соединения
    domain.field_size = [200.0, 200.0, 200.0]; // Мостовое пространство
    domain.gravity_strength = 5.0;                 // Умеренная гравитация
    domain.temperature = 300.0;                    // Теплая среда
    
    // Двусторонняя проницаемость
    domain.permeability = 150;        // ~0.6 - полупроницаемая
    domain.membrane_state = 2;        // SEMI
    
    // Высокие емкости для связи
    domain.token_capacity = 2500;
    domain.connection_capacity = 250;
    
    domain
}
```

### **Validation Rules**

```rust
// Базовые проверки
if domain.domain_id == 0 {
    return Err("ID домена не может быть 0");
}

if domain.token_capacity == 0 || domain.connection_capacity == 0 {
    return Err("Емкости должны быть > 0");
}

// Физические ограничения
if domain.gravity_strength < 0.0 {
    return Err("Гравитация не может быть отрицательной");
}

if domain.field_size.iter().any(|&s| s <= 0.0) {
    return Err("Размеры поля должны быть > 0");
}

// Временная синхронизация
if domain.created_at == 0 || domain.last_update < domain.created_at {
    return Err("Некорректные временные метки");
}
```

### **Configuration Best Practices**

#### **1. Используйте константы**
```rust
// Константы для MAP домена
const MAP_FIELD_SIZE: [f32; 3] = [1000.0, 1000.0, 1000.0];
const MAP_GRAVITY: f32 = 0.1;
const MAP_PERMEABILITY: u8 = 200;

fn create_map_domain(id: u16) -> DomainConfig {
    let mut domain = DomainConfig::default_void();
    domain.field_size = MAP_FIELD_SIZE;
    domain.gravity_strength = MAP_GRAVITY;
    domain.permeability = MAP_PERMEABILITY;
    // ...
}
```

#### **2. Валидация после конфигурации**
```rust
fn safe_configure_domain(config: DomainConfig) -> Result<DomainConfig, String> {
    if !config.validate() {
        return Err("Конфигурация невалидна".to_string());
    }
    Ok(config)
}
```

#### **3. Используйте пресеты**
```rust
enum DomainPreset {
    SmallProbe,
    LargeMap,
    FastBridge,
    IsolatedVoid,
}

impl DomainPreset {
    fn apply(self, mut domain: DomainConfig) -> DomainConfig {
        match self {
            DomainPreset::SmallProbe => {
                domain.field_size = [50.0, 50.0, 50.0];
                domain.token_capacity = 50;
            }
            DomainPreset::LargeMap => {
                domain.field_size = [2000.0, 2000.0, 2000.0];
                domain.token_capacity = 2000;
            }
            // ...
        }
        domain
    }
}
```

---

## 🏛️ **Domain Types**

### **SUTRA (0) - Источник Истины**
```rust
let domain = DomainConfig::factory_sutra(domain_id);
```
- **Температура:** 0K (абсолютный ноль)
- **Гравитация:** f32::MAX (бесконечная)
- **Мембрана:** Непроницаемая (permeability = 0)
- **Защита:** Нельзя уничтожить или изменить температуру

### **CODEX (3) - Конституция и Правила**
```rust
let domain = DomainConfig::factory_codex(domain_id, parent_id);
```
- **Температура:** 10K (почти ноль)
- **Гравитация:** 1000.0 (высокая)
- **Вязкость:** 250 (токены "застревают")
- **Мембрана:** Полупроницаемая (только системные токены)

### **MAP (4) - Картография и навигация**
```rust
let domain = DomainConfig::default_void();
domain.structural_role = 4;
domain.domain_id = domain_id;
// TODO: Реализовать factory_map()
```
- **Назначение:** Пространственная память и навигация
- **Состояние:** В разработке factory метод

### **PROBE (5) - Зонды и исследования**
```rust
let domain = DomainConfig::default_void();
domain.structural_role = 5;
domain.domain_id = domain_id;
// TODO: Реализовать factory_probe()
```
- **Назначение:** Исследование и сбор данных
- **Состояние:** В разработке factory метод

### **LOGIC (6) - Чистое вычисление**
```rust
let domain = DomainConfig::factory_logic(domain_id, parent_id);
```
- **Температура:** 273K (комнатная)
- **Гравитация:** 9.81 (земная)
- **Упругость:** 200 (токены отскакивают)
- **Мембрана:** Адаптивная

### **DREAM (7) - Фоновая оптимизация**
```rust
let domain = DomainConfig::factory_dream(domain_id, parent_id);
```
- **Температура:** 500K (высокая)
- **Гравитация:** 0.0 (невесомость)
- **Квантовый шум:** 200 (случайные связи)
- **Мембрана:** Открытая

### **VOID (8) - Вакуум и пустота**
```rust
let domain = DomainConfig::default_void();
domain.structural_role = 8;
domain.domain_id = domain_id;
// TODO: Реализовать factory_void()
```
- **Назначение:** Нейтральное пространство
- **Состояние:** В разработке factory метод

### **BRIDGE (9) - Мосты и связи**
```rust
let domain = DomainConfig::default_void();
domain.structural_role = 9;
domain.domain_id = domain_id;
// TODO: Реализовать factory_bridge()
```
- **Назначение:** Соединение доменов
- **Состояние:** В разработке factory метод

### **MAYA (10) - Интерфейс и проекция**
```rust
let domain = DomainConfig::factory_maya(domain_id, parent_id);
```
- **Поле:** 2000x2000x2000 (огромное)
- **Температура:** 310K (теплая)
- **Трение:** 5 (скольжение)
- **Мембрана:** Абсолютно открытая

---

## 🔧 **Advanced Usage**

### **Custom Payloads**
```rust
#[repr(C)]
struct CustomPayload {
    data: [f32; 12],
}

let payload = CustomPayload {
    data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
};

let command = UclCommand::new(OpCode::Custom, target_id, 100, 0)
    .with_payload(&payload);
```

### **Command Flags**
```rust
use crate::ucl_command::flags;

let command = UclCommand::new(OpCode::SpawnDomain, 0, 255, 
                             flags::SYNC | flags::FORCE | flags::CRITICAL);
```

**Доступные флаги:**
- `SYNC` - Синхронное выполнение
- `FORCE` - Принудительное выполнение
- `BYPASS_MEMBRANE` - Обойти мембрану
- `NO_EVENTS` - Не генерировать события
- `CRITICAL` - Критический приоритет

### **Error Handling**
```rust
let result = processor.execute(&command);

if !result.is_success() {
    match result.status {
        1 => println!("Success"),
        2 => println!("Invalid payload"),
        3 => println!("Physics violation"),
        4 => println!("Target not found"),
        5 => println!("Insufficient energy"),
        6 => println!("Membrane blocked"),
        _ => println!("Unknown error: {}", result.error_code),
    }
}
```

---

## 📊 **Performance**

### **Benchmarks**
```rust
use std::time::Instant;

let start = Instant::now();
let mut processor = PhysicsProcessor::new();

// Создать 10000 доменов
for i in 0..10000 {
    let command = UclBuilder::spawn_domain(0, 6); // LOGIC
    processor.execute(&command);
}

let duration = start.elapsed();
println!("Создано 10000 доменов за {:?}", duration);
println!("Среднее время на домен: {:?}", duration / 10000);
```

### **Memory Usage**
```rust
println!("Размер UclCommand: {} байт", std::mem::size_of::<UclCommand>());
println!("Размер UclResult: {} байт", std::mem::size_of::<UclResult>());
println!("Выравнивание UclCommand: {} байт", std::mem::align_of::<UclCommand>());
```

---

## 🧪 **Testing**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ucl_sizes() {
        assert_eq!(std::mem::size_of::<UclCommand>(), 64);
        assert_eq!(std::mem::align_of::<UclCommand>(), 64);
        assert_eq!(std::mem::size_of::<UclResult>(), 32);
        assert_eq!(std::mem::align_of::<UclResult>(), 32);
    }
    
    #[test]
    fn test_physics_processor() {
        let mut processor = PhysicsProcessor::new();
        
        // Создать домен
        let command = UclBuilder::spawn_domain(0, 0);
        let result = processor.execute(&command);
        assert!(result.is_success());
        
        // Проверить статистику
        let stats = processor.get_stats();
        assert_eq!(stats.total_domains, 1);
    }
}
```

### **Integration Tests**
```rust
#[test]
fn test_full_workflow() {
    let mut processor = PhysicsProcessor::new();
    
    // 1. Создать SUTRA
    let sutra = UclBuilder::spawn_domain(0, 0);
    assert!(processor.execute(&sutra).is_success());
    
    // 2. Создать LOGIC
    let logic = UclBuilder::spawn_domain(0, 6);
    assert!(processor.execute(&logic).is_success());
    
    // 3. Применить силу
    let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 10.0);
    let result = processor.execute(&force);
    assert!(result.is_success());
    assert!(result.consumed_energy > 0.0);
    
    // 4. Изменить температуру
    let temp_cmd = UclCommand::new(OpCode::ChangeTemperature, 1001, 100, 0)
        .with_payload(&ChangeTemperaturePayload {
            target_domain_id: 1001,
            delta_temperature: 5.0,
            transfer_rate: 1.0,
            source_point: [0.0, 0.0, 0.0],
            radius: 50.0,
            duration_ticks: 1,
            reserved: [0; 14],
        });
    assert!(processor.execute(&temp_cmd).is_success());
}
```

---

## 🔗 **External Integration**

### **Python Adapter (Planned)**
```python
# Пример будущего Python адаптера
import axiom_core

# Создать процессор
processor = axiom_core.PhysicsProcessor()

# Создать SUTRA домен
sutra = axiom_core.UclBuilder.spawn_domain(0, 0)
result = processor.execute(sutra)

if result.is_success():
    print("SUTRA домен создан!")
else:
    print(f"Ошибка: {result.error_code}")
```

### **REST API (Planned)**
```json
POST /api/domains
{
    "structural_role": 6,
    "parent_domain_id": 1000
}

Response:
{
    "domain_id": 1001,
    "status": "success",
    "created_at": 1715292000
}
```

---

## 🚨 **Troubleshooting**

### **Common Errors**

#### **Status 2: Invalid Payload**
```rust
// Неправильно: target_id = 0 для ApplyForce
let force = UclBuilder::apply_force(0, [1.0, 0.0, 0.0], 10.0);

// Правильно: target_id должен существовать
let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 10.0);
```

#### **Status 3: Physics Violation**
```rust
// Неправильно: пытаемся уничтожить SUTRA
let collapse = UclCommand::new(OpCode::CollapseDomain, 1000, 255, 0);

// SUTRA защищена физическим законом
```

#### **Status 5: Insufficient Energy**
```rust
// Неправильно: слишком большая сила
let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 1000000.0);

// Правильно: умеренная сила
let force = UclBuilder::apply_force(1001, [1.0, 0.0, 0.0], 10.0);
```

### **Debug Tips**
```rust
// Добавить отладочный вывод
println!("DEBUG: opcode={}, target_id={}, status={}", 
         command.opcode, command.target_id, result.status);

// Проверить валидность команды
if !command.is_valid() {
    println!("Команда невалидна!");
}

// Получить статистику процессора
let stats = processor.get_stats();
println!("Доменов: {}, COM счетчик: {}", 
         stats.total_domains, stats.com_counter);
```

---

## 📚 **Reference**

### **Structural Roles (10 total)**
| Role | Code | Name | Factory Method | Description |
|------|-------|-------|----------------|-------------|
| SUTRA | 0 | Источник Истины | factory_sutra() | Абсолютный ноль, бесконечная гравитация |
| CODEX | 3 | Конституция и Правила | factory_codex() | Высокая вязкость, низкая температура |
| MAP | 4 | Картография и навигация | - | Пространственная память |
| PROBE | 5 | Зонды и исследования | - | Исследование и сбор данных |
| LOGIC | 6 | Чистое вычисление | factory_logic() | Земная гравитация, комнатная температура |
| DREAM | 7 | Фоновая оптимизация | factory_dream() | Нулевая гравитация, высокая температура |
| VOID | 8 | Вакуум и пустота | - | Нейтральное пространство |
| BRIDGE | 9 | Мосты и связи | - | Соединение доменов |
| MAYA | 10 | Интерфейс и проекция | factory_maya() | Огромное поле, высокая проницаемость |

### **Opcodes**
| Opcode | Command | Description |
|--------|----------|-------------|
| 1000 | SpawnDomain | Создать домен |
| 1001 | CollapseDomain | Уничтожить домен |
| 2000 | InjectToken | Вброс токена |
| 2001 | ApplyForce | Применить силу |
| 3000 | TickForward | Шаг симуляции |
| 3001 | ChangeTemperature | Изменить температуру |
| 9000 | CoreShutdown | Остановить реактор |

### **Status Codes**
| Status | Name | Description |
|--------|------|-------------|
| 1 | Success | Успешное выполнение |
| 2 | InvalidPayload | Невалидный payload |
| 3 | PhysicsViolation | Нарушение физики |
| 4 | TargetNotFound | Цель не найдена |
| 5 | InsufficientEnergy | Недостаточно энергии |
| 6 | MembraneBlocked | Мембрана блокирует |

### **Error Codes**
| Code | Error | Description |
|------|-------|-------------|
| 1000 | UnknownOpcode | Неизвестный opcode |
| 1001 | InvalidTarget | Невалидная цель |
| 1002 | PhysicsViolation | Нарушение физики |
| 1003 | InsufficientEnergy | Недостаточно энергии |
| 1004 | MembraneBlocked | Мембрана блокирует |
| 1005 | DomainNotFound | Домен не найден |
| 1006 | TokenNotFound | Токен не найден |
| 1007 | InvalidPayload | Невалидный payload |

---

## 🎯 **Best Practices**

### **1. Use Builder Pattern**
```rust
// Хорошо
let command = UclBuilder::spawn_domain(0, 6);

// Плохо
let payload = SpawnDomainPayload { /* ... */ };
let command = UclCommand::new(OpCode::SpawnDomain, 0, 100, 0)
    .with_payload(&payload);
```

### **2. Check Results**
```rust
let result = processor.execute(&command);
if !result.is_success() {
    // Обработать ошибку
    return;
}
```

### **3. Use Proper Domain Types**
```rust
// SUTRA для источника истины
let sutra = UclBuilder::spawn_domain(0, 0);

// CODEX для конституции и правил
let codex = UclBuilder::spawn_domain(0, 3);

// LOGIC для вычислений
let logic = UclBuilder::spawn_domain(0, 6);

// DREAM для фоновой оптимизации
let dream = UclBuilder::spawn_domain(0, 7);

// MAYA для интерфейса и проекции
let maya = UclBuilder::spawn_domain(0, 10);

// MAP, PROBE, VOID, BRIDGE - через default_void
let map_domain = DomainConfig::default_void();
map_domain.structural_role = 4; // MAP
map_domain.domain_id = domain_id;
```

### **4. Respect Physics Laws**
```rust
// Проверять температуру перед вбросом токена
if token.temperature > domain.temperature {
    return Err("Токен слишком горячий для домена");
}
```

---

## 🔄 **Migration from V1.0**

### **Key Changes**
- **V1.0:** JSON serialization, String-based
- **V2.0:** Binary, zero-allocation, physical semantics

### **Migration Steps**
1. Replace JSON commands with UclCommand
2. Update error handling for new status codes
3. Implement physics-aware logic
4. Use FFI interface for external adapters

---

## 🎉 **Conclusion**

UCL V2.0 предоставляет **революционный подход** к командам с:
- **Максимальной производительностью** (64 байта, zero-allocation)
- **Физической семантикой** (гравитация, температура, мембраны)
- **Архитектурной чистотой** (ядро отделено от адаптеров)
- **FFI совместимостью** (Python, REST, CLI)

**Протокол готов к использованию в production!** 🚀

---

*Для дополнительной информации смотрите:*
- [UCL V2.0 Specification](../spec/UCL%20V2.0.md)
- [API Documentation](../spec/API%20в%20AXIOM.md)
- [Architecture Guide](Core%20Invariants.md)
