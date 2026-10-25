#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A binary search tree whose nodes live in a `bumpalo` arena.
//!
//! Nodes refer to each other with plain `&'bump` references: no `Box`, no
//! `Rc`, one pointer bump per node, and the whole tree is freed at once when
//! the arena is dropped.
//!
//! Caveat: `Drop` does not run for values allocated with `Bump::alloc`. Use
//! `bumpalo::boxed::Box` (the `boxed` feature) when destructors matter.

use std::cell::Cell;

use bumpalo::Bump;

/// A tree node. The references point into the same arena.
struct Node<'bump> {
    key: i32,
    left: Option<&'bump Self>,
    right: Option<&'bump Self>,
}

/// Returns a new tree that also contains `key` (the old nodes are shared).
fn insert<'bump>(
    bump: &'bump Bump,
    node: Option<&'bump Node<'bump>>,
    key: i32,
) -> &'bump Node<'bump> {
    match node {
        None => bump.alloc(Node {
            key,
            left: None,
            right: None,
        }),
        Some(n) if key < n.key => bump.alloc(Node {
            key: n.key,
            left: Some(insert(bump, n.left, key)),
            right: n.right,
        }),
        Some(n) if key > n.key => bump.alloc(Node {
            key: n.key,
            left: n.left,
            right: Some(insert(bump, n.right, key)),
        }),
        Some(n) => n,
    }
}

/// Collects the keys in order.
fn in_order(node: Option<&Node<'_>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        in_order(n.left, out);
        out.push(n.key);
        in_order(n.right, out);
    }
}

thread_local! {
    /// Counts the destructors that ran.
    static DROPS: Cell<u32> = const { Cell::new(0) };
}

/// A value with a destructor.
struct Noisy;

impl Drop for Noisy {
    fn drop(&mut self) {
        DROPS.with(|d| d.set(d.get() + 1));
    }
}

/// Builds a tree in an arena, then shows what happens to destructors.
fn main() {
    let bump = Bump::new();
    let mut root = None;
    for key in [5, 2, 8, 1, 9, 3, 5] {
        root = Some(insert(&bump, root, key));
    }

    let mut keys = Vec::new();
    in_order(root, &mut keys);
    assert_eq!(keys, vec![1, 2, 3, 5, 8, 9]);
    println!("The arena holds {} bytes.", bump.allocated_bytes());

    // `Bump::alloc` never runs destructors...
    DROPS.with(|d| d.set(0));
    {
        let arena = Bump::new();
        let _noisy = arena.alloc(Noisy);
    }
    assert_eq!(DROPS.with(Cell::get), 0);

    // ...but `bumpalo::boxed::Box` does.
    {
        let arena = Bump::new();
        let _boxed = bumpalo::boxed::Box::new_in(Noisy, &arena);
    }
    assert_eq!(DROPS.with(Cell::get), 1);
}
// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
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
