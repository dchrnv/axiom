// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// UPO v2.1: Connection — docs/spec/UPO v2.1.md §2.2

/// Флаг активной связи (flags & ACTIVE != 0).
pub const FLAG_ACTIVE: u32 = 1;

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
    pub last_active: u64,
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
            last_active: 0,
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
}
