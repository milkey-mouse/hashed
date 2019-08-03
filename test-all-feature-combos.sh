#!/bin/sh
set -e

# note: run with -P0 for a slight speedup if you don't care about the garbled output

permutations() {
    python3 -c 'from itertools import product; import sys; print("\n".join(",".join(filter(None, x)) for x in product(*((None, x.rstrip("\n")) for x in sys.stdin))))'
}

cargo test --no-default-features
sed '1,/^\[features\]$/d' Cargo.toml | cut -d'=' -f1 | tr -d ' ' | grep -Fvx default | permutations | xargs -L1 $1 --no-run-if-empty cargo test --no-default-features --features
