mod corpus;
mod metrics;
mod report;
mod runner;
mod shard;
mod training;

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let corpus_path = args.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("config/obs/corpus.yaml")
    });
    let out_dir = args.get(2).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("obs_out")
    });
    let anchors_dir = args.get(3).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("config/anchors")
    });

    // Load corpus
    let corpus = match corpus::Corpus::from_yaml(&corpus_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[observe] failed to load corpus {}: {e}", corpus_path.display());
            std::process::exit(1);
        }
    };

    eprintln!(
        "[observe] corpus: {} entries, {} ticks",
        corpus.texts.len(),
        corpus.ticks_total
    );

    // Build runner
    let anchors_arg = if anchors_dir.exists() {
        Some(anchors_dir.as_path())
    } else {
        eprintln!("[observe] anchors dir not found — running without anchors");
        None
    };

    let n_shards = corpus.shards.max(1);

    // Prepare output directory early — needed for streaming
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("[observe] cannot create out_dir {}: {e}", out_dir.display());
        std::process::exit(1);
    }

    let (snapshots, events) = if n_shards > 1 {
        eprintln!("[observe] running… ({n_shards} parallel shards, streaming to {}/)", out_dir.display());
        shard::run_parallel(&corpus, anchors_arg, n_shards)
    } else {
        let mut runner = match runner::ObsRunner::new(anchors_arg) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[observe] engine init failed: {e}");
                std::process::exit(1);
            }
        };
        eprintln!("[observe] running… (streaming to {}/)", out_dir.display());
        let result = runner.run_streaming(&corpus, &out_dir);
        // Экспортируем трейсы для последующего импорта в живой движок (OBS-FEED-01)
        match runner.export_traces(&out_dir) {
            Ok(n) => eprintln!("[observe] exported {n} traces → {}/traces.bin", out_dir.display()),
            Err(e) => eprintln!("[observe] trace export skipped: {e}"),
        }
        result
    };
    eprintln!("[observe] done. {} snapshots in RAM, {} events in RAM", snapshots.len(), events.len());

    // Write report
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("[observe] cannot create out_dir {}: {e}", out_dir.display());
        std::process::exit(1);
    }

    // If streaming mode was used, load from JSONL files for report generation
    let loaded_snaps;
    let loaded_events;
    let (snap_ref, ev_ref) = if snapshots.is_empty() && events.is_empty() {
        loaded_snaps = report::load_snapshots_jsonl(&out_dir.join("snapshots.jsonl"));
        loaded_events = report::load_events_jsonl(&out_dir.join("events.jsonl"));
        eprintln!("[observe] loaded {} snapshots, {} events from JSONL", loaded_snaps.len(), loaded_events.len());
        (loaded_snaps.as_slice(), loaded_events.as_slice())
    } else {
        (snapshots.as_slice(), events.as_slice())
    };

    let report = report::generate_markdown(&corpus, snap_ref, ev_ref);
    let report_path = out_dir.join("report.md");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("[observe] failed to write report: {e}");
        std::process::exit(1);
    }

    println!("[observe] report written to {}", report_path.display());
    print!("{report}");
}
