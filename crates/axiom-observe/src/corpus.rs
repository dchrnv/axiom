use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    pub texts: Vec<CorpusEntry>,
}

impl Corpus {
    pub fn from_yaml(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = std::fs::read_to_string(path)?;
        let corpus = serde_yaml::from_str(&raw)?;
        Ok(corpus)
    }
}
