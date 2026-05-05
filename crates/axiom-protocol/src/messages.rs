use serde::{Deserialize, Serialize};

use crate::adapters::AdapterInfo;
use crate::commands::EngineCommand;
use crate::config::{ConfigSchema, ConfigSection};
use crate::events::EngineEvent;
use crate::snapshot::{FrameDetails, SystemSnapshot};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EngineMessage {
    Hello {
        version: u32,
        capabilities: u64,
    },
    Snapshot(SystemSnapshot),
    Event(EngineEvent),
    CommandResult {
        command_id: u64,
        result: Result<CommandResultData, String>,
    },
    Bye {
        reason: ShutdownReason,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Hello {
        version: u32,
        client_kind: ClientKind,
    },
    RequestSnapshot,
    Subscribe {
        event_categories: u64,
    },
    Command {
        command_id: u64,
        command: EngineCommand,
    },
    Bye,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientKind {
    Workstation,
    /// Reserved for future Companion project.
    Companion,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownReason {
    Normal,
    EngineCrashed,
    ClientRequested,
    VersionMismatch,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CommandResultData {
    None,
    AdapterList(Vec<AdapterInfo>),
    FrameDetails(FrameDetails),
    // C2: schema-driven config
    ConfigSchema(ConfigSchema),
    ConfigSection(ConfigSection),
    ConfigUpdateApplied { hot_reloaded: bool },
    ConfigValidationError { field_id: String, message: String },
}
