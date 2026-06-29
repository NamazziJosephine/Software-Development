//! CSES 1073 "Towers" — library crate.
//!
//! Cubes arrive one at a time. A cube may be placed on top of a tower only if
//! the cube currently on top is STRICTLY larger; otherwise it must start a new
//! tower. We minimise the number of towers.
//!
//! Optimal greedy: place each cube `s` on the tower whose top is the SMALLEST
//! value strictly greater than `s` (leaving larger tops free for future larger
//! cubes); if no such tower exists, start a new one. This is exactly *patience
//! sorting*, and the number of towers equals the length of the longest
//! non-decreasing subsequence of the cube sequence.
//!
//! Two algorithms, same signature `pub fn min_towers(cubes: &[u32]) -> u32`:
//!
//!   * `btree`    — the B-tree solution: a `BTreeMap<top, count>` multiset of
//!                  tower tops; each cube does a successor query + erase/insert.
//!   * `patience` — the array contrast: a single ascending `Vec` of tower tops;
//!                  each cube does a binary search + in-place overwrite (append
//!                  for a new tower). Same O(n log n), one contiguous array.

pub mod btree;
pub mod patience;
