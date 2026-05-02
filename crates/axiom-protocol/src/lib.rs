pub const PROTOCOL_VERSION: u32 = 0x01_00_00_00; // 1.0.0

pub mod adapters;
pub mod bench;
pub mod commands;
pub mod config;
pub mod events;
pub mod messages;
pub mod snapshot;

#[cfg(test)]
mod tests;

pub mod event_category {
    pub const TICK: u64           = 1 << 0;
    pub const DOMAIN_ACTIVITY: u64 = 1 << 1;
    pub const DREAM_PHASE: u64    = 1 << 2;
    pub const FRAMES: u64         = 1 << 3;
    pub const GUARDIAN: u64       = 1 << 4;
    pub const ADAPTERS: u64       = 1 << 5;
    pub const BENCHMARKS: u64     = 1 << 6;
    pub const ALERTS: u64         = 1 << 7;

    pub const ALL: u64     = u64::MAX;
    /// All categories except Tick (Tick is high-frequency, UI uses periodic snapshot instead).
    pub const DEFAULT: u64 = ALL & !TICK;
}
