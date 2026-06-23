//! Integration tests for both algorithms, run with `cargo test`.
//!
//! Validity of a split means: every number 1..=n appears exactly once across
//! the two sets, and both sets have the same sum. We check both algorithms
//! exhaustively for small n and on the large boundary values, plus confirm the
//! genuinely impossible cases return None.

use two_sets::{greedy, modular};

type Solver = fn(u32) -> Option<(Vec<u32>, Vec<u32>)>;

fn assert_valid(name: &str, n: u32, parts: Option<(Vec<u32>, Vec<u32>)>) {
    match parts {
        None => assert!(
            n % 4 == 1 || n % 4 == 2,
            "{name}: n={n} reported impossible but a split exists"
        ),
        Some((a, b)) => {
            let sa: u64 = a.iter().map(|&x| x as u64).sum();
            let sb: u64 = b.iter().map(|&x| x as u64).sum();
            assert_eq!(sa, sb, "{name}: n={n} sums differ ({sa} vs {sb})");

            let mut seen = vec![false; n as usize + 1];
            for &x in a.iter().chain(b.iter()) {
                assert!(x >= 1 && x <= n, "{name}: n={n} out-of-range element {x}");
                assert!(!seen[x as usize], "{name}: n={n} duplicate element {x}");
                seen[x as usize] = true;
            }
            for v in 1..=n {
                assert!(seen[v as usize], "{name}: n={n} missing element {v}");
            }
        }
    }
}

fn run_all(name: &str, solver: Solver) {
    // Exhaustive small n.
    for n in 1..=2000u32 {
        assert_valid(name, n, solver(n));
    }
    // Large boundary values (both feasible residues).
    for &n in &[99_999u32, 100_000, 999_996, 999_999, 1_000_000] {
        assert_valid(name, n, solver(n));
    }
    // Genuinely impossible cases must be None.
    for &n in &[1u32, 2, 5, 6, 9, 10, 1_000_001, 1_000_002] {
        assert!(solver(n).is_none(), "{name}: n={n} should be impossible");
    }
}

#[test]
fn greedy_is_correct() {
    run_all("greedy", greedy::partition);
}

#[test]
fn modular_is_correct() {
    run_all("modular", modular::partition);
}

#[test]
fn algorithms_agree_on_feasibility() {
    // The two algorithms must always agree on whether a split exists.
    for n in 1..=5000u32 {
        assert_eq!(
            greedy::partition(n).is_some(),
            modular::partition(n).is_some(),
            "feasibility disagreement at n={n}"
        );
    }
}
