// Локальные копии ServerMessage / BroadcastSnapshot без axiom-* зависимостей.
// Должны совпадать с форматом axiom-agent protocol.rs и axiom-runtime broadcast.rs.

use serde::Deserialize;

// ── broadcast types ───────────────────────────────────────────────────────────

#[derive(Deserialize, Clone, Default)]
pub struct BroadcastSnapshot {
    pub tick_count:       u64,
    pub com_next_id:      u64,
    pub trace_count:      u32,
    pub tension_count:    u32,
    pub domain_summaries: Vec<DomainSummary>,
}

#[derive(Deserialize, Clone)]
pub struct DomainSummary {
    pub domain_id:        u16,
    pub name:             String,
    pub token_count:      usize,
    pub connection_count: usize,
}

#[derive(Deserialize, Clone)]
pub struct DomainDetailSnapshot {
    pub domain_id:   u16,
    pub tokens:      Vec<TokenSnapshot>,
    pub connections: Vec<ConnectionSnapshot>,
}

#[derive(Deserialize, Clone)]
pub struct TokenSnapshot {
    pub sutra_id:    u64,
    pub position:    [i16; 3],
    pub shell:       [u8; 8],
    pub mass:        u8,
    pub temperature: u8,
    pub valence:     i8,
    pub origin:      u8,
    pub is_anchor:   bool,
}

#[derive(Deserialize, Clone)]
pub struct ConnectionSnapshot {
    pub source_id: u64,
    pub target_id: u64,
    pub weight:    f32,
}

// ── server messages ───────────────────────────────────────────────────────────

#[derive(Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Result {
        command_id:     String,
        path:           String,
        domain_id:      u16,
        domain_name:    String,
        coherence:      f32,
        reflex_hit:     bool,
        traces_matched: u32,
        position:       [i16; 3],
        shell:          [u8; 8],
        event_id:       u64,
    },
    Tick {
        tick_count:   u64,
        traces:       u32,
        tension:      u32,
        last_matched: u32,
    },
    State {
        tick_count: u64,
        snapshot:   BroadcastSnapshot,
    },
    CommandResult {
        command_id: String,
        output:     String,
    },
    DomainDetail(DomainDetailSnapshot),
    Error {
        command_id: Option<String>,
        message:    String,
    },
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_tick() {
        let json = r#"{"type":"tick","tick_count":42,"traces":5,"tension":2,"last_matched":3}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        if let ServerMessage::Tick { tick_count, traces, tension, last_matched } = msg {
            assert_eq!(tick_count, 42);
            assert_eq!(traces, 5);
            assert_eq!(tension, 2);
            assert_eq!(last_matched, 3);
        } else {
            panic!("expected Tick");
        }
    }

    #[test]
    fn test_deserialize_state() {
        let json = r#"{
            "type":"state",
            "tick_count":10,
            "snapshot":{
                "tick_count":10,"com_next_id":1,"trace_count":3,"tension_count":1,
                "domain_summaries":[
                    {"domain_id":100,"name":"SUTRA","token_count":5,"connection_count":2}
                ]
            }
        }"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        if let ServerMessage::State { tick_count, snapshot } = msg {
            assert_eq!(tick_count, 10);
            assert_eq!(snapshot.domain_summaries.len(), 1);
            assert_eq!(snapshot.domain_summaries[0].name, "SUTRA");
        } else {
            panic!("expected State");
        }
    }

    #[test]
    fn test_deserialize_command_result() {
        let json = r#"{"type":"command_result","command_id":"1","output":"tick_count: 5\n"}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        assert!(matches!(msg, ServerMessage::CommandResult { .. }));
    }

    #[test]
    fn test_deserialize_error() {
        let json = r#"{"type":"error","command_id":null,"message":"not found"}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        assert!(matches!(msg, ServerMessage::Error { .. }));
    }
}
