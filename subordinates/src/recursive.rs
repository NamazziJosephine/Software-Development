//! Algorithm 1 — recursive depth-first search.
//!
//! For each node we recurse into its children, sum their subtree sizes, store
//! `size - 1` as that node's subordinate count, and return the size upward.
//! This is the most direct expression of the idea, but every level of the tree
//! adds one frame to the program's CALL STACK.
//!
//! A valid input can be a single chain (employee i reports to i-1), giving a
//! tree of depth up to n = 2*10^5. Recursing that deep overflows the default
//! 8 MB stack and crashes. To survive it, we run the DFS on a dedicated thread
//! with a large stack. That requirement is exactly the point of this algorithm:
//! recursion's memory lives on the stack, and deep recursion needs a big one.

use crate::build_tree;

/// Stack size for the worker thread. A chain of 2*10^5 frames needs well over
/// the default 8 MB; 256 MB is comfortably safe and stays inside the 512 MB
/// limit (the OS only commits the pages the recursion actually touches).
const STACK_SIZE: usize = 256 * 1024 * 1024;

/// The recursive worker: returns the size of v's subtree (v included) and
/// writes v's subordinate count (size - 1) into `counts`.
fn dfs(v: usize, children: &[Vec<u32>], counts: &mut [u32]) -> u32 {
    let mut size = 1; // count v itself
    for &c in &children[v] {
        size += dfs(c as usize, children, counts);
    }
    counts[v] = size - 1; // everyone in the subtree except v
    size
}

/// Count subordinates for every employee using recursive DFS.
pub fn count_subordinates(n: usize, boss: &[u32]) -> Vec<u32> {
    let (_parent, children) = build_tree(n, boss);
    let mut counts = vec![0u32; n + 1];

    // `scope` lets the spawned thread borrow `children` and `counts` without
    // moving them or wrapping them in Arc; the scope joins the thread before
    // returning, so the borrows are guaranteed valid.
    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn_scoped(s, || {
                if n >= 1 {
                    dfs(1, &children, &mut counts);
                }
            })
            .expect("failed to spawn DFS thread")
            .join()
            .expect("DFS thread panicked");
    });

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        // Example from the problem statement: boss list 1 1 2 3 for n = 5.
        let counts = count_subordinates(5, &[1, 1, 2, 3]);
        assert_eq!(&counts[1..=5], &[4, 1, 1, 0, 0]);
    }
}
