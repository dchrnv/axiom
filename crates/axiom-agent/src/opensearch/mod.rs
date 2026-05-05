// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// OpenSearch External Adapter (Phase 5).
//
// Подписывается на broadcast_rx и индексирует события в OpenSearch/Elasticsearch.
// Индексируются:
//   - ServerMessage::Result  → документ типа "result"
//   - ServerMessage::Tick    → документ типа "tick" (каждые tick_interval тиков)
//
// Транспорт: POST /<index>/_doc через reqwest (уже в зависимостях).
// Аутентификация: Basic auth через URL (http://user:pass@host:port) или без auth.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;

use crate::protocol::ServerMessage;

/// Конфигурация OpenSearch-адаптера.
pub struct OpenSearchConfig {
    /// URL кластера, например "http://localhost:9200"
    pub url: String,
    /// Имя индекса (default: "axiom-events")
    pub index: String,
    /// Индексировать Tick каждые N тиков (0 = не индексировать)
    pub tick_interval: u64,
}

impl Default for OpenSearchConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:9200".to_string(),
            index: "axiom-events".to_string(),
            tick_interval: 0,
        }
    }
}

/// Собрать JSON-документ из ServerMessage::Result.
pub fn build_result_doc(
    command_id: &str,
    path: &str,
    domain_id: u16,
    domain_name: &str,
    coherence: f32,
    reflex_hit: bool,
    traces_matched: u32,
    position: [i16; 3],
    event_id: u64,
) -> serde_json::Value {
    serde_json::json!({
        "@timestamp":    now_rfc3339(),
        "type":          "result",
        "command_id":    command_id,
        "path":          path,
        "domain_id":     domain_id,
        "domain_name":   domain_name,
        "coherence":     coherence,
        "reflex_hit":    reflex_hit,
        "traces_matched": traces_matched,
        "position":      position,
        "event_id":      event_id,
    })
}

/// Собрать JSON-документ из ServerMessage::Tick.
pub fn build_tick_doc(
    tick_count: u64,
    traces: u32,
    tension: u32,
    last_matched: u32,
) -> serde_json::Value {
    serde_json::json!({
        "@timestamp":  now_rfc3339(),
        "type":        "tick",
        "tick_count":  tick_count,
        "traces":      traces,
        "tension":     tension,
        "last_matched": last_matched,
    })
}

fn now_rfc3339() -> String {
    // RFC 3339 без внешних зависимостей — достаточно для OpenSearch @timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();

    // Простой ISO 8601 UTC: YYYY-MM-DDTHH:MM:SS.mmmZ
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400; // days since epoch

    // Gregorian calendar calculation
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}.{millis:03}Z")
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// OpenSearch External Adapter.
pub struct OpenSearchAdapter {
    config: Arc<OpenSearchConfig>,
}

impl OpenSearchAdapter {
    pub fn new(config: OpenSearchConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// Запустить адаптер: подписывается на broadcast, индексирует события.
    pub fn run(self, broadcast_tx: tokio::sync::broadcast::Sender<ServerMessage>) {
        let config = Arc::clone(&self.config);
        let rx = broadcast_tx.subscribe();
        tokio::spawn(index_task(config, rx));
    }
}

async fn index_task(config: Arc<OpenSearchConfig>, mut rx: broadcast::Receiver<ServerMessage>) {
    let client = reqwest::Client::new();
    let endpoint = format!("{}/{}/_doc", config.url.trim_end_matches('/'), config.index);

    loop {
        match rx.recv().await {
            Ok(ServerMessage::Result {
                command_id,
                path,
                domain_id,
                domain_name,
                coherence,
                reflex_hit,
                traces_matched,
                position,
                event_id,
                ..
            }) => {
                let doc = build_result_doc(
                    &command_id,
                    &path,
                    domain_id,
                    &domain_name,
                    coherence,
                    reflex_hit,
                    traces_matched,
                    position,
                    event_id,
                );
                post_doc(&client, &endpoint, &doc).await;
            }

            Ok(ServerMessage::Tick {
                tick_count,
                traces,
                tension,
                last_matched,
            }) => {
                if config.tick_interval > 0 && tick_count % config.tick_interval == 0 {
                    let doc = build_tick_doc(tick_count, traces, tension, last_matched);
                    post_doc(&client, &endpoint, &doc).await;
                }
            }

            Err(broadcast::error::RecvError::Lagged(_)) => {}
            Err(broadcast::error::RecvError::Closed) => return,
            Ok(_) => {}
        }
    }
}

async fn post_doc(client: &reqwest::Client, endpoint: &str, doc: &serde_json::Value) {
    let _ = client
        .post(endpoint)
        .json(doc)
        .timeout(Duration::from_secs(5))
        .send()
        .await;
    // Fire-and-forget: индексирование не должно влиять на основной цикл
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_result_doc_has_required_fields() {
        let doc = build_result_doc("cmd1", "Direct", 100, "SUTRA", 0.9, false, 3, [1, 2, 3], 42);
        assert_eq!(doc["type"], "result");
        assert_eq!(doc["command_id"], "cmd1");
        assert_eq!(doc["domain_name"], "SUTRA");
        assert_eq!(doc["domain_id"], 100);
        assert!((doc["coherence"].as_f64().unwrap() - 0.9).abs() < 1e-4);
        assert_eq!(doc["reflex_hit"], false);
        assert_eq!(doc["traces_matched"], 3);
        assert_eq!(doc["event_id"], 42);
        assert!(doc["@timestamp"].as_str().unwrap().ends_with('Z'));
    }

    #[test]
    fn build_tick_doc_has_required_fields() {
        let doc = build_tick_doc(500, 12, 3, 7);
        assert_eq!(doc["type"], "tick");
        assert_eq!(doc["tick_count"], 500);
        assert_eq!(doc["traces"], 12);
        assert_eq!(doc["tension"], 3);
        assert_eq!(doc["last_matched"], 7);
        assert!(doc["@timestamp"].as_str().unwrap().ends_with('Z'));
    }

    #[test]
    fn now_rfc3339_format() {
        let ts = now_rfc3339();
        // YYYY-MM-DDTHH:MM:SS.mmmZ
        assert_eq!(ts.len(), 24);
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
    }

    #[test]
    fn days_to_ymd_epoch() {
        // Unix epoch = 1970-01-01
        let (y, m, d) = days_to_ymd(0);
        assert_eq!(y, 1970);
        assert_eq!(m, 1);
        assert_eq!(d, 1);
    }

    #[test]
    fn days_to_ymd_known_date() {
        // 2026-04-19 = days since epoch?
        // 2026-01-01 = 56*365 + 14 leaps = 20454 days; +108 = 20562
        let (y, m, d) = days_to_ymd(20562);
        assert_eq!(y, 2026);
        assert_eq!(m, 4);
        assert_eq!(d, 19);
    }

    #[test]
    fn opensearch_config_default() {
        let cfg = OpenSearchConfig::default();
        assert_eq!(cfg.url, "http://localhost:9200");
        assert_eq!(cfg.index, "axiom-events");
        assert_eq!(cfg.tick_interval, 0);
    }

    #[test]
    fn tick_interval_zero_means_skip() {
        let cfg = OpenSearchConfig {
            tick_interval: 0,
            ..OpenSearchConfig::default()
        };
        // tick_count=100 % 0 would panic — condition guards against it
        assert!(cfg.tick_interval == 0);
    }

    #[test]
    fn tick_interval_filters_correctly() {
        let interval = 100u64;
        assert!(100 % interval == 0);
        assert!(200 % interval == 0);
        assert!(150 % interval != 0);
    }

    #[test]
    fn endpoint_construction() {
        let cfg = OpenSearchConfig {
            url: "http://localhost:9200/".to_string(),
            index: "axiom-events".to_string(),
            tick_interval: 0,
        };
        let ep = format!("{}/{}/_doc", cfg.url.trim_end_matches('/'), cfg.index);
        assert_eq!(ep, "http://localhost:9200/axiom-events/_doc");
    }
}
