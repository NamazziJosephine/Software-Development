//! CSES 1193 "Labyrinth" — library crate.
//!
//! Single start `A`, single end `B`, 4-directional moves on an n×m grid (up to
//! 1000×1000). Every move costs 1, so this is an unweighted shortest-path
//! problem — the setting where an UNINFORMED search (BFS) and an INFORMED,
//! heuristic search (A*) can be compared directly.
//!
//! Two algorithms, same signature `pub fn solve(maze: &Maze) -> SearchResult`:
//!   * `bfs`   — breadth-first search: a plain FIFO queue, explores blindly in
//!               all directions (a growing disk around `A`).
//!   * `astar` — A* with the Manhattan-distance heuristic: a priority queue
//!               ordered by f = g + h, so the frontier is pulled toward `B`.
//!
//! Both return the same shortest-path LENGTH (the path string may differ; any
//! shortest path is accepted) and the number of cells EXPANDED, which is how we
//! show *why* the heuristic helps rather than only timing it.

pub mod astar;
pub mod bfs;

/// Parsed labyrinth. Cells are stored row-major; index = row * m + col.
pub struct Maze {
    pub n: usize,
    pub m: usize,
    pub cells: Vec<u8>, // raw bytes: b'.', b'#', b'A', b'B'
    pub start: usize,
    pub end: usize,
}

/// The four moves, as (drow, dcol, output-char). Index order: U, D, L, R.
pub const DIRS: [(isize, isize, u8); 4] =
    [(-1, 0, b'U'), (1, 0, b'D'), (0, -1, b'L'), (0, 1, b'R')];

/// Result of a search: the path (None if unreachable) and the number of cells
/// expanded (popped and processed). `length` is the path length.
pub struct SearchResult {
    pub path: Option<Vec<u8>>,
    pub expanded: u64,
}

impl SearchResult {
    pub fn length(&self) -> Option<usize> {
        self.path.as_ref().map(|p| p.len())
    }
}

impl Maze {
    /// Parse the CSES input format: first line `n m`, then n grid lines.
    pub fn parse(input: &str) -> Maze {
        let mut lines = input.lines();
        let header = lines.next().unwrap();
        let mut hd = header.split_ascii_whitespace();
        let n: usize = hd.next().unwrap().parse().unwrap();
        let m: usize = hd.next().unwrap().parse().unwrap();

        let mut cells = vec![b'#'; n * m];
        let (mut start, mut end) = (0usize, 0usize);
        for r in 0..n {
            let row = lines.next().unwrap().as_bytes();
            for c in 0..m {
                let ch = row[c];
                let idx = r * m + c;
                cells[idx] = ch;
                if ch == b'A' {
                    start = idx;
                } else if ch == b'B' {
                    end = idx;
                }
            }
        }
        Maze { n, m, cells, start, end }
    }

    /// A cell can be entered iff it is not a wall.
    #[inline]
    pub fn passable(&self, idx: usize) -> bool {
        self.cells[idx] != b'#'
    }

    /// Manhattan distance from `idx` to the end (the A* heuristic). Admissible
    /// and consistent for 4-directional unit moves, so A* stays optimal.
    #[inline]
    pub fn manhattan(&self, idx: usize) -> u32 {
        let (r, c) = (idx / self.m, idx % self.m);
        let (er, ec) = (self.end / self.m, self.end % self.m);
        (r.abs_diff(er) + c.abs_diff(ec)) as u32
    }
}

/// Walk parent pointers from `end` back to `start`, collecting the move chars,
/// then reverse to get the path in forward order.
pub fn reconstruct(start: usize, end: usize, parent: &[u32], came: &[u8]) -> Vec<u8> {
    let mut path = Vec::new();
    let mut cur = end;
    while cur != start {
        path.push(came[cur]);
        cur = parent[cur] as usize;
    }
    path.reverse();
    path
}
