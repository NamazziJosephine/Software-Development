//! Algorithm 2 — the same decision via a binary heap data structure.
//!
//! This computes the identical answer (first player wins iff some heap is odd)
//! but funnels the work through a `BinaryHeap` (a max-heap): build the heap from
//! the coin counts, then pop elements off the top one at a time, stopping as
//! soon as an odd count appears. If the heap drains with no odd count, the
//! second player wins.
//!
//! Building the heap is O(n) (heapify) and each pop is O(log n), so this is
//! O(n log n) versus the parity scan's O(n). The heap does no useful work for
//! this problem — the order in which counts come out is irrelevant to a parity
//! test — so the algorithm exists to demonstrate, and let the benchmark
//! quantify, the cost of using a heap data structure where a flat scan suffices.

use crate::Winner;
use std::collections::BinaryHeap;

pub fn solve(heaps: &[u32]) -> Winner {
    // Heapify the counts into a max-heap (O(n)).
    let mut heap: BinaryHeap<u32> = BinaryHeap::from(heaps.to_vec());

    // Pop from the top until an odd count appears (each pop O(log n)).
    while let Some(x) = heap.pop() {
        if x % 2 == 1 {
            return Winner::First;
        }
    }
    Winner::Second
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
