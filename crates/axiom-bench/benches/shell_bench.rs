// Бенчмарки Shell V3.0: compute_shell, incremental_update, reconcile_batch
//
// Shell — семантический профиль токена [u8; 8], вычисляемый из совокупности
// активных Connection. Пересчитывается при каждом Connection-событии.
// Горячий путь: incremental_update (только dirty токены) vs full_recompute.

use axiom_core::Connection;
use axiom_shell::{
    compute_shell, reconcile_shell_batch, DomainShellCache, SemanticContributionTable,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// Создаёт N связей у токена с id=1 разных типов (категории 1-7)
fn make_connections(n: usize) -> Vec<Connection> {
    (0..n)
        .map(|i| Connection {
            source_id: 1,
            target_id: (i + 2) as u32,
            link_type: ((i % 7) + 1) as u16,
            strength: 0.3 + (i % 10) as f32 * 0.05,
            ..Connection::default()
        })
        .collect()
}

// ============================================================
// compute_shell: полный пересчёт одного токена
// ============================================================

fn bench_compute_shell(c: &mut Criterion) {
    let table = SemanticContributionTable::default_ashti_core();

    let mut group = c.benchmark_group("Shell::compute_shell");

    for conn_count in [0, 5, 20, 100] {
        let connections = make_connections(conn_count);
        group.bench_with_input(
            BenchmarkId::new("connections", conn_count),
            &conn_count,
            |b, _| {
                b.iter(|| {
                    black_box(compute_shell(
                        black_box(1u32),
                        black_box(&connections),
                        black_box(&table),
                    ))
                })
            },
        );
    }
    group.finish();
}

// ============================================================
// DomainShellCache::update_dirty_shells: incremental update
//
// Пересчитываются только dirty-токены.
// Измеряем при разном числе dirty-токенов при фиксированных 20 связях.
// ============================================================

fn bench_shell_incremental_update(c: &mut Criterion) {
    let table = SemanticContributionTable::default_ashti_core();
    let connections = make_connections(20);

    let mut group = c.benchmark_group("Shell::incremental_update");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for dirty_count in [1, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::new("dirty_tokens", dirty_count),
            &dirty_count,
            |b, &dc| {
                b.iter_batched(
                    || {
                        let mut cache = DomainShellCache::new(dc);
                        for i in 0..dc {
                            cache.mark_dirty(i);
                        }
                        cache
                    },
                    |mut cache| {
                        black_box(
                            cache.update_dirty_shells(black_box(&connections), black_box(&table)),
                        )
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

// ============================================================
// reconcile_shell_batch: heartbeat-reconciliation батча
//
// Сравнивает кэшированный Shell с пересчитанным — фиксирует drift.
// ============================================================

fn bench_reconcile_shell_batch(c: &mut Criterion) {
    let table = SemanticContributionTable::default_ashti_core();
    let connections = make_connections(20);

    let mut group = c.benchmark_group("Shell::reconcile_batch");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for batch_size in [1, 10, 50] {
        let indices: Vec<usize> = (0..batch_size).collect();
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size),
            &batch_size,
            |b, &bs| {
                b.iter_batched(
                    || DomainShellCache::new(bs),
                    |mut cache| {
                        black_box(reconcile_shell_batch(
                            black_box(&mut cache),
                            black_box(&indices),
                            black_box(&connections),
                            black_box(&table),
                        ))
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_compute_shell,
    bench_shell_incremental_update,
    bench_reconcile_shell_batch,
);
criterion_main!(benches);
