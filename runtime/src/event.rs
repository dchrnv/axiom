// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// COM: Event, Snapshot — docs/spec/CAUSAL ORDER MODEL (COM).md

use crate::clock::CausalClock;

/// Тип события (причинный порядок).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EventType {
    UpdateToken = 0,
    UpdateConnection = 1,
    Tick = 2,
    Custom(u16),
}

impl From<u16> for EventType {
    fn from(v: u16) -> Self {
        match v {
            0 => EventType::UpdateToken,
            1 => EventType::UpdateConnection,
            2 => EventType::Tick,
            n => EventType::Custom(n),
        }
    }
}

/// Событие — изменение состояния с монотонным причинным индексом.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub event_id: u64,
    pub domain_id: u16,
    pub event_type: u16,
    pub payload_hash: u64,
    pub _reserved: [u8; 8],
}

impl Event {
    pub fn new(domain_id: u16, event_type: EventType, payload_hash: u64) -> Self {
        let event_id = CausalClock::next();
        let et = match event_type {
            EventType::UpdateToken => 0,
            EventType::UpdateConnection => 1,
            EventType::Tick => 2,
            EventType::Custom(n) => n,
        };
        Self {
            event_id,
            domain_id,
            event_type: et,
            payload_hash,
            _reserved: [0; 8],
        }
    }

    pub fn with_id(event_id: u64, domain_id: u16, event_type: u16, payload_hash: u64) -> Self {
        Self {
            event_id,
            domain_id,
            event_type,
            payload_hash,
            _reserved: [0; 8],
        }
    }
}

/// Снапшот состояния после применения событий до snapshot_id.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub snapshot_id: u64,
    pub state_hash: u64,
}
