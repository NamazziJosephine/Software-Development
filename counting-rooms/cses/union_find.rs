// CSES 1192 "Counting Rooms" — Algorithm 3 (Union-Find / DSU).
// SELF-CONTAINED single file for the CSES judge (flattened src/union_find.rs + I/O).
use std::io::{self, Read, Write};

struct Dsu { parent: Vec<u32>, size: Vec<u32> }
impl Dsu {
    fn new(len: usize) -> Self { Dsu { parent: (0..len as u32).collect(), size: vec![1u32; len] } }
    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let gp = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = gp; x = gp;
        }
        x
    }
    fn union(&mut self, a: u32, b: u32) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb { return false; }
        if self.size[ra as usize] < self.size[rb as usize] { std::mem::swap(&mut ra, &mut rb); }
        self.parent[rb as usize] = ra; self.size[ra as usize] += self.size[rb as usize]; true
    }
}

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

    let mut dsu = Dsu::new(n * m);
    let mut rooms = 0u32;
    for &f in &floor { if f { rooms += 1; } }
    for r in 0..n {
        for c in 0..m {
            let i = r * m + c;
            if !floor[i] { continue; }
            if c + 1 < m && floor[i + 1] { if dsu.union(i as u32, (i + 1) as u32) { rooms -= 1; } }
            if r + 1 < n && floor[i + m] { if dsu.union(i as u32, (i + m) as u32) { rooms -= 1; } }
        }
    }
    let mut out = Vec::new();
    out.extend_from_slice(rooms.to_string().as_bytes());
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
