use std::io::{Write, BufWriter};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    // A split is only possible if the total sum is even.
    // 1+2+...+n = n*(n+1)/2, which is even when n % 4 == 0 or n % 4 == 3.
    let total = n * (n + 1) / 2;
    let mut out = BufWriter::new(std::io::stdout().lock());

    if total % 2 != 0 { writeln!(out, "NO").ok(); return; }

    let mut a = vec![];
    let mut b = vec![];
    let mut rem = total / 2;

    // Iterate from n down to 1, greedily filling set A up to the target sum.
    for cur in (1..=n).rev() {
        if rem >= cur { a.push(cur); rem -= cur; }
        else          { b.push(cur); }
    }

    writeln!(out, "YES").ok();
    // Print both sets: count on one line, elements on the next.
    for set in [&a, &b] {
        writeln!(out, "{}", set.len()).ok();
        writeln!(out, "{}", set.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).ok();
    }
}