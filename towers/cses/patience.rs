// CSES 1073 "Towers" — Algorithm 2 (patience sorting on a sorted Vec).
// SELF-CONTAINED single file for the CSES judge.
// Keep tower tops ascending in one Vec; for each cube binary-search the first
// top strictly greater than it: overwrite that slot (place) or append (new tower).
// Final length = number of towers.
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let n = it.next().unwrap() as usize;

    let mut tops: Vec<u32> = Vec::new();
    for _ in 0..n {
        let s = it.next().unwrap();
        let i = tops.partition_point(|&t| t <= s);
        if i < tops.len() {
            tops[i] = s;
        } else {
            tops.push(s);
        }
    }
    let mut out = (tops.len() as u32).to_string();
    out.push('\n');
    io::stdout().write_all(out.as_bytes()).unwrap();
}
