//! Shows that shared reborrows of one mutable reference may overlap.

// ANCHOR: example
/// Shows overlapping immutable reborrows followed by a mutable access.
fn main() {
    let mut value = 42;
    let exclusive = &mut value;
    let first = &*exclusive;
    let second = &*exclusive;

    assert_eq!((*first, *second), (42, 42));

    // The original mutable reference can be used after the shared references'
    // last use.
    *exclusive += 1;
    assert_eq!(*exclusive, 43);
}
// ANCHOR_END: example

/// Runs the overlapping shared-reborrow example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that shared reborrows can overlap and end before mutation
    /// resumes.
    #[test]
    fn shared_reborrows_can_overlap() {
        main();
    }
}
