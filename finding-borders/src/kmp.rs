//! Algorithm 1 — KMP prefix (failure) function.
//!
//! `fail[i]` = the length of the longest border of the prefix `s[0..=i]`.
//! It is built left to right: to extend to position `i`, we try to grow the
//! current best border by one character; if `s[i]` does not match, we fall back
//! to the next shorter border via `fail[k-1]` and try again. This fallback is a
//! data-dependent BACKWARD jump along the border chain.
//!
//! Once `fail[n-1]` is the longest border of the whole string, the nesting
//! property gives all the rest: repeatedly take the longest border of the
//! current border (`k -> fail[k-1]`). That yields lengths in DECREASING order,
//! so we reverse at the end to print INCREASING order.

/// Build the prefix function of `s`.
pub fn prefix_function(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    let mut fail = vec![0u32; n];
    for i in 1..n {
        // `k` is the length of the best border we are trying to extend.
        let mut k = fail[i - 1];
        // Fall back along the border chain until the next char matches (or k=0).
        while k > 0 && s[i] != s[k as usize] {
            k = fail[k as usize - 1];
        }
        if s[i] == s[k as usize] {
            k += 1;
        }
        fail[i] = k;
    }
    fail
}

/// All border lengths of `s`, in increasing order.
pub fn border_lengths(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let fail = prefix_function(s);

    // Follow the nesting chain from the whole string's longest border down.
    let mut borders = Vec::new();
    let mut k = fail[n - 1];
    while k > 0 {
        borders.push(k);
        k = fail[k as usize - 1];
    }
    borders.reverse(); // decreasing -> increasing
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
