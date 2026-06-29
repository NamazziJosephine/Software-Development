//! Binary entry point for the "Traffic Lights" deliverable.
//!
//! Reads `x`, `n`, and the `n` light positions from stdin, then prints the
//! longest gap after each insertion (space-separated). The algorithm is chosen
//! by an optional argument (default: bst):
//!
//!   cargo run --release -- bst      < input.txt
//!   cargo run --release -- offline  < input.txt
//!
//! Both algorithms are imported through the library crate (lib.rs).

use std::io::{self, Read, Write};
use traffic_lights::{bst, offline};

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "bst".to_string());
    let solver: fn(u32, &[u32]) -> Vec<u32> = match which.as_str() {
        "bst" => bst::max_gaps,
        "offline" => offline::max_gaps,
        other => {
            eprintln!("unknown algorithm '{other}', use 'bst' or 'offline'");
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());

    let x = it.next().unwrap();
    let n = it.next().unwrap() as usize;
    let positions: Vec<u32> = (0..n).map(|_| it.next().unwrap()).collect();

    let ans = solver(x, &positions);

    // Space-separated, single buffered write.
    let mut out: Vec<u8> = Vec::with_capacity(ans.len() * 11 + 1);
    for (i, &v) in ans.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        push_u32(&mut out, v);
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}

fn push_u32(out: &mut Vec<u8>, mut v: u32) {
    if v == 0 {
        out.push(b'0');
        return;
    }
    let mut tmp = [0u8; 10];
    let mut i = tmp.len();
    while v > 0 {
        i -= 1;
        tmp[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    out.extend_from_slice(&tmp[i..]);
}
