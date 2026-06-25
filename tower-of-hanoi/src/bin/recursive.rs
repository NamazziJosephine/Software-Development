//! CSES "Tower of Hanoi" — recursive divide-and-conquer solution.
//! Reads n from stdin, prints the move count then each move.

use hanoi::recursive_moves;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    // Buffered output: there can be up to 2^16 - 1 lines, and an unbuffered
    // println! per line is the usual cause of a TLE on this kind of problem.
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let moves = recursive_moves(n);
    writeln!(out, "{}", moves.len()).unwrap();
    for (from, to) in moves {
        writeln!(out, "{} {}", from, to).unwrap();
    }
    out.flush().unwrap();
}
