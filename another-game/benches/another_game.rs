//! Criterion benchmark: parity scan vs the binary-heap version.
//!
//! `all_even` is the fair algorithmic comparison: with no odd heap, NEITHER
//!   algorithm can stop early, so the parity scan does a full O(n) pass and the
//!   heap version does a full heapify + drain (O(n log n)). The gap is exactly
//!   the cost of the heap data structure.
//! `first_odd` puts the only odd heap first: the parity scan stops on element 0
//!   (O(1)), while the heap must still heapify and pop from the largest down, so
//!   it cannot exploit that early odd — a second angle on the heap's overhead.

use another_game::{heap, parity};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn all_even(n: usize) -> Vec<u32> {
    vec![2u32; n]
}
fn first_odd(n: usize) -> Vec<u32> {
    let mut v = vec![2u32; n];
    v[0] = 1; // single odd heap, at the front
    v
}

fn bench_shape(c: &mut Criterion, name: &str, make: fn(usize) -> Vec<u32>) {
    let mut group = c.benchmark_group(name);
    for n in [10_000usize, 100_000, 200_000] {
        let heaps = make(n);
        group.bench_with_input(BenchmarkId::new("parity", n), &n, |b, _| {
            b.iter(|| black_box(parity::solve(black_box(&heaps))));
        });
        group.bench_with_input(BenchmarkId::new("heap", n), &n, |b, _| {
            b.iter(|| black_box(heap::solve(black_box(&heaps))));
        });
    }
    group.finish();
}

fn benches(c: &mut Criterion) {
    bench_shape(c, "all_even", all_even);
    bench_shape(c, "first_odd", first_odd);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
