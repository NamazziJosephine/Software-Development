// CSES 1092 "Two Sets" — Algorithm 1 (greedy descending fill).
//
// SELF-CONTAINED single file for pasting into the CSES editor. It is the
// flattened form of src/greedy.rs + the main.rs I/O: same algorithm, but with
// no library import, because CSES accepts only one standalone source file.

use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let mut out: Vec<u8> = Vec::with_capacity(8 * 1024 * 1024);

    let total = n * (n + 1) / 2;
    if total % 2 != 0 {
        out.extend_from_slice(b"NO\n");
        io::stdout().write_all(&out).unwrap();
        return;
    }

    let mut remaining = total / 2;
    let cap = (n as usize / 2) + 2;
    let mut a: Vec<u32> = Vec::with_capacity(cap);
    let mut b: Vec<u32> = Vec::with_capacity(cap);

    let mut i = n;
    loop {
        if i <= remaining {
            a.push(i as u32);
            remaining -= i;
        } else {
            b.push(i as u32);
        }
        if i == 1 {
            break;
        }
        i -= 1;
    }

    out.extend_from_slice(b"YES\n");
    write_set(&mut out, &a);
    write_set(&mut out, &b);
    io::stdout().write_all(&out).unwrap();
}

fn write_set(out: &mut Vec<u8>, set: &[u32]) {
    push_u32(out, set.len() as u32);
    out.push(b'\n');
    for (idx, &x) in set.iter().enumerate() {
        if idx > 0 {
            out.push(b' ');
        }
        push_u32(out, x);
    }
    out.push(b'\n');
}

fn push_u32(out: &mut Vec<u8>, mut x: u32) {
    if x == 0 {
        out.push(b'0');
        return;
    }
    let mut tmp = [0u8; 10];
    let mut i = tmp.len();
    while x > 0 {
        i -= 1;
        tmp[i] = b'0' + (x % 10) as u8;
        x /= 10;
    }
    out.extend_from_slice(&tmp[i..]);
}
