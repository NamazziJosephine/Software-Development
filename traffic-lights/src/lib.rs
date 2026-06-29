//! CSES 1163 "Traffic Lights" — library crate.
//!
//! A street of length `x`. Lights are added one at a time; after each addition
//! we report the longest gap between adjacent lights (the ends 0 and x always
//! count as boundaries). The street begins as a single gap of length `x`, and
//! every new light **splits** exactly one existing gap into two.
//!
//! Two algorithms are provided, each in its own module, with the same
//! signature `pub fn max_gaps(x: u32, positions: &[u32]) -> Vec<u32>`
//! returning the longest gap after each of the `positions.len()` insertions:
//!
//!   * `bst`     — ONLINE balanced search tree: keep the light positions in an
//!                 ordered set, find each new light's neighbours by predecessor/
//!                 successor queries, and keep a multiset of gap lengths so the
//!                 maximum is always available.
//!   * `offline` — OFFLINE reverse pass: read all insertions first, then undo
//!                 them in reverse. Undoing an insertion MERGES two adjacent
//!                 gaps, so the work becomes a sort plus linear array passes
//!                 (a union-find-on-a-line via a doubly linked list).

pub mod bst;
pub mod offline;
