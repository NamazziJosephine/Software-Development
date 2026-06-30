# CSES 1193 — Labyrinth: two algorithms, benchmark, and interpretation

Find a shortest path from `A` to `B` on an n×m grid (up to 1000×1000) of floor
`.` and wall `#`, moving up/down/left/right. Print `YES`, the shortest length,
and one shortest path as `L/R/U/D` — or `NO`. Every move costs 1, so this is an
unweighted shortest-path problem: the natural setting to compare an **uninformed**
search against an **informed (heuristic)** one.

For the example the answer is `YES`, length `9`, e.g. `LDDRRRRRU`.

---

## Project structure

```
labyrinth/
├── Cargo.toml
├── README.md
├── src/
│   ├── bfs.rs           # Algorithm 1: breadth-first search (uninformed)
│   ├── astar.rs         # Algorithm 2: A* with Manhattan heuristic (informed)
│   ├── lib.rs           # library: shared Maze type, SearchResult, helpers
│   └── main.rs          # binary: imports both via lib.rs, runs one
├── benches/
│   └── labyrinth.rs     # Criterion benchmark: time + expansion counts
├── tests/
│   └── integration.rs   # vs a flood-fill oracle + path validator
└── cses/
    ├── bfs.rs           # flattened single-file copies for the CSES judge
    └── astar.rs         # (both algorithms are Accepted)
```

### How to run

```
cargo run --release -- bfs    < input.txt    # default
cargo run --release -- astar  < input.txt

cargo test          # correctness vs an independent flood-fill oracle
cargo bench         # BFS vs A*: timings + a cells-expanded table
```

For CSES, paste either `cses/bfs.rs` or `cses/astar.rs`; both are **Accepted**.

---

## The two algorithms

### Algorithm 1 — BFS (`src/bfs.rs`), uninformed

A FIFO queue expands cells in waves of equal distance from `A`. On an unweighted
grid the first wave to reach `B` gives the shortest distance. BFS does not know
where `B` is, so it explores blindly outward — roughly a growing diamond around
`A` — until a wave hits `B`. Each cell is touched with O(1) queue operations.
O(n·m) time and space.

### Algorithm 2 — A* (`src/astar.rs`), informed

A* is BFS/Dijkstra guided by a heuristic. It uses a priority queue ordered by
`f = g + h`, where `g` is the steps taken so far and `h` is the **Manhattan
distance** to `B`. On a 4-directional unit grid Manhattan distance never
overestimates the true remaining distance (it is *admissible*) and is also
*consistent*, so A* returns a genuine shortest path and the first pop of `B` is
optimal. The heuristic pulls the frontier toward `B` — but each step now costs an
O(log k) binary-heap push/pop plus a heuristic evaluation, rather than BFS's O(1)
queue work.

Both report the same shortest **length** (the path string may differ; any
shortest path is accepted) and the number of cells **expanded** — the second
number is what lets us explain *why* the heuristic helps, rather than only
timing it.

---

## Complexity

| | BFS | A* |
|---|---|---|
| Time | O(n·m), O(1) per cell | O(n·m log(n·m)) worst case, O(log k) per push |
| Space | O(n·m) | O(n·m) |
| Optimal? | Yes (unweighted) | Yes (admissible + consistent heuristic) |

Both are near-linear in the grid size; the question the benchmark answers is not
their growth rate but **how many cells each actually expands**, and whether A*'s
fewer-but-pricier expansions are worth it. As we will see, the answer is
"sometimes" — and *when* is the whole lesson.

---

## Benchmark

`benches/labyrinth.rs` (run with `cargo bench`) times both algorithms on five
1000×1000 maps chosen to make the heuristic helpful, useless, or misleading. It
also prints a **cells-expanded** table to stderr, because the expansion count is
the cause and the timing is the effect — separating them is what lets us explain
the results rather than just observe them.

* **open_corners** — `A` and `B` at opposite corners, no walls;
* **open_strip** — `A` and `B` at opposite ends of the *middle row*, no walls;
* **walls_20** — 20% random walls;
* **comb_maze** — a serpentine maze (`B` close in Manhattan distance, far by path);
* **no_path** — a full wall splits the grid, so `B` is unreachable.

Correctness is covered by `cargo test`: an independent flood-fill oracle for the
shortest length, plus a validator that replays each emitted path to confirm it is
legal and the right length, on hundreds of random mazes and at the 1000×1000
limit.

### Results

Optimised release build (`opt-level = 3`, `lto = true`) on a single
Intel Xeon core @ 2.8 GHz.

| map | BFS time | BFS expanded | A* time | A* expanded | expanded BFS/A* |
|---|--:|--:|--:|--:|--:|
| open_corners | 25 ms   | 1,000,000 | 56 ms   | 1,000,000 | 1.0× |
| open_strip   | 19 ms   | 750,000   | **0.24 ms** | **1,000** | **750×** |
| walls_20     | 32 ms   | 798,645   | 53 ms   | 457,064   | 1.75× |
| comb_maze    | 11 ms   | 500,500   | 16 ms   | 500,500   | 1.0× |
| no_path      | 11 ms   | 500,000   | 26 ms   | 500,000   | 1.0× |

(Path lengths: open = 1998, comb_maze = 500,499. End to end on a maximal input
both CSES binaries finish in well under 100 ms, inside the 1.00 s limit.)

---

## Interpretation — when does a heuristic actually help?

The naive expectation is "A* is faster than BFS." The benchmark shows that is
**false in four of the five maps** — A* is ~2× *slower* on open_corners,
comb_maze, and no_path, and still slower on walls_20 despite expanding fewer
cells. It is dramatically faster on exactly one map. Understanding why requires
separating the two things the benchmark measures: **how many cells get expanded**
(driven by the heuristic) and **how much each expansion costs** (driven by the
data structure). The single principle that ties every row together is:

> **A heuristic helps only to the extent that it can *discriminate* between
> promising and unpromising cells.** When it can, the win is enormous; when it
> cannot, A* degenerates into BFS carrying extra overhead.

### The win — open_strip (A* expands 750× fewer cells, runs ~80× faster)

`A` and `B` sit at the two ends of the middle row. For a cell `(r, c)`,
`g ≈ |r − mid| + c` and `h = |r − mid| + (m − 1 − c)`, so
`f = g + h = 2·|r − mid| + (m − 1)`. That value is **smallest exactly on the
middle row and grows with distance from it**, so A* strongly prefers cells on the
straight horizontal path and expands only a thin band — about 1,000 cells, the
path length. BFS has no such preference and floods a giant diamond of 750,000
cells before reaching `B`. Here the heuristic discriminates almost perfectly, and
even with its heavier per-cell cost A* wins by ~80×. This is heuristic search
doing what it is supposed to do.

### The degenerate case — open_corners (A* expands the *whole* grid)

Same open grid, but `A` and `B` are at opposite corners. Now for **every** cell,
`g + h = (r + c) + ((n−1−r) + (m−1−c)) = (n−1) + (m−1)` — a **constant**. Every
cell lies on *some* shortest corner-to-corner path, so the heuristic rates them
all equally and provides **zero discrimination**. A* expands all 1,000,000 cells,
exactly like BFS, and is ~2× slower purely because of the binary-heap overhead.
The striking point: the *same* heuristic on the *same* open grid is decisive in
open_strip and useless here — what changed is only the geometry, i.e. whether the
estimate can separate good cells from bad ones.

### Partial help that still loses on time — walls_20

Random walls break the corner-to-corner symmetry, so the heuristic regains some
discrimination: A* expands **1.75× fewer** cells than BFS (457k vs 799k). Yet A*
is still ~1.6× *slower* on the clock. This isolates the cost side of the trade:
BFS's queue is O(1) per cell; A*'s heap is O(log k) per push, so each A*
expansion costs roughly 2–2.5× a BFS expansion. A* only wins on time when it
expands *enough* fewer cells to overcome that constant factor — empirically here,
more than ~2.5× fewer. A 1.75× reduction is real but not enough.

### A misleading heuristic — comb_maze

The serpentine maze places `B` only 999 Manhattan-steps below `A`, but walls force
a path of 500,499 steps. The heuristic keeps insisting "`B` is close, head down,"
but down is walled — the estimate is wildly optimistic and gives no useful
ranking, so A* expands the same 500,500 cells as BFS and loses on time. A
heuristic is only as good as its correlation with the *true* remaining cost; here
that correlation is broken by structure the straight-line estimate cannot see.

### An unreachable goal — no_path

A solid wall splits the grid, so `B` cannot be reached. To *prove* there is no
path, every reachable cell must be examined — there is nothing to prune, by any
heuristic. Both algorithms expand all 500,000 cells of `A`'s half; A* is slower
only because of the heap. When the answer is "no," informedness cannot help.

### The cost model, stated plainly

A* wins on time roughly when

```
(cells BFS expands) / (cells A* expands)   >   (A* per-cell cost) / (BFS per-cell cost)  ≈ 2–2.5
```

The right-hand side is fixed by the data structures (heap vs queue). The
left-hand side is entirely determined by **how well the heuristic discriminates**
on that particular map: 750× on open_strip (huge win), 1.75× on walls_20 (not
enough), 1.0× on open_corners / comb_maze / no_path (no win, pure overhead).

### Takeaway for heuristic search

Both algorithms are correct and optimal — every map returns the same shortest
length (the admissible, consistent Manhattan heuristic guarantees A*'s
optimality). So the comparison is purely about *work*, and the lesson is that the
value of a heuristic is **not a property of the algorithm but of the heuristic's
fit to the problem instance**. A heuristic that strongly separates good cells from
bad ones (open_strip) turns a grid-flood into a near-straight walk; one that
cannot separate them (open_corners, no_path) or actively misleads (comb_maze)
leaves you running BFS with a priority queue strapped on. That is why, in real
heuristic search, *designing a heuristic that discriminates well* — and knowing
when it will fail — is the entire craft, and why measuring **nodes expanded**,
not just wall-clock time, is the honest way to evaluate one.

> Note on CSES: both submissions read the grid at once and write one buffered
> answer. A maximal 1000×1000 input runs in well under 100 ms for either
> algorithm, inside the 1.00 s limit, so either may be submitted. BFS is the
> safer default here: it is simpler, always within a small constant of optimal
> work, and never pays the heap overhead that hurts A* on the adversarial maps.
