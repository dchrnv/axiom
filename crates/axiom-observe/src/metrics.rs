/// Snapshot of engine state at a given tick.
#[derive(Debug, Clone)]
pub struct TickSnapshot {
    pub tick: u64,
    pub frame_count: usize,
    pub total_evaluations: usize,
    pub total_conflicts: usize,
    pub profile_count: usize,
    pub dominant_subsystem: Option<u8>,
    pub dominant_octant: Option<u8>,
    pub depth_store_len: usize,
    pub avg_depths: [u32; 8],
    pub emergent_pending: usize,
    pub emergent_approved: usize,
    /// Number of experience traces in the Arbiter
    pub experience_traces: usize,
    /// Number of tension traces (low-coherence unresolved patterns)
    pub tension_traces: usize,
}

/// Event recorded when a text is injected, with routing diagnostics.
#[derive(Debug, Clone)]
pub struct InjectionEvent {
    pub tick: u64,
    pub entry_id: String,
    pub expected_subsystem: Option<String>,
    pub detected_subsystem: Option<u8>,
    /// Coherence score from routing (None if routing didn't occur)
    pub coherence: Option<f32>,
    /// Whether a reflex was hit (fast path)
    pub reflex_hit: bool,
    /// Number of multi-pass iterations
    pub passes: u8,
    /// Cumulative experience traces at injection time
    pub experience_traces_at_injection: usize,
}
