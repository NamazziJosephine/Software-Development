//! Benchmark: recursive vs iterative Tower of Hanoi move generation.
//!
//! Per the assignment, we measure BOTH algorithms on EACH test case — the
//! full CSES input range n = 1..=16, not just a few sizes.
//!
//! We benchmark the move-generation functions (not stdout) so the numbers
//! reflect the algorithms. Both return the same `Vec`, so the only difference
//! measured is recursion + call overhead vs the iterative peg bookkeeping.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hanoi::{iterative_moves, recursive_moves};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("hanoi");
    // Every CSES test case: n = 1 through 16.
    for n in 1..=16u32 {
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
