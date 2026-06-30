// CSES 1193 "Labyrinth" — Algorithm 1 (BFS, uninformed). Self-contained.
// FIFO queue explores in waves; first wave to reach B gives the shortest path.
use std::io::{self, Read, Write};
use std::collections::VecDeque;

const DIRS: [(isize, isize, u8); 4] = [(-1, 0, b'U'), (1, 0, b'D'), (0, -1, b'L'), (0, 1, b'R')];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut hd = lines.next().unwrap().split_ascii_whitespace();
    let n: usize = hd.next().unwrap().parse().unwrap();
    let m: usize = hd.next().unwrap().parse().unwrap();
    let mut cells = vec![b'#'; n * m];
    let (mut start, mut end) = (0usize, 0usize);
    for r in 0..n {
        let row = lines.next().unwrap().as_bytes();
        for c in 0..m {
            cells[r * m + c] = row[c];
            if row[c] == b'A' { start = r * m + c; }
            else if row[c] == b'B' { end = r * m + c; }
        }
    }

    let size = n * m;
    let mut visited = vec![false; size];
    let mut parent = vec![u32::MAX; size];
    let mut came = vec![0u8; size];
    let mut q = VecDeque::new();
    visited[start] = true;
    q.push_back(start);
    let mut found = start == end;
    while let Some(cur) = q.pop_front() {
        if cur == end { found = true; break; }
        let (r, c) = (cur / m, cur % m);
        for &(dr, dc, ch) in &DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= n as isize || nc >= m as isize { continue; }
            let ni = nr as usize * m + nc as usize;
            if visited[ni] || cells[ni] == b'#' { continue; }
            visited[ni] = true;
            parent[ni] = cur as u32;
            came[ni] = ch;
            q.push_back(ni);
        }
    }

    let mut out: Vec<u8> = Vec::new();
    if found {
        let mut path = Vec::new();
        let mut cur = end;
        while cur != start { path.push(came[cur]); cur = parent[cur] as usize; }
        path.reverse();
        out.extend_from_slice(b"YES\n");
        out.extend_from_slice(path.len().to_string().as_bytes());
        out.push(b'\n');
        out.extend_from_slice(&path);
        out.push(b'\n');
    } else {
        out.extend_from_slice(b"NO\n");
    }
    io::stdout().write_all(&out).unwrap();
}
