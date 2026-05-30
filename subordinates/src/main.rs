use std::io::{self, Read, Write, BufWriter};

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut iter = input.split_ascii_whitespace();

    // Read the total number of employees
    let n: usize = iter.next().unwrap().parse().unwrap();

    // children[i] holds the list of direct reports for employee i
    let mut children = vec![vec![]; n + 1];

    // Employees 2..=n each declare their direct boss; build the tree top-down
    for employee in 2..=n {
        let boss: usize = iter.next().unwrap().parse().unwrap();
        children[boss].push(employee);
    }

    // subordinates[i] will hold the final answer for employee i
    let mut subordinates = vec![0usize; n + 1];

    // Explicit stack holds (node, phase):
    //   phase 0 — first visit: push children onto the stack for processing
    //   phase 1 — post-visit: all children are done, accumulate their counts into parent
    let mut stack: Vec<(usize, u8)> = vec![(1, 0)];

    // parent[i] stores who referred node i onto the stack, so we know where to add back
    let mut parent = vec![0usize; n + 1];

    while let Some((node, phase)) = stack.pop() {
        if phase == 0 {
            // First visit: schedule this node for post-processing after its children finish,
            // then push all children for their own first visit
            stack.push((node, 1));
            for &child in &children[node] {
                parent[child] = node;
                stack.push((child, 0));
            }
        } else {
            // Post-visit: this node's subtree is fully processed.
            // Add this node's total (itself + its subordinates) to its parent's count.
            let p = parent[node];
            if p != 0 {
                // +1 for the node itself, plus however many subordinates it accumulated
                subordinates[p] += 1 + subordinates[node];
            }
        }
    }

    // Output one subordinate count per employee, space-separated
    let result: Vec<String> = (1..=n).map(|i| subordinates[i].to_string()).collect();
    writeln!(out, "{}", result.join(" ")).unwrap();
}

fn main() {
    // No large stack thread needed — the iterative approach uses heap memory
    // for its explicit stack, so it handles chains of 2*10^5 nodes safely.
    solve();
}