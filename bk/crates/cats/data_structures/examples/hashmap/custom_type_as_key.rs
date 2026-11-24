// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrate using a custom type as the key in a `HashMap`.
//!
//! A `HashMap` key must implement `Hash` and `Eq`, and equal keys must have
//! equal hashes. Deriving both gives your type that property. Do not use floats
//! as keys, since `NaN` is not equal to itself.

use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

// A `HashMap` key must implement `Hash` and `Eq`. Deriving both traits
// ensures that equal keys have equal hashes.

/// A student, used as a `HashMap` key.
#[derive(Debug, PartialEq, Eq)]
struct Student {
    /// The identifier of the student, which determines equality and hash.
    id: u32,
}

// Manual implementation of `Hash` for demonstration purposes.
// BEWARE: If two keys are equal, their hashes must be equal.
// This is a fundamental requirement for hash-based collections.
// k1 == k2 -> hash(k1) == hash(k2).
impl Hash for Student {
    /// Feeds the custom key ID into the supplied hasher.
    ///
    /// # Arguments
    ///
    /// * `state` - Shared state and services used to complete the custom_type_as_key workflow.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Stores grades in a `HashMap` keyed by `Student`.
fn main() {
    // Create a new HashMap to store student grades.
    let mut grades = HashMap::new();

    // Create some student instances.
    let alice = Student { id: 1 };
    let alice2 = Student { id: 1 }; // Same ID.
    let bob = Student { id: 2 };

    // Let's make sure the hashing relationship above stands.
    assert_eq!(alice, alice2);
    assert_eq!(calculate_hash(&alice), calculate_hash(&alice2));

    // Insert student grades into the HashMap.
    grades.insert(alice, 95);
    grades.insert(bob, 87);

    // Look up a student's grade using their ID.
    // Note that we create a new Student instance for lookup,
    // but it's considered the same key if the IDs match.
    let lookup_student = Student { id: 2 };
    if let Some(grade) = grades.get(&lookup_student) {
        println!("Student with ID 2 got grade: {grade}");
    }
}

/// Returns the hash of `t` computed with the default hasher.
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
// ANCHOR_END: example

/// Stores grades in a `HashMap` keyed by `Student`.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the custom type as key example through its entry point.
    #[test]
    fn test_custom_type_as_key_example_entry_point() {
        main();
    }
}
