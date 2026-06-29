//! Integration tests (`cargo test`).
//!
//! Oracle: a brute-force O(n^2) recompute (insert into a sorted Vec, scan all
//! gaps) on small inputs. Then both algorithms must agree with each other on
//! large random inputs and at the constraint limits.

use std::collections::BTreeSet;
use traffic_lights::{bst, offline};

/// Brute force: after each insertion, rescan every gap. Small n only.
fn reference(x: u32, positions: &[u32]) -> Vec<u32> {
    let mut lights = vec![0u32, x];
    let mut ans = Vec::with_capacity(positions.len());
    for &p in positions {
        let idx = lights.binary_search(&p).unwrap_err();
        lights.insert(idx, p);
        let mut mx = 0;
        for w in lights.windows(2) {
            mx = mx.max(w[1] - w[0]);
        }
        ans.push(mx);
    }
    ans
}

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

/// `n` distinct positions in 1..x, in a random insertion order.
fn random_positions(rng: &mut Lcg, x: u32, n: usize) -> Vec<u32> {
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert(1 + rng.next_u32(x - 1));
    }
    let mut v: Vec<u32> = set.into_iter().collect();
    // shuffle into a random insertion order (Fisher-Yates)
    for i in (1..v.len()).rev() {
        let j = rng.next_u32((i + 1) as u32) as usize;
        v.swap(i, j);
    }
    v
}

#[test]
fn matches_example() {
    assert_eq!(bst::max_gaps(8, &[3, 6, 2]), vec![5, 3, 3]);
    assert_eq!(offline::max_gaps(8, &[3, 6, 2]), vec![5, 3, 3]);
}

#[test]
fn small_random_match_reference() {
    let mut rng = Lcg(0x243f_6a88_85a3_08d3);
    for _ in 0..300 {
        let x = 5 + rng.next_u32(60);
        let max_n = (x - 1).min(25) as usize;
        if max_n == 0 {
            continue;
        }
        let n = 1 + rng.next_u32(max_n as u32) as usize;
        let positions = random_positions(&mut rng, x, n);
        let want = reference(x, &positions);
        assert_eq!(bst::max_gaps(x, &positions), want, "bst x={x} pos={positions:?}");
        assert_eq!(offline::max_gaps(x, &positions), want, "offline x={x} pos={positions:?}");
    }
}

#[test]
fn edge_cases() {
    // single light in the middle
    assert_eq!(bst::max_gaps(10, &[5]), vec![5]);
    assert_eq!(offline::max_gaps(10, &[5]), vec![5]);
    // light next to a boundary
    assert_eq!(bst::max_gaps(10, &[1]), vec![9]);
    assert_eq!(offline::max_gaps(10, &[1]), vec![9]);
    // inserting in sorted order
    assert_eq!(bst::max_gaps(10, &[2, 4, 6, 8]), offline::max_gaps(10, &[2, 4, 6, 8]));
}

#[test]
fn large_random_algorithms_agree() {
    let mut rng = Lcg(0xb7e1_5162_8aed_2a6b);
    for &(x, n) in &[(1_000_000_000u32, 200_000usize), (1_000_000, 200_000)] {
        let positions = random_positions(&mut rng, x, n);
        assert_eq!(
            bst::max_gaps(x, &positions),
            offline::max_gaps(x, &positions),
            "disagreement at x={x} n={n}"
        );
    }
}

#[test]
fn sorted_insertions_at_limit() {
    // Lights inserted left to right: each new gap is small, the big gap shrinks.
    let n = 200_000usize;
    let x = 1_000_000_000u32;
    let step = x / (n as u32 + 1);
    let positions: Vec<u32> = (1..=n as u32).map(|i| i * step).collect();
    assert_eq!(bst::max_gaps(x, &positions), offline::max_gaps(x, &positions));
}
