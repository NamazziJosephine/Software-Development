//! Algorithm 1 — `BTreeMap` multiset (the B-tree solution).
//!
//! Maintain the tower tops as a multiset `BTreeMap<top, count>`. For each cube
//! `s`, find the smallest top strictly greater than `s` (a successor query):
//!   * found `k`  -> that tower now has top `s`: erase one `k`, insert one `s`
//!                   (tower count unchanged);
//!   * not found  -> start a new tower: insert one `s` (tower count + 1).
//!
//! The multiset's total size always equals the current number of towers, so the
//! answer is simply how many times we started a new tower. `BTreeMap` is a
//! B-tree, so each successor query is an O(log n) walk down a balanced,
//! high-fan-out tree. Overall O(n log n).

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

pub fn min_towers(cubes: &[u32]) -> u32 {
    let mut tops: BTreeMap<u32, u32> = BTreeMap::new();
    let mut towers = 0u32;

    for &s in cubes {
        // Smallest top strictly greater than s.
        let successor = tops
            .range((Excluded(s), Unbounded))
            .next()
            .map(|(&k, _)| k);

        match successor {
            Some(k) => {
                // This tower's top changes from k to s.
                if let Some(c) = tops.get_mut(&k) {
                    *c -= 1;
                    if *c == 0 {
                        tops.remove(&k);
                    }
                }
                *tops.entry(s).or_insert(0) += 1;
            }
            None => {
                // No suitable tower: start a new one.
                *tops.entry(s).or_insert(0) += 1;
                towers += 1;
            }
        }
    }
    towers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(min_towers(&[2, 3, 1]), 2);
    }
}
