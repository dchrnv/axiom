use serde::{Deserialize, Serialize};

use crate::bench::BenchResults;

/// Current lifecycle state of AXIOM Engine (maps to DreamPhaseState internally).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Wake,
    FallingAsleep,
    Dreaming,
    Waking,
}

/// What triggered a dream phase transition.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SleepTrigger {
    FatigueThreshold,
    CriticalSignal,
    SchedulerDecision,
    ManualForceSleep,
    ManualForceWake,
    CycleComplete,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EngineEvent {
    // Metrics
    Tick {
        tick: u64,
        event: u64,
        hot_path_ns: u64,
    },
    DomainActivity {
        domain_id: u16,
        recent_activity: u32,
        layer_activations: [u8; 8],
    },

    // Architecture events
    DreamPhaseTransition {
        from: EngineState,
        to: EngineState,
        trigger: SleepTrigger,
    },
    FrameCrystallized {
        anchor_id: u32,
        layers_present: u8,
        participant_count: u8,
    },
    FrameReactivated {
        anchor_id: u32,
        new_temperature: u8,
    },
    FramePromoted {
        source_anchor_id: u32,
        sutra_anchor_id: u32,
    },
    GuardianVeto {
        reason: String,
        command_summary: String,
    },

    // Knowledge import
    AdapterStarted {
        adapter_id: String,
        source: String,
    },
    AdapterProgress {
        adapter_id: String,
        processed: u64,
        total: u64,
    },
    AdapterFinished {
        adapter_id: String,
        tokens_added: u32,
        errors: u32,
    },

    // Benchmarks
    BenchStarted {
        bench_id: String,
        run_id: u64,
    },
    BenchProgress {
        run_id: u64,
        completed: u32,
        total: u32,
    },
    BenchFinished {
        run_id: u64,
        results: BenchResults,
    },

    Alert {
        level: AlertLevel,
        category: String,
        message: String,
    },
}
