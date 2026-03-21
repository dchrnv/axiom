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

/// Флаги состояния токена
pub const STATE_ACTIVE: u8 = 1;
pub const STATE_SLEEPING: u8 = 2;
pub const STATE_LOCKED: u8 = 3;

/// Токен — элементарная единица пространства
///
/// Структура имеет фиксированный размер 64 байта и выравнивание на 64 байта
/// для оптимального размещения в кеш-линиях процессора.
///
/// Layout (64 байта):
/// - ИДЕНТИФИКАЦИЯ (8 байт): sutra_id, domain_id, type_flags
/// - ЛОКАЛЬНАЯ ФИЗИКА (16 байт): position, velocity, target, reserved_phys
/// - ТЕРМОДИНАМИКА (4 байта): valence, mass, temperature, state
/// - ФРАКТАЛЬНАЯ НАВИГАЦИЯ (36 байт): lineage_hash, momentum, resonance, last_event_id
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
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

    /// Резерв для выравнивания
    pub reserved_phys: u16,

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
            reserved_phys: 0,
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
        let temp_diff = (self.temperature as i16 - other.temperature as i16).abs() as u32;
        let temp_factor = if temp_diff > 100 { 0 } else { 100 - temp_diff };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(std::mem::size_of::<Token>(), 64);
        assert_eq!(std::mem::align_of::<Token>(), 64);
    }

    #[test]
    fn test_token_new() {
        let token = Token::new(1, 1, [0, 0, 0], 100);
        assert_eq!(token.sutra_id, 1);
        assert_eq!(token.domain_id, 1);
        assert_eq!(token.position, [0, 0, 0]);
        assert_eq!(token.last_event_id, 100);
        assert!(token.validate().is_ok());
    }

    #[test]
    fn test_token_validation() {
        let mut token = Token::new(1, 1, [0, 0, 0], 100);
        assert!(token.validate().is_ok());

        token.sutra_id = 0;
        assert!(token.validate().is_err());
        token.sutra_id = 1;

        token.domain_id = 0;
        assert!(token.validate().is_err());
        token.domain_id = 1;

        token.mass = 0;
        assert!(token.validate().is_err());
        token.mass = 100;

        token.last_event_id = 0;
        assert!(token.validate().is_err());
    }

    #[test]
    fn test_token_state_flags() {
        let mut token = Token::new(1, 1, [0, 0, 0], 100);

        token.state = STATE_ACTIVE;
        assert!(token.is_active());
        assert!(!token.is_sleeping());
        assert!(!token.is_locked());

        token.state = STATE_SLEEPING;
        assert!(!token.is_active());
        assert!(token.is_sleeping());
        assert!(!token.is_locked());

        token.state = STATE_LOCKED;
        assert!(!token.is_active());
        assert!(!token.is_sleeping());
        assert!(token.is_locked());
    }

    #[test]
    fn test_update_momentum() {
        let mut token = Token::new(1, 1, [0, 0, 0], 100);

        // Применяем силу
        let force = [10, -5, 3];
        token.update_momentum(force, 101);

        assert_eq!(token.momentum[0], 10);
        assert_eq!(token.momentum[1], -5);
        assert_eq!(token.momentum[2], 3);
        assert_eq!(token.last_event_id, 101);

        // Применяем ещё раз - импульс накапливается
        token.update_momentum([5, 5, -3], 102);
        assert_eq!(token.momentum[0], 15);
        assert_eq!(token.momentum[1], 0);
        assert_eq!(token.momentum[2], 0);
    }

    #[test]
    fn test_compute_resonance() {
        let mut token1 = Token::new(1, 1, [0, 0, 0], 100);
        token1.valence = 4; // Установим валентность
        let mut token2 = Token::new(2, 1, [10, 0, 0], 101);
        token2.valence = 4;

        // Одинаковые параметры, близко → высокий резонанс
        let resonance = token1.compute_resonance(&token1);
        assert!(resonance > 70, "resonance = {}", resonance);

        // Близко, похожие параметры → средний/высокий резонанс
        let resonance = token1.compute_resonance(&token2);
        assert!(resonance > 40, "resonance = {}", resonance);

        // Далеко → более низкий резонанс
        token2.position = [200, 200, 200];
        let resonance = token1.compute_resonance(&token2);
        assert!(resonance < 60, "resonance = {}", resonance);
    }
}
