//! CSES 1674 "Subordinates" — library crate.
//!
//! An employee's number of subordinates = the number of descendants below them
//! in the company tree = (size of their subtree) - 1. So both algorithms do the
//! same core work — compute every subtree size — and differ only in HOW they
//! traverse the tree:
//!
//!   * `recursive` — depth-first search using the program's call stack
//!   * `iterative` — depth-first search using an explicit stack on the heap
//!
//! Each algorithm lives in its own module and exposes the same signature:
//!   pub fn count_subordinates(n: usize, boss: &[u32]) -> Vec<u32>
//! returning a 1-indexed vector where result[v] is employee v's subordinate
//! count (index 0 is unused padding so we can index by employee number).
//!
//! The tree-building step below is shared by both, so the benchmark compares
//! traversal strategies rather than two different parsers.

pub mod iterative;
pub mod recursive;

/// Build the company tree from the boss list.
///
/// `boss` has length n-1: `boss[i]` is the direct boss of employee `i + 2`
/// (employees 2..=n; employee 1 is the director and has no boss).
///
/// Returns `(parent, children)`, both 1-indexed (index 0 is unused):
///   * `parent[v]`   = the direct boss of employee v (0 for the director)
///   * `children[v]` = the list of employees who report directly to v
pub fn build_tree(n: usize, boss: &[u32]) -> (Vec<u32>, Vec<Vec<u32>>) {
    let mut parent = vec![0u32; n + 1];
    let mut children: Vec<Vec<u32>> = vec![Vec::new(); n + 1];

    for emp in 2..=n {
        let b = boss[emp - 2]; // direct boss of `emp`
        parent[emp] = b;
        children[b as usize].push(emp as u32);
    }
    (parent, children)
}
