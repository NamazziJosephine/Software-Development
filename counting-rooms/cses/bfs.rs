// CSES 1192 "Counting Rooms" — Algorithm 1 (BFS flood fill).
// SELF-CONTAINED single file for the CSES judge (flattened src/bfs.rs + I/O).
use std::io::{self, Read, Write};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut h = lines.next().unwrap().split_ascii_whitespace();
    let n: usize = h.next().unwrap().parse().unwrap();
    let m: usize = h.next().unwrap().parse().unwrap();

    let mut floor = vec![false; n * m];
    for r in 0..n {
        let row = lines.next().unwrap_or("").as_bytes();
        for c in 0..m { floor[r * m + c] = row.get(c) == Some(&b'.'); }
    }

    let mut visited = vec![false; n * m];
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut rooms = 0u32;
    for s in 0..n * m {
        if !floor[s] || visited[s] { continue; }
        rooms += 1; visited[s] = true; q.push_back(s);
        while let Some(cell) = q.pop_front() {
            let (r, c) = (cell / m, cell % m);
            let mut tp = |nr: usize, nc: usize, q: &mut VecDeque<usize>| {
                let ni = nr * m + nc;
                if floor[ni] && !visited[ni] { visited[ni] = true; q.push_back(ni); }
            };
            if r > 0 { tp(r - 1, c, &mut q); }
            if r + 1 < n { tp(r + 1, c, &mut q); }
            if c > 0 { tp(r, c - 1, &mut q); }
            if c + 1 < m { tp(r, c + 1, &mut q); }
        }
    }
    let mut out = Vec::new();
    out.extend_from_slice(rooms.to_string().as_bytes());
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
