use crate::corpus::Corpus;
use crate::metrics::{InjectionEvent, TickSnapshot};

pub fn generate_markdown(
    corpus: &Corpus,
    snapshots: &[TickSnapshot],
    events: &[InjectionEvent],
) -> String {
    let mut out = String::new();

    out.push_str("# axiom-observe: OBS-01 Report\n\n");

    // 1. Run parameters
    out.push_str("## Parameters\n\n");
    out.push_str(&format!(
        "- Ticks: {}\n- Snapshot every: {}\n- Corpus entries: {}\n\n",
        corpus.ticks_total,
        corpus.snapshot_every,
        corpus.texts.len()
    ));

    // 2. Final state
    if let Some(last) = snapshots.last() {
        out.push_str("## Final State\n\n");
        out.push_str("| Metric | Value |\n|---|---|\n");
        out.push_str(&format!("| Frames | {} |\n", last.frame_count));
        out.push_str(&format!("| Total evaluations | {} |\n", last.total_evaluations));
        out.push_str(&format!("| Total conflicts | {} |\n", last.total_conflicts));
        out.push_str(&format!("| Profile count | {} |\n", last.profile_count));
        out.push_str(&format!(
            "| Dominant subsystem | {} |\n",
            last.dominant_subsystem.map_or("none".into(), |s| s.to_string())
        ));
        out.push_str(&format!(
            "| Dominant octant | {} |\n",
            last.dominant_octant.map_or("none".into(), |o| o.to_string())
        ));
        out.push_str(&format!("| Depth store entries | {} |\n", last.depth_store_len));
        out.push_str(&format!("| Emergent pending | {} |\n", last.emergent_pending));
        out.push_str(&format!("| Emergent approved | {} |\n", last.emergent_approved));
        out.push_str(&format!("| Experience traces | {} |\n", last.experience_traces));
        out.push_str(&format!("| Tension traces | {} |\n", last.tension_traces));
        out.push_str(&format!("| Activity fill | {} |\n", last.activity_fill));
        out.push_str(&format!("| Dominant persistence | {:.3} |\n", last.dominant_persistence));
        out.push_str(&format!("| Entropy gradient | {:.3} |\n", last.entropy_gradient));
        out.push_str(&format!("| Oscillation score | {:.3} |\n", last.oscillation_score));
        out.push_str(&format!("| Cascade score | {:.3} |\n", last.cascade_score));
        out.push_str(&format!("| Fatigue entries | {} |\n", last.fatigue_count));
        out.push_str(&format!("| Avg shell similarity | {:.3} |\n", last.avg_shell_similarity));
        out.push_str(&format!(
            "| Meta dominant | {} |\n\n",
            last.meta_dominant.as_deref().unwrap_or("none")
        ));
    }

    // 3. Experience trace growth timeline
    out.push_str("## Experience Trace Growth\n\n");
    out.push_str("| Tick | Exp traces | Tension | Frames | Profiles | ShellSim |\n|---|---|---|---|---|---|\n");
    for snap in snapshots {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.3} |\n",
            snap.tick, snap.experience_traces, snap.tension_traces,
            snap.frame_count, snap.profile_count, snap.avg_shell_similarity
        ));
    }
    out.push('\n');

    // 4. Avg depth per octant (final)
    if let Some(last) = snapshots.last() {
        out.push_str("## Avg Depth per Octant (final)\n\n");
        out.push_str("| Octant | Avg depth |\n|---|---|\n");
        for (i, &d) in last.avg_depths.iter().enumerate() {
            let flag = if d >= 8000 { " ★" } else { "" };
            out.push_str(&format!("| O{} | {}{} |\n", i + 1, d, flag));
        }
        out.push('\n');
        out.push_str("★ = depth ≥ 8000 (potential emergent threshold)\n\n");
    }

    // 5. Injection events with routing diagnostics
    out.push_str("## Injection Events\n\n");
    out.push_str("| Tick | Entry | Expected | Per-text | Detected | Coherence | Reflex | Passes | Exp traces |\n");
    out.push_str("|---|---|---|---|---|---|---|---|---|\n");
    for ev in events {
        let expected = ev.expected_subsystem.as_deref().unwrap_or("—");
        let per_text = ev.per_text_detected.as_deref().unwrap_or("—");
        let detected = ev.detected_subsystem.map_or("—".into(), |s| s.to_string());
        let coherence = ev.coherence.map_or("—".into(), |c| format!("{:.2}", c));
        let reflex = if ev.reflex_hit { "✓" } else { "—" };
        // Mark per-text match
        let per_text_display = match &ev.expected_subsystem {
            Some(exp) if ev.per_text_detected.as_deref() == Some(exp.as_str()) => {
                format!("✓ {}", per_text)
            }
            _ => per_text.to_string(),
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            ev.tick, ev.entry_id, expected, per_text_display, detected,
            coherence, reflex, ev.passes, ev.experience_traces_at_injection
        ));
    }
    out.push('\n');

    // 6. Coherence analysis
    out.push_str("## Coherence Analysis\n\n");
    let coherence_values: Vec<f32> = events.iter().filter_map(|e| e.coherence).collect();
    if !coherence_values.is_empty() {
        let avg = coherence_values.iter().sum::<f32>() / coherence_values.len() as f32;
        let min = coherence_values.iter().cloned().fold(f32::MAX, f32::min);
        let max = coherence_values.iter().cloned().fold(f32::MIN, f32::max);
        let reflex_hits = events.iter().filter(|e| e.reflex_hit).count();
        let multi_pass = events.iter().filter(|e| e.passes > 1).count();

        // Per-text accuracy: compare per_text_detected vs expected_subsystem
        let per_text_total = events.iter().filter(|e| e.per_text_detected.is_some() && e.expected_subsystem.is_some()).count();
        let per_text_correct = events.iter().filter(|e| {
            matches!((&e.per_text_detected, &e.expected_subsystem), (Some(d), Some(exp)) if d == exp)
        }).count();

        out.push_str(&format!("- Average coherence: {:.3}\n", avg));
        out.push_str(&format!("- Min coherence: {:.3}\n", min));
        out.push_str(&format!("- Max coherence: {:.3}\n", max));
        out.push_str(&format!("- Reflex hits: {} / {}\n", reflex_hits, events.len()));
        out.push_str(&format!("- Multi-pass events: {} / {}\n", multi_pass, events.len()));
        if per_text_total > 0 {
            out.push_str(&format!(
                "- Per-text accuracy: {} / {} ({:.1}%)\n\n",
                per_text_correct, per_text_total,
                per_text_correct as f32 / per_text_total as f32 * 100.0
            ));
        } else {
            out.push('\n');
        }

        if avg < 0.3 {
            out.push_str(
                "⚠ Low average coherence — system is still in cold-start mode. \
                 More injections or longer run needed.\n\n",
            );
        } else if avg > 0.7 {
            out.push_str(
                "✓ High coherence — system has built good resonance patterns.\n\n",
            );
        }
    } else {
        out.push_str("No coherence data available.\n\n");
    }

    // 7. Threshold assessment
    out.push_str("## Threshold Assessment\n\n");
    if let Some(last) = snapshots.last() {
        let high_depth_octants: Vec<usize> = last
            .avg_depths
            .iter()
            .enumerate()
            .filter(|(_, &d)| d >= 8000)
            .map(|(i, _)| i + 1)
            .collect();

        if high_depth_octants.is_empty() {
            out.push_str("No octants reached depth ≥ 8000. Consider more injections or longer run.\n\n");
        } else {
            out.push_str(&format!(
                "Octants above depth threshold (≥8000): {}\n\n",
                high_depth_octants.iter().map(|o| format!("O{o}")).collect::<Vec<_>>().join(", ")
            ));
        }

        if last.emergent_approved > 0 {
            out.push_str(&format!(
                "✓ {} emergent primitive(s) approved.\n\n",
                last.emergent_approved
            ));
        } else if last.emergent_pending > 0 {
            out.push_str(&format!(
                "⚠ {} emergent candidate(s) pending — not yet above approval threshold.\n\n",
                last.emergent_pending
            ));
        } else {
            out.push_str("No emergent candidates detected yet.\n\n");
        }

        let conflict_rate = if last.total_evaluations > 0 {
            last.total_conflicts as f64 / last.total_evaluations as f64 * 100.0
        } else {
            0.0
        };
        out.push_str(&format!(
            "Conflict rate: {:.1}% ({} / {} evaluations)\n\n",
            conflict_rate, last.total_conflicts, last.total_evaluations
        ));

    }

    // 8. V6 activity dynamics timeline
    out.push_str("## V6 Activity Dynamics\n\n");
    out.push_str("| Tick | Fill | Persistence | Entropy | Oscillation | Cascade | Fatigue | Meta | Signatures |\n");
    out.push_str("|---|---|---|---|---|---|---|---|---|\n");
    for snap in snapshots {
        let sigs = if snap.activity_signatures.is_empty() {
            "—".to_string()
        } else {
            snap.activity_signatures.join(", ")
        };
        let meta = snap.meta_dominant.as_deref().unwrap_or("—");
        out.push_str(&format!(
            "| {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {} | {} | {} |\n",
            snap.tick,
            snap.activity_fill,
            snap.dominant_persistence,
            snap.entropy_gradient,
            snap.oscillation_score,
            snap.cascade_score,
            snap.fatigue_count,
            meta,
            sigs,
        ));
    }
    out.push('\n');

    // 9. Composite suspects (final snapshot)
    if let Some(last) = snapshots.last() {
        out.push_str("## Composite Co-activation Suspects (final)\n\n");
        if last.composite_suspects.is_empty() {
            out.push_str("None detected.\n\n");
        } else {
            for s in &last.composite_suspects {
                out.push_str(&format!("- {s}\n"));
            }
            out.push('\n');
        }

        // Meta store (final)
        out.push_str("## Meta-subsystem Activations (final)\n\n");
        out.push_str(&format!(
            "Active: {}  |  Dominant: {}\n\n",
            last.meta_active_count,
            last.meta_dominant.as_deref().unwrap_or("none"),
        ));
    }

    out
}
