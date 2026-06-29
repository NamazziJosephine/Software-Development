//! Criterion benchmark: both algorithms on every test-case input.
//!
//! Insertion ORDER and size are what matter, so a "test case" is an
//! (order, n) pair, and each one times BOTH algorithms.
//!   * random — lights arrive in random order (the typical case)
//!   * sorted — lights arrive left-to-right (stresses locality differently)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::BTreeSet;
use std::time::Duration;
use traffic_lights::{bst, offline};

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

const X: u32 = 1_000_000_000;

fn distinct_positions(seed: u64, n: usize) -> Vec<u32> {
    let mut rng = Lcg(seed);
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert(1 + rng.next_u32(X - 1));
    }
    set.into_iter().collect()
}

fn random_order(n: usize) -> Vec<u32> {
    let mut v = distinct_positions(0xabcdef, n);
    let mut rng = Lcg(0x1234);
    for i in (1..v.len()).rev() {
        let j = rng.next_u32((i + 1) as u32) as usize;
        v.swap(i, j);
    }
    v
}

fn sorted_order(n: usize) -> Vec<u32> {
    distinct_positions(0xabcdef, n) // already ascending
}

fn bench_order(c: &mut Criterion, name: &str, make: fn(usize) -> Vec<u32>, sizes: &[usize]) {
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);

    for &n in sizes {
        let pos = make(n);
        group.bench_with_input(BenchmarkId::new("bst", n), &n, |b, _| {
            b.iter(|| black_box(bst::max_gaps(X, black_box(&pos))));
        });
        group.bench_with_input(BenchmarkId::new("offline", n), &n, |b, _| {
            b.iter(|| black_box(offline::max_gaps(X, black_box(&pos))));
        });
    }
    group.finish();
}

fn benches(c: &mut Criterion) {
    let sizes = [10_000usize, 100_000, 200_000];
    bench_order(c, "random", random_order, &sizes);
    bench_order(c, "sorted", sorted_order, &sizes);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
