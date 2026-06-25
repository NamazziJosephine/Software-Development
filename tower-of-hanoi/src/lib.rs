//! Tower of Hanoi — two algorithms that produce the unique optimal
//! sequence of `2^n - 1` moves.
//!
//! Pegs are numbered 1, 2, 3 (left, middle, right) to match the CSES output
//! format. Each move is the pair `(from, to)`.
//!
//! Both functions return the full move list so the benchmark measures the
//! algorithm itself (control flow + the shared allocation), independent of
//! stdout formatting and I/O.

/// A single move: take the top disk from peg `.0` and place it on peg `.1`.
pub type Move = (u8, u8);

/// Algorithm 1 — classic recursive divide-and-conquer.
///
/// Move `n-1` disks aside, move the largest disk to the target, move the
/// `n-1` back on top. The recursion depth equals `n`, so the call stack is
/// the data structure doing the work.
pub fn recursive_moves(n: u32) -> Vec<Move> {
    // Pre-size to exactly 2^n - 1 so the Vec never reallocates mid-run;
    // keeps the benchmark measuring the algorithm, not allocator growth.
    let mut moves = Vec::with_capacity((1usize << n).saturating_sub(1));
    rec(n, 1, 3, 2, &mut moves);
    moves
}

fn rec(n: u32, from: u8, to: u8, via: u8, moves: &mut Vec<Move>) {
    if n == 0 {
        return;
    }
    rec(n - 1, from, via, to, moves);
    moves.push((from, to));
    rec(n - 1, via, to, from, moves);
}

/// Algorithm 2 — iterative parity rule, no recursion.
///
/// Facts used:
///   * The smallest disk moves on every odd-numbered step, always cycling in
///     one fixed direction (which direction depends only on the parity of n).
///   * On every even-numbered step there is exactly one legal move that does
///     not involve the smallest disk, so we just make it.
/// We keep the three pegs as explicit stacks only so the even step can read
/// the two relevant top disks; there is no call stack at all.
pub fn iterative_moves(n: u32) -> Vec<Move> {
    let total: u64 = (1u64 << n) - 1;
    let mut moves = Vec::with_capacity(total as usize);

    // Build the start state: all disks on peg 0 (largest at the bottom).
    let mut pegs: [Vec<u32>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for d in (1..=n).rev() {
        pegs[0].push(d);
    }

    // Cyclic direction of the smallest disk, encoded as next_small[current].
    // n even -> 0->1->2->0 (left->middle->right): [1,2,0]
    // n odd  -> 0->2->1->0 (left->right->middle): [2,0,1]
    let next_small: [usize; 3] = if n % 2 == 0 { [1, 2, 0] } else { [2, 0, 1] };

    let mut small = 0usize; // peg index currently holding the smallest disk

    for step in 1..=total {
        if step % 2 == 1 {
            // Odd step: advance the smallest disk along its cycle.
            let to = next_small[small];
            let disk = pegs[small].pop().unwrap();
            pegs[to].push(disk);
            moves.push(((small + 1) as u8, (to + 1) as u8));
            small = to;
        } else {
            // Even step: the only legal move between the two non-smallest pegs.
            let (a, b) = match small {
                0 => (1, 2),
                1 => (0, 2),
                _ => (0, 1),
            };
            let (from, to) = match (pegs[a].last(), pegs[b].last()) {
                // Both non-empty: smaller top disk moves onto the larger.
                (Some(&x), Some(&y)) => if x < y { (a, b) } else { (b, a) },
                (Some(_), None) => (a, b),
                (None, Some(_)) => (b, a),
                (None, None) => unreachable!("an even step always has a legal move"),
            };
            let disk = pegs[from].pop().unwrap();
            pegs[to].push(disk);
            moves.push(((from + 1) as u8, (to + 1) as u8));
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(recursive_moves(2), vec![(1, 2), (1, 3), (2, 3)]);
        assert_eq!(iterative_moves(2), vec![(1, 2), (1, 3), (2, 3)]);
    }

    #[test]
    fn both_agree_and_count_is_optimal() {
        for n in 1..=16 {
            let r = recursive_moves(n);
            let i = iterative_moves(n);
            assert_eq!(r, i, "algorithms disagree at n={n}");
            assert_eq!(r.len() as u64, (1u64 << n) - 1, "wrong move count at n={n}");
        }
    }

    #[test]
    fn moves_are_legal() {
        // Replay every move on real peg stacks and assert no larger-on-smaller.
        for n in 1..=14 {
            for moves in [recursive_moves(n), iterative_moves(n)] {
                let mut pegs: [Vec<u32>; 3] = [Vec::new(), Vec::new(), Vec::new()];
                for d in (1..=n).rev() {
                    pegs[0].push(d);
                }
                for (from, to) in moves {
                    let disk = pegs[(from - 1) as usize].pop().expect("source empty");
                    if let Some(&top) = pegs[(to - 1) as usize].last() {
                        assert!(disk < top, "placed {disk} on {top} at n={n}");
                    }
                    pegs[(to - 1) as usize].push(disk);
                }
                // Everything ended on the right peg, in order.
                assert_eq!(pegs[2], (1..=n).rev().collect::<Vec<_>>());
            }
        }
    }
}
