// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

/// Частота сборки: каждый тик (Level 0 — пульс).
pub const PULSE_INTERVAL: u64 = 1;
/// Частота сборки: каждые 8 тиков (Level 1 — состояние).
pub const STATE_INTERVAL: u64 = 8;
/// Частота сборки: каждые 32 тика (Level 2 — полный срез).
pub const FULL_INTERVAL: u64 = 32;
// Level 3 (срез + память) активируется при выходе из DREAM — управляется извне.

/// Глубина сбора среза.
///
/// Чем глубже — тем дороже собрать и тем больше полей заполнено.
/// Уровни не исключают друг друга: Full включает State, State включает Pulse.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionLevel {
    /// Пульс: ~5 дешёвых полей. Каждый тик. → мандала Workstation.
    Pulse = 0,
    /// Состояние: активные подсистемы, дилеммы, hotspots. Каждые 8 тиков.
    State = 1,
    /// Полный срез: всё из §2 спеки. Каждые 32 тика.
    Full = 2,
    /// Срез + память из EXPERIENCE. При DREAM-пробуждении.
    Memory = 3,
}

impl CollectionLevel {
    /// Определить уровень сборки для данного тика (детерминированно по event_id).
    pub fn for_tick(tick: u64) -> Self {
        if tick % FULL_INTERVAL == 0 {
            CollectionLevel::Full
        } else if tick % STATE_INTERVAL == 0 {
            CollectionLevel::State
        } else {
            CollectionLevel::Pulse
        }
    }
}
