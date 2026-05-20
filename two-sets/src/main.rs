use std::io::{Write, BufWriter};

// Recursively assign numbers from `cur` down to 1 into two sets.
// `rem` tracks how much sum is still needed for set A.
// If the current number fits, it goes to A; otherwise to B.
fn solve(cur: i64, rem: i64, a: &mut Vec<i64>, b: &mut Vec<i64>) {
    if cur == 0 { return; }
    if rem >= cur { a.push(cur); solve(cur - 1, rem - cur, a, b); }
    else          { b.push(cur); solve(cur - 1, rem, a, b); }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    // A split into two equal-sum sets is only possible if the total is even.
    // The total sum 1+2+...+n = n*(n+1)/2, which is even when n % 4 == 0 or n % 4 == 3.
    let total = n * (n + 1) / 2;
    let mut out = BufWriter::new(std::io::stdout().lock());

    if total % 2 != 0 { writeln!(out, "NO").ok(); return; }

    let (mut a, mut b) = (vec![], vec![]);

    // The default stack is ~8 MB, which overflows at recursion depth n = 10^6.
    // Spawning a thread with a larger stack is the standard Rust workaround.
    let (a, b) = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || { solve(n, total / 2, &mut a, &mut b); (a, b) })
        .unwrap().join().unwrap();

    writeln!(out, "YES").ok();
    // Print both sets in the required format: count on one line, elements on the next.
    for set in [&a, &b] {
        writeln!(out, "{}", set.len()).ok();
        writeln!(out, "{}", set.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).ok();
    }
}