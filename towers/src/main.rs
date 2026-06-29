//! Binary entry point for the "Towers" deliverable.
//!
//! Reads `n` and the `n` cube sizes from stdin, prints the minimum number of
//! towers. The algorithm is chosen by an optional argument (default: btree):
//!
//!   cargo run --release -- btree     < input.txt
//!   cargo run --release -- patience  < input.txt
//!
//! Both algorithms are imported through the library crate (lib.rs).

use std::io::{self, Read, Write};
use towers::{btree, patience};

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "btree".to_string());
    let solver: fn(&[u32]) -> u32 = match which.as_str() {
        "btree" => btree::min_towers,
        "patience" => patience::min_towers,
        other => {
            eprintln!("unknown algorithm '{other}', use 'btree' or 'patience'");
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());

    let n = it.next().unwrap() as usize;
    let cubes: Vec<u32> = (0..n).map(|_| it.next().unwrap()).collect();

    let ans = solver(&cubes);

    let mut out = String::new();
    out.push_str(&ans.to_string());
    out.push('\n');
    io::stdout().write_all(out.as_bytes()).unwrap();
}
