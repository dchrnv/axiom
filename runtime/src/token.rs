// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Token V5.1: docs/spec/Token V5.1.md

/// Состояние Token: Active, Sleeping, Locked...
pub const STATE_ACTIVE: u8 = 1;
pub const STATE_SLEEPING: u8 = 2;
pub const STATE_LOCKED: u8 = 3;

/// Token — 64 байта, выравнивание 64.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Token {
    // --- ИДЕНТИФИКАЦИЯ (8 Байт) ---
    pub sutra_id: u32,
    pub domain_id: u16,
    pub type_flags: u16,

    // --- ЛОКАЛЬНАЯ ФИЗИКА ПОЛЯ (16 Байт) ---
    pub position: [i16; 3],
    pub velocity: [i16; 3],
    pub target: [i16; 3],
    pub reserved_phys: u16,

    // --- ТЕРМОДИНАМИКА (4 Байта) ---
    pub valence: i8,
    pub mass: u8,
    pub temperature: u8,
    pub state: u8,

    // --- ФРАКТАЛЬНАЯ НАВИГАЦИЯ (36 Байт) ---
    pub lineage_hash: u64,
    pub momentum: [i32; 3],
    pub resonance: u32,
    pub last_event_id: u64,
    pub reserved_nav: [u8; 4],
}

impl Default for Token {
    fn default() -> Self {
        Self {
            sutra_id: 0,
            domain_id: 0,
            type_flags: 0,
            position: [0; 3],
            velocity: [0; 3],
            target: [0; 3],
            reserved_phys: 0,
            valence: 0,
            mass: 1,
            temperature: 0,
            state: STATE_ACTIVE,
            lineage_hash: 0,
            momentum: [0; 3],
            resonance: 0,
            last_event_id: 0,
            reserved_nav: [0; 4],
        }
    }
}

impl Token {
    pub fn new(sutra_id: u32, domain_id: u16) -> Self {
        Self {
            sutra_id,
            domain_id,
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.state == STATE_ACTIVE
    }

    #[inline]
    pub fn is_sleeping(&self) -> bool {
        self.state == STATE_SLEEPING
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.state == STATE_LOCKED
    }

    /// Валидация согласно спецификации Token V5.1
    pub fn validate(&self) -> bool {
        self.sutra_id > 0
        && self.domain_id > 0
        && self.mass > 0
        && self.last_event_id > 0
        && self.position.iter().all(|&p| p >= i16::MIN && p <= i16::MAX)
    }

    /// Обновление momentum через COM событие
    pub fn update_momentum(&mut self, force: [i32; 3], event_id: u64) {
        self.momentum[0] += force[0];
        self.momentum[1] += force[1];
        self.momentum[2] += force[2];
        self.last_event_id = event_id;
    }

    /// Вычисление резонанса с другим токеном
    pub fn compute_resonance(&self, other: &Token) -> u32 {
        let freq_diff = (self.resonance as i32 - other.resonance as i32).abs();
        (1000 - freq_diff.min(999)) as u32
    }
}
