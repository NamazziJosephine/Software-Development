//! CSES "Tower of Hanoi" — iterative parity-rule solution.
//! Reads n from stdin, prints the move count then each move.

use hanoi::iterative_moves;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let moves = iterative_moves(n);
    writeln!(out, "{}", moves.len()).unwrap();
    for (from, to) in moves {
        writeln!(out, "{} {}", from, to).unwrap();
    }
    out.flush().unwrap();
}
