# AXIOM MODULE SPECIFICATION: UPO V2.2

**Статус:** Актуальная спецификация (core)  
**Версия:** 2.2.0  
**Дата:** 2026-03-04  
**Название:** Universal Phase Operator  
**Формат:** 32 байта DynamicTrace, `repr(C, align(32))`  
**Модель времени:** COM `event_id` (причинный порядок, u64)  
**Связанные спеки:** COM V1.0, Token V5.1, Connection V5.0, Domain V1.3

---

## 1. Назначение

**UPO (Universal Phase Operator)** — модуль наблюдения за динамикой в одном домене Axiom, преобразующий движения Token и напряжение Connection в точки на 3D экране:

- наблюдает за Token и Connection в домене,
- вычисляет динамические характеристики (движение, стресс, резонанс),
- проецирует динамику в точки на экране с весом,
- организует затухание следов по COM event_id,
- обеспечивает вечную память через минимальный предел интенсивности.

UPO **не интерпретирует семантику** и **не ищет паттерны** — это задача других модулей. UPO только преобразует динамику в визуализацию.

---

## 2. Архитектура

```
Token'ы и Connection'ы в домене D
    ↓ (чтение полей)
[UPO] — вычисляет метрики и сворачивает в (x, y, z, weight) + event_id
    ↓ (запись)
Экран — 3D пространство, затухание по event_id, min_intensity > 0
    ↓ (чтение и анализ)
IntuitionEngine и другие модули — ищут паттерны, интерпретируют
    ↓ (опционально)
Заморозка экрана → библиотека следующего уровня
```

---

## 3. Структура DynamicTrace (32 байта)

```rust
#[repr(C, align(32))]
pub struct DynamicTrace {
    // --- ПРОСТРАНСТВО (12 Байт) ---
    pub x: i32,                 // Координата X на экране
    pub y: i32,                 // Координата Y на экране  
    pub z: i32,                 // Координата Z на экране

    // --- ХАРАКТЕРИСТИКИ (8 Байт) ---
    pub weight: f32,            // Вес/интенсивность точки
    pub frequency: f32,         // Частота колебаний

    // --- ВРЕМЯ (8 Байт) ---
    pub created_at: u64,        // COM event_id создания следа
    pub last_update: u64,       // COM event_id последнего обновления

    // --- МЕТАДАННЫЕ (4 Байт) ---
    pub source_type: u8,        // Источник (Token/Connection/Field)
    pub source_id: u32,         // ID источника
    pub flags: u8,              // ACTIVE/FADING/LOCKED
    pub resonance_class: u8,    // Класс резонанса
}
```

---

## 4. Структура Screen

```rust
#[repr(C)]
pub struct Screen {
    // --- ПАРАМЕТРЫ (32 Байта) ---
    pub size: [i32; 3],        // Размеры экрана (X, Y, Z)
    pub resolution: f32,        // Разрешение (единица на пиксель)
    pub min_intensity: f32,     // Минимальная интенсивность (> 0)
    pub decay_rate: f32,        // Скорость затухания
    pub current_event_id: u64,  // Текущий COM event_id
    pub trace_count: u32,       // Количество следов
    pub total_energy: f32,       // Общая энергия экрана
    pub octant_mask: u8,        // Маска активных октантов

    // --- ДАННЫЕ (динамические) ---
    pub traces: Vec<DynamicTrace>, // Массив следов
    pub octants: [OctantStats; 8], // Статистика по октантам
}
```

---

## 5. Вычисление метрик

### 5.1 Token динамика
```rust
fn compute_token_trace(token: &Token, prev_token: &Token) -> DynamicTrace {
    let velocity_magnitude = token.velocity.iter().map(|v| v.abs() as f32).sum();
    let position_change = [
        token.position[0] - prev_token[0],
        token.position[1] - prev_token[1], 
        token.position[2] - prev_position[2],
    ];
    
    DynamicTrace {
        x: position_change[0] as i32,
        y: position_change[1] as i32,
        z: position_change[2] as i32,
        weight: velocity_magnitude * token.mass as f32,
        frequency: token.resonance as f32,
        created_at: token.last_event_id,
        last_update: token.last_event_id,
        source_type: SourceType::Token,
        source_id: token.sutra_id,
        flags: TraceFlags::ACTIVE,
        resonance_class: compute_resonance_class(token.resonance),
    }
}
```

### 5.2 Connection стресс
```rust
fn compute_connection_trace(conn: &Connection) -> DynamicTrace {
    let stress_factor = conn.current_stress / conn.strength;
    let midpoint = compute_midpoint(conn.source_id, conn.target_id);
    
    DynamicTrace {
        x: midpoint.x as i32,
        y: midpoint.y as i32,
        z: midpoint.z as i32,
        weight: stress_factor * conn.elasticity,
        frequency: 1.0 / (conn.ideal_dist + 1.0),
        created_at: conn.created_at,
        last_update: get_current_event_id(),
        source_type: SourceType::Connection,
        source_id: conn.source_id,
        flags: if stress_factor > 0.8 { TraceFlags::CRITICAL } else { TraceFlags::ACTIVE },
        resonance_class: compute_link_resonance(conn.link_type),
    }
}
```

---

## 6. Затухание и память

### 6.1 Функция затухания
```rust
fn apply_decay(trace: &mut DynamicTrace, current_event_id: u64, screen: &Screen) {
    let event_age = current_event_id - trace.last_update;
    let decay_factor = (-event_age as f32 * screen.decay_rate).exp();
    
    trace.weight = (trace.weight * decay_factor).max(screen.min_intensity);
    
    if trace.weight <= screen.min_intensity * 1.1 {
        trace.flags |= TraceFlags::FADING;
    }
}
```

### 6.2 Вечная память
```rust
fn ensure_eternal_memory(screen: &mut Screen) {
    for trace in &mut screen.traces {
        if trace.weight < screen.min_intensity {
            trace.weight = screen.min_intensity;
            trace.flags |= TraceFlags::ETERNAL;
        }
    }
}
```

---

## 7. Октанты экрана

Экран разделен на 8 октантов по знакам координат:

```
     Z+
    /|
   / | Y+
  /__|
 X-   X+
```

```rust
#[repr(u8)]
pub enum Octant {
    +++ = 0,  // X+, Y+, Z+
    -++ = 1,  // X-, Y+, Z+
    +-+ = 2,  // X+, Y-, Z+
    --+ = 3,  // X-, Y-, Z+
    ++- = 4,  // X+, Y+, Z-
    -+- = 5,  // X-, Y+, Z-
    +-- = 6,  // X+, Y-, Z-
    --- = 7,  // X-, Y-, Z-
}
```

---

## 8. Инварианты

1. **Минимальная интенсивность**: `weight >= min_intensity > 0`
2. **COM синхронизация**: `created_at <= last_update <= current_event_id`
3. **Координаты**: Следы в пределах размеров экрана
4. **Детерминизм**: Одинаковые входы → одинаковые следы
5. **Сохранение энергии**: Общая энергия не возрастает без источника

---

## 9. Жизненный цикл

1. **Наблюдение**: Чтение Token и Connection из домена
2. **Вычисление**: Расчет метрик динамики
3. **Проекция**: Создание DynamicTrace на экране
4. **Затухание**: Применение decay по event_id
5. **Накопление**: Следы накапливаются, но не исчезают
6. **Анализ**: Другие модули анализируют паттерны
7. **Заморозка**: Экран сохраняется для следующего уровня

---

## 10. Взаимодействия

### 10.1 С Token
- Читает position, velocity, momentum
- Использует last_event_id для синхронизации
- Учитывает mass, temperature, resonance

### 10.2 С Connection  
- Читает stress, strength, elasticity
- Вычисляет midpoint для проекции
- Анализирует link_type и gates

### 10.3 С Domain
- Следует за physics поля
- Учитывает field_size для проекции
- Использует domain_id для изоляции

### 10.4 С COM
- Синхронизируется через event_id
- Генерирует события для критических изменений
- Поддерживает timeline для затухания

---

## 11. Валидация

```rust
fn validate_trace(trace: &DynamicTrace, screen: &Screen) -> bool {
    trace.weight >= screen.min_intensity
    && trace.created_at > 0
    && trace.last_update >= trace.created_at
    && trace.x >= -screen.size[0]/2 && trace.x <= screen.size[0]/2
    && trace.y >= -screen.size[1]/2 && trace.y <= screen.size[1]/2
    && trace.z >= -screen.size[2]/2 && trace.z <= screen.size[2]/2
}
```

---

## 12. Оптимизации

1. **Spatial indexing**: Октree для быстрого поиска следов
2. **Level-of-detail**: Упрощенная физика для далеких следов
3. **Batch updates**: Группировка обновлений по event_id
4. **Compression**: Сжатие старых следов с min_intensity

---

## 13. История изменений

- **V2.2**: Каноническая спецификация с COM V1.0 интеграцией
- **V2.1**: Предыдущая версия с устаревшими структурами
- **V2.0**: Базовая реализация с DynamicTrace
- **V1.x**: Ранние концептуальные версии
