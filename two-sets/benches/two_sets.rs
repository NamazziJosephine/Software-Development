//! Criterion benchmark: both algorithms timed on every test-case size.
//!
//! Each "test case" is a value of n. For every size we run BOTH algorithms,
//! so the report contains a head-to-head pair per size. We time the pure
//! partition computation (the part that differs between the algorithms); the
//! I/O formatting is identical for both and lives only in main.rs / cses/.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use two_sets::{greedy, modular};

fn bench_partitions(c: &mut Criterion) {
    // Spread over three orders of magnitude, both feasible residues included
    // (999_999 exercises the modular special-case path for n % 4 == 3).
    let sizes: [u32; 5] = [1_000, 10_000, 100_000, 999_999, 1_000_000];

    let mut group = c.benchmark_group("two_sets");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(30);

    for &n in &sizes {
        group.bench_with_input(BenchmarkId::new("greedy", n), &n, |bench, &n| {
            bench.iter(|| black_box(greedy::partition(black_box(n))));
        });
        group.bench_with_input(BenchmarkId::new("modular", n), &n, |bench, &n| {
            bench.iter(|| black_box(modular::partition(black_box(n))));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_partitions);
criterion_main!(benches);
