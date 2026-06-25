//! Algorithm 3 — Union-Find (disjoint set union).
//!
//! A completely different paradigm: instead of traversing components, treat
//! every floor cell as its own set, then UNION each pair of adjacent floor
//! cells. After processing all adjacencies, the number of remaining disjoint
//! sets is the number of rooms.
//!
//! With path compression (path halving below) and union by size, the total
//! cost is O(n*m * alpha(n*m)), where alpha is the inverse Ackermann function
//! (<= 4 for any feasible input) — near-linear, but technically super-linear,
//! unlike the strictly O(n*m) traversals.

use crate::Grid;

struct Dsu {
    parent: Vec<u32>,
    size: Vec<u32>,
}

impl Dsu {
    fn new(len: usize) -> Self {
        Dsu {
            parent: (0..len as u32).collect(),
            size: vec![1u32; len],
        }
    }

    /// Find the set root of x, halving the path on the way up (path compression).
    #[inline]
    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let gp = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = gp; // point x at its grandparent
            x = gp;
        }
        x
    }

    /// Union the sets of a and b. Returns true if they were different sets.
    #[inline]
    fn union(&mut self, a: u32, b: u32) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        // Attach the smaller tree under the larger (union by size).
        if self.size[ra as usize] < self.size[rb as usize] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb as usize] = ra;
        self.size[ra as usize] += self.size[rb as usize];
        true
    }
}

pub fn count_rooms(g: &Grid) -> u32 {
    let (n, m) = (g.n, g.m);
    let mut dsu = Dsu::new(n * m);

    // Start with one room per floor cell; every successful union merges two.
    let mut rooms = 0u32;
    for &f in &g.floor {
        if f {
            rooms += 1;
        }
    }

    // Union each floor cell with its right and down floor neighbours; together
    // these cover every horizontal and vertical adjacency exactly once.
    for r in 0..n {
        for c in 0..m {
            let i = r * m + c;
            if !g.floor[i] {
                continue;
            }
            if c + 1 < m && g.floor[i + 1] {
                if dsu.union(i as u32, (i + 1) as u32) {
                    rooms -= 1;
                }
            }
            if r + 1 < n && g.floor[i + m] {
                if dsu.union(i as u32, (i + m) as u32) {
                    rooms -= 1;
                }
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
