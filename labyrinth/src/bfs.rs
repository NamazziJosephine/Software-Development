//! Algorithm 1 — breadth-first search (uninformed baseline).
//!
//! A plain FIFO queue expands cells in waves of equal distance from `A`. On an
//! unweighted grid the first time a wave reaches `B`, that distance is shortest.
//! BFS has no idea where `B` is, so it explores blindly outward — roughly a
//! growing disk around `A` — until the wave hits `B`. O(n·m) time and space.

use crate::{reconstruct, Maze, SearchResult, DIRS};
use std::collections::VecDeque;

pub fn solve(maze: &Maze) -> SearchResult {
    let size = maze.n * maze.m;
    let mut visited = vec![false; size];
    let mut parent = vec![u32::MAX; size];
    let mut came = vec![0u8; size]; // move used to enter each cell

    let mut queue = VecDeque::new();
    visited[maze.start] = true;
    queue.push_back(maze.start);

    let mut expanded = 0u64;
    let mut found = maze.start == maze.end;

    while let Some(cur) = queue.pop_front() {
        expanded += 1;
        if cur == maze.end {
            found = true;
            break;
        }
        let (r, c) = (cur / maze.m, cur % maze.m);
        for &(dr, dc, ch) in &DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= maze.n as isize || nc >= maze.m as isize {
                continue;
            }
            let nidx = nr as usize * maze.m + nc as usize;
            if visited[nidx] || !maze.passable(nidx) {
                continue;
            }
            visited[nidx] = true;
            parent[nidx] = cur as u32;
            came[nidx] = ch;
            queue.push_back(nidx);
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
