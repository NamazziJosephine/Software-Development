use std::io::{self, Read, Write, BufWriter};

// Recursively computes the number of subordinates for each node in the tree.
// Returns the total count of descendants under `node`.
fn dfs(node: usize, children: &Vec<Vec<usize>>, subordinates: &mut Vec<usize>) -> usize {
    let mut count = 0;

    for &child in &children[node] {
        // Each direct child counts as 1, plus all of that child's own subordinates
        count += 1 + dfs(child, children, subordinates);
    }

    // Store the final subordinate count for this node
    subordinates[node] = count;
    count
}

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

    // Start DFS from employee 1 (the general director, i.e. the root)
    dfs(1, &children, &mut subordinates);

    // Output one subordinate count per employee, space-separated
    let result: Vec<String> = (1..=n).map(|i| subordinates[i].to_string()).collect();
    writeln!(out, "{}", result.join(" ")).unwrap();
}

fn main() {
    // Rust's default stack (~8 MB) can overflow on a degenerate chain of 2*10^5 nodes.
    // Spawning a thread with a larger stack keeps the solution genuinely recursive
    // without hitting a stack overflow.
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024); // 64 MB
    let handler = builder.spawn(solve).unwrap();
    handler.join().unwrap();
}