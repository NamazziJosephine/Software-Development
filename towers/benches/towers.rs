//! Criterion benchmark: both algorithms on several input shapes.
//!
//! The shape matters because it sets how big the data structure grows (= the
//! number of towers), which is what drives cache behaviour:
//!   * random       — large value range, a moderate number of towers
//!   * increasing    — worst case: every cube is a new tower (structure grows to n)
//!   * decreasing    — best case: one tower (structure stays size 1)
//!   * few_distinct  — values in 1..=1000, so a small set of distinct tops
//! Plus a size-scaling sweep on random input.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use towers::{btree, patience};

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn random(n: usize, range: u32) -> Vec<u32> {
    let mut rng = Lcg(0xabc123);
    (0..n).map(|_| 1 + rng.next_u32(range)).collect()
}
fn increasing(n: usize) -> Vec<u32> {
    (1..=n as u32).collect()
}
fn decreasing(n: usize) -> Vec<u32> {
    (1..=n as u32).rev().collect()
}

fn bench_pair(group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>, tag: &str, cubes: &[u32]) {
    group.bench_with_input(BenchmarkId::new("btree", tag), &cubes, |b, c| {
        b.iter(|| black_box(btree::min_towers(black_box(c))));
    });
    group.bench_with_input(BenchmarkId::new("patience", tag), &cubes, |b, c| {
        b.iter(|| black_box(patience::min_towers(black_box(c))));
    });
}

fn by_distribution(c: &mut Criterion) {
    let n = 200_000;
    let mut group = c.benchmark_group("distribution_n200k");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(30);

    bench_pair(&mut group, "random", &random(n, 1_000_000_000));
    bench_pair(&mut group, "increasing", &increasing(n));
    bench_pair(&mut group, "decreasing", &decreasing(n));
    bench_pair(&mut group, "few_distinct", &random(n, 1000));
    group.finish();
}

fn scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_random");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(30);

    for n in [10_000usize, 100_000, 200_000] {
        let cubes = random(n, 1_000_000_000);
        group.bench_with_input(BenchmarkId::new("btree", n), &cubes, |b, c| {
            b.iter(|| black_box(btree::min_towers(black_box(c))));
        });
        group.bench_with_input(BenchmarkId::new("patience", n), &cubes, |b, c| {
            b.iter(|| black_box(patience::min_towers(black_box(c))));
        });
    }
    group.finish();
}

criterion_group!(benches_group, by_distribution, scaling);
criterion_main!(benches_group);
