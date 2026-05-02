use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdapterInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supported_extensions: Vec<String>,
    pub options_schema: Vec<AdapterOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdapterOption {
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdapterProgress {
    pub adapter_id: String,
    pub source: String,
    pub processed: u64,
    /// 0 if total is unknown.
    pub total: u64,
    pub status: AdapterStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterStatus {
    Running,
    Finishing,
}
