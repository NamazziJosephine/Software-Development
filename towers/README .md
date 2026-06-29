# CSES 1073 — Towers: two algorithms, benchmark, and interpretation

Cubes arrive one at a time. A cube may be placed on top of a tower only if the
cube currently on top is **strictly larger**; otherwise it starts a new tower.
Minimise the number of towers. Up to `n = 2·10^5` cubes, sizes up to `10^9`.

**Greedy rule (optimal).** Place each cube `s` on the tower whose top is the
*smallest value strictly greater than `s`*; if no such tower exists, start a new
one. Placing on the smallest sufficient top "wastes" the least — it keeps larger
tops free for future larger cubes (standard exchange argument). The number of
towers is how many times we had to start a new one.

This is exactly **patience sorting**, and the number of towers equals the length
of the longest non-decreasing subsequence of the cube sequence. For the example
(`2 3 1`) the answer is `2`.

---

## Project structure

```
towers/
├── Cargo.toml
├── README.md
├── src/
│   ├── btree.rs         # Algorithm 1: BTreeMap multiset of tower tops (B-tree)
│   ├── patience.rs      # Algorithm 2: patience sorting on a sorted Vec
│   ├── lib.rs           # library: `pub mod` declarations
│   └── main.rs          # binary: imports both via lib.rs, runs one
├── benches/
│   └── towers.rs        # Criterion benchmark (both algorithms, several shapes)
├── tests/
│   └── integration.rs   # vs a brute-force oracle + worst cases at the limit
└── cses/
    ├── btree.rs         # flattened single-file copies for the CSES judge
    └── patience.rs      # (both algorithms are Accepted)
```

### How to run

```
cargo run --release -- btree     < input.txt    # default
cargo run --release -- patience  < input.txt

cargo test          # correctness vs a brute-force oracle + worst cases
cargo bench         # Criterion: both algorithms across input shapes
```

For CSES, paste either `cses/btree.rs` or `cses/patience.rs`; both are
**Accepted**.

---

## The two algorithms

### Algorithm 1 — `BTreeMap` multiset (`src/btree.rs`)

Hold the tower tops in a multiset `BTreeMap<top, count>`. For each cube `s`, find
the smallest top strictly greater than `s` (a **successor query**):
* found `k` → that tower's top becomes `s`: erase one `k`, insert one `s`;
* not found → start a new tower: insert one `s`.

`BTreeMap` is a **B-tree**: a balanced, high-fan-out search tree where each node
holds many keys. Each cube costs one `O(log n)` successor query plus an
erase/insert. Total `O(n log n)`.

### Algorithm 2 — patience sorting on a sorted `Vec` (`src/patience.rs`)

Hold the tower tops in a single ascending `Vec`. For each cube `s`, binary-search
the first top strictly greater than `s` (`partition_point` returns the count of
tops `≤ s`, i.e. the index of the first one `> s`):
* index `i < len` → overwrite `tops[i] = s` in place (the array stays sorted);
* otherwise → append `s` (a new tower).

The final length is the answer. Also `O(n log n)`, but everything happens in one
contiguous array: a binary search and a single write, no per-node allocation.

---

## Time and space complexity

| | BTreeMap | Patience (Vec) |
|---|---|---|
| Time | `O(n log n)` — successor query + erase/insert per cube | `O(n log n)` — binary search + overwrite/append per cube |
| Space | `O(t)` — `t` = number of towers, as tree nodes | `O(t)` — one contiguous array |

Both are `O(n log n)` with the same space class, so — as in the previous
deliverable — any real difference is a **constant factor**, and the benchmark
exists to measure and *explain* it. What makes this problem interesting is that
the constant factor is not fixed: it swings from **3.6× to 40×** depending on the
input shape, and tracing *why* is the whole point.

---

## Benchmark

`benches/towers.rs` (run with `cargo bench`) times **both** algorithms. Because
the number of towers — and therefore how the data structure grows and is used —
depends entirely on the input shape, the benchmark uses four shapes at
`n = 2·10^5`, plus a size sweep on random input:

* **random** — sizes in `1..=10^9`; a moderate number of towers;
* **increasing** — `1,2,…,n`: every cube starts a new tower (structure grows to `n`);
* **decreasing** — `n,…,2,1`: everything stacks (structure stays size 1);
* **few_distinct** — sizes in `1..=1000`: a small set of distinct tops.

Correctness is covered by `cargo test`: a brute-force `O(n²)` oracle on small
inputs, plus agreement between the two algorithms on random, increasing,
decreasing, and all-equal inputs at the `2·10^5` limit.

### Results

Optimised release build (`opt-level = 3`, `lto = true`) on a single
Intel Xeon core @ 2.8 GHz. Per call:

**by input shape, n = 200,000** (towers = the structure's final size):

| shape | BTreeMap | Patience | BTree / Patience | towers |
|---|--:|--:|--:|--:|
| random       | 41 ms   | 10.1 ms | **4.0×**  | 884 |
| increasing   | 29 ms   | 2.28 ms | **12.8×** | 200,000 |
| decreasing   | 8.5 ms  | 0.21 ms | **40×**   | 1 |
| few_distinct | 36 ms   | 10.1 ms | **3.6×**  | 1,077 |

**scaling on random input:**

| n | BTreeMap | Patience | BTree / Patience |
|--:|--:|--:|--:|
| 10,000  | 1.47 ms | 0.38 ms | 3.9× |
| 100,000 | 19 ms   | 4.9 ms  | 3.9× |
| 200,000 | 41 ms   | 10 ms   | 4.0× |

(End to end, including reading a maximal input, the CSES binaries run in ≈ 50 ms
(BTreeMap) and ≈ 19 ms (Patience) — both far inside the 1.00 s limit.)

---

## Interpretation — reading the numbers

Both algorithms are `O(n log n)`, yet the array version wins everywhere, by a
factor that ranges from **3.6× to 40×**. The interesting question is not *that*
it wins but *why the margin moves so much*, and the answer is a clean lesson
about what a B-tree's generality costs.

### Observation 1 — The B-tree's cost tracks the *operation mix*, not the structure size

The most striking pair of rows: **random** builds a structure of only **884**
entries yet is the B-tree's **slowest** shape (41 ms), while **increasing** builds
a **200,000**-entry structure — 226× larger — and is **faster** (29 ms). If cost
were driven by how big the tree gets, this would be impossible. It is driven by
*what operations run*:

* **Random** input: most cubes *do* find a successor, so nearly every one of the
  200,000 cubes performs an **erase + insert** — it removes an existing top and
  inserts a new one. That churns the tree's structure (node updates, occasional
  rebalancing) on essentially every cube.
* **Increasing** input: each cube is larger than all current tops, so the
  successor query always returns *nothing* and the cube is **inserted at the
  maximum** with **no erase**. Inserting at the growing right edge is a far
  gentler workload than constant erase-and-reinsert churn — even though the tree
  ends up huge.

So the B-tree is punished by *modification churn*, not by size. That is the key
insight this benchmark surfaces.

### Observation 2 — `decreasing` (40×) isolates the B-tree's fixed per-operation overhead

With strictly decreasing input the structure **never grows beyond one element**:
every cube is smaller than the only top, so it replaces it. Both algorithms do
`n` iterations on a one-element structure, which strips the comparison down to
pure per-operation overhead:

* **Patience** writes to `tops[0]` every time — the same cache line, one
  branchlessly-located slot — and finishes in **0.21 ms**, essentially just the
  cost of the loop.
* **BTreeMap** still pays its full machinery per cube even on a single node:
  setting up a range query with `Bound`s, walking to find the (absent) successor,
  then an erase and an insert that touch the node. That is **8.5 ms** — about
  **40× more** than an array write.

This row is valuable precisely because it removes cache effects (one element fits
trivially) and shows the B-tree's *constant overhead per operation* in isolation.

### Observation 3 — Why Patience is faster on `increasing` than on `random` (branch prediction, not cache)

A subtle but instructive inversion: Patience is **faster on increasing**
(2.28 ms, array grows to 200,000) than **on random** (10 ms, array only 884
elements) — the *smaller* array is *slower*. Cache cannot explain this: 884
`u32`s is 3.5 KB, comfortably inside L1, so the random case is not missing the
cache. The most plausible explanation is **branch prediction**. On increasing
input, every `partition_point` search resolves in the same direction ("`s` is
≥ everything, go right"), so the CPU predicts the binary-search branches almost
perfectly and the search is nearly free. On random input the search lands at
scattered positions, the branch directions are unpredictable, and each
misprediction stalls the pipeline. Same instruction count, very different cost —
a reminder that on data that already fits in cache, *branch behaviour* can
dominate.

### Observation 4 — On random input the gap is a stable ~4× constant factor

The scaling table holds the shape fixed (random) and grows `n`: the ratio stays
**3.9×–4.0×** from 10k to 200k, and each algorithm roughly doubles when `n`
doubles (100k→200k: BTreeMap ×2.1, Patience ×2.0), consistent with `O(n log n)`.
So for a fixed shape the two curves are parallel — the array version is a
constant ~4× faster, not asymptotically faster. The *only* thing that changes the
constant is the input shape (Observations 1–3), not the size.

### What this means for the B-tree topic

`BTreeMap` is already the cache-conscious member of the balanced-tree family:
high fan-out, many keys per node, designed to minimise block fetches. Yet a plain
sorted `Vec` still beats it 4–40× here. The reason is that the array is, in a
sense, the *degenerate optimum of the B-tree idea* — a single contiguous "node"
holding everything, with zero pointers and zero rebalancing. A B-tree pays for a
capability this problem never uses: efficient insertion and deletion **anywhere**
in the order while staying balanced. Patience sorting only ever overwrites one
slot or appends at the end, so it needs none of that machinery.

The benchmark therefore measures exactly what the B-tree's generality costs when
the workload doesn't require it — and shows that the cost is dominated by
*structure-modifying operations* (Observation 1) and a fixed per-operation
overhead (Observation 2), not by the amount of data stored. Choose the B-tree
when you genuinely need ordered insert/delete/range over a changing set; when the
access pattern collapses to "binary-search then overwrite or append", the flat
array is dramatically faster for the same asymptotic cost.

> Note on CSES: both submissions read all input at once and write a single line.
> A maximal input (`n = 2·10^5`, sizes up to `10^9`) runs in ≈ 50 ms (BTreeMap) or
> ≈ 19 ms (Patience) end to end — both comfortably inside the 1.00 s limit, so
> either may be submitted.
