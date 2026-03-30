# DomainConfig — Руководство разработчика

**Версия:** V2.1 (Cognitive Depth)
**Обновлено:** 2026-03-30

---

## Обзор

`DomainConfig` — 128-байтная конфигурация домена. Размер зафиксирован compile-time ассертом.

**Ключевые свойства:**
- Ровно 128 байт, выравнивание 128 байт (`#[repr(C, align(128))]`)
- Помещается в одну кэш-линию L1
- Квантованные значения вместо f32 там, где возможно
- Compile-time проверка: `const _: () = assert!(size_of::<DomainConfig>() == 128);`

---

## Структура памяти (128 байт)

```
Offset | Size | Секция                | Поля
-------|------|----------------------|--------------------------------------
0      | 16   | ИДЕНТИФИКАЦИЯ        | reserved_id, domain_id, parent_domain_id,
       |      |                      | domain_type, structural_role, generation, flags
16     | 32   | ФИЗИКА ПОЛЯ          | field_size[3], gravity_strength, temperature,
       |      |                      | time_dilation, resonance_freq, pressure,
       |      |                      | rebuild_frequency, friction_coeff, viscosity,
       |      |                      | elasticity, quantum_noise
48     | 16   | СЕМАНТИЧЕСКИЕ ОСИ    | axis_x_ref, axis_y_ref, axis_z_ref, axis_config
64     | 32   | МЕМБРАНА И ARBITER   | input_filter, output_filter,
       |      |                      | reflex_threshold, association_threshold,
       |      |                      | arbiter_flags, reflex_cooldown,
       |      |                      | max_concurrent_hints, feedback_weight_delta,
       |      |                      | max_passes, min_coherence,
       |      |                      | gate_complexity, threshold_mass,
       |      |                      | threshold_temp, permeability, membrane_state
96     | 32   | МЕТАДАННЫЕ           | created_at, last_update, token_capacity,
       |      |                      | connection_capacity, error_count,
       |      |                      | processing_state, complexity_score,
       |      |                      | performance_score, internal_dominance_factor,
       |      |                      | reserved_meta[2]
```

---

## Справочник полей

| Поле | Тип | Диапазон | Описание |
|------|-----|----------|----------|
| `domain_id` | `u16` | 1..65535 | Уникальный ID домена |
| `parent_domain_id` | `u16` | 0..65535 | ID родительского домена (0 = корень) |
| `domain_type` | `u8` | 1..6 | Тип: Logic/Dream/Math/Pattern/Memory/Interface |
| `structural_role` | `u8` | 0..10 | Роль в AshtiCore (см. `StructuralRole`) |
| `generation` | `u8` | 0..255 | Эволюционный индекс |
| `flags` | `u8` | битмаска | DOMAIN_ACTIVE/LOCKED/TEMPORARY |
| `field_size` | `[f32; 3]` | ≥0.0 | Размеры поля X,Y,Z |
| `gravity_strength` | `f32` | 0.0..MAX | Гравитация |
| `temperature` | `f32` | 0.0..1000.0 | Температура поля в Кельвинах |
| `time_dilation` | `u16` | 0..65535 | Замедление времени ×100 (100 = 1.0x) |
| `resonance_freq` | `u16` | 0..65535 | Базовая частота (Hz) |
| `pressure` | `u16` | 0..65535 | Давление (Pa) |
| `rebuild_frequency` | `u16` | 0..65535 | Частота перестройки spatial grid (событий) |
| `friction_coeff` | `u8` | 0..255 | Трение (→ 0.0..1.0) |
| `viscosity` | `u8` | 0..255 | Вязкость (→ 0.0..1.0) |
| `elasticity` | `u8` | 0..255 | Упругость (→ 0.0..1.0) |
| `quantum_noise` | `u8` | 0..255 | Квантовый шум (→ 0.0..1.0) |
| `input_filter` | `u64` | любой | 64-bit Bloom-фильтр входа |
| `output_filter` | `u64` | любой | 64-bit Bloom-фильтр выхода |
| `reflex_threshold` | `u8` | 0..255 | Порог рефлекса (→ 0.0..1.0) |
| `association_threshold` | `u8` | 0..255 | Порог ассоциации |
| `arbiter_flags` | `u8` | битмаска | Поведение Arbiter (см. константы) |
| `reflex_cooldown` | `u8` | 0..255 | Мин. интервал между рефлексами (пульсы) |
| `max_concurrent_hints` | `u8` | 0..255 | Макс. одновременных ассоциаций-подсказок |
| `feedback_weight_delta` | `u8` | 0..255 | Шаг изменения weight при обратной связи |
| `max_passes` | `u8` | 0..255 | Макс. проходов multi-pass (0 = выкл., рек. 3) |
| `min_coherence` | `u8` | 0..255 | Мин. coherence для повторного прохода (153 ≈ 0.6) |
| `gate_complexity` | `u16` | 0..65535 | Вычислительная сложность шлюзов |
| `threshold_mass` | `u16` | 0..65535 | Порог массы для прохода мембраны |
| `threshold_temp` | `u16` | 0..65535 | Порог температуры для прохода мембраны |
| `permeability` | `u8` | 0..255 | Проницаемость (→ 0.0..1.0) |
| `membrane_state` | `u8` | 0..3 | OPEN/SEMI/CLOSED/ADAPTIVE |
| `token_capacity` | `u32` | 1..MAX | Ёмкость токенов |
| `connection_capacity` | `u32` | 1..MAX | Ёмкость связей |
| `error_count` | `u16` | 0..65535 | Счётчик когнитивных ошибок |
| `processing_state` | `u8` | 1..3 | IDLE/ACTIVE/FROZEN |
| `complexity_score` | `u8` | 0..255 | Оценка сложности |
| `performance_score` | `u8` | 0..255 | Производительность |
| `internal_dominance_factor` | `u8` | 0..255 | Доминирование внутренних импульсов (128 = равновесие) |
| `reserved_meta` | `[u8; 2]` | — | Резерв до границы 128 байт |

---

## Константы

### MembraneState

```rust
pub const MEMBRANE_OPEN: u8 = 0;     // Все токены проходят
pub const MEMBRANE_SEMI: u8 = 1;     // Фильтрация по порогу массы
pub const MEMBRANE_CLOSED: u8 = 2;   // Только системные токены
pub const MEMBRANE_ADAPTIVE: u8 = 3; // Меняется по контексту
```

### ProcessingState

```rust
pub const PROCESSING_IDLE: u8 = 1;
pub const PROCESSING_ACTIVE: u8 = 2;
pub const PROCESSING_FROZEN: u8 = 3;
```

### DomainFlags

```rust
pub const DOMAIN_ACTIVE: u32 = 1;
pub const DOMAIN_LOCKED: u32 = 2;
pub const DOMAIN_TEMPORARY: u32 = 3;
```

### ArbiterFlags

```rust
pub const GUARDIAN_CHECK_REQUIRED: u8 = 0x04;
```

---

## Перечисления

### StructuralRole

```rust
pub enum StructuralRole {
    Sutra = 0,      // Источник истины
    Execution = 1,  // Реализация решений
    Shadow = 2,     // Симуляция и предсказание
    Codex = 3,      // Конституция и правила
    Map = 4,        // Карта мира и фактов
    Probe = 5,      // Исследование и анализ
    Logic = 6,      // Чистое вычисление
    Dream = 7,      // Фоновая оптимизация
    Void = 8,       // Аннигиляция и трансформация
    Experience = 9, // Ассоциативная память
    Maya = 10,      // Консолидация результатов
}
```

### DomainType

```rust
pub enum DomainType {
    Logic = 1, Dream = 2, Math = 3,
    Pattern = 4, Memory = 5, Interface = 6,
}
```

---

## Фабричные методы

Каждый метод создаёт конфиг с физикой, подходящей для роли домена.

| Метод | Роль | Физика |
|-------|------|--------|
| `factory_sutra(id)` | Sutra (0) | Абсолютный ноль, колоссальная гравитация, CLOSED |
| `factory_execution(id, parent)` | Execution (1) | 310K, g=9.81, низкое трение, SEMI |
| `factory_shadow(id, parent)` | Shadow (2) | 250K, высокая вязкость, CLOSED |
| `factory_codex(id, parent)` | Codex (3) | 10K, g=1000, вязкость≈0.98, CLOSED |
| `factory_map(id, parent)` | Map (4) | 280K, g=15, CLOSED |
| `factory_probe(id, parent)` | Probe (5) | 350K, высокий резонанс, OPEN |
| `factory_logic(id, parent)` | Logic (6) | 273K, g=9.81, ADAPTIVE |
| `factory_dream(id, parent)` | Dream (7) | 500K, g=0, quantum_noise=200, OPEN |
| `factory_void(id, parent)` | Void (8) | 1000K, g=100, OPEN |
| `factory_experience(id, parent)` | Experience (9) | 300K, g=0.5, поле 5000³, SEMI |
| `factory_maya(id, parent)` | Maya (10) | 310K, max_passes=3, min_coherence=153, OPEN |

`factory_maya` — единственный домен с ненулевым `max_passes` и `min_coherence` (Cognitive Depth V1.0).

---

## Cognitive Depth V1.0 — новые поля

Добавлены в этапе 13 (Cognitive Depth V1.0):

### max_passes + min_coherence (13A — Multi-pass)

```rust
// В factory_maya:
config.max_passes = 3;      // до 3 повторных проходов
config.min_coherence = 153; // 153/255 ≈ 0.6 — порог повторного прохода
```

Arbiter использует `maya_multipass_params()` для чтения этих значений и повторяет
ASHTI-цикл, пока `confidence < min_coherence && passes < max_passes`.

### internal_dominance_factor (13C — Internal Dominance)

```rust
// В metadata секции:
config.internal_dominance_factor = 128; // 128 = равновесие (0..255 → 0.0..2.0)
```

| Значение | Поведение |
|----------|-----------|
| 0 | Чисто реактивный — только внешние сигналы |
| 128 | Равновесие — внешнее и внутреннее на равных |
| 255 | Задумчивый — внутренние импульсы доминируют |

---

## Создание и использование

```rust
use axiom_config::{DomainConfig, DomainType, StructuralRole};

// Из фабрики (рекомендуется)
let maya = DomainConfig::factory_maya(110, 100);

// Из YAML файла
let config = DomainConfig::from_yaml(Path::new("config/presets/domains/maya.yaml"))?;

// Вручную
let mut config = DomainConfig::default_void();
config.domain_id = 42;
config.structural_role = StructuralRole::Logic as u8;

// Проверка мембраны
if config.can_enter(mass, temperature) { /* ... */ }

// Валидация (возвращает Result<(), String>)
config.validate()?;
```

---

## YAML-конфиги

Пресеты находятся в `config/presets/domains/*.yaml`.
Схема соответствует полям структуры `DomainConfig` (serde).

Пример `maya.yaml`:
```yaml
max_passes: 3
min_coherence: 153
internal_dominance_factor: 0
reserved_meta: [0, 0]
# ... остальные поля
```

---

## Реализация

- `crates/axiom-config/src/domain_config.rs` — структура, фабрики, методы
- `config/presets/domains/*.yaml` — 11 пресетов
- `crates/axiom-config/tests/` — тесты конфигурации
