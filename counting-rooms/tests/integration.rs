//! Integration tests for all three algorithms (`cargo test`).
//!
//! Oracle 1: an independent recursive flood fill (different implementation)
//! on small random grids. Oracle 2: structured grids with known room counts.
//! Plus worst-case shapes at the 1000x1000 limit, where all three must agree.

use counting_rooms::{bfs, dfs, union_find, Grid};

/// Independent reference for small grids: recursive flood fill over a boolean
/// floor mask. Safe because tests only call it for small n*m.
fn reference(n: usize, m: usize, floor: &[bool]) -> u32 {
    fn fill(r: usize, c: usize, n: usize, m: usize, floor: &[bool], seen: &mut [bool]) {
        let i = r * m + c;
        if !floor[i] || seen[i] {
            return;
        }
        seen[i] = true;
        if r > 0 { fill(r - 1, c, n, m, floor, seen); }
        if r + 1 < n { fill(r + 1, c, n, m, floor, seen); }
        if c > 0 { fill(r, c - 1, n, m, floor, seen); }
        if c + 1 < m { fill(r, c + 1, n, m, floor, seen); }
    }
    let mut seen = vec![false; n * m];
    let mut rooms = 0;
    for r in 0..n {
        for c in 0..m {
            if floor[r * m + c] && !seen[r * m + c] {
                rooms += 1;
                fill(r, c, n, m, floor, &mut seen);
            }
        }
    }
    rooms
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn all_three(g: &Grid) -> (u32, u32, u32) {
    (bfs::count_rooms(g), dfs::count_rooms(g), union_find::count_rooms(g))
}

#[test]
fn matches_example() {
    let rows: [&[u8]; 5] = [b"########", b"#..#...#", b"####.#.#", b"#..#...#", b"########"];
    let g = Grid::from_rows(&rows);
    assert_eq!(all_three(&g), (3, 3, 3));
}

#[test]
fn random_small_grids_match_reference() {
    let mut rng = Lcg(0xabcd_1234_5678_9999);
    for _ in 0..300 {
        let n = 1 + rng.next(20) as usize;
        let m = 1 + rng.next(20) as usize;
        // ~45% walls, varied density.
        let floor: Vec<bool> = (0..n * m).map(|_| rng.next(100) >= 45).collect();
        let g = Grid::new(n, m, floor.clone());
        let want = reference(n, m, &floor);
        let (b, d, u) = all_three(&g);
        assert_eq!(b, want, "bfs n={n} m={m}");
        assert_eq!(d, want, "dfs n={n} m={m}");
        assert_eq!(u, want, "uf  n={n} m={m}");
    }
}

#[test]
fn structured_known_counts() {
    // all walls -> 0 rooms
    let g = Grid::new(50, 50, vec![false; 2500]);
    assert_eq!(all_three(&g), (0, 0, 0));

    // all floor -> 1 room
    let g = Grid::new(50, 50, vec![true; 2500]);
    assert_eq!(all_three(&g), (1, 1, 1));

    // checkerboard: floor iff (r+c) even -> no two floors adjacent -> rooms =
    // number of floor cells = ceil(n*m / 2)
    let (n, m) = (51usize, 51);
    let floor: Vec<bool> = (0..n * m).map(|i| ((i / m) + (i % m)) % 2 == 0).collect();
    let expected = floor.iter().filter(|&&f| f).count() as u32;
    let g = Grid::new(n, m, floor);
    assert_eq!(all_three(&g), (expected, expected, expected));

    // vertical stripes: even columns floor, odd columns wall -> each floor
    // column is one room -> rooms = number of even columns
    let (n, m) = (40usize, 41);
    let floor: Vec<bool> = (0..n * m).map(|i| (i % m) % 2 == 0).collect();
    let expected = ((m + 1) / 2) as u32;
    let g = Grid::new(n, m, floor);
    assert_eq!(all_three(&g), (expected, expected, expected));
}

#[test]
fn one_giant_room_at_limit() {
    // 1000x1000 all floor: a single component of 10^6 cells (worst case for
    // the traversal frontier and for union-find).
    let (n, m) = (1000usize, 1000);
    let g = Grid::new(n, m, vec![true; n * m]);
    assert_eq!(all_three(&g), (1, 1, 1));
}

#[test]
fn max_components_at_limit() {
    // 1000x1000 checkerboard: ~500k isolated single-cell rooms.
    let (n, m) = (1000usize, 1000);
    let floor: Vec<bool> = (0..n * m).map(|i| ((i / m) + (i % m)) % 2 == 0).collect();
    let expected = floor.iter().filter(|&&f| f).count() as u32;
    let g = Grid::new(n, m, floor);
    assert_eq!(all_three(&g), (expected, expected, expected));
}
