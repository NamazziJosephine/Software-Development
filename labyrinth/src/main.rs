//! Binary entry point for the "Labyrinth" deliverable.
//!
//! Reads the labyrinth from stdin and prints the CSES answer: "YES", the
//! shortest path length, and one shortest path as L/R/U/D — or "NO". The
//! algorithm is chosen by an optional argument (default: bfs):
//!
//!   cargo run --release -- bfs    < input.txt
//!   cargo run --release -- astar  < input.txt

use std::io::{self, Read, Write};
use labyrinth::{astar, bfs, Maze, SearchResult};

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "bfs".to_string());
    let solver: fn(&Maze) -> SearchResult = match which.as_str() {
        "bfs" => bfs::solve,
        "astar" => astar::solve,
        other => {
            eprintln!("unknown algorithm '{other}', use 'bfs' or 'astar'");
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze = Maze::parse(&input);

    let result = solver(&maze);
    let mut out: Vec<u8> = Vec::new();
    match result.path {
        Some(path) => {
            out.extend_from_slice(b"YES\n");
            out.extend_from_slice(path.len().to_string().as_bytes());
            out.push(b'\n');
            out.extend_from_slice(&path);
            out.push(b'\n');
        }
        None => out.extend_from_slice(b"NO\n"),
    }
    io::stdout().write_all(&out).unwrap();
}
