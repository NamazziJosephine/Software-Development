//! Binary entry point for the "Two Sets" deliverable.
//!
//! Reads the integer n from stdin and prints the CSES-format answer. Which
//! algorithm runs is chosen by an optional argument (default: modular):
//!
//!   echo 7 | cargo run --release -- greedy
//!   echo 7 | cargo run --release -- modular
//!
//! The two algorithms are imported through the library crate (lib.rs), which
//! is exactly the structure the brief asks for.

use std::io::{self, Read, Write};
use two_sets::{greedy, modular};

fn main() {
    // ---- pick the algorithm from the first CLI argument ----
    let which = std::env::args().nth(1).unwrap_or_else(|| "modular".to_string());
    let solver: fn(u32) -> Option<(Vec<u32>, Vec<u32>)> = match which.as_str() {
        "greedy" => greedy::partition,
        "modular" => modular::partition,
        other => {
            eprintln!("unknown algorithm '{other}', use 'greedy' or 'modular'");
            std::process::exit(1);
        }
    };

    // ---- read n ----
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().expect("expected a single integer n");

    // ---- solve and print (buffered; one write_all) ----
    let mut out: Vec<u8> = Vec::with_capacity(8 * 1024 * 1024);
    match solver(n) {
        None => out.extend_from_slice(b"NO\n"),
        Some((a, b)) => {
            out.extend_from_slice(b"YES\n");
            write_set(&mut out, &a);
            write_set(&mut out, &b);
        }
    }
    io::stdout().write_all(&out).unwrap();
}

/// Write one set as CSES expects: a count line, then the elements.
fn write_set(out: &mut Vec<u8>, set: &[u32]) {
    push_u32(out, set.len() as u32);
    out.push(b'\n');
    for (idx, &x) in set.iter().enumerate() {
        if idx > 0 {
            out.push(b' ');
        }
        push_u32(out, x);
    }
    out.push(b'\n');
}

/// Manual itoa: append the decimal digits of `x` (fast for ~10^6 numbers).
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
