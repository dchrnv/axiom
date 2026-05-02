use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BenchSpec {
    pub bench_id: String,
    pub iterations: u32,
    pub options: BenchOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct BenchOptions {
    /// Arbitrary key-value parameters for the benchmark.
    pub params: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BenchResults {
    pub bench_id: String,
    pub iterations: u32,
    pub median_ns: f64,
    pub p50_ns: f64,
    pub p99_ns: f64,
    pub std_dev_ns: f64,
    pub environment: BenchEnvironment,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BenchEnvironment {
    pub os: String,
    pub arch: String,
    pub engine_version: u32,
}
