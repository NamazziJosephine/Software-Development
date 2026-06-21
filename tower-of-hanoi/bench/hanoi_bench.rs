//! Benchmark: recursive vs iterative Tower of Hanoi move generation.
//!
//! We benchmark the move-generation functions (not stdout) so the numbers
//! reflect the algorithms. Both return the same `Vec`, so the only difference
//! measured is recursion + call overhead vs the iterative peg bookkeeping.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hanoi::{iterative_moves, recursive_moves};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("hanoi");
    // n = 16 is the CSES maximum; we also sample smaller sizes to see scaling.
    for n in [10u32, 13, 16] {
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
            b.iter(|| recursive_moves(black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| iterative_moves(black_box(n)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
