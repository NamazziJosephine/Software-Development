//! Algorithm 2 — closed-form modular (block) construction.
//!
//! Exposes a single pure function `partition` (no I/O) so it can be reused by
//! `main.rs`, the benchmark, and the tests through `lib.rs`.

/// Split {1, 2, ..., n} into two equal-sum sets using a closed-form block rule.
///
/// Returns `Some((set_a, set_b))` when a balanced split exists, else `None`.
///
/// Idea: feasibility is decided in O(1) from n % 4 (possible iff n % 4 is 0 or
/// 3). The build then follows a fixed positional pattern instead of a running
/// budget: each quadruple (k, k+1, k+2, k+3) splits as {k, k+3} -> A and
/// {k+1, k+2} -> B. Both halves sum to 2k+3, so every block is internally
/// balanced. When n % 4 == 3 the leading 1, 2, 3 are placed by hand
/// ({1,2} -> A, {3} -> B) and the quadruples start at 4.
///
/// The branch (n % 4) is decided once before the loop; the hot loop is
/// branchless and each block is independent, so there is no loop-carried
/// data dependency.
pub fn partition(n: u32) -> Option<(Vec<u32>, Vec<u32>)> {
    // Same feasibility maths as the greedy version, written via n % 4.
    if !(n % 4 == 0 || n % 4 == 3) {
        return None;
    }

    let cap = (n as usize / 2) + 2;
    let mut a = Vec::with_capacity(cap);
    let mut b = Vec::with_capacity(cap);

    let mut k = if n % 4 == 0 {
        1
    } else {
        // n % 4 == 3: the leading 1, 2, 3 do not form a quadruple.
        a.push(1);
        a.push(2);
        b.push(3);
        4
    };

    while k <= n {
        a.push(k);
        a.push(k + 3);
        b.push(k + 1);
        b.push(k + 2);
        k += 4;
    }
    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_sample_answer() {
        // For n = 7 this construction reproduces the problem statement's
        // example: A = {1,2,4,7}, B = {3,5,6}.
        let (a, b) = partition(7).unwrap();
        assert_eq!(a, vec![1, 2, 4, 7]);
        assert_eq!(b, vec![3, 5, 6]);
        assert!(partition(6).is_none());
    }
}
