// CSES 1163 "Traffic Lights" — Algorithm 2 (offline reverse, union-find on a line).
// SELF-CONTAINED single file for the CSES judge (flattens src/offline.rs + I/O).
// Read all lights, then undo insertions in reverse: each undo MERGES two gaps,
// an O(1) doubly-linked-list splice. The longest gap only grows as we merge.
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let x = it.next().unwrap();
    let n = it.next().unwrap() as usize;
    let positions: Vec<u32> = (0..n).map(|_| it.next().unwrap()).collect();

    let mut out: Vec<u8> = Vec::with_capacity(n * 11 + 1);
    if n == 0 {
        out.push(b'\n');
        io::stdout().write_all(&out).unwrap();
        return;
    }

    let mut sorted: Vec<u32> = Vec::with_capacity(n + 2);
    sorted.push(0);
    sorted.extend_from_slice(&positions);
    sorted.push(x);
    sorted.sort_unstable();
    let m = sorted.len();

    let mut left: Vec<u32> = (0..m as u32).map(|i| i.wrapping_sub(1)).collect();
    let mut right: Vec<u32> = (0..m as u32).map(|i| i + 1).collect();

    let mut longest = 0u32;
    for k in 0..m - 1 {
        longest = longest.max(sorted[k + 1] - sorted[k]);
    }
    let mut ans = vec![0u32; n];
    ans[n - 1] = longest;
    for i in (1..n).rev() {
        let idx = sorted.binary_search(&positions[i]).unwrap() as u32;
        let l = left[idx as usize];
        let r = right[idx as usize];
        let merged = sorted[r as usize] - sorted[l as usize];
        if merged > longest { longest = merged; }
        right[l as usize] = r;
        left[r as usize] = l;
        ans[i - 1] = longest;
    }

    for (i, &v) in ans.iter().enumerate() {
        if i > 0 { out.push(b' '); }
        push_u32(&mut out, v);
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}

fn push_u32(out: &mut Vec<u8>, mut v: u32) {
    if v == 0 { out.push(b'0'); return; }
    let mut tmp = [0u8; 10];
    let mut i = tmp.len();
    while v > 0 { i -= 1; tmp[i] = b'0' + (v % 10) as u8; v /= 10; }
    out.extend_from_slice(&tmp[i..]);
}
