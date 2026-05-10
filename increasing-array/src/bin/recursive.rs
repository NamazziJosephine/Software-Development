use std::io::{self, Read};

fn solve(a: &mut Vec<i64>, i: usize) -> i64 {
    // Base case: the first element has nothing to its left,
    // so it always needs 0 moves.
    if i == 0 {
        return 0;
    }

    // Recursively ensure the subarray a[0..i-1] is non-decreasing first.
    // After this call, a[i-1] holds the (possibly increased) correct value.
    let moves = solve(a, i - 1);

    // Now check if the current element is smaller than its predecessor.
    if a[i] < a[i - 1] {
        // It is too small — calculate how many increments are needed
        // to bring a[i] up to a[i-1].
        let diff = a[i - 1] - a[i];

        // Update a[i] in place so future recursive calls (if any)
        // see the corrected value as their left neighbor.
        a[i] = a[i - 1];

        // Bubble the total move count back up the call stack.
        moves + diff
    } else {
        // a[i] is already >= a[i-1], no moves needed here.
        moves
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // Read n, then collect exactly n integers into the array.
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // Kick off the recursion from the last index.
    // The answer accumulates as the call stack unwinds.
    println!("{}", solve(&mut a, n - 1));
}