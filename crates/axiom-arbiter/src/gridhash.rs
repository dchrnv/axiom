// GridHash — O(1) индекс для ассоциативной памяти (Этап 5)
//
// Шаги:
// 1. grid_hash(token, shift) — FNV-1a + rotate_left, только целочисленная арифметика
// 2. AssociativeIndex — HashMap<u64, Vec<u64>>, ключ = grid_hash, значение = created_at следов
// 3. Двухфазный поиск: Phase 1 (O(1) GridHash lookup) → Phase 2 (O(N) физика) при промахе
// 4. Обучение: insert при add_trace, remove при затухании

use std::collections::HashMap;
use axiom_core::Token;

/// Вычислить GridHash токена
///
/// Алгоритм:
/// 1. FNV-1a инициализация
/// 2. Перемешиваем все поля токена
/// 3. Позицию сдвигаем на `shift` бит — это делает ключ «грубее»:
///    - shift=0: точный ключ (каждый квант отдельно)
///    - shift=4: ячейки 16 квантов (хороший баланс)
///    - shift=8: ячейки 256 квантов (крупные группы)
/// 4. rotate_left для распределения битов
///
/// # Детерминизм
/// Одинаковый token + shift → одинаковый хэш. Всегда.
pub fn grid_hash(token: &Token, shift: u32) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut h: u64 = FNV_OFFSET;

    // Семантические поля — без сдвига (точные)
    h ^= token.temperature as u64;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(13);
    h ^= token.mass as u64;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(13);
    h ^= (token.valence as u8) as u64;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(13);

    // Позиция — с coarsening через shift
    let x = ((token.position[0] as i32) >> shift) as u64;
    let y = ((token.position[1] as i32) >> shift) as u64;
    let z = ((token.position[2] as i32) >> shift) as u64;

    h ^= x;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(17);
    h ^= y;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(17);
    h ^= z;
    h = h.wrapping_mul(FNV_PRIME).rotate_left(17);

    h
}

/// GridHash с Shell-профилем (когда доступен)
///
/// Добавляет 8 семантических слоёв в хэш для более точной группировки.
/// Слои смешиваются побайтово перед добавлением к позиционному хэшу.
pub fn grid_hash_with_shell(token: &Token, shell: &[u8; 8], shift: u32) -> u64 {
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut h = grid_hash(token, shift);

    // Подмешиваем Shell-профиль (L1-L8)
    for (i, &layer) in shell.iter().enumerate() {
        h ^= (layer as u64) << (i * 7 % 56);
        h = h.wrapping_mul(FNV_PRIME).rotate_left(11);
    }

    h
}

/// Ассоциативный индекс: grid_hash → список trace_id (created_at)
///
/// Обеспечивает O(1) Phase 1 поиск в Experience.
/// Использует `created_at` как стабильный идентификатор следа
/// (не смещается при eviction из Vec<ExperienceTrace>).
pub struct AssociativeIndex {
    /// grid_hash → Vec<created_at>
    table: HashMap<u64, Vec<u64>>,
    /// created_at → grid_hash (для удаления по trace_id)
    reverse: HashMap<u64, u64>,
    /// Shift-фактор для coarsening позиции
    pub shift: u32,
}

impl AssociativeIndex {
    /// Создать индекс с заданным shift-фактором
    ///
    /// `shift = 4` — хороший старт (ячейки 16 квантов).
    pub fn new(shift: u32) -> Self {
        Self {
            table: HashMap::new(),
            reverse: HashMap::new(),
            shift,
        }
    }

    /// Добавить след в индекс
    pub fn insert(&mut self, key: u64, trace_id: u64) {
        self.table.entry(key).or_default().push(trace_id);
        self.reverse.insert(trace_id, key);
    }

    /// Удалить след из индекса по trace_id
    ///
    /// Возвращает true если след был найден и удалён.
    pub fn remove_by_trace_id(&mut self, trace_id: u64) -> bool {
        if let Some(key) = self.reverse.remove(&trace_id) {
            if let Some(ids) = self.table.get_mut(&key) {
                ids.retain(|&id| id != trace_id);
                if ids.is_empty() {
                    self.table.remove(&key);
                }
            }
            return true;
        }
        false
    }

    /// Найти trace_id по ключу
    ///
    /// Возвращает срез `created_at` значений для этого grid-ключа.
    pub fn lookup(&self, key: u64) -> Option<&[u64]> {
        self.table.get(&key).map(|v| v.as_slice())
    }

    /// Количество уникальных ячеек (занятых grid-ключей)
    pub fn cell_count(&self) -> usize {
        self.table.len()
    }

    /// Количество следов в индексе
    pub fn trace_count(&self) -> usize {
        self.reverse.len()
    }

    /// Очистить индекс
    pub fn clear(&mut self) {
        self.table.clear();
        self.reverse.clear();
    }
}

impl Default for AssociativeIndex {
    fn default() -> Self {
        Self::new(4)
    }
}
