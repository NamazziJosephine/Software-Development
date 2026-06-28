//! Integration tests for both algorithms (`cargo test`).
//!
//! Oracle: the naive O(n^2) definition — length L is a border iff the prefix of
//! length L equals the suffix of length L. Used on small strings. Then the
//! all-equal worst case and large random strings, where the two algorithms must
//! agree.

use finding_borders::{kmp, zalgo};

/// Independent reference straight from the definition (small n only).
fn reference(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    let mut v = Vec::new();
    for l in 1..n {
        if s[..l] == s[n - l..] {
            v.push(l as u32);
        }
    }
    v
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, modulo: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.0 >> 33) as u32) % modulo
    }
}

#[test]
fn matches_example() {
    assert_eq!(kmp::border_lengths(b"abcababcab"), vec![2, 5]);
    assert_eq!(zalgo::border_lengths(b"abcababcab"), vec![2, 5]);
}

#[test]
fn edge_cases() {
    // single char: no proper border
    assert_eq!(kmp::border_lengths(b"a"), Vec::<u32>::new());
    assert_eq!(zalgo::border_lengths(b"a"), Vec::<u32>::new());
    // "aa": border of length 1
    assert_eq!(kmp::border_lengths(b"aa"), vec![1]);
    assert_eq!(zalgo::border_lengths(b"aa"), vec![1]);
    // "ab": no border
    assert_eq!(kmp::border_lengths(b"ab"), Vec::<u32>::new());
    assert_eq!(zalgo::border_lengths(b"ab"), Vec::<u32>::new());
}

#[test]
fn random_small_match_reference() {
    let mut rng = Lcg(0x51ed_270b_3e7a_1144);
    for _ in 0..400 {
        let n = 1 + rng.next(40) as usize;
        // small alphabet (a..c) so borders actually occur often
        let s: Vec<u8> = (0..n).map(|_| b'a' + rng.next(3) as u8).collect();
        let want = reference(&s);
        assert_eq!(kmp::border_lengths(&s), want, "kmp on {:?}", String::from_utf8_lossy(&s));
        assert_eq!(zalgo::border_lengths(&s), want, "zalgo on {:?}", String::from_utf8_lossy(&s));
    }
}

#[test]
fn all_equal_worst_case() {
    // "aaaa...a": every length 1..n-1 is a border.
    let n = 1_000_000;
    let s = vec![b'a'; n];
    let expected: Vec<u32> = (1..n as u32).collect();
    assert_eq!(kmp::border_lengths(&s), expected);
    assert_eq!(zalgo::border_lengths(&s), expected);
}

#[test]
fn large_random_algorithms_agree() {
    let mut rng = Lcg(0x9e37_79b9_7f4a_7c15);
    for &alpha in &[2u32, 4, 26] {
        let n = 1_000_000;
        let s: Vec<u8> = (0..n).map(|_| b'a' + rng.next(alpha) as u8).collect();
        assert_eq!(
            kmp::border_lengths(&s),
            zalgo::border_lengths(&s),
            "disagreement at alphabet size {alpha}"
        );
    }
}

#[test]
fn nested_borders() {
    // A string with several nested borders; check both against the reference.
    let s = b"aabaaaabaaaab";
    let want = reference(s);
    assert_eq!(kmp::border_lengths(s), want);
    assert_eq!(zalgo::border_lengths(s), want);
}
