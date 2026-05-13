/// Convert axiom-runtime's BroadcastSnapshot → axiom-protocol's SystemSnapshot.
use axiom_protocol::{
    events::EngineState,
    snapshot::{
        DomainConfigSummary, DomainSnapshot, DreamPhaseStats, DreamReport, FatigueSnapshot,
        FrameWeaverStats, GuardianStats, OverDomainSnapshot, SystemSnapshot, TokenFieldPoint,
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
    let stride = (tokens.len() / max).max(1);
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

fn build_layer_activations(engine: &AxiomEngine, domain_id: u16) -> [u8; 8] {
    let Some(detail) = engine.domain_detail_snapshot(domain_id) else {
        return [0u8; 8];
    };
    let mut buckets = [0u32; 8];
    for token in &detail.tokens {
        if let Some((i, _)) = token.shell.iter().enumerate().max_by_key(|&(_, &v)| v) {
            buckets[i] += 1;
        }
    }
    let max = buckets.iter().copied().max().unwrap_or(1).max(1);
    let mut out = [0u8; 8];
    for i in 0..8 {
        out[i] = ((buckets[i] as u64 * 255) / max as u64) as u8;
    }
    out
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
            let layer_activations = build_layer_activations(engine, d.domain_id);
            let (capacity, temperature_decay) = engine
                .ashti
                .config_of(d.domain_id)
                .map(|c| (c.token_capacity, (c.threshold_temp >> 8) as u8))
                .unwrap_or((0, 0));
            let recent_activity = engine
                .ashti
                .index_of(d.domain_id)
                .and_then(|i| engine.ashti.domain(i))
                .map(|dom| dom.events_since_rebuild as u32)
                .unwrap_or(0);
            DomainSnapshot {
                id: d.domain_id,
                name: d.name.clone(),
                config_summary: DomainConfigSummary {
                    capacity,
                    temperature_decay,
                },
                token_count: d.token_count as u32,
                connection_count: d.connection_count as u32,
                temperature_avg: d.temperature_avg,
                recent_activity,
                layer_activations,
                token_field,
            }
        })
        .collect();

    let total_tokens: u32 = domains.iter().map(|d| d.token_count).sum();
    let total_connections: u32 = domains.iter().map(|d| d.connection_count).sum();

    let mut global_layer_activations = [0u8; 8];
    for d in &domains {
        for i in 0..8 {
            global_layer_activations[i] =
                global_layer_activations[i].saturating_add(d.layer_activations[i]);
        }
    }

    // token_rate: average experiences per tick (proxy для "activity rate")
    let token_rate = bs.trace_count as f32 / bs.tick_count.max(1) as f32;

    let fatigue = if let Some(dp) = &bs.dream_phase {
        FatigueSnapshot {
            current: dp.current_fatigue as f32 / 255.0,
            threshold: 0.8,
            ticks_since_dream: dp.idle_ticks as u64,
            token_rate,
            history: vec![],
        }
    } else {
        FatigueSnapshot {
            current: 0.0,
            threshold: 0.8,
            ticks_since_dream: 0,
            token_rate,
            history: vec![],
        }
    };

    let frame_weaver_stats = bs.frame_weaver_stats.as_ref().map(|fw| FrameWeaverStats {
        total_frames: fw.crystallizations_approved as u32,
        frames_in_sutra: fw.frames_in_sutra as u32,
        promotions_since_wake: fw.promotions_approved as u32,
        last_crystallization_tick: bs.last_crystallization_tick,
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

    let last_dream_report = bs.last_dream_summary.as_ref().map(|s| DreamReport {
        cycle_id: s.cycle_id,
        started_at_tick: s.started_at_tick,
        ended_at_tick: s.ended_at_tick,
        proposals_accepted: s.proposals_accepted,
        proposals_rejected: s.proposals_rejected,
        sutra_written: s.sutra_written,
        fatigue_before: s.fatigue_before as f32 / 255.0,
        fatigue_after: s.fatigue_after as f32 / 255.0,
    });

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
            layer_activations: global_layer_activations,
        },
        fatigue,
        last_dream_report,
        frame_weaver_stats,
        guardian_stats: {
            let gs = engine.guardian.stats();
            GuardianStats {
                total_vetoes: gs.reflex_vetoed + gs.access_denied + gs.protocol_denied,
                vetoes_since_wake: gs.vetoes_since_wake as u32,
                last_veto_reason: None,
            }
        },
        dream_phase_stats,
        adapter_progress: vec![],
    }
}
