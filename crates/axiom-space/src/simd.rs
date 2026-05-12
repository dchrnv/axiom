// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Batch-обработка гравитации для N токенов.
//
// apply_gravity_batch       — scalar, детерминировано идентичен N вызовам compute_gravity.
// apply_gravity_batch_avx2  — AVX2 f32, только Linear, shift ∈ [8,15].
//   При shift ≥ 16 ранний выход (для i16 позиций force всегда 0).
//   При shift < 8 или InverseSquare → делегирует в apply_gravity_batch.
//   Результат совпадает со scalar в пределах ±1 (f32 ошибка < 0.005 << 1).
//
// SENT-S4b (Axiom Sentinel V1.1).

use crate::{compute_gravity, GravityModel};

/// Размер чанка для L2-cache-friendly batch (512 KB / 8 bytes per token = 65536).
/// При N > 1M токенов chunked-вариант снижает cache-miss, не меняя результат.
pub const L2_CHUNK_TOKENS: usize = 65536;

/// Результат batch-вычисления: ускорение для каждого токена.
#[derive(Debug, Clone, PartialEq)]
pub struct GravityBatchResult {
    /// Ускорения по каждой оси: `accelerations[i] = (ax, ay, az)`
    pub accelerations: Vec<(i16, i16, i16)>,
}

/// Batch-применение гравитации к якорю (0, 0, 0) для N токенов.
///
/// Эквивалент N вызовов `compute_gravity(...)` но в одном цикле,
/// пригодном для авто-векторизации компилятором при `-C target-cpu=native`.
///
/// # Паника
///
/// Паникует если `positions.len() != masses.len()`.
pub fn apply_gravity_batch(
    positions: &[[i16; 3]],
    masses: &[u16],
    gravity_scale_shift: u32,
    model: GravityModel,
) -> GravityBatchResult {
    assert_eq!(
        positions.len(),
        masses.len(),
        "positions and masses must have the same length"
    );

    let n = positions.len();
    let mut accelerations = Vec::with_capacity(n);

    for i in 0..n {
        let [x, y, z] = positions[i];
        let mass = masses[i];
        let (ax, ay, az) = compute_gravity(x, y, z, mass, gravity_scale_shift, model);
        accelerations.push((ax, ay, az));
    }

    GravityBatchResult { accelerations }
}

/// AVX2-ускоренный batch для Linear-гравитации. 8 токенов за цикл.
///
/// Условия переключения:
/// - `model != Linear` → делегирует в [`apply_gravity_batch`]
/// - `shift >= 16` → ранний выход all-zeros (для i16 позиций max dist < 2^16)
/// - `shift < 8` → делегирует в [`apply_gravity_batch`]
/// - AVX2 не обнаружен или `n < 8` → делегирует в [`apply_gravity_batch`]
///
/// Результат совпадает со scalar в пределах ±1 на компоненту для shift ∈ [8, 15].
/// Для большинства позиций результат побитово идентичен scalar.
///
/// # Паника
///
/// Паникует если `positions.len() != masses.len()`.
pub fn apply_gravity_batch_avx2(
    positions: &[[i16; 3]],
    masses: &[u16],
    gravity_scale_shift: u32,
    model: GravityModel,
) -> GravityBatchResult {
    assert_eq!(
        positions.len(),
        masses.len(),
        "positions and masses must have the same length"
    );
    let n = positions.len();

    if model != GravityModel::Linear || n < 8 || gravity_scale_shift < 8 {
        return apply_gravity_batch(positions, masses, gravity_scale_shift, model);
    }

    // i16 позиции гарантируют dist < 2^16 (max ~56755). При shift >= 16 force = 0 всегда.
    if gravity_scale_shift >= 16 {
        return GravityBatchResult {
            accelerations: vec![(0, 0, 0); n],
        };
    }

    #[cfg(all(target_arch = "x86_64", feature = "simd"))]
    if is_x86_feature_detected!("avx2") {
        return unsafe { gravity_linear_avx2_inner(positions, gravity_scale_shift) };
    }

    apply_gravity_batch(positions, masses, gravity_scale_shift, model)
}

/// Chunked-версия apply_gravity_batch: обрабатывает входные срезы окнами по L2_CHUNK_TOKENS.
///
/// Для N ≤ L2_CHUNK_TOKENS — делегирует напрямую в apply_gravity_batch без overhead.
/// Для N > L2_CHUNK_TOKENS — итерирует чанками, удерживая рабочее множество в L2.
/// Результат идентичен apply_gravity_batch.
pub fn apply_gravity_batch_chunked(
    positions: &[[i16; 3]],
    masses: &[u16],
    gravity_scale_shift: u32,
    model: GravityModel,
) -> GravityBatchResult {
    assert_eq!(positions.len(), masses.len());
    if positions.len() <= L2_CHUNK_TOKENS {
        return apply_gravity_batch(positions, masses, gravity_scale_shift, model);
    }
    let n = positions.len();
    let mut accelerations = Vec::with_capacity(n);
    let mut start = 0;
    while start < n {
        let end = (start + L2_CHUNK_TOKENS).min(n);
        let chunk = apply_gravity_batch(
            &positions[start..end],
            &masses[start..end],
            gravity_scale_shift,
            model,
        );
        accelerations.extend(chunk.accelerations);
        start = end;
    }
    GravityBatchResult { accelerations }
}

/// Применить batch-ускорения к скоростям токенов in-place.
pub fn apply_accelerations_to_velocities(velocities: &mut [[i16; 3]], result: &GravityBatchResult) {
    assert_eq!(velocities.len(), result.accelerations.len());

    for (vel, &(ax, ay, az)) in velocities.iter_mut().zip(result.accelerations.iter()) {
        vel[0] = vel[0].saturating_add(ax);
        vel[1] = vel[1].saturating_add(ay);
        vel[2] = vel[2].saturating_add(az);
    }
}

// ─── AVX2 inner implementation ───────────────────────────────────────────────

#[cfg(all(target_arch = "x86_64", feature = "simd"))]
#[target_feature(enable = "avx2")]
unsafe fn gravity_linear_avx2_inner(positions: &[[i16; 3]], shift: u32) -> GravityBatchResult {
    use std::arch::x86_64::*;

    let n = positions.len();
    let mut accelerations = Vec::with_capacity(n);

    // Деинтерливинг AoS [[x,y,z]; N] → SoA xs[], ys[], zs[]
    // Позволяет делать contiguous 128-bit loads по 8 i16 за раз.
    let mut xs = vec![0i16; n];
    let mut ys = vec![0i16; n];
    let mut zs = vec![0i16; n];
    for (i, pos) in positions.iter().enumerate() {
        xs[i] = pos[0];
        ys[i] = pos[1];
        zs[i] = pos[2];
    }

    let chunks = n / 8;
    let shift_v = _mm256_set1_epi32(shift as i32);
    let one = _mm256_set1_ps(1.0f32);
    let zero = _mm256_setzero_ps();
    let lo = _mm256_set1_ps(i16::MIN as f32);
    let hi = _mm256_set1_ps(i16::MAX as f32);

    for chunk in 0..chunks {
        let i = chunk * 8;

        // Загрузить 8 × i16 из каждого SoA-вектора, знаково расширить до i32, конвертировать в f32.
        let x_f = _mm256_cvtepi32_ps(_mm256_cvtepi16_epi32(_mm_loadu_si128(
            xs[i..].as_ptr() as *const __m128i,
        )));
        let y_f = _mm256_cvtepi32_ps(_mm256_cvtepi16_epi32(_mm_loadu_si128(
            ys[i..].as_ptr() as *const __m128i,
        )));
        let z_f = _mm256_cvtepi32_ps(_mm256_cvtepi16_epi32(_mm_loadu_si128(
            zs[i..].as_ptr() as *const __m128i,
        )));

        // dist2 = x² + y² + z²
        let dist2 = _mm256_add_ps(
            _mm256_add_ps(_mm256_mul_ps(x_f, x_f), _mm256_mul_ps(y_f, y_f)),
            _mm256_mul_ps(z_f, z_f),
        );

        // dist_f = sqrt(dist2) — IEEE 754 правильно округлённый
        let dist_f = _mm256_sqrt_ps(dist2);

        // force = (i32) floor(dist_f) >> shift
        let force_f = _mm256_cvtepi32_ps(_mm256_srav_epi32(
            _mm256_cvttps_epi32(dist_f),
            shift_v,
        ));

        // Защита от деления на ноль: dist_safe = max(dist_f, 1.0)
        let dist_safe = _mm256_max_ps(dist_f, one);

        // a = force * (-pos) / dist_safe
        let ax_f = _mm256_div_ps(_mm256_mul_ps(force_f, _mm256_sub_ps(zero, x_f)), dist_safe);
        let ay_f = _mm256_div_ps(_mm256_mul_ps(force_f, _mm256_sub_ps(zero, y_f)), dist_safe);
        let az_f = _mm256_div_ps(_mm256_mul_ps(force_f, _mm256_sub_ps(zero, z_f)), dist_safe);

        // Clamp в i16 диапазон и конвертировать в i32 (truncate toward zero).
        let ax_i = _mm256_cvttps_epi32(_mm256_min_ps(_mm256_max_ps(ax_f, lo), hi));
        let ay_i = _mm256_cvttps_epi32(_mm256_min_ps(_mm256_max_ps(ay_f, lo), hi));
        let az_i = _mm256_cvttps_epi32(_mm256_min_ps(_mm256_max_ps(az_f, lo), hi));

        // Выгрузить из регистров в массивы
        let mut ax_arr = [0i32; 8];
        let mut ay_arr = [0i32; 8];
        let mut az_arr = [0i32; 8];
        _mm256_storeu_si256(ax_arr.as_mut_ptr() as *mut __m256i, ax_i);
        _mm256_storeu_si256(ay_arr.as_mut_ptr() as *mut __m256i, ay_i);
        _mm256_storeu_si256(az_arr.as_mut_ptr() as *mut __m256i, az_i);

        for j in 0..8 {
            accelerations.push((ax_arr[j] as i16, ay_arr[j] as i16, az_arr[j] as i16));
        }
    }

    // Остаток (< 8 токенов) — scalar
    for i in (chunks * 8)..n {
        let (ax, ay, az) =
            compute_gravity(positions[i][0], positions[i][1], positions[i][2], 0, shift, GravityModel::Linear);
        accelerations.push((ax, ay, az));
    }

    GravityBatchResult { accelerations }
}
