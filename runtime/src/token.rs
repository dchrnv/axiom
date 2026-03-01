// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// UPO v2.1: Token — docs/spec/UPO v2.1.md §2.1

/// Состояние Token: Active, Sleeping, Locked...
pub const STATE_ACTIVE: u8 = 1;

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
    pub context_payload: [u8; 28],
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
            context_payload: [0; 28],
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
}
