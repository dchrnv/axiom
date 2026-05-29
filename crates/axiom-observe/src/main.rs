mod corpus;
mod metrics;
mod report;
mod runner;
mod shard;

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

    let (snapshots, events) = if n_shards > 1 {
        eprintln!("[observe] running… ({n_shards} parallel shards)");
        shard::run_parallel(&corpus, anchors_arg, n_shards)
    } else {
        let mut runner = match runner::ObsRunner::new(anchors_arg) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[observe] engine init failed: {e}");
                std::process::exit(1);
            }
        };
        eprintln!("[observe] running…");
        runner.run(&corpus)
    };
    eprintln!("[observe] done. {} snapshots, {} injection events", snapshots.len(), events.len());

    // Write report
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("[observe] cannot create out_dir {}: {e}", out_dir.display());
        std::process::exit(1);
    }

    let report = report::generate_markdown(&corpus, &snapshots, &events);
    let report_path = out_dir.join("report.md");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("[observe] failed to write report: {e}");
        std::process::exit(1);
    }

    println!("[observe] report written to {}", report_path.display());
    print!("{report}");
}
