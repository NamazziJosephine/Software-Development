// CSES 1674 "Subordinates" — Algorithm 2 (iterative DFS, explicit stack).
//
// SELF-CONTAINED single file for pasting into the CSES editor. It flattens
// src/iterative.rs + the main.rs I/O into one file with no library import.
//
// All traversal state lives in heap vectors, so there is no recursion-depth
// limit and no special stack size: even a 2*10^5-deep chain is fine.

use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut parent = vec![0u32; n + 1];
    let mut children: Vec<Vec<u32>> = vec![Vec::new(); n + 1];
    for emp in 2..=n {
        let b: u32 = it.next().unwrap().parse().unwrap();
        parent[emp] = b;
        children[b as usize].push(emp as u32);
    }

    // pass 1: explicit-stack DFS -> parent-before-children order
    let mut order: Vec<u32> = Vec::with_capacity(n);
    let mut stack: Vec<u32> = Vec::with_capacity(n);
    if n >= 1 {
        stack.push(1);
    }
    while let Some(v) = stack.pop() {
        order.push(v);
        for &c in &children[v as usize] {
            stack.push(c);
        }
    }

    // pass 2: reverse accumulation of subtree sizes
    let mut size = vec![1u32; n + 1];
    let mut counts = vec![0u32; n + 1];
    for &v in order.iter().rev() {
        counts[v as usize] = size[v as usize] - 1;
        let p = parent[v as usize];
        if p != 0 {
            size[p as usize] += size[v as usize];
        }
    }

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
