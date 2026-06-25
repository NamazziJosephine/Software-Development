//! Criterion benchmark: both algorithms timed on every test-case shape/size.
//!
//! A "test case" here is a (shape, n) pair. Company trees come in very
//! different shapes, and the shape is what stresses recursion, so we bench
//! three shapes:
//!   * chain  — depth = n (worst case for the call stack)
//!   * star   — depth 1, n-1 direct reports (shallow, wide)
//!   * random — a realistic mixed-depth tree
//! For each shape/size we time BOTH algorithms (build + traverse).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use subordinates::{iterative, recursive};

// Deterministic PRNG so the random tree is identical across runs.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn chain(n: usize) -> Vec<u32> {
    (2..=n as u32).map(|e| e - 1).collect()
}
fn star(n: usize) -> Vec<u32> {
    vec![1u32; n - 1]
}
fn random(n: usize) -> Vec<u32> {
    let mut rng = Lcg(0xdead_beef_cafe_1234);
    (2..=n).map(|e| 1 + rng.next((e - 1) as u32)).collect()
}

fn bench_shape(c: &mut Criterion, shape: &str, make: fn(usize) -> Vec<u32>, sizes: &[usize]) {
    let mut group = c.benchmark_group(shape);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);

    for &n in sizes {
        let boss = make(n);
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
            b.iter(|| black_box(recursive::count_subordinates(n, black_box(&boss))));
        });
        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| black_box(iterative::count_subordinates(n, black_box(&boss))));
        });
    }
    group.finish();
}

fn benches(c: &mut Criterion) {
    let sizes = [10_000usize, 100_000, 200_000];
    bench_shape(c, "chain", chain, &sizes);
    bench_shape(c, "star", star, &sizes);
    bench_shape(c, "random", random, &sizes);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
