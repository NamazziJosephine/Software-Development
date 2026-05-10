use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // okay so first just read n and the array, nothing special here
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // this is where we'll keep track of how many times we had to increment
    let mut moves: i64 = 0;

    // so the idea is, we just walk through the array from left to right
    // and whenever we see an element that's smaller than the one before it,
    // we know we have a problem
    for i in 1..n {
        if a[i] < a[i - 1] {
            // basically a[i] is too small, so we figure out by how much
            // and that's exactly how many moves we need to fix it
            moves += a[i - 1] - a[i];

            // we also update a[i] itself, because the NEXT element will
            // compare against a[i], not the original value — this tripped
            // me up at first honestly
            a[i] = a[i - 1];
        }
        // if a[i] is already >= a[i-1], we don't need to do anything,
        // just move on to the next one
    }

    // and that's it! the total moves is our answer
    println!("{}", moves);
}