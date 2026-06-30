//! Algorithm 2 — A* with the Manhattan-distance heuristic (informed search).
//!
//! A* is BFS/Dijkstra guided by a heuristic. Instead of a FIFO queue it uses a
//! priority queue ordered by `f = g + h`, where `g` is the steps taken so far
//! and `h` is the Manhattan distance to `B`. Because Manhattan distance never
//! overestimates the true remaining distance on a 4-directional unit grid
//! (it is *admissible*), A* still returns a shortest path; because it is also
//! *consistent*, the first time `B` is popped is optimal. The heuristic pulls
//! the frontier toward `B`, so on open maps A* expands far fewer cells than BFS
//! — at the cost of a binary-heap push/pop (O(log)) per step instead of BFS's
//! O(1) queue operations.

use crate::{reconstruct, Maze, SearchResult, DIRS};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve(maze: &Maze) -> SearchResult {
    let size = maze.n * maze.m;
    let mut g = vec![u32::MAX; size]; // best known steps from start
    let mut parent = vec![u32::MAX; size];
    let mut came = vec![0u8; size];

    // Min-heap on (f, idx) via Reverse.
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = BinaryHeap::new();
    g[maze.start] = 0;
    heap.push(Reverse((maze.manhattan(maze.start), maze.start as u32)));

    let mut expanded = 0u64;
    let mut found = false;

    while let Some(Reverse((f, cur_u))) = heap.pop() {
        let cur = cur_u as usize;
        // Skip stale heap entries (a better path to `cur` was found after this
        // entry was pushed).
        if f > g[cur] + maze.manhattan(cur) {
            continue;
        }
        expanded += 1;
        if cur == maze.end {
            found = true;
            break;
        }
        let (r, c) = (cur / maze.m, cur % maze.m);
        let ng = g[cur] + 1;
        for &(dr, dc, ch) in &DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= maze.n as isize || nc >= maze.m as isize {
                continue;
            }
            let nidx = nr as usize * maze.m + nc as usize;
            if !maze.passable(nidx) || ng >= g[nidx] {
                continue;
            }
            g[nidx] = ng;
            parent[nidx] = cur as u32;
            came[nidx] = ch;
            heap.push(Reverse((ng + maze.manhattan(nidx), nidx as u32)));
        }
    }

    let path = if found {
        Some(reconstruct(maze.start, maze.end, &parent, &came))
    } else {
        None
    };
    SearchResult { path, expanded }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        let input = "5 8\n########\n#.A#...#\n#.##.#B#\n#......#\n########\n";
        let maze = Maze::parse(input);
        let r = solve(&maze);
        assert_eq!(r.length(), Some(9));
    }
}
