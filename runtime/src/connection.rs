// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Connection V5.0: docs/spec/Connection V5.0.md

/// Флаги Connection
pub const FLAG_ACTIVE: u32 = 1;
pub const FLAG_INHIBITED: u32 = 2;
pub const FLAG_TEMPORARY: u32 = 4;
pub const FLAG_CRITICAL: u32 = 8;

/// Connection — 64 байта, выравнивание 64.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    // --- ТОПОЛОГИЯ (16 Байт) ---
    pub source_id: u32,
    pub target_id: u32,
    pub domain_id: u16,
    pub link_type: u16,
    pub flags: u32,

    // --- ДИНАМИКА (16 Байт) ---
    pub strength: f32,
    pub current_stress: f32,
    pub ideal_dist: f32,
    pub elasticity: f32,

    // --- ШЛЮЗ (16 Байт) ---
    pub density_gate: u8,
    pub thermal_gate: u8,
    pub reserved_gate: [u8; 14],

    // --- МЕТАДАННЫЕ (16 Байт) ---
    pub created_at: u64,
    pub last_event_id: u64,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            source_id: 0,
            target_id: 0,
            domain_id: 0,
            link_type: 0,
            flags: 0,
            strength: 1.0,
            current_stress: 0.0,
            ideal_dist: 0.0,
            elasticity: 1.0,
            density_gate: 0,
            thermal_gate: 0,
            reserved_gate: [0; 14],
            created_at: 0,
            last_event_id: 0,
        }
    }
}

impl Connection {
    pub fn new(source_id: u32, target_id: u32, domain_id: u16) -> Self {
        Self {
            source_id,
            target_id,
            domain_id,
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.flags & FLAG_ACTIVE != 0
    }

    #[inline]
    pub fn is_inhibited(&self) -> bool {
        self.flags & FLAG_INHIBITED != 0
    }

    #[inline]
    pub fn is_temporary(&self) -> bool {
        self.flags & FLAG_TEMPORARY != 0
    }

    #[inline]
    pub fn is_critical(&self) -> bool {
        self.flags & FLAG_CRITICAL != 0
    }

    /// Валидация согласно спецификации Connection V5.0
    pub fn validate(&self) -> bool {
        self.source_id > 0
        && self.target_id > 0
        && self.domain_id > 0
        && self.strength > 0.0
        && self.current_stress >= 0.0
        && self.elasticity > 0.0
        && self.created_at > 0
        && self.last_event_id >= self.created_at
    }

    /// Проверка прохождения через шлюз массы
    pub fn can_pass_mass(&self, mass: u8) -> bool {
        mass >= self.density_gate
    }

    /// Проверка прохождения через шлюз температуры
    pub fn can_pass_temperature(&self, temperature: u8) -> bool {
        temperature <= self.thermal_gate
    }

    /// Обновление стресса через COM событие
    pub fn update_stress(&mut self, new_stress: f32, event_id: u64) {
        self.current_stress = new_stress.max(0.0);
        self.last_event_id = event_id;
        
        // Автоматическая установка критического флага
        if self.current_stress > self.strength * 0.8 {
            self.flags |= FLAG_CRITICAL;
        }
    }

    /// Вычисление расстояния между токенами
    pub fn compute_distance(&self, source_pos: [i16; 3], target_pos: [i16; 3]) -> f32 {
        let dx = (target_pos[0] - source_pos[0]) as f32;
        let dy = (target_pos[1] - source_pos[1]) as f32;
        let dz = (target_pos[2] - source_pos[2]) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
