use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use axiom_agent::perceptors::text::TextPerceptor;
use axiom_config::AnchorSet;
use axiom_genome::Genome;
use axiom_runtime::over_domain::context_recognizer::MetaDetector;
use axiom_runtime::AxiomEngine;
use axiom_ucl::{OpCode, UclCommand};

use crate::corpus::{Corpus, CorpusEntry};
use crate::metrics::{InjectionEvent, TickSnapshot};
use crate::training::TrainingExample;

/// Каждые N тиков собираем тренировочный пример (не на каждом — дорого).
const TRAINING_SAMPLE_EVERY: u64 = 200;

pub struct ObsRunner {
    engine: AxiomEngine,
    perceptor: TextPerceptor,
    /// Rolling average для avg_shell_similarity (OBS-TD-02).
    /// EMA с α=0.3: обновляется только когда текущее значение > 0.
    /// Кандидаты кристаллизуются быстро (~60 тиков) — между снапшотами (500 тиков)
    /// активных кандидатов нет, поэтому сырое значение всегда 0.
    shell_similarity_ema: f32,
}

impl ObsRunner {
    /// Создать ObsRunner с загрузкой якорей из config_dir (например "config/").
    ///
    /// Использует AnchorSet::load_or_empty — загружает subsystem-якоря И crystal C0
    /// из seeds/crystal_c0.yaml (рядом с config_dir). Crystal доступен TextPerceptor Path 3.
    pub fn new(config_dir: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let genome = Arc::new(Genome::default_ashti_core());
        let mut engine = AxiomEngine::try_new(genome)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("{e}").into() })?;

        let anchor_set = match config_dir {
            Some(dir) => {
                let set = AnchorSet::load_or_empty(dir);
                eprintln!(
                    "[observe] loaded anchors from {} (crystal: {} graphemes)",
                    dir.display(),
                    set.crystal.len()
                );
                Some(Arc::new(set))
            }
            None => None,
        };

        if let Some(ref set) = anchor_set {
            engine.apply_anchor_set(set);
            let n = engine.inject_anchor_tokens(set);
            eprintln!("[observe] injected {n} anchor tokens");
        }

        // SubsystemDependencies из config_dir.
        if let Some(dir) = config_dir {
            let deps = axiom_config::SubsystemDependencies::load_or_empty(dir);
            engine.context_recognizer.set_subsystem_dependencies(deps);
            eprintln!("[observe] subsystem_dependencies loaded from {}", dir.display());
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

        Ok(Self { engine, perceptor, shell_similarity_ema: 0.0 })
    }

    pub fn run(&mut self, corpus: &Corpus) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
        self.run_inner(corpus, None, None)
    }

    /// Экспортировать Experience traces в бинарный файл (для импорта в живой движок).
    ///
    /// Записывает `out_dir/traces.bin`. Только трейсы с weight ≥ 0.1.
    /// Возвращает количество экспортированных трейсов.
    pub fn export_traces(&self, out_dir: &Path) -> Result<u32, String> {
        let path = out_dir.join("traces.bin");
        axiom_persist::export_traces(&self.engine, &path, 0.1)
            .map(|r| r.exported)
            .map_err(|e| e.to_string())
    }

    /// Run with JSONL streaming — write snapshots/events to files in `out_dir` as they arrive.
    /// Returns empty Vecs (data is in the files, not in RAM).
    pub fn run_streaming(
        &mut self,
        corpus: &Corpus,
        out_dir: &Path,
    ) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
        self.run_inner(corpus, None, Some(out_dir))
    }

    /// Run as shard `shard_id` of a parallel split — prefixes progress output.
    pub fn run_shard(
        &mut self,
        shard_id: usize,
        corpus: &Corpus,
    ) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
        self.run_inner(corpus, Some(shard_id), None)
    }

    fn run_inner(
        &mut self,
        corpus: &Corpus,
        shard_id: Option<usize>,
        stream_to: Option<&Path>,
    ) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
        let prefix = match shard_id {
            Some(id) => format!("[observe/shard{id}]"),
            None => "[observe]".to_string(),
        };

        // Build injection schedule: tick -> list of entry indices
        let mut schedule: HashMap<u64, Vec<usize>> = HashMap::new();
        for (i, entry) in corpus.texts.iter().enumerate() {
            for tick in entry.injection_ticks() {
                if tick < corpus.ticks_total {
                    schedule.entry(tick).or_default().push(i);
                }
            }
        }

        // Streaming writers — если задан out_dir, пишем сразу в файлы вместо Vec
        let shard_prefix = shard_id.map(|id| format!("shard{id}_")).unwrap_or_default();
        let mut snap_writer: Option<BufWriter<std::fs::File>> = stream_to.and_then(|dir| {
            let path = dir.join(format!("{shard_prefix}snapshots.jsonl"));
            std::fs::File::create(&path).ok().map(BufWriter::new)
        });
        let mut event_writer: Option<BufWriter<std::fs::File>> = stream_to.and_then(|dir| {
            let path = dir.join(format!("{shard_prefix}events.jsonl"));
            std::fs::File::create(&path).ok().map(BufWriter::new)
        });
        // Neural Integration Этап 1: собираем тренировочные данные (только single-shard)
        let mut training_writer: Option<BufWriter<std::fs::File>> = if shard_id.is_none() {
            stream_to.and_then(|dir| {
                let path = dir.join("training_data.jsonl");
                std::fs::File::create(&path).ok().map(BufWriter::new)
            })
        } else {
            None
        };
        let mut training_count: u64 = 0;

        let mut snapshots: Vec<TickSnapshot> = Vec::new();
        let mut events: Vec<InjectionEvent> = Vec::new();

        let tick_cmd = UclCommand::new(OpCode::TickForward, 0, 100, 0);
        let total = corpus.ticks_total;
        let progress_every = (total / 20).max(10_000); // ~5% intervals, min 10K
        let started = Instant::now();

        for tick in 0..total {
            // Inject texts scheduled for this tick
            if let Some(indices) = schedule.get(&tick) {
                for &idx in indices {
                    let entry: &CorpusEntry = &corpus.texts[idx];
                    let mut cmds = self.perceptor.perceive_and_bond(&entry.content);
                    let per_text_detected = self.perceptor.detect_subsystem(&entry.content);
                    let result = self.engine.process_and_observe(&cmds.remove(0));
                    for cmd in &cmds { self.engine.process_command(&cmd); }

                    let detected = self
                        .engine
                        .context_recognizer
                        .profile_store()
                        .dominant_primary_as_u8();

                    let exp_traces = self.engine.ashti.experience().trace_count();

                    let ev = InjectionEvent {
                        tick,
                        entry_id: entry.id.clone(),
                        expected_subsystem: entry.expected_subsystem.clone(),
                        detected_subsystem: detected,
                        coherence: result.coherence_score,
                        reflex_hit: result.reflex_hit,
                        passes: result.passes,
                        experience_traces_at_injection: exp_traces,
                        per_text_detected,
                    };
                    if let Some(w) = &mut event_writer {
                        if let Ok(line) = serde_json::to_string(&ev) {
                            let _ = writeln!(w, "{line}");
                        }
                    } else {
                        events.push(ev);
                    }
                }
            }

            self.engine.process_command(&tick_cmd);


            if tick > 0 && tick % progress_every == 0 {
                let elapsed = started.elapsed().as_secs_f64();
                let pct = tick as f64 / total as f64 * 100.0;
                let eta = if pct > 0.0 { elapsed / pct * (100.0 - pct) } else { 0.0 };
                eprintln!(
                    "{prefix} {tick}/{total} ({pct:.0}%) — {elapsed:.0}s elapsed, ~{eta:.0}s left"
                );
            }

            // Сбор тренировочных данных каждые TRAINING_SAMPLE_EVERY тиков
            if tick > 0 && tick % TRAINING_SAMPLE_EVERY == 0 {
                if let Some(w) = &mut training_writer {
                    let example = TrainingExample::capture(&self.engine, tick);
                    if let Ok(line) = serde_json::to_string(&example) {
                        let _ = writeln!(w, "{line}");
                        training_count += 1;
                    }
                }
            }

            if tick % corpus.snapshot_every == 0 {
                let snap = self.capture_snapshot(tick);
                if let Some(w) = &mut snap_writer {
                    if let Ok(line) = serde_json::to_string(&snap) {
                        let _ = writeln!(w, "{line}");
                    }
                } else {
                    snapshots.push(snap);
                }
            }
        }

        // Always capture final state
        if corpus.ticks_total % corpus.snapshot_every != 0 {
            let snap = self.capture_snapshot(corpus.ticks_total);
            if let Some(w) = &mut snap_writer {
                if let Ok(line) = serde_json::to_string(&snap) {
                    let _ = writeln!(w, "{line}");
                }
            } else {
                snapshots.push(snap);
            }
        }

        // Flush stream writers
        if let Some(w) = snap_writer.as_mut() { let _ = w.flush(); }
        if let Some(w) = event_writer.as_mut() { let _ = w.flush(); }
        if let Some(w) = training_writer.as_mut() {
            let _ = w.flush();
            eprintln!("{prefix} training_data.jsonl: {training_count} examples");
        }

        let elapsed = started.elapsed().as_secs_f64();
        let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
        eprintln!("{prefix} done in {elapsed:.1}s ({tps:.0} ticks/sec)");

        (snapshots, events)
    }

    fn capture_snapshot(&mut self, tick: u64) -> TickSnapshot {
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
            avg_shell_similarity: {
                // OBS-TD-02: читаем накопленное EMA от кристаллизованных фреймов.
                // avg_crystallized_shell_similarity обновляется при каждой кристаллизации,
                // поэтому не зависит от snapshot interval (кандидаты живут ~60 тиков).
                self.engine.frame_weaver.stats.avg_crystallized_shell_similarity
            },
            dilemma_active: cr.dilemma_store().active_count(),
            dilemma_resolved: cr.dilemma_store().resolved.len(),
        }
    }
}
