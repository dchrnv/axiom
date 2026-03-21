// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// SPACE V6.0: docs/spec/SPACE_V6_0.md
// Целочисленная пространственная модель с детерминистичной физикой

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
/// let pos = (100, 200, 300);
/// let vel = (10, -5, 0);
/// let new_pos = apply_velocity(pos, vel);
/// // Результат: (110, 195, 300)
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
/// let vel = (100, -200, 50);
/// let new_vel = apply_friction(vel, 4); // Коэффициент 1/16
/// // Результат: vel - vel/16 = (94, -187, 47)
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
/// let vel = (100, -50, 0);
/// let acc = (10, 5, -3);
/// let new_vel = apply_acceleration(vel, acc);
/// // Результат: (110, -45, -3)
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
fn integer_sqrt(n: i64) -> i64 {
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
    pub token_index: u32,  // Индекс токена в массиве
    pub next: u32,         // Индекс следующей записи или u32::MAX (конец списка)
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
    pub bucket_heads: Vec<u32>,      // Головы корзин (BUCKET_COUNT элементов)
    pub entries: Vec<CellEntry>,     // Массив записей (растёт по мере добавления)
    pub entry_count: usize,          // Количество активных записей
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

        let hash = (cell_x as u32).wrapping_mul(P1)
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
            current: if head == CellEntry::NONE { None } else { Some(head) },
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
                    for token_index in self.query_cell(cell_center_x, cell_center_y, cell_center_z) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Проверка констант
        assert_eq!(CELL_SIZE, 256);
        assert_eq!(BUCKET_COUNT, 65536);
        assert_eq!(BUCKET_MASK, 65535);
    }

    #[test]
    fn test_cell_entry() {
        let entry = CellEntry::new(42, 100);
        assert_eq!(entry.token_index, 42);
        assert_eq!(entry.next, 100);
        assert!(entry.has_next());

        let terminal = CellEntry::new(99, CellEntry::NONE);
        assert!(!terminal.has_next());
    }

    #[test]
    fn test_cell_key_determinism() {
        // Одинаковые координаты → одинаковый ключ
        let key1 = SpatialHashGrid::cell_key(100, 200, 300);
        let key2 = SpatialHashGrid::cell_key(100, 200, 300);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cell_key_same_cell() {
        // Координаты в пределах одной ячейки → одинаковый ключ
        // CELL_SIZE = 256, поэтому [0..255] → cell_x = 0
        let key1 = SpatialHashGrid::cell_key(0, 0, 0);
        let key2 = SpatialHashGrid::cell_key(100, 100, 100);
        let key3 = SpatialHashGrid::cell_key(255, 255, 255);
        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
    }

    #[test]
    fn test_cell_key_different_cells() {
        // Координаты в разных ячейках → разные ключи
        let key1 = SpatialHashGrid::cell_key(0, 0, 0);
        let key2 = SpatialHashGrid::cell_key(256, 0, 0); // Следующая ячейка по X
        let key3 = SpatialHashGrid::cell_key(0, 256, 0); // Следующая ячейка по Y
        assert_ne!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key2, key3);
    }

    #[test]
    fn test_cell_key_negative() {
        // Отрицательные координаты корректно хешируются
        let key_pos = SpatialHashGrid::cell_key(100, 100, 100);
        let key_neg = SpatialHashGrid::cell_key(-100, -100, -100);
        assert_ne!(key_pos, key_neg);

        // Детерминизм для отрицательных
        let key1 = SpatialHashGrid::cell_key(-500, -600, -700);
        let key2 = SpatialHashGrid::cell_key(-500, -600, -700);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_spatial_hash_grid_insert() {
        let mut grid = SpatialHashGrid::new();

        // Добавить токены
        grid.insert(0, 100, 100, 100);
        grid.insert(1, 150, 150, 150);
        grid.insert(2, 500, 500, 500);

        assert_eq!(grid.entry_count, 3);
        assert_eq!(grid.entries.len(), 3);
    }

    #[test]
    fn test_spatial_hash_grid_query_empty() {
        let grid = SpatialHashGrid::new();

        // Запрос к пустому grid
        let tokens: Vec<u32> = grid.query_cell(0, 0, 0).collect();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_spatial_hash_grid_query_single() {
        let mut grid = SpatialHashGrid::new();
        grid.insert(42, 100, 100, 100);

        // Запрос к той же ячейке
        let tokens: Vec<u32> = grid.query_cell(100, 100, 100).collect();
        assert_eq!(tokens, vec![42]);

        // Запрос к другой ячейке
        let tokens: Vec<u32> = grid.query_cell(500, 500, 500).collect();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_spatial_hash_grid_query_multiple() {
        let mut grid = SpatialHashGrid::new();

        // Добавить несколько токенов в одну ячейку
        grid.insert(10, 100, 100, 100);
        grid.insert(20, 150, 150, 150); // Та же ячейка (в пределах 256)
        grid.insert(30, 200, 200, 200); // Та же ячейка

        let tokens: Vec<u32> = grid.query_cell(100, 100, 100).collect();
        assert_eq!(tokens.len(), 3);

        // Порядок обратный (LIFO - последний добавленный первый)
        assert_eq!(tokens, vec![30, 20, 10]);
    }

    #[test]
    fn test_spatial_hash_grid_clear() {
        let mut grid = SpatialHashGrid::new();
        grid.insert(0, 100, 100, 100);
        grid.insert(1, 200, 200, 200);

        assert_eq!(grid.entry_count, 2);

        grid.clear();

        assert_eq!(grid.entry_count, 0);
        assert_eq!(grid.entries.len(), 0);

        // Запросы возвращают пустоту
        let tokens: Vec<u32> = grid.query_cell(100, 100, 100).collect();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_cell_key_distribution() {
        // Проверка равномерности распределения
        use std::collections::HashSet;

        let mut keys = HashSet::new();

        // Генерировать ключи для разных координат
        for x in (-10..10).step_by(2) {
            for y in (-10..10).step_by(2) {
                for z in (-10..10).step_by(2) {
                    let key = SpatialHashGrid::cell_key(
                        (x * 500) as i16,
                        (y * 500) as i16,
                        (z * 500) as i16,
                    );
                    keys.insert(key);
                }
            }
        }

        // Должно быть много уникальных ключей (хорошее распределение)
        assert!(keys.len() > 900); // 10*10*10 = 1000 точек, минимум коллизий
    }

    #[test]
    fn test_rebuild() {
        // Создать массив позиций токенов
        let positions = vec![
            (100i16, 100i16, 100i16),
            (500, 500, 500),
            (1000, 1000, 1000),
            (150, 150, 150), // Та же ячейка что и token 0
            (-500, -500, -500),
        ];

        let get_position = |index: usize| positions[index];

        let mut grid = SpatialHashGrid::new();

        // Перестроить индекс
        grid.rebuild(positions.len(), get_position);

        assert_eq!(grid.entry_count, 5);
        assert_eq!(grid.entries.len(), 5);

        // Проверить что токены 0 и 3 в одной ячейке
        let tokens: Vec<u32> = grid.query_cell(100, 100, 100).collect();
        assert_eq!(tokens.len(), 2);
        assert!(tokens.contains(&0));
        assert!(tokens.contains(&3));

        // Проверить изолированные токены
        let tokens: Vec<u32> = grid.query_cell(500, 500, 500).collect();
        assert_eq!(tokens, vec![1]);
    }

    #[test]
    fn test_rebuild_overwrite() {
        let positions1 = vec![(100i16, 100i16, 100i16), (200, 200, 200)];
        let positions2 = vec![(500i16, 500i16, 500i16)];

        let mut grid = SpatialHashGrid::new();

        // Первая перестройка
        grid.rebuild(positions1.len(), |i| positions1[i]);
        assert_eq!(grid.entry_count, 2);

        // Вторая перестройка - должна заменить первую
        grid.rebuild(positions2.len(), |i| positions2[i]);
        assert_eq!(grid.entry_count, 1);

        // Старые токены не должны быть найдены
        let tokens: Vec<u32> = grid.query_cell(100, 100, 100).collect();
        assert_eq!(tokens.len(), 0);

        // Новый токен должен быть найден
        let tokens: Vec<u32> = grid.query_cell(500, 500, 500).collect();
        assert_eq!(tokens, vec![0]);
    }

    #[test]
    fn test_find_neighbors_empty() {
        let grid = SpatialHashGrid::new();
        let get_position = |_: u32| (0i16, 0i16, 0i16);

        let neighbors = grid.find_neighbors(0, 0, 0, 100, get_position);
        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn test_find_neighbors_single_in_range() {
        let mut grid = SpatialHashGrid::new();
        let positions = vec![(100i16, 100i16, 100i16)];

        grid.rebuild(positions.len(), |i| positions[i]);

        let get_position = |index: u32| positions[index as usize];

        // Поиск вокруг (100, 100, 100) с радиусом 50
        let neighbors = grid.find_neighbors(100, 100, 100, 50, get_position);
        assert_eq!(neighbors, vec![0]);

        // Поиск вокруг (200, 200, 200) - токен вне радиуса
        let neighbors = grid.find_neighbors(200, 200, 200, 50, get_position);
        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn test_find_neighbors_multiple() {
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (150, 150, 150), // Расстояние ~86 от первого
            (500, 500, 500), // Далеко
            (110, 110, 110), // Близко к первому ~17
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        // Радиус 100 - должен захватить токены 0, 1, 3
        let neighbors = grid.find_neighbors(100, 100, 100, 100, get_position);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&3));
        assert!(!neighbors.contains(&2)); // Токен 2 слишком далеко

        // Радиус 20 - только токены 0 и 3
        let neighbors = grid.find_neighbors(100, 100, 100, 20, get_position);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&3));
    }

    #[test]
    fn test_find_neighbors_boundary() {
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (0i16, 0i16, 0i16),
            (100, 0, 0),   // Расстояние ровно 100
            (0, 100, 0),   // Расстояние ровно 100
            (0, 0, 100),   // Расстояние ровно 100
            (101, 0, 0),   // Расстояние 101 - вне радиуса
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        // Радиус ровно 100 - должен захватить токены 0,1,2,3 (но не 4)
        let neighbors = grid.find_neighbors(0, 0, 0, 100, get_position);
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
        assert!(!neighbors.contains(&4));
    }

    #[test]
    fn test_find_neighbors_negative_coords() {
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (-100i16, -100i16, -100i16),
            (-150, -150, -150),
            (100, 100, 100), // Далеко
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        // Поиск в отрицательных координатах
        let neighbors = grid.find_neighbors(-100, -100, -100, 100, get_position);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&1));
        assert!(!neighbors.contains(&2));
    }

    // --- Distance2 Tests ---

    #[test]
    fn test_distance2_zero() {
        // Расстояние от точки до самой себя
        let d2 = distance2(100, 200, 300, 100, 200, 300);
        assert_eq!(d2, 0);
    }

    #[test]
    fn test_distance2_axis_aligned() {
        // Движение вдоль одной оси
        assert_eq!(distance2(0, 0, 0, 100, 0, 0), 10000); // dx=100 → 100²
        assert_eq!(distance2(0, 0, 0, 0, 200, 0), 40000); // dy=200 → 200²
        assert_eq!(distance2(0, 0, 0, 0, 0, 300), 90000); // dz=300 → 300²
    }

    #[test]
    fn test_distance2_diagonal() {
        // Диагональное движение
        // (0,0,0) → (100,100,100): dist² = 100² + 100² + 100² = 30000
        let d2 = distance2(0, 0, 0, 100, 100, 100);
        assert_eq!(d2, 30000);

        // (100,100,100) → (200,200,200): dist² = 100² + 100² + 100² = 30000
        let d2 = distance2(100, 100, 100, 200, 200, 200);
        assert_eq!(d2, 30000);
    }

    #[test]
    fn test_distance2_negative() {
        // Отрицательные координаты
        let d2 = distance2(-100, -100, -100, 0, 0, 0);
        assert_eq!(d2, 30000); // 100² + 100² + 100²

        let d2 = distance2(-500, -500, -500, -400, -400, -400);
        assert_eq!(d2, 30000); // 100² + 100² + 100²
    }

    #[test]
    fn test_distance2_symmetry() {
        // Симметрия: dist(A,B) = dist(B,A)
        let d1 = distance2(100, 200, 300, 400, 500, 600);
        let d2 = distance2(400, 500, 600, 100, 200, 300);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_distance2_max_coords() {
        // Максимальные координаты i16
        let max = i16::MAX; // 32767
        let min = i16::MIN; // -32768

        // Расстояние от min до max по одной оси
        let d2 = distance2(min, 0, 0, max, 0, 0);
        // Разность: 32767 - (-32768) = 65535
        // Квадрат: 65535² = 4_294_836_225
        assert_eq!(d2, 4_294_836_225);

        // Диагональ от угла до угла (максимальное расстояние)
        let d2 = distance2(min, min, min, max, max, max);
        // Разность по каждой оси: 65535
        // Сумма квадратов: 3 * 65535² = 12_884_508_675
        assert_eq!(d2, 12_884_508_675);
    }

    #[test]
    fn test_distance2_to_anchor_zero() {
        let d2 = distance2_to_anchor(0, 0, 0);
        assert_eq!(d2, 0);
    }

    #[test]
    fn test_distance2_to_anchor_positive() {
        let d2 = distance2_to_anchor(100, 200, 300);
        // 100² + 200² + 300² = 10000 + 40000 + 90000 = 140000
        assert_eq!(d2, 140000);
    }

    #[test]
    fn test_distance2_to_anchor_negative() {
        // Отрицательные координаты - квадраты всегда положительные
        let d2 = distance2_to_anchor(-100, -200, -300);
        assert_eq!(d2, 140000); // То же что и (100,200,300)
    }

    #[test]
    fn test_distance2_to_anchor_vs_distance2() {
        // distance2_to_anchor должен давать тот же результат что distance2 до (0,0,0)
        let x = 123i16;
        let y = 456i16;
        let z = 789i16;

        let d1 = distance2(0, 0, 0, x, y, z);
        let d2 = distance2_to_anchor(x, y, z);

        assert_eq!(d1, d2);
    }

    #[test]
    fn test_distance2_determinism() {
        // Детерминизм: одинаковые входы → одинаковые выходы
        let coords = [(100i16, 200i16, 300i16), (-500, -600, -700)];

        for _ in 0..10 {
            for &(x1, y1, z1) in &coords {
                for &(x2, y2, z2) in &coords {
                    let d1 = distance2(x1, y1, z1, x2, y2, z2);
                    let d2 = distance2(x1, y1, z1, x2, y2, z2);
                    assert_eq!(d1, d2, "Determinism violated");
                }
            }
        }
    }

    #[test]
    fn test_distance2_overflow_safe() {
        // Проверка что нет overflow даже для максимальных значений
        let max = i16::MAX;
        let min = i16::MIN;

        // Эти вычисления не должны паниковать
        let _ = distance2(min, min, min, max, max, max);
        let _ = distance2(0, 0, 0, max, max, max);
        let _ = distance2(min, min, min, 0, 0, 0);
        let _ = distance2_to_anchor(max, max, max);
        let _ = distance2_to_anchor(min, min, min);
    }

    // ========================================================================
    // GRAVITY TESTS
    // ========================================================================

    #[test]
    fn test_gravity_at_anchor() {
        // Токен точно в якоре - нет силы
        let (ax, ay, az) = compute_gravity(0, 0, 0, 1000, 24, GravityModel::Linear);
        assert_eq!((ax, ay, az), (0, 0, 0));

        let (ax, ay, az) = compute_gravity(0, 0, 0, 1000, 24, GravityModel::InverseSquare);
        assert_eq!((ax, ay, az), (0, 0, 0));
    }

    #[test]
    fn test_gravity_linear_direction() {
        // Линейная модель: направление к якорю (0, 0, 0)
        // Используем меньший scale_shift для детектируемой силы
        // Токен справа от якоря → ускорение влево (отрицательное)
        let (ax, ay, az) = compute_gravity(1000, 0, 0, 1000, 8, GravityModel::Linear);
        assert!(ax < 0, "Ускорение должно быть направлено к якорю (got ax={})", ax);
        assert_eq!(ay, 0);
        assert_eq!(az, 0);

        // Токен слева от якоря → ускорение вправо (положительное)
        let (ax, ay, az) = compute_gravity(-1000, 0, 0, 1000, 8, GravityModel::Linear);
        assert!(ax > 0, "Ускорение должно быть направлено к якорю (got ax={})", ax);
        assert_eq!(ay, 0);
        assert_eq!(az, 0);

        // Токен сверху от якоря → ускорение вниз (отрицательное по y)
        let (ax, ay, az) = compute_gravity(0, 1000, 0, 1000, 8, GravityModel::Linear);
        assert_eq!(ax, 0);
        assert!(ay < 0, "Ускорение должно быть направлено к якорю (got ay={})", ay);
        assert_eq!(az, 0);
    }

    #[test]
    fn test_gravity_linear_magnitude() {
        // Линейная модель: F = k * distance
        // Чем дальше от якоря, тем сильнее притяжение

        // Близко к якорю (100 квантов)
        let (ax1, _, _) = compute_gravity(100, 0, 0, 1000, 6, GravityModel::Linear);

        // Далеко от якоря (1000 квантов)
        let (ax2, _, _) = compute_gravity(1000, 0, 0, 1000, 6, GravityModel::Linear);

        // Дальнее расстояние → больше сила (по модулю)
        let ax1_abs = if ax1 < 0 { -ax1 } else { ax1 };
        let ax2_abs = if ax2 < 0 { -ax2 } else { ax2 };

        assert!(
            ax2_abs >= ax1_abs,
            "Дальше от якоря → сильнее притяжение (линейная модель): ax1={}, ax2={}",
            ax1,
            ax2
        );
    }

    #[test]
    fn test_gravity_inverse_square_direction() {
        // Inverse square модель: направление к якорю
        let (ax, ay, az) = compute_gravity(1000, 0, 0, 1000, 20, GravityModel::InverseSquare);
        assert!(ax < 0, "Ускорение должно быть направлено к якорю");
        assert_eq!(ay, 0);
        assert_eq!(az, 0);

        let (ax, ay, az) = compute_gravity(-1000, 0, 0, 1000, 20, GravityModel::InverseSquare);
        assert!(ax > 0, "Ускорение должно быть направлено к якорю");
        assert_eq!(ay, 0);
        assert_eq!(az, 0);
    }

    #[test]
    fn test_gravity_inverse_square_magnitude() {
        // Inverse square модель: F = k * mass / distance²
        // Чем БЛИЖЕ к якорю, тем сильнее притяжение

        // Близко к якорю (100 квантов)
        let (ax1, _, _) = compute_gravity(100, 0, 0, 1000, 20, GravityModel::InverseSquare);

        // Далеко от якоря (1000 квантов)
        let (ax2, _, _) = compute_gravity(1000, 0, 0, 1000, 20, GravityModel::InverseSquare);

        // Ближнее расстояние → больше сила (по модулю)
        // Используем безопасное вычисление abs для i16
        let ax1_abs = if ax1 == i16::MIN {
            i16::MAX as i32 + 1
        } else if ax1 < 0 {
            (-ax1) as i32
        } else {
            ax1 as i32
        };

        let ax2_abs = if ax2 == i16::MIN {
            i16::MAX as i32 + 1
        } else if ax2 < 0 {
            (-ax2) as i32
        } else {
            ax2 as i32
        };

        assert!(
            ax1_abs > ax2_abs,
            "Ближе к якорю → сильнее притяжение (inverse square модель): ax1={}, ax2={}",
            ax1,
            ax2
        );
    }

    #[test]
    fn test_gravity_mass_effect() {
        // Масса влияет на силу (в inverse square модели)
        let (ax_light, _, _) = compute_gravity(1000, 0, 0, 100, 20, GravityModel::InverseSquare);
        let (ax_heavy, _, _) = compute_gravity(1000, 0, 0, 10000, 20, GravityModel::InverseSquare);

        // Большая масса → больше ускорение
        assert!(
            ax_heavy.abs() > ax_light.abs(),
            "Большая масса → больше притяжение"
        );
    }

    #[test]
    fn test_gravity_scale_shift() {
        // gravity_scale_shift управляет силой гравитации
        // Большой shift → слабая гравитация
        let (ax_weak, _, _) = compute_gravity(1000, 0, 0, 1000, 12, GravityModel::Linear);
        let (ax_strong, _, _) = compute_gravity(1000, 0, 0, 1000, 4, GravityModel::Linear);

        // Меньший shift → больше сила
        let ax_weak_abs = if ax_weak < 0 { -ax_weak } else { ax_weak };
        let ax_strong_abs = if ax_strong < 0 { -ax_strong } else { ax_strong };

        assert!(
            ax_strong_abs > ax_weak_abs,
            "Меньший scale_shift → сильнее гравитация: strong={}, weak={}",
            ax_strong,
            ax_weak
        );
    }

    #[test]
    fn test_gravity_determinism() {
        // Детерминизм: одинаковые входы → одинаковые выходы
        let coords = [
            (100i16, 200i16, 300i16),
            (-500, -600, -700),
            (1000, 0, 0),
            (0, 1000, 0),
        ];

        for &(x, y, z) in &coords {
            for _ in 0..10 {
                let r1 = compute_gravity(x, y, z, 1000, 24, GravityModel::Linear);
                let r2 = compute_gravity(x, y, z, 1000, 24, GravityModel::Linear);
                assert_eq!(r1, r2, "Determinism violated (Linear)");

                let r1 = compute_gravity(x, y, z, 1000, 20, GravityModel::InverseSquare);
                let r2 = compute_gravity(x, y, z, 1000, 20, GravityModel::InverseSquare);
                assert_eq!(r1, r2, "Determinism violated (InverseSquare)");
            }
        }
    }

    #[test]
    fn test_gravity_no_overflow() {
        // Проверка что нет overflow даже для экстремальных значений
        let max = i16::MAX;
        let min = i16::MIN;

        // Эти вычисления не должны паниковать
        let _ = compute_gravity(max, max, max, u16::MAX, 24, GravityModel::Linear);
        let _ = compute_gravity(min, min, min, u16::MAX, 24, GravityModel::Linear);
        let _ = compute_gravity(max, max, max, u16::MAX, 20, GravityModel::InverseSquare);
        let _ = compute_gravity(min, min, min, u16::MAX, 20, GravityModel::InverseSquare);

        // Проверка с минимальным scale_shift
        let _ = compute_gravity(max, 0, 0, u16::MAX, 0, GravityModel::Linear);
        let _ = compute_gravity(max, 0, 0, u16::MAX, 0, GravityModel::InverseSquare);
    }

    #[test]
    fn test_gravity_diagonal() {
        // Проверка для диагональных направлений
        // Токен на диагонали (1000, 1000, 1000)
        // Используем меньший scale_shift для детектируемой силы
        let (ax, ay, az) = compute_gravity(1000, 1000, 1000, 1000, 8, GravityModel::Linear);

        // Все компоненты должны быть направлены к якорю
        assert!(ax < 0, "ax должен быть направлен к якорю (got {})", ax);
        assert!(ay < 0, "ay должен быть направлен к якорю (got {})", ay);
        assert!(az < 0, "az должен быть направлен к якорю (got {})", az);

        // Компоненты должны быть примерно равны (симметрия)
        let ax_abs = if ax < 0 { -ax } else { ax };
        let ay_abs = if ay < 0 { -ay } else { ay };
        let az_abs = if az < 0 { -az } else { az };

        let diff_xy = if ax_abs > ay_abs {
            ax_abs - ay_abs
        } else {
            ay_abs - ax_abs
        };
        let diff_xz = if ax_abs > az_abs {
            ax_abs - az_abs
        } else {
            az_abs - ax_abs
        };

        assert!(
            diff_xy <= 2 && diff_xz <= 2,
            "Диагональное направление должно иметь равные компоненты (ax={}, ay={}, az={})",
            ax,
            ay,
            az
        );
    }

    #[test]
    fn test_integer_sqrt() {
        // Тестирование целочисленного квадратного корня
        assert_eq!(integer_sqrt(0), 0);
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(9), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(100), 10);
        assert_eq!(integer_sqrt(10000), 100);

        // Проверка для неполных квадратов (должен вернуть floor)
        assert_eq!(integer_sqrt(5), 2); // floor(sqrt(5)) = 2
        assert_eq!(integer_sqrt(8), 2); // floor(sqrt(8)) = 2
        assert_eq!(integer_sqrt(15), 3); // floor(sqrt(15)) = 3

        // Большие значения
        assert_eq!(integer_sqrt(1000000), 1000);
        assert_eq!(integer_sqrt(i64::MAX), 3037000499); // sqrt(2^63 - 1)
    }

    // ========================================================================
    // INERTIA AND MOTION TESTS
    // ========================================================================

    #[test]
    fn test_apply_velocity_basic() {
        // Базовое применение скорости
        let pos = (100i16, 200i16, 300i16);
        let vel = (10i16, -5i16, 0i16);
        let new_pos = apply_velocity(pos, vel);

        assert_eq!(new_pos, (110, 195, 300));
    }

    #[test]
    fn test_apply_velocity_zero() {
        // Нулевая скорость - позиция не меняется
        let pos = (100i16, 200i16, 300i16);
        let vel = (0i16, 0i16, 0i16);
        let new_pos = apply_velocity(pos, vel);

        assert_eq!(new_pos, pos);
    }

    #[test]
    fn test_apply_velocity_negative() {
        // Отрицательная скорость
        let pos = (0i16, 0i16, 0i16);
        let vel = (-50i16, -100i16, -150i16);
        let new_pos = apply_velocity(pos, vel);

        assert_eq!(new_pos, (-50, -100, -150));
    }

    #[test]
    fn test_apply_velocity_overflow() {
        // Проверка saturating операций при переполнении
        let pos = (i16::MAX - 10, i16::MIN + 10, 0);
        let vel = (100, -100, 0);

        let new_pos = apply_velocity(pos, vel);

        // Должно быть i16::MAX и i16::MIN (saturating)
        assert_eq!(new_pos, (i16::MAX, i16::MIN, 0));
    }

    #[test]
    fn test_apply_friction_basic() {
        // Базовое применение трения
        let vel = (100i16, -200i16, 50i16);
        let new_vel = apply_friction(vel, 4); // Коэффициент 1/16

        // vel - vel/16 с учётом битового сдвига
        // Положительные: 100 >> 4 = 6, 100 - 6 = 94
        // Отрицательные: -200 >> 4 = -13 (округление вниз), -200 - (-13) = -187
        // Положительные: 50 >> 4 = 3, 50 - 3 = 47
        assert_eq!(new_vel.0, 94);
        assert_eq!(new_vel.1, -187);
        assert_eq!(new_vel.2, 47);
    }

    #[test]
    fn test_apply_friction_zero_velocity() {
        // Нулевая скорость остаётся нулевой
        let vel = (0i16, 0i16, 0i16);
        let new_vel = apply_friction(vel, 8);

        assert_eq!(new_vel, (0, 0, 0));
    }

    #[test]
    fn test_apply_friction_strong() {
        // Сильное трение (малый shift)
        let vel = (100i16, -100i16, 100i16);
        let new_vel = apply_friction(vel, 2); // Коэффициент 1/4

        // vel - vel/4 = 100 - 25 = 75, -100 - (-25) = -75
        assert_eq!(new_vel, (75, -75, 75));
    }

    #[test]
    fn test_apply_friction_weak() {
        // Слабое трение (большой shift)
        let vel = (100i16, -100i16, 100i16);
        let new_vel = apply_friction(vel, 16); // Практически не влияет

        assert_eq!(new_vel, vel); // Не изменилось
    }

    #[test]
    fn test_apply_friction_convergence() {
        // Трение постепенно уменьшает скорость к нулю
        let mut vel = (1000i16, -1000i16, 1000i16);

        for _ in 0..50 {
            vel = apply_friction(vel, 3); // Коэффициент 1/8
        }

        // После многих итераций скорость должна стать близка к нулю
        assert!(vel.0.abs() < 10);
        assert!(vel.1.abs() < 10);
        assert!(vel.2.abs() < 10);
    }

    #[test]
    fn test_apply_acceleration_basic() {
        // Базовое применение ускорения
        let vel = (100i16, -50i16, 0i16);
        let acc = (10i16, 5i16, -3i16);
        let new_vel = apply_acceleration(vel, acc);

        assert_eq!(new_vel, (110, -45, -3));
    }

    #[test]
    fn test_apply_acceleration_zero() {
        // Нулевое ускорение - скорость не меняется
        let vel = (100i16, -50i16, 0i16);
        let acc = (0i16, 0i16, 0i16);
        let new_vel = apply_acceleration(vel, acc);

        assert_eq!(new_vel, vel);
    }

    #[test]
    fn test_apply_acceleration_overflow() {
        // Проверка saturating операций при переполнении
        let vel = (i16::MAX - 10, i16::MIN + 10, 0);
        let acc = (100, -100, 0);

        let new_vel = apply_acceleration(vel, acc);

        // Должно быть i16::MAX и i16::MIN (saturating)
        assert_eq!(new_vel, (i16::MAX, i16::MIN, 0));
    }

    #[test]
    fn test_apply_acceleration_deceleration() {
        // Ускорение в противоположном направлении (торможение)
        let vel = (100i16, 100i16, 100i16);
        let acc = (-10i16, -10i16, -10i16);

        let new_vel = apply_acceleration(vel, acc);

        assert_eq!(new_vel, (90, 90, 90));
    }

    #[test]
    fn test_clamp_i16_within_range() {
        // Значения в пределах i16
        assert_eq!(clamp_i16(0), 0);
        assert_eq!(clamp_i16(100), 100);
        assert_eq!(clamp_i16(-100), -100);
        assert_eq!(clamp_i16(32767), 32767);
        assert_eq!(clamp_i16(-32768), -32768);
    }

    #[test]
    fn test_clamp_i16_overflow() {
        // Значения вне пределов i16
        assert_eq!(clamp_i16(50000), i16::MAX);
        assert_eq!(clamp_i16(-50000), i16::MIN);
        assert_eq!(clamp_i16(i32::MAX), i16::MAX);
        assert_eq!(clamp_i16(i32::MIN), i16::MIN);
    }

    #[test]
    fn test_move_towards_basic() {
        // Базовое движение к цели
        let pos = (100i16, 100i16, 100i16);
        let target = (200i16, 100i16, 100i16);
        let acc = move_towards(pos, target, 4); // Сила = distance / 16

        // (200-100)/16 = 100/16 = 6
        assert_eq!(acc, (6, 0, 0));
    }

    #[test]
    fn test_move_towards_at_target() {
        // Уже в цели - нет ускорения
        let pos = (100i16, 100i16, 100i16);
        let target = pos;
        let acc = move_towards(pos, target, 4);

        assert_eq!(acc, (0, 0, 0));
    }

    #[test]
    fn test_move_towards_negative_direction() {
        // Цель в отрицательном направлении
        let pos = (100i16, 100i16, 100i16);
        let target = (0i16, 50i16, 150i16);
        let acc = move_towards(pos, target, 3); // Сила = distance / 8

        // Битовый сдвиг для отрицательных округляет вниз:
        // (0-100) >> 3 = -100 >> 3 = -13 (не -12)
        // (50-100) >> 3 = -50 >> 3 = -7 (не -6)
        // (150-100) >> 3 = 50 >> 3 = 6
        assert_eq!(acc, (-13, -7, 6));
    }

    #[test]
    fn test_move_towards_weak_attraction() {
        // Слабое притяжение (большой shift)
        let pos = (100i16, 100i16, 100i16);
        let target = (200i16, 100i16, 100i16);
        let acc = move_towards(pos, target, 16); // Очень слабое

        assert_eq!(acc, (0, 0, 0)); // Слишком слабо
    }

    #[test]
    fn test_move_towards_strong_attraction() {
        // Сильное притяжение (малый shift)
        let pos = (100i16, 100i16, 100i16);
        let target = (200i16, 100i16, 100i16);
        let acc = move_towards(pos, target, 0); // Максимальная сила

        // (200-100)/1 = 100
        assert_eq!(acc, (100, 0, 0));
    }

    #[test]
    fn test_move_towards_diagonal() {
        // Диагональное движение
        let pos = (0i16, 0i16, 0i16);
        let target = (100i16, 100i16, 100i16);
        let acc = move_towards(pos, target, 2); // Сила = distance / 4

        // 100/4 = 25 для каждой оси
        assert_eq!(acc, (25, 25, 25));
    }

    #[test]
    fn test_motion_simulation() {
        // Симуляция движения с гравитацией, трением и инерцией
        let mut pos = (1000i16, 0i16, 0i16);
        let mut vel = (0i16, 0i16, 0i16);

        // 10 шагов симуляции
        for _ in 0..10 {
            // 1. Вычислить гравитацию к якорю
            let gravity = compute_gravity(pos.0, pos.1, pos.2, 1000, 8, GravityModel::Linear);

            // 2. Применить ускорение от гравитации
            vel = apply_acceleration(vel, gravity);

            // 3. Применить трение
            vel = apply_friction(vel, 6);

            // 4. Применить скорость к позиции
            pos = apply_velocity(pos, vel);
        }

        // Токен должен сдвинуться в направлении якоря (0,0,0)
        assert!(pos.0 < 1000, "Токен должен двигаться к якорю: pos.x={}", pos.0);
    }

    #[test]
    fn test_motion_with_target() {
        // Симуляция движения к целевой точке
        let mut pos = (0i16, 0i16, 0i16);
        let mut vel = (0i16, 0i16, 0i16);
        let target = (500i16, 0i16, 0i16);

        let start_x = pos.0;

        // 20 шагов симуляции
        for _ in 0..20 {
            // 1. Вычислить ускорение к цели
            let acc = move_towards(pos, target, 5);

            // 2. Применить ускорение
            vel = apply_acceleration(vel, acc);

            // 3. Применить трение
            vel = apply_friction(vel, 6);

            // 4. Применить скорость
            pos = apply_velocity(pos, vel);
        }

        // Токен должен значительно приблизиться к цели
        assert!(
            pos.0 > start_x,
            "Токен должен двигаться в направлении цели: pos.x={}, start.x={}",
            pos.0,
            start_x
        );

        // Проверка что токен движется в правильном направлении (может превысить из-за инерции)
        let distance_to_target = (pos.0 - target.0).abs();
        let initial_distance = (start_x - target.0).abs();

        assert!(
            distance_to_target < initial_distance,
            "Токен должен быть ближе к цели: distance={}, initial={}",
            distance_to_target,
            initial_distance
        );
    }

    #[test]
    fn test_motion_determinism() {
        // Детерминизм: одинаковые входы → одинаковые результаты
        let start_pos = (100i16, 200i16, 300i16);
        let start_vel = (10i16, -5i16, 3i16);

        let mut pos1 = start_pos;
        let mut vel1 = start_vel;

        let mut pos2 = start_pos;
        let mut vel2 = start_vel;

        // Два независимых прогона симуляции
        for _ in 0..5 {
            let gravity = compute_gravity(pos1.0, pos1.1, pos1.2, 1000, 8, GravityModel::Linear);
            vel1 = apply_acceleration(vel1, gravity);
            vel1 = apply_friction(vel1, 6);
            pos1 = apply_velocity(pos1, vel1);
        }

        for _ in 0..5 {
            let gravity = compute_gravity(pos2.0, pos2.1, pos2.2, 1000, 8, GravityModel::Linear);
            vel2 = apply_acceleration(vel2, gravity);
            vel2 = apply_friction(vel2, 6);
            pos2 = apply_velocity(pos2, vel2);
        }

        assert_eq!(pos1, pos2, "Determinism violated: positions differ");
        assert_eq!(vel1, vel2, "Determinism violated: velocities differ");
    }

    // ========================================================================
    // SPATIAL EVENTS TESTS
    // ========================================================================

    #[test]
    fn test_has_moved_no_movement() {
        // Токен не двигался
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (100i16, 200i16, 300i16);

        assert!(!has_moved(old_pos, new_pos));
    }

    #[test]
    fn test_has_moved_x_axis() {
        // Движение по оси X
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (110i16, 200i16, 300i16);

        assert!(has_moved(old_pos, new_pos));
    }

    #[test]
    fn test_has_moved_all_axes() {
        // Движение по всем осям
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (110i16, 195i16, 305i16);

        assert!(has_moved(old_pos, new_pos));
    }

    #[test]
    fn test_has_moved_small_delta() {
        // Даже малое движение (1 квант) детектируется
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (100i16, 201i16, 300i16);

        assert!(has_moved(old_pos, new_pos));
    }

    #[test]
    fn test_cell_changed_no_change() {
        // Токен остался в той же ячейке
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (110i16, 210i16, 310i16);

        // Ячейка 256x256x256, эти позиции в одной ячейке
        assert!(!cell_changed(old_pos, new_pos));
    }

    #[test]
    fn test_cell_changed_x_axis() {
        // Токен перешёл в соседнюю ячейку по X
        let old_pos = (100i16, 200i16, 300i16);
        let new_pos = (300i16, 200i16, 300i16); // Перешёл через границу ячейки

        assert!(cell_changed(old_pos, new_pos));
    }

    #[test]
    fn test_cell_changed_boundary() {
        // Переход через границу ячейки (256 квантов)
        let old_pos = (255i16, 0i16, 0i16);
        let new_pos = (256i16, 0i16, 0i16);

        // old_pos в ячейке 0, new_pos в ячейке 1
        assert!(cell_changed(old_pos, new_pos));
    }

    #[test]
    fn test_cell_changed_negative_coords() {
        // Работа с отрицательными координатами
        let old_pos = (-100i16, -100i16, -100i16);
        let new_pos = (-300i16, -100i16, -100i16);

        assert!(cell_changed(old_pos, new_pos));
    }

    #[test]
    fn test_cell_changed_diagonal() {
        // Переход через границу по всем осям
        let old_pos = (100i16, 100i16, 100i16);
        let new_pos = (500i16, 500i16, 500i16);

        assert!(cell_changed(old_pos, new_pos));
    }

    #[test]
    fn test_detect_collisions_empty() {
        // Нет соседей - нет столкновений
        let grid = SpatialHashGrid::new();
        let get_position = |_: u32| (0i16, 0i16, 0i16);

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_detect_collisions_no_neighbors() {
        // Есть токены, но далеко - нет столкновений
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (1000, 1000, 1000), // Далеко
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_detect_collisions_single() {
        // Один сосед в радиусе столкновения
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (130, 100, 100), // Расстояние 30 < радиус 50
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0], 1);
    }

    #[test]
    fn test_detect_collisions_multiple() {
        // Несколько соседей в радиусе
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (110, 100, 100), // Близко (расстояние 10)
            (120, 100, 100), // Близко (расстояние 20)
            (200, 100, 100), // Далеко (расстояние 100)
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        // Должны обнаружить токены 1 и 2, но не 3
        assert_eq!(collisions.len(), 2);
        assert!(collisions.contains(&1));
        assert!(collisions.contains(&2));
        assert!(!collisions.contains(&3));
    }

    #[test]
    fn test_detect_collisions_excludes_self() {
        // Не должен включать сам токен в список столкновений
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (110, 100, 100),
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        // Не должен содержать индекс 0 (сам токен)
        assert!(!collisions.contains(&0));
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0], 1);
    }

    #[test]
    fn test_detect_collisions_boundary() {
        // Токен ровно на границе радиуса столкновения
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (150, 100, 100), // Расстояние ровно 50
            (151, 100, 100), // Расстояние 51 - вне радиуса
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 50, get_position, &grid);

        // Токен 1 ровно на границе - должен быть включён
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0], 1);
    }

    #[test]
    fn test_detect_collisions_diagonal() {
        // Столкновение по диагонали
        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            (100i16, 100i16, 100i16),
            (130, 130, 130), // Диагональное расстояние ~52 (sqrt(30²+30²+30²))
            (120, 120, 120), // Диагональное расстояние ~35 (sqrt(20²+20²+20²))
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        let collisions = detect_collisions(0, (100, 100, 100), 40, get_position, &grid);

        // Только токен 2 в радиусе (~35 < 40)
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0], 2);
    }

    #[test]
    fn test_spatial_events_integration() {
        // Интеграционный тест: движение + смена ячейки + столкновение
        let old_pos = (100i16, 100i16, 100i16);
        let mut pos = old_pos;
        let mut vel = (20i16, 0i16, 0i16);

        let mut grid = SpatialHashGrid::new();
        let positions = vec![
            pos,
            (150, 100, 100), // Потенциальное столкновение после движения
        ];

        grid.rebuild(positions.len(), |i| positions[i]);
        let get_position = |index: u32| positions[index as usize];

        // Шаг симуляции
        pos = apply_velocity(pos, vel);

        // Проверки событий
        assert!(has_moved(old_pos, pos), "Должно быть событие TokenMoved");

        // Новая позиция (120, 100, 100)
        assert_eq!(pos, (120, 100, 100));

        // Ячейка не изменилась (100 и 120 в одной ячейке 0)
        assert!(
            !cell_changed(old_pos, pos),
            "Ячейка не должна измениться (обе позиции в ячейке 0)"
        );

        // Обнаружение столкновения
        let collisions = detect_collisions(0, pos, 50, get_position, &grid);

        // Токен 1 на расстоянии 30 от новой позиции - столкновение
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0], 1);
    }
}
