// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
//! UPO v2.1 benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use axiom_core::{Connection, DynamicTrace, Screen, Token, UPO, UPOConfig, token::STATE_ACTIVE, TraceSourceType};

fn bench_token_creation(c: &mut Criterion) {
    c.bench_function("token_new", |b| {
        let mut id = 0u32;
        b.iter(|| {
            id = id.wrapping_add(1);
            black_box(Token::new(black_box(id), 0))
        })
    });
}

fn bench_connection_creation(c: &mut Criterion) {
    c.bench_function("connection_new", |b| {
        b.iter(|| black_box(Connection::new(100, 200, 0)))
    });
}

fn bench_causal_clock(c: &mut Criterion) {
    c.bench_function("causal_clock_next", |b| {
        b.iter(|| black_box(axiom_core::CausalClock::next()))
    });
}

fn bench_upo_compute(c: &mut Criterion) {
    let tokens: Vec<Token> = (0..100)
        .map(|i| {
            let mut t = Token::new(i, 0);
            t.velocity = [100, 50, -30];
            t.mass = 128;
            t.temperature = 64;
            t.valence = 1;
            t.state = STATE_ACTIVE;
            t
        })
        .collect();
    let connections: Vec<Connection> = vec![];
    let mut upo = UPO::new(UPOConfig::default());

    c.bench_function("upo_compute_100_tokens", |b| {
        b.iter(|| black_box(upo.compute(black_box(&tokens), black_box(&connections), 1)))
    });
}

fn bench_screen_write(c: &mut Criterion) {
    let mut screen = Screen::new([1000, 1000, 1000], 0.001, 0.01);
    screen.set_current_event(100);
    let trace = DynamicTrace::new(10, -20, 50, 1.0, 440.0, 100, TraceSourceType::Token, 1, 0);

    c.bench_function("screen_write", |b| {
        b.iter(|| screen.write(black_box(&trace)))
    });
}

criterion_group!(
    benches,
    bench_token_creation,
    bench_connection_creation,
    bench_causal_clock,
    bench_upo_compute,
    bench_screen_write
);
criterion_main!(benches);
