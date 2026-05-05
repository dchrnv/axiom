// Этап 12B — SIMD batch-обработка гравитации

use axiom_space::{apply_accelerations_to_velocities, apply_gravity_batch, GravityModel};

// ─── apply_gravity_batch ──────────────────────────────────────────────────────

#[test]
fn test_batch_empty() {
    let result = apply_gravity_batch(&[], &[], 24, GravityModel::Linear);
    assert!(result.accelerations.is_empty());
}

#[test]
fn test_batch_single_token() {
    let positions = [[100i16, 0, 0]];
    let masses = [1000u16];
    let result = apply_gravity_batch(&positions, &masses, 24, GravityModel::Linear);
    assert_eq!(result.accelerations.len(), 1);
    // Токен справа от якоря → ускорение по X отрицательное (к якорю)
    let (ax, _, _) = result.accelerations[0];
    assert!(
        ax <= 0,
        "ускорение должно быть в сторону якоря (≤ 0), got {ax}"
    );
}

#[test]
fn test_batch_matches_scalar() {
    use axiom_space::compute_gravity;

    let positions = [
        [100i16, 200, 300],
        [-50, 100, 0],
        [0, 0, 500],
        [1000, -500, 200],
    ];
    let masses = [500u16, 1000, 200, 750];
    let scale_shift = 24;
    let model = GravityModel::Linear;

    let batch = apply_gravity_batch(&positions, &masses, scale_shift, model);

    for (i, ([x, y, z], &mass)) in positions.iter().zip(masses.iter()).enumerate() {
        let expected = compute_gravity(*x, *y, *z, mass, scale_shift, model);
        assert_eq!(batch.accelerations[i], expected, "mismatch at index {i}");
    }
}

#[test]
fn test_batch_token_at_anchor_gives_zero() {
    let positions = [[0i16, 0, 0]];
    let masses = [500u16];
    let result = apply_gravity_batch(&positions, &masses, 24, GravityModel::InverseSquare);
    assert_eq!(result.accelerations[0], (0, 0, 0));
}

#[test]
fn test_batch_inverse_square() {
    let positions = [[0i16, 100, 0]];
    let masses = [2000u16];
    let result = apply_gravity_batch(&positions, &masses, 20, GravityModel::InverseSquare);
    assert_eq!(result.accelerations.len(), 1);
    let (_, ay, _) = result.accelerations[0];
    assert!(ay <= 0, "ускорение должно быть к якорю, got ay={ay}");
}

#[test]
fn test_batch_5000_tokens() {
    let n = 5000;
    let positions: Vec<[i16; 3]> = (0..n)
        .map(|i| [(i % 1000) as i16, (i % 500) as i16, 0])
        .collect();
    let masses: Vec<u16> = (0..n).map(|_| 100u16).collect();

    let result = apply_gravity_batch(&positions, &masses, 24, GravityModel::Linear);
    assert_eq!(result.accelerations.len(), n);
}

// ─── apply_accelerations_to_velocities ────────────────────────────────────────

#[test]
fn test_apply_accelerations_basic() {
    let mut velocities = [[10i16, -20, 0], [5, 5, 5]];
    let result = apply_gravity_batch(
        &[[100i16, 0, 0], [-100, 0, 0]],
        &[1000u16, 1000],
        24,
        GravityModel::Linear,
    );
    apply_accelerations_to_velocities(&mut velocities, &result);
    // Просто проверяем что не паникует и размер совпадает
    assert_eq!(velocities.len(), 2);
}

#[test]
fn test_apply_accelerations_zero() {
    let mut velocities = [[100i16, 100, 100]];
    let result = apply_gravity_batch(&[[0i16, 0, 0]], &[500u16], 24, GravityModel::Linear);
    apply_accelerations_to_velocities(&mut velocities, &result);
    // Токен в якоре — нет изменений
    assert_eq!(velocities[0], [100, 100, 100]);
}

#[test]
#[should_panic]
fn test_batch_length_mismatch_panics() {
    apply_gravity_batch(&[[0i16, 0, 0]], &[], 24, GravityModel::Linear);
}
