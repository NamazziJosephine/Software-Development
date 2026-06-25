//! Criterion benchmark: all three algorithms on every test-case grid.
//!
//! Grid shape is what stresses each approach, so a "test case" is a (shape,
//! side) pair, and for each we time ALL THREE algorithms. Shapes:
//!   * open    — every cell floor: ONE giant room (biggest frontier / set)
//!   * checker — floor iff (r+c) even: ~half the cells, all isolated rooms
//!   * random  — ~50% walls: a realistic mix of small/medium rooms

use counting_rooms::{bfs, dfs, union_find, Grid};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn open(side: usize) -> Grid {
    Grid::new(side, side, vec![true; side * side])
}
fn checker(side: usize) -> Grid {
    let floor = (0..side * side).map(|i| ((i / side) + (i % side)) % 2 == 0).collect();
    Grid::new(side, side, floor)
}
fn random(side: usize) -> Grid {
    let mut rng = Lcg(0x1357_9bdf_2468_ace0);
    let floor = (0..side * side).map(|_| rng.next(100) >= 50).collect();
    Grid::new(side, side, floor)
}

fn bench_shape(c: &mut Criterion, name: &str, make: fn(usize) -> Grid, sides: &[usize]) {
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);

    for &side in sides {
        let g = make(side);
        group.bench_with_input(BenchmarkId::new("bfs", side), &side, |b, _| {
            b.iter(|| black_box(bfs::count_rooms(black_box(&g))));
        });
        group.bench_with_input(BenchmarkId::new("dfs", side), &side, |b, _| {
            b.iter(|| black_box(dfs::count_rooms(black_box(&g))));
        });
        group.bench_with_input(BenchmarkId::new("union_find", side), &side, |b, _| {
            b.iter(|| black_box(union_find::count_rooms(black_box(&g))));
        });
    }
    group.finish();
}

fn benches(c: &mut Criterion) {
    let sides = [500usize, 1000];
    bench_shape(c, "open", open, &sides);
    bench_shape(c, "checker", checker, &sides);
    bench_shape(c, "random", random, &sides);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
