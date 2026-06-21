//! Benchmark: in-place greedy vs prefix-maximum auxiliary array.
//!
//! No library is used, so this file keeps its own copies of the two algorithms
//! as pure functions. They are identical in logic to the src/bin/ solutions.
//!
//! "Each test case" here means a range of array sizes (1e3 .. 2e5, the CSES
//! maximum). The same deterministic pseudo-random input is fed to both
//! algorithms at each size so the comparison is fair.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Algorithm A — in-place greedy, O(1) extra space.
fn solve_inplace(a: &[i64]) -> i64 {
    let mut moves = 0i64;
    let mut prev = a[0];
    for &x in &a[1..] {
        if x < prev {
            moves += prev - x;
        } else {
            prev = x;
        }
    }
    moves
}

/// Algorithm B — prefix-maximum auxiliary array, O(n) extra space.
fn solve_prefix_max(a: &[i64]) -> i64 {
    let n = a.len();
    let mut pmax = Vec::with_capacity(n);
    let mut running = i64::MIN;
    for &x in a {
        running = running.max(x);
        pmax.push(running);
    }
    let mut moves = 0i64;
    for i in 0..n {
        moves += pmax[i] - a[i];
    }
    moves
}

/// Deterministic pseudo-random array (xorshift) so runs are reproducible.
/// Values land in [1, 1e9] to mimic the CSES constraints.
fn make_input(n: usize) -> Vec<i64> {
    let mut state: u64 = 0x9E3779B97F4A7C15;
    (0..n)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state % 1_000_000_000) as i64 + 1
        })
        .collect()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("increasing_array");
    for &n in &[1_000usize, 10_000, 100_000, 200_000] {
        let input = make_input(n);
        group.bench_with_input(BenchmarkId::new("inplace", n), &input, |b, inp| {
            b.iter(|| solve_inplace(black_box(inp)))
        });
        group.bench_with_input(BenchmarkId::new("prefix_max", n), &input, |b, inp| {
            b.iter(|| solve_prefix_max(black_box(inp)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
