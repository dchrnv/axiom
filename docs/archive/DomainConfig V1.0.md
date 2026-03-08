# AXIOM MODULE SPECIFICATION: DOMAINCONFIG V1.0

**Статус:** Актуальная спецификация (core)  
**Версия:** 1.0.0  
**Дата:** 2026-03-04  
**Формат:** 128 байт, `repr(C, align(128))`  
**Связанные спеки:** Domain V1.3, COM V1.0, Token V5.1, Connection V5.0

---

## 1. Назначение

**DomainConfig** — 128-байтная конфигурация, полностью определяющая природу и поведение Домена. Это "ДНК" Домена, которое задает:

- структурную роль в Ashti_Core (SUTRA, ASHTI, MAYA),
- уникальную физику поля (гравитация, трение, температура),
- семантические оси координат,
- мембранные фильтры и пороги,
- метаданные и состояние.

DomainConfig **не изменяется после создания** Домена. Изменение конфигурации требует создания нового Домена.

---

## 2. Структура (128 байт)

```rust
#[repr(C, align(128))]
pub struct DomainConfig {
    // --- ИДЕНТИФИКАЦИЯ (16 Байт) ---
    pub domain_id: u16,         // Уникальный ID Домена
    pub domain_type: u16,       // Тип Домена (Logic, Dream, Math...)
    pub structural_role: u8,    // Роль в Ashti_Core (0, 1-8, 10)
    pub generation: u8,        // Поколение (для эволюции)
    pub parent_domain_id: u16,  // Родительский Домен
    pub flags: u32,             // ACTIVE/LOCKED/TEMPORARY/SYSTEM
    pub reserved_id: [u8; 8],   // Резерв для будущих полей

    // --- ФИЗИКА ПОЛЯ (32 Байт) ---
    pub field_size: [f32; 3],   // Размеры поля (X, Y, Z)
    pub gravity_strength: f32,  // Сила гравитации (0.0..MAX)
    pub friction_coeff: f32,    // Коэффициент трения (0.0..1.0)
    pub resonance_freq: f32,    // Базовая частота резонанса (Hz)
    pub temperature: f32,       // Базовая температура поля (K)
    pub pressure: f32,          // Давление в поле (Pa)
    pub viscosity: f32,         // Вязкость среды (0.0..1.0)
    pub elasticity: f32,        // Упругость поля (0.0..1.0)
    pub quantum_noise: f32,     // Квантовый шум (0.0..1.0)
    pub time_dilation: f32,     // Замедление времени (0.0..10.0)
    pub reserved_physics: [u32; 3], // Резерв

    // --- СЕМАНТИЧЕСКИЕ ОСИ (16 Байт) ---
    pub axis_x_ref: u32,        // Референс концепции оси X
    pub axis_y_ref: u32,        // Референс концепции оси Y  
    pub axis_z_ref: u32,        // Референс концепции оси Z
    pub axis_config: u32,       // Конфигурация осей (бинарные полюса)

    // --- МЕМБРАНА (32 Байт) ---
    pub input_filter: [u8; 16], // Хеши разрешенных входных паттернов
    pub output_filter: [u8; 16], // Хеши разрешенных выходных паттернов
    pub permeability: f32,      // Проницаемость мембраны (0.0..1.0)
    pub threshold_mass: u8,     // Порог массы для входа (1..255)
    pub threshold_temp: u8,     // Порог температуры для входа (0..255)
    pub gate_complexity: u16,   // Сложность шлюзов (0..1000)
    pub membrane_state: u8,      // OPEN/CLOSED/SEMI/ADAPTIVE
    pub reserved_membrane: [u8; 5], // Резерв

    // --- МЕТАДАННЫЕ (32 Байт) ---
    pub created_at: u64,        // COM event_id создания
    pub last_update: u64,       // COM event_id последнего обновления
    pub token_capacity: u32,     // Максимальное количество Token
    pub connection_capacity: u32, // Максимальное количество Connection
    pub energy_budget: f32,      // Бюджет энергии Домена
    pub complexity_score: f32,   // Оценка сложности (0.0..1.0)
    pub processing_state: u8,    // IDLE/PROCESSING/FROZEN/CRASHED
    pub error_count: u16,        // Счетчик ошибок
    pub performance_score: f32,  // Оценка производительности
    pub reserved_meta: [u8; 8],  // Резерв
}
```

---

## 3. Структурные роли в Ashti_Core

```rust
#[repr(u8)]
pub enum StructuralRole {
    Sutra = 0,          // Источник истины, генератор Token
    Execution = 1,       // Реальность, "здесь и сейчас"
    Shadow = 2,          // Воображение, симуляция
    Codex = 3,           // Конституция, правила
    Map = 4,             // Карта мира, факты
    Probe = 5,           // Активное зондирование
    Logic = 6,           // Чистое вычисление
    Dream = 7,           // Фоновая оптимизация
    Void = 8,            // Неопределенность, угрозы
    // 9 зарезервирован
    Maya = 10            // Интерфейс, проекция результата
}
```

---

## 4. Предустановленные конфигурации

### 4.1 SUTRA (Домен 0)
```rust
DomainConfig {
    structural_role: StructuralRole::Sutra,
    field_size: [1000.0, 1000.0, 1000.0],
    gravity_strength: f32::MAX,    // Максимальная гравитация
    friction_coeff: 0.0,           // Нет трения
    temperature: 0.0,              // Абсолютный ноль
    pressure: 0.0,                 // Нет давления
    viscosity: 0.0,                // Нет вязкости
    permeability: 0.0,              // Непроницаемая мембрана
    membrane_state: MembraneState::Closed,
    // ... остальные поля
}
```

### 4.2 LOGIC (Домен 6)
```rust
DomainConfig {
    structural_role: StructuralRole::Logic,
    field_size: [500.0, 500.0, 500.0],
    gravity_strength: 10.0,        // Высокая гравитация
    friction_coeff: 0.1,            // Низкое трение
    temperature: 273.0,             // Комнатная температура
    axis_x_ref: CONCEPT_TRUE_FALSE, // Ось истина/ложь
    axis_y_ref: CONCEPT_DEDUCTION,  // Ось дедукция
    axis_z_ref: CONCEPT_PRECISION,  // Ось точность
    permeability: 0.5,              // Полупроницаемая
    // ... остальные поля
}
```

### 4.3 DREAM (Домен 7)
```rust
DomainConfig {
    structural_role: StructuralRole::Dream,
    field_size: [1000.0, 1000.0, 1000.0],
    gravity_strength: 0.0,          // Нет гравитации
    friction_coeff: 0.8,            // Высокое трение
    temperature: 373.0,             // Высокая температура
    viscosity: 0.9,                // Высокая вязкость
    quantum_noise: 0.7,             // Высокий квантовый шум
    permeability: 0.8,              // Высокая проницаемость
    // ... остальные поля
}
```

### 4.4 MAYA (Домен 10)
```rust
DomainConfig {
    structural_role: StructuralRole::Maya,
    field_size: [2000.0, 2000.0, 2000.0], // Большое поле
    gravity_strength: 1.0,          // Минимальная гравитация
    friction_coeff: 0.01,           // Минимальное трение
    temperature: 310.0,             // Теплое поле
    elasticity: 0.9,                // Высокая упругость
    permeability: 1.0,              // Полностью проницаемая
    membrane_state: MembraneState::Open,
    // ... остальные поля
}
```

---

## 5. Семантические оси

### 5.1 Конфигурация осей
```rust
pub struct AxisConfig {
    pub x_positive: u16,  // Полюс + оси X
    pub x_negative: u16,  // Полюс - оси X
    pub y_positive: u16,  // Полюс + оси Y
    pub y_negative: u16,  // Полюс - оси Y
    pub z_positive: u16,  // Полюс + оси Z
    pub z_negative: u16,  // Полюс - оси Z
}
```

### 5.2 Примеры концепций
```rust
pub enum ConceptRef {
    // Ось X
    WorldSystem = 1001,     // Внешнее vs Внутреннее
    ConceptAbstract = 1002,  // Абстрактное vs Конкретное
    
    // Ось Y  
    ActionObservation = 2001, // Действие vs Наблюдение
    ActivePassive = 2002,     // Активное vs Пассивное
    
    // Ось Z
    TruthHypothesis = 3001,  // Истина vs Гипотеза
    FactNoise = 3002,         // Факт vs Шум
}
```

---

## 6. Мембранные фильтры

### 6.1 Фильтрация по массе
```rust
fn can_enter_by_mass(token: &Token, config: &DomainConfig) -> bool {
    token.mass >= config.threshold_mass
}
```

### 6.2 Фильтрация по температуре
```rust
fn can_enter_by_temperature(token: &Token, config: &DomainConfig) -> bool {
    token.temperature <= config.threshold_temp
}
```

### 6.3 Фильтрация по паттерну
```rust
fn can_enter_by_pattern(sutra_id: u32, config: &DomainConfig) -> bool {
    let pattern_hash = hash_sutra_id(sutra_id);
    config.input_filter.contains(&pattern_hash)
}
```

---

## 7. Инварианты

1. **Размер**: Строго 128 байт с выравниванием 128
2. **Уникальность**: domain_id уникален в рамках системы
3. **Неизменность**: Конфигурация не меняется после создания
4. **Физика**: Все физические параметры в допустимых диапазонах
5. **Емкость**: token_capacity и connection_capacity > 0
6. **COM синхронизация**: created_at > 0, last_update >= created_at

---

## 8. Валидация

```rust
fn validate_domain_config(config: &DomainConfig) -> bool {
    // Базовые проверки
    config.domain_id > 0
    && config.token_capacity > 0
    && config.connection_capacity > 0
    
    // Физические ограничения
    && config.gravity_strength >= 0.0
    && config.friction_coeff >= 0.0 && config.friction_coeff <= 1.0
    && config.permeability >= 0.0 && config.permeability <= 1.0
    
    // Температурные ограничения
    && config.temperature >= 0.0 && config.temperature <= 1000.0
    
    // Размеры поля
    && config.field_size.iter().all(|&s| s > 0.0)
    
    // COM синхронизация
    && config.created_at > 0
    && config.last_update >= config.created_at
}
```

---

## 9. Сериализация

```rust
impl DomainConfig {
    pub fn to_bytes(&self) -> [u8; 128] {
        unsafe { std::mem::transmute_copy(self) }
    }
    
    pub fn from_bytes(bytes: [u8; 128]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}
```

---

## 10. Оптимизации

1. **Cache alignment**: Выравнивание 128 для кэш-линий
2. **Hot/cold separation**: Частые поля в начале структуры
3. **Bit packing**: Флаги и состояния упакованы в биты
4. **Reserved space**: Резерв для будущих расширений

---

## 11. История изменений

- **V1.0**: Каноническая спецификация с полной структурой
- **V0.x**: Концептуальные описания в Domain V1.x
