// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Criterion benchmark comparing arena allocation (`bumpalo`) against standard
//! heap (`Box`) allocations.
//!
//! An arena allocator allocates many small objects from a few large blocks.
//! Each allocation advances a pointer (a fast bump allocation). All memory is
//! freed at once when the arena is dropped.
//!
//! Contrast with standard heap allocation (`Box::new`), which invokes the
//! global allocator for each item and deallocates items individually.

use std::hint::black_box;

use bumpalo::Bump;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

/// A binary tree node living in a `bumpalo` arena with reference pointers.
struct ArenaNode<'bump> {
    val: i32,
    left: Option<&'bump ArenaNode<'bump>>,
    right: Option<&'bump ArenaNode<'bump>>,
}

/// A binary tree node living on the standard heap using `Box`.
struct BoxNode {
    val: i32,
    left: Option<Box<BoxNode>>,
    right: Option<Box<BoxNode>>,
}

/// Inserts a value into an arena-backed binary search tree.
fn insert_arena<'bump>(
    bump: &'bump Bump,
    node: Option<&'bump ArenaNode<'bump>>,
    val: i32,
) -> &'bump ArenaNode<'bump> {
    match node {
        None => bump.alloc(ArenaNode {
            val,
            left: None,
            right: None,
        }),
        Some(n) if val < n.val => bump.alloc(ArenaNode {
            val: n.val,
            left: Some(insert_arena(bump, n.left, val)),
            right: n.right,
        }),
        Some(n) if val > n.val => bump.alloc(ArenaNode {
            val: n.val,
            left: n.left,
            right: Some(insert_arena(bump, n.right, val)),
        }),
        Some(n) => n,
    }
}

/// Inserts a value into a heap-backed binary search tree using `Box`.
fn insert_box(node: Option<Box<BoxNode>>, val: i32) -> Box<BoxNode> {
    match node {
        None => Box::new(BoxNode {
            val,
            left: None,
            right: None,
        }),
        Some(mut n) if val < n.val => {
            n.left = Some(insert_box(n.left, val));
            n
        }
        Some(mut n) if val > n.val => {
            n.right = Some(insert_box(n.right, val));
            n
        }
        Some(n) => n,
    }
}

/// Benchmarks building a tree in a `bumpalo` arena versus allocating individual
/// `Box` nodes.
fn benchmark_tree_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_construction");

    group.bench_function("bumpalo_arena", |b| {
        b.iter(|| {
            let bump = Bump::new();
            let mut root = None;
            for i in 0..black_box(100) {
                // Alternating insertion pattern to build a semi-balanced tree.
                let val = if i % 2 == 0 { i } else { -i };
                root = Some(insert_arena(&bump, root, val));
            }
            black_box(root);
        });
    });

    group.bench_function("standard_box", |b| {
        b.iter(|| {
            let mut root = None;
            for i in 0..black_box(100) {
                let val = if i % 2 == 0 { i } else { -i };
                root = Some(insert_box(root, val));
            }
            black_box(root);
        });
    });

    group.finish();
}

/// Benchmarks batch allocating small objects in `bumpalo` vs standard `Box`.
fn benchmark_batch_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_allocations");

    group.bench_function("bumpalo_arena", |b| {
        b.iter(|| {
            let bump = Bump::new();
            for i in 0..black_box(1_000) {
                black_box(bump.alloc(i));
            }
        });
    });

    group.bench_function("standard_box", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1_000);
            for i in 0..black_box(1_000) {
                vec.push(Box::new(i));
            }
            black_box(vec);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_tree_construction,
    benchmark_batch_allocations
);
criterion_main!(benches);
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Runs the arena benchmark through its entry point.
    #[test]
    fn test_arena_bench_entry_point() {
        super::main();
    }
}
