// Parallel corpus sharding for axiom-observe.
//
// Each shard is an independent AxiomEngine with a round-robin slice of corpus texts.
// Shards run concurrently on separate OS threads (std::thread::spawn).
// Results are merged: events concatenated (global accuracy), snapshots from shard 0
// (engine state timeline).

use crate::corpus::{Corpus, CorpusEntry};
use crate::metrics::{InjectionEvent, TickSnapshot};
use crate::runner::ObsRunner;

/// Split corpus texts round-robin across `n` shards.
/// Each shard is a Corpus clone with the same tick parameters but only its texts.
pub fn split(corpus: &Corpus, n: usize) -> Vec<Corpus> {
    assert!(n >= 1);
    let mut buckets: Vec<Vec<CorpusEntry>> = (0..n).map(|_| Vec::new()).collect();
    for (i, entry) in corpus.texts.iter().enumerate() {
        buckets[i % n].push(entry.clone());
    }
    buckets
        .into_iter()
        .map(|texts| Corpus {
            ticks_total: corpus.ticks_total,
            snapshot_every: corpus.snapshot_every,
            max_tokens_per_domain: corpus.max_tokens_per_domain,
            shards: 1,
            texts,
        })
        .collect()
}

/// Merge shard results.
/// Events: concatenated from all shards (complete picture for accuracy metrics).
/// Snapshots: taken from shard 0 (engine state timeline — representative sample).
pub fn merge(
    results: Vec<(Vec<TickSnapshot>, Vec<InjectionEvent>)>,
) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
    let mut all_events = Vec::new();
    let mut primary_snapshots = Vec::new();

    for (i, (snapshots, events)) in results.into_iter().enumerate() {
        all_events.extend(events);
        if i == 0 {
            primary_snapshots = snapshots;
        }
    }

    // Sort events by tick for stable report ordering
    all_events.sort_by_key(|e| e.tick);

    (primary_snapshots, all_events)
}

/// Run `n` shards of `corpus` in parallel, merge and return results.
/// Anchors dir is passed through to each ObsRunner.
pub fn run_parallel(
    corpus: &Corpus,
    anchors_dir: Option<&std::path::Path>,
    n: usize,
) -> (Vec<TickSnapshot>, Vec<InjectionEvent>) {
    let shards = split(corpus, n);
    let anchors_owned: Option<std::path::PathBuf> = anchors_dir.map(|p| p.to_path_buf());

    eprintln!("[observe] shards={n}, texts/shard≈{}", corpus.texts.len().div_ceil(n));

    let handles: Vec<_> = shards
        .into_iter()
        .enumerate()
        .map(|(idx, shard)| {
            let anchors = anchors_owned.clone();
            std::thread::spawn(move || {
                let anchors_ref = anchors.as_deref();
                match ObsRunner::new(anchors_ref) {
                    Ok(mut runner) => {
                        eprintln!("[observe] shard {idx}: {} texts", shard.texts.len());
                        let result = runner.run_shard(idx, &shard);
                        result
                    }
                    Err(e) => {
                        eprintln!("[observe] shard {idx}: engine init failed: {e}");
                        (Vec::new(), Vec::new())
                    }
                }
            })
        })
        .collect();

    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap_or_else(|_| (Vec::new(), Vec::new())))
        .collect();

    merge(results)
}
