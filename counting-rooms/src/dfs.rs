//! Algorithm 2 — DFS flood fill (depth-first traversal).
//!
//! Identical scan to BFS, but the frontier is a LIFO STACK instead of a queue.
//! The stack is an explicit `Vec` on the heap (not recursion): a single open
//! room can contain n*m = 10^6 cells, and recursing that deep would overflow
//! the call stack. With a LIFO order, the next cell processed is one we just
//! pushed — a neighbour, close by in memory — which tends to keep the work
//! inside a small, recently-touched region.

use crate::Grid;

pub fn count_rooms(g: &Grid) -> u32 {
    let (n, m) = (g.n, g.m);
    let mut visited = vec![false; n * m];
    let mut stack: Vec<usize> = Vec::new();
    let mut rooms = 0u32;

    for start in 0..n * m {
        if !g.floor[start] || visited[start] {
            continue;
        }
        rooms += 1;
        visited[start] = true;
        stack.push(start);

        while let Some(cell) = stack.pop() {
            let (r, c) = (cell / m, cell % m);
            let mut try_push = |nr: usize, nc: usize, s: &mut Vec<usize>| {
                let ni = nr * m + nc;
                if g.floor[ni] && !visited[ni] {
                    visited[ni] = true; // mark on push to avoid duplicates
                    s.push(ni);
                }
            };
            if r > 0 {
                try_push(r - 1, c, &mut stack);
            }
            if r + 1 < n {
                try_push(r + 1, c, &mut stack);
            }
            if c > 0 {
                try_push(r, c - 1, &mut stack);
            }
            if c + 1 < m {
                try_push(r, c + 1, &mut stack);
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
