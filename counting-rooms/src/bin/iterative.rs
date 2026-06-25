use std::io::{self, BufRead};

fn flood_fill(grid: &mut Vec<Vec<char>>, row: usize, col: usize, n: usize, m: usize) {
    grid[row][col] = '#';

    // THE ITERATIVE IDEA: we manage our own stack with a Vec.
    // Instead of the function calling itself, we push neighbours onto this list
    // and keep looping until there is nothing left to visit.
    let mut stack = vec![(row, col)];

    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == '.' {
                    grid[nr][nc] = '#'; // mark before pushing so it is never added twice
                    stack.push((nr, nc)); // <-- no function call, just add to the list
                }
            }
        }
    } // loop ends when stack is empty = entire room visited
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