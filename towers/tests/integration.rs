//! Integration tests (`cargo test`).
//!
//! Oracle: a literal O(n^2) simulation of the greedy — for each cube, linearly
//! scan the current tower tops for the smallest one strictly greater than the
//! cube, replace it (or start a new tower). Both fast algorithms must match it
//! on small inputs, then agree with each other on large and worst-case inputs.

use towers::{btree, patience};

/// Brute force: scan all tops each time. Small n only.
fn reference(cubes: &[u32]) -> u32 {
    let mut tops: Vec<u32> = Vec::new();
    for &s in cubes {
        // smallest top strictly greater than s
        let mut best: Option<usize> = None;
        for (idx, &t) in tops.iter().enumerate() {
            if t > s && best.map_or(true, |b| t < tops[b]) {
                best = Some(idx);
            }
        }
        match best {
            Some(idx) => tops[idx] = s,
            None => tops.push(s),
        }
    }
    tops.len() as u32
}

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

#[test]
fn matches_example() {
    assert_eq!(btree::min_towers(&[2, 3, 1]), 2);
    assert_eq!(patience::min_towers(&[2, 3, 1]), 2);
}

#[test]
fn small_random_match_reference() {
    let mut rng = Lcg(0x243f_6a88_85a3_08d3);
    for _ in 0..400 {
        let n = 1 + rng.next_u32(40) as usize;
        // small value range so duplicates and ties occur often
        let cubes: Vec<u32> = (0..n).map(|_| 1 + rng.next_u32(15)).collect();
        let want = reference(&cubes);
        assert_eq!(btree::min_towers(&cubes), want, "btree {cubes:?}");
        assert_eq!(patience::min_towers(&cubes), want, "patience {cubes:?}");
    }
}

#[test]
fn edge_cases() {
    assert_eq!(btree::min_towers(&[5]), 1); // single cube
    assert_eq!(patience::min_towers(&[5]), 1);

    // strictly increasing: nothing can stack -> n towers
    assert_eq!(btree::min_towers(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(patience::min_towers(&[1, 2, 3, 4, 5]), 5);

    // strictly decreasing: everything stacks -> 1 tower
    assert_eq!(btree::min_towers(&[5, 4, 3, 2, 1]), 1);
    assert_eq!(patience::min_towers(&[5, 4, 3, 2, 1]), 1);

    // all equal: cannot stack equal on equal -> n towers
    assert_eq!(btree::min_towers(&[7, 7, 7, 7]), 4);
    assert_eq!(patience::min_towers(&[7, 7, 7, 7]), 4);
}

#[test]
fn large_random_algorithms_agree() {
    let mut rng = Lcg(0xb7e1_5162_8aed_2a6b);
    for &range in &[1_000_000_000u32, 1000, 5] {
        let n = 200_000usize;
        let cubes: Vec<u32> = (0..n).map(|_| 1 + rng.next_u32(range)).collect();
        assert_eq!(
            btree::min_towers(&cubes),
            patience::min_towers(&cubes),
            "disagreement at range={range}"
        );
    }
}

#[test]
fn worst_cases_at_limit() {
    let n = 200_000u32;
    let inc: Vec<u32> = (1..=n).collect(); // all increasing -> n towers
    assert_eq!(btree::min_towers(&inc), n);
    assert_eq!(patience::min_towers(&inc), n);

    let dec: Vec<u32> = (1..=n).rev().collect(); // all decreasing -> 1 tower
    assert_eq!(btree::min_towers(&dec), 1);
    assert_eq!(patience::min_towers(&dec), 1);
}
