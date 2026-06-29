// CSES 1163 "Traffic Lights" — Algorithm 1 (online balanced BST).
// SELF-CONTAINED single file for the CSES judge (flattens src/bst.rs + I/O).
// BTreeSet/BTreeMap are B-trees (balanced search trees). Each new light finds
// its neighbours by predecessor/successor queries; a gap multiset gives the max.
use std::io::{self, Read, Write};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let x = it.next().unwrap();
    let n = it.next().unwrap() as usize;

    let mut lights: BTreeSet<u32> = BTreeSet::new();
    lights.insert(0);
    lights.insert(x);
    let mut gaps: BTreeMap<u32, u32> = BTreeMap::new();
    gaps.insert(x, 1);

    let mut out: Vec<u8> = Vec::with_capacity(n * 11 + 1);
    for k in 0..n {
        let p = it.next().unwrap();
        let left = *lights.range(..p).next_back().unwrap();
        let right = *lights.range(p..).next().unwrap();
        let old = right - left;
        if let Some(c) = gaps.get_mut(&old) {
            *c -= 1;
            if *c == 0 { gaps.remove(&old); }
        }
        *gaps.entry(p - left).or_insert(0) += 1;
        *gaps.entry(right - p).or_insert(0) += 1;
        lights.insert(p);
        if k > 0 { out.push(b' '); }
        push_u32(&mut out, *gaps.last_key_value().unwrap().0);
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
