#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A fixed-size bit set with `fixedbitset`: a visited set for a graph walk.
//!
//! `FixedBitSet` uses one bit per element (a `Vec<bool>` uses a byte) and
//! offers fast set operations. Its size is fixed at creation; `bitvec`
//! offers arbitrary bit-level views and a growable vector instead.

use fixedbitset::FixedBitSet;

/// Returns the nodes reachable from `start`, visiting each once.
fn reachable(adjacency: &[Vec<usize>], start: usize) -> FixedBitSet {
    let mut visited = FixedBitSet::with_capacity(adjacency.len());
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        // `put` sets the bit and returns its previous value.
        if visited.put(node) {
            continue;
        }
        stack.extend(
            adjacency[node]
                .iter()
                .copied()
                .filter(|n| !visited.contains(*n)),
        );
    }
    visited
}

/// Walks a small graph with two components.
fn main() {
    // 0 -> 1 -> 2 -> 0, and 3 -> 4
    let adjacency = vec![vec![1], vec![2], vec![0], vec![4], vec![]];

    let from_0 = reachable(&adjacency, 0);
    assert_eq!(from_0.ones().collect::<Vec<_>>(), vec![0, 1, 2]);
    assert!(!from_0.contains(3));

    let from_3 = reachable(&adjacency, 3);
    assert_eq!(from_3.ones().collect::<Vec<_>>(), vec![3, 4]);

    // Set operations work on whole words at a time.
    let mut all = from_0.clone();
    all.union_with(&from_3);
    assert_eq!(all.count_ones(..), 5);
    assert_eq!(all.ones().count(), adjacency.len());

    let mut common = from_0;
    common.intersect_with(&from_3);
    assert!(common.is_clear());
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
