//! Binary entry point for the "Finding Borders" deliverable.
//!
//! Reads the string from stdin and prints all border lengths in increasing
//! order. The algorithm is chosen by an optional argument (default: zalgo):
//!
//!   cargo run --release -- kmp    < input.txt
//!   cargo run --release -- zalgo  < input.txt
//!
//! Both algorithms are imported through the library crate (lib.rs).

use std::io::{self, Read, Write};
use finding_borders::{kmp, zalgo};

fn main() {
    // ---- pick the algorithm from the first CLI argument ----
    let which = std::env::args().nth(1).unwrap_or_else(|| "zalgo".to_string());
    let solver: fn(&[u8]) -> Vec<u32> = match which.as_str() {
        "kmp" => kmp::border_lengths,
        "zalgo" | "z" => zalgo::border_lengths,
        other => {
            eprintln!("unknown algorithm '{other}', use 'kmp' or 'zalgo'");
            std::process::exit(1);
        }
    };

    // ---- read the string (trim trailing whitespace/newline) ----
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim_end().as_bytes();

    // ---- solve ----
    let borders = solver(s);

    // ---- print the lengths space-separated, one buffered write ----
    let mut out: Vec<u8> = Vec::with_capacity(borders.len() * 7 + 1);
    for (i, &b) in borders.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        push_u32(&mut out, b);
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}

/// Manual itoa: append the decimal digits of `x`.
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
