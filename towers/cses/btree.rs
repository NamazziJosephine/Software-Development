// CSES 1073 "Towers" — Algorithm 1 (BTreeMap multiset, the B-tree solution).
// SELF-CONTAINED single file for the CSES judge.
// Greedy: place each cube on the tower whose top is the smallest value strictly
// greater than it (successor query on a multiset of tops); else new tower.
use std::io::{self, Read, Write};
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let n = it.next().unwrap() as usize;

    let mut tops: BTreeMap<u32, u32> = BTreeMap::new();
    let mut towers = 0u32;
    for _ in 0..n {
        let s = it.next().unwrap();
        let succ = tops.range((Excluded(s), Unbounded)).next().map(|(&k, _)| k);
        match succ {
            Some(k) => {
                if let Some(c) = tops.get_mut(&k) {
                    *c -= 1;
                    if *c == 0 { tops.remove(&k); }
                }
                *tops.entry(s).or_insert(0) += 1;
            }
            None => {
                *tops.entry(s).or_insert(0) += 1;
                towers += 1;
            }
        }
    }
    let mut out = towers.to_string();
    out.push('\n');
    io::stdout().write_all(out.as_bytes()).unwrap();
}
