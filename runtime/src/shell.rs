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
}
