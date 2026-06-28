//! CSES 1732 "Finding Borders" — library crate.
//!
//! A *border* of a string is a prefix that is also a suffix, but not the whole
//! string. The key structural fact: **borders nest**. The longest border of the
//! whole string, then the longest border of that border, and so on, enumerates
//! every border. Both algorithms below compute, in O(n), the "longest matching
//! prefix" information that exposes this, then read the border lengths out.
//!
//! Each algorithm is in its own module and exposes the same signature:
//!   pub fn border_lengths(s: &[u8]) -> Vec<u32>
//! returning all border lengths in INCREASING order.
//!
//!   * `kmp`   — KMP prefix (failure) function, then follow the border chain
//!   * `zalgo` — Z-algorithm, then test each suffix against the prefix

pub mod kmp;
pub mod zalgo;
