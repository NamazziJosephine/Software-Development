//! CSES 2208 "Another Game" — library crate.
//!
//! Game: n heaps of coins; each move a player removes one coin from ANY chosen
//! subset of the nonempty heaps; whoever takes the last coin wins.
//!
//! The result hinges on one observation: **the first player wins iff at least
//! one heap holds an odd number of coins.** If every heap is even, the second
//! player mirrors each move (removing one from the exact same subset), restoring
//! "all even" after every round, so the first player can never take the last
//! coin. If some heap is odd, the first player moves to "all even" and hands that
//! losing position to the opponent. So the answer is determined entirely by
//! whether any heap count is odd.
//!
//! Two algorithms compute that, each in its own module, with the same signature
//! `pub fn solve(heaps: &[u32]) -> Winner`:
//!
//!   * `parity` — a single O(n) scan: is any heap odd?  (the natural solution)
//!   * `heap`   — the same decision routed through a binary heap data structure
//!                (`BinaryHeap`): heapify the counts, then pop them, stopping at
//!                the first odd one. Correct, but O(n log n) and deliberately the
//!                "wrong tool" — the benchmark measures exactly what that extra
//!                data structure costs.
//!
//! (Note on the topic: this problem sits under "Heaps". The problem's heaps are
//! *coin heaps*, not the data structure; the `heap` algorithm uses an actual
//! binary-heap data structure so the deliverable exercises one, while being
//! honest that it is not the better choice here.)

pub mod heap;
pub mod parity;

/// Who wins with optimal play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    First,
    Second,
}

impl Winner {
    /// The exact string CSES expects.
    pub fn as_str(self) -> &'static str {
        match self {
            Winner::First => "first",
            Winner::Second => "second",
        }
    }
}
