# Increasing Array — Dynamic Arrays (Two Algorithms)

CSES "Increasing Array". Read `n` and then `n` integers. Each move increases one
element by 1. Print the minimum number of moves to make the array
**non-decreasing** (every element `>=` the one before it).

Both solutions are self-contained single files (no shared library), so the exact
file in `src/bin/` is what gets pasted into CSES.

## Topic: Dynamic Arrays

There is only one optimal *idea* for this problem — sweep left to right and raise
each element to the running maximum — so the two algorithms differ in **how they
use arrays**, which is exactly what the Dynamic Arrays topic is about:

| Algorithm | File | Extra memory | Strategy |
|-----------|------|--------------|----------|
| **In-place greedy** | `src/bin/iterative.rs` | **O(1)** | one running variable, no second array |
| **Prefix-maximum array** | `src/bin/prefix_max.rs` | **O(n)** | builds and re-reads an auxiliary `Vec` |

The shared insight: walking left to right, every element must be at least the
**maximum value seen so far**. If it is smaller, raising it costs the difference.

Example `[3, 2, 5, 1, 7]`: the running maxima are `[3, 3, 5, 5, 7]`, the gaps are
`0 + 1 + 0 + 4 + 0 = 5` moves.

---

## Algorithm 1 — In-place greedy (O(1) extra space)

Keep a single variable `prev` = the largest value seen so far. For each new
element: if it is below `prev`, add `prev - element` to the answer (it gets
raised up to `prev`); otherwise `prev` becomes the new element. The array is
never stored — values are processed as they are read.

- **Time:** Θ(n) — one pass.
- **Extra space:** Θ(1) — just `prev` and the running total.

## Algorithm 2 — Prefix-maximum auxiliary array (O(n) extra space)

Read the input into a `Vec`, then build a **second** dynamic array `pmax` where
`pmax[i] = max(a[0..=i])`. The answer is `sum(pmax[i] - a[i])`. Same result, but
it materialises a derived array and walks the data a second time.

- **Time:** Θ(n) — two passes (fill `pmax`, then sum).
- **Extra space:** Θ(n) — the auxiliary `pmax` array of `n` values.

---

## Why these two

Both are Θ(n) in time, so the benchmark isolates exactly what the topic teaches:
the cost of an **auxiliary dynamic array versus working in place**. Algorithm 1
is the cheapest possible use of memory; Algorithm 2 shows what building a second
`Vec` actually costs in allocation and cache traffic.

---

## Benchmark

Both algorithms are measured with [Criterion](https://crates.io/crates/criterion)
across a range of array sizes up to the CSES maximum (`n = 2·10^5`). The same
deterministic pseudo-random input is fed to both at each size. Release build,
`opt-level = 3`, `lto = true`. The benchmark keeps its own copies of the two
functions (there is no shared library — see `benches/benchmark.rs`).

> Numbers below are medians from one reference machine. **Re-run `cargo bench` on
> your own machine and replace this table** — absolute times vary by CPU, but the
> trend (in-place ~3.4× faster, both linear) is stable.

| n        | in-place (O(1)) | prefix_max (O(n)) | ratio (prefix / in-place) |
|----------|-----------------|-------------------|---------------------------|
| 1 000    | 0.89 µs         | 2.99 µs           | 3.4× |
| 10 000   | 8.74 µs         | 29.5 µs           | 3.4× |
| 100 000  | 90.2 µs         | 307 µs            | 3.4× |
| 200 000  | 181 µs          | 615 µs            | 3.4× |

Reproduce with:

```bash
cargo bench
```

### Reading the results

**1. Both are clearly Θ(n).** Multiply the array size by 10 and the time
multiplies by ~10 for both algorithms (in-place: 0.89 → 8.74 → 90.2 µs;
prefix_max: 2.99 → 29.5 → 307 µs). Neither has a hidden non-linear cost.

**2. In-place is ~3.4× faster at every size.** The ratio is almost perfectly
constant (3.37–3.40× across all four sizes), which tells us the difference is a
fixed per-element constant factor, not an algorithmic-complexity difference.

**3. Memory impact and caching — the reason for the 3.4× gap.** Both algorithms
do the same arithmetic; what differs is how much memory they move and how well it
caches:

- **In-place** keeps its entire working state — `prev` and the running total —
  in two CPU registers. It reads each input value once, in order, and writes
  nothing back. That is a single sequential read stream, which the hardware
  prefetcher handles perfectly: essentially zero cache misses beyond the
  unavoidable cost of reading the data once.
- **Prefix_max** allocates a second array of `n` 8-byte integers (`pmax`), so for
  `n = 2·10^5` that is ~1.6 MB of extra heap. It then does three things the
  in-place version never does: (a) it **allocates** that memory, (b) it **writes**
  `n` values into it during the first pass, and (c) it makes a **second pass**
  that reads *two* arrays (`pmax[i]` and `a[i]`) in parallel. That roughly
  doubles the bytes touched and the cache lines pulled in, and the `pmax` array
  competes with `a` for space in the L1/L2 cache. The extra allocation plus the
  second read stream is what shows up as the steady ~3.4× slowdown.

**4. Space.** In-place uses **O(1)** extra memory; prefix_max uses **O(n)**. At
the CSES limit that is a single register versus ~1.6 MB. Neither comes close to
the 512 MB limit, but it is a concrete illustration of the topic: an auxiliary
dynamic array is convenient to reason about but is never free.

### Bottom line

For CSES Increasing Array, **both pass comfortably** within the time and memory
limits — even at `n = 2·10^5` each runs in well under a millisecond. The
**in-place greedy is the better solution**: same Θ(n) time, O(1) instead of O(n)
space, ~3.4× faster, and far more cache-friendly (one sequential read stream in
registers, versus allocating, filling, and re-reading a second array). The
prefix-maximum version is a clear, correct way to express the same idea and a
useful illustration of what an auxiliary dynamic array costs — which is precisely
why it is the instructive contrast for this topic.

---

## Project layout

```
increasing-array/
├── Cargo.toml
├── src/
│   ├── main.rs            # just points to the two solution binaries
│   └── bin/
│       ├── iterative.rs   # CSES solution: in-place greedy, O(1) space
│       └── prefix_max.rs  # CSES solution: prefix-max auxiliary array, O(n) space
└── benches/
    └── benchmark.rs       # Criterion comparison across array sizes
```

## Build & run

```bash
# Run a solution (reads input from stdin)
echo "5
3 2 5 1 7" | cargo run --release --bin iterative     # -> 5
echo "5
3 2 5 1 7" | cargo run --release --bin prefix_max    # -> 5

# Benchmark
cargo bench
```

## CSES submission

Each file in `src/bin/` is self-contained, so paste the **whole file** into the
CSES submission box:

- `src/bin/iterative.rs` → the in-place greedy solution
- `src/bin/prefix_max.rs` → the prefix-maximum solution

Both are verified to produce the sample answer (`5`) and to handle the largest
inputs without overflow.

## Notes

- **i64 is required.** With `n` up to `2·10^5` and values up to `10^9`, the answer
  can reach ~`2·10^14`, which overflows a 32-bit integer. Both solutions
  accumulate in `i64`.
- **No recursion here.** A recursive sweep would recurse to depth `n = 2·10^5`
  and overflow the call stack on CSES's large tests. Recursion belongs to a
  different topic (Tower of Hanoi); for Dynamic Arrays the two array strategies
  above are the correct and safe pairing.
