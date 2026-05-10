use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut moves: i64 = 0;
    for i in 1..n {
        if a[i] < a[i - 1] {
            moves += a[i - 1] - a[i];
            a[i] = a[i - 1];
        }
    }

    println!("{}", moves);
}