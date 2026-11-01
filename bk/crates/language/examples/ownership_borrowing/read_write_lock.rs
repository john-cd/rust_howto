// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Many shared references or one mutable reference.
//!
//! The borrow rules allow many shared references or exactly one mutable
//! reference at a time, never both. This prevents data races and iterator
//! invalidation at compile time. A `RwLock` applies the same rule at run time
//! across threads.

/// A struct holding a flag.
#[derive(Debug)]
struct MyStruct(bool);

/// Takes shared references, then a mutable reference.
fn main() {
    let mut s1 = MyStruct(true);

    // We can take multiple immutable (shared) references at the same time:
    let ref_s1 = &s1;
    let ref_s2 = &s1;

    // We cannot modify `s1` while holding shared references to it.
    // s1.0 = false;
    // ERROR: the borrow checker rejects this assignment while the shared
    // references are still in use.

    println!("{ref_s1:?} {ref_s2:?}"); // Last use of the `ref_*` variables.

    // However, you can modify `s1` once the shared references are no longer
    // in use.
    s1 = MyStruct(false);
    s1.0 = true;

    println!("{s1:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the read write lock example through its entry point.
    #[test]
    fn test_read_write_lock_example_entry_point() {
        main();
    }

    /// Shared borrows end, then the value can be reassigned and mutated.
    #[test]
    fn test_reassign_after_borrows() {
        let mut s1 = MyStruct(true);
        let (a, b) = (&s1, &s1);
        assert!(a.0 && b.0);
        s1 = MyStruct(false);
        assert!(!s1.0);
        s1.0 = true;
        assert!(s1.0);
    }
}
