//! Algorithm 2 — offline reverse pass (union-find on a line).
//!
//! The online tree pays a logarithmic search on every insertion. We can avoid
//! that entirely if we are allowed to see all the input first.
//!
//! Trick: an insertion SPLITS a gap; undoing it MERGES two gaps. So sort all
//! positions once, build the final configuration, then process the insertions
//! in REVERSE. Each step removes the most-recently-added light and merges the
//! two gaps it was separating — an O(1) operation with a doubly linked list
//! over the sorted positions (equivalent to union-find merging adjacent
//! segments on a line). The longest gap only grows as we merge, so a single
//! running maximum gives every answer.
//!
//! Cost is dominated by the one sort (O(n log n)); everything after is linear
//! array work with near-constant per-step cost. This is OFFLINE: it needs the
//! entire input up front and cannot answer mid-stream.

pub fn max_gaps(x: u32, positions: &[u32]) -> Vec<u32> {
    let n = positions.len();
    if n == 0 {
        return Vec::new();
    }

    // Sorted array of all node positions: 0, the n lights, and x.
    let mut sorted: Vec<u32> = Vec::with_capacity(n + 2);
    sorted.push(0);
    sorted.extend_from_slice(positions);
    sorted.push(x);
    sorted.sort_unstable();

    let m = sorted.len(); // n + 2

    // Doubly linked list over the sorted order: left[k]/right[k] are the
    // indices of k's neighbours among the still-present nodes.
    let mut left: Vec<u32> = (0..m as u32).map(|i| i.wrapping_sub(1)).collect();
    let mut right: Vec<u32> = (0..m as u32).map(|i| i + 1).collect();

    // Longest gap of the FULL configuration (= answer after all n insertions).
    let mut longest = 0u32;
    for k in 0..m - 1 {
        longest = longest.max(sorted[k + 1] - sorted[k]);
    }

    let mut ans = vec![0u32; n];
    ans[n - 1] = longest;

    // Remove lights in reverse insertion order. Removing positions[i] yields the
    // state of the first i lights, whose answer is ans[i-1].
    for i in (1..n).rev() {
        let idx = sorted.binary_search(&positions[i]).unwrap() as u32;
        let l = left[idx as usize];
        let r = right[idx as usize];

        // The two gaps around idx merge into one spanning [l, r].
        let merged = sorted[r as usize] - sorted[l as usize];
        if merged > longest {
            longest = merged;
        }

        // Unlink idx.
        right[l as usize] = r;
        left[r as usize] = l;

        ans[i - 1] = longest;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(max_gaps(8, &[3, 6, 2]), vec![5, 3, 3]);
    }
}
