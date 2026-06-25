//! Algorithm 2 — iterative depth-first search with an explicit stack.
//!
//! Identical result to the recursive version, but the traversal state lives in
//! a `Vec` on the HEAP instead of the call stack. Two passes:
//!
//!   1. Pop-push DFS from the root produces a visit order in which every parent
//!      appears before all of its children (a preorder).
//!   2. Walking that order in REVERSE, every node first records its own count
//!      (size - 1), then adds its subtree size to its parent. Because children
//!      come before parents in reverse, a parent's size is complete by the time
//!      we reach it.
//!
//! No recursion means no call-stack growth and no depth limit: a chain of
//! 2*10^5 nodes is handled with an ordinary heap vector.

use crate::build_tree;

/// Count subordinates for every employee using iterative DFS.
pub fn count_subordinates(n: usize, boss: &[u32]) -> Vec<u32> {
    let (parent, children) = build_tree(n, boss);

    // ---- pass 1: explicit-stack DFS to get a parent-before-children order ----
    let mut order: Vec<u32> = Vec::with_capacity(n);
    let mut stack: Vec<u32> = Vec::with_capacity(n);
    if n >= 1 {
        stack.push(1);
    }
    while let Some(v) = stack.pop() {
        order.push(v);
        for &c in &children[v as usize] {
            stack.push(c);
        }
    }

    // ---- pass 2: reverse accumulation of subtree sizes ----
    let mut size = vec![1u32; n + 1]; // every node counts itself
    let mut counts = vec![0u32; n + 1];
    for &v in order.iter().rev() {
        counts[v as usize] = size[v as usize] - 1;
        let p = parent[v as usize];
        if p != 0 {
            // v is not the director: hand its full subtree size up to the boss
            size[p as usize] += size[v as usize];
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        let counts = count_subordinates(5, &[1, 1, 2, 3]);
        assert_eq!(&counts[1..=5], &[4, 1, 1, 0, 0]);
    }
}
