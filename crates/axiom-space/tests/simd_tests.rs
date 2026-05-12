// Этап 12B — SIMD batch-обработка гравитации; SENT-S4b — AVX2

use axiom_space::{
    apply_accelerations_to_velocities, apply_gravity_batch, apply_gravity_batch_avx2, GravityModel,
};

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

// ─── apply_gravity_batch_avx2 ─────────────────────────────────────────────────

#[test]
fn test_avx2_empty() {
    let result = apply_gravity_batch_avx2(&[], &[], 8, GravityModel::Linear);
    assert!(result.accelerations.is_empty());
}

#[test]
fn test_avx2_shift24_all_zeros() {
    // Для shift=24 и i16 позиций force всегда 0 — ранний выход.
    let positions: Vec<[i16; 3]> = (0..16).map(|i| [(i * 100) as i16, 0, 0]).collect();
    let masses = vec![1000u16; 16];
    let result = apply_gravity_batch_avx2(&positions, &masses, 24, GravityModel::Linear);
    assert!(result.accelerations.iter().all(|&a| a == (0, 0, 0)));
}

#[test]
fn test_avx2_at_anchor_gives_zero() {
    let positions = vec![[0i16, 0, 0]; 8];
    let masses = vec![500u16; 8];
    let result = apply_gravity_batch_avx2(&positions, &masses, 8, GravityModel::Linear);
    assert!(result.accelerations.iter().all(|&a| a == (0, 0, 0)));
}

#[test]
fn test_avx2_direction_correct() {
    // Токен правее якоря → ax < 0 (притяжение влево)
    let positions = vec![[256i16, 0, 0]; 8];
    let masses = vec![1000u16; 8];
    let result = apply_gravity_batch_avx2(&positions, &masses, 8, GravityModel::Linear);
    // dist=256, force=256>>8=1, ax = 1 * (-256) / 256 = -1
    for &(ax, ay, az) in &result.accelerations {
        assert_eq!(ax, -1, "ax must be -1");
        assert_eq!(ay, 0);
        assert_eq!(az, 0);
    }
}

#[test]
fn test_avx2_matches_scalar_shift8() {
    // Для shift=8 и этих позиций результат AVX2 совпадает со scalar (±1 допускается).
    let positions: Vec<[i16; 3]> = [
        [256i16, 0, 0],   // dist=256, force=1, ax=-1
        [512, 0, 0],      // dist=512, force=2, ax=-2
        [1000, 0, 0],     // dist=1000, force=3, ax=-3
        [-256, 0, 0],     // ax=+1
        [-512, 0, 0],     // ax=+2
        [0, 256, 0],      // ay=-1
        [0, 0, 512],      // az=-2
        [300, 400, 0],    // dist=500, force=1, ax=1*(-300)/500=-0→0 ay=1*(-400)/500=-0→0
        [0, 0, 0],        // anchor
        [1000, 0, 100],   // dist≈1005, force=3, ax≈-3
        [32000, 32000, 0],// near max i16
        [100, 200, 300],  // mixed
        [256, 256, 0],    // dist≈362, force=1, ax=-1*256/362=0
        [512, 512, 0],    // dist≈724, force=2, ax=-2*512/724=-1
        [-1000, 500, 200],
        [0, 0, 1],
    ]
    .to_vec();
    let masses = vec![1000u16; 16];
    let avx2 = apply_gravity_batch_avx2(&positions, &masses, 8, GravityModel::Linear);
    let scalar = apply_gravity_batch(&positions, &masses, 8, GravityModel::Linear);
    assert_eq!(avx2.accelerations.len(), scalar.accelerations.len());
    for (i, (&a, &s)) in avx2.accelerations.iter().zip(scalar.accelerations.iter()).enumerate() {
        let dx = (a.0 as i32 - s.0 as i32).abs();
        let dy = (a.1 as i32 - s.1 as i32).abs();
        let dz = (a.2 as i32 - s.2 as i32).abs();
        assert!(
            dx <= 1 && dy <= 1 && dz <= 1,
            "index {i}: avx2={a:?} scalar={s:?}"
        );
    }
}

#[test]
fn test_avx2_inverse_square_falls_back() {
    // InverseSquare → делегирует в scalar, результат идентичен.
    let positions: Vec<[i16; 3]> = (0..8).map(|i| [0i16, (i * 100 + 100) as i16, 0]).collect();
    let masses = vec![2000u16; 8];
    let avx2 = apply_gravity_batch_avx2(&positions, &masses, 12, GravityModel::InverseSquare);
    let scalar = apply_gravity_batch(&positions, &masses, 12, GravityModel::InverseSquare);
    assert_eq!(avx2.accelerations, scalar.accelerations);
}

#[test]
fn test_avx2_large_n_divisible_by_8() {
    let n = 1024usize;
    let positions: Vec<[i16; 3]> = (0..n)
        .map(|i| [((i % 500) as i16) * 60, ((i % 300) as i16) * 80, 0])
        .collect();
    let masses = vec![500u16; n];
    let result = apply_gravity_batch_avx2(&positions, &masses, 8, GravityModel::Linear);
    assert_eq!(result.accelerations.len(), n);
}

#[test]
fn test_avx2_n_not_multiple_of_8() {
    // Проверка remainder path (последние токены через scalar).
    let n = 19usize;
    let positions: Vec<[i16; 3]> = (0..n).map(|i| [(i as i16) * 200, 0, 0]).collect();
    let masses = vec![1000u16; n];
    let avx2 = apply_gravity_batch_avx2(&positions, &masses, 8, GravityModel::Linear);
    let scalar = apply_gravity_batch(&positions, &masses, 8, GravityModel::Linear);
    assert_eq!(avx2.accelerations.len(), n);
    for (i, (&a, &s)) in avx2.accelerations.iter().zip(scalar.accelerations.iter()).enumerate() {
        let dx = (a.0 as i32 - s.0 as i32).abs();
        let dy = (a.1 as i32 - s.1 as i32).abs();
        let dz = (a.2 as i32 - s.2 as i32).abs();
        assert!(dx <= 1 && dy <= 1 && dz <= 1, "index {i}: avx2={a:?} scalar={s:?}");
    }
}
