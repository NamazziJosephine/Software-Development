//! Criterion benchmark: BFS vs A* on map types that make the heuristic helpful,
//! useless, or actively misleading. Criterion measures TIME; since the *reason*
//! a heuristic helps is the number of cells expanded, this bench also prints an
//! expansion-count table (to stderr) before timing.
//!
//! Maps (all 1000x1000):
//!   * open_corners — A and B at opposite corners, no walls. Here g+h is the
//!     SAME for every cell, so the heuristic cannot discriminate: A* explores
//!     the whole grid like BFS.
//!   * open_strip   — A and B at opposite ends of the MIDDLE ROW, no walls.
//!     Here g+h is minimised along the straight path, so A* expands a thin band.
//!   * walls_20     — 20% random walls: walls break the symmetry and give the
//!     heuristic something to discriminate on.
//!   * comb_maze    — a serpentine maze: B is close in Manhattan distance but
//!     far by path, so the heuristic is misleading.
//!   * no_path      — a full wall splits the grid: the goal is unreachable, so
//!     every reachable cell must be examined regardless of heuristic.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use labyrinth::{astar, bfs, Maze};

const N: usize = 1000;

fn open_corners() -> Maze {
    let mut cells = vec![b'.'; N * N];
    let (s, e) = (0, N * N - 1);
    cells[s] = b'A';
    cells[e] = b'B';
    Maze { n: N, m: N, cells, start: s, end: e }
}
fn open_strip() -> Maze {
    let mut cells = vec![b'.'; N * N];
    let s = (N / 2) * N;
    let e = (N / 2) * N + (N - 1);
    cells[s] = b'A';
    cells[e] = b'B';
    Maze { n: N, m: N, cells, start: s, end: e }
}
fn walls_20() -> Maze {
    let mut state = 0x55u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 33) as u32) % 100
    };
    let mut cells = vec![b'.'; N * N];
    for x in cells.iter_mut() {
        if next() < 20 {
            *x = b'#';
        }
    }
    let (s, e) = (0, N * N - 1);
    cells[s] = b'A';
    cells[e] = b'B';
    Maze { n: N, m: N, cells, start: s, end: e }
}
fn comb_maze() -> Maze {
    let mut cells = vec![b'.'; N * N];
    for r in (1..N).step_by(2) {
        for c in 0..N {
            cells[r * N + c] = b'#';
        }
        let gap = if (r / 2) % 2 == 0 { N - 1 } else { 0 };
        cells[r * N + gap] = b'.';
    }
    let s = 0;
    let e = (N - 1) * N;
    cells[s] = b'A';
    cells[e] = b'B';
    Maze { n: N, m: N, cells, start: s, end: e }
}
fn no_path() -> Maze {
    let mut cells = vec![b'.'; N * N];
    for r in 0..N {
        cells[r * N + N / 2] = b'#';
    }
    let (s, e) = (0, N * N - 1);
    cells[s] = b'A';
    cells[e] = b'B';
    Maze { n: N, m: N, cells, start: s, end: e }
}

fn benches(c: &mut Criterion) {
    let cases: Vec<(&str, Maze)> = vec![
        ("open_corners", open_corners()),
        ("open_strip", open_strip()),
        ("walls_20", walls_20()),
        ("comb_maze", comb_maze()),
        ("no_path", no_path()),
    ];

    // Expansion-count summary (the "why"), printed once before timing.
    eprintln!("\n--- cells expanded (the reason behind the timings) ---");
    eprintln!("{:>14} {:>12} {:>12}", "map", "BFS", "A*");
    for (name, mz) in &cases {
        let be = bfs::solve(mz).expanded;
        let ae = astar::solve(mz).expanded;
        eprintln!("{:>14} {:>12} {:>12}", name, be, ae);
    }
    eprintln!();

    let mut group = c.benchmark_group("labyrinth");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    for (name, mz) in &cases {
        group.bench_with_input(BenchmarkId::new("bfs", name), mz, |b, mz| {
            b.iter(|| black_box(bfs::solve(black_box(mz))));
        });
        group.bench_with_input(BenchmarkId::new("astar", name), mz, |b, mz| {
            b.iter(|| black_box(astar::solve(black_box(mz))));
        });
    }
    group.finish();
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
