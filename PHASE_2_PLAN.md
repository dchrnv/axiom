# Phase 2: Shell V3.0 - Детальный план реализации

**Версия:** 1.0
**Дата:** 2026-03-21
**Статус:** Подготовка к реализации

---

## 🎯 Цель Phase 2

Реализовать Shell V3.0 — легкий семантический кэш (8 байт на токен), вычисляемый из связей токена. Shell описывает "чем является сущность" в восьми ортогональных измерениях восприятия.

---

## 📋 Phase 2.1: Базовые структуры

**Файл:** `runtime/src/shell.rs` (новый модуль)

### Структуры данных:

```rust
// Семантический профиль токена (8 слоев × u8)
pub type ShellProfile = [u8; 8];  // L1..L8

// Вклад типа связи в семантические слои
pub type ShellContribution = [u8; 8];

// Кэш Shell для домена
pub struct DomainShellCache {
    pub profiles: Vec<ShellProfile>,    // profiles[token_index]
    pub dirty_flags: BitVec,            // Токены, требующие пересчёта
    pub generation: u64,                // Счётчик reconciliation
}
```

### Константы и пресеты:

```rust
// Нулевой профиль (токен без связей)
pub const EMPTY_SHELL: ShellProfile = [0, 0, 0, 0, 0, 0, 0, 0];

// Имена слоёв для отладки
pub const LAYER_NAMES: [&str; 8] = [
    "Physical", "Sensory", "Motor", "Emotional",
    "Cognitive", "Social", "Temporal", "Abstract"
];
```

### Методы DomainShellCache:

```rust
impl DomainShellCache {
    pub fn new(capacity: usize) -> Self;
    pub fn get(&self, token_index: usize) -> &ShellProfile;
    pub fn set(&mut self, token_index: usize, profile: ShellProfile);
    pub fn mark_dirty(&mut self, token_index: usize);
    pub fn is_dirty(&self, token_index: usize) -> bool;
    pub fn clear_dirty(&mut self, token_index: usize);
    pub fn clear_all_dirty(&mut self);
}
```

### Тесты (минимум 5):

1. `test_shell_profile_size` - ShellProfile = 8 байт
2. `test_domain_shell_cache_new` - создание кэша
3. `test_domain_shell_cache_get_set` - чтение/запись профилей
4. `test_domain_shell_cache_dirty_flags` - mark/is/clear dirty
5. `test_empty_shell_constant` - нулевой профиль

### Зависимости:

- `bitvec` crate для BitVec (уже в Cargo.toml?)

---

## 📋 Phase 2.2: Справочник семантических вкладов

**Файл:** `runtime/src/shell.rs` (расширение)

### Структура:

```rust
pub struct SemanticContributionTable {
    categories: [ShellContribution; 256],        // Базовые профили категорий
    overrides: HashMap<u16, ShellContribution>,  // Конкретные переопределения
}

impl SemanticContributionTable {
    pub fn new() -> Self;
    pub fn get(&self, link_type: u16) -> &ShellContribution;
    pub fn set_category(&mut self, category: u8, contribution: ShellContribution);
    pub fn set_override(&mut self, link_type: u16, contribution: ShellContribution);
}
```

### Пресеты (hardcoded на старте):

```rust
impl SemanticContributionTable {
    pub fn default_ashti_core() -> Self {
        // 7 категорий из спецификации:
        // 0x01: Structural [20, 5, 0, 0, 5, 0, 0, 0]
        // 0x02: Semantic   [0, 0, 0, 0, 15, 0, 0, 10]
        // 0x03: Causal     [0, 0, 5, 0, 15, 0, 10, 8]
        // 0x04: Experiential [5, 20, 0, 15, 0, 0, 0, 0]
        // 0x05: Social     [0, 0, 0, 5, 0, 25, 0, 0]
        // 0x06: Temporal   [0, 0, 0, 0, 5, 0, 25, 0]
        // 0x07: Motor      [10, 0, 25, 0, 5, 0, 0, 0]
    }
}
```

### Тесты (минимум 5):

1. `test_semantic_contribution_table_new` - создание таблицы
2. `test_category_lookup` - get() для категории
3. `test_override_lookup` - get() для переопределения
4. `test_default_ashti_core` - пресет загружается
5. `test_two_level_hierarchy` - category vs override приоритет

---

## 📋 Phase 2.3: YAML конфигурация

**Статус:** ⏸️ ОТЛОЖЕНО (как Phase 1.10)

**Причина:** ConfigLoader ещё не реализован (см. DEFERRED.md 3.2)

**Временное решение:** Hardcoded пресет `default_ashti_core()`

**Требуется позже:**
- Schema: `config/schema/semantic_contributions.yaml`
- Загрузка через ConfigLoader
- Валидация

---

## 📋 Phase 2.4: Алгоритм вычисления Shell

**Файл:** `runtime/src/shell.rs` (расширение)

### Функция compute_shell:

```rust
pub fn compute_shell(
    token: &Token,
    connections: &[Connection],
    contribution_table: &SemanticContributionTable,
) -> ShellProfile {
    // 1. Найти связи где source_id или target_id = token.sutra_id
    // 2. Accumulator [f32; 8] = [0.0; 8]
    // 3. Для каждой связи:
    //    contribution = table.get(conn.link_type)
    //    weight = conn.strength
    //    accumulator[i] += contribution[i] * weight
    // 4. Нормализация:
    //    max_value = accumulator.max()
    //    if max_value > 255: factor = 255.0 / max_value
    //    accumulator *= factor
    // 5. Округление до u8
}
```

### Тесты (минимум 8):

1. `test_compute_shell_no_connections` - токен без связей = EMPTY_SHELL
2. `test_compute_shell_single_connection` - одна связь
3. `test_compute_shell_multiple_connections` - несколько связей
4. `test_compute_shell_normalization` - переполнение > 255
5. `test_compute_shell_strength_weighting` - strength влияет на вклад
6. `test_compute_shell_proportions_preserved` - нормализация сохраняет пропорции
7. `test_compute_shell_different_categories` - разные категории
8. `test_compute_shell_with_overrides` - переопределения работают

---

## 📋 Phase 2.5: Инкрементальное обновление

**Файл:** `runtime/src/shell.rs` (расширение)

### Механизм dirty tracking:

```rust
impl DomainShellCache {
    // Пометить токен для пересчёта
    pub fn mark_token_dirty(&mut self, token_index: usize);

    // Пересчитать Shell для всех dirty токенов
    pub fn recompute_dirty(
        &mut self,
        tokens: &[Token],
        connections: &[Connection],
        contribution_table: &SemanticContributionTable,
    ) -> usize;  // Возвращает количество пересчитанных
}
```

### Тесты (минимум 5):

1. `test_incremental_mark_dirty` - mark_dirty работает
2. `test_incremental_recompute_single` - пересчёт одного токена
3. `test_incremental_recompute_multiple` - пересчёт нескольких
4. `test_incremental_recompute_empty` - нет dirty = 0 пересчётов
5. `test_incremental_clear_after_recompute` - dirty flags сбрасываются

---

## 📋 Phase 2.6: Интеграция с Causal Frontier

**Файл:** `runtime/src/domain.rs` (модификация)

### Расширение Domain:

```rust
pub struct Domain {
    pub shell_cache: DomainShellCache,  // НОВОЕ ПОЛЕ
    // ...existing fields
}
```

### Расширение process_frontier:

```rust
// В Domain::process_frontier() после обработки Connection событий:
if event.event_type == EventType::ConnectionCreated
    || event.event_type == EventType::ConnectionDeleted
    || event.event_type == EventType::ConnectionStrengthChanged {

    // Получить source_index и target_index
    self.shell_cache.mark_dirty(source_index);
    self.shell_cache.mark_dirty(target_index);
}
```

### Тесты (минимум 5):

1. `test_domain_has_shell_cache` - Domain содержит shell_cache
2. `test_connection_created_marks_dirty` - ConnectionCreated → dirty
3. `test_connection_deleted_marks_dirty` - ConnectionDeleted → dirty
4. `test_frontier_updates_shell` - frontier обработка → Shell обновлён
5. `test_shell_no_com_events` - Shell не генерирует COM события

---

## 📋 Phase 2.7: Reconciliation через Heartbeat

**Файл:** `runtime/src/heartbeat.rs` (модификация)

### Расширение HeartbeatConfig:

```rust
pub struct HeartbeatConfig {
    pub enable_shell_reconciliation: bool,  // НОВОЕ ПОЛЕ
    // ...existing fields
}
```

### Reconciliation в heartbeat батче:

```rust
// В Domain::process_frontier() при обработке HeartbeatTrigger:
if self.heartbeat_config.enable_shell_reconciliation {
    // Пересчитать Shell для токена
    let old_shell = self.shell_cache.get(token_index);
    let new_shell = compute_shell(...);

    if old_shell != &new_shell {
        self.shell_cache.set(token_index, new_shell);
        // Опционально: логировать drift
    }
}
```

### Тесты (минимум 4):

1. `test_heartbeat_has_shell_reconciliation_flag` - флаг существует
2. `test_heartbeat_shell_reconciliation_enabled` - reconciliation работает
3. `test_heartbeat_shell_reconciliation_disabled` - не работает когда выключен
4. `test_heartbeat_detects_shell_drift` - обнаружение рассинхронизации

---

## 📋 Phase 2.8: Интеграция с Domain (финализация)

**Файл:** `runtime/src/domain.rs` (расширение)

### Методы Domain:

```rust
impl Domain {
    // Получить Shell токена по индексу
    pub fn get_shell(&self, token_index: usize) -> &ShellProfile;

    // Первичное вычисление Shell для всех токенов
    pub fn rebuild_all_shells(&mut self, contribution_table: &SemanticContributionTable);
}
```

### Тесты (минимум 3):

1. `test_domain_get_shell` - get_shell работает
2. `test_domain_rebuild_all_shells` - первичное вычисление
3. `test_domain_shell_persistence` - Shell сохраняется между операциями

---

## 📋 Phase 2.9: Runtime конфигурация

**Статус:** ⏸️ ОТЛОЖЕНО (как Phase 1.10, Phase 2.3)

**Временное решение:** Hardcoded defaults в HeartbeatConfig

```rust
enable_shell_reconciliation: true
```

---

## 📋 Phase 2.10: Финальная валидация

**Файл:** `runtime/src/shell.rs` (тесты)

### Cross-spec validation тесты:

```rust
#[test]
fn test_shell_profile_size_validation() {
    assert_eq!(size_of::<ShellProfile>(), 8);
}

#[test]
fn test_shell_no_com_events_validation() {
    // Shell не должен генерировать COM события
}

#[test]
fn test_shell_domain_locality_validation() {
    // Shell локален для домена
}

#[test]
fn test_shell_determinism_validation() {
    // compute_shell детерминистичен
}
```

### Документация:

- Комментарии в коде runtime/src/shell.rs
- Примеры использования

---

## 📊 Прогресс Phase 2

| Задача | Файлы | Тесты | Статус |
|--------|-------|-------|--------|
| 2.1 Базовые структуры | shell.rs | 9 | ✅ Complete |
| 2.2 Справочник | shell.rs | 5+ | ⏸️ Pending |
| 2.3 YAML конфигурация | - | - | ⏸️ Deferred |
| 2.4 Алгоритм compute | shell.rs | 8+ | ⏸️ Pending |
| 2.5 Инкрементальное | shell.rs | 5+ | ⏸️ Pending |
| 2.6 Frontier интеграция | domain.rs | 5+ | ⏸️ Pending |
| 2.7 Heartbeat reconciliation | heartbeat.rs | 4+ | ⏸️ Pending |
| 2.8 Domain финализация | domain.rs | 3+ | ⏸️ Pending |
| 2.9 Runtime config | - | - | ⏸️ Deferred |
| 2.10 Валидация | shell.rs | 4+ | ⏸️ Pending |

**Итого:** 1/10 завершено (10%), 9 тестов pass (+9 новых, всего 294)

---

## 🎯 Следующий шаг

**Готово к реализации:** Phase 2.1 - Базовые структуры

**Команда:** "начнём с Phase 2.1" или "продолжим?"

---

**Версия:** 1.0
**Последнее обновление:** 2026-03-21
