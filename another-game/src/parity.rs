//! Algorithm 1 — parity scan (the natural O(n) solution).
//!
//! First player wins iff at least one heap is odd. A single pass with an
//! early exit on the first odd heap: O(n) time, O(1) space.

use crate::Winner;

pub fn solve(heaps: &[u32]) -> Winner {
    if heaps.iter().any(|&x| x % 2 == 1) {
        Winner::First
    } else {
        Winner::Second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_examples() {
        assert_eq!(solve(&[1, 2, 3]), Winner::First);
        assert_eq!(solve(&[2, 2]), Winner::Second);
        assert_eq!(solve(&[5, 5, 4, 5]), Winner::First);
    }
}
