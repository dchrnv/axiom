/// C4: BroadcastingConfig — параметры throttling и dropping.
/// Доступна через Configuration tab (Engine → Broadcasting).
#[derive(Debug, Clone)]
pub struct BroadcastingConfig {
    /// Send Tick event every N engine ticks.
    pub tick_event_interval: u32,
    /// Send DomainActivity only when recent_activity delta >= N.
    pub domain_activity_threshold: u32,
    /// Max pending messages per client before dropping.
    pub max_event_queue_per_client: usize,
    pub event_drop_strategy: DropStrategy,
    /// Trigger full Snapshot resync when queue reaches this size.
    pub snapshot_resync_threshold: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropStrategy {
    /// Drop oldest events first — client always gets latest state.
    DropOldest,
    /// Drop newest events first.
    DropNewest,
}

impl Default for BroadcastingConfig {
    fn default() -> Self {
        Self {
            tick_event_interval: 100,
            domain_activity_threshold: 5,
            max_event_queue_per_client: 1000,
            event_drop_strategy: DropStrategy::DropOldest,
            snapshot_resync_threshold: 800,
        }
    }
}
