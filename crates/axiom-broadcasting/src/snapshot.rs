/// Convert axiom-runtime's BroadcastSnapshot → axiom-protocol's SystemSnapshot.
///
/// Only fields available in BroadcastSnapshot are mapped; everything else gets
/// a safe zero/empty default. Full field coverage happens when Engine exposes
/// a richer snapshot API (future errata).
use axiom_protocol::{
    events::EngineState,
    snapshot::{
        DomainConfigSummary, DomainSnapshot, DreamPhaseStats, FatigueSnapshot, FrameWeaverStats,
        GuardianStats, OverDomainSnapshot, SystemSnapshot, TokenFieldPoint,
    },
};
use axiom_runtime::{AxiomEngine, BroadcastSnapshot};

fn build_token_field(engine: &AxiomEngine, domain_id: u16, max: usize) -> Vec<TokenFieldPoint> {
    let Some(detail) = engine.domain_detail_snapshot(domain_id) else {
        return Vec::new();
    };
    let tokens = &detail.tokens;
    if tokens.is_empty() {
        return Vec::new();
    }
    let stride = if tokens.len() > max {
        tokens.len() / max
    } else {
        1
    };
    tokens
        .iter()
        .step_by(stride)
        .take(max)
        .map(|t| {
            let layer = t
                .shell
                .iter()
                .enumerate()
                .max_by_key(|&(_, &v)| v)
                .map(|(i, _)| i as u8)
                .unwrap_or(0);
            let anchor_membership = if t.is_anchor { Some(t.sutra_id) } else { None };
            TokenFieldPoint {
                position: [
                    t.position[0] as f32 / 32767.0,
                    t.position[1] as f32 / 32767.0,
                    t.position[2] as f32 / 32767.0,
                ],
                layer,
                temperature: t.temperature,
                anchor_membership,
            }
        })
        .collect()
}

pub fn engine_state_from(s: &BroadcastSnapshot) -> EngineState {
    use axiom_runtime::over_domain::DreamPhaseState;
    if let Some(dp) = &s.dream_phase {
        match dp.state {
            DreamPhaseState::Wake => EngineState::Wake,
            DreamPhaseState::FallingAsleep => EngineState::FallingAsleep,
            DreamPhaseState::Dreaming => EngineState::Dreaming,
            DreamPhaseState::Waking => EngineState::Waking,
        }
    } else {
        EngineState::Wake
    }
}

pub fn build_system_snapshot(engine: &AxiomEngine, last_tick_ns: u64) -> SystemSnapshot {
    let bs = engine.snapshot_for_broadcast();

    let engine_state = engine_state_from(&bs);

    const TOKEN_FIELD_MAX: usize = 300;

    let domains: Vec<DomainSnapshot> = bs
        .domain_summaries
        .iter()
        .map(|d| {
            let token_field = build_token_field(engine, d.domain_id, TOKEN_FIELD_MAX);
            DomainSnapshot {
                id: d.domain_id,
                name: d.name.clone(),
                config_summary: DomainConfigSummary {
                    capacity: 0,
                    temperature_decay: 0,
                },
                token_count: d.token_count as u32,
                connection_count: d.connection_count as u32,
                temperature_avg: d.temperature_avg,
                recent_activity: 0,
                layer_activations: [0u8; 8],
                token_field,
            }
        })
        .collect();

    let total_tokens: u32 = domains.iter().map(|d| d.token_count).sum();
    let total_connections: u32 = domains.iter().map(|d| d.connection_count).sum();

    let fatigue = if let Some(dp) = &bs.dream_phase {
        FatigueSnapshot {
            current: dp.current_fatigue as f32 / 255.0,
            threshold: 0.8,
            ticks_since_dream: dp.idle_ticks as u64,
            token_rate: 0.0,
            history: vec![],
        }
    } else {
        FatigueSnapshot {
            current: 0.0,
            threshold: 0.8,
            ticks_since_dream: 0,
            token_rate: 0.0,
            history: vec![],
        }
    };

    let frame_weaver_stats = bs.frame_weaver_stats.as_ref().map(|fw| FrameWeaverStats {
        total_frames: fw.crystallizations_approved as u32,
        frames_in_sutra: fw.frames_in_sutra as u32,
        promotions_since_wake: fw.promotions_approved as u32,
        last_crystallization_tick: 0,
        syntactic_layer_activations: fw.syntactic_layer_activations,
    });

    let dream_phase_stats = if let Some(dp) = &bs.dream_phase {
        DreamPhaseStats {
            cycles_completed: dp.stats.total_sleeps,
            last_transition_tick: dp.stats.last_wake_tick,
        }
    } else {
        DreamPhaseStats {
            cycles_completed: 0,
            last_transition_tick: 0,
        }
    };

    SystemSnapshot {
        engine_state,
        current_tick: bs.tick_count,
        current_event: bs.com_next_id,
        hot_path_ns: last_tick_ns,
        domains,
        over_domain: OverDomainSnapshot {
            total_tokens,
            total_connections,
            cross_domain_events_recent: 0,
            layer_activations: [0u8; 8],
        },
        fatigue,
        last_dream_report: None,
        frame_weaver_stats,
        guardian_stats: {
            let gs = engine.guardian.stats();
            GuardianStats {
                total_vetoes: gs.reflex_vetoed + gs.access_denied + gs.protocol_denied,
                vetoes_since_wake: 0,
                last_veto_reason: None,
            }
        },
        dream_phase_stats,
        adapter_progress: vec![],
    }
}
