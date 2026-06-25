//! Integration tests for both algorithms, run with `cargo test`.
//!
//! Strategy: compare both algorithms against an independent O(n^2) reference on
//! many random trees, then stress the two worst-case shapes (deep chain, wide
//! star) at the constraint limit, and check order-independence.

use subordinates::{iterative, recursive};

/// Independent reference: employee v's subordinates = number of employees u
/// whose ancestor chain passes through v. Walks parent pointers up to the root.
/// O(n^2), only used for small n in tests.
fn reference(n: usize, boss: &[u32]) -> Vec<u32> {
    let mut parent = vec![0u32; n + 1];
    for emp in 2..=n {
        parent[emp] = boss[emp - 2];
    }
    let mut counts = vec![0u32; n + 1];
    for u in 1..=n {
        let mut cur = parent[u];
        while cur != 0 {
            counts[cur as usize] += 1;
            cur = parent[cur as usize];
        }
    }
    counts
}

/// Tiny deterministic PRNG (LCG) so tests are reproducible without a crate.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

#[test]
fn matches_example() {
    let expected = vec![4, 1, 1, 0, 0];
    assert_eq!(&recursive::count_subordinates(5, &[1, 1, 2, 3])[1..=5], &expected[..]);
    assert_eq!(&iterative::count_subordinates(5, &[1, 1, 2, 3])[1..=5], &expected[..]);
}

#[test]
fn single_employee() {
    // n = 1: just the director, no bosses, zero subordinates.
    assert_eq!(recursive::count_subordinates(1, &[])[1], 0);
    assert_eq!(iterative::count_subordinates(1, &[])[1], 0);
}

#[test]
fn order_independent_boss_can_outrank_number() {
    // Employee 2's boss is 3, employee 3's boss is 1  ->  tree 1 -> 3 -> 2.
    let boss = [3u32, 1];
    let expected = [2u32, 0, 1]; // employees 1,2,3
    assert_eq!(&recursive::count_subordinates(3, &boss)[1..=3], &expected);
    assert_eq!(&iterative::count_subordinates(3, &boss)[1..=3], &expected);
}

#[test]
fn random_trees_match_reference() {
    let mut rng = Lcg(0x1234_5678_9abc_def0);
    for &n in &[1usize, 2, 3, 5, 10, 50, 200, 500] {
        for _ in 0..20 {
            // Each employee e (2..=n) gets a random boss in 1..e (valid tree).
            let boss: Vec<u32> = (2..=n).map(|e| 1 + rng.next((e - 1) as u32)).collect();
            let want = reference(n, &boss);
            assert_eq!(recursive::count_subordinates(n, &boss), want, "recursive n={n}");
            assert_eq!(iterative::count_subordinates(n, &boss), want, "iterative n={n}");
        }
    }
}

#[test]
fn deep_chain_at_limit() {
    // Worst case for recursion: 1 -> 2 -> 3 -> ... -> n, depth = n.
    let n = 200_000;
    let boss: Vec<u32> = (2..=n as u32).map(|e| e - 1).collect();
    let r = recursive::count_subordinates(n, &boss);
    let it = iterative::count_subordinates(n, &boss);
    // Employee i has exactly n - i subordinates in a chain.
    for i in 1..=n {
        assert_eq!(r[i], (n - i) as u32);
        assert_eq!(it[i], (n - i) as u32);
    }
}

#[test]
fn wide_star_at_limit() {
    // Director with n-1 direct reports, depth 1.
    let n = 200_000;
    let boss: Vec<u32> = vec![1u32; n - 1];
    let r = recursive::count_subordinates(n, &boss);
    let it = iterative::count_subordinates(n, &boss);
    assert_eq!(r[1], (n - 1) as u32);
    assert_eq!(it[1], (n - 1) as u32);
    for i in 2..=n {
        assert_eq!(r[i], 0);
        assert_eq!(it[i], 0);
    }
}
