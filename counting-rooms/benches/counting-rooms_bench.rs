use std::time::Instant;

// ---- ITERATIVE flood fill ----
fn flood_fill_iterative(grid: &mut Vec<Vec<char>>, row: usize, col: usize, n: usize, m: usize) {
    grid[row][col] = '#';
    let mut stack = vec![(row, col)];
    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == '.' {
                    grid[nr][nc] = '#';
                    stack.push((nr, nc));
                }
            }
        }
    }
}

// ---- RECURSIVE flood fill ----
// Note: will stack overflow on large grids — we use a smaller grid for this version
fn flood_fill_recursive(grid: &mut Vec<Vec<char>>, row: usize, col: usize, n: usize, m: usize) {
    grid[row][col] = '#';
    for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == '.' {
                flood_fill_recursive(grid, nr, nc, n, m);
            }
        }
    }
}

// Builds a fresh all-floor grid of size n x m
fn make_grid(n: usize, m: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; m]; n]
}

// Runs a flood fill function RUNS times and returns the average duration in milliseconds
fn benchmark<F>(label: &str, mut f: F, n: usize, m: usize, runs: u32)
where
    F: FnMut(&mut Vec<Vec<char>>, usize, usize, usize, usize),
{
    let mut total_ms = 0.0;

    for run in 1..=runs {
        let mut grid = make_grid(n, m); // fresh grid every run so results are fair
        let start = Instant::now();
        f(&mut grid, 0, 0, n, m);
        let elapsed = start.elapsed().as_secs_f64() * 1000.0; // convert to milliseconds
        total_ms += elapsed;
        println!("  {} | run {:>2} | {:.3} ms", label, run, elapsed);
    }

    println!("  {} | AVERAGE over {} runs: {:.3} ms\n", label, runs, total_ms / runs as f64);
}

fn main() {
    // Iterative: safe to run on full 1000x1000
    let (n_iter, m_iter) = (1000, 1000);

    // Recursive: limited to ~200x200 to avoid stack overflow
    // A 1000x1000 all-floor grid would require ~1,000,000 nested calls
    let (n_rec, m_rec) = (200, 200);

    let runs = 10;

    println!("=== Iterative ({}x{}) ===", n_iter, m_iter);
    benchmark("iterative", flood_fill_iterative, n_iter, m_iter, runs);

    println!("=== Recursive ({}x{}) ===", n_rec, m_rec);
    benchmark("recursive", flood_fill_recursive, n_rec, m_rec, runs);

    println!("NOTE: recursive uses a smaller grid ({}x{}) to avoid stack overflow.", n_rec, m_rec);
    println!("For a fair comparison, try iterative on {}x{} too:", n_rec, m_rec);

    println!("\n=== Iterative ({}x{}) for fair comparison ===", n_rec, m_rec);
    benchmark("iterative (small)", flood_fill_iterative, n_rec, m_rec, runs);
}