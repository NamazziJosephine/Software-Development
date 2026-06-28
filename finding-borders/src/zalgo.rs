//! Algorithm 2 — Z-algorithm.
//!
//! `z[i]` = the length of the longest substring starting at `i` that matches a
//! prefix of `s`. It is computed with a sliding window `[l, r]` (the "Z-box"),
//! the rightmost prefix-match seen so far: inside the box we reuse an already
//! computed `z` value, and otherwise we extend by a FORWARD scan. The dominant
//! access pattern is therefore sequential over `s` and `z`.
//!
//! Border test: a border of length `L` is a suffix of length `L` (starting at
//! index `n - L`) that equals the prefix of length `L`. Only `L` characters
//! remain from `n - L`, so the match length there is at most `L`; it equals `L`
//! exactly when `z[n - L] == L`. Scanning `L = 1, 2, ...` yields the border
//! lengths already in INCREASING order.

/// Compute the Z-array of `s` (`z[0]` is left as 0; it is never needed here).
pub fn z_array(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    let mut z = vec![0u32; n];
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            // Inside the current Z-box: reuse the mirror value, capped by the box.
            z[i] = z[i - l].min((r - i) as u32);
        }
        // Try to extend the match beyond what we already know (forward scan).
        while i + (z[i] as usize) < n && s[z[i] as usize] == s[i + z[i] as usize] {
            z[i] += 1;
        }
        // If we extended past the old box, slide the box to start at i.
        if i + (z[i] as usize) > r {
            l = i;
            r = i + z[i] as usize;
        }
    }
    z
}

/// All border lengths of `s`, in increasing order.
pub fn border_lengths(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let z = z_array(s);

    let mut borders = Vec::new();
    // L from 1 to n-1 (a border is strictly shorter than the whole string).
    for l in 1..n {
        if z[n - l] as usize == l {
            borders.push(l as u32);
        }
    }
    borders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        assert_eq!(border_lengths(b"abcababcab"), vec![2, 5]);
    }
}
