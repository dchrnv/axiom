use serde::{Deserialize, Serialize};

use crate::adapters::AdapterProgress;
use crate::events::EngineState;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SystemSnapshot {
    pub engine_state: EngineState,
    pub current_tick: u64,
    pub current_event: u64,

    pub domains: Vec<DomainSnapshot>,
    pub over_domain: OverDomainSnapshot,
    pub fatigue: FatigueSnapshot,
    pub last_dream_report: Option<DreamReport>,

    pub frame_weaver_stats: Option<FrameWeaverStats>,
    pub guardian_stats: GuardianStats,
    pub dream_phase_stats: DreamPhaseStats,

    pub adapter_progress: Vec<AdapterProgress>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DomainSnapshot {
    pub id: u16,
    pub name: String,
    pub config_summary: DomainConfigSummary,
    pub token_count: u32,
    pub connection_count: u32,
    pub temperature_avg: u8,
    pub recent_activity: u32,
    /// Activity per semantic layer (8 layers).
    pub layer_activations: [u8; 8],
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DomainConfigSummary {
    pub capacity: u32,
    pub temperature_decay: u8,
}

/// Aggregate view of the Over-Domain coordination layer.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OverDomainSnapshot {
    pub total_tokens: u32,
    pub total_connections: u32,
    pub cross_domain_events_recent: u32,
    /// Aggregate layer activations across all domains.
    pub layer_activations: [u8; 8],
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FatigueSnapshot {
    /// Normalised fatigue level 0.0..1.0.
    pub current: f32,
    pub threshold: f32,
    /// Ticks elapsed since the last dream cycle ended.
    pub ticks_since_dream: u64,
    /// Recent token-addition rate (tokens per tick, rolling window).
    pub token_rate: f32,
    /// Last N samples — used for sparkline in Dream State window.
    pub history: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DreamReport {
    pub cycle_id: u64,
    pub started_at_tick: u64,
    pub ended_at_tick: u64,
    pub proposals_accepted: u32,
    pub proposals_rejected: u32,
    pub sutra_written: u32,
    pub fatigue_before: f32,
    pub fatigue_after: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FrameWeaverStats {
    pub total_frames: u32,
    pub frames_in_sutra: u32,
    /// Promotions since last Wake transition.
    pub promotions_since_wake: u32,
    pub last_crystallization_tick: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GuardianStats {
    pub total_vetoes: u64,
    /// Vetoes since last Wake transition.
    pub vetoes_since_wake: u32,
    pub last_veto_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DreamPhaseStats {
    pub cycles_completed: u64,
    pub last_transition_tick: u64,
}

/// Returned by RequestFrameDetails command.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FrameDetails {
    pub anchor_id: u32,
    pub layers_present: u8,
    pub participant_count: u8,
    pub temperature: u8,
    pub crystallized_at_tick: u64,
    pub last_reactivated_at_tick: Option<u64>,
    pub promotion_rule: Option<String>,
}
