use std::io::{self, BufRead};

fn flood_fill(grid: &mut Vec<Vec<char>>, row: usize, col: usize, n: usize, m: usize) {
    grid[row][col] = '#';

    // THE RECURSIVE IDEA: no loop, no manual stack.
    // For each unvisited neighbour, we simply call flood_fill again.
    // Each call takes care of one cell, then triggers further calls for its neighbours.
    // The call stack itself acts as the stack — Rust tracks where to return automatically.
    // RISK: a huge room means thousands of nested calls, which can cause a stack overflow.
    for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == '.' {
                flood_fill(grid, nr, nc, n, m); // <-- function calls itself here
            }
        }
    }
    // when all 4 neighbours are walls or visited, this call simply returns
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first = lines.next().unwrap().unwrap();
    let mut it = first.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();

    let mut rooms = 0;
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == '.' {
                rooms += 1;
                flood_fill(&mut grid, r, c, n, m);
            }
        }
    }

    println!("{}", rooms);
}