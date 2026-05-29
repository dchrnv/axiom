use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CorpusEntry {
    pub id: String,
    pub content: String,
    #[serde(default)]
    pub expected_subsystem: Option<String>,
    pub inject_every: u64,
    pub inject_count: u32,
    #[serde(default)]
    pub start_at_tick: u64,
}

impl CorpusEntry {
    pub fn injection_ticks(&self) -> Vec<u64> {
        (0..self.inject_count)
            .map(|i| self.start_at_tick + i as u64 * self.inject_every)
            .collect()
    }
}

fn default_snapshot_every() -> u64 {
    100
}

#[derive(Debug, Deserialize)]
pub struct Corpus {
    pub ticks_total: u64,
    #[serde(default = "default_snapshot_every")]
    pub snapshot_every: u64,
    /// Cap live tokens per domain. Coldest non-protected tokens are evicted when exceeded.
    /// Set to keep tick time stable on long runs. Recommended: 1000–3000.
    #[serde(default)]
    pub max_tokens_per_domain: Option<usize>,
    /// Split corpus texts across this many independent engine instances running in parallel.
    /// Each shard gets a round-robin slice of texts. Results are merged on completion.
    /// Default: 1 (single-threaded). Set to num_cpus for maximum throughput.
    #[serde(default = "default_shards")]
    pub shards: usize,
    pub texts: Vec<CorpusEntry>,
}

fn default_shards() -> usize { 1 }

impl Corpus {
    pub fn from_yaml(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = std::fs::read_to_string(path)?;
        let corpus = serde_yaml::from_str(&raw)?;
        Ok(corpus)
    }
}
