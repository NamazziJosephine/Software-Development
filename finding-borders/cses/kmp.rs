// CSES 1732 "Finding Borders" — Algorithm 1 (KMP prefix/failure function).
// SELF-CONTAINED single file for the CSES judge (flattens src/kmp.rs + I/O).
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

    // prefix function: fail[i] = longest border length of s[0..=i]
    let mut fail = vec![0u32; n];
    for i in 1..n {
        let mut k = fail[i - 1];
        while k > 0 && s[i] != s[k as usize] {
            k = fail[k as usize - 1];
        }
        if s[i] == s[k as usize] {
            k += 1;
        }
        fail[i] = k;
    }

    // follow the nesting chain: longest border, border of that, ...
    let mut borders = Vec::new();
    let mut k = fail[n - 1];
    while k > 0 {
        borders.push(k);
        k = fail[k as usize - 1];
    }
    borders.reverse(); // decreasing -> increasing

    for (i, &b) in borders.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        push_u32(&mut out, b);
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
