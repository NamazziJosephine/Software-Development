//! Integration tests (`cargo test`).
//!
//! The oracle is an exhaustive game search by the literal rules (a position is a
//! win iff some move leaves the opponent in a loss). It is exponential, so it is
//! confined to the tests and run only on tiny inputs — but it lets us prove that
//! BOTH deliverable algorithms compute the true game outcome, i.e. that the
//! parity rule they rely on is actually correct. Then the two algorithms must
//! agree with each other on large inputs.

use std::collections::HashMap;
use another_game::{heap, parity, Winner};

/// Exhaustive, memoised game solver — the independent oracle. Exponential.
fn oracle(heaps: &[u32]) -> Winner {
    let mut state: Vec<u32> = heaps.iter().copied().filter(|&x| x > 0).collect();
    state.sort_unstable_by(|a, b| b.cmp(a));
    let mut memo: HashMap<Vec<u32>, bool> = HashMap::new();
    if is_winning(&state, &mut memo) { Winner::First } else { Winner::Second }
}

fn is_winning(state: &[u32], memo: &mut HashMap<Vec<u32>, bool>) -> bool {
    if state.is_empty() {
        return false; // no coins: player to move has lost
    }
    if let Some(&c) = memo.get(state) {
        return c;
    }
    let k = state.len();
    let mut win = false;
    for mask in 1u64..(1u64 << k) {
        let mut child = Vec::with_capacity(k);
        for (i, &h) in state.iter().enumerate() {
            let v = if mask & (1 << i) != 0 { h - 1 } else { h };
            if v > 0 {
                child.push(v);
            }
        }
        child.sort_unstable_by(|a, b| b.cmp(a));
        if !is_winning(&child, memo) {
            win = true;
            break;
        }
    }
    memo.insert(state.to_vec(), win);
    win
}

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

fn for_each_config(n: usize, maxv: u32, f: &mut impl FnMut(&[u32])) {
    let mut cur = vec![1u32; n];
    loop {
        f(&cur);
        let mut i = 0;
        loop {
            if i == n {
                return;
            }
            if cur[i] < maxv {
                cur[i] += 1;
                break;
            } else {
                cur[i] = 1;
                i += 1;
            }
        }
    }
}

#[test]
fn matches_examples() {
    for algo in [parity::solve, heap::solve] {
        assert_eq!(algo(&[1, 2, 3]).as_str(), "first");
        assert_eq!(algo(&[2, 2]).as_str(), "second");
        assert_eq!(algo(&[5, 5, 4, 5]).as_str(), "first");
    }
}

#[test]
fn both_match_oracle_exhaustively() {
    // Every configuration up to 5 heaps with values 1..=4, plus up to 3 heaps
    // with values 1..=7. The exponential oracle is the source of truth.
    for n in 1..=5usize {
        for_each_config(n, 4, &mut |heaps| {
            let truth = oracle(heaps);
            assert_eq!(parity::solve(heaps), truth, "parity wrong on {heaps:?}");
            assert_eq!(heap::solve(heaps), truth, "heap wrong on {heaps:?}");
        });
    }
    for n in 1..=3usize {
        for_each_config(n, 7, &mut |heaps| {
            let truth = oracle(heaps);
            assert_eq!(parity::solve(heaps), truth, "parity wrong on {heaps:?}");
            assert_eq!(heap::solve(heaps), truth, "heap wrong on {heaps:?}");
        });
    }
}

#[test]
fn parity_and_heap_agree_on_large_random() {
    let mut rng = Lcg(0xb7e1_5162_8aed_2a6b);
    for _ in 0..50 {
        let n = 1 + rng.next_u32(5000) as usize;
        let heaps: Vec<u32> = (0..n).map(|_| 1 + rng.next_u32(1_000_000_000)).collect();
        assert_eq!(parity::solve(&heaps), heap::solve(&heaps));
    }
}

#[test]
fn edge_and_limit_cases() {
    assert_eq!(parity::solve(&[1]), Winner::First);
    assert_eq!(heap::solve(&[1]), Winner::First);
    assert_eq!(parity::solve(&[1_000_000_000]), Winner::Second); // even
    assert_eq!(heap::solve(&[1_000_000_000]), Winner::Second);

    // all even, maximal size -> second; both must scan/drain everything
    let all_even: Vec<u32> = vec![2; 200_000];
    assert_eq!(parity::solve(&all_even), Winner::Second);
    assert_eq!(heap::solve(&all_even), Winner::Second);
}
