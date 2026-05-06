use serde::{Deserialize, Serialize};

use crate::bench::BenchSpec;
use crate::config::ConfigValue;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImportOptions {
    pub params: Vec<(String, String)>,
    pub target_domain: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EngineCommand {
    // Sleep control
    ForceSleep,
    ForceWake,

    // Configuration — C2: GetConfig removed, replaced by schema-aware commands
    GetConfigSchema,
    GetConfigSection {
        id: String,
    },
    UpdateConfigField {
        section_id: String,
        field_id: String,
        value: ConfigValue,
    },

    // Knowledge import
    ListAdapters,
    StartImport {
        adapter_id: String,
        source_path: String,
        options: ImportOptions,
    },
    CancelImport {
        import_id: String,
    },

    // Conversation
    SubmitText {
        text: String,
        target_domain: u16,
    },

    // Debug injection
    InjectToken {
        domain_id: u16,
        layer: u8,
        content: String,
    },
    InjectConnection {
        from_domain: u16,
        to_domain: u16,
    },

    // Lifecycle
    GracefulShutdown,
    ForceShutdown,

    // Queries
    RequestFullSnapshot,
    RequestFrameDetails {
        anchor_id: u32,
    },

    // Benchmarks
    RunBench {
        spec: BenchSpec,
    },
}
