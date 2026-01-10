// Axiom - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

pub mod connection_v3;
/// Axiom - Rust Core Implementation
///
/// Cognitive architecture with emergent knowledge structures
///
/// # Architecture
///
/// - Token V2.0: 64-byte atomic unit of information
/// - Connection V3.0: 64-byte learning-capable link with Guardian integration
/// - 8-dimensional semantic space (L1-L8)
/// - ADNA v3.0: 256-byte Policy Engine
/// - ExperienceStream v2.1: Event-based memory system (128-byte events)
/// - Archive: Long-term compressed storage (ExperienceToken 128-byte)
/// - HybridLearning v2.2: ADNA ↔ Connection feedback loops
/// - Binary-compatible format for cross-language interop
/// - Python FFI bindings via PyO3 (optional)
/// - Zero core dependencies (pure Rust)
pub mod token;

// Re-export Connection v3.0 types (primary API)
pub use connection_v3::{
    ConnectionField, ConnectionMutability, ConnectionProposal, ConnectionType, ConnectionV3,
};

// Alias for backward compatibility
pub type Connection = ConnectionV3;
pub mod action_controller;
pub mod action_executor;
pub mod action_types;
pub mod adapters; // NEW: v1.0 Output/Input Adapters (v0.36.0)
pub mod adna;
pub mod api; // NEW: v1.0 REST API (v0.39.0)
pub mod appraisers;
pub mod archive;
pub mod async_wal; // NEW: v1.0 Async WAL Writer (v0.44.2)
pub mod black_box; // NEW: v1.0 Black Box Recorder (v0.42.0)
pub mod bootstrap; // NEW: v1.2 Bootstrap Library (v0.33.0)
pub mod cdna;
pub mod coordinates;
pub mod curiosity; // NEW: v1.0 Curiosity Drive (v0.38.0)
pub mod evolution_manager;
pub mod executors;
pub mod experience_stream;
pub mod feedback; // NEW: v1.0 Feedback System (v0.37.0)
pub mod gateway; // NEW: v1.0 Gateway (v0.35.0)
pub mod graph;
pub mod grid;
pub mod guardian;
pub mod hybrid_learning; // NEW: v2.2 Hybrid Learning Integration (v0.30.2)
pub mod intuition_engine;
pub mod logging_utils; // NEW: v1.0 Logging Utilities (v0.42.0)
pub mod metrics; // NEW: v1.0 Prometheus Metrics (v0.42.0)
pub mod module_id; // NEW: v1.0 Module ID Enum (v0.63.0)
pub mod module_registry;
pub mod panic_handler; // NEW: v1.0 Panic Recovery (v0.41.0)
pub mod persistence;
pub mod policy;
pub mod reflex_layer; // NEW: v3.0 Reflex System (v0.31.0)
pub mod runtime_storage; // NEW: v1.0 Runtime Storage (v0.50.0)
pub mod signal_system; // NEW: v1.1 Signal System - Event Processing (v0.53.0)
pub mod tracing_otel; // NEW: v1.0 OpenTelemetry Distributed Tracing (v0.44.0)
pub mod tracing_sampling; // NEW: v1.0 Adaptive Tracing Sampling (v0.44.3)
pub mod wal; // NEW: v1.0 Write-Ahead Log (v0.41.0) // NEW: v1.0 Module Registry (v0.63.0)

// Python bindings v1.0 (v0.40.0) - PyO3 FFI
#[cfg(feature = "python-bindings")]
pub mod python;

// Old FFI (deprecated, will be removed in favor of python module)
// #[cfg(feature = "python")]
// pub mod ffi;

pub use token::{flags as token_flags, CoordinateSpace, EntityType, Token, SCALE_FACTORS};

pub use grid::{Grid, GridConfig};

pub use graph::{
    AccumulationMode,
    ActivatedNode,
    ActivationResult,
    Direction,
    EdgeId,
    EdgeInfo,
    Graph,
    GraphConfig,
    // SignalSystem v1.0
    NodeActivation,
    NodeId,
    Path,
    SignalConfig,
    Subgraph,
};

pub use cdna::{
    CDNAFlags, ProfileId, ProfileState, CDNA, CDNA_MAGIC, CDNA_VERSION_MAJOR, CDNA_VERSION_MINOR,
};

pub use guardian::{Event, EventType, Guardian, GuardianConfig, Subscription, ValidationError};

pub use adna::{
    ADNAError,
    ADNAHeader,
    ADNAReader,
    ActionPolicy,
    AppraiserConfig,
    CuriosityParams,
    EfficiencyParams,
    EvolutionMetrics,
    GoalDirectedParams,
    // Appraiser configuration
    HomeostasisParams,
    InMemoryADNAReader,
    Intent,
    PolicyPointer,
    PolicyType,
    // Learning loop structures
    Proposal,
    StateMapping,
    ADNA,
    ADNA_MAGIC,
    ADNA_VERSION_MAJOR,
    ADNA_VERSION_MINOR,
};

pub use coordinates::{CoordinateExt, CoordinateIndex};

pub use appraisers::{
    AppraiserSet, CuriosityAppraiser, EfficiencyAppraiser, GoalDirectedAppraiser,
    HomeostasisAppraiser,
};

pub use experience_stream::{
    // Metadata for action events
    ActionMetadata,
    AppraiserType,
    EventFlags,
    EventType as ExperienceEventType,
    ExperienceBatch,
    ExperienceEvent,
    ExperienceReader,
    ExperienceStream,
    ExperienceWriter,
    HotBuffer,
    // Sampling for IntuitionEngine
    SamplingStrategy,
};

pub use archive::{ExperienceToken, InfoFlags, EXPERIENCE_TOKEN_MAGIC};

pub use policy::{Gradient, GradientSource, LinearPolicy, Policy, PolicyError};

pub use intuition_engine::{
    IdentifiedPattern, IntuitionConfig, IntuitionEngine, IntuitionEngineBuilder,
};

pub use hybrid_learning::{
    adna_to_connection_feedback, connection_to_adna_hint, HybridLearningError, HybridLearningStats,
    HybridProposal, ProposalOutcome, ProposalRouter,
};

pub use reflex_layer::{
    compute_grid_hash, token_similarity, AdaptiveTuner, AdaptiveTuningConfig, AssociativeMemory,
    AssociativeStats, FastPathConfig, FastPathResult, IntuitionStats, ShiftConfig,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 47;
pub const VERSION_PATCH: u8 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.47.0");
    }
}

pub use evolution_manager::{ADNAState, EvolutionConfig, EvolutionManager, ValidationResult};

pub use action_executor::{ActionError, ActionExecutor, ActionResult};

pub use action_types::{ActionIntent, ActionType, DecisionSource};

pub use action_controller::{
    ActionController, ActionControllerConfig, ArbiterConfig, ArbiterStats,
};

pub use executors::{MessageSenderExecutor, NoOpExecutor};

// Tracing sampling exports (v0.44.3+)
pub use tracing_sampling::{
    DynamicRateConfig, // v0.44.4: dynamic rate adjustment
    LoadMonitor,       // v0.44.4: load monitoring
    SamplingContext,
    SamplingDecision,
    SamplingPriority, // v0.44.4: head-based sampling
    SamplingStats,
    TraceSampler,
    TraceSamplingConfig,
};

// Persistence exports (only available with 'persistence' feature)
pub use persistence::{PersistenceBackend, PersistenceError, QueryOptions};

#[cfg(feature = "persistence")]
pub use persistence::PostgresBackend;

// Bootstrap Library v1.2
pub use bootstrap::{BootstrapConfig, BootstrapError, BootstrapLibrary, PCAModel, SemanticConcept};

// Gateway v1.0
pub use gateway::{Gateway, GatewayError};

pub use gateway::config::{GatewayConfig, UnknownWordStrategy};

pub use gateway::signals::{
    FeedbackType, InputSignal, ProcessedMetadata, ProcessedSignal, SignalSource, SignalType,
    SystemCommand, TokenOperation,
};

pub use gateway::channels::{ResultReceiver, SignalReceipt};

pub use gateway::stats::GatewayStats;

// Adapters v1.0
pub use adapters::{
    FormattedOutput, OutputAdapter, OutputContext, OutputError,
    SignalSource as AdapterSignalSource, SignalType as AdapterSignalType,
};

pub use adapters::console::{ConsoleConfig, ConsoleInputAdapter, ConsoleOutputAdapter};

// Feedback v1.0
pub use feedback::{
    DetailedFeedbackType, FeedbackError, FeedbackProcessor, FeedbackResult, FeedbackSignal,
};

// Curiosity v1.0
pub use curiosity::{
    CuriosityConfig, CuriosityContext, CuriosityDrive, CuriosityScore, CuriosityStats,
    ExplorationMode, ExplorationPriority, ExplorationReason, ExplorationTarget,
};

// Panic Recovery v1.0
pub use panic_handler::{
    catch_panic, catch_panic_async, install_panic_hook, PanicError, PanicResult,
};

// WAL (Write-Ahead Log) v1.0
pub use wal::{WalEntry, WalEntryHeader, WalEntryType, WalError, WalReader, WalStats, WalWriter};

// Runtime Storage v1.0 (v0.50.0)
pub use runtime_storage::{RuntimeStorage, StorageError, StorageResult};
