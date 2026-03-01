// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// UPO v2.1 — docs/spec/UPO v2.1.md

use std::f32::consts::PI;

use crate::connection::Connection;
use crate::token::{Token, STATE_ACTIVE};

/// Режим обновления UPO.
#[derive(Clone, Copy, Debug)]
pub enum UpdateMode {
    OnEvent,
    Periodic(u64),
    OnDemand,
}

/// DynamicTrace — 32 байта. Запись на экран.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DynamicTrace {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub weight: f32,
    pub event_id: u64,
    pub _reserved: [u8; 8],
}

impl DynamicTrace {
    pub fn new(x: f32, y: f32, z: f32, weight: f32, event_id: u64) -> Self {
        Self {
            x: x.clamp(-1.0, 1.0),
            y: y.clamp(-1.0, 1.0),
            z: z.clamp(-1.0, 1.0),
            weight,
            event_id,
            _reserved: [0; 8],
        }
    }
}

/// Экран — 256³, ленивое затухание по event_id.
pub const GRID_SIZE: usize = 256;

pub struct Screen {
    pub grid: Box<[[[f32; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]>,
    pub last_event: Box<[[[u64; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]>,
    pub decay: f32,
    pub min_intensity: f32,
    pub current_event: u64,
}

impl Screen {
    pub fn new(decay: f32, min_intensity: f32) -> Self {
        Self {
            grid: Box::new([[[0.0_f32; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]),
            last_event: Box::new([[[0_u64; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]),
            decay,
            min_intensity,
            current_event: 0,
        }
    }

    pub fn set_current_event(&mut self, event_id: u64) {
        self.current_event = event_id;
    }

    /// Запись DynamicTrace на экран с ленивым затуханием.
    pub fn write(&mut self, trace: &DynamicTrace) {
        let ix = ((trace.x + 1.0) * 0.5 * (GRID_SIZE as f32 - 0.001)).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;
        let iy = ((trace.y + 1.0) * 0.5 * (GRID_SIZE as f32 - 0.001)).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;
        let iz = ((trace.z + 1.0) * 0.5 * (GRID_SIZE as f32 - 0.001)).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;

        let last = self.last_event[ix][iy][iz];
        if last < self.current_event {
            let steps = self.current_event - last;
            let decay_factor = self.decay.powi(steps as i32);
            let v = &mut self.grid[ix][iy][iz];
            *v *= decay_factor;
            if v.abs() < self.min_intensity {
                *v = self.min_intensity * v.signum();
            }
        }

        self.grid[ix][iy][iz] += trace.weight;
        self.last_event[ix][iy][iz] = self.current_event;
    }
}

/// Конфигурация UPO.
#[derive(Clone, Debug)]
pub struct UPOConfig {
    pub domain_id: u16,
    pub update_mode: UpdateMode,
    pub event_threshold: f32,
    pub smoothing: f32,
    pub alpha1: f32,
    pub alpha2: f32,
    pub alpha3: f32,
    pub beta: f32,
    pub gamma: f32,
    pub delta: f32,
    pub use_connections: bool,
    pub min_tokens: usize,
    pub v_max_adaptive: f32,
    pub stress_max: f32,
}

impl Default for UPOConfig {
    fn default() -> Self {
        Self {
            domain_id: 0,
            update_mode: UpdateMode::OnEvent,
            event_threshold: 0.0,
            smoothing: 0.5,
            alpha1: 1.0,
            alpha2: 0.5,
            alpha3: 0.5,
            beta: 1.0,
            gamma: 1.0,
            delta: 0.5,
            use_connections: false,
            min_tokens: 1,
            v_max_adaptive: 1.0,
            stress_max: 1.0,
        }
    }
}

/// UPO — вычисление метрик и генерация DynamicTrace.
pub struct UPO {
    config: UPOConfig,
    prev_avg_vel: [f32; 3],
}

impl UPO {
    pub fn new(config: UPOConfig) -> Self {
        Self {
            config,
            prev_avg_vel: [0.0; 3],
        }
    }

    /// Вычисляет DynamicTrace по срезам Token и Connection.
    pub fn compute(
        &mut self,
        tokens: &[Token],
        connections: &[Connection],
        event_id: u64,
    ) -> Option<DynamicTrace> {
        let active_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.domain_id == self.config.domain_id && t.is_active())
            .collect();

        if active_tokens.is_empty() {
            return None;
        }

        let total_mass: u64 = active_tokens.iter().map(|t| t.mass as u64).sum();
        if total_mass == 0 {
            return None;
        }
        let total_mass_f = total_mass as f32;

        // avg_vel
        let mut avg_vel = [0.0_f32; 3];
        for t in &active_tokens {
            let m = t.mass as f32;
            for i in 0..3 {
                avg_vel[i] += (t.velocity[i] as f32) * m;
            }
        }
        for i in 0..3 {
            avg_vel[i] /= total_mass_f;
        }

        let v = (avg_vel[0].powi(2) + avg_vel[1].powi(2) + avg_vel[2].powi(2)).sqrt();
        let v_norm = (v / self.config.v_max_adaptive).min(1.0);

        // кривизна: угол между prev_avg_vel и avg_vel
        let pv = &self.prev_avg_vel;
        let pv_norm = (pv[0].powi(2) + pv[1].powi(2) + pv[2].powi(2)).sqrt();
        let c = if pv_norm > 1e-9 && v > 1e-9 {
            let dot = pv[0] * avg_vel[0] + pv[1] * avg_vel[1] + pv[2] * avg_vel[2];
            let cos_angle = (dot / (pv_norm * v)).clamp(-1.0, 1.0);
            (1.0 - cos_angle) / 2.0
        } else {
            0.0
        };
        self.prev_avg_vel = avg_vel;
        let c_norm = (c / PI).min(1.0);

        // avg_temp (signed)
        let mut avg_temp = 0.0_f32;
        for t in &active_tokens {
            let sign = if t.valence >= 0 { 1.0 } else { -1.0 };
            let temp_norm = t.temperature as f32 / 255.0;
            avg_temp += sign * temp_norm * (t.mass as f32);
        }
        avg_temp /= total_mass_f;

        let avg_mass = total_mass_f / (active_tokens.len() as f32);
        let avg_mass_norm = (avg_mass / 255.0).min(1.0);

        let mut stress_norm = 0.0_f32;
        let mut total_stress_energy = 0.0_f32;
        let mut total_strength = 0.0_f32;

        if self.config.use_connections {
            let active_conns: Vec<_> = connections
                .iter()
                .filter(|c| c.domain_id == self.config.domain_id && c.is_active())
                .collect();
            for conn in &active_conns {
                total_stress_energy += conn.current_stress * conn.strength;
                total_strength += conn.strength;
            }
            if total_strength > 0.0 {
                let avg_stress = total_stress_energy / total_strength;
                stress_norm = (avg_stress / self.config.stress_max).min(1.0);
            }
        }

        let x = (self.config.alpha1 * v_norm + self.config.alpha2 * c_norm
            + if self.config.use_connections {
                self.config.alpha3 * stress_norm
            } else {
                0.0
            })
        .tanh();
        let y = (self.config.beta * (1.0 - v_norm) * (1.0 + avg_mass_norm)).tanh();
        let z = (self.config.gamma * avg_temp).tanh();

        let confidence = (active_tokens.len() as f32 / self.config.min_tokens as f32).min(1.0);
        let total_stress_norm = if self.config.stress_max > 0.0 {
            total_stress_energy / self.config.stress_max
        } else {
            0.0
        };
        let weight = if self.config.use_connections {
            (avg_mass_norm * avg_temp + self.config.delta * total_stress_norm) * confidence
        } else {
            avg_mass_norm * avg_temp * confidence
        };

        Some(DynamicTrace::new(x, y, z, weight, event_id))
    }
}
