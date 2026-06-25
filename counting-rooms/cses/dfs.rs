// CSES 1192 "Counting Rooms" — Algorithm 2 (DFS flood fill, explicit stack).
// SELF-CONTAINED single file for the CSES judge (flattened src/dfs.rs + I/O).
// Iterative on purpose: one open room can be 10^6 cells, too deep for recursion.
use std::io::{self, Read, Write};

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
    let mut st: Vec<usize> = Vec::new();
    let mut rooms = 0u32;
    for s in 0..n * m {
        if !floor[s] || visited[s] { continue; }
        rooms += 1; visited[s] = true; st.push(s);
        while let Some(cell) = st.pop() {
            let (r, c) = (cell / m, cell % m);
            let mut tp = |nr: usize, nc: usize, s: &mut Vec<usize>| {
                let ni = nr * m + nc;
                if floor[ni] && !visited[ni] { visited[ni] = true; s.push(ni); }
            };
            if r > 0 { tp(r - 1, c, &mut st); }
            if r + 1 < n { tp(r + 1, c, &mut st); }
            if c > 0 { tp(r, c - 1, &mut st); }
            if c + 1 < m { tp(r, c + 1, &mut st); }
        }
    }
    let mut out = Vec::new();
    out.extend_from_slice(rooms.to_string().as_bytes());
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
