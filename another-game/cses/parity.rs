// CSES 2208 "Another Game" — Algorithm 1 (parity scan). Self-contained.
// First player wins iff at least one heap is odd (else the second player mirrors
// every move and keeps "all even", so the first player can never take the last
// coin). One linear scan per test case.
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let t = it.next().unwrap();
    let mut out: Vec<u8> = Vec::with_capacity(t as usize * 7);
    for _ in 0..t {
        let n = it.next().unwrap();
        let mut any_odd = false;
        for _ in 0..n {
            if it.next().unwrap() % 2 == 1 {
                any_odd = true;
            }
        }
        out.extend_from_slice(if any_odd { b"first\n" } else { b"second\n" });
    }
    io::stdout().write_all(&out).unwrap();
}
