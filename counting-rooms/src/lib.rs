//! CSES 1192 "Counting Rooms" — library crate.
//!
//! A "room" is a connected component of floor squares ('.'), where two floor
//! squares are connected if they are horizontally or vertically adjacent. So
//! the task is: count the connected components of the grid graph whose nodes
//! are floor cells. Three classic graph approaches are provided, each in its
//! own module and each exposing the same signature:
//!
//!   pub fn count_rooms(grid: &Grid) -> u32
//!
//!   * `bfs`        — flood fill with a FIFO queue   (breadth-first traversal)
//!   * `dfs`        — flood fill with a LIFO stack   (depth-first traversal)
//!   * `union_find` — disjoint-set union of adjacent floor cells (no traversal)
//!
//! The grid is parsed once into a flat floor mask shared by all three, so the
//! benchmark compares the algorithms, not three different parsers.

pub mod bfs;
pub mod dfs;
pub mod union_find;

/// The building map as a flat, row-major floor mask.
pub struct Grid {
    pub n: usize,            // height (rows)
    pub m: usize,            // width (columns)
    pub floor: Vec<bool>,    // length n*m; true = floor ('.'), false = wall ('#')
}

impl Grid {
    /// Build a grid from a pre-computed floor mask of length n*m.
    pub fn new(n: usize, m: usize, floor: Vec<bool>) -> Self {
        debug_assert_eq!(floor.len(), n * m);
        Grid { n, m, floor }
    }

    /// Build a grid from `n` rows of bytes; a cell is floor iff its byte is '.'.
    pub fn from_rows(rows: &[&[u8]]) -> Self {
        let n = rows.len();
        let m = if n > 0 { rows[0].len() } else { 0 };
        let mut floor = vec![false; n * m];
        for (r, row) in rows.iter().enumerate() {
            for c in 0..m {
                floor[r * m + c] = row.get(c) == Some(&b'.');
            }
        }
        Grid { n, m, floor }
    }

    /// Flat index of cell (r, c) in the row-major mask.
    #[inline]
    pub fn idx(&self, r: usize, c: usize) -> usize {
        r * self.m + c
    }
}
