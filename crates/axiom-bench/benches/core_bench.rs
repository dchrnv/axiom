// Бенчмарки axiom-core: Token, Connection, Event
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use axiom_core::{Token, Connection, Event, EventType, EventPriority};

fn bench_token_new(c: &mut Criterion) {
    c.bench_function("Token::new", |b| {
        b.iter(|| {
            black_box(Token::new(
                black_box(42),
                black_box(7),
                black_box([100i16, 200, 300]),
                black_box(1000),
            ))
        })
    });
}

fn bench_token_compute_resonance(c: &mut Criterion) {
    let t1 = Token::new(1, 1, [0, 0, 0], 1);
    let mut t2 = Token::new(2, 1, [100, 100, 100], 1);
    t2.temperature = 150;
    t2.mass = 80;

    c.bench_function("Token::compute_resonance", |b| {
        b.iter(|| black_box(t1.compute_resonance(black_box(&t2))))
    });
}

fn bench_token_copy(c: &mut Criterion) {
    let token = Token::new(1, 1, [0, 0, 0], 1);
    c.bench_function("Token copy (Copy trait)", |b| {
        b.iter(|| {
            let t: Token = black_box(token);
            black_box(t)
        })
    });
}

fn bench_event_new(c: &mut Criterion) {
    c.bench_function("Event::new", |b| {
        b.iter(|| {
            black_box(Event::new(
                black_box(1),
                black_box(1),
                black_box(EventType::TokenCollision),
                black_box(EventPriority::Normal),
                black_box(0xdeadbeef),
                black_box(10),
                black_box(20),
                black_box(0),
            ))
        })
    });
}

fn bench_connection_default(c: &mut Criterion) {
    c.bench_function("Connection::default", |b| {
        b.iter(|| black_box(Connection::default()))
    });
}

// Размер структур (проверка на этапе компиляции, документируется через bench)
fn bench_struct_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("struct_sizes");
    group.bench_function("Token (64 bytes)", |b| {
        b.iter(|| {
            assert_eq!(std::mem::size_of::<Token>(), 64);
            black_box(64usize)
        })
    });
    group.bench_function("Connection (64 bytes)", |b| {
        b.iter(|| {
            assert_eq!(std::mem::size_of::<Connection>(), 64);
            black_box(64usize)
        })
    });
    group.bench_function("Event (64 bytes)", |b| {
        b.iter(|| {
            assert_eq!(std::mem::size_of::<Event>(), 64);
            black_box(64usize)
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_token_new,
    bench_token_compute_resonance,
    bench_token_copy,
    bench_event_new,
    bench_connection_default,
    bench_struct_sizes,
);
criterion_main!(benches);
