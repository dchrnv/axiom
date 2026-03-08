# DomainConfig V2.0 - Руководство разработчика

## 📋 Обзор

DomainConfig V2.0 - это оптимизированная структура конфигурации домена размером ровно 128 байт с data packing для максимальной производительности.

### 🎯 **Ключевые особенности:**
- **Точно 128 байт** - идеальное кэширование (одна кэш-линия)
- **SIMD-готовность** - выравнивание для AVX-512 инструкций
- **Data Packing** - квантованные значения вместо плавающей точки
- **64-bit Bloom фильтры** - эффективная мембранная фильтрация

---

## 🏗️ Структура DomainConfig

### 📊 **Размещение памяти (128 байт):**

```
Offset | Size | Section          | Fields
-------|------|------------------|-------------------------------
0      | 16   | ИДЕНТИФИКАЦИЯ    | reserved_id, domain_id, parent_domain_id, domain_type, structural_role, generation, flags
16     | 32   | ФИЗИКА ПОЛЯ      | field_size[3], gravity_strength, temperature, time_dilation, resonance_freq, pressure, reserved_physics, friction_coeff, viscosity, elasticity, quantum_noise
48     | 16   | СЕМАНТИЧЕСКИЕ ОСИ | axis_x_ref, axis_y_ref, axis_z_ref, axis_config
64     | 32   | МЕМБРАНА         | input_filter, output_filter, reserved_membrane, gate_complexity, threshold_mass, threshold_temp, permeability, membrane_state
96     | 32   | МЕТАДАННЫЕ       | created_at, last_update, token_capacity, connection_capacity, error_count, processing_state, complexity_score, performance_score, reserved_meta[3]
```

### 🔧 **Типы данных и диапазоны:**

| Поле | Тип | Диапазон | Описание | Пример |
|-------|------|-----------|-----------|---------|
| `domain_id` | `u16` | 1..65535 | Уникальный ID домена | `1` |
| `domain_type` | `u8` | 1..255 | Тип домена | `1` (Logic) |
| `structural_role` | `u8` | 0..10 | Роль в Ashti_Core | `6` (Logic) |
| `field_size` | `[f32; 3]` | >0.0 | Размеры поля X,Y,Z | `[100.0, 100.0, 100.0]` |
| `gravity_strength` | `f32` | 0.0..MAX | Гравитация | `1.0` |
| `temperature` | `f32` | 0.0..1000.0 | Температура в K | `293.15` |
| `time_dilation` | `u16` | 0..65535 | Замедление времени ×100 | `100` (1.0x) |
| `friction_coeff` | `u8` | 0..255 | Трение (0..1.0) | `25` (≈0.098) |
| `viscosity` | `u8` | 0..255 | Вязкость (0..1.0) | `3` (≈0.012) |
| `elasticity` | `u8` | 0..255 | Упругость (0..1.0) | `128` (≈0.502) |
| `quantum_noise` | `u8` | 0..255 | Квантовый шум (0..1.0) | `1` (≈0.004) |
| `input_filter` | `u64` | 64-bit hash | Bloom фильтр входа | `u64::MAX` |
| `output_filter` | `u64` | 64-bit hash | Bloom фильтр выхода | `u64::MAX` |
| `threshold_mass` | `u16` | 0..65535 | Порог массы | `1` |
| `threshold_temp` | `u16` | 0..65535 | Порог температуры | `200` |
| `permeability` | `u8` | 0..255 | Проницаемость (0..1.0) | `255` (1.0) |
| `membrane_state` | `u8` | 0..3 | Состояние мембраны | `0` (OPEN) |
| `token_capacity` | `u32` | 1..MAX | Емкость токенов | `1000` |
| `connection_capacity` | `u32` | 1..MAX | Емкость связей | `5000` |
| `processing_state` | `u8` | 0..3 | Состояние обработки | `0` (IDLE) |
| `complexity_score` | `u8` | 0..255 | Сложность (0..1.0) | `0` (0.0) |
| `performance_score` | `u8` | 0..255 | Производительность (0..1.0) | `255` (1.0) |

---

## 🚀 Использование DomainConfig

### 📝 **Создание домена:**

```rust
use axiom_core::{DomainConfig, DomainType, StructuralRole};

// Создание с параметрами по умолчанию
let domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti6);

// Создание с предустановкой
let domain = DomainConfig::from_preset("logic")?;

// Создание вручную
let mut domain = DomainConfig::default();
domain.domain_id = 42;
domain.domain_type = DomainType::Dream as u8;
domain.structural_role = StructuralRole::Ashti3 as u8;
```

### ⚙️ **Настройка параметров:**

```rust
let mut domain = DomainConfig::default();

// Физика поля
domain.field_size = [200.0, 150.0, 100.0];
domain.gravity_strength = 0.8;
domain.temperature = 310.15; // 37°C
domain.time_dilation = 150; // 1.5x замедление

// Квантированные коэффициенты (0..255)
domain.friction_coeff = 50;    // ≈0.196
domain.viscosity = 10;         // ≈0.039
domain.elasticity = 200;       // ≈0.784
domain.quantum_noise = 5;      // ≈0.020

// Мембрана
domain.input_filter = 0x123456789ABCDEF0;
domain.output_filter = 0xFEDCBA9876543210;
domain.threshold_mass = 10;
domain.threshold_temp = 250;
domain.permeability = 128; // ≈0.502
domain.membrane_state = MEMBRANE_OPEN;
```

### 🔍 **Валидация:**

```rust
if domain.validate() {
    println!("Domain конфигурация корректна");
} else {
    println!("Domain конфигурация некорректна");
}
```

### 🚪 **Проверка мембраны:**

```rust
// Проверка входа в домен
let mass = 15;
let temperature = 300;

if domain.can_enter(mass, temperature) {
    println!("Объект может войти в домен");
} else {
    println!("Объект не может войти в домен");
}
```

### 📊 **Расчет сложности:**

```rust
let complexity = domain.calculate_complexity();
println!("Сложность домена: {:.2}", complexity);
```

### 🔄 **Обновление метаданных:**

```rust
// Обновление после изменений
domain.update_metadata(event_id);

// Проверка состояния
if domain.is_active() {
    println!("Домен активен");
}
```

---

## 🎛️ Предустановленные конфигурации

### 📋 **Доступные пресеты:**

| Пресет | Тип | Роль | Описание |
|--------|------|-------|----------|
| `"logic"` | Logic | Ashti6 | Логическая обработка |
| `"dream"` | Dream | Ashti3 | Обработка снов |
| `"math"` | Math | Ashti2 | Математические вычисления |
| `"pattern"` | Pattern | Ashti4 | Распознавание паттернов |
| `"memory"` | Memory | Ashti5 | Хранение памяти |
| `"interface"` | Interface | Ashti7 | Интерфейс взаимодействия |

### 🔧 **Использование пресетов:**

```rust
// Загрузка пресета
let logic_domain = DomainConfig::from_preset("logic")?;
let dream_domain = DomainConfig::from_preset("dream")?;

// Кастомизация пресета
let mut custom_domain = DomainConfig::from_preset("math")?;
custom_domain.domain_id = 999;
custom_domain.gravity_strength = 2.0;
```

---

## 🧪 Тестирование

### ✅ **Проверка размера:**

```rust
use std::mem;

let size = mem::size_of::<DomainConfig>();
assert_eq!(size, 128, "DomainConfig должен быть 128 байт");
```

### 🧪 **Юнит тесты:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_creation() {
        let domain = DomainConfig::new(1, DomainType::Logic, StructuralRole::Ashti6);
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.domain_type, DomainType::Logic as u8);
        assert_eq!(domain.structural_role, StructuralRole::Ashti6 as u8);
    }

    #[test]
    fn test_domain_validation() {
        let domain = DomainConfig::default();
        assert!(domain.validate());
    }

    #[test]
    fn test_membrane_filters() {
        let mut domain = DomainConfig::default();
        domain.threshold_mass = 10;
        domain.threshold_temp = 20;
        domain.membrane_state = MEMBRANE_OPEN;
        
        assert!(domain.can_enter(15, 25));
        assert!(!domain.can_enter(5, 25));
        assert!(!domain.can_enter(15, 15));
    }
}
```

---

## 🔧 Оптимизация производительности

### ⚡ **SIMD-оптимизации:**

```rust
// Пакетная обработка доменов
fn process_domains_batch(domains: &[DomainConfig]) {
    // DomainConfig выровнен по 128 байт - идеально для SIMD
    // Можно использовать AVX-512 для обработки 4 доменов одновременно
    
    for chunk in domains.chunks_exact(4) {
        // SIMD обработка 4 доменов
        unsafe {
            // AVX-512 инструкции здесь
        }
    }
}
```

### 🎯 **Кэш-оптимизации:**

```rust
// DomainConfig помещается в одну кэш-линию L1
// Операции с доменом очень быстрые

fn fast_domain_check(domain: &DomainConfig) -> bool {
    // Все поля в кэше L1 - минимальная задержка
    domain.is_active() && domain.processing_state == PROCESSING_IDLE
}
```

---

## 📚 Константы и перечисления

### 🏷️ **DomainType:**

```rust
#[repr(u16)]
pub enum DomainType {
    Logic = 1,
    Dream = 2,
    Math = 3,
    Pattern = 4,
    Memory = 5,
    Interface = 6,
}
```

### 🏗️ **StructuralRole:**

```rust
#[repr(u8)]
pub enum StructuralRole {
    Sutra = 0,
    Ashti1 = 1,
    Ashti2 = 2,
    Ashti3 = 3,
    Ashti4 = 4,
    Ashti5 = 5,
    Ashti6 = 6,
    Ashti7 = 7,
    Ashti8 = 8,
    Maya = 10,
}
```

### 🚪 **MembraneState:**

```rust
pub const MEMBRANE_OPEN: u8 = 0;
pub const MEMBRANE_CLOSED: u8 = 1;
pub const MEMBRANE_SEMI: u8 = 2;
pub const MEMBRANE_ADAPTIVE: u8 = 3;
```

### ⚙️ **ProcessingState:**

```rust
pub const PROCESSING_IDLE: u8 = 0;
pub const PROCESSING_PROCESSING: u8 = 1;
pub const PROCESSING_FROZEN: u8 = 2;
pub const PROCESSING_CRASHED: u8 = 3;
```

---

## 🔍 Отладка и диагностика

### 🐛 **Отладочные макросы:**

```rust
#[cfg(debug_assertions)]
macro_rules! debug_domain {
    ($domain:expr) => {
        println!("Domain ID: {}", $domain.domain_id);
        println!("Type: {}", $domain.domain_type);
        println!("Size: {} bytes", std::mem::size_of_val($domain));
        println!("Active: {}", $domain.is_active());
    };
}
```

### 📊 **Валидация в runtime:**

```rust
impl DomainConfig {
    pub fn debug_print(&self) {
        println!("=== DomainConfig Debug ===");
        println!("ID: {}", self.domain_id);
        println!("Type: {} (role: {})", self.domain_type, self.structural_role);
        println!("Field: [{:.1}, {:.1}, {:.1}]", 
                self.field_size[0], self.field_size[1], self.field_size[2]);
        println!("Physics: g={:.2}, t={:.1}K", 
                self.gravity_strength, self.temperature);
        println!("Membrane: {:#x}/{:#x}", 
                self.input_filter, self.output_filter);
        println!("Capacity: {} tokens, {} connections", 
                self.token_capacity, self.connection_capacity);
        println!("========================");
    }
}
```

---

## 🎯 Лучшие практики

### ✅ **Рекомендации:**

1. **Используйте пресеты** для стандартных конфигураций
2. **Валидируйте** после создания и изменений
3. **Оптимизируйте** квантованные значения (0..255)
4. **Используйте Bloom фильтры** для мембранной фильтрации
5. **Пакетная обработка** для максимальной производительности

### 🚫 **Избегайте:**

1. **Прямое изменение** полей без валидации
2. **Жестко закодированные** значения вместо пресетов
3. **Игнорирование** квантованных диапазонов
4. **Избыточные** проверки в hot paths
5. **Плавающую точку** там где достаточно квантованных значений

---

## 📚 Дополнительные ресурсы

### 📖 **Спецификации:**
- `docs/spec/DomainConfig V2.0 .md` - Полная спецификация
- `docs/spec/DomainConfig V1.0.md` - Предыдущая версия (архив)

### 🔧 **Реализация:**
- `runtime/src/domain.rs` - Основная реализация
- `runtime/src/domain/tests.rs` - Тесты

### ⚙️ **Конфигурации:**
- `config/schema/domain.yaml` - YAML схема
- `config/presets/domain/` - Предустановленные конфигурации

---

**Последнее обновление:** 2026-03-08  
**Версия:** DomainConfig V2.0 (Data Packing)
