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
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_profile_size() {
        // Shell V3.0: ShellProfile должен быть ровно 8 байт
        use std::mem::size_of;
        assert_eq!(size_of::<ShellProfile>(), 8, "ShellProfile must be 8 bytes");
    }

    #[test]
    fn test_empty_shell_constant() {
        // EMPTY_SHELL должен быть нулевым профилем
        assert_eq!(EMPTY_SHELL, [0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_layer_names_count() {
        // Должно быть 8 имён слоёв
        assert_eq!(LAYER_NAMES.len(), 8);
        assert_eq!(LAYER_NAMES[0], "Physical");
        assert_eq!(LAYER_NAMES[7], "Abstract");
    }

    #[test]
    fn test_domain_shell_cache_new() {
        // DomainShellCache::new() создаёт кэш с нулевыми профилями
        let cache = DomainShellCache::new(10);

        assert_eq!(cache.capacity(), 10);
        assert_eq!(cache.generation, 0);

        // Все профили должны быть EMPTY_SHELL
        for i in 0..10 {
            assert_eq!(cache.get(i), &EMPTY_SHELL);
            assert!(!cache.is_dirty(i), "Token {} should not be dirty initially", i);
        }
    }

    #[test]
    fn test_domain_shell_cache_get_set() {
        // get() и set() работают корректно
        let mut cache = DomainShellCache::new(5);

        let profile = [10, 20, 30, 40, 50, 60, 70, 80];
        cache.set(2, profile);

        assert_eq!(cache.get(2), &profile);
        assert_eq!(cache.get(0), &EMPTY_SHELL); // Другие не изменились
    }

    #[test]
    fn test_domain_shell_cache_dirty_flags() {
        // mark_dirty(), is_dirty(), clear_dirty() работают
        let mut cache = DomainShellCache::new(5);

        // Изначально все чистые
        assert!(!cache.is_dirty(0));
        assert!(!cache.is_dirty(4));

        // Помечаем токен 1 как грязный
        cache.mark_dirty(1);
        assert!(cache.is_dirty(1));
        assert!(!cache.is_dirty(0)); // Остальные чистые

        // Снимаем флаг
        cache.clear_dirty(1);
        assert!(!cache.is_dirty(1));
    }

    #[test]
    fn test_domain_shell_cache_clear_all_dirty() {
        // clear_all_dirty() снимает все флаги
        let mut cache = DomainShellCache::new(5);

        // Помечаем несколько токенов
        cache.mark_dirty(0);
        cache.mark_dirty(2);
        cache.mark_dirty(4);

        assert!(cache.is_dirty(0));
        assert!(cache.is_dirty(2));
        assert!(cache.is_dirty(4));

        // Снимаем все флаги
        cache.clear_all_dirty();

        for i in 0..5 {
            assert!(!cache.is_dirty(i), "Token {} should be clean after clear_all", i);
        }
    }

    #[test]
    fn test_domain_shell_cache_multiple_operations() {
        // Комбинация операций: set + mark_dirty + get
        let mut cache = DomainShellCache::new(3);

        let profile1 = [1, 2, 3, 4, 5, 6, 7, 8];
        let profile2 = [10, 20, 30, 40, 50, 60, 70, 80];

        cache.set(0, profile1);
        cache.mark_dirty(0);

        cache.set(1, profile2);
        // Не помечаем токен 1 как грязный

        assert_eq!(cache.get(0), &profile1);
        assert_eq!(cache.get(1), &profile2);
        assert_eq!(cache.get(2), &EMPTY_SHELL);

        assert!(cache.is_dirty(0));
        assert!(!cache.is_dirty(1));
        assert!(!cache.is_dirty(2));
    }

    #[test]
    fn test_shell_contribution_type() {
        // ShellContribution должен быть того же типа что ShellProfile
        use std::mem::size_of;
        assert_eq!(size_of::<ShellContribution>(), 8);

        let contribution: ShellContribution = [5, 10, 15, 20, 25, 30, 35, 40];
        assert_eq!(contribution.len(), 8);
    }

    // --- SemanticContributionTable Tests ---

    #[test]
    fn test_semantic_contribution_table_new() {
        // new() создаёт таблицу с нулевыми профилями
        let table = SemanticContributionTable::new();

        // Все категории должны быть нулевыми
        for category in 0..=255u8 {
            let link_type = (category as u16) << 8;
            assert_eq!(table.get(link_type), &[0; 8]);
        }
    }

    #[test]
    fn test_semantic_contribution_table_set_category() {
        // set_category() устанавливает базовый профиль категории
        let mut table = SemanticContributionTable::new();

        let structural_profile = [20, 5, 0, 0, 5, 0, 0, 0];
        table.set_category(0x01, structural_profile);

        // Любой link_type из категории 0x01 должен вернуть этот профиль
        assert_eq!(table.get(0x0100), &structural_profile);
        assert_eq!(table.get(0x0101), &structural_profile);
        assert_eq!(table.get(0x01FF), &structural_profile);

        // Другие категории остаются нулевыми
        assert_eq!(table.get(0x0200), &[0; 8]);
    }

    #[test]
    fn test_semantic_contribution_table_set_override() {
        // set_override() переопределяет конкретный тип
        let mut table = SemanticContributionTable::new();

        let category_profile = [0, 0, 5, 0, 15, 0, 10, 8];
        let override_profile = [0, 0, 0, 20, 10, 0, 10, 5];

        table.set_category(0x03, category_profile);
        table.set_override(0x0302, override_profile);

        // 0x0302 должен вернуть переопределённый профиль
        assert_eq!(table.get(0x0302), &override_profile);

        // Другие типы из категории 0x03 возвращают базовый профиль
        assert_eq!(table.get(0x0300), &category_profile);
        assert_eq!(table.get(0x0301), &category_profile);
        assert_eq!(table.get(0x0303), &category_profile);
    }

    #[test]
    fn test_semantic_contribution_table_two_level_hierarchy() {
        // Проверка двухуровневой иерархии: category vs override
        let mut table = SemanticContributionTable::new();

        table.set_category(0x04, [5, 20, 0, 15, 0, 0, 0, 0]);
        table.set_override(0x0406, [0, 15, 0, 10, 0, 0, 0, 15]);

        // Override приоритетнее category
        assert_eq!(table.get(0x0406), &[0, 15, 0, 10, 0, 0, 0, 15]);

        // Без override используется category
        assert_eq!(table.get(0x0405), &[5, 20, 0, 15, 0, 0, 0, 0]);
    }

    #[test]
    fn test_semantic_contribution_table_default_ashti_core() {
        // default_ashti_core() загружает 7 категорий
        let table = SemanticContributionTable::default_ashti_core();

        // 0x01: Structural [20, 5, 0, 0, 5, 0, 0, 0]
        assert_eq!(table.get(0x0100), &[20, 5, 0, 0, 5, 0, 0, 0]);

        // 0x02: Semantic [0, 0, 0, 0, 15, 0, 0, 10]
        assert_eq!(table.get(0x0200), &[0, 0, 0, 0, 15, 0, 0, 10]);

        // 0x03: Causal [0, 0, 5, 0, 15, 0, 10, 8]
        assert_eq!(table.get(0x0300), &[0, 0, 5, 0, 15, 0, 10, 8]);

        // 0x04: Experiential [5, 20, 0, 15, 0, 0, 0, 0]
        assert_eq!(table.get(0x0400), &[5, 20, 0, 15, 0, 0, 0, 0]);

        // 0x05: Social [0, 0, 0, 5, 0, 25, 0, 0]
        assert_eq!(table.get(0x0500), &[0, 0, 0, 5, 0, 25, 0, 0]);

        // 0x06: Temporal [0, 0, 0, 0, 5, 0, 25, 0]
        assert_eq!(table.get(0x0600), &[0, 0, 0, 0, 5, 0, 25, 0]);

        // 0x07: Motor [10, 0, 25, 0, 5, 0, 0, 0]
        assert_eq!(table.get(0x0700), &[10, 0, 25, 0, 5, 0, 0, 0]);

        // Неопределённые категории остаются нулевыми
        assert_eq!(table.get(0x0800), &[0; 8]);
        assert_eq!(table.get(0xFF00), &[0; 8]);
    }

    #[test]
    fn test_semantic_contribution_table_category_extraction() {
        // Проверка правильности извлечения категории из link_type
        let mut table = SemanticContributionTable::new();

        table.set_category(0x42, [1, 2, 3, 4, 5, 6, 7, 8]);

        // Старший байт 0x42, младший может быть любым
        assert_eq!(table.get(0x4200), &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(table.get(0x42AB), &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(table.get(0x42FF), &[1, 2, 3, 4, 5, 6, 7, 8]);

        // Другая категория
        assert_eq!(table.get(0x4300), &[0; 8]);
    }
}
