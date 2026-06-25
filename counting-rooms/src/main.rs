use std::io::{self, BufRead};

// Visits every floor square connected to (row, col) by calling itself on each neighbour.
// Each call handles one cell, then triggers a new call for every unvisited neighbour —
// this is what makes it recursive.
// WARNING: on very large grids this can crash with a stack overflow (use iterative instead).
fn flood_fill(grid: &mut Vec<Vec<char>>, row: usize, col: usize, n: usize, m: usize) {
    grid[row][col] = '#'; // mark as visited so this cell is never entered again

    // Check all 4 neighbours: up, down, left, right
    // i32 is used so that subtraction can go negative without underflowing
    for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;

        // Skip neighbours that fall outside the grid
        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == '.' {
                flood_fill(grid, nr, nc, n, m); // recurse into this unvisited neighbour
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first = lines.next().unwrap().unwrap();
    let mut it = first.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap(); // rows
    let m: usize = it.next().unwrap().parse().unwrap(); // cols

    let mut grid: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();

    let mut rooms = 0;
    for r in 0..n {
        for c in 0..m {
            // Every unvisited '.' is the entry point of a room we have not seen yet
            if grid[r][c] == '.' {
                rooms += 1;
                flood_fill(&mut grid, r, c, n, m);
            }
        }
    }

    println!("{}", rooms);
}