// Бенчмарки axiom-space: SpatialHashGrid — горячий путь физики
use axiom_space::{distance2, SpatialHashGrid};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn make_positions(count: usize) -> Vec<(i16, i16, i16)> {
    (0..count)
        .map(|i| {
            let x = ((i * 37) % 1000) as i16 - 500;
            let y = ((i * 53) % 1000) as i16 - 500;
            let z = ((i * 71) % 1000) as i16 - 500;
            (x, y, z)
        })
        .collect()
}

fn bench_grid_rebuild(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpatialHashGrid::rebuild");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    for count in [100, 500, 1000, 5000] {
        let positions = make_positions(count);
        let mut grid = SpatialHashGrid::new();

        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &positions,
            |b, positions| {
                b.iter(|| {
                    grid.rebuild(positions.len(), |i| positions[i]);
                    black_box(&grid);
                })
            },
        );
    }
    group.finish();
}

fn bench_find_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpatialHashGrid::find_neighbors");

    for count in [100, 500, 1000] {
        let positions = make_positions(count);
        let mut grid = SpatialHashGrid::new();
        grid.rebuild(positions.len(), |i| positions[i]);

        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, _| {
            b.iter(|| {
                let result = grid.find_neighbors(
                    black_box(0i16),
                    black_box(0i16),
                    black_box(0i16),
                    black_box(100i16),
                    |idx| positions[idx as usize],
                );
                black_box(result.len())
            })
        });
    }
    group.finish();
}

fn bench_distance2(c: &mut Criterion) {
    c.bench_function("distance2", |b| {
        b.iter(|| {
            black_box(distance2(
                black_box(100i16),
                black_box(200i16),
                black_box(300i16),
                black_box(400i16),
                black_box(500i16),
                black_box(600i16),
            ))
        })
    });
}

criterion_group!(
    benches,
    bench_grid_rebuild,
    bench_find_neighbors,
    bench_distance2,
);
criterion_main!(benches);
