# AXIOM MODULE SPECIFICATION: DOMAIN V1.3

**Статус:** Актуальная спецификация (core)  
**Версия:** 1.3.0  
**Дата:** 2026-03-04  
**Codename:** "The Cell" (Клетка)  
**Формат:** 128 байт конфигурации + динамическое поле  
**Модель времени:** COM `event_id` (причинный порядок, u64)  
**Связанные спеки:** COM V1.0, Token V5.1, Connection V5.0, UPO v2.1

---

## 1. Назначение

**Domain** — изолированный вычислительный контейнер (клетка) с суверенитетом, обладающий уникальной физикой поля:

- предоставляет изолированное пространство для Token и Connection,
- имеет мембрану (конфигурацию) фильтрующую вход/выход,
- реализует уникальную физику взаимодействия внутри поля,
- принимает сложные паттерны (вопросы) и выдает сжатые кристаллы (ответы),
- может быть заморожен и передан как библиотека следующему уровню.

Domain **не использует wall-clock время**. Все изменения синхронизируются через **COM event_id**.

---

## 2. Архитектура

Domain состоит из трех компонентов:

### 2.1. Anchor (Сутра-Центр)
Точка нуля координат (0,0,0) - гравитационный центр домена.

### 2.2. Field (Активное Поле)
3D пространство с уникальной физикой для Token взаимодействия.

### 2.3. Membrane (Конфигурация)
Фильтрующий слой для входящих/исходящих паттернов.

---

## 3. Структура DomainConfig (128 байт)

```rust
#[repr(C, align(128))]
pub struct DomainConfig {
    // --- ИДЕНТИФИКАЦИЯ (16 Байт) ---
    pub domain_id: u16,         // Уникальный ID домена
    pub domain_type: u16,       // Тип домена (Logic, Dream, Math...)
    pub structural_role: u8,    // Роль в Ashti_Core (0-10)
    pub generation: u8,        // Поколение домена
    pub parent_domain_id: u16,  // Родительский домен
    pub flags: u32,             // ACTIVE/LOCKED/TEMPORARY
    pub reserved_id: [u8; 8],  // Резерв

    // --- ФИЗИКА ПОЛЯ (32 Байт) ---
    pub field_size: [f32; 3],   // Размеры поля (X, Y, Z)
    pub gravity_strength: f32,  // Сила гравитации
    pub friction_coeff: f32,    // Коэффициент трения
    pub resonance_freq: f32,    // Базовая частота резонанса
    pub temperature: f32,       // Базовая температура поля
    pub pressure: f32,          // Давление в поле
    pub viscosity: f32,         // Вязкость среды
    pub elasticity: f32,        // Упругость поля
    pub quantum_noise: f32,     // Квантовый шум
    pub time_dilation: f32,     // Замедление времени
    pub reserved_physics: [u32; 3], // Резерв

    // --- МЕМБРАНА (32 Байт) ---
    pub input_filter: [u8; 16], // Фильтр входящих паттернов
    pub output_filter: [u8; 16],// Фильтр исходящих паттернов
    pub permeability: f32,      // Проницаемость мембраны
    pub threshold_mass: u8,      // Порог массы для входа
    pub threshold_temp: u8,      // Порог температуры
    pub gate_complexity: u16,   // Сложность шлюзов
    pub membrane_state: u8,      // OPEN/CLOSED/SEMI
    pub reserved_membrane: [u8; 5], // Резерв

    // --- МЕТАДАННЫЕ (48 Байт) ---
    pub created_at: u64,        // COM event_id создания
    pub last_update: u64,       // COM event_id последнего обновления
    pub token_count: u32,       // Текущее количество токенов
    pub connection_count: u32,   // Текущее количество связей
    pub energy_level: f32,      // Уровень энергии домена
    pub complexity_score: f32,  // Оценка сложности
    pub processing_state: u8,   // IDLE/PROCESSING/FROZEN
    pub error_count: u16,       // Счетчик ошибок
    pub performance_score: f32,  // Оценка производительности
    pub reserved_meta: [u8; 12], // Резерв
}
```

---

## 4. Структурные роли в Ashti_Core

```rust
#[repr(u8)]
pub enum StructuralRole {
    Sutra = 0,          // Домен-источник, абсолютная истина
    Logic = 1,          // Логическая обработка
    Math = 2,           // Математические операции
    Pattern = 3,        // Распознавание паттернов
    Memory = 4,         // Хранение и воспоминания
    Ethics = 5,         // Этическая оценка
    Intuition = 6,      // Интуитивные выводы
    Creativity = 7,      // Творческая генерация
    Integration = 8,     // Интеграция результатов
    Maya = 10           // Домен-интерфейс, проекция
}
```

---

## 5. Физика поля

### 5.1 Гравитация
```rust
fn apply_gravity(token: &mut Token, config: &DomainConfig) {
    let distance_to_anchor = token.position.norm();
    let gravity_force = config.gravity_strength / (distance_to_anchor + 1.0);
    token.velocity -= gravity_force * token.position.normalize();
}
```

### 5.2 Резонанс
```rust
fn apply_resonance(token_a: &mut Token, token_b: &Token, config: &DomainConfig) {
    let freq_diff = (token_a.resonance - token_b.resonance).abs();
    if freq_diff < config.resonance_freq * 0.1 {
        // Резонансное взаимодействие
        exchange_momentum(token_a, token_b);
    }
}
```

### 5.3 Термодинамика
```rust
fn apply_thermodynamics(token: &mut Token, config: &DomainConfig) {
    // Обмен температурой с полем
    let temp_diff = config.temperature - token.temperature;
    token.temperature += temp_diff * 0.01; // Медленная адаптация
    
    // Влияние на массу через температуру
    if token.temperature > 200 {
        token.mass = (token.mass * 0.99).max(1); // Испарение
    }
}
```

---

## 6. Мембранные фильтры

### 6.1 Входной фильтр
```rust
fn can_enter_domain(token: &Token, config: &DomainConfig) -> bool {
    token.mass >= config.threshold_mass
    && token.temperature <= config.threshold_temp
    && matches_pattern(&token.sutra_id, &config.input_filter)
}
```

### 6.2 Выходной фильтр
```rust
fn can_exit_domain(token: &Token, config: &DomainConfig) -> bool {
    token.state != TokenState::Locked
    && config.membrane_state != MembraneState::Closed
    && matches_pattern(&token.sutra_id, &config.output_filter)
}
```

---

## 7. Инварианты

1. **Изоляция**: Token не может существовать вне домена
2. **Суверенитет**: Каждый домен имеет уникальную физику
3. **Консервация**: Общая энергия домена сохраняется
4. **Детерминизм**: Одинаковые входы → одинаковые выходы
5. **COM синхронизация**: Все изменения через event_id

---

## 8. Жизненный цикл

1. **Создание**: Domain создается с уникальной конфигурацией
2. **Инициализация**: Поле заполняется начальными Token
3. **Обработка**: Принимает входящие паттерны через мембрану
4. **Вычисление**: Внутренняя динамика обрабатывает паттерны
5. **Проекция**: Результаты выходят через мембрану
6. **Заморозка**: Состояние сохраняется для следующего уровня

---

## 9. Взаимодействия

### 9.1 С Token
- Token.position ограничена field_size
- Token.velocity модифицируется физикой поля
- Token.temperature адаптируется к температуре домена

### 9.2 С Connection
- Connection.stress влияет на энергию домена
- Connection.gates фильтруются через мембрану
- Connection.ideal_dist зависит от физики поля

### 9.3 С COM
- Domain.created_at/last_update хранят event_id
- Все изменения конфигурации генерируют COM события
- Обработка паттернов создает последовательность событий

---

## 10. Валидация

```rust
fn validate_domain_config(config: &DomainConfig) -> bool {
    config.domain_id > 0
    && config.field_size.iter().all(|&s| s > 0.0)
    && config.gravity_strength >= 0.0
    && config.permeability >= 0.0 && config.permeability <= 1.0
    && config.created_at > 0
}
```

---

## 11. Оптимизации

1. **Spatial indexing**: Октree для быстрого поиска соседей
2. **Batch processing**: Группировка обновлений токенов
3. **Level-of-detail**: Упрощенная физика для далеких токенов
4. **Caching**: Кэширование резонансных пар

---

## 12. История изменений

- **V1.3**: Каноническая спецификация с COM интеграцией
- **V1.2**: Предыдущая версия с экспериментальной физикой
- **V1.x**: Ранние версии без детальной структуры
