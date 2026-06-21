//! CSES "Increasing Array" — in-place greedy, O(1) extra space.
//!
//! Each move adds 1 to one element. We want the fewest moves to make the array
//! non-decreasing (every element >= the one before it).
//!
//! Idea: sweep left to right keeping `prev` = the largest value seen so far.
//! Any element below `prev` must be raised up to `prev`; that costs the gap.
//! We never store the array — just one running value — so extra space is O(1).
//!
//! Self-contained (no library) so this whole file can be pasted into CSES.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    // The answer can reach ~2e14 (n up to 2e5, values up to 1e9), so it MUST be
    // i64 — an i32 would overflow and fail CSES.
    let mut prev: i64 = it.next().unwrap().parse().unwrap(); // first element
    let mut moves: i64 = 0;

    for _ in 1..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if x < prev {
            moves += prev - x; // raise x up to prev
        } else {
            prev = x; // x is the new running maximum
        }
    }

    println!("{}", moves);
}
