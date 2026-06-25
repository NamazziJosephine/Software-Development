# CSES 1674 — Subordinates: two algorithms, benchmark, and interpretation

For every employee in a company tree, print how many subordinates they have.
Employee 1 is the director; every other employee has one direct boss.

The key observation: **an employee's number of subordinates is the size of
their subtree minus one** (everyone below them in the tree). So every solution
does the same core work — compute each subtree's size by summing children
before parents — and the only real choice is *how to traverse the tree*. The two
algorithms here differ in exactly that, which is why their performance depends
on the tree's *shape*.

For the example `n = 5`, bosses `1 1 2 3`:

```
        1
       / \
      2   3
      |   |
      4   5
```

subtree sizes are 5,2,2,1,1, so the answer (size - 1) is `4 1 1 0 0`.

---

## Project structure

```
subordinates/
├── Cargo.toml
├── README.md
├── src/
│   ├── recursive.rs     # Algorithm 1: recursive DFS (call stack)
│   ├── iterative.rs     # Algorithm 2: iterative DFS (explicit heap stack)
│   ├── lib.rs           # library: shared build_tree + `pub mod` declarations
│   └── main.rs          # binary: imports both algorithms via lib.rs, runs one
├── benches/
│   └── subordinates.rs  # Criterion benchmark (both algorithms, every shape/size)
├── tests/
│   └── integration.rs   # correctness vs an independent reference + worst cases
└── cses/
    ├── recursive.rs     # flattened single-file copy for the CSES judge
    └── iterative.rs     # flattened single-file copy for the CSES judge
```

Each algorithm is its own file in `src/`, exposed through `lib.rs`, and consumed
by `main.rs`, the benchmark, and the tests — one implementation reused
everywhere. The tree-building step (`build_tree`) is shared in `lib.rs`, so the
benchmark compares *traversal*, not parsing. The `cses/` folder holds standalone
single-file versions, because the online judge accepts only one self-contained
file and cannot import a library; the algorithm in each `cses/` file is
identical to its `src/` module.

### How to run

```
cargo run --release -- recursive  < input.txt   # choose the algorithm
cargo run --release -- iterative  < input.txt   # (default: iterative)

cargo test          # correctness vs reference + worst-case shapes
cargo bench         # Criterion benchmark, both algorithms on every shape/size
```

For CSES, paste `cses/recursive.rs` and `cses/iterative.rs` as two separate
submissions; both return **Accepted**.

---

## The two algorithms

Both first build the tree: from the boss list they form `children[v]` (and, for
the iterative one, `parent[v]`). Both are then a depth-first traversal that adds
up subtree sizes. They differ in **where the traversal's bookkeeping lives**.

### Algorithm 1 — Recursive DFS (`src/recursive.rs`)

`dfs(v)` recurses into each child, sums the returned subtree sizes, records
`size - 1` for `v`, and returns `size`. The traversal state — which child we are
on, the running size, the return address — lives in a **call-stack frame**, one
frame per level of the tree.

The catch: a valid company can be a single chain (employee `i` reports to
`i-1`), giving depth up to `n = 2*10^5`. Recursing that deep overflows the
default 8 MB stack and crashes. To survive it, the DFS runs on a dedicated
thread with a **256 MB stack**. That enlarged stack *is* recursion's memory cost
made visible.

### Algorithm 2 — Iterative DFS with an explicit stack (`src/iterative.rs`)

Same traversal, but the stack is an ordinary `Vec` on the **heap**, so there is
no depth limit and no special thread. Two passes:

1. A pop-push loop from the root produces a visit order in which every parent
   appears before its children.
2. Walking that order in **reverse**, each node records its own count and then
   adds its subtree size to its parent. Because children precede parents in
   reverse, every parent's size is complete by the time it is processed.

---

## Time and space complexity

| | Recursive | Iterative |
|---|---|---|
| Build tree | O(n) | O(n) |
| Traverse | O(n) | O(n) |
| **Total time** | **O(n)** | **O(n)** |
| Auxiliary space | O(n) heap (`children`, `counts`) **+ O(depth) call stack** | O(n) heap (`children`, `parent`, `order`, `size`, `counts`) |
| Worst-case stack/​depth | **O(n)** frames → needs a 256 MB stack | O(1) call depth; explicit stack is O(n) heap |

Both are linear in time and space. The difference the benchmark exposes is not
asymptotic — it is *where* the O(n) memory sits and how the hardware treats it.

---

## Benchmark

`benches/subordinates.rs` is a [Criterion](https://crates.io/crates/criterion)
benchmark. Because the tree's *shape* is what stresses a traversal, each "test
case" is a (shape, n) pair, and for every one we time **both** algorithms
(build + traverse). Three shapes:

* **chain** — depth = n (worst case for the call stack)
* **star** — depth 1, the director has n-1 direct reports (shallow, very wide)
* **random** — a realistic mixed-depth tree (each employee gets a random boss)

Run it with `cargo bench`. Correctness is covered by `cargo test`, which checks
both algorithms against an independent O(n^2) ancestor-walk reference on many
random trees, verifies order-independence, and runs the chain and star at the
`n = 2*10^5` limit.

### Results

Optimised release build (`opt-level = 3`, `lto = true`) on a single
Intel Xeon core @ 2.8 GHz. Per call (median of repeated runs):

| shape | n | recursive | iterative | recursive / iterative |
|-------|--:|----------:|----------:|----------------------:|
| chain  | 10,000  | 0.90 ms | 0.54 ms | 1.66× |
| chain  | 100,000 | 10.8 ms | 7.53 ms | 1.43× |
| chain  | 200,000 | 26.0 ms | 16.0 ms | **1.6×**  |
| star   | 10,000  | 0.13 ms | 0.12 ms | 1.05× |
| star   | 100,000 | 1.06 ms | 1.33 ms | 0.80× |
| star   | 200,000 | 2.05 ms | 5.37 ms | **0.39×** |
| random | 10,000  | 0.63 ms | 0.53 ms | 1.19× |
| random | 100,000 | 8.13 ms | 7.67 ms | 1.06× |
| random | 200,000 | 18.3 ms | 17.3 ms | 1.05× |

The headline is that **there is no single winner — it depends on the shape**:

* on a **deep chain**, iterative is ~1.6× faster;
* on a **wide star**, recursive is ~2.6× faster (ratio 0.39);
* on a **random tree**, they are within a few percent.

Every case finishes far inside the 1.00 s limit (worst observed ≈ 26 ms).

---

## Interpretation — why the winner depends on shape (memory & caching)

Both algorithms are O(n) time and O(n) space and visit every node once. The
runtime gap comes from **where each one keeps its O(n) state and how that
memory behaves in the cache**.

### Deep chain — recursion loses

A chain forces the recursion `n` levels deep, so `n` call frames are live at
once. A call frame is far larger than a single number: it holds saved registers,
the return address, and the loop state for iterating that node's children — on
the order of tens of bytes per node, versus the 4 bytes the iterative version
stores per node in its explicit stack. So recursion's working set for the
traversal is several times larger, and it is touched in a deep
push-all-then-pop-all pattern that has poor temporal locality: by the time the
deepest frame returns, the top frames have long been evicted from L1/L2. The
iterative version instead streams compact `u32` ids through small heap vectors
that pack many nodes per cache line, so it stays cache-friendly. On top of the
speed gap, recursion here also *requires* committing a large stack (the 256 MB
thread) — without it the program crashes, which is the sharpest possible
statement of its memory cost. Net: on deep trees recursion is both **slower and
riskier**.

### Wide star — iteration loses

When the tree is shallow (depth 1), recursion barely uses the stack: `dfs(root)`
just loops over the children, and each child call returns immediately. It is
essentially one flat sweep. The iterative version, by contrast, always pays for
its **two passes and extra arrays**: it fills an `order` vector, then walks it in
reverse touching the `size` and `parent` arrays. That is several full O(n)
passes over several distinct arrays. When the per-node work is trivial (a leaf
that returns at once), this extra memory traffic dominates, and the iterative
formulation ends up doing more total work — hence ~2.6× slower on the star.

### Random tree — near parity

A realistic tree is shallow on average (depth ~log n), so recursion never builds
a dangerous stack and its per-node call overhead roughly cancels the iterative
version's extra passes. The two finish within a few percent of each other.

### Takeaway

The asymptotics are identical; the real trade-off is **stack vs heap and the
cache behaviour of each**:

* **Recursive DFS** keeps state in fat, scattered call frames. It is excellent
  when the tree is shallow (minimal depth, minimal overhead) but degrades on
  deep trees — worse locality, a larger working set, and a hard dependence on a
  big stack to avoid overflow.
* **Iterative DFS** keeps state in compact heap vectors with no depth limit. It
  is robust and cache-friendly on deep trees, but its fixed multi-pass overhead
  makes it lose on very shallow, wide trees.

For a problem where the input *can* be a 200,000-deep chain, the iterative
version is the safer default — it cannot overflow and wins the case that would
otherwise crash. The recursive version is kept correct and competitive by giving
it the large stack it needs, which is precisely the memory lesson this exercise
is about.

> Note on CSES: both submissions read all input at once, build the tree, and
> write the whole answer in a single buffered `write_all` (with a manual integer
> formatter) instead of printing per number, so I/O is not the bottleneck. The
> recursive submission additionally runs its DFS on a 256 MB-stack thread to
> handle the chain case. Both are Accepted, with the worst case (~26 ms in this
> benchmark; ~42 ms end-to-end including I/O) far inside the 1.00 s limit.
