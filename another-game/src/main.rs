//! Binary entry point for the "Another Game" deliverable.
//!
//! Reads t test cases from stdin and prints "first"/"second" for each. The
//! algorithm is chosen by an optional argument (default: parity):
//!
//!   cargo run --release -- parity  < input.txt
//!   cargo run --release -- heap    < input.txt
//!
//! Both algorithms are imported through the library crate (lib.rs) and give the
//! same answers; `heap` routes the decision through a binary-heap data structure.

use std::io::{self, Read, Write};
use another_game::{heap, parity, Winner};

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "parity".to_string());
    let solver: fn(&[u32]) -> Winner = match which.as_str() {
        "parity" => parity::solve,
        "heap" => heap::solve,
        other => {
            eprintln!("unknown algorithm '{other}', use 'parity' or 'heap'");
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|tok| tok.parse::<u32>().unwrap());

    let t = it.next().unwrap();
    let mut out: Vec<u8> = Vec::with_capacity(t as usize * 7);
    let mut heaps: Vec<u32> = Vec::new();

    for _ in 0..t {
        let n = it.next().unwrap() as usize;
        heaps.clear();
        heaps.extend((0..n).map(|_| it.next().unwrap()));
        out.extend_from_slice(solver(&heaps).as_str().as_bytes());
        out.push(b'\n');
    }
    io::stdout().write_all(&out).unwrap();
}
