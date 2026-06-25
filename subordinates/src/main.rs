//! Binary entry point for the "Subordinates" deliverable.
//!
//! Reads the input from stdin and prints each employee's subordinate count.
//! The algorithm is chosen by an optional argument (default: iterative):
//!
//!   cargo run --release -- recursive  < input.txt
//!   cargo run --release -- iterative  < input.txt
//!
//! Both algorithms are imported through the library crate (lib.rs).

use std::io::{self, Read, Write};
use subordinates::{iterative, recursive};

fn main() {
    // ---- pick the algorithm from the first CLI argument ----
    let which = std::env::args().nth(1).unwrap_or_else(|| "iterative".to_string());
    let solver: fn(usize, &[u32]) -> Vec<u32> = match which.as_str() {
        "recursive" => recursive::count_subordinates,
        "iterative" => iterative::count_subordinates,
        other => {
            eprintln!("unknown algorithm '{other}', use 'recursive' or 'iterative'");
            std::process::exit(1);
        }
    };

    // ---- read all input and parse integers ----
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let boss: Vec<u32> = (0..n.saturating_sub(1))
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    // ---- solve ----
    let counts = solver(n, &boss);

    // ---- print counts[1..=n], space separated, in one buffered write ----
    let mut out: Vec<u8> = Vec::with_capacity(n * 7 + 16);
    for v in 1..=n {
        if v > 1 {
            out.push(b' ');
        }
        push_u32(&mut out, counts[v]);
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}

/// Manual itoa: append the decimal digits of `x` (fast for ~2*10^5 numbers).
fn push_u32(out: &mut Vec<u8>, mut x: u32) {
    if x == 0 {
        out.push(b'0');
        return;
    }
    let mut tmp = [0u8; 10];
    let mut i = tmp.len();
    while x > 0 {
        i -= 1;
        tmp[i] = b'0' + (x % 10) as u8;
        x /= 10;
    }
    out.extend_from_slice(&tmp[i..]);
}
