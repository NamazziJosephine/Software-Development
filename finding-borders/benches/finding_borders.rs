//! Criterion benchmark: both algorithms on every test-case input family.
//!
//! Border structure is what stresses each algorithm, so a "test case" is an
//! (input family, n) pair, and each one times BOTH algorithms. Families:
//!   * random26 — random a..z: very few borders (ordinary text)
//!   * random2  — random a..b: more repetition, more borders
//!   * periodic — "abc" repeated: regular borders every 3 chars
//!   * all_a    — "aaaa...": every length is a border (worst case: longest
//!                KMP fallback chains, most border-chain jumps)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use finding_borders::{kmp, zalgo};
use std::time::Duration;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn random(n: usize, alpha: u32, seed: u64) -> Vec<u8> {
    let mut r = Lcg(seed);
    (0..n).map(|_| b'a' + r.next(alpha) as u8).collect()
}
fn periodic(n: usize) -> Vec<u8> {
    (0..n).map(|i| b'a' + (i % 3) as u8).collect()
}
fn all_a(n: usize) -> Vec<u8> {
    vec![b'a'; n]
}

fn bench_family(c: &mut Criterion, name: &str, make: impl Fn(usize) -> Vec<u8>, sizes: &[usize]) {
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);

    for &n in sizes {
        let s = make(n);
        group.bench_with_input(BenchmarkId::new("kmp", n), &n, |b, _| {
            b.iter(|| black_box(kmp::border_lengths(black_box(&s))));
        });
        group.bench_with_input(BenchmarkId::new("zalgo", n), &n, |b, _| {
            b.iter(|| black_box(zalgo::border_lengths(black_box(&s))));
        });
    }
    group.finish();
}

fn benches(c: &mut Criterion) {
    let sizes = [100_000usize, 1_000_000];
    bench_family(c, "random26", |n| random(n, 26, 0x1111), &sizes);
    bench_family(c, "random2", |n| random(n, 2, 0x2222), &sizes);
    bench_family(c, "periodic", periodic, &sizes);
    bench_family(c, "all_a", all_a, &sizes);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
