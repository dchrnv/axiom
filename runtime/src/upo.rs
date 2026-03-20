// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// UPO v2.2 — docs/spec/UPO v2.2.md

use crate::connection::Connection;
use crate::token::Token;

/// Источники следа
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TraceSourceType {
    Token = 1,
    Connection = 2,
    Field = 3,
}

/// Флаги следа
pub const TRACE_ACTIVE: u8 = 1;
pub const TRACE_FADING: u8 = 2;
pub const TRACE_LOCKED: u8 = 4;
pub const TRACE_ETERNAL: u8 = 8;

/// UPO DynamicTrace — 32 байта
/// UPO V2.3: координаты i16, один timestamp
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug)]
pub struct DynamicTrace {
    // --- ВРЕМЯ [8 байт] ---
    pub last_update: u64,       // 8b  | COM event_id последнего обновления

    // --- ХАРАКТЕРИСТИКИ [8 байт] ---
    pub weight: f32,            // 4b  | Вес/интенсивность точки
    pub frequency: f32,         // 4b  | Частота колебаний

    // --- ИСТОЧНИК [8 байт] ---
    pub source_id: u32,         // 4b  | ID источника (Token/Connection)
    pub x: i16,                 // 2b  | Координата X на экране
    pub y: i16,                 // 2b  | Координата Y на экране

    // --- МЕТАДАННЫЕ [8 байт] ---
    pub z: i16,                 // 2b  | Координата Z на экране
    pub source_type: u8,        // 1b  | Источник (Token/Connection/Field)
    pub flags: u8,              // 1b  | ACTIVE/FADING/LOCKED/ETERNAL
    pub resonance_class: u8,    // 1b  | Класс резонанса
    pub _pad: [u8; 3],          // 3b  | Явный padding (можно использовать в будущем)
}

impl DynamicTrace {
    pub fn new(
        x: i16, y: i16, z: i16,
        weight: f32, frequency: f32,
        last_update: u64,
        source_type: TraceSourceType,
        source_id: u32,
        resonance_class: u8,
    ) -> Self {
        Self {
            last_update,
            weight,
            frequency,
            source_id,
            x,
            y,
            z,
            source_type: source_type as u8,
            flags: TRACE_ACTIVE,
            resonance_class,
            _pad: [0; 3],
        }
    }

    pub fn is_active(&self) -> bool {
        self.flags & TRACE_ACTIVE != 0
    }

    pub fn is_fading(&self) -> bool {
        self.flags & TRACE_FADING != 0
    }

    pub fn is_eternal(&self) -> bool {
        self.flags & TRACE_ETERNAL != 0
    }

    /// Валидация согласно спецификации UPO v2.3
    pub fn validate(&self, screen: &Screen) -> bool {
        let x_i32 = i32::from(self.x);
        let y_i32 = i32::from(self.y);
        let z_i32 = i32::from(self.z);

        self.weight >= screen.min_intensity
        && self.last_update > 0
        && x_i32 >= -screen.size[0]/2 && x_i32 <= screen.size[0]/2
        && y_i32 >= -screen.size[1]/2 && y_i32 <= screen.size[1]/2
        && z_i32 >= -screen.size[2]/2 && z_i32 <= screen.size[2]/2
    }
}

/// Статистика по октанту
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OctantStats {
    pub trace_count: u32,       // Количество следов в октанте
    pub total_energy: f32,       // Общая энергия
    pub dominant_frequency: f32,  // Доминирующая частота
    pub last_event_id: u64,      // Последний event_id
}

/// Экран — 3D пространство следов с затуханием по event_id.
pub struct Screen {
    // --- ПАРАМЕТРЫ (32 Байта) ---
    pub size: [i32; 3],        // Размеры экрана (X, Y, Z)
    pub resolution: f32,        // Разрешение (единица на пиксель)
    pub min_intensity: f32,     // Минимальная интенсивность (> 0)
    pub decay_rate: f32,        // Скорость затухания
    pub current_event_id: u64,  // Текущий COM event_id
    pub trace_count: u32,       // Количество следов
    pub total_energy: f32,       // Общая энергия экрана
    pub octant_mask: u8,        // Маска активных октантов

    // --- ДАННЫЕ (динамические) ---
    pub traces: Vec<DynamicTrace>, // Массив следов
    pub octants: [OctantStats; 8], // Статистика по октантам
}

impl Screen {
    pub fn new(size: [i32; 3], min_intensity: f32, decay_rate: f32) -> Self {
        Self {
            size,
            resolution: 1.0,
            min_intensity,
            decay_rate,
            current_event_id: 0,
            trace_count: 0,
            total_energy: 0.0,
            octant_mask: 0xFF, // Все октанты активны
            traces: Vec::new(),
            octants: [OctantStats {
                trace_count: 0,
                total_energy: 0.0,
                dominant_frequency: 0.0,
                last_event_id: 0,
            }; 8],
        }
    }

    pub fn set_current_event(&mut self, event_id: u64) {
        self.current_event_id = event_id;
        self.apply_decay();
    }

    /// Добавление следа на экран
    pub fn write(&mut self, trace: &DynamicTrace) {
        if !trace.validate(self) {
            return;
        }

        // Обновление октанта
        let octant = self.get_octant(trace.x.into(), trace.y.into(), trace.z.into());
        self.octants[octant].trace_count += 1;
        self.octants[octant].total_energy += trace.weight;
        self.octants[octant].dominant_frequency = trace.frequency;
        self.octants[octant].last_event_id = trace.last_update;

        self.traces.push(*trace);
        self.trace_count += 1;
        self.total_energy += trace.weight;
    }

    /// Применение затухания ко всем следам
    pub fn apply_decay(&mut self) {
        for trace in &mut self.traces {
            let event_age = self.current_event_id - trace.last_update;
            let decay_factor = (-(event_age as f32) * self.decay_rate).exp();
            
            trace.weight = (trace.weight * decay_factor).max(self.min_intensity);
            
            // Вечная память - если вес достиг минимума
            if trace.weight == self.min_intensity {
                trace.flags |= TRACE_ETERNAL;
            }
            
            // Флаг затухания - если вес уменьшился
            if event_age > 0 {
                trace.flags |= TRACE_FADING;
            }
        }
    }

    /// Получение октанта по координатам
    fn get_octant(&self, x: i32, y: i32, z: i32) -> usize {
        let x_oct = if x >= 0 { 1 } else { 0 };
        let y_oct = if y >= 0 { 1 } else { 0 };
        let z_oct = if z >= 0 { 1 } else { 0 };
        (x_oct << 2) | (y_oct << 1) | z_oct
    }

    /// Очистка старых следов
    pub fn cleanup(&mut self, max_age: u64) {
        self.traces.retain(|trace| {
            self.current_event_id - trace.last_update <= max_age || trace.is_eternal()
        });
        self.trace_count = self.traces.len() as u32;
    }
}

/// Режим обновления UPO.
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum UpdateMode {
    OnEvent,
    Periodic(u64),
    OnDemand,
}

/// Конфигурация UPO.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct UPOConfig {
    pub domain_id: u16,
    pub update_mode: UpdateMode,
    pub screen_size: [i32; 3],    // Размеры экрана
    pub min_intensity: f32,        // Минимальная интенсивность
    pub decay_rate: f32,           // Скорость затухания
    pub use_connections: bool,      // Использовать Connection
    pub min_tokens: usize,         // Минимальное количество токенов
    pub resonance_threshold: f32,   // Порог резонанса
}

impl Default for UPOConfig {
    fn default() -> Self {
        Self {
            domain_id: 0,
            update_mode: UpdateMode::OnEvent,
            screen_size: [256, 256, 256],
            min_intensity: 0.001,
            decay_rate: 0.01,
            use_connections: false,
            min_tokens: 1,
            resonance_threshold: 0.5,
        }
    }
}

/// UPO — вычисление метрик и генерация DynamicTrace.
#[allow(dead_code)]
pub struct UPO {
    config: UPOConfig,
}

#[allow(dead_code)]
impl UPO {
    pub fn new(config: UPOConfig) -> Self {
        Self {
            config,
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

        if active_tokens.len() < self.config.min_tokens {
            return None;
        }

        // Вычисление Token динамики
        let token_trace = self.compute_token_dynamics(&active_tokens, event_id)?;
        
        // Вычисление Connection стресса если включено
        let connection_stress = if self.config.use_connections {
            self.compute_connection_stress(connections)
        } else {
            0.0
        };

        // Комбинированный след
        let combined_trace = DynamicTrace::new(
            token_trace.x,
            token_trace.y,
            token_trace.z,
            token_trace.weight + connection_stress,
            token_trace.frequency,
            token_trace.last_update,
            TraceSourceType::Token,
            active_tokens[0].sutra_id,
            self.compute_resonance_class(&active_tokens),
        );

        Some(combined_trace)
    }

    /// Вычисление динамики Token
    fn compute_token_dynamics(&self, tokens: &[&Token], event_id: u64) -> Option<DynamicTrace> {
        if tokens.is_empty() {
            return None;
        }

        // Средняя скорость
        let avg_velocity = self.compute_average_velocity(tokens);

        // Позиция на экране (UPO V2.3: координаты i16)
        let x = (avg_velocity[0] * 100.0) as i16;
        let y = (avg_velocity[1] * 100.0) as i16;
        let z = (avg_velocity[2] * 100.0) as i16;

        // Вес и частота
        let total_mass: f32 = tokens.iter().map(|t| t.mass as f32).sum();
        let avg_temperature: f32 = tokens.iter().map(|t| t.temperature as f32).sum::<f32>() / tokens.len() as f32;
        let weight = total_mass * (avg_temperature / 255.0);
        let frequency = self.compute_average_frequency(tokens);

        Some(DynamicTrace::new(
            x, y, z,
            weight,
            frequency,
            event_id,
            TraceSourceType::Token,
            tokens[0].sutra_id,
            0,
        ))
    }

    /// Вычисление средней скорости
    fn compute_average_velocity(&self, tokens: &[&Token]) -> [f32; 3] {
        let mut sum = [0.0; 3];
        let total_mass: f32 = tokens.iter().map(|t| t.mass as f32).sum();
        
        for token in tokens {
            let mass = token.mass as f32;
            for i in 0..3 {
                sum[i] += (token.velocity[i] as f32) * mass;
            }
        }
        
        for i in 0..3 {
            sum[i] /= total_mass;
        }
        
        sum
    }

    /// Вычисление средней частоты
    fn compute_average_frequency(&self, tokens: &[&Token]) -> f32 {
        let sum: f32 = tokens.iter().map(|t| t.resonance as f32).sum();
        sum / tokens.len() as f32
    }

    /// Вычисление стресса от Connection
    fn compute_connection_stress(&self, connections: &[Connection]) -> f32 {
        let active_conns: Vec<_> = connections
            .iter()
            .filter(|c| c.domain_id == self.config.domain_id && c.is_active())
            .collect();

        if active_conns.is_empty() {
            return 0.0;
        }

        let total_stress: f32 = active_conns.iter()
            .map(|c| c.current_stress / c.strength)
            .sum();

        total_stress / active_conns.len() as f32
    }

    /// Вычисление класса резонанса
    fn compute_resonance_class(&self, tokens: &[&Token]) -> u8 {
        if tokens.is_empty() {
            return 0;
        }

        let avg_resonance = tokens.iter()
            .map(|t| t.resonance)
            .sum::<u32>() / tokens.len() as u32;

        // Классификация резонанса (0-255)
        (avg_resonance / 1000).min(255) as u8
    }
}
