// Бенчмарки axiom-domain: EventGenerator, resonance_search, Arbiter route
use axiom_arbiter::{Arbiter, ExperienceModule as Experience, COM};
use axiom_config::DomainConfig;
use axiom_core::Token;
use axiom_domain::EventGenerator;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
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
            |b, _| b.iter(|| black_box(exp.resonance_search(black_box(&query)))),
        );
    }
    group.finish();
}

// ============================================================
// Arbiter::route_token при разных порогах классификации
//
// Пороги управляют выбором пути:
//   Strict (200/180): мало рефлексов → чаще slow path
//   Loose  (50/30):   много рефлексов → чаще fast path (если обучена память)
//
// 50 traces — достаточно для срабатывания рефлексов при loose порогах.
// ============================================================

fn make_arbiter(reflex_t: u8, assoc_t: u8, trace_count: usize) -> Arbiter {
    let configs = [
        DomainConfig::factory_sutra(100),
        DomainConfig::factory_execution(101, 100),
        DomainConfig::factory_shadow(102, 100),
        DomainConfig::factory_codex(103, 100),
        DomainConfig::factory_map(104, 100),
        DomainConfig::factory_probe(105, 100),
        DomainConfig::factory_logic(106, 100),
        DomainConfig::factory_dream(107, 100),
        DomainConfig::factory_void(108, 100),
        DomainConfig::factory_experience(109, 100),
        DomainConfig::factory_maya(110, 100),
    ];
    let mut domain_map = HashMap::new();
    for c in &configs {
        domain_map.insert(c.domain_id, *c);
    }
    let mut arbiter = Arbiter::new(domain_map, COM::new());
    for (role, c) in configs.iter().enumerate() {
        let _ = arbiter.register_domain(role as u8, c.domain_id);
    }
    arbiter.experience_mut().set_thresholds(reflex_t, assoc_t);
    for i in 0..trace_count {
        let mut t = Token::new(i as u32 + 1, 100, [0, 0, 0], 1);
        t.temperature = (i % 256) as u8;
        t.mass = ((i * 3) % 256) as u8;
        arbiter.experience_mut().add_trace(t, 0.9, i as u64 + 1);
    }
    arbiter
}

fn bench_arbiter_route(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arbiter::route_token");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    let token = Token::new(1, 100, [10, 20, 30], 1);

    // (label, reflex_threshold, assoc_threshold)
    let configs: &[(&str, u8, u8)] = &[("strict_200_180", 200, 180), ("loose_50_30", 50, 30)];

    for &(label, reflex_t, assoc_t) in configs {
        group.bench_with_input(
            BenchmarkId::new("thresholds", label),
            &(reflex_t, assoc_t),
            |b, &(rt, at)| {
                b.iter_batched(
                    || make_arbiter(rt, at, 50),
                    |mut arbiter| black_box(arbiter.route_token(black_box(token), 0)),
                    criterion::BatchSize::SmallInput,
                )
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
    bench_arbiter_route,
);
criterion_main!(benches);
