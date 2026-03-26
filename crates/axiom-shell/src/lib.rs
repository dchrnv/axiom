// Shell V3.0 - Семантический профиль токена
//
// Shell описывает "чем является сущность" в восьми ортогональных измерениях восприятия.
// Shell не хранится внутри Token (Token V5.2 остаётся 64 байта).
// Shell — внешний кэш, вычисляемый из совокупности активных Connection токена.
//
// Спецификация: docs/spec/Shell_V3_0.md

use bitvec::prelude::*;

/// Семантический профиль токена (8 слоев × u8)
///
/// Восемь слоёв описывают восемь ортогональных измерений восприятия:
/// - L1: Physical (материальность)
/// - L2: Sensory (ощущения)
/// - L3: Motor (действие, движение)
/// - L4: Emotional (эмоции)
/// - L5: Cognitive (мышление, знание)
/// - L6: Social (отношения, роли)
/// - L7: Temporal (время, ритм)
/// - L8: Abstract (абстракция, символы)
///
/// Каждый слой: u8 (0..255), где 0 = отсутствие, 255 = максимум
pub type ShellProfile = [u8; 8];

/// Вклад типа связи в семантические слои
pub type ShellContribution = [u8; 8];

/// Нулевой профиль (токен без связей)
pub const EMPTY_SHELL: ShellProfile = [0, 0, 0, 0, 0, 0, 0, 0];

/// Имена слоёв для отладки
pub const LAYER_NAMES: [&str; 8] = [
    "Physical",   // L1
    "Sensory",    // L2
    "Motor",      // L3
    "Emotional",  // L4
    "Cognitive",  // L5
    "Social",     // L6
    "Temporal",   // L7
    "Abstract",   // L8
];

/// Кэш Shell для домена
///
/// Хранится параллельно массиву токенов домена.
/// Индексация: profiles[token_index] соответствует domain_state.tokens[token_index]
#[derive(Debug)]
pub struct DomainShellCache {
    /// Семантические профили токенов (indexed by token_index)
    pub profiles: Vec<ShellProfile>,

    /// Флаги "грязных" токенов, требующих пересчёта Shell
    pub dirty_flags: BitVec,

    /// Монотонный счётчик reconciliation (увеличивается при каждом batch reconciliation)
    pub generation: u64,
}

impl DomainShellCache {
    /// Создать новый кэш с заданной ёмкостью
    ///
    /// Все профили инициализируются EMPTY_SHELL, все флаги = false
    pub fn new(capacity: usize) -> Self {
        Self {
            profiles: vec![EMPTY_SHELL; capacity],
            dirty_flags: bitvec![0; capacity],
            generation: 0,
        }
    }

    /// Получить Shell профиль токена по индексу
    ///
    /// # Panics
    /// Паникует если token_index >= capacity
    pub fn get(&self, token_index: usize) -> &ShellProfile {
        &self.profiles[token_index]
    }

    /// Установить Shell профиль токена
    ///
    /// # Panics
    /// Паникует если token_index >= capacity
    pub fn set(&mut self, token_index: usize, profile: ShellProfile) {
        self.profiles[token_index] = profile;
    }

    /// Пометить токен как требующий пересчёта Shell
    ///
    /// # Panics
    /// Паникует если token_index >= capacity
    pub fn mark_dirty(&mut self, token_index: usize) {
        self.dirty_flags.set(token_index, true);
    }

    /// Проверить, помечен ли токен как "грязный"
    ///
    /// # Panics
    /// Паникует если token_index >= capacity
    pub fn is_dirty(&self, token_index: usize) -> bool {
        self.dirty_flags[token_index]
    }

    /// Снять флаг "грязный" с токена
    ///
    /// # Panics
    /// Паникует если token_index >= capacity
    pub fn clear_dirty(&mut self, token_index: usize) {
        self.dirty_flags.set(token_index, false);
    }

    /// Снять все флаги "грязный"
    pub fn clear_all_dirty(&mut self) {
        self.dirty_flags.fill(false);
    }

    /// Количество токенов в кэше
    pub fn capacity(&self) -> usize {
        self.profiles.len()
    }

    /// Получить список всех dirty токенов
    pub fn get_dirty_tokens(&self) -> Vec<usize> {
        self.dirty_flags
            .iter()
            .enumerate()
            .filter_map(|(i, is_dirty)| if *is_dirty { Some(i) } else { None })
            .collect()
    }

    /// Инкрементальное обновление: пересчитать только dirty токены
    ///
    /// # Arguments
    /// * `connections` - все связи домена
    /// * `table` - справочник семантических вкладов
    ///
    /// # Returns
    /// Количество обновлённых профилей
    pub fn update_dirty_shells(
        &mut self,
        connections: &[Connection],
        table: &SemanticContributionTable,
    ) -> usize {
        let mut updated = 0;

        for token_index in 0..self.dirty_flags.len() {
            if self.dirty_flags[token_index] {
                // Пересчитываем профиль (token_id = token_index + 1)
                let token_id = (token_index + 1) as u32;
                let new_profile = compute_shell(token_id, connections, table);

                // Обновляем кэш
                self.profiles[token_index] = new_profile;
                self.dirty_flags.set(token_index, false);
                updated += 1;
            }
        }

        if updated > 0 {
            self.generation += 1;
        }

        updated
    }
}

// ============================================================================
// SEMANTIC CONTRIBUTION TABLE
// ============================================================================

use std::collections::HashMap;

/// Справочник семантических вкладов
///
/// Определяет, какой вклад в семантические слои (L1-L8) вносит каждый тип связи.
/// Двухуровневая организация:
/// - Категория (старший байт link_type) - базовый профиль для класса связей
/// - Конкретный тип (полный link_type) - переопределение для специфичных типов
#[derive(Debug)]
pub struct SemanticContributionTable {
    /// Базовые профили для 256 категорий (indexed by category = link_type >> 8)
    categories: [ShellContribution; 256],

    /// Переопределения для конкретных типов связей (link_type → contribution)
    overrides: HashMap<u16, ShellContribution>,
}

impl SemanticContributionTable {
    /// Создать пустую таблицу (все категории = нулевой вклад)
    pub fn new() -> Self {
        Self {
            categories: [[0; 8]; 256],
            overrides: HashMap::new(),
        }
    }

    /// Получить вклад для типа связи
    ///
    /// Алгоритм:
    /// 1. Проверить переопределение для конкретного link_type
    /// 2. Если нет - вернуть базовый профиль категории (link_type >> 8)
    pub fn get(&self, link_type: u16) -> &ShellContribution {
        // Сначала проверяем переопределение
        if let Some(profile) = self.overrides.get(&link_type) {
            return profile;
        }

        // Иначе возвращаем базовый профиль категории
        let category = (link_type >> 8) as usize;
        &self.categories[category]
    }

    /// Установить базовый профиль для категории
    pub fn set_category(&mut self, category: u8, contribution: ShellContribution) {
        self.categories[category as usize] = contribution;
    }

    /// Установить переопределение для конкретного типа связи
    pub fn set_override(&mut self, link_type: u16, contribution: ShellContribution) {
        self.overrides.insert(link_type, contribution);
    }

    /// Пресет ASHTI Core - 7 базовых категорий
    ///
    /// Категории из Shell V3.0 спецификации:
    /// - 0x01: Structural (Part_Of, Contains, Member_Of...)
    /// - 0x02: Semantic (Synonym, Antonym, Hypernym...)
    /// - 0x03: Causal (Cause, Effect, Enables...)
    /// - 0x04: Experiential (Feels_Like, Tastes_Like, Sounds_Like...)
    /// - 0x05: Social (Friend_Of, Reports_To, Belongs_To_Group...)
    /// - 0x06: Temporal (Precedes, Follows, During...)
    /// - 0x07: Motor (Used_For, Performed_By, Requires_Action...)
    pub fn default_ashti_core() -> Self {
        let mut table = Self::new();

        // 0x01: Structural - высокий Physical, немного Cognitive
        table.set_category(0x01, [20, 5, 0, 0, 5, 0, 0, 0]);

        // 0x02: Semantic - высокий Cognitive, немного Abstract
        table.set_category(0x02, [0, 0, 0, 0, 15, 0, 0, 10]);

        // 0x03: Causal - Motor, Cognitive, Temporal, Abstract
        table.set_category(0x03, [0, 0, 5, 0, 15, 0, 10, 8]);

        // 0x04: Experiential - Physical, Sensory, Emotional
        table.set_category(0x04, [5, 20, 0, 15, 0, 0, 0, 0]);

        // 0x05: Social - Emotional, высокий Social
        table.set_category(0x05, [0, 0, 0, 5, 0, 25, 0, 0]);

        // 0x06: Temporal - Cognitive, высокий Temporal
        table.set_category(0x06, [0, 0, 0, 0, 5, 0, 25, 0]);

        // 0x07: Motor - Physical, высокий Motor, Cognitive
        table.set_category(0x07, [10, 0, 25, 0, 5, 0, 0, 0]);

        table
    }
}

impl Default for SemanticContributionTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SHELL COMPUTATION ALGORITHM
// ============================================================================

use axiom_core::connection::Connection;

/// Пометить токены как dirty при изменении Connection
///
/// Триггер для инкрементального обновления.
/// Вызывается когда Connection создана/удалена/изменена.
///
/// # Arguments
/// * `cache` - кэш Shell профилей
/// * `source_id` - ID source токена
/// * `target_id` - ID target токена
pub fn mark_connection_dirty(cache: &mut DomainShellCache, source_id: u32, target_id: u32) {
    // token_id = token_index + 1
    if source_id > 0 {
        let source_index = (source_id - 1) as usize;
        cache.mark_dirty(source_index);
    }

    if target_id > 0 {
        let target_index = (target_id - 1) as usize;
        cache.mark_dirty(target_index);
    }
}

/// Собрать затронутые токены из Connection событий
///
/// Frontier integration helper: при обработке Connection событий
/// собирает все токены (source + target), которые должны быть добавлены в frontier.
///
/// # Arguments
/// * `connections` - массив связей для обработки
///
/// # Returns
/// Vec с индексами токенов (token_index = token_id - 1)
pub fn collect_affected_tokens(connections: &[Connection]) -> Vec<usize> {
    let mut affected = Vec::new();

    for conn in connections {
        if conn.source_id > 0 {
            let source_idx = (conn.source_id - 1) as usize;
            if !affected.contains(&source_idx) {
                affected.push(source_idx);
            }
        }

        if conn.target_id > 0 {
            let target_idx = (conn.target_id - 1) as usize;
            if !affected.contains(&target_idx) {
                affected.push(target_idx);
            }
        }
    }

    affected
}

/// Обработать Connection событие и пометить затронутые токены как dirty
///
/// Shell V3.0 Phase 3.1: SPACE ↔ Shell Integration
/// При любом изменении Connection (create/update/delete) - помечаем
/// затронутые токены (source + target) как dirty для последующего пересчёта Shell.
///
/// # Arguments
/// * `cache` - DomainShellCache для пометки dirty flags
/// * `connection` - Connection которая изменилась
///
/// # Example
/// ```ignore
/// // После создания/обновления Connection:
/// process_connection_event(&mut domain.shell_cache, &connection);
/// // Затронутые токены будут пересчитаны в heartbeat reconciliation
/// ```
pub fn process_connection_event(cache: &mut DomainShellCache, connection: &Connection) {
    // Собираем затронутые токены (source + target)
    let affected = collect_affected_tokens(&[*connection]);

    // Помечаем каждый затронутый токен как dirty
    for token_idx in affected {
        if token_idx < cache.capacity() {
            cache.mark_dirty(token_idx);
        }
    }
}

/// Вычислить семантический профиль токена на основе его связей
///
/// Алгоритм (Shell V3.0 spec):
/// 1. Собрать все Connection где token_id == source_id ИЛИ target_id
/// 2. Для каждой связи:
///    - Получить contribution из таблицы (по link_type)
///    - Добавить contribution × strength в аккумулятор
/// 3. Нормализовать аккумулятор (max → 255)
/// 4. Округлить до [u8; 8]
///
/// # Arguments
/// * `token_id` - ID токена для которого вычисляется профиль
/// * `connections` - все связи домена
/// * `table` - справочник семантических вкладов
///
/// # Returns
/// Семантический профиль [u8; 8] (L1-L8)
pub fn compute_shell(
    token_id: u32,
    connections: &[Connection],
    table: &SemanticContributionTable,
) -> ShellProfile {
    // 1. Аккумулятор для 8 слоёв (используем f32 для точности)
    let mut acc = [0.0f32; 8];

    // 2. Проходим по всем связям
    for conn in connections {
        // Только связи где token_id участвует (source или target)
        if conn.source_id != token_id && conn.target_id != token_id {
            continue;
        }

        // Получаем вклад для типа связи
        let contribution = table.get(conn.link_type);

        // Добавляем взвешенный вклад (contribution × strength)
        for i in 0..8 {
            acc[i] += contribution[i] as f32 * conn.strength;
        }
    }

    // 3. Нормализация: находим максимум, масштабируем к [0, 255]
    let max_val = acc.iter().copied().fold(0.0f32, f32::max);

    if max_val == 0.0 {
        // Нет связей или все вклады нулевые
        return EMPTY_SHELL;
    }

    // 4. Округление до u8 с нормализацией
    let scale = 255.0 / max_val;
    let mut profile = EMPTY_SHELL;
    for i in 0..8 {
        profile[i] = (acc[i] * scale).round() as u8;
    }

    profile
}

/// Reconciliation batch: пересчёт и проверка Shell для батча токенов
///
/// Heartbeat V2.0 Phase 2.7: Shell reconciliation
/// Функция для использования в heartbeat батче.
/// Пересчитывает Shell для указанных токенов и обновляет кэш если отличается.
///
/// # Arguments
/// * `cache` - DomainShellCache для обновления
/// * `token_indices` - индексы токенов для reconciliation
/// * `connections` - все связи домена
/// * `table` - SemanticContributionTable для вычисления вкладов
///
/// # Returns
/// Количество токенов, профиль которых изменился (drift detected)
pub fn reconcile_shell_batch(
    cache: &mut DomainShellCache,
    token_indices: &[usize],
    connections: &[Connection],
    table: &SemanticContributionTable,
) -> usize {
    let mut drift_count = 0;

    for &token_index in token_indices {
        // token_id = token_index + 1
        let token_id = (token_index + 1) as u32;

        // Пересчитать Shell
        let new_profile = compute_shell(token_id, connections, table);

        // Сравнить с кэшем
        let cached_profile = cache.get(token_index);

        // Если отличается — обновить кэш
        if cached_profile != &new_profile {
            cache.profiles[token_index] = new_profile;
            drift_count += 1;
        }
    }

    drift_count
}

// ============================================================================
// TESTS
// ============================================================================

