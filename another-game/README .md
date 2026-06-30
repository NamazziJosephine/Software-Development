# CSES 2208 — Another Game: two algorithms, benchmark, and interpretation

`n` heaps of coins; two players alternate. On a move, a player removes one coin
from **any chosen subset** of the nonempty heaps; whoever takes the last coin
wins. Decide the winner under optimal play, for up to `2·10^5` test cases (total
heaps `≤ 2·10^5`, coin counts up to `10^9`).

**The result:** the first player wins **iff at least one heap is odd**. If every
heap is even, the second player mirrors each move — removing one coin from the
exact same subset — so every heap returns to even after each round; "all even"
is therefore a losing position for whoever must move, and the second player can
always hand it back. If some heap is odd, the first player makes one move to "all
even" and gives that losing position away. So the whole question reduces to:
*is any heap count odd?* (Examples: `1 2 3` → first; `2 2` → second;
`5 5 4 5` → first.)

> Topic note. This problem sits under "Heaps". The heaps in the statement are
> *coin heaps*, not the data structure. To exercise an actual heap data
> structure under this topic, the second algorithm computes the same answer
> through a binary heap (`BinaryHeap`). It is deliberately not the better tool —
> and the benchmark measures exactly what using it costs.

---

## Project structure

```
another-game/
├── Cargo.toml
├── README.md
├── src/
│   ├── parity.rs        # Algorithm 1: O(n) parity scan (the natural solution)
│   ├── heap.rs          # Algorithm 2: the same decision via a BinaryHeap
│   ├── lib.rs           # library: Winner type + module declarations
│   └── main.rs          # binary: imports both via lib.rs, runs one
├── benches/
│   └── another_game.rs  # Criterion benchmark
├── tests/
│   └── integration.rs   # exhaustive game-search oracle proves both correct
└── cses/
    ├── parity.rs        # flattened single-file copies for the CSES judge
    └── heap.rs          # (both algorithms are Accepted)
```

### How to run

```
cargo run --release -- parity  < input.txt    # default
cargo run --release -- heap    < input.txt

cargo test          # both algorithms vs an exhaustive game-search oracle
cargo bench         # parity scan vs binary heap
```

For CSES, paste either `cses/parity.rs` or `cses/heap.rs`; both are **Accepted**.

---

## The two algorithms

### Algorithm 1 — parity scan (`src/parity.rs`)

One pass over the heaps, returning "first" as soon as an odd count is seen,
otherwise "second". O(n) time, O(1) space, with an early exit on the first odd.

### Algorithm 2 — binary heap (`src/heap.rs`)

Compute the *same* answer through a max-heap: build a `BinaryHeap` from the coin
counts, then pop from the top one at a time, stopping at the first odd value (if
the heap drains with none, the second player wins). Building the heap is O(n)
(heapify); each pop is O(log n); so this is O(n log n). The order in which counts
emerge is irrelevant to a parity test, so the heap does no useful work here — it
is included to demonstrate the data structure and to measure its overhead.

Correctness of both is established in `cargo test` by an **exhaustive game-search
oracle**: a literal (exponential) solver of the game's rules, run on every small
configuration, confirming that the parity rule — and both algorithms built on it
— give the true game outcome.

---

## Time and space complexity

| | Parity scan | Binary heap |
|---|---|---|
| Time | **O(n)**, early exit on first odd | **O(n log n)** (heapify O(n) + up to n pops at O(log n)) |
| Space | **O(1)** | **O(n)** for the heap |

Both solve the same problem; the heap version is asymptotically worse and, as the
benchmark shows, much worse by a constant factor too.

---

## Benchmark

`benches/another_game.rs` (run with `cargo bench`) times both algorithms on two
shapes:

* **all_even** — every heap is even, so there is no odd value to find: *neither*
  algorithm can exit early, and each does its full work. This is the fair
  algorithmic comparison — the difference is purely the cost of the heap.
* **first_odd** — the single odd heap is placed first. The parity scan exits
  immediately; the heap, popping largest-first, cannot use that and must drain.

### Results

Optimised release build (`opt-level = 3`, `lto = true`) on a single
Intel Xeon core @ 2.8 GHz. Per call:

**all_even (full work for both):**

| n | parity | heap | heap / parity |
|--:|-------:|-----:|--------------:|
| 10,000  | 4.7 µs | 0.18 ms | ~38× |
| 100,000 | 46 µs  | 2.9 ms  | ~63× |
| 200,000 | 94 µs  | 9.2 ms  | ~98× |

**first_odd (odd heap at the front):**

| n | parity | heap | heap / parity |
|--:|-------:|-----:|--------------:|
| 10,000  | 1.5 ns | 0.16 ms | ~100,000× |
| 100,000 | 1.5 ns | 3.0 ms  | ~2,000,000× |
| 200,000 | 1.5 ns | 8.4 ms  | ~5,000,000× |

(End to end on CSES, a maximal input runs in ≈ 19 ms (parity) and ≈ 24 ms (heap)
— both far inside the 1.00 s limit.)

---

## Interpretation — what the heap costs, and why

Both algorithms return the same answer on every input (the exhaustive oracle in
the tests proves it). So the benchmark is not about correctness but about the
price of solving a one-pass problem with a data structure built for something
else. Two shapes expose two different parts of that price.

### all_even — the honest "cost of the data structure" (≈ 40–100×)

With no odd heap, neither algorithm can stop early, so this isolates the work
each *must* do. The parity scan is a single sequential pass: read each count,
test its low bit, move on — O(n) with perfect cache behaviour (the prefetcher
sees a straight walk through memory). The heap version must first heapify the
counts and then pop all `n` of them, each pop an O(log n) sift-down. That is
O(n log n), and the measured gap is large: **38× at n = 10k rising to ~98× at
n = 200k.**

The fact that the ratio *grows* with `n` is itself informative. A pure `log n`
factor would widen the gap only mildly over this range (`log` grows from ~13 to
~18, a factor of ~1.3). The gap instead grows ~2.6×, so something beyond the log
factor is at work: **cache behaviour.** A sift-down repeatedly jumps from index
`i` to its children at `2i+1` / `2i+2`, addresses that spread further apart as
the heap grows; once the heap array exceeds the cache (200k × 4 B ≈ 800 KB, past
L2) those jumps start missing, while the parity scan keeps streaming
sequentially. So the heap pays the log factor *and* a worsening cache penalty as
the input grows.

### first_odd — the heap destroys the natural early exit (up to millions×)

This shape shows a qualitatively different cost. The parity scan reads heaps in
order and returns the instant it sees an odd one; with the odd heap first it
finishes in **1.5 ns regardless of `n`**. The heap *cannot* do this: a max-heap
imposes its own order, popping the largest counts first, so a small odd value is
found last no matter where it sat in the input. The heap therefore heapifies and
drains essentially everything, and the ratio explodes into the millions.

The lesson is sharper than "the heap is a bit slower": the data structure
**actively discards the property that makes the scan fast.** Input order — which
the scan exploits for an O(1) early exit — is exactly what the heap throws away.

### Takeaway — match the structure to the operation

A binary heap is the right tool when you need to repeatedly extract the
minimum or maximum of a changing set (scheduling, Dijkstra, k-largest, merging
runs). This problem needs none of that — it asks a single existence question, "is
any count odd?", which a flat scan answers in one sequential pass with O(1) space
and an early exit. Routing it through a heap adds an asymptotic `log n` factor,
cache-unfriendly access, a heap allocation, and throws away the early exit — it
is strictly worse on every axis, by 40× to millions× depending on the input.

That is the point of the comparison: the same correct answer, computed two ways,
showing concretely that choosing a data structure is about fitting it to the
operations the problem actually needs — and that a powerful structure used where
it doesn't fit is pure overhead.

> Note on CSES: both submissions read all input at once and write all answers in
> a single buffered write. A maximal input (`t = 2·10^5`, total heaps `2·10^5`)
> is solved in ≈ 19 ms (parity) or ≈ 24 ms (heap), both well inside the 1.00 s
> limit, so either may be submitted — though parity is the natural choice.
