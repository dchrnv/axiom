use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use axiom_agent::perceptors::text::TextPerceptor;
use axiom_config::AnchorSet;
use axiom_genome::Genome;
use axiom_runtime::over_domain::context_recognizer::MetaDetector;
use axiom_runtime::AxiomEngine;
use axiom_ucl::{OpCode, UclCommand};

use crate::corpus::{Corpus, CorpusEntry};
use crate::metrics::{InjectionEvent, TickSnapshot};

pub struct ObsRunner {
    engine: AxiomEngine,
    perceptor: TextPerceptor,
}

impl ObsRunner {
    pub fn new(anchors_dir: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let genome = Arc::new(Genome::default_ashti_core());
        let mut engine = AxiomEngine::try_new(genome)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("{e}").into() })?;

        let anchor_set = match anchors_dir {
            Some(dir) => match AnchorSet::load_dir(dir) {
                Ok(set) => {
                    eprintln!("[observe] loaded anchors from {}", dir.display());
                    Some(Arc::new(set))
                }
                Err(e) => {
                    eprintln!("[observe] anchor load failed: {e} — using FNV fallback");
                    None
                }
            },
            None => None,
        };

        if let Some(ref set) = anchor_set {
            engine.apply_anchor_set(set);
            let n = engine.inject_anchor_tokens(set);
            eprintln!("[observe] injected {n} anchor tokens");
        }

        // Load MetaDetector
        let meta_path = std::path::Path::new("config/meta_primitives.yaml");
        match MetaDetector::from_yaml(meta_path) {
            Ok(det) => {
                eprintln!("[observe] loaded {} meta primitives", det.len());
                engine.apply_meta_detector(det);
            }
            Err(e) => eprintln!("[observe] meta_primitives not loaded: {e}"),
        }

        let perceptor = match anchor_set {
            Some(set) => TextPerceptor::with_anchors(set),
            None => TextPerceptor::new(),
        };

        Ok(Self { engine, perceptor })
    }

    pub fn run(
        &mut self,
        corpus: &Corpus,
    ) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
        // Build injection schedule: tick -> list of entry indices
        let mut schedule: HashMap<u64, Vec<usize>> = HashMap::new();
        for (i, entry) in corpus.texts.iter().enumerate() {
            for tick in entry.injection_ticks() {
                if tick < corpus.ticks_total {
                    schedule.entry(tick).or_default().push(i);
                }
            }
        }

        let mut snapshots: Vec<TickSnapshot> = Vec::new();
        let mut events: Vec<InjectionEvent> = Vec::new();

        let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);

        for tick in 0..corpus.ticks_total {
            // Inject texts scheduled for this tick
            if let Some(indices) = schedule.get(&tick) {
                for &idx in indices {
                    let entry: &CorpusEntry = &corpus.texts[idx];
                    let cmd = self.perceptor.perceive(&entry.content);

                    let per_text_detected = self.perceptor.detect_subsystem(&entry.content);
                    let result = self.engine.process_and_observe(&cmd);

                    let detected = self
                        .engine
                        .context_recognizer
                        .profile_store()
                        .dominant_primary_as_u8();

                    let exp_traces = self.engine.ashti.experience().trace_count();

                    events.push(InjectionEvent {
                        tick,
                        entry_id: entry.id.clone(),
                        expected_subsystem: entry.expected_subsystem.clone(),
                        detected_subsystem: detected,
                        coherence: result.coherence_score,
                        reflex_hit: result.reflex_hit,
                        passes: result.passes,
                        experience_traces_at_injection: exp_traces,
                        per_text_detected,
                    });
                }
            }

            self.engine.process_command(&tick_cmd);

            if tick % corpus.snapshot_every == 0 {
                snapshots.push(self.capture_snapshot(tick));
            }
        }

        // Always capture final state
        if corpus.ticks_total % corpus.snapshot_every != 0 {
            snapshots.push(self.capture_snapshot(corpus.ticks_total));
        }

        (snapshots, events)
    }

    fn capture_snapshot(&self, tick: u64) -> TickSnapshot {
        let storage = self.engine.axial_evaluator.storage();
        let cr = &self.engine.context_recognizer;
        let profile_store = cr.profile_store();
        let depth_store = cr.depth_store();
        let emergent_store = self.engine.neural_advisor.emergent_store();

        let experience_traces = self.engine.ashti.experience().trace_count();
        let tension_traces = self.engine.ashti.experience().tension_count();

        let dynamics = cr.activity_dynamics();
        let signatures = cr.activity_signatures();
        let meta_store = cr.meta_store();

        TickSnapshot {
            tick,
            frame_count: storage.store().frame_count(),
            total_evaluations: storage.store().total_evaluations(),
            total_conflicts: storage.total_conflicts as usize,
            profile_count: profile_store.len(),
            dominant_subsystem: profile_store.dominant_primary_as_u8(),
            dominant_octant: storage.store().most_common_octant(),
            depth_store_len: depth_store.len(),
            avg_depths: depth_store.avg_depths(),
            emergent_pending: emergent_store.get_pending().count(),
            emergent_approved: emergent_store.get_approved().count(),
            experience_traces,
            tension_traces,
            // V6 fields
            activity_fill: dynamics.fill_count,
            dominant_persistence: dynamics.dominant_persistence,
            entropy_gradient: dynamics.entropy_gradient,
            oscillation_score: dynamics.oscillation_score,
            cascade_score: dynamics.cascade_score,
            activity_signatures: signatures.iter().map(|s| s.name().to_string()).collect(),
            meta_active_count: meta_store.len(),
            meta_dominant: meta_store.dominant().map(|id| id.name().to_string()),
            composite_suspects: cr
                .composite_suspects()
                .iter()
                .map(|c| format!("{}({:.2})", c.name, c.confidence))
                .collect(),
            fatigue_count: cr.fatigue_store().len(),
            avg_shell_similarity: self.engine.frame_weaver.avg_candidate_shell_similarity(),
        }
    }
}
