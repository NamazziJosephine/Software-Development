//! Algorithm 2 — patience sorting on a sorted `Vec` (the array contrast).
//!
//! Keep the tower tops in a single ascending `Vec`. For each cube `s`, find the
//! first top strictly greater than `s` with a binary search:
//!   * found at index `i` -> overwrite `tops[i] = s` in place (the array stays
//!                           sorted, because tops[i-1] <= s < tops[i+1]);
//!   * not found          -> append `s` (it is >= every top, so it goes at the
//!                           end): a new tower.
//!
//! The final length of the array is the number of towers. `partition_point`
//! returns the count of elements `<= s`, i.e. the index of the first element
//! strictly greater than `s`. Same O(n log n) as the B-tree version, but all
//! work happens in one contiguous array: binary-search probes plus an in-place
//! write, with no per-node allocation.

pub fn min_towers(cubes: &[u32]) -> u32 {
    let mut tops: Vec<u32> = Vec::new();

    for &s in cubes {
        // Index of the first top strictly greater than s.
        let i = tops.partition_point(|&t| t <= s);
        if i < tops.len() {
            tops[i] = s; // place the cube on that tower
        } else {
            tops.push(s); // new tower
        }
    }
    tops.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(min_towers(&[2, 3, 1]), 2);
    }
}
