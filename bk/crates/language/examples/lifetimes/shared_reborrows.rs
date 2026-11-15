// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Shows that shared reborrows of one mutable reference may overlap.

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
/// Shows that shared reborrows of one mutable reference may overlap.
pub fn run() {
    main();
}

/// resumes.
#[cfg(test)]
mod tests {
    use super::*;

    /// resumes.
    #[test]
    fn shared_reborrows_can_overlap() {
        main();
    }
}
