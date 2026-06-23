//! CSES 1092 "Two Sets" — library crate.
//!
//! Each algorithm lives in its own module and is re-exported here so that the
//! binary (`main.rs`), the benchmark (`benches/`), and the tests (`tests/`)
//! all consume the same single implementation:
//!
//!   use two_sets::greedy;
//!   use two_sets::modular;
//!
//! Both modules expose the same signature:
//!   pub fn partition(n: u32) -> Option<(Vec<u32>, Vec<u32>)>

pub mod greedy;
pub mod modular;
