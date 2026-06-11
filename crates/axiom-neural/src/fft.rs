// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// FFT-frontend: превращает ActivityTrace ring в спектр внутренних ритмов.
// Один экземпляр на советника — буферы предвыделены, ноль alloc в compute().

use rustfft::{FftPlanner, num_complex::Complex, Fft};
use std::sync::Arc;

/// FFT-frontend для одного размера окна.
/// Предвыделяет все буферы при new() — compute() не аллоцирует.
pub struct FftFrontend {
    fft: Arc<dyn Fft<f32>>,
    /// Рабочий буфер (in-place FFT).
    scratch: Vec<Complex<f32>>,
    /// Внутренний буфер scratch для rustfft.
    fft_scratch: Vec<Complex<f32>>,
    /// Размер окна.
    size: usize,
}

impl FftFrontend {
    /// Создать для заданного размера окна (должен быть степенью 2 для скорости).
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(size);
        let scratch_len = fft.get_inplace_scratch_len();
        Self {
            fft,
            scratch: vec![Complex::new(0.0, 0.0); size],
            fft_scratch: vec![Complex::new(0.0, 0.0); scratch_len],
            size,
        }
    }

    /// Число выходных компонент (half-spectrum: size/2 + 1).
    pub fn output_len(&self) -> usize {
        self.size / 2 + 1
    }

    /// Преобразовать ring-буфер → magnitude spectrum.
    ///
    /// `input`: срез длиной `self.size` (ActivityTrace ring).
    /// `output`: предвыделенный срез длиной `self.output_len()`.
    /// Ноль alloc.
    pub fn compute(&mut self, input: &[f32], output: &mut [f32]) {
        debug_assert_eq!(input.len(), self.size);
        debug_assert_eq!(output.len(), self.output_len());

        // Заполняем scratch комплексными значениями
        for (c, &v) in self.scratch.iter_mut().zip(input.iter()) {
            *c = Complex::new(v, 0.0);
        }

        self.fft.process_with_scratch(&mut self.scratch, &mut self.fft_scratch);

        // Берём magnitude первой половины спектра
        let norm = (self.size as f32).sqrt();
        for (o, c) in output.iter_mut().zip(self.scratch.iter().take(self.output_len())) {
            *o = c.norm() / norm;
        }
    }
}

/// Набор из нескольких FFT-планов для ActivityTrace rings разного размера.
///
/// Short=16, Mid=64, Long=256 — три плана, один экземпляр на советника.
pub struct ActivityFft {
    pub short: FftFrontend,   // size=16  → 9 компонент
    pub mid: FftFrontend,     // size=64  → 33 компоненты
    pub long: FftFrontend,    // size=256 → 129 компонент
    /// Предвыделённые буферы для результатов.
    pub short_out: Vec<f32>,
    pub mid_out: Vec<f32>,
    pub long_out: Vec<f32>,
}

impl ActivityFft {
    pub fn new() -> Self {
        let short = FftFrontend::new(16);
        let mid = FftFrontend::new(64);
        let long = FftFrontend::new(256);
        let short_out = vec![0.0; short.output_len()];
        let mid_out = vec![0.0; mid.output_len()];
        let long_out = vec![0.0; long.output_len()];
        Self { short, mid, long, short_out, mid_out, long_out }
    }

    /// Полный размер выходного спектра для одной подсистемы (9+33+129=171).
    pub fn total_output_len(&self) -> usize {
        self.short.output_len() + self.mid.output_len() + self.long.output_len()
    }

    /// Вычислить FFT для трёх колец одной подсистемы, записать в out_slice.
    /// `out_slice` должен быть длиной `total_output_len()`.
    pub fn compute_rings(&mut self, short_ring: &[f32], mid_ring: &[f32], long_ring: &[f32], out_slice: &mut [f32]) {
        let s_len = self.short.output_len();
        let m_len = self.mid.output_len();
        let l_len = self.long.output_len();

        self.short.compute(short_ring, &mut self.short_out);
        self.mid.compute(mid_ring, &mut self.mid_out);
        self.long.compute(long_ring, &mut self.long_out);

        out_slice[..s_len].copy_from_slice(&self.short_out);
        out_slice[s_len..s_len + m_len].copy_from_slice(&self.mid_out);
        out_slice[s_len + m_len..s_len + m_len + l_len].copy_from_slice(&self.long_out);
    }
}

impl Default for ActivityFft {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_output_len() {
        let f = FftFrontend::new(16);
        assert_eq!(f.output_len(), 9);
        let f64 = FftFrontend::new(64);
        assert_eq!(f64.output_len(), 33);
        let f256 = FftFrontend::new(256);
        assert_eq!(f256.output_len(), 129);
    }

    #[test]
    fn test_fft_zeros_silent() {
        let mut f = FftFrontend::new(16);
        let input = vec![0.0f32; 16];
        let mut out = vec![0.0f32; f.output_len()];
        f.compute(&input, &mut out);
        for &v in &out { assert!(v.abs() < 1e-6, "silence → zero spectrum"); }
    }

    #[test]
    fn test_fft_dc_component() {
        let mut f = FftFrontend::new(16);
        let input = vec![1.0f32; 16];
        let mut out = vec![0.0f32; f.output_len()];
        f.compute(&input, &mut out);
        // DC-компонента (out[0]) должна быть ненулевой
        assert!(out[0] > 0.0, "DC component should be nonzero for constant signal");
    }

    #[test]
    fn test_activity_fft_total_len() {
        let af = ActivityFft::new();
        assert_eq!(af.total_output_len(), 9 + 33 + 129);
    }

    #[test]
    fn test_activity_fft_no_alloc_in_compute() {
        let mut af = ActivityFft::new();
        let short = vec![0.0f32; 16];
        let mid = vec![0.0f32; 64];
        let long = vec![0.0f32; 256];
        let mut out = vec![0.0f32; af.total_output_len()];
        // Просто проверяем что не паникует
        af.compute_rings(&short, &mid, &long, &mut out);
    }
}
