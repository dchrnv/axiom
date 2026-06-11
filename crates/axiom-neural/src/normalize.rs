// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Z-score нормализация. Inplace, ноль alloc.

/// Нормализовать срез по Z-score inplace: (x - mean) / (std + ε).
/// Если std ≈ 0 (константный сигнал) — обнуляет срез.
pub fn zscore_inplace(data: &mut [f32]) {
    if data.is_empty() { return; }

    let n = data.len() as f32;
    let mean = data.iter().sum::<f32>() / n;
    let var = data.iter().map(|&x| (x - mean) * (x - mean)).sum::<f32>() / n;
    let std = var.sqrt();

    if std < 1e-7 {
        data.iter_mut().for_each(|x| *x = 0.0);
        return;
    }

    data.iter_mut().for_each(|x| *x = (*x - mean) / std);
}

/// Нормализовать срез по min-max в [0, 1]. Ноль alloc.
pub fn minmax_inplace(data: &mut [f32]) {
    if data.is_empty() { return; }
    let min = data.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;
    if range < 1e-7 {
        data.iter_mut().for_each(|x| *x = 0.0);
        return;
    }
    data.iter_mut().for_each(|x| *x = (*x - min) / range);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zscore_basic() {
        let mut data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
        zscore_inplace(&mut data);
        let mean: f32 = data.iter().sum::<f32>() / data.len() as f32;
        assert!(mean.abs() < 1e-5, "mean after zscore should be ~0");
    }

    #[test]
    fn test_zscore_constant() {
        let mut data = vec![5.0f32; 10];
        zscore_inplace(&mut data);
        for &v in &data { assert_eq!(v, 0.0); }
    }

    #[test]
    fn test_zscore_empty() {
        let mut data: Vec<f32> = vec![];
        zscore_inplace(&mut data); // не паникует
    }

    #[test]
    fn test_minmax_range() {
        let mut data = vec![0.0f32, 5.0, 10.0];
        minmax_inplace(&mut data);
        assert!((data[0] - 0.0).abs() < 1e-6);
        assert!((data[1] - 0.5).abs() < 1e-6);
        assert!((data[2] - 1.0).abs() < 1e-6);
    }
}
