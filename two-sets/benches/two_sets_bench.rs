// Criterion is the industry-standard Rust benchmarking library.
// It runs each function many times and gives you statistically reliable timing.
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

// ── Recursive approach ────────────────────────────────────────────────────────

// Same function from src/bin/recursive.rs.
// We recurse from `cur` down to 1, greedily filling set A up to `rem`.
fn solve_recursive(cur: i64, rem: i64, a: &mut Vec<i64>, b: &mut Vec<i64>) {
    if cur == 0 { return; }
    if rem >= cur { a.push(cur); solve_recursive(cur - 1, rem - cur, a, b); }
    else          { b.push(cur); solve_recursive(cur - 1, rem,       a, b); }
}

fn run_recursive(n: i64) {
    let total = n * (n + 1) / 2;
    if total % 2 != 0 { return; }
    let (mut a, mut b) = (vec![], vec![]);
    // We need a large stack here too — same reason as the bin: depth = n
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || { solve_recursive(n, total / 2, &mut a, &mut b); (a, b) })
        .unwrap().join().unwrap();
}

// ── Iterative approach ────────────────────────────────────────────────────────

// Same logic, but with a plain for-loop instead of recursion.
// No thread needed since there is no call stack to overflow.
fn run_iterative(n: i64) {
    let total = n * (n + 1) / 2;
    if total % 2 != 0 { return; }
    let mut a = vec![];
    let mut b = vec![];
    let mut rem = total / 2;
    for cur in (1..=n).rev() {
        if rem >= cur { a.push(cur); rem -= cur; }
        else          { b.push(cur); }
    }
}

// ── Benchmark definitions ─────────────────────────────────────────────────────

fn benchmark(c: &mut Criterion) {
    // We test multiple values of n so we can see how each algorithm scales.
    let inputs = [1_000, 10_000, 100_000, 1_000_000];

    // BenchmarkId lets Criterion label each run with the n value it used.
    let mut group = c.benchmark_group("two_sets");

    for &n in &inputs {
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
            // `b.iter` is the measurement loop — Criterion decides how many times to run it.
            b.iter(|| run_recursive(n));
        });

        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| run_iterative(n));
        });
    }

    group.finish();
}

// These macros wire up the benchmark runner so you don't need a manual main().
criterion_group!(benches, benchmark);
criterion_main!(benches);