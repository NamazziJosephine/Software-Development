//! Binary entry point for the "Counting Rooms" deliverable.
//!
//! Reads the map from stdin and prints the number of rooms. The algorithm is
//! chosen by an optional argument (default: dfs):
//!
//!   cargo run --release -- bfs        < input.txt
//!   cargo run --release -- dfs        < input.txt
//!   cargo run --release -- union_find < input.txt
//!
//! All three algorithms are imported through the library crate (lib.rs).

use std::io::{self, Read, Write};
use counting_rooms::{bfs, dfs, union_find, Grid};

fn main() {
    // ---- pick the algorithm from the first CLI argument ----
    let which = std::env::args().nth(1).unwrap_or_else(|| "dfs".to_string());
    let solver: fn(&Grid) -> u32 = match which.as_str() {
        "bfs" => bfs::count_rooms,
        "dfs" => dfs::count_rooms,
        "union_find" | "uf" => union_find::count_rooms,
        other => {
            eprintln!("unknown algorithm '{other}', use 'bfs', 'dfs', or 'union_find'");
            std::process::exit(1);
        }
    };

    // ---- read all input ----
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // first line: n and m
    let mut header = lines.next().unwrap().split_ascii_whitespace();
    let n: usize = header.next().unwrap().parse().unwrap();
    let m: usize = header.next().unwrap().parse().unwrap();

    // next n lines: the map. Floor iff the byte is '.'.
    let mut floor = vec![false; n * m];
    for r in 0..n {
        let row = lines.next().unwrap_or("").as_bytes();
        for c in 0..m {
            floor[r * m + c] = row.get(c) == Some(&b'.');
        }
    }

    let grid = Grid::new(n, m, floor);
    let rooms = solver(&grid);

    let mut out = Vec::with_capacity(16);
    out.extend_from_slice(rooms.to_string().as_bytes());
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
