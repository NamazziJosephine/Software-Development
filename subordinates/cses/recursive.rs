// CSES 1674 "Subordinates" — Algorithm 1 (recursive DFS).
//
// SELF-CONTAINED single file for pasting into the CSES editor. It flattens
// src/recursive.rs + the main.rs I/O into one file with no library import.
//
// The DFS runs on a thread with a 256 MB stack: a company that is one long
// chain (employee i reports to i-1) has depth up to 2*10^5, which overflows
// the default 8 MB stack. This enlarged stack is recursion's memory cost made
// explicit.

use std::io::{self, Read, Write};

const STACK_SIZE: usize = 256 * 1024 * 1024;

fn dfs(v: usize, children: &[Vec<u32>], counts: &mut [u32]) -> u32 {
    let mut size = 1;
    for &c in &children[v] {
        size += dfs(c as usize, children, counts);
    }
    counts[v] = size - 1;
    size
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut children: Vec<Vec<u32>> = vec![Vec::new(); n + 1];
    for emp in 2..=n {
        let b: u32 = it.next().unwrap().parse().unwrap();
        children[b as usize].push(emp as u32);
    }

    let mut counts = vec![0u32; n + 1];
    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn_scoped(s, || {
                if n >= 1 {
                    dfs(1, &children, &mut counts);
                }
            })
            .unwrap()
            .join()
            .unwrap();
    });

    let mut out: Vec<u8> = Vec::with_capacity(n * 7 + 16);
    for v in 1..=n {
        if v > 1 {
            out.push(b' ');
        }
        push_u32(&mut out, counts[v]);
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
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
