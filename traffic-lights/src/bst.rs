//! Algorithm 1 — online balanced search tree.
//!
//! Rust's `BTreeSet` / `BTreeMap` are B-trees: balanced search trees with high
//! fan-out. We hold the light positions in an ordered set and the gap lengths in
//! an ordered multiset (`BTreeMap<length, count>`), so the longest gap is always
//! the largest key.
//!
//! Per insertion at position `p`:
//!   1. predecessor = nearest light < p, successor = nearest light > p  (two
//!      O(log n) range queries into the tree);
//!   2. the gap `[predecessor, successor]` is destroyed and replaced by
//!      `p - predecessor` and `successor - p` (multiset updates, O(log n));
//!   3. the current answer is the largest key in the gap multiset (O(1) /
//!      O(log n)).
//!
//! This is ONLINE: each query is answered the moment its light arrives, before
//! the next light is known.

use std::collections::{BTreeMap, BTreeSet};

pub fn max_gaps(x: u32, positions: &[u32]) -> Vec<u32> {
    // Light positions, seeded with the two permanent boundaries 0 and x.
    let mut lights: BTreeSet<u32> = BTreeSet::new();
    lights.insert(0);
    lights.insert(x);

    // Multiset of current gap lengths: length -> how many gaps have it.
    let mut gaps: BTreeMap<u32, u32> = BTreeMap::new();
    gaps.insert(x, 1); // one gap spanning the whole street

    let mut ans = Vec::with_capacity(positions.len());

    for &p in positions {
        // Neighbours of p (p is not in the set yet, positions are distinct).
        let left = *lights.range(..p).next_back().unwrap();
        let right = *lights.range(p..).next().unwrap();

        // Destroy the old gap, create the two new ones.
        let old = right - left;
        remove_gap(&mut gaps, old);
        add_gap(&mut gaps, p - left);
        add_gap(&mut gaps, right - p);

        lights.insert(p);

        // Largest key = current longest gap.
        let longest = *gaps.last_key_value().unwrap().0;
        ans.push(longest);
    }
    ans
}

#[inline]
fn add_gap(gaps: &mut BTreeMap<u32, u32>, len: u32) {
    *gaps.entry(len).or_insert(0) += 1;
}

#[inline]
fn remove_gap(gaps: &mut BTreeMap<u32, u32>, len: u32) {
    if let Some(count) = gaps.get_mut(&len) {
        *count -= 1;
        if *count == 0 {
            gaps.remove(&len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(max_gaps(8, &[3, 6, 2]), vec![5, 3, 3]);
    }
}
