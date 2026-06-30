// CSES 2208 "Another Game" — Algorithm 2 (binary-heap version). Self-contained.
// Same decision (first wins iff some heap is odd) routed through a BinaryHeap:
// heapify the counts, then pop from the top until an odd one appears. Correct
// and within limits, though O(n log n) vs the parity scan's O(n).
use std::io::{self, Read, Write};
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u32>().unwrap());
    let t = it.next().unwrap();
    let mut out: Vec<u8> = Vec::with_capacity(t as usize * 7);
    for _ in 0..t {
        let n = it.next().unwrap() as usize;
        let heaps: Vec<u32> = (0..n).map(|_| it.next().unwrap()).collect();
        let mut heap: BinaryHeap<u32> = BinaryHeap::from(heaps);
        let mut first = false;
        while let Some(x) = heap.pop() {
            if x % 2 == 1 {
                first = true;
                break;
            }
        }
        out.extend_from_slice(if first { b"first\n" } else { b"second\n" });
    }
    io::stdout().write_all(&out).unwrap();
}
