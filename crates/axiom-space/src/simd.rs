// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Batch-обработка гравитации для N токенов.
//
// Без feature "simd": скалярный цикл.
// С feature "simd" + RUSTFLAGS="-C target-cpu=native": компилятор авто-векторизует
// Inner loop — чистые i64 вычисления без ветвлений → AVX2/SSE4.2.
//
// Этап 12B.

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
/// # Аргументы
///
/// - `positions`: массив координат `[x, y, z]` для каждого токена
/// - `masses`: масса каждого токена (параллельно с positions)
/// - `gravity_scale_shift`: битовый сдвиг масштабирования (обычно 24)
/// - `model`: модель гравитации (Linear или InverseSquare)
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

    // Scalar path — ясный цикл без ветвлений в горячем пути.
    // Компилятор авто-векторизует при -C target-cpu=native (AVX2/SSE4.2).
    for i in 0..n {
        let [x, y, z] = positions[i];
        let mass = masses[i];
        let (ax, ay, az) = compute_gravity(x, y, z, mass, gravity_scale_shift, model);
        accelerations.push((ax, ay, az));
    }

    GravityBatchResult { accelerations }
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
///
/// `velocities[i]` += `accelerations[i]` (saturating).
pub fn apply_accelerations_to_velocities(velocities: &mut [[i16; 3]], result: &GravityBatchResult) {
    assert_eq!(velocities.len(), result.accelerations.len());

    for (vel, &(ax, ay, az)) in velocities.iter_mut().zip(result.accelerations.iter()) {
        vel[0] = vel[0].saturating_add(ax);
        vel[1] = vel[1].saturating_add(ay);
        vel[2] = vel[2].saturating_add(az);
    }
}
