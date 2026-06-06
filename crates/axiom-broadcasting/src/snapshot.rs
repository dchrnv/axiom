/// Build axiom-protocol's SystemSnapshot directly from AxiomEngine — без BroadcastSnapshot.
/// SEN-TD-01 Фаза F: BroadcastSnapshot удалён как посредник.
use axiom_protocol::{
    events::EngineState,
    snapshot::{
        AdvisoryFrameSnapshot, CognitiveDepthSnapshot, DomainConfigSummary, DomainSnapshot,
        DreamPhaseStats, DreamReport, EmergentCandidateSnapshot, FatigueSnapshot, FrameWeaverStats,
        GuardianStats, ImpulsesSnapshot, OverDomainSnapshot, PendingAdvisorySnapshot, PerfSnapshot,
        PhaseCSnapshot, ReflectorDomainStats, ReflectorSnapshot, SystemSnapshot,
        TensionTraceSnapshot, TokenFieldPoint, TraceSnapshot,
    },
};
use axiom_runtime::over_domain::AdvisoryAction;
use axiom_runtime::{domain_name, AxiomEngine};

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

fn build_phase_c_snapshot(engine: &AxiomEngine) -> Option<PhaseCSnapshot> {
    let dominant_octant = engine.axial_evaluator.storage().store().most_common_octant();
    let dominant_subsystem = engine
        .context_recognizer
        .profile_store()
        .dominant_primary_as_u8();
    let emergent_store = engine.neural_advisor.emergent_store();
    let candidates: Vec<EmergentCandidateSnapshot> = emergent_store
        .get_pending()
        .take(20)
        .map(|p| EmergentCandidateSnapshot {
            sutra_id: p.sutra_id,
            discovery_octant: p.discovery_octant as u8,
            initial_depth: p.initial_depth,
        })
        .collect();
    let pending_emergent_count = emergent_store.get_pending().count() as u32;
    let advisory_frames: Vec<AdvisoryFrameSnapshot> = engine
        .neural_advisor
        .result_store()
        .frames_with_advice()
        .map(|r| AdvisoryFrameSnapshot {
            anchor_id: r.sutra_id,
            has_octant_suggestion: r.octant_suggestion.is_some(),
            has_conflict: r.conflict_diagnosis.is_some(),
            has_subsystem_suggestion: r.subsystem_suggestion.is_some(),
            has_depth_hint: r.depth_hint.is_some(),
        })
        .collect();
    let octant_depth_avg = engine.context_recognizer.depth_store().avg_depths();

    let pending_advisories: Vec<PendingAdvisorySnapshot> = engine
        .over_domain_arbiter
        .pending_snapshot()
        .iter()
        .take(20)
        .map(|p| {
            let type_index = match p.advisory.advisory_type {
                axiom_runtime::over_domain::AdvisoryType::DepthHint => 0u8,
                axiom_runtime::over_domain::AdvisoryType::OctantCorrection => 1,
                axiom_runtime::over_domain::AdvisoryType::ConflictDiagnosis => 2,
                axiom_runtime::over_domain::AdvisoryType::SubsystemAttribution => 3,
                axiom_runtime::over_domain::AdvisoryType::EmergentCandidate => 4,
                axiom_runtime::over_domain::AdvisoryType::NarrativeShift => 5,
            };
            let label = match &p.advisory.action {
                AdvisoryAction::NotifyWorkstation { label } => label.clone(),
                AdvisoryAction::ApplyDepth { octant, depth } => {
                    format!("#{} depth oct{octant}→{depth}", p.advisory.subject_id)
                }
                AdvisoryAction::OverrideOctant { sutra_id, octant_idx } => {
                    format!("#{sutra_id} override oct{octant_idx}")
                }
            };
            PendingAdvisorySnapshot {
                advisory_id: p.advisory.id,
                advisory_type: type_index,
                subject_id: p.advisory.subject_id,
                confidence: p.advisory.confidence,
                label,
                queued_at_event: p.queued_at_event,
            }
        })
        .collect();

    Some(PhaseCSnapshot {
        dominant_octant,
        dominant_subsystem,
        pending_emergent_count,
        emergent_candidates: candidates,
        advisory_frames,
        octant_depth_avg,
        pending_advisories,
    })
}

pub fn build_system_snapshot(engine: &AxiomEngine, last_tick_ns: u64, perf: PerfSnapshot) -> SystemSnapshot {
    use axiom_runtime::over_domain::DreamPhaseState;

    // — Домены —
    const TOKEN_FIELD_MAX: usize = 300;
    let level_id = engine.ashti.level_id();

    let domains: Vec<DomainSnapshot> = (0u16..=10)
        .map(|offset| {
            let domain_id = level_id * 100 + offset;
            let token_field = build_token_field(engine, domain_id, TOKEN_FIELD_MAX);
            let layer_activations = build_layer_activations(engine, domain_id);
            let (token_count, conn_count, temp_avg) = engine
                .ashti
                .index_of(domain_id)
                .and_then(|i| engine.ashti.state(i))
                .map(|s| {
                    let ta = if s.tokens.is_empty() { 0u8 } else {
                        let sum: u64 = s.tokens.iter().map(|t| t.temperature as u64).sum();
                        (sum / s.tokens.len() as u64) as u8
                    };
                    (s.token_count(), s.connection_count(), ta)
                })
                .unwrap_or((0, 0, 0));
            let (capacity, temperature_decay) = engine
                .ashti
                .config_of(domain_id)
                .map(|c| (c.token_capacity, (c.threshold_temp >> 8) as u8))
                .unwrap_or((0, 0));
            let recent_activity = engine
                .ashti
                .index_of(domain_id)
                .and_then(|i| engine.ashti.domain(i))
                .map(|dom| dom.events_since_rebuild as u32)
                .unwrap_or(0);
            DomainSnapshot {
                id: domain_id,
                name: domain_name(domain_id).to_string(),
                config_summary: DomainConfigSummary { capacity, temperature_decay },
                token_count: token_count as u32,
                connection_count: conn_count as u32,
                temperature_avg: temp_avg,
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

    // — Engine state —
    let engine_state = match engine.dream_phase_state {
        DreamPhaseState::Wake        => EngineState::Wake,
        DreamPhaseState::FallingAsleep => EngineState::FallingAsleep,
        DreamPhaseState::Dreaming    => EngineState::Dreaming,
        DreamPhaseState::Waking      => EngineState::Waking,
    };

    // — Fatigue —
    let trace_count   = engine.trace_count() as u32;
    let tension_count_val = engine.tension_count() as u32;
    let token_rate = trace_count as f32 / engine.tick_count.max(1) as f32;
    let current_fatigue = engine.dream_scheduler.current_fatigue();
    let idle_ticks = engine.dream_scheduler.current_idle_ticks();
    let fatigue = FatigueSnapshot {
        current: current_fatigue as f32 / 255.0,
        threshold: 0.8,
        ticks_since_dream: idle_ticks as u64,
        token_rate,
        history: vec![],
    };

    // — FrameWeaver —
    let fw = &engine.frame_weaver.stats;
    let frame_weaver_stats = Some(FrameWeaverStats {
        total_frames: fw.crystallizations_approved as u32,
        frames_in_sutra: fw.frames_in_sutra as u32,
        promotions_since_wake: fw.promotions_approved as u32,
        last_crystallization_tick: engine.last_crystallization_tick,
        syntactic_layer_activations: fw.syntactic_layer_activations,
    });

    // — Dream stats —
    let dream_phase_stats = DreamPhaseStats {
        cycles_completed: engine.dream_phase_stats.total_sleeps,
        last_transition_tick: engine.dream_phase_stats.last_wake_tick,
    };

    // — Last dream report —
    let last_dream_report = engine.last_dream_summary.as_ref().map(|s| DreamReport {
        cycle_id: s.cycle_id,
        started_at_tick: s.started_at_tick,
        ended_at_tick: s.ended_at_tick,
        proposals_accepted: s.proposals_accepted,
        proposals_rejected: s.proposals_rejected,
        sutra_written: s.sutra_written,
        fatigue_before: s.fatigue_before as f32 / 255.0,
        fatigue_after:  s.fatigue_after  as f32 / 255.0,
    });

    // — Traces —
    let exp = engine.ashti.experience();
    let tick = engine.tick_count;
    let mut sorted_traces: Vec<_> = exp.traces().iter().collect();
    sorted_traces.sort_by(|a, b| b.weight.total_cmp(&a.weight));
    let top_traces: Vec<TraceSnapshot> = sorted_traces.iter().take(20).map(|t| {
        TraceSnapshot {
            weight: t.weight,
            temperature: t.pattern.temperature,
            mass: t.pattern.mass,
            valence: t.pattern.valence,
            position: t.pattern.position,
            age_ticks: tick.saturating_sub(t.created_at),
            success_count: t.success_count,
            pattern_hash: (t.pattern_hash & 0xFFFFFFFF) as u32,
        }
    }).collect();
    let tension_traces: Vec<TensionTraceSnapshot> = exp.tension_traces().iter().map(|t| {
        TensionTraceSnapshot {
            temperature: t.temperature,
            age_ticks: tick.saturating_sub(t.created_at),
        }
    }).collect();

    // — Reflector —
    let reflector_data = engine.ashti.reflector();
    let per_domain: Vec<ReflectorDomainStats> = (1u8..=8).filter_map(|role| {
        let profile = reflector_data.domain_profile(role)?;
        let total = profile.total_calls();
        if total == 0 { return None; }
        let domain_id = level_id * 100 + role as u16;
        Some(ReflectorDomainStats {
            role,
            domain_id,
            name: domain_name(domain_id).to_string(),
            success: (profile.overall_success_rate() * total as f32) as u32,
            total,
            success_rate: profile.overall_success_rate(),
        })
    }).collect();
    let reflector = ReflectorSnapshot {
        patterns_tracked: reflector_data.tracked_patterns() as u32,
        total_success: reflector_data.total_success(),
        total_fail: reflector_data.total_fail(),
        per_domain,
    };

    // — Cognitive depth —
    let (max_passes, min_coh) = engine.maya_multipass_params();
    let maya_id = level_id * 100 + 10;
    let internal_dominance = engine.ashti.config_of(maya_id)
        .map(|c| c.internal_dominance_factor as f32 / 128.0)
        .unwrap_or(0.0);
    let cognitive_depth = CognitiveDepthSnapshot {
        max_passes: max_passes as u32,
        min_coherence: min_coh,
        internal_dominance,
    };

    // — Impulses —
    let goal_count = engine.ashti.generate_goal_impulses(
        tick, engine.tick_schedule.goal_check_interval as u64
    ).len() as u32;
    let curiosity_count = exp.find_crystallizable(0.72, 2).len() as u32;
    let impulses = ImpulsesSnapshot {
        tension_count: tension_count_val,
        goal_count,
        curiosity_count,
    };

    let skills_count = engine.ashti.skills_count() as u32;
    let guardian_vetoes_since_wake = engine.guardian.stats().vetoes_since_wake;

    SystemSnapshot {
        engine_state,
        current_tick: engine.tick_count,
        current_event: engine.com_next_id,
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
        guardian_stats: GuardianStats {
            total_vetoes: {
                let gs = engine.guardian.stats();
                gs.reflex_vetoed + gs.access_denied + gs.protocol_denied
            },
            vetoes_since_wake: guardian_vetoes_since_wake as u32,
            last_veto_reason: None,
        },
        dream_phase_stats,
        adapter_progress: vec![],
        phase_c: build_phase_c_snapshot(engine),
        perf,
        traces_count: trace_count,
        tension_count: tension_count_val,
        top_traces,
        tension_traces,
        reflector,
        cognitive_depth,
        impulses,
        skills_count,
    }
}
