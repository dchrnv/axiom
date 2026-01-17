use axiom_core::{
    ConnectionType, ConnectionV3, CoordinateSpace, EntityType, ExperienceEvent, ExperienceStream,
    Graph, IntuitionEngine, SamplingStrategy, Token,
};
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

const SCALES: [usize; 4] = [10, 100_000, 1_000_000, 9_900_000];

fn main() {
    println!("================================================================");
    println!("   AXIOM CORE - COMPREHENSIVE BENCHMARK SUITE (FAT BENCH)");
    println!("================================================================");
    println!("Scales: {:?}", SCALES);
    println!();

    for &scale in &SCALES {
        run_benchmark_at_scale(scale);
    }
}

fn run_benchmark_at_scale(scale: usize) {
    println!("----------------------------------------------------------------");
    println!("BENCHMARKING SCALE: {} items", scale);
    println!("----------------------------------------------------------------");

    // 1. TOKEN BENCHMARKS
    benchmark_tokens(scale);

    // 2. CONNECTION BENCHMARKS
    benchmark_connections(scale);

    // 3. GRAPH BENCHMARKS
    benchmark_graph(scale);

    // 4. INTUITION BENCHMARKS
    benchmark_intuition(scale);

    println!();
}

fn benchmark_tokens(scale: usize) {
    println!(">> MODULE: TOKENS");

    // Creation
    let start = Instant::now();
    let mut tokens = Vec::with_capacity(scale);
    for i in 0..scale {
        let mut token = Token::new(i as u32);
        token.set_entity_type(EntityType::Concept);
        token.set_coordinates(CoordinateSpace::L1Physical, 1.0, 2.0, 3.0);
        tokens.push(token);
    }
    let duration = start.elapsed();
    report_metric("Creation", scale, duration);

    // Update (modify weight and coordinates)
    let start = Instant::now();
    for token in tokens.iter_mut() {
        token.weight = 0.8;
        token.set_coordinates(CoordinateSpace::L4Emotional, 0.5, -0.5, 0.0);
    }
    let duration = start.elapsed();
    report_metric("Update", scale, duration);

    // Spatial Query Simulation (iterate and distance check)
    let start = Instant::now();
    let target = [1.0, 2.0, 3.0];
    let mut matches = 0;
    for token in tokens.iter() {
        let coords = token.get_coordinates(CoordinateSpace::L1Physical);
        let dist_sq = (coords[0] - target[0]).powi(2)
            + (coords[1] - target[1]).powi(2)
            + (coords[2] - target[2]).powi(2);
        if dist_sq < 0.1 {
            matches += 1;
        }
    }
    let duration = start.elapsed();
    report_metric("Spatial Query (L1)", scale, duration);

    // Cleanup to free memory for next steps
    drop(tokens);
}

fn benchmark_connections(scale: usize) {
    println!(">> MODULE: CONNECTIONS");

    // Creation
    let start = Instant::now();
    let mut connections = Vec::with_capacity(scale);
    for i in 0..scale {
        let mut conn = ConnectionV3::new(i as u32, (i + 1) as u32);
        conn.set_connection_type(ConnectionType::AssociatedWith);
        connections.push(conn);
    }
    let duration = start.elapsed();
    report_metric("Creation", scale, duration);

    // Learning Update (simulate reinforcement)
    let start = Instant::now();
    for conn in connections.iter_mut() {
        conn.activate();
        conn.update_confidence(true);
    }
    let duration = start.elapsed();
    report_metric("Learning Update", scale, duration);

    drop(connections);
}

fn benchmark_graph(scale: usize) {
    println!(">> MODULE: GRAPH");

    // We limit graph scale because it's heavier
    let graph_scale = if scale > 1_000_000 { 1_000_000 } else { scale };
    if scale > graph_scale {
        println!(
            "(Capping graph benchmark at {} for memory safety)",
            graph_scale
        );
    }

    let mut graph = Graph::new();

    // Node Insertion
    let start = Instant::now();
    for i in 0..graph_scale {
        graph.add_node(i as u32);
    }
    let duration = start.elapsed();
    report_metric("Node Insertion", graph_scale, duration);

    // Edge Insertion (connect i to i+1)
    let start = Instant::now();
    for i in 0..graph_scale - 1 {
        let from_id = i as u32;
        let to_id = (i + 1) as u32;
        let edge_type = ConnectionType::AssociatedWith as u8;
        let edge_id = Graph::compute_edge_id(from_id, to_id, edge_type);

        let _ = graph.add_edge(edge_id, from_id, to_id, edge_type, 1.0, false);
    }
    let duration = start.elapsed();
    report_metric("Edge Insertion", graph_scale, duration);
}

fn benchmark_intuition(scale: usize) {
    println!(">> MODULE: INTUITION");

    // Ensure capacity is enough for the scale
    let capacity = std::cmp::max(scale, 1000);
    let stream = Arc::new(ExperienceStream::new(capacity, 100));

    // Pre-fill the stream with events
    // We measure Ingestion (Writing) separately
    let fill_start = Instant::now();
    for i in 0..scale {
        let mut event = ExperienceEvent::default();
        event.event_id = i as u128;
        // Fill some data to avoid zero-optimizations
        event.state[0] = 1.0;
        let _ = stream.write_event(event);
    }
    let fill_duration = fill_start.elapsed();
    report_metric("Ingestion (Write)", scale, fill_duration);

    // Use builder to create engine
    let _engine = IntuitionEngine::builder()
        .with_experience(stream.clone())
        .build()
        .expect("Failed to create IntuitionEngine");

    // Processing Cycle Simulation
    // We measure how fast we can sample batches from a populated stream.
    // Sampling from a large buffer is O(N) because it scans/collects events.
    // We run a fixed number of cycles to measure throughput (Cycles/sec).
    let num_cycles = 100; // Fixed number of cycles to measure latency/throughput

    let start = Instant::now();
    for _ in 0..num_cycles {
        // Simulate a cycle by sampling
        let _ = stream.sample_batch(1, SamplingStrategy::Uniform);
    }
    let duration = start.elapsed();
    report_metric("Cycle Throughput", num_cycles, duration);
}

fn report_metric(name: &str, count: usize, duration: Duration) {
    let secs = duration.as_secs_f64();
    let ops_per_sec = count as f64 / secs;

    // Format numbers nicely
    let ops_str = if ops_per_sec > 1_000_000.0 {
        format!("{:.2} M/s", ops_per_sec / 1_000_000.0)
    } else if ops_per_sec > 1_000.0 {
        format!("{:.2} K/s", ops_per_sec / 1_000.0)
    } else {
        format!("{:.2} /s", ops_per_sec)
    };

    println!("  - {:<20}: {:>10.4}s | {:>12}", name, secs, ops_str);
}
