// ANCHOR: example
use std::collections::BTreeSet;

/// The set operation results for the two topic sets.
struct SetComparison {
    /// Elements present in both sets.
    shared: BTreeSet<&'static str>,
    /// Elements present in either set.
    combined: BTreeSet<&'static str>,
    /// Elements present only in the mathematics set.
    math_only: BTreeSet<&'static str>,
}

/// Returns the shared, combined, and first-only elements of two sets.
fn compare_sets() -> SetComparison {
    let math_topics = BTreeSet::from(["logic", "proofs", "sets"]);
    let programming_topics = BTreeSet::from(["logic", "proofs", "types"]);

    let shared = math_topics
        .intersection(&programming_topics)
        .copied()
        .collect();
    let combined = math_topics.union(&programming_topics).copied().collect();
    let math_only = math_topics
        .difference(&programming_topics)
        .copied()
        .collect();

    SetComparison {
        shared,
        combined,
        math_only,
    }
}

/// Prints the set intersection, union, and difference.
fn main() {
    let results = compare_sets();
    println!("Shared: {:?}", results.shared);
    println!("Combined: {:?}", results.combined);
    println!("Math only: {:?}", results.math_only);
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies the set operations return the expected elements.
#[test]
fn test_set_operations_return_expected_elements() {
    let results = compare_sets();
    assert_eq!(results.shared, BTreeSet::from(["logic", "proofs"]));
    assert_eq!(
        results.combined,
        BTreeSet::from(["logic", "proofs", "sets", "types"])
    );
    assert_eq!(results.math_only, BTreeSet::from(["sets"]));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
