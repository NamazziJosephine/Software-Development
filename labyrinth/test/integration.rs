//! Integration tests (`cargo test`).
//!
//! Because any shortest path is accepted, we check two things:
//!   1. each algorithm's emitted path is LEGAL (starts at A, only steps onto
//!      floor, ends at B) and has the length the algorithm reported;
//!   2. that length equals an INDEPENDENT flood-fill shortest distance (the
//!      oracle), and BFS and A* agree (both a length, or both "no path").

use std::collections::VecDeque;
use labyrinth::{astar, bfs, Maze};

/// Independent oracle: flood fill from A, return distance to B (or None).
fn oracle_len(maze: &Maze) -> Option<usize> {
    let size = maze.n * maze.m;
    let mut dist = vec![usize::MAX; size];
    let mut q = VecDeque::new();
    dist[maze.start] = 0;
    q.push_back(maze.start);
    while let Some(cur) = q.pop_front() {
        if cur == maze.end {
            return Some(dist[cur]);
        }
        let (r, c) = (cur / maze.m, cur % maze.m);
        for (dr, dc) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= maze.n as isize || nc >= maze.m as isize {
                continue;
            }
            let nidx = nr as usize * maze.m + nc as usize;
            if maze.passable(nidx) && dist[nidx] == usize::MAX {
                dist[nidx] = dist[cur] + 1;
                q.push_back(nidx);
            }
        }
    }
    if dist[maze.end] == usize::MAX { None } else { Some(dist[maze.end]) }
}

/// Validate a path: legal moves only, A -> B; returns its length if valid.
fn validate(maze: &Maze, path: &[u8]) -> Option<usize> {
    let (mut r, mut c) = (maze.start / maze.m, maze.start % maze.m);
    for &mv in path {
        let (dr, dc) = match mv {
            b'U' => (-1isize, 0isize),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => return None,
        };
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr < 0 || nc < 0 || nr >= maze.n as isize || nc >= maze.m as isize {
            return None;
        }
        r = nr as usize;
        c = nc as usize;
        if !maze.passable(r * maze.m + c) {
            return None;
        }
    }
    if r * maze.m + c == maze.end { Some(path.len()) } else { None }
}

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

/// Random maze: each cell a wall with probability `wall_pct`%, A and B placed
/// on two distinct floor cells.
fn random_maze(rng: &mut Lcg, n: usize, m: usize, wall_pct: u32) -> Maze {
    let size = n * m;
    let mut cells = vec![b'.'; size];
    for cell in cells.iter_mut() {
        if rng.next_u32(100) < wall_pct {
            *cell = b'#';
        }
    }
    let a = rng.next_u32(size as u32) as usize;
    let mut b = rng.next_u32(size as u32) as usize;
    while b == a {
        b = rng.next_u32(size as u32) as usize;
    }
    cells[a] = b'A';
    cells[b] = b'B';
    Maze { n, m, cells, start: a, end: b }
}

fn check(maze: &Maze) {
    let want = oracle_len(maze);
    let rb = bfs::solve(maze);
    let ra = astar::solve(maze);
    assert_eq!(rb.length(), want, "bfs length disagrees with oracle");
    assert_eq!(ra.length(), want, "astar length disagrees with oracle");
    if let Some(p) = &rb.path {
        assert_eq!(validate(maze, p), Some(p.len()), "bfs path invalid");
    }
    if let Some(p) = &ra.path {
        assert_eq!(validate(maze, p), Some(p.len()), "astar path invalid");
    }
}

#[test]
fn matches_example() {
    let input = "5 8\n########\n#.A#...#\n#.##.#B#\n#......#\n########\n";
    check(&Maze::parse(input));
}

#[test]
fn random_mazes_various_densities() {
    let mut rng = Lcg(0x243f_6a88_85a3_08d3);
    for _ in 0..300 {
        let n = 1 + rng.next_u32(30) as usize;
        let m = 1 + rng.next_u32(30) as usize;
        let wall_pct = rng.next_u32(60); // 0..60% walls
        check(&random_maze(&mut rng, n, m, wall_pct));
    }
}

#[test]
fn no_path_when_walled_off() {
    // B sealed behind walls.
    let input = "3 3\nA.#\n.##\n##B\n";
    let maze = Maze::parse(input);
    assert_eq!(bfs::solve(&maze).length(), None);
    assert_eq!(astar::solve(&maze).length(), None);
}

#[test]
fn open_map_is_manhattan() {
    // No walls: shortest length equals Manhattan distance A->B.
    let mut cells = vec![b'.'; 10 * 10];
    cells[0] = b'A';
    cells[99] = b'B'; // corner to corner
    let maze = Maze { n: 10, m: 10, cells, start: 0, end: 99 };
    assert_eq!(bfs::solve(&maze).length(), Some(18)); // 9 + 9
    assert_eq!(astar::solve(&maze).length(), Some(18));
}

#[test]
fn large_map_agreement() {
    let mut rng = Lcg(0xb7e1_5162_8aed_2a6b);
    for &wall in &[20u32, 40] {
        let maze = random_maze(&mut rng, 1000, 1000, wall);
        let rb = bfs::solve(&maze);
        let ra = astar::solve(&maze);
        assert_eq!(rb.length(), ra.length(), "bfs/astar disagree at wall%={wall}");
    }
}
