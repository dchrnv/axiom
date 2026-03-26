//! Connection — связь между токенами в пространстве AXIOM
//!
//! Connection представляет направленную связь между двумя токенами. Размер строго 64 байта.
//! Использует выравнивание на 64 байта для оптимизации кеширования.
//!
//! # Инварианты
//! - `source_id > 0` — источник связи должен существовать
//! - `target_id > 0` — цель связи должна существовать
//! - `domain_id > 0` — связь принадлежит домену
//! - `strength > 0.0` — сила связи всегда положительна
//! - `current_stress >= 0.0` — стресс не может быть отрицательным
//! - `elasticity > 0.0` — эластичность всегда положительна
//! - `created_at > 0` — связь имеет время создания
//! - `last_event_id >= created_at` — события монотонно возрастают
//! - Размер структуры строго 64 байта

use std::fmt;

/// Флаги состояния связи
pub const FLAG_ACTIVE: u32 = 1;
pub const FLAG_INHIBITED: u32 = 2;
pub const FLAG_TEMPORARY: u32 = 4;
pub const FLAG_CRITICAL: u32 = 8;

/// Connection — связь между двумя токенами
///
/// Структура имеет фиксированный размер 64 байта и выравнивание на 64 байта.
/// Содержит топологию, динамику, шлюзы и метаданные связи.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    // --- ТОПОЛОГИЯ (16 Байт) ---
    /// ID токена-источника связи
    pub source_id: u32,

    /// ID токена-цели связи
    pub target_id: u32,

    /// ID домена, в котором существует связь
    pub domain_id: u16,

    /// Тип связи (application-defined)
    pub link_type: u16,

    /// Флаги состояния связи
    pub flags: u32,

    // --- ДИНАМИКА (16 Байт) ---
    /// Сила связи (влияет на распространение сигналов)
    pub strength: f32,

    /// Текущий стресс (нагрузка на связь)
    pub current_stress: f32,

    /// Идеальная дистанция между токенами
    pub ideal_dist: f32,

    /// Эластичность (способность восстанавливаться)
    pub elasticity: f32,

    // --- ШЛЮЗ (16 Байт) ---
    /// Шлюз плотности (минимальная масса для прохождения)
    pub density_gate: u8,

    /// Термальный шлюз (максимальная температура для прохождения)
    pub thermal_gate: u8,

    /// Резерв для будущих шлюзов
    pub reserved_gate: [u8; 14],

    // --- МЕТАДАННЫЕ (16 Байт) ---
    /// Event ID создания связи (COM timestamp)
    pub created_at: u64,

    /// ID последнего события, изменившего связь (COM timestamp)
    pub last_event_id: u64,
}

// Проверка размера на этапе компиляции
const _: () = assert!(std::mem::size_of::<Connection>() == 64);

impl Default for Connection {
    fn default() -> Self {
        Self {
            source_id: 0,
            target_id: 0,
            domain_id: 0,
            link_type: 0,
            flags: 0,
            strength: 1.0,
            current_stress: 0.0,
            ideal_dist: 0.0,
            elasticity: 1.0,
            density_gate: 0,
            thermal_gate: 0,
            reserved_gate: [0; 14],
            created_at: 0,
            last_event_id: 0,
        }
    }
}

impl Connection {
    /// Создает новую связь с минимальными параметрами
    ///
    /// # Arguments
    /// * `source_id` - ID токена-источника
    /// * `target_id` - ID токена-цели
    /// * `domain_id` - ID домена
    /// * `event_id` - ID события создания
    ///
    /// # Returns
    /// Новый экземпляр Connection с параметрами по умолчанию
    pub fn new(source_id: u32, target_id: u32, domain_id: u16, event_id: u64) -> Self {
        Self {
            source_id,
            target_id,
            domain_id,
            created_at: event_id,
            last_event_id: event_id,
            flags: FLAG_ACTIVE,
            ..Default::default()
        }
    }

    /// Проверяет, активна ли связь
    #[inline]
    pub fn is_active(&self) -> bool {
        (self.flags & FLAG_ACTIVE) != 0
    }

    /// Проверяет, ингибирована ли связь
    #[inline]
    pub fn is_inhibited(&self) -> bool {
        (self.flags & FLAG_INHIBITED) != 0
    }

    /// Проверяет, временная ли связь
    #[inline]
    pub fn is_temporary(&self) -> bool {
        (self.flags & FLAG_TEMPORARY) != 0
    }

    /// Проверяет, критична ли связь
    #[inline]
    pub fn is_critical(&self) -> bool {
        (self.flags & FLAG_CRITICAL) != 0
    }

    /// Валидирует инварианты связи
    ///
    /// # Returns
    /// `Ok(())` если все инварианты соблюдены, `Err(String)` с описанием ошибки иначе
    pub fn validate(&self) -> Result<(), String> {
        if self.source_id == 0 {
            return Err("Connection.source_id must be > 0".to_string());
        }
        if self.target_id == 0 {
            return Err("Connection.target_id must be > 0".to_string());
        }
        if self.domain_id == 0 {
            return Err("Connection.domain_id must be > 0".to_string());
        }
        if self.strength <= 0.0 {
            return Err("Connection.strength must be > 0.0".to_string());
        }
        if self.current_stress < 0.0 {
            return Err("Connection.current_stress must be >= 0.0".to_string());
        }
        if self.elasticity <= 0.0 {
            return Err("Connection.elasticity must be > 0.0".to_string());
        }
        if self.created_at == 0 {
            return Err("Connection.created_at must be > 0".to_string());
        }
        if self.last_event_id < self.created_at {
            return Err("Connection.last_event_id must be >= created_at".to_string());
        }
        Ok(())
    }

    /// Проверяет, может ли пройти токен с заданной массой
    ///
    /// # Arguments
    /// * `mass` - Масса токена
    ///
    /// # Returns
    /// `true` если токен может пройти через шлюз плотности
    #[inline]
    pub fn can_pass_mass(&self, mass: u8) -> bool {
        mass >= self.density_gate
    }

    /// Проверяет, может ли пройти токен с заданной температурой
    ///
    /// # Arguments
    /// * `temperature` - Температура токена
    ///
    /// # Returns
    /// `true` если токен может пройти через термальный шлюз
    #[inline]
    pub fn can_pass_temperature(&self, temperature: u8) -> bool {
        temperature <= self.thermal_gate
    }

    /// Обновляет стресс связи
    ///
    /// Автоматически устанавливает флаг `FLAG_CRITICAL` если стресс превышает 80% силы.
    ///
    /// # Arguments
    /// * `new_stress` - Новое значение стресса
    /// * `event_id` - ID события, вызвавшего обновление
    pub fn update_stress(&mut self, new_stress: f32, event_id: u64) {
        self.current_stress = new_stress.max(0.0);
        self.last_event_id = event_id;

        // Автоматическая установка критического флага
        if self.current_stress > self.strength * 0.8 {
            self.flags |= FLAG_CRITICAL;
        } else {
            self.flags &= !FLAG_CRITICAL;
        }
    }

    /// Вычисляет расстояние между позициями токенов
    ///
    /// # Arguments
    /// * `source_pos` - Позиция токена-источника
    /// * `target_pos` - Позиция токена-цели
    ///
    /// # Returns
    /// Евклидово расстояние между позициями
    pub fn compute_distance(&self, source_pos: [i32; 3], target_pos: [i32; 3]) -> f32 {
        let dx = (target_pos[0] - source_pos[0]) as f32;
        let dy = (target_pos[1] - source_pos[1]) as f32;
        let dz = (target_pos[2] - source_pos[2]) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Вычисляет силу притяжения/отталкивания на основе текущей и идеальной дистанции
    ///
    /// Сила пропорциональна отклонению от идеальной дистанции, умноженному на эластичность.
    ///
    /// # Arguments
    /// * `current_distance` - Текущее расстояние между токенами
    ///
    /// # Returns
    /// Сила (положительная = притяжение, отрицательная = отталкивание)
    pub fn compute_spring_force(&self, current_distance: f32) -> f32 {
        let displacement = current_distance - self.ideal_dist;
        -displacement * self.elasticity * self.strength
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Connection[{}->{}, domain={}, strength={:.2}, stress={:.2}, flags={:04x}]",
            self.source_id,
            self.target_id,
            self.domain_id,
            self.strength,
            self.current_stress,
            self.flags
        )
    }
}
