use serde::{Deserialize, Serialize};

use crate::adapters::AdapterProgress;
use crate::events::EngineState;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SystemSnapshot {
    pub engine_state: EngineState,
    pub current_tick: u64,
    pub current_event: u64,
    /// Duration of the last engine tick in nanoseconds (0 = not yet measured).
    pub hot_path_ns: u64,

    pub domains: Vec<DomainSnapshot>,
    pub over_domain: OverDomainSnapshot,
    pub fatigue: FatigueSnapshot,
    pub last_dream_report: Option<DreamReport>,

    pub frame_weaver_stats: Option<FrameWeaverStats>,
    pub guardian_stats: GuardianStats,
    pub dream_phase_stats: DreamPhaseStats,

    pub adapter_progress: Vec<AdapterProgress>,

    pub phase_c: Option<PhaseCSnapshot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TokenFieldPoint {
    pub position: [f32; 3],
    pub layer: u8,
    pub temperature: u8,
    pub anchor_membership: Option<u32>,
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
    /// Sampled token positions for Live Field (max 300 per domain).
    pub token_field: Vec<TokenFieldPoint>,
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
    /// Activation counts per syntactic layer (S1–S8) since last crystallization.
    pub syntactic_layer_activations: [u8; 8],
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

/// Pending emergent primitive candidate (for Workstation approval panel).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmergentCandidateSnapshot {
    pub sutra_id: u32,
    /// Octant as u8 (0=CreativeAffirmation … 7=SelfDestructiveApathic).
    pub discovery_octant: u8,
    pub initial_depth: u16,
}

/// NeuralAdvisor summary for one Frame (advisory-only, read-only).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdvisoryFrameSnapshot {
    pub anchor_id: u32,
    pub has_octant_suggestion: bool,
    pub has_conflict: bool,
    pub has_subsystem_suggestion: bool,
    pub has_depth_hint: bool,
}

/// Phase C state snapshot — AxialEvaluator + ContextRecognizer + NeuralAdvisor.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhaseCSnapshot {
    /// Most common octant across all frame evaluations (None = no evaluations yet).
    pub dominant_octant: Option<u8>,
    /// Most common primary subsystem as u8 (0=Writing…5=Unknown). None = no profiles yet.
    pub dominant_subsystem: Option<u8>,
    /// Total number of pending emergent primitive candidates.
    pub pending_emergent_count: u32,
    /// Top-20 pending candidates for the Workstation approval panel.
    pub emergent_candidates: Vec<EmergentCandidateSnapshot>,
    /// Frames with at least one active NeuralAdvisor recommendation.
    pub advisory_frames: Vec<AdvisoryFrameSnapshot>,
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
