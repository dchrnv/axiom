// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SPACE V6.0: docs/spec/SPACE_V6_0.md
// Целочисленная пространственная модель с детерминистичной физикой

pub mod simd;
pub use simd::{
    apply_accelerations_to_velocities, apply_gravity_batch, apply_gravity_batch_avx2,
    apply_gravity_batch_chunked, GravityBatchResult, L2_CHUNK_TOKENS,
};

use serde::{Deserialize, Serialize};

/// Константы пространственной модели
///
/// CELL_SHIFT определяет размер ячейки как степень двойки:
/// - cell_size = 1 << CELL_SHIFT
/// - CELL_SHIFT = 8 → cell_size = 256 квантов
pub const CELL_SHIFT: u32 = 8;
pub const CELL_SIZE: i32 = 1 << CELL_SHIFT; // 256

/// Количество корзин в хеш-таблице (степень двойки для быстрого маскирования)
pub const BUCKET_COUNT_LOG2: u32 = 16;
pub const BUCKET_COUNT: usize = 1 << BUCKET_COUNT_LOG2; // 65536
pub const BUCKET_MASK: u32 = (BUCKET_COUNT - 1) as u32;

/// Якорь - центр гравитации (0, 0, 0)
pub const ANCHOR_X: i16 = 0;
pub const ANCHOR_Y: i16 = 0;
pub const ANCHOR_Z: i16 = 0;

/// Вычислить квадрат расстояния между двумя точками
///
/// Целочисленная арифметика с детерминистичным overflow:
/// - Входы: i16 координаты (-32768..+32767)
/// - Промежуточные вычисления: i64 для безопасности
/// - Результат: i64 для квадратов
///
/// Формула: dist² = dx² + dy² + dz²
///
/// ВАЖНО: Возвращает КВАДРАТ расстояния (без корня).
/// Это позволяет избежать вычисления корня и сохранить детерминизм.
///
/// Пример:
/// - distance2((0,0,0), (100,0,0)) = 10000
/// - distance2((0,0,0), (100,100,100)) = 30000
#[inline]
pub fn distance2(x1: i16, y1: i16, z1: i16, x2: i16, y2: i16, z2: i16) -> i64 {
    let dx = (x2 as i64) - (x1 as i64);
    let dy = (y2 as i64) - (y1 as i64);
    let dz = (z2 as i64) - (z1 as i64);

    // Вычисление квадратов в i64 - безопасно для любых i16
    dx * dx + dy * dy + dz * dz
}

/// Вычислить квадрат расстояния до якоря (0, 0, 0)
///
/// Оптимизированная версия distance2 для расстояния до начала координат
#[inline]
pub fn distance2_to_anchor(x: i16, y: i16, z: i16) -> i64 {
    let x64 = x as i64;
    let y64 = y as i64;
    let z64 = z as i64;

    x64 * x64 + y64 * y64 + z64 * z64
}

/// Модель гравитации для вычисления силы притяжения
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GravityModel {
    /// Линейная модель: F = k * distance
    /// Сила прямо пропорциональна расстоянию
    Linear,

    /// Обратная квадратичная модель: F = k * mass / distance²
    /// Сила обратно пропорциональна квадрату расстояния (как в физике)
    InverseSquare,
}

/// Вычислить силу гравитации к якорю (0, 0, 0)
///
/// Аргументы:
/// - x, y, z: координаты токена
/// - mass: масса токена (из Token.energy)
/// - gravity_scale_shift: битовый сдвиг для масштабирования (обычно 24)
/// - model: модель гравитации (Linear или InverseSquare)
///
/// Возвращает:
/// - (ax, ay, az): компоненты ускорения в направлении якоря
///
/// ЛИНЕЙНАЯ модель:
/// - F = k * distance
/// - Чем дальше от якоря, тем сильнее притяжение
/// - Простая модель для начальной реализации
///
/// INVERSE_SQUARE модель:
/// - F = k * mass / distance²
/// - Чем ближе к якорю, тем сильнее притяжение (реалистичная физика)
/// - Требует обработку distance = 0 (токен точно в якоре)
///
/// Масштабирование:
/// - gravity_scale_shift определяет силу гравитации
/// - Большие значения (24-28) → слабая гравитация
/// - Малые значения (16-20) → сильная гравитация
///
/// Пример:
/// ```
/// use axiom_space::*;
/// let (ax, ay, az) = compute_gravity(100, 200, 300, 1000, 24, GravityModel::Linear);
/// // Вернёт ускорение в направлении (0, 0, 0)
/// ```
pub fn compute_gravity(
    x: i16,
    y: i16,
    z: i16,
    mass: u16,
    gravity_scale_shift: u32,
    model: GravityModel,
) -> (i16, i16, i16) {
    // Токен точно в якоре - нет силы
    if x == ANCHOR_X && y == ANCHOR_Y && z == ANCHOR_Z {
        return (0, 0, 0);
    }

    let dx = (ANCHOR_X as i64) - (x as i64); // Направление к якорю
    let dy = (ANCHOR_Y as i64) - (y as i64);
    let dz = (ANCHOR_Z as i64) - (z as i64);

    let dist2 = distance2_to_anchor(x, y, z);

    // Вычисление силы в зависимости от модели
    let force_magnitude = match model {
        GravityModel::Linear => {
            // F = k * distance
            // Используем целочисленный квадратный корень для distance
            let dist = integer_sqrt(dist2);

            // Масштабируем силу: dist делим на 2^scale_shift
            if gravity_scale_shift >= 32 {
                0
            } else {
                dist >> gravity_scale_shift
            }
        }
        GravityModel::InverseSquare => {
            // F = k * mass / distance²
            if dist2 == 0 {
                return (0, 0, 0); // Предотвращение деления на ноль
            }

            let mass64 = mass as i64;
            // Масштабированная сила: (mass << scale) / dist²
            // Для предотвращения переполнения используем saturating операции
            if gravity_scale_shift >= 40 {
                0 // Слишком слабая гравитация
            } else {
                let scale_limited = gravity_scale_shift.min(30);
                let numerator = mass64.saturating_mul(1i64 << scale_limited);
                numerator / dist2
            }
        }
    };

    // Применяем силу к каждой компоненте пропорционально расстоянию
    // Нормализация: force * (direction / distance)
    if force_magnitude == 0 {
        return (0, 0, 0);
    }

    // Вычисление компонент ускорения
    // a_x = force * dx / distance
    let dist = integer_sqrt(dist2).max(1); // Предотвращение деления на ноль

    let ax = ((force_magnitude * dx) / dist).clamp(i16::MIN as i64, i16::MAX as i64) as i16;
    let ay = ((force_magnitude * dy) / dist).clamp(i16::MIN as i64, i16::MAX as i64) as i16;
    let az = ((force_magnitude * dz) / dist).clamp(i16::MIN as i64, i16::MAX as i64) as i16;

    (ax, ay, az)
}

/// Применить скорость к позиции (обновление координат)
///
/// Аргументы:
/// - pos: текущая позиция (x, y, z)
/// - vel: текущая скорость (vx, vy, vz)
///
/// Возвращает:
/// - новая позиция с применённой скоростью (с clamp в пределах i16)
///
/// Формула:
/// - new_pos = pos + vel
///
/// ВАЖНО: Использует saturating операции для предотвращения overflow
///
/// Пример:
/// ```
/// use axiom_space::*;
/// let pos = (100, 200, 300);
/// let vel = (10, -5, 0);
/// let new_pos = apply_velocity(pos, vel);
/// assert_eq!(new_pos, (110, 195, 300));
/// ```
#[inline]
pub fn apply_velocity(pos: (i16, i16, i16), vel: (i16, i16, i16)) -> (i16, i16, i16) {
    let (x, y, z) = pos;
    let (vx, vy, vz) = vel;

    let new_x = x.saturating_add(vx);
    let new_y = y.saturating_add(vy);
    let new_z = z.saturating_add(vz);

    (new_x, new_y, new_z)
}

/// Применить трение к скорости (замедление)
///
/// Аргументы:
/// - vel: текущая скорость (vx, vy, vz)
/// - friction_shift: битовый сдвиг для коэффициента трения
///
/// Возвращает:
/// - новая скорость с применённым трением
///
/// Формула:
/// - new_vel = vel - (vel >> friction_shift)
/// - friction_shift = 0 → полное гашение (vel становится 0)
/// - friction_shift = 8 → vel * (1 - 1/256) ≈ 99.6% от исходной
/// - friction_shift = 16 → vel * (1 - 1/65536) ≈ 99.998% (очень слабое трение)
///
/// ВАЖНО: Всегда уменьшает скорость, никогда не меняет направление
///
/// Пример:
/// ```
/// use axiom_space::*;
/// let vel = (100, -200, 50);
/// let new_vel = apply_friction(vel, 4); // Коэффициент 1/16
/// assert_eq!(new_vel, (94, -187, 47)); // vel - vel/16
/// ```
#[inline]
pub fn apply_friction(vel: (i16, i16, i16), friction_shift: u32) -> (i16, i16, i16) {
    let (vx, vy, vz) = vel;

    if friction_shift >= 16 {
        // Очень слабое трение - практически не влияет
        return vel;
    }

    // Вычисляем уменьшение скорости: vel >> friction_shift
    let dx = vx >> friction_shift;
    let dy = vy >> friction_shift;
    let dz = vz >> friction_shift;

    // Применяем трение: vel = vel - delta
    // Для положительных vel уменьшаем, для отрицательных увеличиваем (к нулю)
    let new_vx = vx.saturating_sub(dx);
    let new_vy = vy.saturating_sub(dy);
    let new_vz = vz.saturating_sub(dz);

    (new_vx, new_vy, new_vz)
}

/// Применить ускорение к скорости
///
/// Аргументы:
/// - vel: текущая скорость (vx, vy, vz)
/// - acc: ускорение (ax, ay, az)
///
/// Возвращает:
/// - новая скорость с применённым ускорением (с clamp в пределах i16)
///
/// Формула:
/// - new_vel = vel + acc
///
/// ВАЖНО: Использует saturating операции для предотвращения overflow
///
/// Пример:
/// ```
/// use axiom_space::*;
/// let vel = (100, -50, 0);
/// let acc = (10, 5, -3);
/// let new_vel = apply_acceleration(vel, acc);
/// assert_eq!(new_vel, (110, -45, -3));
/// ```
#[inline]
pub fn apply_acceleration(vel: (i16, i16, i16), acc: (i16, i16, i16)) -> (i16, i16, i16) {
    let (vx, vy, vz) = vel;
    let (ax, ay, az) = acc;

    let new_vx = vx.saturating_add(ax);
    let new_vy = vy.saturating_add(ay);
    let new_vz = vz.saturating_add(az);

    (new_vx, new_vy, new_vz)
}

/// Ограничить значение в пределах i16
///
/// Аргументы:
/// - value: значение для ограничения (i32 для промежуточных вычислений)
///
/// Возвращает:
/// - значение, ограниченное диапазоном [i16::MIN, i16::MAX]
///
/// Пример:
/// ```
/// use axiom_space::*;
/// assert_eq!(clamp_i16(100), 100);
/// assert_eq!(clamp_i16(50000), 32767); // i16::MAX
/// assert_eq!(clamp_i16(-50000), -32768); // i16::MIN
/// ```
#[inline]
pub fn clamp_i16(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Вычислить ускорение в направлении целевой точки (аттрактор)
///
/// Аргументы:
/// - pos: текущая позиция (x, y, z)
/// - target: целевая позиция (target_x, target_y, target_z)
/// - attraction_shift: битовый сдвиг для силы притяжения
///
/// Возвращает:
/// - ускорение в направлении цели (ax, ay, az)
///
/// Формула:
/// - direction = target - pos
/// - acceleration = direction >> attraction_shift
///
/// ВАЖНО:
/// - attraction_shift = 0 → максимальная сила (полная разность)
/// - attraction_shift = 8 → сила = distance / 256
/// - attraction_shift = 16 → сила = distance / 65536 (очень слабая)
///
/// Пример:
/// ```
/// use axiom_space::*;
/// let pos = (100, 100, 100);
/// let target = (200, 100, 100);
/// let acc = move_towards(pos, target, 4);
/// // Результат: ((200-100)/16, 0, 0) = (6, 0, 0)
/// ```
#[inline]
pub fn move_towards(
    pos: (i16, i16, i16),
    target: (i16, i16, i16),
    attraction_shift: u32,
) -> (i16, i16, i16) {
    let (x, y, z) = pos;
    let (tx, ty, tz) = target;

    // Вычисляем направление к цели
    let dx = (tx as i32) - (x as i32);
    let dy = (ty as i32) - (y as i32);
    let dz = (tz as i32) - (z as i32);

    // Уже в цели - нет ускорения
    if dx == 0 && dy == 0 && dz == 0 {
        return (0, 0, 0);
    }

    // Применяем сдвиг для силы притяжения
    if attraction_shift >= 16 {
        // Очень слабое притяжение
        return (0, 0, 0);
    }

    let ax = clamp_i16(dx >> attraction_shift);
    let ay = clamp_i16(dy >> attraction_shift);
    let az = clamp_i16(dz >> attraction_shift);

    (ax, ay, az)
}

/// Проверить произошло ли движение токена
///
/// Аргументы:
/// - old_pos: предыдущая позиция (x, y, z)
/// - new_pos: новая позиция (x, y, z)
///
/// Возвращает:
/// - true если токен переместился (хотя бы одна координата изменилась)
///
/// Используется для генерации события TokenMoved
#[inline]
pub fn has_moved(old_pos: (i16, i16, i16), new_pos: (i16, i16, i16)) -> bool {
    old_pos.0 != new_pos.0 || old_pos.1 != new_pos.1 || old_pos.2 != new_pos.2
}

/// Проверить изменилась ли ячейка spatial hash
///
/// Аргументы:
/// - old_pos: предыдущая позиция (x, y, z)
/// - new_pos: новая позиция (x, y, z)
///
/// Возвращает:
/// - true если токен вошёл в новую ячейку spatial hash
///
/// Используется для генерации события TokenEnteredCell и триггера rebuild
#[inline]
pub fn cell_changed(old_pos: (i16, i16, i16), new_pos: (i16, i16, i16)) -> bool {
    let old_cell_x = (old_pos.0 as i32) >> CELL_SHIFT;
    let old_cell_y = (old_pos.1 as i32) >> CELL_SHIFT;
    let old_cell_z = (old_pos.2 as i32) >> CELL_SHIFT;

    let new_cell_x = (new_pos.0 as i32) >> CELL_SHIFT;
    let new_cell_y = (new_pos.1 as i32) >> CELL_SHIFT;
    let new_cell_z = (new_pos.2 as i32) >> CELL_SHIFT;

    old_cell_x != new_cell_x || old_cell_y != new_cell_y || old_cell_z != new_cell_z
}

/// Обнаружить столкновения токена с соседями
///
/// Аргументы:
/// - token_index: индекс проверяемого токена
/// - pos: позиция токена (x, y, z)
/// - collision_radius: радиус столкновения (в квантах)
/// - get_position: функция получения позиции по индексу
/// - grid: spatial hash grid для поиска соседей
///
/// Возвращает:
/// - Vec индексов токенов, с которыми произошло столкновение
///
/// Алгоритм:
/// 1. Найти всех соседей через spatial hash
/// 2. Отфильтровать по точному расстоянию <= collision_radius
/// 3. Исключить сам токен из результата
///
/// Используется для генерации событий TokenCollision
pub fn detect_collisions<F>(
    token_index: u32,
    pos: (i16, i16, i16),
    collision_radius: i16,
    get_position: F,
    grid: &SpatialHashGrid,
) -> Vec<u32>
where
    F: Fn(u32) -> (i16, i16, i16),
{
    let neighbors = grid.find_neighbors(pos.0, pos.1, pos.2, collision_radius, get_position);

    // Отфильтровать сам токен
    neighbors
        .into_iter()
        .filter(|&idx| idx != token_index)
        .collect()
}

/// Целочисленный квадратный корень (алгоритм Ньютона)
///
/// Вычисляет floor(sqrt(n)) для целых чисел без использования float
///
/// Алгоритм:
/// - Итеративное приближение через формулу Ньютона: x_new = (x + n/x) / 2
/// - Сходится за O(log(bits)) итераций
/// - Детерминистичен и точен для всех i64
#[inline]
pub fn integer_sqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    // Начальное приближение через битовый сдвиг
    // Находим позицию старшего бита и начинаем с sqrt(2^bits) = 2^(bits/2)
    let bits = 64 - n.leading_zeros() as i64;
    let mut x = 1i64 << ((bits + 1) / 2);

    // Итерация Ньютона до сходимости
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return x;
        }
        x = y;
    }
}

/// Запись в ячейке spatial hash grid
/// Хранит индекс токена и ссылку на следующую запись (linked list)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CellEntry {
    pub token_index: u32, // Индекс токена в массиве
    pub next: u32,        // Индекс следующей записи или u32::MAX (конец списка)
}

impl CellEntry {
    pub const NONE: u32 = u32::MAX;

    pub fn new(token_index: u32, next: u32) -> Self {
        Self { token_index, next }
    }

    pub fn has_next(&self) -> bool {
        self.next != Self::NONE
    }
}

/// Spatial Hash Grid - O(1) поиск соседей в 3D пространстве
///
/// Архитектура:
/// - bucket_heads[bucket_key] → индекс первой записи в корзине
/// - entries[index] → CellEntry с token_index и next
/// - entry_count → количество активных записей
///
/// Алгоритм:
/// 1. cell_key(x, y, z) → bucket_key через хеш-функцию
/// 2. bucket_heads[bucket_key] → начало linked list
/// 3. Итерация по цепочке через CellEntry.next
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SpatialHashGrid {
    pub bucket_heads: Vec<u32>,  // Головы корзин (BUCKET_COUNT элементов)
    pub entries: Vec<CellEntry>, // Массив записей (растёт по мере добавления)
    pub entry_count: usize,      // Количество активных записей
}

impl Default for SpatialHashGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialHashGrid {
    /// Создать пустой spatial hash grid
    pub fn new() -> Self {
        Self {
            bucket_heads: vec![CellEntry::NONE; BUCKET_COUNT],
            entries: Vec::with_capacity(4096), // Начальная ёмкость
            entry_count: 0,
        }
    }

    /// Очистить grid для перестройки
    pub fn clear(&mut self) {
        self.bucket_heads.fill(CellEntry::NONE);
        self.entries.clear();
        self.entry_count = 0;
    }

    /// Вычислить bucket_key для координат
    ///
    /// Детерминистичная хеш-функция:
    /// - cell_x = x >> CELL_SHIFT (деление на CELL_SIZE)
    /// - cell_y = y >> CELL_SHIFT
    /// - cell_z = z >> CELL_SHIFT
    /// - hash = (cell_x * P1 + cell_y * P2 + cell_z * P3) & BUCKET_MASK
    ///
    /// Простые числа для минимизации коллизий:
    /// P1 = 73856093, P2 = 19349663, P3 = 83492791
    pub fn cell_key(x: i16, y: i16, z: i16) -> u32 {
        // Преобразование в ячейки (деление на CELL_SIZE с сохранением знака)
        let cell_x = (x as i32) >> CELL_SHIFT;
        let cell_y = (y as i32) >> CELL_SHIFT;
        let cell_z = (z as i32) >> CELL_SHIFT;

        // Хеш-функция с простыми числами (wrapping для детерминизма)
        const P1: u32 = 73856093;
        const P2: u32 = 19349663;
        const P3: u32 = 83492791;

        let hash = (cell_x as u32)
            .wrapping_mul(P1)
            .wrapping_add((cell_y as u32).wrapping_mul(P2))
            .wrapping_add((cell_z as u32).wrapping_mul(P3));

        hash & BUCKET_MASK
    }

    /// Добавить токен в grid
    ///
    /// Алгоритм:
    /// 1. Вычислить bucket_key
    /// 2. Создать CellEntry с token_index
    /// 3. Вставить в начало linked list
    /// 4. Обновить bucket_heads[bucket_key]
    pub fn insert(&mut self, token_index: u32, x: i16, y: i16, z: i16) {
        let bucket_key = Self::cell_key(x, y, z) as usize;
        let old_head = self.bucket_heads[bucket_key];

        // Создать новую запись
        let entry = CellEntry::new(token_index, old_head);
        let entry_index = self.entries.len() as u32;
        self.entries.push(entry);

        // Обновить голову корзины
        self.bucket_heads[bucket_key] = entry_index;
        self.entry_count += 1;
    }

    /// Найти все токены в ячейке
    ///
    /// Возвращает итератор по индексам токенов в данной ячейке
    pub fn query_cell(&self, x: i16, y: i16, z: i16) -> CellIterator<'_> {
        let bucket_key = Self::cell_key(x, y, z) as usize;
        let head = self.bucket_heads[bucket_key];

        CellIterator {
            grid: self,
            current: if head == CellEntry::NONE {
                None
            } else {
                Some(head)
            },
        }
    }

    /// Полная перестройка spatial hash grid из массива токенов
    ///
    /// O(N) операция - проходит по всем токенам и перестраивает индекс
    ///
    /// Алгоритм:
    /// 1. Очистить текущий grid
    /// 2. Для каждого токена: insert(token_index, position)
    /// 3. Результат: полностью перестроенный индекс
    pub fn rebuild<F>(&mut self, token_count: usize, get_position: F)
    where
        F: Fn(usize) -> (i16, i16, i16),
    {
        self.clear();

        for token_index in 0..token_count {
            let (x, y, z) = get_position(token_index);
            self.insert(token_index as u32, x, y, z);
        }
    }

    /// Найти всех соседей токена в заданном радиусе
    ///
    /// Алгоритм:
    /// 1. Вычислить диапазон ячеек вокруг токена (radius в квантах)
    /// 2. Проверить все ячейки в диапазоне
    /// 3. Собрать токены из этих ячеек
    /// 4. Опционально: фильтровать по точному расстоянию
    ///
    /// ВНИМАНИЕ: radius в квантах координат, не в ячейках
    /// Например, radius=300 покроет ~2 ячейки (CELL_SIZE=256)
    pub fn find_neighbors<F>(
        &self,
        center_x: i16,
        center_y: i16,
        center_z: i16,
        radius: i16,
        get_position: F,
    ) -> Vec<u32>
    where
        F: Fn(u32) -> (i16, i16, i16),
    {
        let mut neighbors = Vec::new();

        // Вычислить границы поиска в квантах
        let min_x = center_x.saturating_sub(radius);
        let max_x = center_x.saturating_add(radius);
        let min_y = center_y.saturating_sub(radius);
        let max_y = center_y.saturating_add(radius);
        let min_z = center_z.saturating_sub(radius);
        let max_z = center_z.saturating_add(radius);

        // Вычислить диапазон ячеек
        let min_cell_x = (min_x as i32) >> CELL_SHIFT;
        let max_cell_x = (max_x as i32) >> CELL_SHIFT;
        let min_cell_y = (min_y as i32) >> CELL_SHIFT;
        let max_cell_y = (max_y as i32) >> CELL_SHIFT;
        let min_cell_z = (min_z as i32) >> CELL_SHIFT;
        let max_cell_z = (max_z as i32) >> CELL_SHIFT;

        // Проверить все ячейки в диапазоне
        for cell_x in min_cell_x..=max_cell_x {
            for cell_y in min_cell_y..=max_cell_y {
                for cell_z in min_cell_z..=max_cell_z {
                    // Вычислить координату центра ячейки
                    let cell_center_x = ((cell_x << CELL_SHIFT) + (CELL_SIZE / 2)) as i16;
                    let cell_center_y = ((cell_y << CELL_SHIFT) + (CELL_SIZE / 2)) as i16;
                    let cell_center_z = ((cell_z << CELL_SHIFT) + (CELL_SIZE / 2)) as i16;

                    // Собрать токены из этой ячейки
                    for token_index in self.query_cell(cell_center_x, cell_center_y, cell_center_z)
                    {
                        let (tx, ty, tz) = get_position(token_index);

                        // Точная проверка расстояния
                        let dx = (tx as i32) - (center_x as i32);
                        let dy = (ty as i32) - (center_y as i32);
                        let dz = (tz as i32) - (center_z as i32);
                        let dist2 = (dx * dx + dy * dy + dz * dz) as i64;
                        let radius2 = (radius as i64) * (radius as i64);

                        if dist2 <= radius2 {
                            neighbors.push(token_index);
                        }
                    }
                }
            }
        }

        neighbors
    }
}

/// Итератор по токенам в ячейке
pub struct CellIterator<'a> {
    grid: &'a SpatialHashGrid,
    current: Option<u32>,
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = u32; // token_index

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(entry_index) => {
                let entry = &self.grid.entries[entry_index as usize];
                let token_index = entry.token_index;

                // Переместиться к следующей записи
                self.current = if entry.has_next() {
                    Some(entry.next)
                } else {
                    None
                };

                Some(token_index)
            }
        }
    }
}

// ============================================================================
// SPATIAL CONFIG
// ============================================================================

/// Ошибка загрузки конфигурации пространства
#[derive(Debug)]
pub enum SpatialConfigError {
    /// Ошибка чтения файла
    IoError(String),
    /// Ошибка парсинга YAML
    ParseError(String),
    /// Некорректные параметры
    ValidationError(String),
}

impl std::fmt::Display for SpatialConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpatialConfigError::IoError(e) => write!(f, "IO error: {}", e),
            SpatialConfigError::ParseError(e) => write!(f, "Parse error: {}", e),
            SpatialConfigError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for SpatialConfigError {}

/// Конфигурация пространственного хеш-грида
///
/// Параметры определяют компромисс между гранулярностью поиска и памятью:
/// - `cell_shift`: размер ячейки = 1 << cell_shift (меньше → точнее, больше памяти)
/// - `bucket_count_log2`: количество корзин = 1 << bucket_count_log2
/// - `initial_capacity`: предварительная аллокация entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialConfig {
    /// Сдвиг для вычисления размера ячейки (cell_size = 1 << cell_shift)
    pub cell_shift: u32,
    /// Логарифм числа корзин (bucket_count = 1 << bucket_count_log2)
    pub bucket_count_log2: u32,
    /// Начальная ёмкость массива entries
    pub initial_capacity: u32,
}

impl SpatialConfig {
    /// Tight: мелкие ячейки, большая таблица — точный поиск, больше памяти
    ///
    /// cell_size = 64, buckets = 131072
    pub fn tight() -> Self {
        Self {
            cell_shift: 6,
            bucket_count_log2: 17,
            initial_capacity: 8192,
        }
    }

    /// Medium: баланс точности и памяти (параметры по умолчанию)
    ///
    /// cell_size = 256, buckets = 65536
    pub fn medium() -> Self {
        Self {
            cell_shift: CELL_SHIFT,
            bucket_count_log2: BUCKET_COUNT_LOG2,
            initial_capacity: 4096,
        }
    }

    /// Loose: крупные ячейки, меньше памяти — грубый поиск, быстрее rebuild
    ///
    /// cell_size = 1024, buckets = 16384
    pub fn loose() -> Self {
        Self {
            cell_shift: 10,
            bucket_count_log2: 14,
            initial_capacity: 2048,
        }
    }

    /// Загрузить конфигурацию из YAML файла
    pub fn from_yaml(path: &std::path::Path) -> Result<Self, SpatialConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SpatialConfigError::IoError(e.to_string()))?;
        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| SpatialConfigError::ParseError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Проверить корректность параметров
    pub fn validate(&self) -> Result<(), SpatialConfigError> {
        if self.cell_shift == 0 || self.cell_shift > 15 {
            return Err(SpatialConfigError::ValidationError(format!(
                "cell_shift must be 1..15, got {}",
                self.cell_shift
            )));
        }
        if self.bucket_count_log2 < 8 || self.bucket_count_log2 > 24 {
            return Err(SpatialConfigError::ValidationError(format!(
                "bucket_count_log2 must be 8..24, got {}",
                self.bucket_count_log2
            )));
        }
        if self.initial_capacity == 0 {
            return Err(SpatialConfigError::ValidationError(
                "initial_capacity must be > 0".to_string(),
            ));
        }
        Ok(())
    }

    /// Размер ячейки в квантах
    pub fn cell_size(&self) -> u32 {
        1 << self.cell_shift
    }

    /// Количество корзин
    pub fn bucket_count(&self) -> usize {
        1 << self.bucket_count_log2
    }
}

impl SpatialHashGrid {
    /// Создать grid с заданной конфигурацией
    ///
    /// В отличие от `new()`, использует параметры из `SpatialConfig`
    /// вместо глобальных констант.
    pub fn with_config(config: &SpatialConfig) -> Self {
        Self {
            bucket_heads: vec![CellEntry::NONE; config.bucket_count()],
            entries: Vec::with_capacity(config.initial_capacity as usize),
            entry_count: 0,
        }
    }
}
