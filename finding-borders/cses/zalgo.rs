// CSES 1732 "Finding Borders" — Algorithm 2 (Z-algorithm).
// SELF-CONTAINED single file for the CSES judge (flattens src/zalgo.rs + I/O).
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim_end().as_bytes();
    let n = s.len();

    let mut out: Vec<u8> = Vec::with_capacity(n * 7 + 1);
    if n == 0 {
        out.push(b'\n');
        io::stdout().write_all(&out).unwrap();
        return;
    }

    // Z-array: z[i] = longest prefix of s that starts at position i
    let mut z = vec![0u32; n];
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min((r - i) as u32);
        }
        while i + (z[i] as usize) < n && s[z[i] as usize] == s[i + z[i] as usize] {
            z[i] += 1;
        }
        if i + (z[i] as usize) > r {
            l = i;
            r = i + z[i] as usize;
        }
    }

    // border of length L exists iff z[n-L] == L
    for ln in 1..n {
        if z[n - ln] as usize == ln {
            if out.len() > 0 {
                out.push(b' ');
            }
            push_u32(&mut out, ln as u32);
        }
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}

fn push_u32(out: &mut Vec<u8>, mut x: u32) {
    if x == 0 { out.push(b'0'); return; }
    let mut tmp = [0u8; 10];
    let mut i = tmp.len();
    while x > 0 { i -= 1; tmp[i] = b'0' + (x % 10) as u8; x /= 10; }
    out.extend_from_slice(&tmp[i..]);
}
