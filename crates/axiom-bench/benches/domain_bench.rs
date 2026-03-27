// Бенчмарки axiom-domain: EventGenerator, resonance_search
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use axiom_core::Token;
use axiom_domain::EventGenerator;
use axiom_arbiter::ExperienceModule as Experience;
use std::time::Duration;

// --- EventGenerator ---

fn bench_check_decay(c: &mut Criterion) {
    let mut gen = EventGenerator::new();
    gen.set_event_id(10000);
    let mut token = Token::new(1, 6, [0, 0, 0], 0);
    token.valence = 5;

    c.bench_function("EventGenerator::check_decay", |b| {
        b.iter(|| black_box(gen.check_decay(black_box(&token), black_box(0.001))))
    });
}

fn bench_generate_gravity_update(c: &mut Criterion) {
    let gen = EventGenerator::new();
    let token = Token::new(42, 7, [100, 200, 300], 1);

    c.bench_function("EventGenerator::generate_gravity_update", |b| {
        b.iter(|| black_box(gen.generate_gravity_update(black_box(&token))))
    });
}

fn bench_generate_collision(c: &mut Criterion) {
    let gen = EventGenerator::new();
    let t1 = Token::new(10, 6, [0, 0, 0], 1);
    let t2 = Token::new(20, 6, [50, 0, 0], 1);

    c.bench_function("EventGenerator::generate_collision", |b| {
        b.iter(|| black_box(gen.generate_collision(black_box(&t1), black_box(&t2))))
    });
}

// --- Experience::resonance_search ---

fn make_token(temp: u8, mass: u8) -> Token {
    let mut t = Token::new(1, 1, [0, 0, 0], 1);
    t.temperature = temp;
    t.mass = mass;
    t
}

fn bench_resonance_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Experience::resonance_search");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for trace_count in [0, 10, 100, 500, 1000] {
        let mut exp = Experience::new();
        for i in 0..trace_count {
            let t = make_token((i % 256) as u8, ((i * 3) % 256) as u8);
            exp.add_trace(t, 0.5 + (i as f32 % 50.0) / 100.0, i as u64 + 1);
        }
        let query = make_token(100, 100);

        group.bench_with_input(
            BenchmarkId::from_parameter(trace_count),
            &trace_count,
            |b, _| {
                b.iter(|| black_box(exp.resonance_search(black_box(&query))))
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_check_decay,
    bench_generate_gravity_update,
    bench_generate_collision,
    bench_resonance_search,
);
criterion_main!(benches);
