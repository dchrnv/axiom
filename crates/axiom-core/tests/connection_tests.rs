use axiom_core::{Connection, FLAG_ACTIVE, FLAG_CRITICAL, FLAG_INHIBITED, FLAG_TEMPORARY};

#[test]
fn test_connection_size() {
    assert_eq!(std::mem::size_of::<Connection>(), 64);
    assert_eq!(std::mem::align_of::<Connection>(), 64);
}

#[test]
fn test_connection_new() {
    let conn = Connection::new(1, 2, 1, 100);
    assert_eq!(conn.source_id, 1);
    assert_eq!(conn.target_id, 2);
    assert_eq!(conn.domain_id, 1);
    assert_eq!(conn.created_at, 100);
    assert_eq!(conn.last_event_id, 100);
    assert!(conn.is_active());
    assert!(conn.validate().is_ok());
}

#[test]
fn test_connection_validation() {
    let mut conn = Connection::new(1, 2, 1, 100);
    assert!(conn.validate().is_ok());

    conn.source_id = 0;
    assert!(conn.validate().is_err());
    conn.source_id = 1;

    conn.target_id = 0;
    assert!(conn.validate().is_err());
    conn.target_id = 2;

    conn.domain_id = 0;
    assert!(conn.validate().is_err());
    conn.domain_id = 1;

    conn.strength = 0.0;
    assert!(conn.validate().is_err());
    conn.strength = 1.0;

    conn.current_stress = -1.0;
    assert!(conn.validate().is_err());
    conn.current_stress = 0.0;

    conn.elasticity = 0.0;
    assert!(conn.validate().is_err());
    conn.elasticity = 1.0;

    conn.created_at = 0;
    assert!(conn.validate().is_err());
    conn.created_at = 100;

    conn.last_event_id = 99;
    assert!(conn.validate().is_err());
}

#[test]
fn test_connection_flags() {
    let mut conn = Connection::new(1, 2, 1, 100);

    conn.flags = FLAG_ACTIVE;
    assert!(conn.is_active());
    assert!(!conn.is_inhibited());
    assert!(!conn.is_temporary());
    assert!(!conn.is_critical());

    conn.flags = FLAG_INHIBITED;
    assert!(!conn.is_active());
    assert!(conn.is_inhibited());

    conn.flags = FLAG_TEMPORARY;
    assert!(conn.is_temporary());

    conn.flags = FLAG_ACTIVE | FLAG_CRITICAL;
    assert!(conn.is_active());
    assert!(conn.is_critical());
}

#[test]
fn test_update_stress() {
    let mut conn = Connection::new(1, 2, 1, 100);
    conn.strength = 10.0;

    // Нормальный стресс
    conn.update_stress(5.0, 101);
    assert_eq!(conn.current_stress, 5.0);
    assert_eq!(conn.last_event_id, 101);
    assert!(!conn.is_critical());

    // Высокий стресс (>80%) должен установить FLAG_CRITICAL
    conn.update_stress(9.0, 102);
    assert_eq!(conn.current_stress, 9.0);
    assert!(conn.is_critical());

    // Снижение стресса должно убрать FLAG_CRITICAL
    conn.update_stress(5.0, 103);
    assert!(!conn.is_critical());

    // Отрицательный стресс должен быть скорректирован до 0
    conn.update_stress(-1.0, 104);
    assert_eq!(conn.current_stress, 0.0);
}

#[test]
fn test_gates() {
    let mut conn = Connection::new(1, 2, 1, 100);

    conn.density_gate = 50;
    assert!(!conn.can_pass_mass(40));
    assert!(conn.can_pass_mass(50));
    assert!(conn.can_pass_mass(60));

    conn.thermal_gate = 100;
    assert!(conn.can_pass_temperature(90));
    assert!(conn.can_pass_temperature(100));
    assert!(!conn.can_pass_temperature(110));
}

#[test]
fn test_compute_distance() {
    let conn = Connection::new(1, 2, 1, 100);

    let source_pos = [0, 0, 0];
    let target_pos = [3, 4, 0];
    let distance = conn.compute_distance(source_pos, target_pos);
    assert!((distance - 5.0).abs() < 0.001);

    let target_pos_3d = [1, 1, 1];
    let distance_3d = conn.compute_distance(source_pos, target_pos_3d);
    assert!((distance_3d - 1.732).abs() < 0.01);
}

#[test]
fn test_spring_force() {
    let mut conn = Connection::new(1, 2, 1, 100);
    conn.ideal_dist = 10.0;
    conn.elasticity = 1.0;
    conn.strength = 1.0;

    // Расстояние больше идеального → притяжение (отрицательная сила)
    let force = conn.compute_spring_force(15.0);
    assert!(force < 0.0);

    // Расстояние меньше идеального → отталкивание (положительная сила)
    let force = conn.compute_spring_force(5.0);
    assert!(force > 0.0);

    // Расстояние равно идеальному → нет силы
    let force = conn.compute_spring_force(10.0);
    assert_eq!(force, 0.0);
}
