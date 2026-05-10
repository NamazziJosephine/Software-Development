use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// we can't import directly from bin/ in Rust, so we just copy the two
// algorithm functions here — this is completely normal for benchmarks

fn iterative(a: &mut Vec<i64>) -> i64 {
    let mut moves: i64 = 0;
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            moves += a[i - 1] - a[i];
            a[i] = a[i - 1];
        }
    }
    moves
}

fn recursive(a: &mut Vec<i64>, i: usize) -> i64 {
    if i == 0 {
        return 0;
    }
    let moves = recursive(a, i - 1);
    if a[i] < a[i - 1] {
        let diff = a[i - 1] - a[i];
        a[i] = a[i - 1];
        moves + diff
    } else {
        moves
    }
}

// worst case = fully decreasing array e.g. [n, n-1, n-2, ..., 1]
// every single element needs fixing, so both algorithms do maximum work
// this is the fairest way to stress test and compare them
fn make_worst_case(n: usize) -> Vec<i64> {
    (0..n).map(|i| (n - i) as i64).collect()
}

fn bench_algorithms(c: &mut Criterion) {
    // test at multiple sizes so we can see how each algorithm scales —
    // an algorithm that's fast at n=100 might fall apart at n=10_000
    let sizes = [100, 1_000, 10_000];

    // a benchmark group lets criterion print them side by side
    // and generate a combined comparison graph in the HTML report
    let mut group = c.benchmark_group("increasing_array");

    for &n in &sizes {
        // BenchmarkId just gives each run a nice label like "iterative/100"
        // so we can tell them apart in the output
        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| {
                let mut data = make_worst_case(n);
                // black_box stops the compiler from being "too clever" —
                // without it, the compiler might notice we never use the result
                // and optimize the entire call away, giving us fake 0ns timings
                iterative(black_box(&mut data))
            });
        });

        // we cap recursive at 10_000 because beyond that the call stack
        // gets too deep and the program crashes with a stack overflow —
        // which is exactly why we'd never submit the recursive version to a judge
        if n <= 10_000 {
            group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
                b.iter(|| {
                    let mut data = make_worst_case(n);
                    let last = data.len() - 1;
                    recursive(black_box(&mut data), last)
                });
            });
        }
    }

    // always call finish() at the end — it flushes the results and
    // triggers the HTML report generation under target/criterion/
    group.finish();
}

// these two macros are criterion's way of wiring everything together —
// criterion_group bundles our functions, criterion_main makes it a runnable binary
criterion_group!(benches, bench_algorithms);
criterion_main!(benches);