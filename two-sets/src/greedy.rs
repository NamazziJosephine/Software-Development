//! Algorithm 1 — greedy descending fill.
//!
//! Exposes a single pure function `partition` (no I/O) so it can be reused by
//! `main.rs`, the benchmark, and the tests through `lib.rs`.

/// Split {1, 2, ..., n} into two equal-sum sets using a greedy descending fill.
///
/// Returns `Some((set_a, set_b))` when a balanced split exists, else `None`.
///
/// Idea: the total sum is S = n(n+1)/2; a balanced split needs S even. Start a
/// budget `remaining = S/2` and walk i from n down to 1: if i still fits the
/// budget it joins A (and shrinks the budget), otherwise it joins B. Because
/// {1..n} is a complete sequence, the budget always lands exactly on 0.
///
/// The test `i <= remaining` reads a value that earlier iterations wrote, so
/// this loop carries a data dependency on `remaining`.
pub fn partition(n: u32) -> Option<(Vec<u32>, Vec<u32>)> {
    // S in u64: at n = 10^6 it is ~5*10^11 and overflows u32.
    let total = (n as u64) * (n as u64 + 1) / 2;
    if total % 2 != 0 {
        return None;
    }
    let mut remaining = total / 2;

    // Reserve an upper bound so the algorithm never pays for Vec re-growth.
    let cap = (n as usize / 2) + 2;
    let mut a = Vec::with_capacity(cap);
    let mut b = Vec::with_capacity(cap);

    let mut i = n;
    loop {
        if (i as u64) <= remaining {
            a.push(i);
            remaining -= i as u64;
        } else {
            b.push(i);
        }
        if i == 1 {
            break;
        }
        i -= 1;
    }
    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_examples() {
        // n = 7 is feasible, n = 6 is not.
        let (a, b) = partition(7).unwrap();
        let sa: u32 = a.iter().sum();
        let sb: u32 = b.iter().sum();
        assert_eq!(sa, sb);
        assert!(partition(6).is_none());
    }
}
