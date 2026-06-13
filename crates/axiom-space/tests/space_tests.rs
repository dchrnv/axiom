use axiom_space::*;

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
                let key =
                    SpatialHashGrid::cell_key((x * 500) as i16, (y * 500) as i16, (z * 500) as i16);
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
    let positions = [
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
    let positions1 = [(100i16, 100i16, 100i16), (200, 200, 200)];
    let positions2 = [(500i16, 500i16, 500i16)];

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
    let positions = [(100i16, 100i16, 100i16)];

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
    let positions = [
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
    let positions = [
        (0i16, 0i16, 0i16),
        (100, 0, 0), // Расстояние ровно 100
        (0, 100, 0), // Расстояние ровно 100
        (0, 0, 100), // Расстояние ровно 100
        (101, 0, 0), // Расстояние 101 - вне радиуса
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
    let positions = [
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
    let positions = [
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
    let positions = [
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
    let positions = [
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
    let positions = [(100i16, 100i16, 100i16), (110, 100, 100)];

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
    let positions = [
        (100i16, 100i16, 100i16),
        (150i16, 100i16, 100i16), // Расстояние ровно 50
        (151i16, 100i16, 100i16), // Расстояние 51 - вне радиуса
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
    let positions = [
        (100i16, 100i16, 100i16),
        (130i16, 130i16, 130i16), // Диагональное расстояние ~52 (sqrt(30²+30²+30²))
        (120i16, 120i16, 120i16), // Диагональное расстояние ~35 (sqrt(20²+20²+20²))
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
    let vel = (20i16, 0i16, 0i16);

    let mut grid = SpatialHashGrid::new();
    let positions = [
        pos,
        (150i16, 100i16, 100i16), // Потенциальное столкновение после движения
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

// ─── SpatialConfig ───────────────────────────────────────────────────────────

fn spatial_preset_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/presets/spatial")
        .join(name)
}

#[test]
fn test_spatial_config_presets_valid() {
    assert!(SpatialConfig::tight().validate().is_ok());
    assert!(SpatialConfig::medium().validate().is_ok());
    assert!(SpatialConfig::loose().validate().is_ok());
}

#[test]
fn test_spatial_config_medium_matches_constants() {
    let cfg = SpatialConfig::medium();
    assert_eq!(cfg.cell_shift, CELL_SHIFT);
    assert_eq!(cfg.bucket_count_log2, BUCKET_COUNT_LOG2);
    assert_eq!(cfg.cell_size(), CELL_SIZE as u32);
    assert_eq!(cfg.bucket_count(), BUCKET_COUNT);
}

#[test]
fn test_spatial_config_tight_smaller_cells() {
    let tight = SpatialConfig::tight();
    let medium = SpatialConfig::medium();
    assert!(tight.cell_size() < medium.cell_size());
    assert!(tight.bucket_count() > medium.bucket_count());
}

#[test]
fn test_spatial_config_loose_larger_cells() {
    let loose = SpatialConfig::loose();
    let medium = SpatialConfig::medium();
    assert!(loose.cell_size() > medium.cell_size());
    assert!(loose.bucket_count() < medium.bucket_count());
}

#[test]
fn test_spatial_config_validation_bad_cell_shift() {
    let mut cfg = SpatialConfig::medium();
    cfg.cell_shift = 0;
    assert!(cfg.validate().is_err());
    cfg.cell_shift = 16;
    assert!(cfg.validate().is_err());
}

#[test]
fn test_spatial_config_validation_bad_bucket_log2() {
    let mut cfg = SpatialConfig::medium();
    cfg.bucket_count_log2 = 7;
    assert!(cfg.validate().is_err());
    cfg.bucket_count_log2 = 25;
    assert!(cfg.validate().is_err());
}

#[test]
fn test_spatial_hash_grid_with_config() {
    let cfg = SpatialConfig::tight();
    let mut grid = SpatialHashGrid::with_config(&cfg);

    // bucket_heads длина соответствует конфигурации
    assert_eq!(grid.bucket_heads.len(), cfg.bucket_count());
    assert_eq!(grid.entry_count, 0);

    // insert работает
    grid.insert(0, 100, 200, 300);
    assert_eq!(grid.entry_count, 1);
}

#[test]
fn test_from_yaml_tight() {
    let path = spatial_preset_path("tight.yaml");
    let cfg = SpatialConfig::from_yaml(&path).unwrap();
    let reference = SpatialConfig::tight();
    assert_eq!(cfg.cell_shift, reference.cell_shift);
    assert_eq!(cfg.bucket_count_log2, reference.bucket_count_log2);
    assert_eq!(cfg.initial_capacity, reference.initial_capacity);
}

#[test]
fn test_from_yaml_medium() {
    let path = spatial_preset_path("medium.yaml");
    let cfg = SpatialConfig::from_yaml(&path).unwrap();
    let reference = SpatialConfig::medium();
    assert_eq!(cfg.cell_shift, reference.cell_shift);
    assert_eq!(cfg.bucket_count_log2, reference.bucket_count_log2);
    assert_eq!(cfg.initial_capacity, reference.initial_capacity);
}

#[test]
fn test_from_yaml_loose() {
    let path = spatial_preset_path("loose.yaml");
    let cfg = SpatialConfig::from_yaml(&path).unwrap();
    let reference = SpatialConfig::loose();
    assert_eq!(cfg.cell_shift, reference.cell_shift);
    assert_eq!(cfg.bucket_count_log2, reference.bucket_count_log2);
    assert_eq!(cfg.initial_capacity, reference.initial_capacity);
}

#[test]
fn test_from_yaml_missing_file() {
    let result = SpatialConfig::from_yaml(std::path::Path::new("/nonexistent.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_from_yaml_all_presets_valid() {
    for name in &["tight.yaml", "medium.yaml", "loose.yaml"] {
        let cfg = SpatialConfig::from_yaml(&spatial_preset_path(name)).unwrap();
        assert!(cfg.validate().is_ok(), "preset {} invalid", name);
    }
}
