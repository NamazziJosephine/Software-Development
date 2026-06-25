//! Algorithm 1 — BFS flood fill (breadth-first traversal).
//!
//! Scan every cell. Each time we meet an unvisited floor cell, that is a new
//! room: we breadth-first flood the whole component (marking it visited) so it
//! is counted once. BFS uses a FIFO QUEUE, so it processes cells in expanding
//! "rings" outward from the start.

use crate::Grid;
use std::collections::VecDeque;

pub fn count_rooms(g: &Grid) -> u32 {
    let (n, m) = (g.n, g.m);
    let mut visited = vec![false; n * m];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut rooms = 0u32;

    for start in 0..n * m {
        if !g.floor[start] || visited[start] {
            continue;
        }
        // New room: flood its whole component.
        rooms += 1;
        visited[start] = true;
        queue.push_back(start);

        while let Some(cell) = queue.pop_front() {
            let (r, c) = (cell / m, cell % m);
            // Visit the four orthogonal neighbours.
            let mut try_push = |nr: usize, nc: usize, q: &mut VecDeque<usize>| {
                let ni = nr * m + nc;
                if g.floor[ni] && !visited[ni] {
                    visited[ni] = true; // mark on enqueue to avoid duplicates
                    q.push_back(ni);
                }
            };
            if r > 0 {
                try_push(r - 1, c, &mut queue);
            }
            if r + 1 < n {
                try_push(r + 1, c, &mut queue);
            }
            if c > 0 {
                try_push(r, c - 1, &mut queue);
            }
            if c + 1 < m {
                try_push(r, c + 1, &mut queue);
            }
        }
    }
    rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        let rows: [&[u8]; 5] = [
            b"########",
            b"#..#...#",
            b"####.#.#",
            b"#..#...#",
            b"########",
        ];
        assert_eq!(count_rooms(&Grid::from_rows(&rows)), 3);
    }
}
