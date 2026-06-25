# CSES 1192 — Counting Rooms: three algorithms, benchmark, and interpretation

Count the rooms in an `n x m` building map. A square is floor (`.`) or wall
(`#`); you move up/down/left/right through floor squares. A **room** is a
connected group of floor squares.

This is the classic **connected-components** problem on a grid graph: each floor
cell is a node, edges join orthogonally-adjacent floor cells, and a room is one
connected component. The grid can be up to `1000 x 1000 = 10^6` cells. Three
standard graph approaches are compared.

For the example (`5 x 8`) the answer is `3`.

---

## Project structure

```
counting-rooms/
├── Cargo.toml
├── README.md
├── src/
│   ├── bfs.rs            # Algorithm 1: BFS flood fill (FIFO queue)
│   ├── dfs.rs           # Algorithm 2: DFS flood fill (LIFO stack)
│   ├── union_find.rs    # Algorithm 3: Union-Find (disjoint set union)
│   ├── lib.rs           # library: shared Grid + `pub mod` declarations
│   └── main.rs          # binary: imports all three via lib.rs, runs one
├── benches/
│   └── counting_rooms.rs # Criterion benchmark (all three, every grid)
├── tests/
│   └── integration.rs   # vs an independent oracle + worst-case grids
└── cses/
    ├── bfs.rs           # flattened single-file copies for the CSES judge
    ├── dfs.rs
    └── union_find.rs
```

Each algorithm is its own file in `src/`, exposed through `lib.rs`, and consumed
by `main.rs`, the benchmark, and the tests. The grid is parsed once into a
shared `Grid` (a flat floor mask), so the benchmark compares algorithms, not
parsers. The `cses/` folder holds standalone single-file versions, because the
judge accepts only one self-contained file.

### How to run

```
cargo run --release -- bfs        < input.txt
cargo run --release -- dfs        < input.txt    # default
cargo run --release -- union_find < input.txt

cargo test          # correctness vs an independent oracle + worst cases
cargo bench         # Criterion: all three algorithms on every grid
```

For CSES, paste `cses/bfs.rs`, `cses/dfs.rs`, and `cses/union_find.rs` as
separate submissions; all three return **Accepted**.

---

## The three algorithms

All three read the grid into a flat floor mask. They differ in how they find the
components.

### Algorithm 1 — BFS flood fill (`src/bfs.rs`)

Scan all cells; at each unvisited floor cell, count a new room and flood its
component using a **FIFO queue**. BFS processes cells in expanding rings outward
from the start.

### Algorithm 2 — DFS flood fill (`src/dfs.rs`)

Identical scan, but the frontier is a **LIFO stack** (an explicit `Vec`, not
recursion — one open room can be `10^6` cells deep, which would overflow the
call stack). With LIFO order the next cell processed is one just pushed: a
neighbour, close by in memory.

### Algorithm 3 — Union-Find (`src/union_find.rs`)

A different paradigm: no traversal. Every floor cell starts as its own set; we
**union** each floor cell with its right and down floor neighbours (this covers
all adjacencies). The number of disjoint sets left is the number of rooms. Uses
path compression (path halving) and union by size.

---

## Time and space complexity

| | BFS | DFS | Union-Find |
|---|---|---|---|
| Time | O(n·m) | O(n·m) | O(n·m · α(n·m)) |
| Space | O(n·m) | O(n·m) | O(n·m) |

`α` is the inverse Ackermann function. **This is the one real asymptotic
difference:** BFS and DFS are strictly linear in the number of cells, while
Union-Find with path compression + union by size is *near*-linear — `α(n·m) ≤ 4`
for any input that fits in memory, so it is effectively constant, but technically
super-linear. In other words, Big-O predicts BFS ≈ DFS, and Union-Find very
slightly worse. The benchmark shows the real story is dominated by constant
factors (memory access patterns), not by `α`.

---

## Benchmark

`benches/counting_rooms.rs` is a [Criterion](https://crates.io/crates/criterion)
benchmark. Grid *shape* is what stresses each approach, so a "test case" is a
(shape, side) pair, and each one times **all three** algorithms:

* **open** — every cell floor: one giant room (maximum adjacencies / unions)
* **checker** — floor iff `(r+c)` even: ~half the cells, all isolated (no
  adjacencies)
* **random** — ~50% walls: many small/medium rooms

Run with `cargo bench`. Correctness is covered by `cargo test`, which checks all
three against an independent recursive-flood-fill oracle on random grids, against
structured grids with known counts, and on the `1000 x 1000` worst cases.

### Results

Optimised release build (`opt-level = 3`, `lto = true`) on a single
Intel Xeon core @ 2.8 GHz. Per call, pure computation (parsing excluded):

| shape | side | BFS | DFS | Union-Find |
|-------|-----:|----:|----:|-----------:|
| open    | 500  | 3.37 ms | 3.13 ms | 4.48 ms |
| open    | 1000 | 14.0 ms | **12.6 ms** | 16.0 ms |
| checker | 500  | 1.45 ms | 1.26 ms | 0.55 ms |
| checker | 1000 | 5.9 ms  | 5.0 ms  | **2.3 ms** |
| random  | 500  | 5.90 ms | 5.00 ms | 3.69 ms |
| random  | 1000 | 24.7 ms | 20.5 ms | **15.1 ms** |

(Going from side 500 to 1000 is 4× the cells; every time roughly 4×, confirming
the linear scaling — the `α` term never shows up as growth.)

Two clear patterns:

1. **DFS beats BFS on every grid** (~10–20% faster).
2. **Union-Find is the fastest or the slowest depending on the grid**: slowest on
   the open room, fastest on checker and random.

Worst case is ~25 ms — far inside the 1.00 s limit.

---

## Interpretation — complexity vs. actual performance (memory & caching)

The headline is that **computational complexity and measured performance
disagree here**, and the gap is explained by memory access patterns.

### Complexity perspective

By Big-O, BFS and DFS are identical (`O(n·m)`) and Union-Find is *marginally*
worse (`O(n·m·α)`). If complexity were the whole story, BFS and DFS would tie and
Union-Find would always lose slightly. The benchmark shows none of that holds in
practice — so the interesting differences are **constant factors driven by the
memory hierarchy**, exactly what Big-O ignores.

### DFS vs BFS — same work, different cache behaviour

BFS and DFS do the *same* number of operations (each cell enqueued/pushed once,
each edge checked once). The difference is the order in which memory is touched:

* **DFS (LIFO stack):** the next cell popped is the one most recently pushed —
  an immediate neighbour of the current cell, so a nearby index in the row-major
  arrays. DFS therefore keeps working inside a small, recently-used region, and
  the `floor`/`visited` bytes it needs are usually already in L1/L2. Good spatial
  and temporal locality.
* **BFS (FIFO queue):** the next cell dequeued was enqueued long ago and belongs
  to the current "ring" of the flood. In a large room a ring spans cells that are
  many rows apart, so consecutive accesses jump across distant rows of the array
  (each row is `m` cells ≈ several KB apart). Those jumps miss the cache more
  often. BFS's queue also holds a whole frontier at once, a larger live working
  set than DFS's stack in a compact region.

Same operations, worse access pattern → BFS is consistently ~10–20% slower. This
is a textbook "identical Big-O, different real speed because of caching" case.

### Union-Find — cost scales with the number of merges

Union-Find does **not** traverse; its cost is dominated by the `union`/`find`
work, which is proportional to the number of adjacent floor *pairs* (edges), and
each `find` chases pointers through the `parent` array:

* **Open room (worst for UF):** every one of the `10^6` cells is floor, so there
  are ~`2·10^6` adjacencies and therefore ~`2·10^6` union/find operations, each
  with pointer-chasing and path-compression writes. Worse, half of them union a
  cell with its *down* neighbour, an index `m = 1000` cells (≈4 KB) away — a
  long stride that misses the cache much like BFS's row jumps. All that
  per-edge overhead makes UF the **slowest** here, beaten even by BFS.
* **Checker (best for UF):** no two floor cells are adjacent, so **zero** unions
  happen. UF degenerates to "initialise the array, then scan and find no
  neighbours" — almost pure linear streaming with no pointer chasing. The
  traversals, by contrast, still pay to start 500 000 separate floods (push,
  pop, four neighbour checks each). UF wins by ~2×.
* **Random (UF fastest):** moderate adjacency. UF's per-cell work (a couple of
  finds, mostly on short compressed paths) is cheaper than the traversals'
  queue/stack churn, and the scattered walls wreck the traversals' frontier
  locality more than they hurt UF's mostly-sequential right-neighbour unions.

So Union-Find's ranking tracks **how much merging the grid forces**: lots of
merging (one big room) is its worst case; little or cheap merging (sparse or
isolated rooms) is its best.

### Takeaway

* Complexity alone is misleading here: all three are near-linear, and the `α`
  term in Union-Find never materialises as visible growth.
* **DFS is the safe default**: it matches BFS's complexity but wins on every
  grid thanks to LIFO cache locality, and (being iterative) it cannot overflow
  the stack on a giant room.
* **Union-Find is shape-sensitive**: excellent when components are many and small
  (few merges), poor when the whole map is one component (maximal merges).
* **BFS** is never the fastest here — its FIFO frontier has the worst locality of
  the three — but it is simple and correct and well within the time limit.

> Note on CSES: reading and parsing the `10^6`-character map is itself an `O(n·m)`
> pass and a large share of real wall time, so end-to-end the three submissions
> are closer than the compute-only benchmark above (which excludes parsing). All
> three read the whole grid at once and write a single integer, and all three are
> Accepted with the worst case around 20–30 ms including I/O — far inside 1.00 s.
