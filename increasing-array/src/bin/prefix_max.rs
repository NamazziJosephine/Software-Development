//! CSES "Increasing Array" — prefix-maximum auxiliary array, O(n) extra space.
//!
//! Same idea as the in-place version, but instead of one running variable we
//! build a SECOND dynamic array `pmax`, where pmax[i] = max(a[0..=i]). The
//! answer is then the sum of (pmax[i] - a[i]) over all i.
//!
//! This is the "dynamic arrays" version: it allocates, fills, and re-reads an
//! auxiliary Vec, so it touches twice as much memory as the in-place version.
//!
//! Self-contained (no library) so this whole file can be pasted into CSES.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    // Read the input into a dynamic array (Vec).
    let a: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    // Auxiliary dynamic array of running maxima: pmax[i] = max(a[0..=i]).
    let mut pmax: Vec<i64> = Vec::with_capacity(n);
    let mut running: i64 = i64::MIN;
    for &x in &a {
        running = running.max(x);
        pmax.push(running);
    }

    // Every element must rise to the max seen so far; sum the gaps in i64
    // (the total can reach ~2e14, which overflows i32).
    let mut moves: i64 = 0;
    for i in 0..n {
        moves += pmax[i] - a[i];
    }

    println!("{}", moves);
}
