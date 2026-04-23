//! Token — базовая единица пространства AXIOM
//!
//! Token представляет элементарный узел в пространстве. Размер строго 64 байта.
//! Используется выравнивание на 64 байта для оптимизации кеширования.
//!
//! # Инварианты
//! - `sutra_id > 0` — каждый токен принадлежит конкретному потоку
//! - `domain_id > 0` — каждый токен принадлежит домену
//! - `mass > 0` — масса всегда положительна
//! - `last_event_id > 0` — каждый токен имеет событие создания
//! - Размер структуры строго 64 байта

use std::fmt;

/// Флаги типа токена в `type_flags` поле
///
/// Токен является целью (Goal) — CODEX повышает его mass и temperature,
/// Arbiter генерирует Goal-импульсы для незавершённых целей.
pub const TOKEN_FLAG_GOAL: u16 = 0x0001;

/// Токен является внутренним импульсом (Cognitive Depth).
/// Установлен для tension и goal импульсов перед повторной маршрутизацией.
/// Предотвращает петлю: impulse → tension → impulse → ...
pub const TOKEN_FLAG_IMPULSE: u16 = 0x0002;

/// Токен является анкером Frame в EXPERIENCE (domain_id=109, state=STATE_ACTIVE).
/// Устанавливается FrameWeaver при кристаллизации узора.
pub const TOKEN_FLAG_FRAME_ANCHOR: u16 = 0x0010;

/// Frame-анкер был промотирован из EXPERIENCE в SUTRA через CODEX-санкцию.
/// В SUTRA: domain_id=100, state=STATE_LOCKED, оригинал в EXPERIENCE сохраняется.
pub const TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE: u16 = 0x0020;

/// Маска для выделения категории Frame из type_flags (старший байт).
pub const FRAME_CATEGORY_MASK: u16 = 0xFF00;

/// Синтаксический Frame: подлежащее–сказуемое–дополнение и реляционные роли (V1.1).
pub const FRAME_CATEGORY_SYNTAX: u16 = 0x0100;

/// Флаги состояния токена
pub const STATE_ACTIVE: u8 = 1;
/// Токен находится в спящем режиме
pub const STATE_SLEEPING: u8 = 2;
/// Токен заблокирован
pub const STATE_LOCKED: u8 = 3;

/// Токен — элементарная единица пространства
///
/// Структура имеет фиксированный размер 64 байта и выравнивание на 64 байта
/// для оптимального размещения в кеш-линиях процессора.
///
/// Layout (64 байта):
/// - ИДЕНТИФИКАЦИЯ (8 байт): sutra_id, domain_id, type_flags
/// - ЛОКАЛЬНАЯ ФИЗИКА (16 байт): position, velocity, target, origin
/// - ТЕРМОДИНАМИКА (4 байта): valence, mass, temperature, state
/// - ФРАКТАЛЬНАЯ НАВИГАЦИЯ (36 байт): lineage_hash, momentum, resonance, last_event_id
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Token {
    // --- ИДЕНТИФИКАЦИЯ (8 Байт) ---
    /// ID потока (Sutra), которому принадлежит токен
    pub sutra_id: u32,

    /// ID домена, в котором существует токен
    pub domain_id: u16,

    /// Флаги типа токена
    pub type_flags: u16,

    // --- ЛОКАЛЬНАЯ ФИЗИКА ПОЛЯ (16 Байт) ---
    /// Позиция в пространстве [x, y, z] (целочисленные координаты)
    pub position: [i16; 3],

    /// Вектор скорости [vx, vy, vz]
    pub velocity: [i16; 3],

    /// Целевая позиция [tx, ty, tz]
    pub target: [i16; 3],

    /// Происхождение токена (откуда пришёл).
    /// 0x0000 = создан здесь, 0x01..=0xFF = пришёл с уровня N FractalChain,
    /// 0xFE00 = восстановлен из persistence, 0xFF00..=0xFFFF = внешний источник.
    pub origin: u16,

    // --- ТЕРМОДИНАМИКА (4 Байта) ---
    /// Валентность — способность формировать связи
    pub valence: i8,

    /// Масса токена (влияет на динамику)
    pub mass: u8,

    /// Температура (активность токена)
    pub temperature: u8,

    /// Текущее состояние
    pub state: u8,

    // --- ФРАКТАЛЬНАЯ НАВИГАЦИЯ (36 Байт) ---
    /// Хеш линии наследования
    pub lineage_hash: u64,

    /// Импульс [px, py, pz]
    pub momentum: [i32; 3],

    /// Резонанс (совместимость с другими токенами)
    pub resonance: u32,

    /// ID последнего события, изменившего токен (COM timestamp)
    pub last_event_id: u64,
}

// Проверка размера на этапе компиляции
const _: () = assert!(std::mem::size_of::<Token>() == 64);

// === Token origin ===
/// Токен рождён в текущем уровне (создан в SUTRA)
pub const TOKEN_ORIGIN_LOCAL: u16 = 0x0000;
// 0x0001..=0x00FF — пришёл с уровня N FractalChain
// Пример: origin = 3 означает "пришёл с уровня 3 через MAYA[3] → SUTRA[4]"

/// Восстановлен из persistence (загружен с диска)
pub const TOKEN_ORIGIN_PERSISTED: u16 = 0xFE00;

/// База для внешних источников (future multi-instance)
/// Конкретный source_id кодируется в младших 8 битах: 0xFF00 | source_id
pub const TOKEN_ORIGIN_EXTERNAL_BASE: u16 = 0xFF00;

impl Token {
    /// Создает новый токен с минимальными параметрами
    ///
    /// # Arguments
    /// * `sutra_id` - ID потока-владельца
    /// * `domain_id` - ID домена
    /// * `position` - Начальная позиция в пространстве
    /// * `event_id` - ID события создания
    ///
    /// # Returns
    /// Новый экземпляр Token с параметрами по умолчанию
    pub fn new(sutra_id: u32, domain_id: u16, position: [i16; 3], event_id: u64) -> Self {
        Self {
            sutra_id,
            domain_id,
            type_flags: 0,
            position,
            velocity: [0, 0, 0],
            target: position,
            origin: TOKEN_ORIGIN_LOCAL,
            valence: 0,
            mass: 100,
            temperature: 100,
            state: STATE_ACTIVE,
            lineage_hash: 0,
            momentum: [0, 0, 0],
            resonance: 0,
            last_event_id: event_id,
        }
    }

    /// Валидирует инварианты токена
    ///
    /// # Returns
    /// `Ok(())` если все инварианты соблюдены, `Err(String)` с описанием ошибки иначе
    pub fn validate(&self) -> Result<(), String> {
        if self.sutra_id == 0 {
            return Err("Token.sutra_id must be > 0".to_string());
        }
        if self.domain_id == 0 {
            return Err("Token.domain_id must be > 0".to_string());
        }
        if self.mass == 0 {
            return Err("Token.mass must be > 0".to_string());
        }
        if self.last_event_id == 0 {
            return Err("Token.last_event_id must be > 0".to_string());
        }
        Ok(())
    }

    /// Проверяет, активен ли токен
    #[inline]
    pub fn is_active(&self) -> bool {
        self.state == STATE_ACTIVE
    }

    /// Проверяет, находится ли токен в спящем режиме
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        self.state == STATE_SLEEPING
    }

    /// Проверяет, заблокирован ли токен
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.state == STATE_LOCKED
    }

    /// Обновляет импульс токена применяя силу
    ///
    /// # Arguments
    /// * `force` - Вектор силы [fx, fy, fz]
    /// * `event_id` - ID события для обновления last_event_id
    pub fn update_momentum(&mut self, force: [i32; 3], event_id: u64) {
        self.momentum[0] += force[0];
        self.momentum[1] += force[1];
        self.momentum[2] += force[2];
        self.last_event_id = event_id;
    }

    /// Вычисляет резонанс между двумя токенами
    ///
    /// Резонанс зависит от:
    /// - Разницы температур
    /// - Валентности обоих токенов
    /// - Расстояния между токенами
    ///
    /// # Arguments
    /// * `other` - Другой токен для расчета резонанса
    ///
    /// # Returns
    /// Значение резонанса (0..=100)
    pub fn compute_resonance(&self, other: &Token) -> u32 {
        // Разница температур (чем меньше, тем лучше резонанс)
        let temp_diff = (self.temperature as i16 - other.temperature as i16).unsigned_abs() as u32;
        let temp_factor = 100u32.saturating_sub(temp_diff);

        // Валентность (среднее значение, преобразуем i8 в положительные значения)
        let val1 = self.valence.max(0) as u32;
        let val2 = other.valence.max(0) as u32;
        let valence_factor = ((val1 + val2) / 2).min(100);

        // Расстояние
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        let dist_sq = (dx as i32 * dx as i32 + dy as i32 * dy as i32 + dz as i32 * dz as i32) as u32;
        let dist_factor = if dist_sq > 10000 { 0 } else { 100 - (dist_sq / 100) };

        // Итоговый резонанс (взвешенное среднее)
        (temp_factor * 2 + valence_factor + dist_factor) / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_size_unchanged() {
        assert_eq!(std::mem::size_of::<Token>(), 64);
    }

    #[test]
    fn token_origin_default_is_local() {
        let t = Token::new(1, 1, [0, 0, 0], 1);
        assert_eq!(t.origin, TOKEN_ORIGIN_LOCAL);
        assert_eq!(t.origin, 0);
    }

    #[test]
    fn token_origin_fractalchain_level() {
        let mut t = Token::new(1, 1, [0, 0, 0], 1);
        t.origin = 3; // пришёл с уровня 3
        assert_eq!(t.origin, 3);
    }

    #[test]
    fn token_origin_persisted() {
        let mut t = Token::new(1, 1, [0, 0, 0], 1);
        t.origin = TOKEN_ORIGIN_PERSISTED;
        assert_eq!(t.origin, 0xFE00);
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token[sutra={}, domain={}, pos=({},{},{}), state={:04x}, event={}]",
            self.sutra_id,
            self.domain_id,
            self.position[0],
            self.position[1],
            self.position[2],
            self.state,
            self.last_event_id
        )
    }
}
