// Mirrors axiom-protocol Rust types. Serde's default enum encoding:
//   unit variant   → "VariantName"
//   struct variant → { "VariantName": { ...fields } }
//   tuple variant  → { "VariantName": value }

export type EngineState = 'Wake' | 'FallingAsleep' | 'Dreaming' | 'Waking';

export interface TokenFieldPoint {
  position: [number, number, number];
  layer: number;
  temperature: number;
  anchor_membership: number | null;
}

export interface DomainConfigSummary {
  capacity: number;
  temperature_decay: number;
}

export interface DomainSnapshot {
  id: number;
  name: string;
  config_summary: DomainConfigSummary;
  token_count: number;
  connection_count: number;
  temperature_avg: number;
  recent_activity: number;
  layer_activations: number[];
  token_field: TokenFieldPoint[];
}

export interface OverDomainSnapshot {
  total_tokens: number;
  total_connections: number;
  cross_domain_events_recent: number;
  layer_activations: number[];
}

export interface FatigueSnapshot {
  current: number;
  threshold: number;
  ticks_since_dream: number;
  token_rate: number;
  history: number[];
}

export interface DreamReport {
  cycle_id: number;
  started_at_tick: number;
  ended_at_tick: number;
  proposals_accepted: number;
  proposals_rejected: number;
  sutra_written: number;
  fatigue_before: number;
  fatigue_after: number;
}

export interface GuardianStats {
  total_vetoes: number;
  vetoes_since_wake: number;
  last_veto_reason: string | null;
}

export interface DreamPhaseStats {
  cycles_completed: number;
  last_transition_tick: number;
}

export interface EmergentCandidateSnapshot {
  sutra_id: number;
  discovery_octant: number;
  initial_depth: number;
}

export interface PendingAdvisorySnapshot {
  advisory_id: number;
  advisory_type: number;
  subject_id: number;
  confidence: number;
  label: string;
  queued_at_event: number;
}

export interface PhaseCSnapshot {
  dominant_octant: number | null;
  dominant_subsystem: number | null;
  pending_emergent_count: number;
  emergent_candidates: EmergentCandidateSnapshot[];
  advisory_frames: unknown[];
  octant_depth_avg: number[];
  pending_advisories: PendingAdvisorySnapshot[];
}

export interface FrameWeaverStats {
  total_frames: number;
  frames_in_sutra: number;
  promotions_since_wake: number;
  last_crystallization_tick: number;
  syntactic_layer_activations: number[];
}

export interface PerfSnapshot {
  uptime_secs: number;
  actual_hz: number;
  tick_ns_avg: number;
  tick_ns_peak: number;
  total_ticks: number;
}

export interface TraceSnapshot {
  weight: number;
  temperature: number;
  mass: number;
  valence: number;
  position: [number, number, number];
  age_ticks: number;
  success_count: number;
  pattern_hash: number;
}

export interface TensionTraceSnapshot {
  temperature: number;
  age_ticks: number;
}

export interface ReflectorDomainStats {
  role: number;
  domain_id: number;
  name: string;
  success: number;
  total: number;
  success_rate: number;
}

export interface ReflectorSnapshot {
  patterns_tracked: number;
  total_success: number;
  total_fail: number;
  per_domain: ReflectorDomainStats[];
}

export interface CognitiveDepthSnapshot {
  max_passes: number;
  min_coherence: number;
  internal_dominance: number;
}

export interface ImpulsesSnapshot {
  tension_count: number;
  goal_count: number;
  curiosity_count: number;
}

export interface SystemSnapshot {
  engine_state: EngineState;
  current_tick: number;
  current_event: number;
  hot_path_ns: number;
  domains: DomainSnapshot[];
  over_domain: OverDomainSnapshot;
  fatigue: FatigueSnapshot;
  last_dream_report: DreamReport | null;
  frame_weaver_stats: FrameWeaverStats | null;
  guardian_stats: GuardianStats;
  dream_phase_stats: DreamPhaseStats;
  adapter_progress: unknown[];
  phase_c: PhaseCSnapshot | null;
  // Extended metrics (WS-5)
  perf: PerfSnapshot;
  traces_count: number;
  tension_count: number;
  top_traces: TraceSnapshot[];
  tension_traces: TensionTraceSnapshot[];
  reflector: ReflectorSnapshot;
  cognitive_depth: CognitiveDepthSnapshot;
  impulses: ImpulsesSnapshot;
  skills_count: number;
}

// EngineEvent — variants from axiom-protocol/src/events.rs
export type EngineEvent =
  | { Tick: { tick: number; event: number; hot_path_ns: number } }
  | { DomainActivity: { domain_id: number; recent_activity: number; layer_activations: number[] } }
  | { DreamPhaseTransition: { from: EngineState; to: EngineState; trigger: string } }
  | { FrameCrystallized: { anchor_id: number; layers_present: number; participant_count: number } }
  | { FrameReactivated: { anchor_id: number; new_temperature: number } }
  | { FramePromoted: { anchor_id: number } }
  | { Alert: { level: string; message: string } };

// EngineMessage — enum serialized as tagged object by serde
export type EngineMessage =
  | { Hello: { version: number; capabilities: number } }
  | { Snapshot: SystemSnapshot }
  | { Event: EngineEvent }
  | { CommandResult: { command_id: number; result: unknown } }
  | { Bye: { reason: string } };
