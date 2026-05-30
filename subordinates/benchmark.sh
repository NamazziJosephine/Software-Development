#!/bin/bash

# Build both binaries in release mode first
echo "Building..."
cargo build --release

echo ""
echo "=== Chain input (worst case: 1->2->3->...->n) ==="
python3 -c "
n=200000
print(n)
print(' '.join(str(i) for i in range(1, n)))
" > /tmp/chain.txt

echo "Recursive:"
time ./target/release/recursive < /tmp/chain.txt > /dev/null

echo "Iterative:"
time ./target/release/iterative < /tmp/chain.txt > /dev/null

echo ""
echo "=== Balanced tree ==="
python3 -c "
n=200000
print(n)
print(' '.join(str(i//2) for i in range(2, n+1)))
" > /tmp/balanced.txt

echo "Recursive:"
time ./target/release/recursive < /tmp/balanced.txt > /dev/null

echo "Iterative:"
time ./target/release/iterative < /tmp/balanced.txt > /dev/null

echo ""
echo "=== Star input (everyone reports to node 1) ==="
python3 -c "
n=200000
print(n)
print(' '.join(['1'] * (n-1)))
" > /tmp/star.txt

echo "Recursive:"
time ./target/release/recursive < /tmp/star.txt > /dev/null

echo "Iterative:"
time ./target/release/iterative < /tmp/star.txt > /dev/null